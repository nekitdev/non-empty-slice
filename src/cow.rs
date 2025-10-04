//! Non-empty [`Cow<'_, [T]>`](Cow).

#[cfg(not(any(feature = "std", feature = "alloc")))]
compile_error!("expected either `std` or `alloc` to be enabled");

#[cfg(feature = "std")]
use std::borrow::Cow;

#[cfg(all(not(feature = "std"), feature = "alloc"))]
use alloc::borrow::Cow;

use crate::{boxed::NonEmptyBoxedSlice, slice::NonEmptySlice, vec::NonEmptyVec};

/// Represents non-empty clone-on-write slices, [`Cow<'a, NonEmptySlice<T>>`](Cow).
pub type NonEmptyCowSlice<'a, T> = Cow<'a, NonEmptySlice<T>>;

impl<T: Clone> From<NonEmptyCowSlice<'_, T>> for NonEmptyVec<T> {
    fn from(non_empty: NonEmptyCowSlice<'_, T>) -> Self {
        non_empty.into_owned()
    }
}

impl<T: Clone> From<NonEmptyCowSlice<'_, T>> for NonEmptyBoxedSlice<T> {
    fn from(non_empty: NonEmptyCowSlice<'_, T>) -> Self {
        non_empty.into_owned().into_non_empty_boxed_slice()
    }
}

impl<'a, T: Clone> From<&'a NonEmptySlice<T>> for NonEmptyCowSlice<'a, T> {
    fn from(non_empty: &'a NonEmptySlice<T>) -> Self {
        Self::Borrowed(non_empty)
    }
}

impl<T: Clone> From<NonEmptyVec<T>> for NonEmptyCowSlice<'_, T> {
    fn from(non_empty: NonEmptyVec<T>) -> Self {
        Self::Owned(non_empty)
    }
}

impl<'a, T: Clone> From<&'a NonEmptyVec<T>> for NonEmptyCowSlice<'a, T> {
    fn from(non_empty: &'a NonEmptyVec<T>) -> Self {
        Self::Borrowed(non_empty.as_non_empty_slice())
    }
}
