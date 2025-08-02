//! Non-empty slices.

use core::ops::Deref;

use thiserror::Error;

/// The error message used when the slice is empty.
pub const EMPTY: &str = "the slice is empty";

/// Represents errors returned when received slices are empty.
#[derive(Debug, Error)]
#[error("{EMPTY}")]
#[cfg_attr(
    feature = "diagnostics",
    derive(miette::Diagnostic),
    diagnostic(code(non_empty_slice::slice), help("make sure the slice is non-empty"))
)]
pub struct Empty;

/// Represents non-empty bytes, [`Slice<u8>`].
pub type Bytes = Slice<u8>;

/// Represents non-empty slices.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct Slice<T> {
    inner: [T],
}

#[cfg(any(feature = "std", feature = "alloc"))]
mod owned {
    use crate::owned::OwnedSlice;

    #[cfg(all(not(feature = "std"), feature = "alloc"))]
    use alloc::borrow::ToOwned;

    use super::Slice;

    impl<T: Clone> ToOwned for Slice<T> {
        type Owned = OwnedSlice<T>;

        fn to_owned(&self) -> Self::Owned {
            Self::Owned::from_slice(self)
        }
    }
}

impl<'a, T> TryFrom<&'a [T]> for &'a Slice<T> {
    type Error = Empty;

    fn try_from(slice: &'a [T]) -> Result<Self, Self::Error> {
        Slice::try_from_slice(slice)
    }
}

impl<'a, T> From<&'a Slice<T>> for &'a [T] {
    fn from(slice: &'a Slice<T>) -> Self {
        slice.get()
    }
}

impl<T> AsRef<[T]> for Slice<T> {
    fn as_ref(&self) -> &[T] {
        self.get()
    }
}

impl<T> Deref for Slice<T> {
    type Target = [T];

    fn deref(&self) -> &Self::Target {
        self.get()
    }
}

impl<T> Slice<T> {
    /// Constructs [`Self`] from anything that can be converted to slice, provided it is non-empty.
    ///
    /// Prefer [`try_from_slice`] if only [`[T]`](slice) is used,
    /// as this allows for `const` evaluation.
    ///
    /// # Errors
    ///
    /// Returns [`Empty`] if the slice is empty.
    ///
    /// [`try_from_slice`]: Self::try_from_slice
    pub fn try_new<S: AsRef<[T]> + ?Sized>(slice: &S) -> Result<&Self, Empty> {
        Self::try_from_slice(slice.as_ref())
    }

    /// Similar to [`try_new`], but the error is discarded.
    ///
    /// Prefer [`from_slice`] if only [`[T]`](slice) is used, as it allows for `const` evaluation.
    ///
    /// # Examples
    ///
    /// ```
    /// use non_empty_slice::Slice;
    ///
    /// let array = [1, 2, 3];
    ///
    /// let non_empty = Slice::new(&array).unwrap();
    ///
    /// // `Slice<T>` is `AsRef<[T]>`, so it can also be used here!
    /// let from_non_empty = Slice::from_slice(non_empty).unwrap();
    /// ```
    ///
    /// [`try_new`]: Self::try_new
    /// [`from_slice`]: Self::from_slice
    pub fn new<S: AsRef<[T]> + ?Sized>(slice: &S) -> Option<&Self> {
        Self::from_slice(slice.as_ref())
    }

    /// Constructs [`Self`] from anything that can be converted to slice, without doing any checks.
    ///
    /// Prefer [`from_slice_unchecked`] if only [`[T]`](slice) is used,
    /// as this allows for `const` evaluation.
    ///
    /// # Safety
    ///
    /// The caller must ensure that the slice is non-empty.
    ///
    /// [`from_slice_unchecked`]: Self::from_slice_unchecked
    #[must_use]
    pub unsafe fn new_unchecked<S: AsRef<[T]> + ?Sized>(slice: &S) -> &Self {
        // SAFETY: the caller must ensure that the slice is non-empty
        unsafe { Self::from_slice_unchecked(slice.as_ref()) }
    }

    /// Constructs [`Self`] from [`[T]`](slice), provided the slice is non-empty.
    ///
    /// # Errors
    ///
    /// Returns [`Empty`] if the slice is empty.
    pub const fn try_from_slice(slice: &[T]) -> Result<&Self, Empty> {
        if slice.is_empty() {
            return Err(Empty);
        }

        // SAFETY: the slice is non-empty at this point
        Ok(unsafe { Self::from_slice_unchecked(slice) })
    }

    /// Similar to [`try_from_slice`], but the error is discarded.
    ///
    /// # Examples
    ///
    /// Basic snippet:
    ///
    /// ```
    /// use non_empty_slice::Slice;
    ///
    /// let array = [1, 2, 3];
    ///
    /// let non_empty = Slice::from_slice(&array).unwrap();
    /// ```
    ///
    /// [`None`] is returned if the slice is empty, therefore the following snippet panics:
    ///
    /// ```should_panic
    /// use non_empty_slice::Bytes;
    ///
    /// let empty = [];
    ///
    /// let never = Bytes::from_slice(&empty).unwrap();
    /// ```
    ///
    /// [`try_from_slice`]: Self::try_from_slice
    pub const fn from_slice(slice: &[T]) -> Option<&Self> {
        if slice.is_empty() {
            return None;
        }

        // SAFETY: the slice is non-empty at this point
        Some(unsafe { Self::from_slice_unchecked(slice) })
    }

    /// Constructs [`Self`] from [`[T]`](slice), without checking if the slice is empty.
    ///
    /// # Safety
    ///
    /// The caller must ensure that the slice is non-empty.
    #[must_use]
    pub const unsafe fn from_slice_unchecked(slice: &[T]) -> &Self {
        debug_assert!(!slice.is_empty());

        // SAFETY: the caller must ensure that the slice is non-empty
        // `Slice` is `repr(transparent)`, so it is safe to transmute
        #[allow(clippy::ref_as_ptr)]
        unsafe {
            &*(slice as *const [T] as *const Self)
        }
    }

    #[cfg(feature = "unsafe-assert")]
    const fn assert_non_empty(&self) {
        use core::hint::assert_unchecked;

        // SAFETY: the slice is non-empty by construction
        unsafe {
            assert_unchecked(!self.inner.is_empty());
        }
    }

    /// Returns the contained slice.
    ///
    /// # Examples
    ///
    /// ```
    /// use non_empty_slice::Bytes;
    ///
    /// let nekit = b"nekit";
    ///
    /// let non_empty = Bytes::from_slice(nekit).unwrap();
    ///
    /// assert_eq!(non_empty.get(), nekit);
    /// ```
    #[must_use]
    pub const fn get(&self) -> &[T] {
        #[cfg(feature = "unsafe-assert")]
        self.assert_non_empty();

        &self.inner
    }
}

#[cfg(feature = "serde")]
mod serde {
    use super::Slice;

    use serde::{Serialize, Serializer};

    impl<T: Serialize> Serialize for Slice<T> {
        fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
            self.get().serialize(serializer)
        }
    }
}
