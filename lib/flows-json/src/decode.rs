// This is free and unencumbered software released into the public domain.

use alloc::vec::Vec;
use async_flow::{Inputs, Outputs, Port, Result};
use serde_json::Value;

/// A block that decodes JSON from byte inputs to value outputs.
pub async fn decode(mut inputs: Inputs<Vec<u8>>, outputs: Outputs<Value>) -> Result {
    while let Some(input) = inputs.recv().await? {
        let output: Value = serde_json::from_slice(&input)?;
        if outputs.is_connected() {
            outputs.send(output).await?;
        }
    }
    Ok(())
}
