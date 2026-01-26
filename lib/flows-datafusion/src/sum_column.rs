// This is free and unencumbered software released into the public domain.

use arrow_array::{ArrayRef, RecordBatch};
use async_flow::{Inputs, Output, Port, Result};
use datafusion_common::ScalarValue;
use flows_derive::block;

/// A block that outputs the sum of the values in a given column.
///
/// Panics in case the specified column index is out of bounds.
/// Outputs `ScalarValue::Null` in case the specified column has a non-numeric
/// datatype.
#[block]
pub async fn sum_column(
    column: usize,
    mut inputs: Inputs<RecordBatch>,
    output: Output<ScalarValue>,
) -> Result {
    let mut result: ScalarValue = ScalarValue::Null;

    while let Some(input) = inputs.recv().await? {
        if input.num_rows() == 0 {
            continue; // skip empty batches
        }

        let column_array = input.column(column);
        let Some(column_sum) = sum_array(column_array) else {
            continue; // skip unsupported datatypes
        };

        result = if result.is_null() {
            column_sum
        } else {
            result.add(column_sum).unwrap()
        }
    }

    if !output.is_closed() {
        output.send(result).await?;
    }

    Ok(())
}

pub fn sum_array(array: &ArrayRef) -> Option<ScalarValue> {
    use arrow_arith::aggregate::sum;
    use arrow_array::{cast::AsArray, types::*};
    use arrow_schema::DataType::*;
    Some(match array.data_type() {
        Int8 => ScalarValue::from(sum(array.as_primitive::<Int8Type>())),
        Int16 => ScalarValue::from(sum(array.as_primitive::<Int16Type>())),
        Int32 => ScalarValue::from(sum(array.as_primitive::<Int32Type>())),
        Int64 => ScalarValue::from(sum(array.as_primitive::<Int64Type>())),
        UInt8 => ScalarValue::from(sum(array.as_primitive::<UInt8Type>())),
        UInt16 => ScalarValue::from(sum(array.as_primitive::<UInt16Type>())),
        UInt32 => ScalarValue::from(sum(array.as_primitive::<UInt32Type>())),
        UInt64 => ScalarValue::from(sum(array.as_primitive::<UInt64Type>())),
        Float16 => ScalarValue::from(sum(array.as_primitive::<Float16Type>())),
        Float32 => ScalarValue::from(sum(array.as_primitive::<Float32Type>())),
        Float64 => ScalarValue::from(sum(array.as_primitive::<Float64Type>())),
        // TODO: Decimal32, Decimal64, Decimal128, Decimal256
        _ => return None,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloc::{boxed::Box, vec};
    use arrow_array::record_batch;
    use async_flow::{Channel, InputPort};
    use core::error::Error;

    #[tokio::test]
    async fn test_sum_column_i32() -> Result<(), Box<dyn Error>> {
        let mut in_ = Channel::bounded(10);
        let mut out = Channel::oneshot();
        let summer = tokio::spawn(sum_column(0, in_.rx, out.tx));

        in_.tx.send(sample_data()).await?;
        in_.tx.send(sample_data()).await?;
        in_.tx.close();

        let _ = tokio::join!(summer);

        let outputs = out.rx.recv_all().await?;
        assert_eq!(outputs.len(), 1);
        assert_eq!(outputs[0], ScalarValue::from(30i32));

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
