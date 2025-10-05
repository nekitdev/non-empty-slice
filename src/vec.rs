//! Non-empty [`Vec<T>`].

#[cfg(not(any(feature = "std", feature = "alloc")))]
compile_error!("expected either `std` or `alloc` to be enabled");

#[cfg(feature = "std")]
use std::{collections::TryReserveError, vec::IntoIter};

#[cfg(all(not(feature = "std"), feature = "alloc"))]
use alloc::{
    collections::TryReserveError,
    vec::{IntoIter, Vec},
};

use core::{
    borrow::{Borrow, BorrowMut},
    mem::MaybeUninit,
    ops::{Deref, DerefMut, Index, IndexMut, RangeBounds},
    slice::{Iter, IterMut, SliceIndex, from_raw_parts_mut},
};

use non_empty_iter::{
    FromNonEmptyIterator, IntoNonEmptyIterator, NonEmptyAdapter, NonEmptyIterator,
};
use non_zero_size::Size;
use thiserror::Error;

use crate::{
    boxed::EmptyBoxedSlice,
    format,
    iter::{IntoNonEmptyIter, NonEmptyIter, NonEmptyIterMut},
    slice::{EmptySlice, NonEmptySlice},
};

/// The error message used when the vector is empty.
pub const EMPTY_VEC: &str = "the vector is empty";

/// Similar to [`EmptySlice`], but holds the empty vector provided.
///
/// [`EmptySlice`]: crate::slice::EmptySlice
#[derive(Error)]
#[error("{EMPTY_VEC}")]
#[cfg_attr(
    feature = "diagnostics",
    derive(miette::Diagnostic),
    diagnostic(code(non_empty_slice::vec), help("make sure the vector is non-empty"))
)]
pub struct EmptyVec<T> {
    vec: Vec<T>,
}

format::debug!(EmptyVec, vec);

impl<T> EmptyVec<T> {
    // NOTE: this is private to prevent creating this error with non-empty vectors
    pub(crate) const fn new(vec: Vec<T>) -> Self {
        Self { vec }
    }

    /// Returns the contained empty vector.
    #[must_use]
    pub fn get(self) -> Vec<T> {
        self.vec
    }

    /// Constructs [`Self`] from [`EmptyBoxedSlice<T>`].
    #[must_use]
    pub fn from_empty_boxed_slice(empty: EmptyBoxedSlice<T>) -> Self {
        Self::new(empty.get().into_vec())
    }

    /// Converts [`Self`] into [`EmptyBoxedSlice<T>`].
    #[must_use]
    pub fn into_empty_boxed_slice(self) -> EmptyBoxedSlice<T> {
        EmptyBoxedSlice::from_empty_vec(self)
    }
}

/// Represents empty byte vectors, [`EmptyVec<u8>`].
pub type EmptyByteVec = EmptyVec<u8>;

/// Represents non-empty [`Vec<T>`] values.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct NonEmptyVec<T> {
    inner: Vec<T>,
}

impl<T: Clone> Clone for NonEmptyVec<T> {
    fn clone(&self) -> Self {
        // SAFETY: the vector is non-empty by construction
        unsafe { Self::new_unchecked(self.as_vec().clone()) }
    }

    fn clone_from(&mut self, source: &Self) {
        // SAFETY: cloning from non-empty vector can not make the vector empty
        unsafe {
            self.as_mut_vec().clone_from(source.as_vec());
        }
    }
}

/// Represents non-empty byte vectors, [`NonEmptyVec<u8>`].
pub type NonEmptyByteVec = NonEmptyVec<u8>;

impl<T> Borrow<NonEmptySlice<T>> for NonEmptyVec<T> {
    fn borrow(&self) -> &NonEmptySlice<T> {
        self.as_non_empty_slice()
    }
}

impl<T> BorrowMut<NonEmptySlice<T>> for NonEmptyVec<T> {
    fn borrow_mut(&mut self) -> &mut NonEmptySlice<T> {
        self.as_non_empty_mut_slice()
    }
}

impl<T> Borrow<[T]> for NonEmptyVec<T> {
    fn borrow(&self) -> &[T] {
        self.as_slice()
    }
}

