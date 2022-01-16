//! Tons of extension utility functions for Rust.
//!
#![cfg_attr(not(feature = "std"), no_std)]
#![doc(html_root_url = "https://docs.rs/lazyext/0.0.2")]
#![cfg_attr(docsrs, feature(doc_cfg))]
#![cfg_attr(docsrs, allow(unused_attributes))]
#![deny(missing_docs)]

pub use lazyext_bytes as slice_ext;

/// util macros
#[cfg(feature = "lazyext-macros")]
pub use lazyext_macros::{cfg_test, cfg_unix, cfg_windows};
