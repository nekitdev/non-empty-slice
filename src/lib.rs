//! Non-empty slices.

#![deny(missing_docs)]
#![cfg_attr(not(feature = "std"), no_std)]
#![cfg_attr(docsrs, feature(doc_cfg))]

#[cfg(feature = "alloc")]
#[macro_use]
extern crate alloc;

#[macro_use]
pub mod macros;

pub mod slice;

pub mod iter;

#[doc(inline)]
pub use slice::{EmptySlice, NonEmptyBytes, NonEmptySlice};

#[cfg(any(feature = "std", feature = "alloc"))]
pub mod boxed;

#[doc(inline)]
#[cfg(any(feature = "std", feature = "alloc"))]
pub use boxed::{EmptyBoxedBytes, EmptyBoxedSlice, NonEmptyBoxedBytes, NonEmptyBoxedSlice};

#[cfg(any(feature = "std", feature = "alloc"))]
pub mod vec;

#[doc(inline)]
#[cfg(any(feature = "std", feature = "alloc"))]
pub use vec::{EmptyByteVec, EmptyVec, NonEmptyByteVec, NonEmptyVec};

#[cfg(any(feature = "std", feature = "alloc"))]
pub mod cow;

#[doc(inline)]
#[cfg(any(feature = "std", feature = "alloc"))]
pub use cow::NonEmptyCowSlice;

#[cfg(any(feature = "std", feature = "alloc"))]
pub(crate) mod format;

#[cfg(feature = "std")]
pub(crate) mod io;

#[cfg(feature = "ownership")]
pub(crate) mod ownership;

#[cfg(feature = "serde")]
pub(crate) mod serde;
