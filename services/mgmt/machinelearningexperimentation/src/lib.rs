#![allow(clippy::module_inception)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::ptr_arg)]
#![allow(clippy::large_enum_variant)]
#![doc = "generated by AutoRust"]
#[cfg(feature = "package-2017-05-preview")]
pub mod package_2017_05_preview;
#[cfg(all(feature = "package-2017-05-preview", not(feature = "no-default-tag")))]
pub use package_2017_05_preview::{models, operations, operations::Client, operations::ClientBuilder};
