//! Non-empty [`[T]`](prim@slice).

use core::{
    array::TryFromSliceError,
    mem::MaybeUninit,
    ops::{Deref, DerefMut, Index, IndexMut, Range},
    ptr,
    slice::{Iter, IterMut, SliceIndex},
};

use non_empty_iter::{IntoNonEmptyIterator, NonEmptyAdapter};
use non_zero_size::Size;
use thiserror::Error;

use crate::iter::{
    ChunkBy, ChunkByMut, Chunks, ChunksExact, ChunksExactMut, ChunksMut, NonEmptyIter,
    NonEmptyIterMut, RChunks, RChunksExact, RChunksExactMut, RChunksMut, Windows,
};

/// The error message used when the slice is empty.
pub const EMPTY_SLICE: &str = "the slice is empty";

/// Represents errors returned when received slices are empty.
#[derive(Debug, Error)]
#[error("{EMPTY_SLICE}")]
#[cfg_attr(
    feature = "diagnostics",
    derive(miette::Diagnostic),
    diagnostic(code(non_empty_slice::slice), help("make sure the slice is non-empty"))
)]
pub struct EmptySlice;

/// Represents non-empty bytes, [`NonEmptySlice<u8>`].
pub type NonEmptyBytes = NonEmptySlice<u8>;

/// Represents non-empty slices.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct NonEmptySlice<T> {
    inner: [T],
}

/// Represents non-empty slices of possibly uninitialized values, [`NonEmptySlice<MaybeUninit<T>>`].
pub type NonEmptyMaybeUninitSlice<T> = NonEmptySlice<MaybeUninit<T>>;

#[cfg(any(feature = "std", feature = "alloc"))]
mod owned {
    use crate::vec::NonEmptyVec;

    #[cfg(all(not(feature = "std"), feature = "alloc"))]
    use alloc::borrow::ToOwned;

    use super::NonEmptySlice;

    impl<T: Clone> ToOwned for NonEmptySlice<T> {
        type Owned = NonEmptyVec<T>;

        fn to_owned(&self) -> Self::Owned {
            Self::Owned::from_non_empty_slice(self)
        }
    }
}

impl<'a, T> TryFrom<&'a [T]> for &'a NonEmptySlice<T> {
    type Error = EmptySlice;

    fn try_from(slice: &'a [T]) -> Result<Self, Self::Error> {
        NonEmptySlice::try_from_slice(slice)
    }
}

impl<'a, T> TryFrom<&'a mut [T]> for &'a mut NonEmptySlice<T> {
    type Error = EmptySlice;

    fn try_from(slice: &'a mut [T]) -> Result<Self, Self::Error> {
        NonEmptySlice::try_from_mut_slice(slice)
    }
}

impl<'a, T> From<&'a NonEmptySlice<T>> for &'a [T] {
    fn from(slice: &'a NonEmptySlice<T>) -> Self {
        slice.as_slice()
    }
}

impl<'a, T> From<&'a mut NonEmptySlice<T>> for &'a mut [T] {
    fn from(slice: &'a mut NonEmptySlice<T>) -> Self {
        slice.as_mut_slice()
    }
}

impl<T> AsRef<Self> for NonEmptySlice<T> {
    fn as_ref(&self) -> &Self {
        self
    }
}

impl<T> AsRef<[T]> for NonEmptySlice<T> {
    fn as_ref(&self) -> &[T] {
        self.as_slice()
    }
}

impl<T> AsMut<Self> for NonEmptySlice<T> {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl<T> AsMut<[T]> for NonEmptySlice<T> {
    fn as_mut(&mut self) -> &mut [T] {
        self.as_mut_slice()
    }
}

impl<T> Deref for NonEmptySlice<T> {
    type Target = [T];

    fn deref(&self) -> &Self::Target {
        self.as_slice()
    }
}

impl<T> DerefMut for NonEmptySlice<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.as_mut_slice()
    }
}

impl<T, I: SliceIndex<[T]>> Index<I> for NonEmptySlice<T> {
    type Output = I::Output;

    fn index(&self, index: I) -> &Self::Output {
        self.as_slice().index(index)
    }
}

impl<T, I: SliceIndex<[T]>> IndexMut<I> for NonEmptySlice<T> {
    fn index_mut(&mut self, index: I) -> &mut Self::Output {
        self.as_mut_slice().index_mut(index)
    }
}

impl<'a, T, const N: usize> TryFrom<&'a NonEmptySlice<T>> for &'a [T; N] {
    type Error = TryFromSliceError;

    fn try_from(non_empty: &'a NonEmptySlice<T>) -> Result<Self, Self::Error> {
        non_empty.as_slice().try_into()
    }
}