impl<T> BorrowMut<[T]> for NonEmptyVec<T> {
    fn borrow_mut(&mut self) -> &mut [T] {
        self.as_mut_slice()
    }
}

impl<T> TryFrom<Vec<T>> for NonEmptyVec<T> {
    type Error = EmptyVec<T>;

    fn try_from(value: Vec<T>) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

impl<T> From<NonEmptyVec<T>> for Vec<T> {
    fn from(non_empty: NonEmptyVec<T>) -> Self {
        non_empty.into_vec()
    }
}

impl<T: Clone> From<&NonEmptySlice<T>> for NonEmptyVec<T> {
    fn from(non_empty: &NonEmptySlice<T>) -> Self {
        non_empty.to_non_empty_vec()
    }
}

impl<T: Clone> From<&mut NonEmptySlice<T>> for NonEmptyVec<T> {
    fn from(non_empty: &mut NonEmptySlice<T>) -> Self {
        non_empty.to_non_empty_vec()
    }
}

impl<T: Clone> TryFrom<&[T]> for NonEmptyVec<T> {
    type Error = EmptySlice;

    fn try_from(slice: &[T]) -> Result<Self, Self::Error> {
        let non_empty_slice: &NonEmptySlice<T> = slice.try_into()?;

        Ok(non_empty_slice.into())
    }
}

impl<T: Clone> TryFrom<&mut [T]> for NonEmptyVec<T> {
    type Error = EmptySlice;

    fn try_from(slice: &mut [T]) -> Result<Self, Self::Error> {
        let non_empty_slice: &mut NonEmptySlice<T> = slice.try_into()?;

        Ok(non_empty_slice.into())
    }
}

impl<T> AsRef<Self> for NonEmptyVec<T> {
    fn as_ref(&self) -> &Self {
        self
    }
}

impl<T> AsMut<Self> for NonEmptyVec<T> {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl<T> AsRef<Vec<T>> for NonEmptyVec<T> {
    fn as_ref(&self) -> &Vec<T> {
        self.as_vec()
    }
}

impl<T> AsRef<NonEmptySlice<T>> for NonEmptyVec<T> {
    fn as_ref(&self) -> &NonEmptySlice<T> {
        self.as_non_empty_slice()
    }
}

impl<T> AsMut<NonEmptySlice<T>> for NonEmptyVec<T> {
    fn as_mut(&mut self) -> &mut NonEmptySlice<T> {
        self.as_non_empty_mut_slice()
    }
}

impl<T> AsRef<[T]> for NonEmptyVec<T> {
    fn as_ref(&self) -> &[T] {
        self.as_slice()
    }
}

impl<T> AsMut<[T]> for NonEmptyVec<T> {
    fn as_mut(&mut self) -> &mut [T] {
        self.as_mut_slice()
    }
}

impl<T> Deref for NonEmptyVec<T> {
    type Target = NonEmptySlice<T>;

    fn deref(&self) -> &Self::Target {
        self.as_non_empty_slice()
    }
}

impl<T> DerefMut for NonEmptyVec<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.as_non_empty_mut_slice()
    }
}

impl<T, I: SliceIndex<[T]>> Index<I> for NonEmptyVec<T> {
    type Output = I::Output;

    fn index(&self, index: I) -> &Self::Output {
        self.as_vec().index(index)
    }
}

impl<T, I: SliceIndex<[T]>> IndexMut<I> for NonEmptyVec<T> {
    fn index_mut(&mut self, index: I) -> &mut Self::Output {
        // SAFETY: indexing can not make the vector empty
        unsafe { self.as_mut_vec().index_mut(index) }
    }
}

