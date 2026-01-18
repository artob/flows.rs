// This is free and unencumbered software released into the public domain.

use arrow_array::RecordBatch;
use async_flow::{Inputs, Outputs, Result};

/// A block that outputs row counts of input record batches.
pub async fn count_rows(
    mut batches: Inputs<RecordBatch>,
    counts: Outputs<usize>,
    total: Outputs<usize>,
) -> Result {
    let mut total_rows = 0;

    while let Some(batch) = batches.recv().await? {
        let batch_rows = batch.num_rows();
        total_rows += batch_rows;

        if !counts.is_closed() {
            counts.send(batch_rows).await?;
        }
    }

    if !total.is_closed() {
        total.send(total_rows).await?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloc::vec;
    use arrow_array::{Float32Array, Int32Array};
    use std::sync::Arc;

    #[tokio::test]
    async fn test_count_rows() {
        use async_flow::bounded;

        let col_1 = Arc::new(Int32Array::from_iter([1, 2, 3])) as _;
        let col_2 = Arc::new(Float32Array::from_iter([1., 6.3, 4.])) as _;
        let batch = RecordBatch::try_from_iter(vec![("col_1", col_1), ("col_2", col_2)]).unwrap();

        let (mut batches_tx, batches_rx) = bounded(10);
        let (counts_tx, mut counts_rx) = bounded(10);
        let (total_tx, mut total_rx) = bounded(10);

        let counter = tokio::spawn(count_rows(batches_rx, counts_tx, total_tx));

        batches_tx.send(batch.clone()).await.unwrap();
        batches_tx.send(batch.clone()).await.unwrap();
        batches_tx.close();

        let _ = tokio::join!(counter);

        assert_eq!(counts_rx.recv().await.unwrap(), Some(3));
        assert_eq!(counts_rx.recv().await.unwrap(), Some(3));

        assert_eq!(total_rx.recv().await.unwrap(), Some(6));
    }
}
