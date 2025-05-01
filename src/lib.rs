//! Non-empty slices.

#![cfg_attr(not(feature = "std"), no_std)]
#![deny(missing_docs)]
#![cfg_attr(docsrs, feature(doc_auto_cfg))]

#[cfg(feature = "alloc")]
extern crate alloc;

#[cfg(any(feature = "alloc", feature = "std"))]
pub mod cow;

#[cfg(any(feature = "alloc", feature = "std"))]
pub mod owned;

pub mod empty;
pub mod slice;

#[macro_use]
pub mod macros;

#[cfg(any(feature = "alloc", feature = "std"))]
pub use cow::CowSlice;

#[cfg(any(feature = "alloc", feature = "std"))]
pub use owned::OwnedSlice;

pub use empty::Empty;
pub use slice::Slice;

#[cfg(feature = "static")]
pub use slice::StaticSlice;

#[cfg(all(any(feature = "alloc", feature = "std"), feature = "static"))]
pub use cow::StaticCowSlice;
