#![allow(clippy::module_inception)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::ptr_arg)]
#![allow(clippy::large_enum_variant)]
#![doc = "generated by AutoRust"]
#[cfg(feature = "v1")]
pub mod v1;
#[cfg(all(feature = "v1", not(feature = "no-default-tag")))]
pub use v1::{models, operations, operations::Client, operations::ClientBuilder};
