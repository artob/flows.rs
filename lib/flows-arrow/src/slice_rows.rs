// This is free and unencumbered software released into the public domain.

use arrow_array::RecordBatch;
use async_flow::{Inputs, Outputs, Result};

/// A block that applies offsets/limits to batches of rows.
#[allow(unused)]
pub async fn slice_rows(
    mut offset: usize,
    mut limit: Option<usize>,
    mut inputs: Inputs<RecordBatch>,
    outputs: Outputs<RecordBatch>,
) -> Result {
    let mut total_rows = 0;

    while let Some(input) = inputs.recv().await? {
        let batch_len = input.num_rows();
        total_rows += batch_len;

        let output = match (offset, limit) {
            (0, Some(0)) => RecordBatch::new_empty(input.schema()),
            (o, Some(0)) => {
                offset -= batch_len.min(o);
                RecordBatch::new_empty(input.schema())
            },

            (0, None) => input,
            (0, Some(n)) if n <= batch_len => {
                limit = Some(0);
                if n == batch_len {
                    input
                } else {
                    input.slice(0, n)
                }
            },
            (0, Some(n)) if n > batch_len => {
                limit = Some(n - batch_len);
                input
            },

            (o, None) if o <= batch_len => {
                offset -= batch_len.min(o);
                input.slice(o, batch_len - o)
            },
            (o, None) if o > batch_len => {
                offset -= batch_len;
                RecordBatch::new_empty(input.schema())
            },

            (o, Some(n)) if o + n <= batch_len => {
                offset = 0;
                limit = Some(0);
                input.slice(o, n)
            },
            (o, Some(n)) if o >= batch_len => {
                offset -= batch_len;
                RecordBatch::new_empty(input.schema())
            },
            (o, Some(n)) if o < batch_len => {
                let output_len = batch_len - o;
                offset -= o;
                limit = Some(n - output_len);
                input.slice(o, output_len)
            },

            (_, _) => unreachable!(),
        };
        if !outputs.is_closed() {
            outputs.send(output).await?;
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloc::{boxed::Box, vec, vec::Vec};
    use arrow_array::record_batch;
    use async_flow::Channel;
    use core::error::Error;

    #[tokio::test]
    async fn test_slice_rows() -> Result<(), Box<dyn Error>> {
        let output = exec_slice_rows(0, Some(0)).await?;
        assert_eq!(output, vec![]);

        let output = exec_slice_rows(10, Some(0)).await?;
        assert_eq!(output, vec![]);

        let output = exec_slice_rows(0, None).await?;
        assert_eq!(output, (0..=29).collect::<Vec<i32>>());

        let output = exec_slice_rows(0, Some(1)).await?;
        assert_eq!(output, vec![0]);

        let output = exec_slice_rows(0, Some(10)).await?;
        assert_eq!(output, (0..=9).collect::<Vec<i32>>());

        let output = exec_slice_rows(0, Some(11)).await?;
        assert_eq!(output, (0..=10).collect::<Vec<i32>>());

        let output = exec_slice_rows(0, Some(21)).await?;
        assert_eq!(output, (0..=20).collect::<Vec<i32>>());

        let output = exec_slice_rows(1, Some(1)).await?;
        assert_eq!(output, vec![1]);

        let output = exec_slice_rows(9, Some(1)).await?;
        assert_eq!(output, vec![9]);

        let output = exec_slice_rows(9, Some(2)).await?;
        assert_eq!(output, vec![9, 10]);

        let output = exec_slice_rows(9, Some(3)).await?;
        assert_eq!(output, vec![9, 10, 11]);

        let output = exec_slice_rows(9, Some(12)).await?;
        assert_eq!(output, (9..=20).collect::<Vec<i32>>());

        let output = exec_slice_rows(10, Some(1)).await?;
        assert_eq!(output, vec![10]);

        let output = exec_slice_rows(19, Some(2)).await?;
        assert_eq!(output, vec![19, 20]);

        let output = exec_slice_rows(19, Some(3)).await?;
        assert_eq!(output, vec![19, 20, 21]);

        let output = exec_slice_rows(29, Some(1)).await?;
        assert_eq!(output, vec![29]);

        let output = exec_slice_rows(29, Some(2)).await?;
        assert_eq!(output, vec![29]);

        Ok(())
    }

    async fn exec_slice_rows(
        offset: usize,
        limit: Option<usize>,
    ) -> Result<Vec<i32>, Box<dyn Error>> {
        std::eprintln!("\n");
        let mut in_ = Channel::bounded(10);
        let mut out = Channel::bounded(10);
        let slicer = tokio::spawn(slice_rows(offset, limit, in_.rx, out.tx));

        let batch = record_batch!(("n", Int32, [0, 1, 2, 3, 4, 5, 6, 7, 8, 9]))?;
        in_.tx.send(batch).await?;

        let batch = record_batch!(("n", Int32, [10, 11, 12, 13, 14, 15, 16, 17, 18, 19]))?;
        in_.tx.send(batch).await?;

        let batch = record_batch!(("n", Int32, [20, 21, 22, 23, 24, 25, 26, 27, 28, 29]))?;
        in_.tx.send(batch).await?;

        in_.tx.close();

        let _ = tokio::join!(slicer);

        let outputs = out.rx.recv_all().await?;
        assert_eq!(outputs.len(), 3);

        let schema = outputs[0].schema();
        let batch = arrow_select::concat::concat_batches(&schema, &outputs).unwrap();
        let output = batch
            .column(0)
            .as_any()
            .downcast_ref::<arrow_array::Int32Array>()
            .unwrap()
            .values()
            .to_vec();

        Ok(output)
    }
}
