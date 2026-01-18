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
    use alloc::boxed::Box;
    use async_flow::Channel;
    use core::error::Error;

    #[tokio::test]
    async fn test_split_string() -> Result<(), Box<dyn Error>> {
        let mut in_ = Channel::bounded(1);
        let mut out = Channel::bounded(10);

        let splitter = tokio::spawn(split_string(",", in_.rx, out.tx));

        for input in ["hello,world", "foo,bar,baz", "qux"] {
            in_.tx.send(input.into()).await.unwrap();
        }
        in_.tx.close();

        let _ = tokio::join!(splitter);

        let outputs = out.rx.recv_all().await.unwrap();
        assert_eq!(
            outputs,
            alloc::vec!["hello", "world", "foo", "bar", "baz", "qux"]
        );

        Ok(())
    }
}
