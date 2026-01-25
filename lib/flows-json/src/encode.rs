// This is free and unencumbered software released into the public domain.

use alloc::{string::String, vec::Vec};
use async_flow::{Inputs, Outputs, Port};
use serde_json::{Result, Value};

/// A block that encodes JSON value inputs to bytes outputs.
pub async fn encode_bytes(
    mut inputs: Inputs<Value>,
    outputs: Outputs<Result<Vec<u8>>>,
) -> async_flow::Result {
    while let Some(input) = inputs.recv().await? {
        let output = serde_json::to_vec(&input);
        if outputs.is_connected() {
            outputs.send(output).await?;
        }
    }
    Ok(())
}

/// A block that encodes JSON value inputs to string outputs.
pub async fn encode_string(
    mut inputs: Inputs<Value>,
    outputs: Outputs<Result<String>>,
) -> async_flow::Result {
    while let Some(input) = inputs.recv().await? {
        let output = serde_json::to_string(&input);
        if outputs.is_connected() {
            outputs.send(output).await?;
        }
    }
    Ok(())
}