impl<'a, T, const N: usize> TryFrom<&'a mut NonEmptySlice<T>> for &'a mut [T; N] {
    type Error = TryFromSliceError;

    fn try_from(non_empty: &'a mut NonEmptySlice<T>) -> Result<Self, Self::Error> {
        non_empty.as_mut_slice().try_into()
    }
}

impl<T> NonEmptySlice<T> {
    /// Constructs [`Self`] from anything that can be converted to slice, provided it is non-empty.
    ///
    /// Prefer [`try_from_slice`] if only [`[T]`](prim@slice) is used,
    /// as this allows for `const` evaluation.
    ///
    /// # Errors
    ///
    /// Returns [`EmptySlice`] if the slice is empty.
    ///
    /// [`try_from_slice`]: Self::try_from_slice
    pub fn try_new<S: AsRef<[T]> + ?Sized>(slice: &S) -> Result<&Self, EmptySlice> {
        Self::try_from_slice(slice.as_ref())
    }

    /// Constructs [`Self`] from anything that can be mutably converted to slice,
    /// provided it is non-empty.
    ///
    /// Prefer [`try_from_mut_slice`] if only [`[T]`](prim@slice) is used,
    /// as this allows for `const` evaluation.
    ///
    /// # Errors
    ///
    /// Returns [`EmptySlice`] if the slice is empty.
    ///
    /// [`try_from_mut_slice`]: Self::try_from_mut_slice
    pub fn try_new_mut<S: AsMut<[T]> + ?Sized>(slice: &mut S) -> Result<&mut Self, EmptySlice> {
        Self::try_from_mut_slice(slice.as_mut())
    }

    /// Similar to [`try_new`], but the error is discarded.
    ///
    /// Prefer [`from_slice`] if only [`[T]`](prim@slice) is used,
    /// as it allows for `const` evaluation.
    ///
    /// # Examples
    ///
    /// ```
    /// use non_empty_slice::NonEmptySlice;
    ///
    /// let array = [1, 2, 3];
    ///
    /// let non_empty = NonEmptySlice::new(&array).unwrap();
    ///
    /// // `NonEmptySlice<T>` is `AsRef<[T]>`, so it can also be used here!
    /// let from_non_empty = NonEmptySlice::new(non_empty).unwrap();
    /// ```
    ///
    /// [`try_new`]: Self::try_new
    /// [`from_slice`]: Self::from_slice
    pub fn new<S: AsRef<[T]> + ?Sized>(slice: &S) -> Option<&Self> {
        Self::from_slice(slice.as_ref())
    }

    /// Similar to [`try_new_mut`], but the error is discarded.
    ///
    /// Prefer [`from_mut_slice`] if only [`[T]`](prim@slice) is used,
    /// as it allows for `const` evaluation.
    ///
    /// # Examples
    ///
    /// ```
    /// use non_empty_slice::NonEmptySlice;
    ///
    /// let mut array = [1, 2, 3];
    ///
    /// let non_empty = NonEmptySlice::new_mut(&mut array).unwrap();
    ///
    /// // `Slice<T>` is `AsMut<[T]>`, so it can also be used here!
    /// let from_non_empty = NonEmptySlice::new_mut(non_empty).unwrap();
    /// ```
    ///
    /// [`try_new_mut`]: Self::try_new_mut
    /// [`from_mut_slice`]: Self::from_mut_slice
    pub fn new_mut<S: AsMut<[T]> + ?Sized>(slice: &mut S) -> Option<&mut Self> {
        Self::from_mut_slice(slice.as_mut())
    }

    /// Constructs [`Self`] from anything that can be converted to slice, without doing any checks.
    ///
    /// Prefer [`from_slice_unchecked`] if only [`[T]`](prim@slice) is used,
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

    /// Constructs [`Self`] from anything that can be mutably converted to slice,
    /// without doing any checks.
    ///
    /// Prefer [`from_mut_slice_unchecked`] if only [`[T]`](prim@slice) is used,
    /// as this allows for `const` evaluation.
    ///
    /// # Safety
    ///
    /// The caller must ensure that the slice is non-empty.
    ///
    /// [`from_mut_slice_unchecked`]: Self::from_mut_slice_unchecked
    #[must_use]
    pub unsafe fn new_unchecked_mut<S: AsMut<[T]> + ?Sized>(slice: &mut S) -> &mut Self {
        // SAFETY: the caller must ensure that the slice is non-empty
        unsafe { Self::from_mut_slice_unchecked(slice.as_mut()) }
    }

