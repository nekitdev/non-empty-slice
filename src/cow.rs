//! Non-empty [`Cow<'_, T>`](Cow).

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

impl<T: Clone> Deref for CowSlice<'_, T> {
    type Target = [T];

    fn deref(&self) -> &Self::Target {
        self.get()
    }
}

impl<'a, T: Clone> CowSlice<'a, T> {
    pub fn new(value: Cow<'a, [T]>) -> Result<Self, Empty> {
        const_early!(value.is_empty() => Empty);

        // SAFETY: the value is non-empty at this point
        Ok(unsafe { Self::new_unchecked(value) })
    }

    pub const unsafe fn new_unchecked(value: Cow<'a, [T]>) -> Self {
        Self { value }
    }

    pub fn new_ok(value: Cow<'a, [T]>) -> Option<Self> {
        const_ok!(Self::new(value))
    }

    pub const fn borrowed(value: &'a [T]) -> Result<Self, Empty> {
        const_early!(value.is_empty() => Empty);

        // SAFETY: the value is non-empty at this point
        Ok(unsafe { Self::borrowed_unchecked(value) })
    }

    pub const fn borrowed_ok(value: &'a [T]) -> Option<Self> {
        const_quick!(value.is_empty());

        // SAFETY: the value is non-empty at this point
        Some(unsafe { Self::borrowed_unchecked(value) })
    }

    pub const unsafe fn borrowed_unchecked(value: &'a [T]) -> Self {
        // SAFETY: the caller must ensure that the value is non-empty
        unsafe { Self::new_unchecked(Cow::Borrowed(value)) }
    }

    pub fn owned(value: Vec<T>) -> Result<Self, Empty> {
        const_early!(value.is_empty() => Empty);

        Ok(unsafe { Self::owned_unchecked(value) })
    }

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

    pub fn take(self) -> Cow<'a, [T]> {
        #[cfg(feature = "unsafe-assert")]
        self.assert_non_empty();

        self.value
    }

    pub const fn from_slice(slice: Slice<'a, T>) -> Self {
        // SAFETY: the contained slice is non-empty
        unsafe { Self::borrowed_unchecked(slice.take()) }
    }

    pub fn from_owned_slice(slice: OwnedSlice<T>) -> Self {
        // SAFETY: the contained slice is non-empty
        unsafe { Self::owned_unchecked(slice.take()) }
    }
}

impl<T: Clone> CowSlice<'_, T> {
    pub fn get(&self) -> &[T] {
        #[cfg(feature = "unsafe-assert")]
        self.assert_non_empty();

        self.value.as_ref()
    }
}

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
