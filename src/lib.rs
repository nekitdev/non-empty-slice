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

cfg_select! {
    any(feature = "std", feature = "alloc") => {
        pub mod boxed;
        pub mod cow;
        pub mod vec;

        #[doc(inline)]
        pub use boxed::{EmptyBoxedBytes, EmptyBoxedSlice, NonEmptyBoxedBytes, NonEmptyBoxedSlice};

        #[doc(inline)]
        pub use cow::NonEmptyCowSlice;

        #[doc(inline)]
        pub use vec::{EmptyByteVec, EmptyVec, NonEmptyByteVec, NonEmptyVec};

        pub(crate) mod internals;
    }
    _ => {}
}

pub(crate) mod cmp;

#[cfg(feature = "std")]
pub(crate) mod io;

#[cfg(feature = "ownership")]
pub(crate) mod ownership;

#[cfg(feature = "serde")]
pub(crate) mod serde;