    /// Constructs [`Self`] from [`[T]`](prim@slice), provided the slice is non-empty.
    ///
    /// # Errors
    ///
    /// Returns [`EmptySlice`] if the slice is empty.
    pub const fn try_from_slice(slice: &[T]) -> Result<&Self, EmptySlice> {
        if slice.is_empty() {
            return Err(EmptySlice);
        }

        // SAFETY: the slice is non-empty at this point
        Ok(unsafe { Self::from_slice_unchecked(slice) })
    }

    /// Constructs [`Self`] from mutable [`[T]`](prim@slice), provided the slice is non-empty.
    ///
    /// # Errors
    ///
    /// Returns [`EmptySlice`] if the slice is empty.
    pub const fn try_from_mut_slice(slice: &mut [T]) -> Result<&mut Self, EmptySlice> {
        if slice.is_empty() {
            return Err(EmptySlice);
        }

        // SAFETY: the slice is non-empty at this point
        Ok(unsafe { Self::from_mut_slice_unchecked(slice) })
    }

    /// Similar to [`try_from_slice`], but the error is discarded.
    ///
    /// # Examples
    ///
    /// Basic snippet:
    ///
    /// ```
    /// use non_empty_slice::NonEmptySlice;
    ///
    /// let array = [1, 2, 3];
    ///
    /// let non_empty = NonEmptySlice::from_slice(&array).unwrap();
    /// ```
    ///
    /// [`None`] is returned if the slice is empty, therefore the following snippet panics:
    ///
    /// ```should_panic
    /// use non_empty_slice::NonEmptyBytes;
    ///
    /// let empty = [];
    ///
    /// let never = NonEmptyBytes::from_slice(&empty).unwrap();
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

    /// Similar to [`try_from_mut_slice`], but the error is discarded.
    ///
    /// # Examples
    ///
    /// Basic snippet:
    ///
    /// ```
    /// use non_empty_slice::NonEmptySlice;
    ///
    /// let mut array = [1, 2, 3];
    ///
    /// let non_empty = NonEmptySlice::from_mut_slice(&mut array).unwrap();
    /// ```
    ///
    /// [`None`] is returned if the slice is empty, therefore the following snippet panics:
    ///
    /// ```should_panic
    /// use non_empty_slice::NonEmptyBytes;
    ///
    /// let mut empty = [];
    ///
    /// let never = NonEmptyBytes::from_mut_slice(&mut empty).unwrap();
    /// ```
    ///
    /// [`try_from_mut_slice`]: Self::try_from_mut_slice
    pub const fn from_mut_slice(slice: &mut [T]) -> Option<&mut Self> {
        if slice.is_empty() {
            return None;
        }

        // SAFETY: the slice is non-empty at this point
        Some(unsafe { Self::from_mut_slice_unchecked(slice) })
    }

    /// Constructs [`Self`] from immutable [`[T]`](prim@slice),
    /// without checking if the slice is empty.
    ///
    /// # Safety
    ///
    /// The caller must ensure that the slice is non-empty.
    #[must_use]
    pub const unsafe fn from_slice_unchecked(slice: &[T]) -> &Self {
        debug_assert!(!slice.is_empty());

        // SAFETY: the caller must ensure that the slice is non-empty
        // `Self` is `repr(transparent)`, so it is safe to transmute
        unsafe { &*(ptr::from_ref(slice) as *const Self) }
    }

    /// Constructs [`Self`] from mutable [`[T]`](prim@slice),
    /// without checking if the slice is empty.
    ///
    /// # Safety
    ///
    /// The caller must ensure that the slice is non-empty.
    #[must_use]
    pub const unsafe fn from_mut_slice_unchecked(slice: &mut [T]) -> &mut Self {
        debug_assert!(!slice.is_empty());

        // SAFETY: the caller must ensure that the slice is non-empty
        // `Self` is `repr(transparent)`, so it is safe to transmute
        unsafe { &mut *(ptr::from_mut(slice) as *mut Self) }
    }

    #[cfg(feature = "unsafe-assert")]
    const fn assert_non_empty(&self) {
        use core::hint::assert_unchecked;

        // SAFETY: the slice is non-empty by construction
        unsafe {
            assert_unchecked(!self.as_slice_no_assert().is_empty());
        }
    }

    const fn as_slice_no_assert(&self) -> &[T] {
        &self.inner
    }

    const fn as_mut_slice_no_assert(&mut self) -> &mut [T] {
        &mut self.inner
    }