impl<T> NonEmptyVec<T> {
    /// Constructs [`Self`], provided that the [`Vec<T>`] provided is non-empty.
    ///
    /// # Errors
    ///
    /// Returns [`EmptyVec<T>`] if the provided vector is empty.
    ///
    /// # Examples
    ///
    /// Basic snippet:
    ///
    /// ```
    /// use non_empty_slice::NonEmptyVec;
    ///
    /// let non_empty_vec = NonEmptyVec::new(vec![1, 2, 3]).unwrap();
    /// ```
    ///
    /// Handling possible errors and recovering empty vectors (see [`EmptyVec<T>`] for more):
    ///
    /// ```
    /// use non_empty_slice::NonEmptyByteVec;
    ///
    /// let empty_vec = NonEmptyByteVec::new(Vec::new()).unwrap_err();
    ///
    /// let empty = empty_vec.get();
    /// ```
    pub const fn new(vector: Vec<T>) -> Result<Self, EmptyVec<T>> {
        if vector.is_empty() {
            return Err(EmptyVec::new(vector));
        }

        // SAFETY: the vector is non-empty at this point
        Ok(unsafe { Self::new_unchecked(vector) })
    }

    /// Constructs [`Self`] without checking that the [`Vec<T>`] is non-empty.
    ///
    /// # Safety
    ///
    /// The caller must ensure that the vector is non-empty.
    #[must_use]
    pub const unsafe fn new_unchecked(inner: Vec<T>) -> Self {
        Self { inner }
    }

    #[cfg(feature = "unsafe-assert")]
    const fn assert_non_empty(&self) {
        use core::hint::assert_unchecked;

        // SAFETY: the vector is non-empty by construction
        unsafe {
            assert_unchecked(!self.as_vec_no_assert().is_empty());
        }
    }

    const fn as_vec_no_assert(&self) -> &Vec<T> {
        &self.inner
    }

    const unsafe fn as_mut_vec_no_assert(&mut self) -> &mut Vec<T> {
        &mut self.inner
    }

    fn into_vec_no_assert(self) -> Vec<T> {
        self.inner
    }

    /// Returns the contained slice reference as [`NonEmptySlice<T>`].
    #[must_use]
    pub const fn as_non_empty_slice(&self) -> &NonEmptySlice<T> {
        // SAFETY: the slice is non-empty by construction
        unsafe { NonEmptySlice::from_slice_unchecked(self.as_slice()) }
    }

    /// Returns the contained slice reference as mutable [`NonEmptySlice<T>`].
    #[must_use]
    pub const fn as_non_empty_mut_slice(&mut self) -> &mut NonEmptySlice<T> {
        // SAFETY: the slice is non-empty by construction
        unsafe { NonEmptySlice::from_mut_slice_unchecked(self.as_mut_slice()) }
    }

    /// Extracts the slice containing the entire vector.
    #[must_use]
    pub const fn as_slice(&self) -> &[T] {
        self.as_vec().as_slice()
    }

    /// Extracts the mutable slice containing the entire vector.
    #[must_use]
    pub const fn as_mut_slice(&mut self) -> &mut [T] {
        // SAFETY: getting mutable slice can not make the vector empty
        unsafe { self.as_mut_vec().as_mut_slice() }
    }

    /// Returns the contained [`Vec<T>`] behind immutable reference.
    #[must_use]
    pub const fn as_vec(&self) -> &Vec<T> {
        #[cfg(feature = "unsafe-assert")]
        self.assert_non_empty();

        self.as_vec_no_assert()
    }

    /// Returns the contained [`Vec<T>`] behind mutable reference.
    ///
    /// # Safety
    ///
    /// The caller must ensure that the returned vector remains non-empty.
    #[must_use]
    pub const unsafe fn as_mut_vec(&mut self) -> &mut Vec<T> {
        #[cfg(feature = "unsafe-assert")]
        self.assert_non_empty();

        // SAFETY: the caller must ensure that the returned vector remains non-empty
        unsafe { self.as_mut_vec_no_assert() }
    }

    /// Returns the contained [`Vec<T>`].
    #[must_use]
    pub fn into_vec(self) -> Vec<T> {
        #[cfg(feature = "unsafe-assert")]
        self.assert_non_empty();

        self.into_vec_no_assert()
    }
}

impl<T: Clone> NonEmptyVec<T> {
    /// Constructs [`Self`] from [`NonEmptySlice<T>`] via cloning.
    ///
    /// # Examples
    ///
    /// Basic snippet:
    ///
    /// ```
    /// use non_empty_slice::{NonEmptyByteVec, NonEmptyBytes};
    ///
    /// let nekit = NonEmptyBytes::from_slice(b"nekit").unwrap();
    ///
    /// let owned = NonEmptyByteVec::from_non_empty_slice(nekit);
    /// ```
    pub fn from_non_empty_slice(non_empty: &NonEmptySlice<T>) -> Self {
        // SAFETY: the slice is non-empty by construction
        unsafe { Self::new_unchecked(non_empty.to_vec()) }
    }
}

