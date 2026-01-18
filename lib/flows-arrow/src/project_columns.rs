// This is free and unencumbered software released into the public domain.

use arrow_array::RecordBatch;
use async_flow::{Inputs, Outputs, Result};

/// A block that projects columns from input batches to output batches.
///
/// Panics in case the specified columns are out of bounds.
pub async fn project_columns(
    columns: &[usize],
    mut inputs: Inputs<RecordBatch>,
    outputs: Outputs<RecordBatch>,
) -> Result {
    while let Some(input) = inputs.recv().await? {
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
    use alloc::vec;
    use arrow_array::record_batch;
    use async_flow::bounded;

    #[tokio::test]
    async fn test_project_columns() {
        let input = record_batch!(
            ("a", Int32, [1, 2, 3]),
            ("b", Float64, [Some(4.0), None, Some(5.0)]),
            ("c", Utf8, ["alpha", "beta", "gamma"])
        )
        .unwrap();
        assert_eq!(input.num_columns(), 3);
        assert_eq!(input.num_rows(), 3);

        let (mut inputs_tx, inputs_rx) = bounded(10);
        let (outputs_tx, mut outputs_rx) = bounded(10);

        let projecter = tokio::spawn(project_columns(&[1], inputs_rx, outputs_tx));

        inputs_tx.send(input.clone()).await.unwrap();
        inputs_tx.send(input.clone()).await.unwrap();
        inputs_tx.close();

        let _ = tokio::join!(projecter);

        for output in outputs_rx.recv_all().await.unwrap() {
            assert_eq!(output.num_columns(), 1);
            assert_eq!(output.num_rows(), 3);
        }
    }
}