    /// Returns the contained slice.
    ///
    /// # Examples
    ///
    /// ```
    /// use non_empty_slice::NonEmptyBytes;
    ///
    /// let nekit = b"nekit";
    ///
    /// let non_empty = NonEmptyBytes::from_slice(nekit).unwrap();
    ///
    /// assert_eq!(non_empty.as_slice(), nekit);
    /// ```
    #[must_use]
    pub const fn as_slice(&self) -> &[T] {
        #[cfg(feature = "unsafe-assert")]
        self.assert_non_empty();

        self.as_slice_no_assert()
    }

    /// Returns the contained mutable slice.
    #[must_use]
    pub const fn as_mut_slice(&mut self) -> &mut [T] {
        #[cfg(feature = "unsafe-assert")]
        self.assert_non_empty();

        self.as_mut_slice_no_assert()
    }

    /// Checks if the slice is empty. Always returns [`false`].
    ///
    /// This method is marked as deprecated since the slice is never empty.
    #[deprecated = "this slice is never empty"]
    pub const fn is_empty(&self) -> bool {
        false
    }

    /// Returns the length of the slice as [`Size`].
    pub const fn len(&self) -> Size {
        let len = self.as_slice().len();

        // SAFETY: the slice is non-empty by construction,
        // therefore its length is guaranteed to be non-zero
        unsafe { Size::new_unchecked(len) }
    }

