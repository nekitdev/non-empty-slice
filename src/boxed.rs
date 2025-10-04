//! Non-empty [`Box<[T]>`](Box).

#[cfg(not(any(feature = "std", feature = "alloc")))]
compile_error!("expected either `std` or `alloc` to be enabled");

use core::mem::MaybeUninit;

#[cfg(feature = "std")]
use std::vec::IntoIter;

#[cfg(all(not(feature = "std"), feature = "alloc"))]
use alloc::{
    boxed::Box,
    vec::{IntoIter, Vec},
};

use non_empty_iter::{FromNonEmptyIterator, IntoNonEmptyIterator};
use non_zero_size::Size;
use thiserror::Error;

use crate::{
    format,
    iter::IntoNonEmptyIter,
    slice::{EmptySlice, NonEmptyMaybeUninitSlice, NonEmptySlice},
    vec::{EmptyVec, NonEmptyVec},
};

/// Represents non-empty boxed slices, [`Box<NonEmptySlice<T>>`].
pub type NonEmptyBoxedSlice<T> = Box<NonEmptySlice<T>>;

/// Represents non-empty boxed slices of possibly uninitialized values,
/// [`NonEmptyBoxedSlice<MaybeUninit<T>>`].
pub type NonEmptyMaybeUninitBoxedSlice<T> = NonEmptyBoxedSlice<MaybeUninit<T>>;

/// Represents non-empty boxed bytes, [`NonEmptyBoxedSlice<u8>`].
pub type NonEmptyBoxedBytes = NonEmptyBoxedSlice<u8>;

/// The error message used when the boxed slice is empty.
pub const EMPTY_BOXED: &str = "the boxed slice is empty";

/// Similar to [`EmptyVec<T>`], but contains the empty boxed slice provided.
#[derive(Error)]
#[error("{EMPTY_BOXED}")]
#[cfg_attr(
    feature = "diagnostics",
    derive(miette::Diagnostic),
    diagnostic(
        code(non_empty_slice::boxed),
        help("make sure the boxed slice is non-empty")
    )
)]
pub struct EmptyBoxedSlice<T> {
    boxed: Box<[T]>,
}

format::debug!(EmptyBoxedSlice, boxed);

/// Represents empty boxed bytes, [`EmptyBoxedSlice<u8>`].
pub type EmptyBoxedBytes = EmptyBoxedSlice<u8>;

impl<T> EmptyBoxedSlice<T> {
    pub(crate) const fn new(boxed: Box<[T]>) -> Self {
        Self { boxed }
    }

    /// Returns the contained empty boxed slice.
    #[must_use]
    pub fn get(self) -> Box<[T]> {
        self.boxed
    }

    /// Constructs [`Self`] from [`EmptyVec<T>`].
    #[must_use]
    pub fn from_empty_vec(empty: EmptyVec<T>) -> Self {
        Self::new(empty.get().into_boxed_slice())
    }

    /// Converts [`Self`] into [`EmptyVec<T>`].
    #[must_use]
    pub fn into_empty_vec(self) -> EmptyVec<T> {
        EmptyVec::from_empty_boxed_slice(self)
    }
}

impl<T> From<NonEmptyBoxedSlice<T>> for Box<[T]> {
    fn from(boxed: NonEmptyBoxedSlice<T>) -> Self {
        boxed.into_boxed_slice()
    }
}

impl<T> TryFrom<Box<[T]>> for NonEmptyBoxedSlice<T> {
    type Error = EmptyBoxedSlice<T>;

    fn try_from(boxed: Box<[T]>) -> Result<Self, Self::Error> {
        NonEmptySlice::from_boxed_slice(boxed)
    }
}

impl<T> TryFrom<Vec<T>> for NonEmptyBoxedSlice<T> {
    type Error = EmptyVec<T>;

    fn try_from(vec: Vec<T>) -> Result<Self, Self::Error> {
        let non_empty_vec = NonEmptyVec::new(vec)?;

        Ok(non_empty_vec.into())
    }
}

