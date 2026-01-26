// This is free and unencumbered software released into the public domain.

#![no_std]
#![forbid(unsafe_code)]

extern crate alloc;

#[cfg(feature = "std")]
extern crate std;

mod block;

use proc_macro::TokenStream;

/// Derives a Flows.rs block type from an async function.
#[proc_macro_attribute]
pub fn block(attr: TokenStream, item: TokenStream) -> TokenStream {
    block::block(attr, item)
}
