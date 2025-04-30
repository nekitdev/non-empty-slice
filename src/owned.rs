//! Non-empty [`Vec<T>`].

#[cfg(not(any(feature = "std", feature = "alloc")))]
compile_error!("expected either `std` or `alloc` to be enabled");

#[cfg(feature = "unsafe-assert")]
use core::hint::assert_unchecked;

use core::ops::Deref;

#[cfg(all(not(feature = "std"), feature = "alloc"))]
use alloc::{borrow::ToOwned, vec::Vec};

use const_macros::{const_early, const_ok};

#[cfg(feature = "serde")]
use serde::{Deserialize, Deserializer, Serialize, Serializer, de::Error};

use crate::{cow::CowSlice, empty::Empty, slice::Slice};

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
    pub fn new(value: Vec<T>) -> Result<Self, Empty> {
        const_early!(value.is_empty() => Empty);

        Ok(unsafe { Self::new_unchecked(value) })
    }

    pub fn new_ok(value: Vec<T>) -> Option<Self> {
        const_ok!(Self::new(value))
    }

    pub const unsafe fn new_unchecked(value: Vec<T>) -> Self {
        Self { value }
    }

    #[cfg(feature = "unsafe-assert")]
    fn assert_non_empty(&self) {
        unsafe {
            assert_unchecked(!self.value.is_empty());
        }
    }

    pub fn take(self) -> Vec<T> {
        #[cfg(feature = "unsafe-assert")]
        self.assert_non_empty();

        self.value
    }

    pub fn get(&self) -> &[T] {
        #[cfg(feature = "unsafe-assert")]
        self.assert_non_empty();

        self.value.as_slice()
    }
}

impl<T: Clone> OwnedSlice<T> {
    pub fn from_slice(value: Slice<'_, T>) -> Self {
        unsafe { Self::new_unchecked(value.take().to_owned()) }
    }

    pub fn from_cow_slice(value: CowSlice<'_, T>) -> Self {
        unsafe { Self::new_unchecked(value.take().into_owned()) }
    }
}