impl<T> From<NonEmptyBoxedSlice<T>> for Vec<T> {
    fn from(boxed: NonEmptyBoxedSlice<T>) -> Self {
        boxed.into_boxed_slice().into_vec()
    }
}

impl<T> From<NonEmptyBoxedSlice<T>> for NonEmptyVec<T> {
    fn from(boxed: NonEmptyBoxedSlice<T>) -> Self {
        boxed.into_non_empty_vec()
    }
}

impl<T> From<NonEmptyVec<T>> for NonEmptyBoxedSlice<T> {
    fn from(non_empty: NonEmptyVec<T>) -> Self {
        non_empty.into_non_empty_boxed_slice()
    }
}

impl<T: Clone> TryFrom<&[T]> for NonEmptyBoxedSlice<T> {
    type Error = EmptySlice;

    fn try_from(slice: &[T]) -> Result<Self, Self::Error> {
        let non_empty_slice = NonEmptySlice::try_from_slice(slice)?;

        Ok(non_empty_slice.into())
    }
}

impl<T: Clone> TryFrom<&mut [T]> for NonEmptyBoxedSlice<T> {
    type Error = EmptySlice;

    fn try_from(slice: &mut [T]) -> Result<Self, Self::Error> {
        let non_empty_slice = NonEmptySlice::try_from_mut_slice(slice)?;

        Ok(non_empty_slice.into())
    }
}

impl<T: Clone> From<&NonEmptySlice<T>> for NonEmptyBoxedSlice<T> {
    fn from(non_empty: &NonEmptySlice<T>) -> Self {
        non_empty.to_non_empty_vec().into_non_empty_boxed_slice()
    }
}

impl<T: Clone> From<&mut NonEmptySlice<T>> for NonEmptyBoxedSlice<T> {
    fn from(non_empty: &mut NonEmptySlice<T>) -> Self {
        non_empty.to_non_empty_vec().into_non_empty_boxed_slice()
    }
}

impl<T: Clone> Clone for NonEmptyBoxedSlice<T> {
    fn clone(&self) -> Self {
        self.to_non_empty_vec().into_non_empty_boxed_slice()
    }

    fn clone_from(&mut self, source: &Self) {
        if self.len() == source.len() {
            self.clone_from_non_empty_slice(source);
        } else {
            *self = source.clone();
        }
    }
}

impl<T> NonEmptySlice<T> {
    /// Constructs [`Self`] from [`Box<[T]>`](Box), provided the boxed slice is non-empty.
    ///
    /// # Errors
    ///
    /// Returns [`EmptyBoxedSlice<T>`] if the boxed slice is empty.
    pub fn from_boxed_slice(boxed: Box<[T]>) -> Result<Box<Self>, EmptyBoxedSlice<T>> {
        if boxed.is_empty() {
            return Err(EmptyBoxedSlice::new(boxed));
        }

        // SAFETY: the boxed slice is non-empty at this point
        Ok(unsafe { Self::from_boxed_slice_unchecked(boxed) })
    }

    /// Constructs [`Self`] from [`Box<[T]>`](Box), without checking if the boxed slice is empty.
    ///
    /// # Safety
    ///
    /// The caller must ensure that the boxed slice is non-empty.
    #[must_use]
    pub unsafe fn from_boxed_slice_unchecked(boxed: Box<[T]>) -> Box<Self> {
        // SAFETY: the caller must ensure that the boxed slice is non-empty
        // moreover, `Slice` is `repr(transparent)`, so it is safe to transmute
        // finally, `Box` is created from the raw pointer existing within this function only
        unsafe { Box::from_raw(Box::into_raw(boxed) as *mut Self) }
    }

    /// Converts [`Self`] into [`Box<[T]>`](Box).
    #[must_use]
    pub fn into_boxed_slice(self: Box<Self>) -> Box<[T]> {
        // SAFETY: `Slice` is `repr(transparent)`, so it is safe to transmute
        // moreover, `Box` is created from the raw pointer existing within this function only
        unsafe { Box::from_raw(Box::into_raw(self) as *mut [T]) }
    }

