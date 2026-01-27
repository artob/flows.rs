// This is free and unencumbered software released into the public domain.

#![no_std]
#![forbid(unsafe_code)]

extern crate alloc;

#[cfg(feature = "std")]
extern crate std;

mod block;
mod params;
mod r#type;

use proc_macro::TokenStream;
use syn::{ItemFn, parse_macro_input};

/// Derives a Flows.rs block type from an async function.
#[proc_macro_attribute]
pub fn block(attr: TokenStream, item: TokenStream) -> TokenStream {
    let attr = parse_macro_input!(attr);
    let input_fn = parse_macro_input!(item as ItemFn);
    TokenStream::from(block::block(attr, input_fn))
}
