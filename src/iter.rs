//! Various non-empty iterators over non-empty vectors and slices.

#[cfg(feature = "std")]
use std::vec::IntoIter;

#[cfg(all(not(feature = "std"), feature = "alloc"))]
use alloc::vec::IntoIter;

use core::{
    iter::Map,
    slice::{self, Iter, IterMut},
};

use non_zero_size::Size;

use non_empty_iter::{NonEmptyAdapter, NonEmptyIterator};

use crate::slice::{NonEmptyBytes, NonEmptySlice};

/// Represents non-empty by-value iterators.
#[cfg(any(feature = "std", feature = "alloc"))]
pub type IntoNonEmptyIter<T> = NonEmptyAdapter<IntoIter<T>>;

/// Represents non-empty by-reference iterators.
pub type NonEmptyIter<'a, T> = NonEmptyAdapter<Iter<'a, T>>;

/// Represents non-empty by-mutable-reference iterators.
pub type NonEmptyIterMut<'a, T> = NonEmptyAdapter<IterMut<'a, T>>;

/// Represents functions mapping chunks to non-empty slices.
///
/// This is mostly an implementation detail, though it can be useful in case
/// one needs to name the type of the iterator explicitly.
pub type NonEmptySliceFn<'a, T> = fn(&'a [T]) -> &'a NonEmptySlice<T>;

/// Represents functions mapping mutable chunks to non-empty mutable slices.
///
/// This is mostly an implementation detail, though it can be useful in case
/// one needs to name the type of the iterator explicitly.
pub type NonEmptyMutSliceFn<'a, T> = fn(&'a mut [T]) -> &'a mut NonEmptySlice<T>;

/// Represents non-empty iterators over non-empty slices in (non-overlapping) chunks,
/// starting at the beginning of the non-empty slice.
///
/// This `struct` is created by the [`chunks`] method on [`NonEmptySlice<T>`].
///
/// [`chunks`]: NonEmptySlice::chunks
pub struct Chunks<'a, T> {
    slice: &'a NonEmptySlice<T>,
    size: Size,
}

impl<'a, T> Chunks<'a, T> {
    /// Constructs [`Self`].
    pub const fn new(slice: &'a NonEmptySlice<T>, size: Size) -> Self {
        Self { slice, size }
    }
}

impl<'a, T> IntoIterator for Chunks<'a, T> {
    type Item = &'a NonEmptySlice<T>;

    type IntoIter = Map<slice::Chunks<'a, T>, NonEmptySliceFn<'a, T>>;

    fn into_iter(self) -> Self::IntoIter {
        self.slice
            .as_slice()
            .chunks(self.size.get())
            // SAFETY: chunks are never empty
            .map(|chunk| unsafe { NonEmptySlice::from_slice_unchecked(chunk) })
    }
}

unsafe impl<T> NonEmptyIterator for Chunks<'_, T> {}

/// Represents non-empty iterators over non-empty slices in (non-overlapping) mutable chunks,
/// starting at the beginning of the non-empty slice.
///
/// This `struct` is created by the [`chunks_mut`] method on [`NonEmptySlice<T>`].
///
/// [`chunks_mut`]: NonEmptySlice::chunks_mut
pub struct ChunksMut<'a, T> {
    slice: &'a mut NonEmptySlice<T>,
    size: Size,
}

impl<'a, T> ChunksMut<'a, T> {
    /// Constructs [`Self`].
    pub const fn new(slice: &'a mut NonEmptySlice<T>, size: Size) -> Self {
        Self { slice, size }
    }
}

impl<'a, T> IntoIterator for ChunksMut<'a, T> {
    type Item = &'a mut NonEmptySlice<T>;

    type IntoIter = Map<slice::ChunksMut<'a, T>, NonEmptyMutSliceFn<'a, T>>;

    fn into_iter(self) -> Self::IntoIter {
        self.slice
            .as_mut_slice()
            .chunks_mut(self.size.get())
            // SAFETY: chunks are never empty
            .map(|chunk| unsafe { NonEmptySlice::from_mut_slice_unchecked(chunk) })
    }
}

unsafe impl<T> NonEmptyIterator for ChunksMut<'_, T> {}