    /// Returns regular by-reference iterator over the slice.
    pub fn iter(&self) -> Iter<'_, T> {
        self.as_slice().iter()
    }

    /// Returns regular by-mutable-reference iterator over the mutable slice.
    pub fn iter_mut(&mut self) -> IterMut<'_, T> {
        self.as_mut_slice().iter_mut()
    }

    /// Returns non-empty by-reference iterator over the slice.
    pub fn non_empty_iter(&self) -> NonEmptyIter<'_, T> {
        // SAFETY: the slice is non-empty by construction, so is the underlying iterator
        unsafe { NonEmptyAdapter::new(self.iter()) }
    }

    /// Returns non-empty by-mutable-reference iterator over the mutable slice.
    pub fn non_empty_iter_mut(&mut self) -> NonEmptyIterMut<'_, T> {
        // SAFETY: the slice is non-empty by construction, so is the underlying iterator
        unsafe { NonEmptyAdapter::new(self.iter_mut()) }
    }

    /// Returns the first item of the slice.
    ///
    /// Since the slice is guaranteed to be non-empty, this method always returns some value.
    pub const fn first(&self) -> &T {
        let option = self.as_slice().first();

        // SAFETY: the slice is non-empty by construction, so there is always some first value
        unsafe { option.unwrap_unchecked() }
    }

    /// Returns the first mutable item of the mutable slice.
    ///
    /// Since the slice is guaranteed to be non-empty, this method always returns some value.
    pub const fn first_mut(&mut self) -> &mut T {
        let option = self.as_mut_slice().first_mut();

        // SAFETY: the slice is non-empty by construction, so there is always some first value
        unsafe { option.unwrap_unchecked() }
    }

    /// Returns the last item of the slice.
    ///
    /// Since the slice is guaranteed to be non-empty, this method always returns some value.
    pub const fn last(&self) -> &T {
        let option = self.as_slice().last();

        // SAFETY: the slice is non-empty by construction, so there is always some last value
        unsafe { option.unwrap_unchecked() }
    }

    /// Returns the last mutable item of the mutable slice.
    ///
    /// Since the slice is guaranteed to be non-empty, this method always returns some value.
    pub const fn last_mut(&mut self) -> &mut T {
        let option = self.as_mut_slice().last_mut();

        // SAFETY: the slice is non-empty by construction, so there is always some last value
        unsafe { option.unwrap_unchecked() }
    }

    /// Returns the first and all the rest of the items in the slice.
    pub const fn split_first(&self) -> (&T, &[T]) {
        let option = self.as_slice().split_first();

        // SAFETY: the slice is non-empty by construction, so there is always some first value
        unsafe { option.unwrap_unchecked() }
    }

    /// Returns the first mutable item and all the rest of the items in the mutable slice.
    pub const fn split_first_mut(&mut self) -> (&mut T, &mut [T]) {
        let option = self.as_mut_slice().split_first_mut();

        // SAFETY: the slice is non-empty by construction, so there is always some first value
        unsafe { option.unwrap_unchecked() }
    }

    /// Returns the last and all the rest of the items in the slice.
    pub const fn split_last(&self) -> (&T, &[T]) {
        let option = self.as_slice().split_last();

        // SAFETY: the slice is non-empty by construction, so there is always some last value
        unsafe { option.unwrap_unchecked() }
    }

    /// Returns the last mutable item and all the rest of the items in the mutable slice.
    pub const fn split_last_mut(&mut self) -> (&mut T, &mut [T]) {
        let option = self.as_mut_slice().split_last_mut();

        // SAFETY: the slice is non-empty by construction, so there is always some last value
        unsafe { option.unwrap_unchecked() }
    }

    /// Returns the first `N` items of the slice as [`[T; N]`](prim@array).
    ///
    /// If there are less than `N` items, [`None`] is returned.
    pub const fn first_chunk<const N: usize>(&self) -> Option<&[T; N]> {
        self.as_slice().first_chunk()
    }

    /// Returns the first mutable `N` items of the mutable slice as [`[T; N]`](prim@array).
    ///
    /// If there are less than `N` items, [`None`] is returned.
    pub const fn first_chunk_mut<const N: usize>(&mut self) -> Option<&mut [T; N]> {
        self.as_mut_slice().first_chunk_mut()
    }

    /// Returns the first `N` items of the slice as [`[T; N]`](prim@array)
    /// and all the rest of the items.
    ///
    /// If there are less than `N` items, [`None`] is returned.
    pub const fn split_first_chunk<const N: usize>(&self) -> Option<(&[T; N], &[T])> {
        self.as_slice().split_first_chunk()
    }

    /// Returns the first mutable `N` items of the mutable slice as [`[T; N]`](prim@array)
    /// and all the rest of the items.
    ///
    /// If there are less than `N` items, [`None`] is returned.
    pub const fn split_first_chunk_mut<const N: usize>(
        &mut self,
    ) -> Option<(&mut [T; N], &mut [T])> {
        self.as_mut_slice().split_first_chunk_mut()
    }

    /// Returns the last `N` items of the slice as [`[T; N]`](prim@array).
    ///
    /// If there are less than `N` items, [`None`] is returned.
    pub const fn last_chunk<const N: usize>(&self) -> Option<&[T; N]> {
        self.as_slice().last_chunk()
    }

    /// Returns the last mutable `N` items of the mutable slice as [`[T; N]`](prim@array).
    ///
    /// If there are less than `N` items, [`None`] is returned.
    pub const fn last_chunk_mut<const N: usize>(&mut self) -> Option<&mut [T; N]> {
        self.as_mut_slice().last_chunk_mut()
    }

    /// Returns the last `N` items of the slice as [`[T; N]`](prim@array)
    /// and all the rest of the items.
    ///
    /// If there are less than `N` items, [`None`] is returned.
    pub const fn split_last_chunk<const N: usize>(&self) -> Option<(&[T], &[T; N])> {
        self.as_slice().split_last_chunk()
    }

    /// Returns the last mutable `N` items of the mutable slice as [`[T; N]`](prim@array)
    /// and all the rest of the items.
    ///
    /// If there are less than `N` items, [`None`] is returned.
    pub const fn split_last_chunk_mut<const N: usize>(
        &mut self,
    ) -> Option<(&mut [T], &mut [T; N])> {
        self.as_mut_slice().split_last_chunk_mut()
    }

    /// Returns the raw pointer to the slice.
    pub const fn as_ptr(&self) -> *const T {
        self.as_slice().as_ptr()
    }

    /// Returns the raw mutable pointer to the mutable slice.
    pub const fn as_mut_ptr(&mut self) -> *mut T {
        self.as_mut_slice().as_mut_ptr()
    }

    /// Returns the two raw pointers spanning the slice.
    ///
    /// The end pointer is one element past the end of the slice.
    pub const fn as_ptr_range(&self) -> Range<*const T> {
        self.as_slice().as_ptr_range()
    }

    /// Returns the two raw mutable pointers spanning the mutable slice.
    ///
    /// The end pointer is one element past the end of the slice.
    pub const fn as_mut_ptr_range(&mut self) -> Range<*mut T> {
        self.as_mut_slice().as_mut_ptr_range()
    }

    /// Reinterprets the slice as [`[T; N]`](prim@array).
    ///
    /// If the length is not equal to `N`, [`None`] is returned.
    pub const fn as_array<const N: usize>(&self) -> Option<&[T; N]> {
        if self.len().get() == N {
            let ptr = self.as_ptr().cast();

            // SAFETY: length is equal to `N`, so we can reinterpret this
            let this = unsafe { &*ptr };

            Some(this)
        } else {
            None
        }
    }

    /// Reinterprets the mutable slice as [`[T; N]`](prim@array).
    ///
    /// If the length is not equal to `N`, [`None`] is returned.
    pub const fn as_mut_array<const N: usize>(&mut self) -> Option<&mut [T; N]> {
        if self.len().get() == N {
            let ptr = self.as_mut_ptr().cast();

            // SAFETY: length is equal to `N`, so we can reinterpret this
            let this = unsafe { &mut *ptr };

            Some(this)
        } else {
            None
        }
    }

    /// Swaps two items in the slice.
    ///
    /// # Panics
    ///
    /// Panics if `first` or `other` are out of bounds.
    pub const fn swap(&mut self, first: usize, other: usize) {
        self.as_mut_slice().swap(first, other);
    }

    /// Reverses the slice in place.
    pub const fn reverse(&mut self) {
        self.as_mut_slice().reverse();
    }

    /// Returns non-empty iterator over the slice in (non-overlapping) non-empty chunks
    /// of given [`Size`], starting at the beginning of the slice.
    pub const fn chunks(&self, size: Size) -> Chunks<'_, T> {
        Chunks::new(self, size)
    }

    /// Returns non-empty iterator over the slice in (non-overlapping) non-empty mutable chunks
    /// of given [`Size`], starting at the beginning of the slice.
    pub const fn chunks_mut(&mut self, size: Size) -> ChunksMut<'_, T> {
        ChunksMut::new(self, size)
    }

    /// Returns non-empty iterator over the slice in (non-overlapping) non-empty chunks
    /// of given [`Size`], starting at the end of the slice.
    pub const fn rchunks(&self, size: Size) -> RChunks<'_, T> {
        RChunks::new(self, size)
    }

    /// Returns non-empty iterator over the slice in (non-overlapping) non-empty mutable chunks
    /// of given [`Size`], starting at the end of the slice.
    pub const fn rchunks_mut(&mut self, size: Size) -> RChunksMut<'_, T> {
        RChunksMut::new(self, size)
    }

    /// Returns non-empty iterator over the slice in (non-overlapping) chunks
    /// of given [`Size`], starting at the beginning of the slice.
    ///
    /// When the length of the slice is not divisible by the chunk size,
    /// the last chunk will be omitted.
    pub const fn chunks_exact(&self, size: Size) -> ChunksExact<'_, T> {
        ChunksExact::new(self, size)
    }

    /// Returns non-empty iterator over the slice in (non-overlapping) mutable chunks
    /// of given [`Size`], starting at the beginning of the slice.
    ///
    /// When the length of the slice is not divisible by the chunk size,
    /// the last chunk will be omitted.
    pub const fn chunks_exact_mut(&mut self, size: Size) -> ChunksExactMut<'_, T> {
        ChunksExactMut::new(self, size)
    }

    /// Returns non-empty iterator over the slice in (non-overlapping) chunks
    /// of given [`Size`], starting at the end of the slice.
    ///
    /// When the length of the slice is not divisible by the chunk size,
    /// the last chunk will be omitted.
    pub const fn rchunks_exact(&self, size: Size) -> RChunksExact<'_, T> {
        RChunksExact::new(self, size)
    }

    /// Returns non-empty iterator over the slice in (non-overlapping) mutable chunks
    /// of given [`Size`], starting at the end of the slice.
    ///
    /// When the length of the slice is not divisible by the chunk size,
    /// the last chunk will be omitted.
    pub const fn rchunks_exact_mut(&mut self, size: Size) -> RChunksExactMut<'_, T> {
        RChunksExactMut::new(self, size)
    }

    /// Returns non-empty iterator over the slice in (overlapping) windows of given [`Size`].
    pub const fn windows(&self, size: Size) -> Windows<'_, T> {
        Windows::new(self, size)
    }

    /// Returns non-empty iterator over the slice in (non-overlapping) chunks,
    /// separated by the given predicate.
    pub const fn chunk_by<P: FnMut(&T, &T) -> bool>(&self, predicate: P) -> ChunkBy<'_, T, P> {
        ChunkBy::new(self, predicate)
    }

    /// Returns non-empty iterator over the slice in (non-overlapping) mutable chunks,
    /// separated by the given predicate.
    pub const fn chunk_by_mut<P: FnMut(&T, &T) -> bool>(
        &mut self,
        predicate: P,
    ) -> ChunkByMut<'_, T, P> {
        ChunkByMut::new(self, predicate)
    }

    /// Splits the slice into chunks of `N` items, starting at the beginning of the slice,
    /// returning the remainder as another slice.
    ///
    /// # Panics
    ///
    /// Panics if `N` is zero.
    pub const fn as_chunks<const N: usize>(&self) -> (&[[T; N]], &[T]) {
        self.as_slice().as_chunks()
    }

    /// Splits the slice into mutable chunks of `N` items, starting at the beginning of the slice,
    /// returning the remainder as another mutable slice.
    ///
    /// # Panics
    ///
    /// Panics if `N` is zero.
    pub const fn as_chunks_mut<const N: usize>(&mut self) -> (&mut [[T; N]], &mut [T]) {
        self.as_mut_slice().as_chunks_mut()
    }

    /// Splits the slice into chunks of `N` items, assuming there is no remainder.
    ///
    /// # Safety
    ///
    /// The caller must ensure that the length of the slice is divisible by `N` (and `N != 0`).
    pub const unsafe fn as_chunks_unchecked<const N: usize>(&self) -> &[[T; N]] {
        // SAFETY: the caller must ensure that the length of the slice is divisible by `N`
        // and that `N` is non-zero
        unsafe { self.as_slice().as_chunks_unchecked() }
    }

    /// Splits the slice into mutable chunks of `N` items, assuming there is no remainder.
    ///
    /// # Safety
    ///
    /// The caller must ensure that the length of the slice is divisible by `N`.
    pub const unsafe fn as_chunks_unchecked_mut<const N: usize>(&mut self) -> &mut [[T; N]] {
        // SAFETY: the caller must ensure that the length of the slice is divisible by `N`
        // and that `N` is non-zero
        unsafe { self.as_mut_slice().as_chunks_unchecked_mut() }
    }

    /// Splits the slice into chunks of `N` items, starting at the end of the slice,
    /// returning the remainder as another slice.
    ///
    /// # Panics
    ///
    /// Panics if `N` is zero.
    pub const fn as_rchunks<const N: usize>(&self) -> (&[T], &[[T; N]]) {
        self.as_slice().as_rchunks()
    }

    /// Splits the mutable slice into mutable chunks of `N` items, starting at the end of the slice,
    /// returning the remainder as another mutable slice.
    ///
    /// # Panics
    ///
    /// Panics if `N` is zero.
    pub const fn as_rchunks_mut<const N: usize>(&mut self) -> (&mut [T], &mut [[T; N]]) {
        self.as_mut_slice().as_rchunks_mut()
    }

    /// Splits the slice into two at the given non-zero index.
    ///
    /// The index has to be non-zero in order to guarantee non-emptiness of the left slice.
    ///
    /// # Panics
    ///
    /// Panics if the index is out of bounds.
    pub const fn split_at(&self, index: Size) -> (&Self, &[T]) {
        let (left, right) = self.as_slice().split_at(index.get());

        // SAFETY: splitting non-empty slice at non-zero index yields non-empty left slice
        let left_non_empty = unsafe { Self::from_slice_unchecked(left) };

        (left_non_empty, right)
    }

    /// Splits the mutable slice into two at the given non-zero index.
    ///
    /// The index has to be non-zero in order to guarantee non-emptiness of the left slice.
    ///
    /// # Panics
    ///
    /// Panics if the index is out of bounds.
    pub const fn split_at_mut(&mut self, index: Size) -> (&mut Self, &mut [T]) {
        let (left, right) = self.as_mut_slice().split_at_mut(index.get());

        // SAFETY: splitting non-empty slice at non-zero index yields non-empty left slice
        let left_non_empty = unsafe { Self::from_mut_slice_unchecked(left) };

        (left_non_empty, right)
    }

    /// Splits the slice into two at the given non-zero index, without doing any bounds checks.
    ///
    /// The index has to be non-zero in order to guarantee non-emptiness of the left slice.
    ///
    /// # Safety
    ///
    /// The caller must ensure that the index is in bounds.
    pub const unsafe fn split_at_unchecked(&self, index: Size) -> (&Self, &[T]) {
        // SAFETY: the caller must ensure the index is in bounds
        let (left, right) = unsafe { self.as_slice().split_at_unchecked(index.get()) };

        // SAFETY: splitting non-empty slice at non-zero index yields non-empty left slice
        let left_non_empty = unsafe { Self::from_slice_unchecked(left) };

        (left_non_empty, right)
    }

    /// Splits the mutable slice into two at the given non-zero index,
    /// without doing any bounds checks.
    ///
    /// The index has to be non-zero in order to guarantee non-emptiness of the left slice.
    ///
    /// # Safety
    ///
    /// The caller must ensure that the index is in bounds.
    pub const unsafe fn split_at_mut_unchecked(&mut self, index: Size) -> (&mut Self, &mut [T]) {
        // SAFETY: the caller must ensure the index is in bounds
        let (left, right) = unsafe { self.as_mut_slice().split_at_mut_unchecked(index.get()) };

        // SAFETY: splitting non-empty slice at non-zero index yields non-empty left slice
        let left_non_empty = unsafe { Self::from_mut_slice_unchecked(left) };

        (left_non_empty, right)
    }

    /// Splits the slice into two at the given non-zero index, returning [`None`] if out of bounds.
    ///
    /// The index has to be non-zero in order to guarantee non-emptiness of the left slice.
    pub const fn split_at_checked(&self, index: Size) -> Option<(&Self, &[T])> {
        let Some((left, right)) = self.as_slice().split_at_checked(index.get()) else {
            return None;
        };

        // SAFETY: splitting non-empty slice at non-zero index yields non-empty left slice
        let left_non_empty = unsafe { Self::from_slice_unchecked(left) };

        Some((left_non_empty, right))
    }

    /// Splits the mutable slice into two at the given non-zero index,
    /// returning [`None`] if out of bounds.
    ///
    /// The index has to be non-zero in order to guarantee non-emptiness of the left slice.
    pub const fn split_at_mut_checked(&mut self, index: Size) -> Option<(&mut Self, &mut [T])> {
        let Some((left, right)) = self.as_mut_slice().split_at_mut_checked(index.get()) else {
            return None;
        };

        // SAFETY: splitting non-empty slice at non-zero index yields non-empty left slice
        let left_non_empty = unsafe { Self::from_mut_slice_unchecked(left) };

        Some((left_non_empty, right))
    }

    // NOTE: other methods are available via deref coercion to `[T]`
}

