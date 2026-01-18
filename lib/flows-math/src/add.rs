// This is free and unencumbered software released into the public domain.

use async_flow::{Inputs, Outputs, Result};
use core::ops::Add;
use tokio::try_join;

/// A block that outputs the sums of input numbers.
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

    #[tokio::test]
    async fn test_add() {
        use async_flow::bounded;

        let (mut lhs_tx, lhs_rx) = bounded(1);
        let (mut rhs_tx, rhs_rx) = bounded(1);
        let (sums_tx, mut sums_rx) = bounded(10);

        let adder = tokio::spawn(add::<isize>(lhs_rx, rhs_rx, sums_tx));

        lhs_tx.send(1).await.unwrap();
        lhs_tx.close();

        rhs_tx.send(2).await.unwrap();
        rhs_tx.close();

        let _ = tokio::join!(adder);

        let sum = sums_rx.recv().await.unwrap();
        assert_eq!(sum, Some(3));

        let sum = sums_rx.recv().await.unwrap();
        assert_eq!(sum, None);
    }
}