/// Represents non-empty iterators over non-empty slices in (non-overlapping) chunks,
/// starting at the end of the non-empty slice.
///
/// This `struct` is created by the [`rchunks`] method on [`NonEmptySlice<T>`].
///
/// [`rchunks`]: NonEmptySlice::rchunks
pub struct RChunks<'a, T> {
    slice: &'a NonEmptySlice<T>,
    size: Size,
}

impl<'a, T> RChunks<'a, T> {
    /// Constructs [`Self`].
    pub const fn new(slice: &'a NonEmptySlice<T>, size: Size) -> Self {
        Self { slice, size }
    }
}

unsafe impl<T> NonEmptyIterator for RChunks<'_, T> {}

impl<'a, T> IntoIterator for RChunks<'a, T> {
    type Item = &'a NonEmptySlice<T>;

    type IntoIter = Map<slice::RChunks<'a, T>, NonEmptySliceFn<'a, T>>;

    fn into_iter(self) -> Self::IntoIter {
        self.slice
            .as_slice()
            .rchunks(self.size.get())
            // SAFETY: chunks are never empty
            .map(|chunk| unsafe { NonEmptySlice::from_slice_unchecked(chunk) })
    }
}

/// Represents non-empty iterators over non-empty slices in (non-overlapping) mutable chunks,
/// starting at the end of the non-empty slice.
///
/// This `struct` is created by the [`rchunks_mut`] method on [`NonEmptySlice<T>`].
///
/// [`rchunks_mut`]: NonEmptySlice::rchunks_mut
pub struct RChunksMut<'a, T> {
    slice: &'a mut NonEmptySlice<T>,
    size: Size,
}

impl<'a, T> RChunksMut<'a, T> {
    /// Constructs [`Self`].
    pub const fn new(slice: &'a mut NonEmptySlice<T>, size: Size) -> Self {
        Self { slice, size }
    }
}

impl<'a, T> IntoIterator for RChunksMut<'a, T> {
    type Item = &'a mut NonEmptySlice<T>;

    type IntoIter = Map<slice::RChunksMut<'a, T>, NonEmptyMutSliceFn<'a, T>>;

    fn into_iter(self) -> Self::IntoIter {
        self.slice
            .as_mut_slice()
            .rchunks_mut(self.size.get())
            // SAFETY: chunks are never empty
            .map(|chunk| unsafe { NonEmptySlice::from_mut_slice_unchecked(chunk) })
    }
}

unsafe impl<T> NonEmptyIterator for RChunksMut<'_, T> {}

/// Represents non-empty iterators over non-empty slices in (non-overlapping) chunks,
/// starting at the beginning of the non-empty slice.
///
/// When the length of the non-empty slice is not divisible by the chunk size,
/// the last chunk will be omitted.
///
/// This `struct` is created by the [`chunks_exact`] method on [`NonEmptySlice<T>`].
///
/// [`chunks_exact`]: NonEmptySlice::chunks_exact
pub struct ChunksExact<'a, T> {
    slice: &'a NonEmptySlice<T>,
    size: Size,
}

impl<'a, T> ChunksExact<'a, T> {
    /// Constructs [`Self`].
    pub const fn new(slice: &'a NonEmptySlice<T>, size: Size) -> Self {
        Self { slice, size }
    }
}

impl<'a, T> IntoIterator for ChunksExact<'a, T> {
    type Item = &'a NonEmptySlice<T>;

    type IntoIter = Map<slice::ChunksExact<'a, T>, NonEmptySliceFn<'a, T>>;

    fn into_iter(self) -> Self::IntoIter {
        self.slice
            .as_slice()
            .chunks_exact(self.size.get())
            // SAFETY: chunks are never empty
            .map(|chunk| unsafe { NonEmptySlice::from_slice_unchecked(chunk) })
    }
}

unsafe impl<T> NonEmptyIterator for ChunksExact<'_, T> {}

/// Represents non-empty iterators over non-empty slices in (non-overlapping) mutable chunks,
/// starting at the beginning of the non-empty slice.
///
/// When the length of the non-empty slice is not divisible by the chunk size,
/// the last chunk will be omitted.
///
/// This `struct` is created by the [`chunks_exact_mut`] method on [`NonEmptySlice<T>`].
///
/// [`chunks_exact_mut`]: NonEmptySlice::chunks_exact_mut
pub struct ChunksExactMut<'a, T> {
    slice: &'a mut NonEmptySlice<T>,
    size: Size,
}

