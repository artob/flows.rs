// This is free and unencumbered software released into the public domain.

#![no_std]
#![forbid(unsafe_code)]
//#![allow(unused)]

extern crate alloc;

#[cfg(feature = "std")]
extern crate std;

pub use async_flow::*;

#[cfg(feature = "arrow")]
pub use flows_arrow as arrow;

#[cfg(feature = "audio")]
pub use flows_audio as audio;

#[cfg(feature = "datafusion")]
pub use flows_datafusion as datafusion;

#[cfg(feature = "derive")]
pub use flows_derive as derive;

#[cfg(feature = "dns")]
pub use flows_dns as dns;

#[cfg(feature = "hash")]
pub use flows_hash as hash;

#[cfg(feature = "http")]
pub use flows_http as http;

#[cfg(feature = "image")]
pub use flows_image as image;

#[cfg(feature = "io")]
pub use flows_io as io;

#[cfg(feature = "json")]
pub use flows_json as json;

#[cfg(feature = "math")]
pub use flows_math as math;

#[cfg(feature = "mdns")]
pub use flows_mdns as mdns;

#[cfg(feature = "pubsub")]
pub use flows_pubsub as pubsub;

#[cfg(feature = "rand")]
pub use flows_rand as rand;

#[cfg(feature = "text")]
pub use flows_text as text;

#[cfg(feature = "video")]
pub use flows_video as video;

#[doc = include_str!("../../../README.md")]
#[cfg(doctest)]
pub struct ReadmeDoctests;
