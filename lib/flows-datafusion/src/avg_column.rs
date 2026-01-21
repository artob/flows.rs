// This is free and unencumbered software released into the public domain.

use super::sum_array;
use arrow_array::RecordBatch;
use async_flow::{Inputs, Output, Port, Result};
use datafusion_common::ScalarValue;

/// A block that outputs the average of all values in a given column.
///
/// Panics in case the specified column index is out of bounds.
/// Outputs `ScalarValue::Null` in case the specified column has a non-numeric
/// datatype.
pub async fn avg_column(
    column: usize,
    mut inputs: Inputs<RecordBatch>,
    output: Output<ScalarValue>,
) -> Result {
    let mut tally: ScalarValue = ScalarValue::Null;
    let mut count: usize = 0;

    while let Some(input) = inputs.recv().await? {
        if input.num_rows() == 0 {
            continue; // skip empty batches
        }

        let column_array = input.column(column);
        let column_len = column_array.len() - column_array.null_count();
        if column_len == 0 {
            continue; // skip null-only batches
        }

        let Some(column_sum) = sum_array(column_array) else {
            continue; // skip unsupported datatypes
        };

        let column_avg = avg(column_sum, column_len).unwrap();

        tally = if tally.is_null() {
            column_avg
        } else {
            tally.add(column_avg).unwrap()
        };
        count += 1;
    }

    let result = if count == 0 {
        ScalarValue::Null
    } else {
        avg(tally, count).unwrap()
    };

    if !output.is_closed() {
        output.send(result).await?;
    }

    Ok(())
}

pub fn avg(sum: ScalarValue, len: usize) -> Option<ScalarValue> {
    assert!(len > 0);
    use arrow_schema::DataType::*;
    let sum = sum.cast_to(&Float64).unwrap();
    let len = ScalarValue::Float64(Some(len as f64));
    sum.div(len).ok()
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloc::{boxed::Box, vec};
    use arrow_array::record_batch;
    use async_flow::{Channel, InputPort};
    use core::error::Error;

    #[tokio::test]
    async fn test_avg_column_i32() -> Result<(), Box<dyn Error>> {
        let mut in_ = Channel::bounded(10);
        let mut out = Channel::oneshot();
        let averager = tokio::spawn(avg_column(0, in_.rx, out.tx));

        in_.tx.send(sample_data()).await?;
        in_.tx.send(sample_data()).await?;
        in_.tx.close();

        let _ = tokio::join!(averager);

        let outputs = out.rx.recv_all().await?;
        assert_eq!(outputs.len(), 1);
        assert_eq!(outputs[0], ScalarValue::from(3.0));

        Ok(())
    }

    fn sample_data() -> RecordBatch {
        record_batch!(
            ("a", Int32, [1, 2, 3, 4, 5]),
            ("b", Float64, [Some(4.0), None, Some(5.0), None, None]),
            ("c", Utf8, ["alpha", "beta", "gamma", "", ""])
        )
        .unwrap()
    }
}