impl<'a, T> ChunksExactMut<'a, T> {
    /// Constructs [`Self`].
    pub const fn new(slice: &'a mut NonEmptySlice<T>, size: Size) -> Self {
        Self { slice, size }
    }
}

impl<'a, T> IntoIterator for ChunksExactMut<'a, T> {
    type Item = &'a mut NonEmptySlice<T>;

    type IntoIter = Map<slice::ChunksExactMut<'a, T>, NonEmptyMutSliceFn<'a, T>>;

    fn into_iter(self) -> Self::IntoIter {
        self.slice
            .as_mut_slice()
            .chunks_exact_mut(self.size.get())
            // SAFETY: chunks are never empty
            .map(|chunk| unsafe { NonEmptySlice::from_mut_slice_unchecked(chunk) })
    }
}

unsafe impl<T> NonEmptyIterator for ChunksExactMut<'_, T> {}

/// Represents non-empty iterators over non-empty slices in (non-overlapping) chunks,
/// starting at the end of the non-empty slice.
///
/// When the length of the non-empty slice is not divisible by the chunk size,
/// the last chunk will be omitted.
///
/// This `struct` is created by the [`rchunks_exact`] method on [`NonEmptySlice<T>`].
///
/// [`rchunks_exact`]: NonEmptySlice::rchunks_exact
pub struct RChunksExact<'a, T> {
    slice: &'a NonEmptySlice<T>,
    size: Size,
}

impl<'a, T> RChunksExact<'a, T> {
    /// Constructs [`Self`].
    pub const fn new(slice: &'a NonEmptySlice<T>, size: Size) -> Self {
        Self { slice, size }
    }
}

impl<'a, T> IntoIterator for RChunksExact<'a, T> {
    type Item = &'a NonEmptySlice<T>;

    type IntoIter = Map<slice::RChunksExact<'a, T>, NonEmptySliceFn<'a, T>>;

    fn into_iter(self) -> Self::IntoIter {
        self.slice
            .as_slice()
            .rchunks_exact(self.size.get())
            // SAFETY: chunks are never empty
            .map(|chunk| unsafe { NonEmptySlice::from_slice_unchecked(chunk) })
    }
}

unsafe impl<T> NonEmptyIterator for RChunksExact<'_, T> {}

/// Represents non-empty iterators over non-empty slices in (non-overlapping) mutable chunks,
/// starting at the end of the non-empty slice.
///
/// When the length of the non-empty slice is not divisible by the chunk size,
/// the last chunk will be omitted.
///
/// This `struct` is created by the [`rchunks_exact_mut`] method on [`NonEmptySlice<T>`].
///
/// [`rchunks_exact_mut`]: NonEmptySlice::rchunks_exact_mut
pub struct RChunksExactMut<'a, T> {
    slice: &'a mut NonEmptySlice<T>,
    size: Size,
}

impl<'a, T> RChunksExactMut<'a, T> {
    /// Constructs [`Self`].
    pub const fn new(slice: &'a mut NonEmptySlice<T>, size: Size) -> Self {
        Self { slice, size }
    }
}

impl<'a, T> IntoIterator for RChunksExactMut<'a, T> {
    type Item = &'a mut NonEmptySlice<T>;

    type IntoIter = Map<slice::RChunksExactMut<'a, T>, NonEmptyMutSliceFn<'a, T>>;

    fn into_iter(self) -> Self::IntoIter {
        self.slice
            .as_mut_slice()
            .rchunks_exact_mut(self.size.get())
            // SAFETY: chunks are never empty
            .map(|chunk| unsafe { NonEmptySlice::from_mut_slice_unchecked(chunk) })
    }
}

unsafe impl<T> NonEmptyIterator for RChunksExactMut<'_, T> {}

/// Represents non-empty iterators over non-empty slices in (overlapping) windows.
///
/// This `struct` is created by the [`windows`] method on [`NonEmptySlice<T>`].
///
/// [`windows`]: NonEmptySlice::windows
pub struct Windows<'a, T> {
    slice: &'a NonEmptySlice<T>,
    size: Size,
}

impl<'a, T> Windows<'a, T> {
    /// Constructs [`Self`].
    pub const fn new(slice: &'a NonEmptySlice<T>, size: Size) -> Self {
        Self { slice, size }
    }
}

impl<'a, T> IntoIterator for Windows<'a, T> {
    type Item = &'a NonEmptySlice<T>;

