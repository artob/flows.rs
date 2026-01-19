// This is free and unencumbered software released into the public domain.

#![no_std]
#![forbid(unsafe_code)]
//#![allow(unused)]

extern crate alloc;

#[cfg(feature = "std")]
extern crate std;

mod concat_batches;
pub use concat_batches::*;

mod count_rows;
pub use count_rows::*;

mod project_columns;
pub use project_columns::*;

mod slice_rows;
pub use slice_rows::*;
