// This is free and unencumbered software released into the public domain.

use alloc::boxed::Box;
use core::error::Error as StdError;
use thiserror::Error;

pub type Result<T = (), E = Error> = core::result::Result<T, E>;

#[derive(Debug, Error)]
pub enum Error {
    #[error("missing URL scheme")]
    MissingUrlScheme,

    #[error("missing URL host")]
    MissingUrlHost,

    #[error("failed TCP connection: {0}")]
    TcpConnectFailed(std::io::Error),

    #[error("failed HTTP handshake: {0}")]
    HttpHandshakeFailed(hyper::Error),

    #[error("failed HTTP request: {0}")]
    HttpRequestFailed(#[from] hyper::Error),

    #[cfg(feature = "std")]
    #[error("failed I/O: {0}")]
    Stdio(#[from] std::io::Error),

    #[error("unknown error: {0}")]
    Other(#[from] Box<dyn StdError + Send + Sync>),
}
