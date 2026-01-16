// This is free and unencumbered software released into the public domain.

use alloc::string::String;
use async_flow::{Inputs, Outputs, Result};

/// A block that splits input strings based on a delimiter.
pub async fn split_string(
    delimiter: impl AsRef<str>,
    mut inputs: Inputs<String>,
    outputs: Outputs<String>,
) -> Result {
    let delimiter = delimiter.as_ref();
    while let Some(input) = inputs.recv().await? {
        for output in input.split(&delimiter) {
            outputs.send(output.into()).await?;
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_split_string() {
        use async_flow::{Port, bounded};

        let (mut in_tx, in_rx) = bounded(1);
        let (out_tx, mut out_rx) = bounded(10);

        let splitter = tokio::spawn(split_string(",", in_rx, out_tx));

        for input in ["hello,world", "foo,bar,baz", "qux"] {
            in_tx.send(input.into()).await.unwrap();
        }
        in_tx.close();

        let _ = tokio::join!(splitter);

        let outputs = out_rx.recv_all().await.unwrap();
        assert_eq!(
            outputs,
            alloc::vec!["hello", "world", "foo", "bar", "baz", "qux"]
        );
    }
}
