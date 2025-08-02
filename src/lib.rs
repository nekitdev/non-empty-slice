//! Non-empty slices.

#![cfg_attr(not(feature = "std"), no_std)]
#![deny(missing_docs)]
#![cfg_attr(docsrs, feature(doc_auto_cfg))]

#[cfg(feature = "alloc")]
extern crate alloc;

#[macro_use]
pub mod macros;

pub mod slice;

pub use slice::{Bytes, Empty, Slice};

#[cfg(any(feature = "std", feature = "alloc"))]
pub mod owned;

#[cfg(any(feature = "std", feature = "alloc"))]
pub use owned::{EmptyOwned, OwnedBytes, OwnedSlice};
