// This is free and unencumbered software released into the public domain.

use alloc::{string::String, vec::Vec};
use async_flow::{Inputs, Outputs, Port};
use flows_derive::block;
use serde_json::{Result, Value};

/// A block that decodes JSON value outputs from bytes inputs.
#[block]
pub async fn decode_bytes(
    mut inputs: Inputs<Vec<u8>>,
    outputs: Outputs<Result<Value>>,
) -> async_flow::Result {
    while let Some(input) = inputs.recv().await? {
        let output: Result<Value> = serde_json::from_slice(&input);
        if outputs.is_connected() {
            outputs.send(output).await?;
        }
    }
    Ok(())
}

/// A block that decodes JSON value outputs from from string inputs.
#[block]
pub async fn decode_string(
    mut inputs: Inputs<String>,
    outputs: Outputs<Result<Value>>,
) -> async_flow::Result {
    while let Some(input) = inputs.recv().await? {
        let output: Result<Value> = serde_json::from_str(&input);
        if outputs.is_connected() {
            outputs.send(output).await?;
        }
    }
    Ok(())
}
