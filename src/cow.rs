//! Non-empty [`Cow<'_, [T]>`](Cow).

#[cfg(not(any(feature = "std", feature = "alloc")))]
compile_error!("expected either `std` or `alloc` to be enabled");

#[cfg(feature = "unsafe-assert")]
use core::hint::assert_unchecked;

use core::ops::Deref;

#[cfg(feature = "std")]
use std::borrow::Cow;

#[cfg(all(not(feature = "std"), feature = "alloc"))]
use alloc::{borrow::Cow, vec::Vec};

use const_macros::{const_early, const_ok, const_quick};

#[cfg(feature = "static")]
use into_static::IntoStatic;

#[cfg(feature = "serde")]
use serde::{Deserialize, Deserializer, Serialize, Serializer, de::Error};

use crate::{empty::Empty, owned::OwnedSlice, slice::Slice};

/// Represents non-empty clone-on-write slices.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CowSlice<'a, T: Clone> {
    value: Cow<'a, [T]>,
}

#[cfg(feature = "serde")]
impl<T: Clone + Serialize> Serialize for CowSlice<'_, T> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        self.get().serialize(serializer)
    }
}

#[cfg(feature = "serde")]
impl<'de, T: Clone + Deserialize<'de>> Deserialize<'de> for CowSlice<'_, T> {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = Cow::deserialize(deserializer)?;

        Self::new(value).map_err(Error::custom)
    }
}

impl<T: Clone> AsRef<[T]> for CowSlice<'_, T> {
    fn as_ref(&self) -> &[T] {
        self.get()
    }
}

impl<T: Clone> Deref for CowSlice<'_, T> {
    type Target = [T];

    fn deref(&self) -> &Self::Target {
        self.get()
    }
}

impl<'a, T: Clone> TryFrom<Cow<'a, [T]>> for CowSlice<'a, T> {
    type Error = Empty;

    fn try_from(value: Cow<'a, [T]>) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

impl<'a, T: Clone> TryFrom<&'a [T]> for CowSlice<'a, T> {
    type Error = Empty;

    fn try_from(value: &'a [T]) -> Result<Self, Self::Error> {
        Self::borrowed(value)
    }
}

impl<T: Clone> TryFrom<Vec<T>> for CowSlice<'_, T> {
    type Error = Empty;

    fn try_from(value: Vec<T>) -> Result<Self, Self::Error> {
        Self::owned(value)
    }
}

impl<'a, T: Clone> From<Slice<'a, T>> for CowSlice<'a, T> {
    fn from(slice: Slice<'a, T>) -> Self {
        Self::from_slice(slice)
    }
}

impl<T: Clone> From<OwnedSlice<T>> for CowSlice<'_, T> {
    fn from(value: OwnedSlice<T>) -> Self {
        Self::from_owned_slice(value)
    }
}

impl<'a, T: Clone> From<CowSlice<'a, T>> for Cow<'a, [T]> {
    fn from(value: CowSlice<'a, T>) -> Self {
        value.take()
    }
}

impl<'a, T: Clone> CowSlice<'a, T> {
    /// Constructs [`Self`], providing that the value is non-empty.
    ///
    /// # Errors
    ///
    /// Returns [`Empty`] if the value is empty.
    pub fn new(value: Cow<'a, [T]>) -> Result<Self, Empty> {
        const_early!(value.is_empty() => Empty);

        // SAFETY: the value is non-empty at this point
        Ok(unsafe { Self::new_unchecked(value) })
    }

    /// Similar to [`new`], except the error is discarded.
    ///
    /// [`new`]: Self::new
    pub fn new_ok(value: Cow<'a, [T]>) -> Option<Self> {
        const_ok!(Self::new(value))
    }

    /// Constructs [`Self`] without checking if the value is non-empty.
    ///
    /// # Safety
    ///
    /// The caller must ensure that the value is non-empty.
    pub const unsafe fn new_unchecked(value: Cow<'a, [T]>) -> Self {
        Self { value }
    }

