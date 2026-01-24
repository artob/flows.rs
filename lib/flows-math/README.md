# Flows.rs: Mathematical Operations

[![License](https://img.shields.io/badge/license-Public%20Domain-blue.svg)](https://unlicense.org)
[![Compatibility](https://img.shields.io/badge/rust-1.85%2B-blue)](https://blog.rust-lang.org/2025/02/20/Rust-1.85.0/)
[![Package](https://img.shields.io/crates/v/flows-math)](https://crates.io/crates/flows-math)
[![Documentation](https://img.shields.io/docsrs/flows-math?label=docs.rs)](https://docs.rs/flows-math)
[![Featured](https://img.shields.io/badge/awesome-fbp-lightgrey)](https://github.com/artob/awesome-fbp)

_"Τὰ πάντα ῥεῖ καὶ οὐδὲν μένει" — Heraclitus_

**Building blocks for flow-based mathematical operations in Rust.**
This package is part of [Flows.rs], a growing shrink-wrap inventory of
standard, reusable dataflow blocks for common use cases.

> [!TIP]
> 🚧 _We are building in public. This is presently under heavy construction._

<br/>

<sub>

[[Features](#-features)] |
[[Prerequisites](#%EF%B8%8F-prerequisites)] |
[[Installation](#%EF%B8%8F-installation)] |
[[Examples](#-examples)] |
[[Reference](#-reference)] |
[[Development](#%E2%80%8D-development)]

</sub>

## ✨ Features

- Provides components for [flow-based programming] (FBP) based on [Tokio].
- Enables dataflow systems through reusable components called blocks.
- Built on the dataflow primitives provided by the [Async-Flow] project.
- Supports opting out of any feature using comprehensive feature flags.
- Adheres to the Rust API Guidelines in its [naming conventions].
- Cuts red tape: 100% free and unencumbered public domain software.

## 🛠️ Prerequisites

- [Rust](https://rust-lang.org) 1.85+ (2024 edition)

## ⬇️ Installation

### Installation via Cargo

```bash
cargo add flows-math
```

## 👉 Examples

### Importing the Library

```rust
use flows_math::*;
```

## 📚 Reference

[docs.rs/flows-math](https://docs.rs/flows-math)

### Packages

| Package | Summary | Crate | Documentation |
| :------ | :------ | :---- | :------------ |
| [flows](https://github.com/artob/flows.rs/tree/master/lib/flows) | Flow-based programming (FBP). | [![Package](https://img.shields.io/crates/v/flows)](https://crates.io/crates/flows) | [![Documentation](https://img.shields.io/docsrs/flows?label=docs.rs)](https://docs.rs/flows) |
| [flows-arrow](https://github.com/artob/flows.rs/tree/master/lib/flows-arrow) | Flow-based data processing with Apache Arrow. | [![Package](https://img.shields.io/crates/v/flows-arrow)](https://crates.io/crates/flows-arrow) | [![Documentation](https://img.shields.io/docsrs/flows-arrow?label=docs.rs)](https://docs.rs/flows-arrow) |
| [flows-audio](https://github.com/artob/flows.rs/tree/master/lib/flows-audio) | Flow-based audio processing. | [![Package](https://img.shields.io/crates/v/flows-audio)](https://crates.io/crates/flows-audio) | [![Documentation](https://img.shields.io/docsrs/flows-audio?label=docs.rs)](https://docs.rs/flows-audio) |
| [flows-datafusion](https://github.com/artob/flows.rs/tree/master/lib/flows-datafusion) | Flow-based query processing with Apache DataFusion. | [![Package](https://img.shields.io/crates/v/flows-datafusion)](https://crates.io/crates/flows-datafusion) | [![Documentation](https://img.shields.io/docsrs/flows-datafusion?label=docs.rs)](https://docs.rs/flows-datafusion) |
| [flows-derive](https://github.com/artob/flows.rs/tree/master/lib/flows-derive) | Derive macros for flow-based programming (FBP). | [![Package](https://img.shields.io/crates/v/flows-derive)](https://crates.io/crates/flows-derive) | [![Documentation](https://img.shields.io/docsrs/flows-derive?label=docs.rs)](https://docs.rs/flows-derive) |
| [flows-hash](https://github.com/artob/flows.rs/tree/master/lib/flows-hash) | Flow-based cryptographic hashing. | [![Package](https://img.shields.io/crates/v/flows-hash)](https://crates.io/crates/flows-hash) | [![Documentation](https://img.shields.io/docsrs/flows-hash?label=docs.rs)](https://docs.rs/flows-hash) |
| [flows-http](https://github.com/artob/flows.rs/tree/master/lib/flows-http) | Flow-based HTTP requests & responses. | [![Package](https://img.shields.io/crates/v/flows-http)](https://crates.io/crates/flows-http) | [![Documentation](https://img.shields.io/docsrs/flows-http?label=docs.rs)](https://docs.rs/flows-http) |
| [flows-image](https://github.com/artob/flows.rs/tree/master/lib/flows-image) | Flow-based image processing. | [![Package](https://img.shields.io/crates/v/flows-image)](https://crates.io/crates/flows-image) | [![Documentation](https://img.shields.io/docsrs/flows-image?label=docs.rs)](https://docs.rs/flows-image) |
| [flows-io](https://github.com/artob/flows.rs/tree/master/lib/flows-io) | Flow-based I/O readers & writers. | [![Package](https://img.shields.io/crates/v/flows-io)](https://crates.io/crates/flows-io) | [![Documentation](https://img.shields.io/docsrs/flows-io?label=docs.rs)](https://docs.rs/flows-io) |
| [flows-json](https://github.com/artob/flows.rs/tree/master/lib/flows-json) | Flow-based JSON encoding & decoding. | [![Package](https://img.shields.io/crates/v/flows-json)](https://crates.io/crates/flows-json) | [![Documentation](https://img.shields.io/docsrs/flows-json?label=docs.rs)](https://docs.rs/flows-json) |
| [flows-math](https://github.com/artob/flows.rs/tree/master/lib/flows-math) | Flow-based mathematical operations. | [![Package](https://img.shields.io/crates/v/flows-math)](https://crates.io/crates/flows-math) | [![Documentation](https://img.shields.io/docsrs/flows-math?label=docs.rs)](https://docs.rs/flows-math) |
| [flows-rand](https://github.com/artob/flows.rs/tree/master/lib/flows-rand) | Flow-based random number generation. | [![Package](https://img.shields.io/crates/v/flows-rand)](https://crates.io/crates/flows-rand) | [![Documentation](https://img.shields.io/docsrs/flows-rand?label=docs.rs)](https://docs.rs/flows-rand) |
| [flows-text](https://github.com/artob/flows.rs/tree/master/lib/flows-text) | Flow-based text processing. | [![Package](https://img.shields.io/crates/v/flows-text)](https://crates.io/crates/flows-text) | [![Documentation](https://img.shields.io/docsrs/flows-text?label=docs.rs)](https://docs.rs/flows-text) |

### Integrations

TBD

### Glossary

- **System**: A collection of blocks that are connected together.
  Systems are the top-level entities in dataflow programs.

- **Block**: An encapsulated system component that processes messages.
  Blocks are the autonomous units of computation in a system.

- **Port**: A named connection point on a block that sends or receives
  messages. Ports are the only interfaces through which blocks communicate
  with each other.

- **Message**: A unit of data that flows between blocks in a system, from port
  to port. Any Rust type that implements the `Send + Sync + 'static` traits can
  be used as a message.

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

[Async-Flow]: https://github.com/artob/async-flow
[Flows.rs]: https://github.com/artob/flows.rs
[Tokio]: https://tokio.rs
[flow-based programming]: https://jpaulm.github.io/fbp/
[naming conventions]: https://rust-lang.github.io/api-guidelines/naming.html
