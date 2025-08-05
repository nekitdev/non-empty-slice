//! Non-empty [`Vec<T>`].

#[cfg(not(any(feature = "std", feature = "alloc")))]
compile_error!("expected either `std` or `alloc` to be enabled");

#[cfg(all(not(feature = "std"), feature = "alloc"))]
use alloc::{borrow::ToOwned, vec::Vec};

use core::{borrow::Borrow, fmt, ops::Deref};

use thiserror::Error;

use crate::slice::Slice;

/// The error message used when the owned slice is empty.
pub const EMPTY_OWNED: &str = "the owned slice is empty";

/// Similar to [`Empty`], but holds the empty slice provided.
///
/// [`Empty`]: crate::slice::Empty
#[derive(Error)]
#[error("{EMPTY_OWNED}")]
#[cfg_attr(
    feature = "diagnostics",
    derive(miette::Diagnostic),
    diagnostic(
        code(non_empty_slice::owned),
        help("make sure the owned slice is non-empty")
    )
)]
pub struct EmptyOwned<T> {
    slice: Vec<T>,
}

const NAME: &str = "EmptyOwned";
const SLICE: &str = "slice";

struct EmptySlice;

impl fmt::Debug for EmptySlice {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.debug_list().finish()
    }
}

impl<T> fmt::Debug for EmptyOwned<T> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct(NAME)
            .field(SLICE, &EmptySlice)
            .finish()
    }
}

impl<T> EmptyOwned<T> {
    // NOTE: this is private to prevent creating this error with non-empty slices
    const fn new(slice: Vec<T>) -> Self {
        Self { slice }
    }

    /// Returns the contained empty slice.
    #[must_use]
    pub fn get(self) -> Vec<T> {
        self.slice
    }
}

/// Represents non-empty owned bytes, [`OwnedSlice<u8>`].
pub type OwnedBytes = OwnedSlice<u8>;

/// Represents non-empty [`Vec<T>`] values.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct OwnedSlice<T> {
    inner: Vec<T>,
}

impl<T> Borrow<Slice<T>> for OwnedSlice<T> {
    fn borrow(&self) -> &Slice<T> {
        self.as_slice()
    }
}

impl<T> TryFrom<Vec<T>> for OwnedSlice<T> {
    type Error = EmptyOwned<T>;

    fn try_from(value: Vec<T>) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

impl<T> From<OwnedSlice<T>> for Vec<T> {
    fn from(owned: OwnedSlice<T>) -> Self {
        owned.get()
    }
}

impl<T: Clone> From<&Slice<T>> for OwnedSlice<T> {
    fn from(slice: &Slice<T>) -> Self {
        Self::from_slice(slice)
    }
}

impl<T> AsRef<Self> for OwnedSlice<T> {
    fn as_ref(&self) -> &Self {
        self
    }
}

impl<T> AsRef<Vec<T>> for OwnedSlice<T> {
    fn as_ref(&self) -> &Vec<T> {
        self.as_owned()
    }
}

impl<T> AsRef<Slice<T>> for OwnedSlice<T> {
    fn as_ref(&self) -> &Slice<T> {
        self.as_slice()
    }
}

impl<T> AsRef<[T]> for OwnedSlice<T> {
    fn as_ref(&self) -> &[T] {
        self.as_slice().get()
    }
}

impl<T> Deref for OwnedSlice<T> {
    type Target = Vec<T>;

    fn deref(&self) -> &Self::Target {
        self.as_owned()
    }
}

impl<T> OwnedSlice<T> {
    /// Constructs [`Self`], provided that the [`Vec<T>`] is non-empty.
    ///
    /// # Errors
    ///
    /// Returns [`EmptyOwned<T>`] if the slice is empty.
    ///
    /// # Examples
    ///
    /// Basic snippet:
    ///
    /// ```
    /// use non_empty_slice::OwnedSlice;
    ///
    /// let non_empty = OwnedSlice::new(vec![1, 2, 3]).unwrap();
    /// ```
    ///
    /// Handling possible errors and recovering empty slices:
    ///
    /// ```
    /// use non_empty_slice::OwnedBytes;
    ///
    /// let empty_owned = OwnedBytes::new(Vec::new()).unwrap_err();
    ///
    /// let empty = empty_owned.get();
    /// ```
    pub const fn new(slice: Vec<T>) -> Result<Self, EmptyOwned<T>> {
        if slice.is_empty() {
            return Err(EmptyOwned::new(slice));
        }

        // SAFETY: the slice is non-empty at this point
        Ok(unsafe { Self::new_unchecked(slice) })
    }

    /// Constructs [`Self`] without checking that the slice is non-empty.
    ///
    /// # Safety
    ///
    /// The caller must ensure that the slice is non-empty.
    #[must_use]
    pub const unsafe fn new_unchecked(inner: Vec<T>) -> Self {
        Self { inner }
    }

    #[cfg(feature = "unsafe-assert")]
    const fn assert_non_empty(&self) {
        use core::hint::assert_unchecked;

        // SAFETY: the slice is non-empty by construction
        unsafe {
            assert_unchecked(!self.inner.is_empty());
        }
    }

    /// Returns the contained slice reference as [`Slice<T>`].
    #[must_use]
    pub const fn as_slice(&self) -> &Slice<T> {
        // SAFETY: the slice is non-empty by construction
        unsafe { Slice::from_slice_unchecked(self.inner.as_slice()) }
    }

    /// Returns the contained slice reference.
    #[must_use]
    pub const fn as_owned(&self) -> &Vec<T> {
        #[cfg(feature = "unsafe-assert")]
        self.assert_non_empty();

        &self.inner
    }

    /// Returns the contained [`Vec<T>`].
    #[must_use]
    pub fn get(self) -> Vec<T> {
        #[cfg(feature = "unsafe-assert")]
        self.assert_non_empty();

        self.inner
    }
}

impl<T: Clone> OwnedSlice<T> {
    /// Constructs [`Self`] from [`Slice<T>`] via cloning.
    ///
    /// # Examples
    ///
    /// Basic snippet:
    ///
    /// ```
    /// use non_empty_slice::{OwnedBytes, Bytes};
    ///
    /// let nekit = Bytes::from_slice(b"nekit").unwrap();
    ///
    /// let owned = OwnedBytes::from_slice(nekit);
    /// ```
    pub fn from_slice(slice: &Slice<T>) -> Self {
        // SAFETY: the slice is non-empty by construction
        unsafe { Self::new_unchecked(slice.get().to_owned()) }
    }
}

#[cfg(feature = "ownership")]
mod ownership {
    use ownership::IntoOwned;

    use super::OwnedSlice;

    impl<T: IntoOwned> IntoOwned for OwnedSlice<T> {
        type Owned = OwnedSlice<T::Owned>;

        fn into_owned(self) -> Self::Owned {
            // SAFETY: `into_owned` does not affect slice non-emptiness
            unsafe { OwnedSlice::new_unchecked(self.get().into_owned()) }
        }
    }
}
