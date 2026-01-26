// This is free and unencumbered software released into the public domain.

use async_flow::{Inputs, Outputs, Result};
use core::ops::Add;
use flows_derive::block;
use tokio::try_join;

/// A block that outputs the sums of input numbers.
#[block]
pub async fn add<T>(mut lhs: Inputs<T>, mut rhs: Inputs<T>, sums: Outputs<T>) -> Result
where
    T: Add<Output = T>,
{
    loop {
        let (a, b) = try_join!(lhs.recv(), rhs.recv())?;
        match (a, b) {
            (Some(a), Some(b)) => sums.send(a + b).await?,
            _ => break,
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
    async fn test_add() -> Result<(), Box<dyn Error>> {
        let mut lhs = Channel::bounded(1);
        let mut rhs = Channel::bounded(1);
        let mut sums = Channel::bounded(10);

        let adder = tokio::spawn(add::<isize>(lhs.rx, rhs.rx, sums.tx));

        lhs.tx.send(1).await.unwrap();
        lhs.tx.close();

        rhs.tx.send(2).await.unwrap();
        rhs.tx.close();

        let _ = tokio::join!(adder);

        let sum = sums.rx.recv().await.unwrap();
        assert_eq!(sum, Some(3));

        let sum = sums.rx.recv().await.unwrap();
        assert_eq!(sum, None);

        Ok(())
    }
}