impl<T: Clone> NonEmptySlice<T> {
    /// Constructs [`Vec<T>`] from the slice via cloning.
    pub fn to_vec(&self) -> Vec<T> {
        self.as_slice().to_vec()
    }

    /// Constructs [`NonEmptyVec<T>`] from the non-empty slice via cloning.
    pub fn to_non_empty_vec(&self) -> NonEmptyVec<T> {
        NonEmptyVec::from_non_empty_slice(self)
    }
}

impl<T> NonEmptyVec<T> {
    /// Checks if the vector is empty. Always returns [`false`].
    ///
    /// This method is marked as deprecated since the vector is never empty.
    #[must_use]
    #[deprecated = "this vector is never empty"]
    pub const fn is_empty(&self) -> bool {
        false
    }

    /// Returns the length of the vector as [`Size`].
    #[must_use]
    pub const fn len(&self) -> Size {
        self.as_non_empty_slice().len()
    }

    /// Returns the capacity of the vector as [`Size`].
    #[must_use]
    pub const fn capacity(&self) -> Size {
        let capacity = self.as_vec().capacity();

        // SAFETY: non-empty vector implies non-zero capacity
        unsafe { Size::new_unchecked(capacity) }
    }

    /// Appends the given value to the end of the vector.
    ///
    /// # Panics
    ///
    /// Panics on capacity overflow.
    pub fn push(&mut self, value: T) {
        // SAFETY: pushing can not make the vector empty
        unsafe {
            self.as_mut_vec().push(value);
        }
    }

    /// Reserves capacity for at least `additional` more values to be inserted into the vector.
    ///
    /// Note that the additional capacity is required to be non-zero via [`Size`].
    ///
    /// This method can over-allocate to speculatively avoid frequent reallocations.
    ///
    /// Does nothing if the capacity is already sufficient.
    ///
    /// # Panics
    ///
    /// Panics on capacity overflow.
    pub fn reserve(&mut self, additional: Size) {
        // SAFETY: reserving can not make the vector empty
        unsafe {
            self.as_mut_vec().reserve(additional.get());
        }
    }

    /// Reserves the minimum capacity for exactly `additional` more values to be inserted
    /// into the vector.
    ///
    /// Note that the additional capacity is required to be non-zero via [`Size`].
    ///
    /// Unlike [`reserve`], this method will not deliberately over-allocate
    /// to speculatively avoid frequent reallocations.
    ///
    /// Does nothing if the capacity is already sufficient.
    ///
    /// # Panics
    ///
    /// Panics on capacity overflow.
    ///
    /// [`reserve`]: Self::reserve
    pub fn reserve_exact(&mut self, additional: Size) {
        // SAFETY: reserving can not make the vector empty
        unsafe {
            self.as_mut_vec().reserve_exact(additional.get());
        }
    }

    /// Tries to reserve capacity for at least `additional` more values to be inserted
    /// into the vector.
    ///
    /// Note that the additional capacity is required to be non-zero via [`Size`].
    ///
    /// This method can over-allocate to speculatively avoid frequent reallocations.
    ///
    /// Does nothing if the capacity is already sufficient.
    ///
    /// # Errors
    ///
    /// Returns [`TryReserveError`] if the allocation fails or capacity overflows.
    pub fn try_reserve(&mut self, additional: Size) -> Result<(), TryReserveError> {
        // SAFETY: reserving can not make the vector empty
        unsafe { self.as_mut_vec().try_reserve(additional.get()) }
    }

