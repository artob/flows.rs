# Flows.rs

[![License](https://img.shields.io/badge/license-Public%20Domain-blue.svg)](https://unlicense.org)
[![Compatibility](https://img.shields.io/badge/rust-1.85%2B-blue)](https://blog.rust-lang.org/2025/02/20/Rust-1.85.0/)
[![Package](https://img.shields.io/crates/v/flows)](https://crates.io/crates/flows)
[![Documentation](https://docs.rs/flows/badge.svg)](https://docs.rs/flows)

Building blocks for flow-based programming (FBP) in Rust.

🚧 _We are building in public. This is presently under heavy construction._

## 🛠️ Prerequisites

- [Rust](https://rust-lang.org) 1.85+ (2024 edition)

## ⬇️ Installation

### Installation via Cargo

```bash
cargo add flows
```

## 👉 Examples

### Importing the library

```rust
use flows::*;
```

### Implementing blocks

#### Implementing a `split_string` block

```rust
use flows::{Inputs, Outputs, Result};

/// A block that splits input strings based on a delimiter.
async fn split_string(delim: &str, mut ins: Inputs<String>, outs: Outputs<String>) -> Result {
    while let Some(input) = ins.recv().await? {
        for output in input.split(delim) {
            outs.send(output.into()).await?;
        }
    }
    Ok(())
}
```

#### Implementing an `add_ints` block

```rust
use flows::{Inputs, Outputs, Result};

/// A block that outputs the sums of input numbers.
async fn add_ints(mut lhs: Inputs<i64>, mut rhs: Inputs<i64>, sums: Outputs<i64>) -> Result {
    loop {
        let (a, b) = tokio::try_join!(lhs.recv(), rhs.recv())?;
        match (a, b) {
            (Some(a), Some(b)) => sums.send(a + b).await?,
            _ => break,
        }
    }
    Ok(())
}
```

## 📚 Reference

[docs.rs/flows](https://docs.rs/flows)

## 👨‍💻 Development

```bash
git clone https://github.com/artob/flows.rs.git
```

---

[![Share on X](https://img.shields.io/badge/share%20on-x-03A9F4?logo=x)](https://x.com/intent/post?url=https://github.com/artob/flows.rs&text=Flows.rs)
[![Share on Reddit](https://img.shields.io/badge/share%20on-reddit-red?logo=reddit)](https://reddit.com/submit?url=https://github.com/artob/flows.rs&title=Flows.rs)
[![Share on Hacker News](https://img.shields.io/badge/share%20on-hn-orange?logo=ycombinator)](https://news.ycombinator.com/submitlink?u=https://github.com/artob/flows.rs&t=Flows.rs)
[![Share on Facebook](https://img.shields.io/badge/share%20on-fb-1976D2?logo=facebook)](https://www.facebook.com/sharer/sharer.php?u=https://github.com/artob/flows.rs)
[![Share on LinkedIn](https://img.shields.io/badge/share%20on-linkedin-3949AB?logo=linkedin)](https://www.linkedin.com/sharing/share-offsite/?url=https://github.com/artob/flows.rs)
