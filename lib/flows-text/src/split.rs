// This is free and unencumbered software released into the public domain.

use alloc::string::String;
use async_flow::{
    io::{Port, Result},
    tokio::{Inputs, Outputs, System},
};

/// A block that splits strings based on a delimiter.
async fn split_string(
    delimiter: String,
    mut inputs: Inputs<String>,
    outputs: Outputs<String>,
) -> Result {
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
        use async_flow::tokio::bounded;

        let (mut in_tx, in_rx) = bounded(1);
        let (out_tx, mut out_rx) = bounded(10);

        let splitter = tokio::spawn(split_string(",".into(), in_rx, out_tx));

        for input in ["hello,world", "foo,bar,baz", "qux"] {
            in_tx.send(input.into()).await.unwrap();
        }
        in_tx.close();

        tokio::join!(splitter);

        let outputs = out_rx.recv_all().await.unwrap();
        assert_eq!(
            outputs,
            alloc::vec!["hello", "world", "foo", "bar", "baz", "qux"]
        );
    }
}