    /// Tries to reserve the minimum capacity for exactly `additional` more values
    /// to be inserted into the vector.
    ///
    /// Note that the additional capacity is required to be non-zero via [`Size`].
    ///
    /// Unlike [`try_reserve`], this method will not deliberately over-allocate
    /// to speculatively avoid frequent reallocations.
    ///
    /// Does nothing if the capacity is already sufficient.
    ///
    /// # Errors
    ///
    /// Returns [`TryReserveError`] if the allocation fails or capacity overflows.
    ///
    /// [`try_reserve`]: Self::try_reserve
    pub fn try_reserve_exact(&mut self, additional: usize) -> Result<(), TryReserveError> {
        // SAFETY: reserving can not make the vector empty
        unsafe { self.as_mut_vec().try_reserve_exact(additional) }
    }

    /// Shrinks the capacity of the vector as much as possible.
    pub fn shrink_to_fit(&mut self) {
        // SAFETY: shrinking can not make the vector empty
        unsafe {
            self.as_mut_vec().shrink_to_fit();
        }
    }

    /// Shrinks the capacity of the vector to the specified amount.
    ///
    /// The capacity will remain at least as large as both the length and the supplied amount.
    ///
    /// Does nothing if the current capacity is less than or equal to the given amount.
    pub fn shrink_to(&mut self, capacity: Size) {
        // SAFETY: shrinking can not make the vector empty
        unsafe {
            self.as_mut_vec().shrink_to(capacity.get());
        }
    }

    /// Shortens the vector, keeping the first `len` items and dropping the rest.
    pub fn truncate(&mut self, len: Size) {
        // SAFETY: length provided is non-zero, so truncating can not make the vector empty
        unsafe {
            self.as_mut_vec().truncate(len.get());
        }
    }

    /// Moves all the items out of `other` into `self`, leaving `other` empty.
    ///
    /// # Panics
    ///
    /// Panics on capacity overflow.
    pub fn append(&mut self, other: &mut Vec<T>) {
        // SAFETY: appending can not make the vector empty
        unsafe {
            self.as_mut_vec().append(other);
        }
    }

    /// Inserts the given value at the specified index, shifting all items after it to the right.
    ///
    /// # Panics
    ///
    /// Panics if the index is out of bounds.
    pub fn insert(&mut self, index: usize, value: T) {
        // SAFETY: inserting can not make the vector empty
        unsafe {
            self.as_mut_vec().insert(index, value);
        }
    }

    /// Checks whether the vector is almost empty, meaning it only contains one value.
    #[must_use]
    pub fn next_empty(&self) -> bool {
        self.len() == Size::MIN
    }

    /// The negated version of [`next_empty`].
    ///
    /// [`next_empty`]: Self::next_empty
    #[must_use]
    pub fn next_non_empty(&self) -> bool {
        !self.next_empty()
    }