    type IntoIter = Map<slice::Windows<'a, T>, NonEmptySliceFn<'a, T>>;

    fn into_iter(self) -> Self::IntoIter {
        self.slice
            .as_slice()
            .windows(self.size.get())
            // SAFETY: windows are never empty
            .map(|window| unsafe { NonEmptySlice::from_slice_unchecked(window) })
    }
}

unsafe impl<T> NonEmptyIterator for Windows<'_, T> {}

/// Represents non-empty iterators over non-empty slices in (non-overlapping) chunks,
/// separated by the given predicate.
///
/// This `struct` is created by the [`chunk_by`] method on [`NonEmptySlice<T>`].
///
/// [`chunk_by`]: NonEmptySlice::chunk_by
pub struct ChunkBy<'a, T, P: FnMut(&T, &T) -> bool> {
    slice: &'a NonEmptySlice<T>,
    predicate: P,
}

impl<'a, T, P: FnMut(&T, &T) -> bool> ChunkBy<'a, T, P> {
    /// Constructs [`Self`].
    pub const fn new(slice: &'a NonEmptySlice<T>, predicate: P) -> Self {
        Self { slice, predicate }
    }
}

impl<'a, T, P: FnMut(&T, &T) -> bool> IntoIterator for ChunkBy<'a, T, P> {
    type Item = &'a NonEmptySlice<T>;

    type IntoIter = Map<slice::ChunkBy<'a, T, P>, NonEmptySliceFn<'a, T>>;

    fn into_iter(self) -> Self::IntoIter {
        self.slice
            .as_slice()
            .chunk_by(self.predicate)
            // SAFETY: chunks are never empty
            .map(|chunk| unsafe { NonEmptySlice::from_slice_unchecked(chunk) })
    }
}

unsafe impl<T, P: FnMut(&T, &T) -> bool> NonEmptyIterator for ChunkBy<'_, T, P> {}

/// Represents non-empty iterators over non-empty slices in (non-overlapping) mutable chunks,
/// separated by the given predicate.
///
/// This `struct` is created by the [`chunk_by_mut`] method on [`NonEmptySlice<T>`].
///
/// [`chunk_by_mut`]: NonEmptySlice::chunk_by_mut
pub struct ChunkByMut<'a, T, P: FnMut(&T, &T) -> bool> {
    slice: &'a mut NonEmptySlice<T>,
    predicate: P,
}

impl<'a, T, P: FnMut(&T, &T) -> bool> ChunkByMut<'a, T, P> {
    /// Constructs [`Self`].
    pub const fn new(slice: &'a mut NonEmptySlice<T>, predicate: P) -> Self {
        Self { slice, predicate }
    }
}

impl<'a, T, P: FnMut(&T, &T) -> bool> IntoIterator for ChunkByMut<'a, T, P> {
    type Item = &'a mut NonEmptySlice<T>;

    type IntoIter = Map<slice::ChunkByMut<'a, T, P>, NonEmptyMutSliceFn<'a, T>>;

    fn into_iter(self) -> Self::IntoIter {
        self.slice
            .as_mut_slice()
            .chunk_by_mut(self.predicate)
            // SAFETY: chunks are never empty
            .map(|chunk| unsafe { NonEmptySlice::from_mut_slice_unchecked(chunk) })
    }
}

unsafe impl<T, P: FnMut(&T, &T) -> bool> NonEmptyIterator for ChunkByMut<'_, T, P> {}

/// Represents non-empty iterators that produce escaped versions of provided slices,
/// treating them as ASCII strings.
///
/// This `struct` is created by the [`escape_ascii`] method on [`NonEmptyBytes`].
///
/// [`escape_ascii`]: NonEmptyBytes::escape_ascii
pub struct EscapeAscii<'a> {
    bytes: &'a NonEmptyBytes,
}

impl<'a> EscapeAscii<'a> {
    /// Constructs [`Self`].
    pub const fn new(bytes: &'a NonEmptyBytes) -> Self {
        Self { bytes }
    }
}

impl<'a> IntoIterator for EscapeAscii<'a> {
    type Item = u8;

    type IntoIter = slice::EscapeAscii<'a>;

    fn into_iter(self) -> Self::IntoIter {
        self.bytes.as_slice().escape_ascii()
    }
}

unsafe impl NonEmptyIterator for EscapeAscii<'_> {}
