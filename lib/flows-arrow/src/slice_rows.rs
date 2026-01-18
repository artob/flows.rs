// This is free and unencumbered software released into the public domain.

use arrow_array::RecordBatch;
use async_flow::{Inputs, Outputs, Result};

/// A block that applies offsets/limits to batches of rows.
#[allow(unused)]
pub async fn slice_rows(
    _offset: usize,
    _limit: usize,
    mut _inputs: Inputs<RecordBatch>,
    _outputs: Outputs<RecordBatch>,
) -> Result {
    todo!(); // TODO

    Ok(())
}