    /// Peeks at the last item of the vector mutably.
    pub const fn peek_mut(&mut self) -> PeekMut<'_, T> {
        PeekMut::new(self)
    }

    /// Removes the last item from the vector and returns it,
    /// or [`None`] if the vector would become empty.
    pub fn pop(&mut self) -> Option<T> {
        self.next_non_empty()
            // SAFETY: popping only if the vector would remain non-empty
            .then(|| unsafe { self.as_mut_vec().pop() })
            .flatten()
    }

    /// Removes the last item from the vector if the predicate returns [`true`],
    /// or [`None`] if [`false`] is returned or if the vector would become empty.
    pub fn pop_if<P: FnOnce(&mut T) -> bool>(&mut self, predicate: P) -> Option<T> {
        self.next_non_empty()
            // SAFETY: popping only if the vector would remain non-empty
            .then(|| unsafe { self.as_mut_vec().pop_if(predicate) })
            .flatten()
    }

    /// Removes and returns the item at the given index within the vector,
    /// shifting all items after it to the left.
    ///
    /// Returns [`None`] if the vector would become empty.
    pub fn remove(&mut self, index: usize) -> Option<T> {
        self.next_non_empty()
            // SAFETY: removing only if the vector would remain non-empty
            .then(|| unsafe { self.as_mut_vec().remove(index) })
    }

    /// Removes and returns the item at the given index within the vector,
    /// replacing it with the last item of the vector.
    ///
    /// Returns [`None`] if the vector would become empty.
    pub fn swap_remove(&mut self, index: usize) -> Option<T> {
        self.next_non_empty()
            // SAFETY: swap-removing only if the vector would remain non-empty
            .then(|| unsafe { self.as_mut_vec().swap_remove(index) })
    }

    /// Splits the vector into two at the given non-zero index.
    ///
    /// The index has to be non-zero to guarantee the vector would remain non-empty.
    ///
    /// # Panics
    ///
    /// Panics if the provided index is out of bounds.
    pub fn split_off(&mut self, at: Size) -> Vec<T> {
        // SAFETY: splitting at non-zero index can not make the vector empty
        unsafe { self.as_mut_vec().split_off(at.get()) }
    }

    /// Resizes the vector in-place so that its length is equal to `new`.
    ///
    /// If `new` is greater than [`len`], the vector is extended by the difference,
    /// with each additional slot filled by the result of calling the provided function.
    ///
    /// The additional items will appear in the same order as they are generated.
    ///
    /// [`len`]: Self::len
    pub fn resize_with<F: FnMut() -> T>(&mut self, new: Size, function: F) {
        // SAFETY: resizing to non-zero length can not make the vector empty
        unsafe {
            self.as_mut_vec().resize_with(new.get(), function);
        }
    }

    /// Consumes and leaks the vector, returning the mutable slice of its contents.
    #[must_use]
    pub fn leak<'a>(self) -> &'a mut [T] {
        self.into_vec().leak()
    }

    /// Similar to [`leak`], but yields [`NonEmptySlice<T>`].
    ///
    /// [`leak`]: Self::leak
    #[must_use]
    pub fn leak_non_empty<'a>(self) -> &'a mut NonEmptySlice<T> {
        // SAFETY: leaking non-empty vector yields non-empty mutable slice
        unsafe { NonEmptySlice::from_mut_slice_unchecked(self.leak()) }
    }

    /// Forces the length of the vector to the given [`Size`].
    ///
    /// # Safety
    ///
    /// The `new` length must be less than or equal to the [`capacity`].
    ///
    /// The items at `len..new` must be initialized.
    ///
    /// [`capacity`]: Self::capacity
    pub unsafe fn set_len(&mut self, new: Size) {
        // SAFETY: setting non-zero length guarantees the vector is non-empty
        // moreover, the caller must uphold all safety requirements of this method
        unsafe { self.as_mut_vec().set_len(new.get()) }
    }

    /// Returns the spare capacity of the vector as mutable slice of [`MaybeUninit<T>`].
    ///
    /// This is useful for low-level manipulation of the vector, often coupled with [`set_len`].
    ///
    /// [`set_len`]: Self::set_len
    pub fn spare_capacity_mut(&mut self) -> &mut MaybeUninitSlice<T> {
        // SAFETY: returning spare capacity can not make the vector empty
        unsafe { self.as_mut_vec().spare_capacity_mut() }
    }

    /// Splits the vector into the non-empty initialized part and the spare capacity part.
    ///
    /// This essentially returns [`as_non_empty_mut_slice`] and [`spare_capacity_mut`].
    ///
    /// [`as_non_empty_mut_slice`]: Self::as_non_empty_mut_slice
    /// [`spare_capacity_mut`]: Self::spare_capacity_mut
    pub const fn split_at_spare_mut(
        &mut self,
    ) -> (&mut NonEmptySlice<T>, &mut MaybeUninitSlice<T>) {
        let len = self.len().get();

        let capacity = self.capacity().get();

        // SAFETY: nothing here changes the length of the vector, therefore it remains non-empty
        let ptr = unsafe { self.as_mut_vec().as_mut_ptr() };

        // SAFETY: possibly there are uninitialized items past `len`, but the pointer is immediately
        // cast from `T` to `MaybeUninit<T>`, so this is safe
        let spare_unsafe_ptr = unsafe { ptr.add(len) };

        // cast from `T` to `MaybeUninit<T>`, making the pointer safe
        let spare_ptr = spare_unsafe_ptr.cast();

        let spare_len = capacity - len;

        unsafe {
            // SAFETY: `ptr` is valid for `len` items
            let init = from_raw_parts_mut(ptr, len);

            // SAFETY: `spare_ptr` points one item past `init`, so they do not overlap
            let spare = from_raw_parts_mut(spare_ptr, spare_len);

            // SAFETY: `len` is actually non-zero, therefore this is safe
            let non_empty = NonEmptySlice::from_mut_slice_unchecked(init);

            (non_empty, spare)
        }
    }
}