    /// Constructs [`Self`] from [`NonEmptyVec<T>`].
    #[must_use]
    pub fn from_non_empty_vec(non_empty: NonEmptyVec<T>) -> Box<Self> {
        // SAFETY: the vector is non-empty by construction, so is the underlying boxed slice
        unsafe { Self::from_boxed_slice_unchecked(non_empty.into_vec().into_boxed_slice()) }
    }

    /// Converts [`Self`] into [`NonEmptyVec<T>`].
    #[must_use]
    pub fn into_non_empty_vec(self: Box<Self>) -> NonEmptyVec<T> {
        NonEmptyVec::from_non_empty_boxed_slice(self)
    }

    /// Constructs uninitialized [`NonEmptyMaybeUninitBoxedSlice<T>`] of given non-zero length.
    #[must_use]
    pub fn new_uninit(len: Size) -> NonEmptyMaybeUninitBoxedSlice<T> {
        let boxed = Box::new_uninit_slice(len.get());

        // SAFETY: `len` is non-zero, therefore this is safe
        unsafe { NonEmptySlice::from_boxed_slice_unchecked(boxed) }
    }
}

impl<T> FromNonEmptyIterator<T> for NonEmptyBoxedSlice<T> {
    fn from_non_empty_iter<I: IntoNonEmptyIterator<Item = T>>(iterable: I) -> Self {
        let non_empty_vec = NonEmptyVec::from_non_empty_iter(iterable);

        non_empty_vec.into_non_empty_boxed_slice()
    }
}

impl<T> NonEmptyMaybeUninitSlice<T> {
    /// Converts [`Self`] into initialized [`NonEmptyBoxedSlice<T>`].
    ///
    /// # Safety
    ///
    /// The caller must guarantee that the items are in initialized state.
    /// Calling this when the contents are not fully initialized causes
    /// *immediate undefined behavior*.
    #[must_use]
    pub unsafe fn assume_init(self: Box<Self>) -> NonEmptyBoxedSlice<T> {
        // SAFETY: the caller must guarantee that the items are in initialized state
        let boxed = unsafe { self.into_boxed_slice().assume_init() };

        // SAFETY: `self` is non-empty, so is `boxed`, therefore this is safe
        unsafe { NonEmptySlice::from_boxed_slice_unchecked(boxed) }
    }
}

impl<T> NonEmptyVec<T> {
    /// Constructs [`Self`] from [`NonEmptyBoxedSlice<T>`].
    #[must_use]
    pub fn from_non_empty_boxed_slice(non_empty: NonEmptyBoxedSlice<T>) -> Self {
        // SAFETY: the boxed slice is non-empty by construction
        unsafe { Self::new_unchecked(non_empty.into_boxed_slice().into_vec()) }
    }

    /// Converts [`Self`] into [`NonEmptyBoxedSlice<T>`].
    #[must_use]
    pub fn into_non_empty_boxed_slice(self) -> NonEmptyBoxedSlice<T> {
        NonEmptySlice::from_non_empty_vec(self)
    }

    /// Converts [`Self`] into [`Box<[T]>`](Box).
    #[must_use]
    pub fn into_boxed_slice(self) -> Box<[T]> {
        self.into_non_empty_boxed_slice().into_boxed_slice()
    }
}

impl<T> IntoIterator for NonEmptyBoxedSlice<T> {
    type Item = T;

    type IntoIter = IntoIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        self.into_non_empty_vec().into_iter()
    }
}

impl<T> IntoNonEmptyIterator for NonEmptyBoxedSlice<T> {
    type IntoNonEmptyIter = IntoNonEmptyIter<T>;

    fn into_non_empty_iter(self) -> Self::IntoNonEmptyIter {
        self.into_non_empty_vec().into_non_empty_iter()
    }
}
