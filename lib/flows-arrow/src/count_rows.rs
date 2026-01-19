// This is free and unencumbered software released into the public domain.

use arrow_array::RecordBatch;
use async_flow::{Inputs, Output, Outputs, Result};

/// A block that outputs row counts of input record batches.
pub async fn count_rows(
    mut batches: Inputs<RecordBatch>,
    counts: Outputs<usize>,
    total: Output<usize>,
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
    use alloc::{boxed::Box, vec};
    use arrow_array::record_batch;
    use async_flow::Channel;
    use core::error::Error;

    #[tokio::test]
    async fn test_count_rows() -> Result<(), Box<dyn Error>> {
        let mut batches = Channel::bounded(10);
        let mut counts = Channel::bounded(10);
        let mut total = Channel::oneshot();
        let counter = tokio::spawn(count_rows(batches.rx, counts.tx, total.tx));

        let batch = record_batch!(
            ("a", Int32, [1, 2, 3]),
            ("b", Float64, [Some(4.0), None, Some(5.0)]),
            ("c", Utf8, ["alpha", "beta", "gamma"])
        )?;
        batches.tx.send(batch.clone()).await?;
        batches.tx.send(batch.clone()).await?;
        batches.tx.close();

        let _ = tokio::join!(counter);

        let counts = counts.rx.recv_all().await?;
        assert_eq!(counts.len(), 2);
        for count in counts {
            assert_eq!(count, 3);
        }

        assert_eq!(total.rx.recv().await?, Some(6));

        Ok(())
    }
}
