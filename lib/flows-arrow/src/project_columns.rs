// This is free and unencumbered software released into the public domain.

use arrow_array::RecordBatch;
use async_flow::{Inputs, Outputs, Port, Result};
use flows_derive::block;

/// A block that projects columns from input batches to output batches.
///
/// Panics in case the specified columns are out of bounds.
#[block]
pub async fn project_columns(
    columns: &[usize],
    mut inputs: Inputs<RecordBatch>,
    outputs: Outputs<RecordBatch>,
) -> Result {
    while let Some(input) = inputs.recv().await? {
        if input.num_rows() == 0 {
            continue; // skip empty batches
        }

        let output = input.project(columns).unwrap();
        if !outputs.is_closed() {
            outputs.send(output).await?;
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
    async fn test_project_columns() -> Result<(), Box<dyn Error>> {
        let mut inputs = Channel::bounded(10);
        let mut outputs = Channel::bounded(10);
        let projecter = tokio::spawn(project_columns(&[1], inputs.rx, outputs.tx));

        let input = record_batch!(
            ("a", Int32, [1, 2, 3]),
            ("b", Float64, [Some(4.0), None, Some(5.0)]),
            ("c", Utf8, ["alpha", "beta", "gamma"])
        )?;
        assert_eq!(input.num_columns(), 3);
        assert_eq!(input.num_rows(), 3);
        inputs.tx.send(input.clone()).await?;
        inputs.tx.send(input.clone()).await?;
        inputs.tx.close();

        let _ = tokio::join!(projecter);

        let outputs = outputs.rx.recv_all().await?;
        assert_eq!(outputs.len(), 2);
        for output in outputs {
            assert_eq!(output.num_columns(), 1);
            assert_eq!(output.num_rows(), 3);
        }

        Ok(())
    }
}
