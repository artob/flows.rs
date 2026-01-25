// This is free and unencumbered software released into the public domain.

use alloc::vec::Vec;
use async_flow::{Inputs, Outputs, Port, Result};
use serde_json::Value;

/// A block that encodes JSON from value inputs to byte outputs.
pub async fn encode(mut inputs: Inputs<Value>, outputs: Outputs<Vec<u8>>) -> Result {
    while let Some(input) = inputs.recv().await? {
        let output = serde_json::to_vec(&input)?;
        if outputs.is_connected() {
            outputs.send(output).await?;
        }
    }
    Ok(())
}
