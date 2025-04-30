//! Non-empty slices.

#[cfg(feature = "unsafe-assert")]
use core::hint::assert_unchecked;

use core::ops::Deref;

use const_macros::{const_early, const_ok};

#[cfg(feature = "serde")]
use serde::{Serialize, Serializer};

use crate::empty::Empty;

/// Represents non-empty slices.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Slice<'a, T> {
    value: &'a [T],
}

#[cfg(feature = "serde")]
impl<T: Serialize> Serialize for Slice<'_, T> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        self.get().serialize(serializer)
    }
}

impl<'a, T> TryFrom<&'a [T]> for Slice<'a, T> {
    type Error = Empty;

    fn try_from(value: &'a [T]) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

impl<'a, T> From<Slice<'a, T>> for &'a [T] {
    fn from(slice: Slice<'a, T>) -> Self {
        slice.take()
    }
}

impl<T> AsRef<[T]> for Slice<'_, T> {
    fn as_ref(&self) -> &[T] {
        self.get()
    }
}

impl<T> Deref for Slice<'_, T> {
    type Target = [T];

    fn deref(&self) -> &Self::Target {
        self.get()
    }
}

impl<'a, T> Slice<'a, T> {
    pub const fn new(value: &'a [T]) -> Result<Self, Empty> {
        const_early!(value.is_empty() => Empty);

        // SAFETY: the value is non-empty at this point
        Ok(unsafe { Self::new_unchecked(value) })
    }

    pub const fn new_ok(value: &'a [T]) -> Option<Self> {
        const_ok!(Self::new(value))
    }

    pub const unsafe fn new_unchecked(value: &'a [T]) -> Self {
        Self { value }
    }

    #[cfg(feature = "unsafe-assert")]
    const fn assert_non_empty(&self) {
        unsafe {
            assert_unchecked(!self.value.is_empty());
        }
    }

    pub const fn take(self) -> &'a [T] {
        #[cfg(feature = "unsafe-assert")]
        self.assert_non_empty();

        self.value
    }
}

#[cfg(feature = "static")]
pub type StaticSlice<T> = Slice<'static, T>;

impl<T> Slice<'_, T> {
    pub const fn get(&self) -> &[T] {
        #[cfg(feature = "unsafe-assert")]
        self.assert_non_empty();

        self.value
    }
}