type MaybeUninitSlice<T> = [MaybeUninit<T>];

impl<T> NonEmptyVec<T> {
    /// Removes consecutive duplicated items in the vector, as determined by [`PartialEq`].
    ///
    /// If the vector is sorted, this will remove all duplicates.
    pub fn dedup(&mut self)
    where
        T: PartialEq,
    {
        // SAFETY: deduping can not make the vector empty
        unsafe {
            self.as_mut_vec().dedup();
        }
    }

    /// Removes consecutive duplicated items in the vector, as determined by the supplied function.
    ///
    /// The function provided receives mutable references to the items to be compared.
    ///
    /// The items are passed in the opposite order from their order in the vector,
    /// so if `function(a, b)` returns [`true`], then `a` is removed.
    ///
    /// If the vector is sorted, this will remove all duplicates.
    pub fn dedup_by<F: FnMut(&mut T, &mut T) -> bool>(&mut self, function: F) {
        // SAFETY: deduping can not make the vector empty
        unsafe {
            self.as_mut_vec().dedup_by(function);
        }
    }

    /// Removes consecutive duplicated items in the vector, as determined by the keys returned
    /// from the provided function.
    ///
    /// If the vector is sorted, this will remove all duplicates.
    pub fn dedup_by_key<F: FnMut(&mut T) -> K, K: PartialEq>(&mut self, function: F) {
        // SAFETY: deduping can not make the vector empty
        unsafe {
            self.as_mut_vec().dedup_by_key(function);
        }
    }
}

impl<T: Clone> NonEmptyVec<T> {
    /// Resizes the vector in-place so that its length is equal to provided [`Size`].
    ///
    /// If `new` is greater than [`len`], the vector is extended by the difference,
    /// with each additional slot filled with `value` that is repeatedly cloned.
    ///
    /// Otherwise, the vector is simply truncated.
    ///
    /// [`len`]: Self::len
    pub fn resize(&mut self, new: Size, value: T) {
        // SAFETY: resizing to non-zero length can not make the vector empty
        unsafe {
            self.as_mut_vec().resize(new.get(), value);
        }
    }

    /// Extends the vector by cloning all items from the provided value that can be
    /// converted to [`[T]`](prim@slice).
    ///
    /// The `slice` provided is traversed in-order.
    pub fn extend_from<S: AsRef<[T]>>(&mut self, slice: S) {
        // SAFETY: extending can not make the vector empty
        unsafe {
            self.as_mut_vec().extend_from_slice(slice.as_ref());
        }
    }

    /// Given the range within the vector, clones the items in that range
    /// and appends them to the end of the vector.
    ///
    /// # Panics
    ///
    /// Panics if the range is out of bounds.
    pub fn extend_from_within<R: RangeBounds<usize>>(&mut self, range: R) {
        // SAFETY: extending can not make the vector empty
        unsafe {
            self.as_mut_vec().extend_from_within(range);
        }
    }
}

/// Peeks into the last item of the vector mutably.
///
/// This `struct` implements [`Deref`] and [`DerefMut`] to the last item of the vector.
pub struct PeekMut<'a, T> {
    non_empty: &'a mut NonEmptyVec<T>,
}

impl<'a, T> PeekMut<'a, T> {
    /// Constructs [`Self`].
    pub const fn new(non_empty: &'a mut NonEmptyVec<T>) -> Self {
        Self { non_empty }
    }

    /// Removes the last item from the vector and returns it,
    /// or [`None`] if the vector would become empty.
    #[must_use]
    pub fn pop(self) -> Option<T> {
        self.non_empty.pop()
    }
}

impl<T> Deref for PeekMut<'_, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        self.non_empty.last()
    }
}

impl<T> DerefMut for PeekMut<'_, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.non_empty.last_mut()
    }
}

impl<T> NonEmptyVec<T> {
    /// Constructs [`Self`] containing the single value provided.
    pub fn single(value: T) -> Self {
        let vec = vec![value];

        // SAFETY: non-empty construction
        unsafe { Self::new_unchecked(vec) }
    }

