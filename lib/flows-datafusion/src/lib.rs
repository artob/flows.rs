// This is free and unencumbered software released into the public domain.

#![no_std]
#![forbid(unsafe_code)]
//#![allow(unused)]

extern crate alloc;

#[cfg(feature = "std")]
extern crate std;

mod max_column;
pub use max_column::*;

mod min_column;
pub use min_column::*;

mod sum_column;
pub use sum_column::*;