    /// Similar to [`new`], but accepts borrowed data.
    ///
    /// # Errors
    ///
    /// Returns [`Empty`] if the value is empty.
    ///
    /// [`new`]: Self::new
    pub const fn borrowed(value: &'a [T]) -> Result<Self, Empty> {
        const_early!(value.is_empty() => Empty);

        // SAFETY: the value is non-empty at this point
        Ok(unsafe { Self::borrowed_unchecked(value) })
    }

    /// Similar to [`borrowed`], but the error is discarded.
    ///
    /// [`borrowed`]: Self::borrowed
    pub const fn borrowed_ok(value: &'a [T]) -> Option<Self> {
        const_quick!(value.is_empty());

        // SAFETY: the value is non-empty at this point
        Some(unsafe { Self::borrowed_unchecked(value) })
    }

    /// Similar to [`new_unchecked`], but accepts borrowed data.
    ///
    /// # Safety
    ///
    /// The caller must ensure that the value is non-empty.
    ///
    /// [`new_unchecked`]: Self::new_unchecked
    pub const unsafe fn borrowed_unchecked(value: &'a [T]) -> Self {
        // SAFETY: the caller must ensure that the value is non-empty
        unsafe { Self::new_unchecked(Cow::Borrowed(value)) }
    }

    /// Similar to [`new`], but accepts owned data.
    ///
    /// # Errors
    ///
    /// Returns [`Empty`] if the provided value is empty.
    ///
    /// [`new`]: Self::new
    pub fn owned(value: Vec<T>) -> Result<Self, Empty> {
        const_early!(value.is_empty() => Empty);

        Ok(unsafe { Self::owned_unchecked(value) })
    }

    /// Similar to [`owned`], except the error is discarded.
    ///
    /// [`owned`]: Self::owned
    pub fn owned_ok(value: Vec<T>) -> Option<Self> {
        const_ok!(Self::owned(value))
    }

    /// Similar to [`new_unchecked`], but accepts owned data.
    ///
    /// # Safety
    ///
    /// The caller must ensure that the value is non-empty.
    ///
    /// [`new_unchecked`]: Self::new_unchecked
    pub const unsafe fn owned_unchecked(value: Vec<T>) -> Self {
        // SAFETY: the caller must ensure that the value is non-empty
        unsafe { Self::new_unchecked(Cow::Owned(value)) }
    }

    #[cfg(feature = "unsafe-assert")]
    fn assert_non_empty(&self) {
        // SAFETY: the value is non-empty by construction
        unsafe {
            assert_unchecked(!self.value.is_empty());
        }
    }

    /// Consumes [`Self`] and returns the wrapped data.
    pub fn take(self) -> Cow<'a, [T]> {
        #[cfg(feature = "unsafe-assert")]
        self.assert_non_empty();

        self.value
    }

    /// Constructs [`Self`] from [`Slice<'a, T>`](Slice).
    pub const fn from_slice(slice: Slice<'a, T>) -> Self {
        // SAFETY: the contained slice is non-empty
        unsafe { Self::borrowed_unchecked(slice.take()) }
    }

    /// Constructs [`Self`] from [`OwnedSlice<T>`](OwnedSlice).
    pub fn from_owned_slice(slice: OwnedSlice<T>) -> Self {
        // SAFETY: the contained slice is non-empty
        unsafe { Self::owned_unchecked(slice.take()) }
    }
}

impl<T: Clone> CowSlice<'_, T> {
    /// Returns the wrapped slice.
    pub fn get(&self) -> &[T] {
        #[cfg(feature = "unsafe-assert")]
        self.assert_non_empty();

        self.value.as_ref()
    }
}

/// An alias for [`CowSlice<'static, T>`](CowSlice).
#[cfg(feature = "static")]
pub type StaticCowSlice<T> = CowSlice<'static, T>;

#[cfg(feature = "static")]
impl<T: Clone + IntoStatic<Static: Clone>> IntoStatic for CowSlice<'_, T> {
    type Static = StaticCowSlice<T::Static>;

    fn into_static(self) -> Self::Static {
        // SAFETY: the contained slice is non-empty
        unsafe { Self::Static::new_unchecked(self.take().into_static()) }
    }
}
