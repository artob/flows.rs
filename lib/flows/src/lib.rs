// This is free and unencumbered software released into the public domain.

#![no_std]
#![forbid(unsafe_code)]
#![allow(unused)]

extern crate alloc;

#[cfg(feature = "std")]
extern crate std;

pub use async_flow::*;

#[cfg(feature = "audio")]
pub use flows_audio as audio;

#[cfg(feature = "derive")]
pub use flows_derive as derive;

#[cfg(feature = "hash")]
pub use flows_hash as hash;

#[cfg(feature = "image")]
pub use flows_image as image;

#[cfg(feature = "io")]
pub use flows_io as io;

#[cfg(feature = "json")]
pub use flows_json as json;

#[cfg(feature = "math")]
pub use flows_math as math;

#[cfg(feature = "rand")]
pub use flows_rand as rand;

#[cfg(feature = "text")]
pub use flows_text as text;

#[doc = include_str!("../../../README.md")]
#[cfg(doctest)]
pub struct ReadmeDoctests;
