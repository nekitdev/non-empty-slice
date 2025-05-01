//! Non-empty [`Vec<T>`].

#[cfg(not(any(feature = "std", feature = "alloc")))]
compile_error!("expected either `std` or `alloc` to be enabled");

#[cfg(feature = "unsafe-assert")]
use core::hint::assert_unchecked;

use core::ops::Deref;

#[cfg(all(not(feature = "std"), feature = "alloc"))]
use alloc::{borrow::ToOwned, vec::Vec};

use const_macros::{const_early, const_ok};

#[cfg(feature = "static")]
use into_static::IntoStatic;

#[cfg(feature = "serde")]
use serde::{Deserialize, Deserializer, Serialize, Serializer, de::Error};

use crate::{cow::CowSlice, empty::Empty, slice::Slice};

/// Represents non-empty owned slices.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct OwnedSlice<T> {
    value: Vec<T>,
}

#[cfg(feature = "serde")]
impl<T: Serialize> Serialize for OwnedSlice<T> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        self.get().serialize(serializer)
    }
}

#[cfg(feature = "serde")]
impl<'de, T: Deserialize<'de>> Deserialize<'de> for OwnedSlice<T> {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = Vec::deserialize(deserializer)?;

        Self::new(value).map_err(Error::custom)
    }
}

impl<T> TryFrom<Vec<T>> for OwnedSlice<T> {
    type Error = Empty;

    fn try_from(value: Vec<T>) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

impl<T> From<OwnedSlice<T>> for Vec<T> {
    fn from(owned: OwnedSlice<T>) -> Self {
        owned.take()
    }
}

impl<T: Clone> From<Slice<'_, T>> for OwnedSlice<T> {
    fn from(slice: Slice<'_, T>) -> Self {
        Self::from_slice(slice)
    }
}

impl<T: Clone> From<CowSlice<'_, T>> for OwnedSlice<T> {
    fn from(cow: CowSlice<'_, T>) -> Self {
        Self::from_cow_slice(cow)
    }
}

impl<T> Deref for OwnedSlice<T> {
    type Target = [T];

    fn deref(&self) -> &Self::Target {
        self.get()
    }
}

impl<T> OwnedSlice<T> {
    /// Constructs [`Self`], provided that the input is non-empty.
    ///
    /// # Errors
    ///
    /// Returns [`Empty`] if the input is empty.
    pub fn new(value: Vec<T>) -> Result<Self, Empty> {
        const_early!(value.is_empty() => Empty);

        // SAFETY: the value is non-empty at this point
        Ok(unsafe { Self::new_unchecked(value) })
    }

    /// Similar to [`new`], but the error is discarded.
    ///
    /// [`new`]: Self::new
    pub fn new_ok(value: Vec<T>) -> Option<Self> {
        const_ok!(Self::new(value))
    }

    /// Constructs [`Self`] without checking that the input is non-empty.
    ///
    /// # Safety
    ///
    /// The caller must ensure that the value is non-empty.
    pub const unsafe fn new_unchecked(value: Vec<T>) -> Self {
        Self { value }
    }

    #[cfg(feature = "unsafe-assert")]
    fn assert_non_empty(&self) {
        // SAFETY: the value is non-empty by construction
        unsafe {
            assert_unchecked(!self.value.is_empty());
        }
    }

    /// Consumes [`Self`] and returns the contained [`Vec<T>`].
    pub fn take(self) -> Vec<T> {
        #[cfg(feature = "unsafe-assert")]
        self.assert_non_empty();

        self.value
    }

    /// Returns the contained slice.
    pub fn get(&self) -> &[T] {
        #[cfg(feature = "unsafe-assert")]
        self.assert_non_empty();

        self.value.as_slice()
    }
}

impl<T: Clone> OwnedSlice<T> {
    /// Constructs [`Self`] from [`Slice<'_, T>`](Slice) via cloning.
    pub fn from_slice(value: Slice<'_, T>) -> Self {
        // SAFETY: the value is non-empty by construction
        unsafe { Self::new_unchecked(value.take().to_owned()) }
    }

    /// Constructs [`Self`] from [`CowSlice<'_, T>`](CowSlice) via (optionally) cloning.
    pub fn from_cow_slice(value: CowSlice<'_, T>) -> Self {
        // SAFETY: the value is non-empty by construction
        unsafe { Self::new_unchecked(value.take().into_owned()) }
    }
}

#[cfg(feature = "static")]
impl<T: Clone + IntoStatic<Static: Clone>> IntoStatic for OwnedSlice<T> {
    type Static = OwnedSlice<T::Static>;

    fn into_static(self) -> Self::Static {
        // SAFETY: the value is non-empty by construction
        unsafe { Self::Static::new_unchecked(self.take().into_static()) }
    }
}
