// This is free and unencumbered software released into the public domain.

use alloc::vec::Vec;
use arrow_array::RecordBatch;
use async_flow::{Inputs, Output, Port, Result};

/// A block that concatenates input batches into a single output batch.
pub async fn concat_batches(
    mut inputs: Inputs<RecordBatch>,
    output: Output<RecordBatch>,
) -> Result {
    let mut batches: Vec<RecordBatch> = Vec::new();

    while let Some(batch) = inputs.recv().await? {
        if batch.num_rows() == 0 && !batches.is_empty() {
            // Skip empty batches after the first one
            continue;
        }
        batches.push(batch);
    }

    if !batches.is_empty() {
        let schema = batches[0].schema();
        let batch = arrow_select::concat::concat_batches(&schema, &batches).unwrap();

        if !output.is_closed() {
            output.send(batch).await?;
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloc::{boxed::Box, vec};
    use arrow_array::record_batch;
    use async_flow::{Channel, InputPort};
    use core::error::Error;

    #[tokio::test]
    async fn test_concat_batches() -> Result<(), Box<dyn Error>> {
        let mut in_ = Channel::bounded(1);
        let mut out = Channel::oneshot();
        let concatter = tokio::spawn(concat_batches(in_.rx, out.tx));

        let batch = record_batch!(("n", Int32, [1, 2, 3, 4, 5, 6, 7, 8, 9, 10]))?;
        in_.tx.send(batch.clone()).await?;
        in_.tx.send(batch.clone()).await?;
        in_.tx.close();

        let _ = tokio::join!(concatter);

        let outputs = out.rx.recv_all().await?;
        assert_eq!(outputs.len(), 1);

        for output in outputs {
            assert_eq!(output.num_rows(), 20);
            assert_eq!(output.num_columns(), 1);
        }

        Ok(())
    }
}