    /// Constructs [`Self`] with the specified capacity, pushing the value provided.
    ///
    /// # Panics
    ///
    /// Panics on capacity overflow.
    pub fn with_capacity_and_value(capacity: Size, value: T) -> Self {
        let mut vec = Vec::with_capacity(capacity.get());

        vec.push(value);

        // SAFETY: non-empty construction
        unsafe { Self::new_unchecked(vec) }
    }
}

impl<T> NonEmptyVec<T> {
    /// Returns regular by-reference iterator over the vector.
    pub fn iter(&self) -> Iter<'_, T> {
        self.as_slice().iter()
    }

    /// Returns regular by-mutable-reference iterator over the vector.
    pub fn iter_mut(&mut self) -> IterMut<'_, T> {
        self.as_mut_slice().iter_mut()
    }
}

impl<T> IntoIterator for NonEmptyVec<T> {
    type Item = T;

    type IntoIter = IntoIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        self.into_vec().into_iter()
    }
}

impl<'a, T> IntoIterator for &'a NonEmptyVec<T> {
    type Item = &'a T;

    type IntoIter = Iter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<'a, T> IntoIterator for &'a mut NonEmptyVec<T> {
    type Item = &'a mut T;

    type IntoIter = IterMut<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter_mut()
    }
}

impl<T> Extend<T> for NonEmptyVec<T> {
    fn extend<I: IntoIterator<Item = T>>(&mut self, iterable: I) {
        // SAFETY: extending can not make the vector empty
        unsafe {
            self.as_mut_vec().extend(iterable);
        }
    }
}

impl<'a, T: Copy + 'a> Extend<&'a T> for NonEmptyVec<T> {
    fn extend<I: IntoIterator<Item = &'a T>>(&mut self, iterable: I) {
        // SAFETY: extending can not make the vector empty
        unsafe {
            self.as_mut_vec().extend(iterable);
        }
    }
}

impl<T: Clone> NonEmptyVec<T> {
    /// Constructs [`Self`] by repeating the provided value supplied number of times.
    pub fn repeat(value: T, count: Size) -> Self {
        let vec = vec![value; count.get()];

        // SAFETY: non-empty construction
        unsafe { Self::new_unchecked(vec) }
    }
}

impl<T> NonEmptyVec<T> {
    /// Returns non-empty by-reference iterator over the vector.
    pub fn non_empty_iter(&self) -> NonEmptyIter<'_, T> {
        // SAFETY: the slice is non-empty by construction
        unsafe { NonEmptyAdapter::new(self.iter()) }
    }

    /// Returns non-empty by-mutable-reference iterator over the vector.
    pub fn non_empty_iter_mut(&mut self) -> NonEmptyIterMut<'_, T> {
        // SAFETY: the slice is non-empty by construction
        unsafe { NonEmptyAdapter::new(self.iter_mut()) }
    }
}

impl<T> FromNonEmptyIterator<T> for NonEmptyVec<T> {
    fn from_non_empty_iter<I: IntoNonEmptyIterator<Item = T>>(iterable: I) -> Self {
        let (item, iterator) = iterable.into_non_empty_iter().consume();

        let mut output = Self::single(item);

        output.extend(iterator);

        output
    }
}

impl<T> IntoNonEmptyIterator for NonEmptyVec<T> {
    type IntoNonEmptyIter = IntoNonEmptyIter<T>;

    fn into_non_empty_iter(self) -> Self::IntoNonEmptyIter {
        // SAFETY: the slice is non-empty by construction
        unsafe { NonEmptyAdapter::new(self.into_iter()) }
    }
}

impl<'a, T> IntoNonEmptyIterator for &'a NonEmptyVec<T> {
    type IntoNonEmptyIter = NonEmptyIter<'a, T>;

    fn into_non_empty_iter(self) -> Self::IntoNonEmptyIter {
        self.non_empty_iter()
    }
}

impl<'a, T> IntoNonEmptyIterator for &'a mut NonEmptyVec<T> {
    type IntoNonEmptyIter = NonEmptyIterMut<'a, T>;

    fn into_non_empty_iter(self) -> Self::IntoNonEmptyIter {
        self.non_empty_iter_mut()
    }
}
