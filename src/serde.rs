#[cfg(not(feature = "serde"))]
compile_error!("expected `serde` to be enabled");

#[cfg(all(not(feature = "std"), feature = "alloc"))]
use alloc::vec::Vec;

#[cfg(any(feature = "std", feature = "alloc"))]
use serde::{Deserialize, Deserializer, de::Error};

use serde::{Serialize, Serializer};

use crate::slice::NonEmptySlice;

#[cfg(any(feature = "std", feature = "alloc"))]
use crate::{boxed::NonEmptyBoxedSlice, vec::NonEmptyVec};

impl<T: Serialize> Serialize for NonEmptySlice<T> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        self.as_slice().serialize(serializer)
    }
}

#[cfg(any(feature = "std", feature = "alloc"))]
impl<T: Serialize> Serialize for NonEmptyVec<T> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        self.as_vec().serialize(serializer)
    }
}

#[cfg(any(feature = "std", feature = "alloc"))]
impl<'de, T: Deserialize<'de>> Deserialize<'de> for NonEmptyVec<T> {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let maybe_empty = Vec::deserialize(deserializer)?;

        Self::new(maybe_empty).map_err(D::Error::custom)
    }
}

// NOTE: `Serialize` is implemented for `Box<U>`, provided `U: Serialize`
// `NonEmptySlice<T>` is `Serialize`, therefore `NonEmptyBoxedSlice<T>` is as well

#[cfg(any(feature = "std", feature = "alloc"))]
impl<'de, T: Deserialize<'de>> Deserialize<'de> for NonEmptyBoxedSlice<T> {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let non_empty_vec = NonEmptyVec::deserialize(deserializer)?;

        Ok(non_empty_vec.into_non_empty_boxed_slice())
    }
}