impl<T: Clone> NonEmptySlice<T> {
    /// Clones all items from another non-empty slice into this one.
    ///
    /// # Panics
    ///
    /// Panics if the slices have different lengths.
    pub fn clone_from_non_empty_slice(&mut self, other: &Self) {
        self.as_mut_slice().clone_from_slice(other.as_slice());
    }
}

type Bytes = [u8];

impl NonEmptyBytes {
    /// Checks if all bytes in the slice are within the ASCII range.
    #[must_use]
    pub const fn is_ascii(&self) -> bool {
        self.as_slice().is_ascii()
    }

    /// Checks that the two slices are ASCII case-insensitively equal.
    #[must_use]
    pub const fn eq_ignore_ascii_case(&self, other: &Self) -> bool {
        self.as_slice().eq_ignore_ascii_case(other.as_slice())
    }

    /// Converts the slice to its ASCII uppercase equivalent in-place.
    pub const fn make_ascii_uppercase(&mut self) {
        self.as_mut_slice().make_ascii_uppercase();
    }

    /// Converts the slice to its ASCII lowercase equivalent in-place.
    pub const fn make_ascii_lowercase(&mut self) {
        self.as_mut_slice().make_ascii_lowercase();
    }

    /// Returns new slice with leading ASCII whitespace bytes removed.
    #[must_use]
    pub const fn trim_ascii_start(&self) -> &Bytes {
        self.as_slice().trim_ascii_start()
    }

    /// Returns new slice with trailing ASCII whitespace bytes removed.
    #[must_use]
    pub const fn trim_ascii_end(&self) -> &Bytes {
        self.as_slice().trim_ascii_end()
    }

    /// Returns new slice with leading and trailing ASCII whitespace bytes removed.
    #[must_use]
    pub const fn trim_ascii(&self) -> &Bytes {
        self.as_slice().trim_ascii()
    }
}

impl<'a, T> IntoIterator for &'a NonEmptySlice<T> {
    type Item = &'a T;

    type IntoIter = Iter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<'a, T> IntoIterator for &'a mut NonEmptySlice<T> {
    type Item = &'a mut T;

    type IntoIter = IterMut<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter_mut()
    }
}

impl<'a, T> IntoNonEmptyIterator for &'a NonEmptySlice<T> {
    type IntoNonEmptyIter = NonEmptyIter<'a, T>;

    fn into_non_empty_iter(self) -> Self::IntoNonEmptyIter {
        self.non_empty_iter()
    }
}

impl<'a, T> IntoNonEmptyIterator for &'a mut NonEmptySlice<T> {
    type IntoNonEmptyIter = NonEmptyIterMut<'a, T>;

    fn into_non_empty_iter(self) -> Self::IntoNonEmptyIter {
        self.non_empty_iter_mut()
    }
}
