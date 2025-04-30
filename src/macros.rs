//! Macros used for constructing non-empty slices.

/// Constructs [`Slice`] from the given slice, panicking if it is empty.
///
/// [`Slice`]: crate::slice::Slice
#[macro_export]
macro_rules! const_slice {
    ($slice: expr) => {
        $crate::slice::Slice::new_ok($slice).expect($crate::empty::EMPTY)
    };
}

/// Similar to [`const_slice`], but constructs borrowed [`CowSlice`].
///
/// [`CowSlice`]: crate::cow::CowSlice
#[cfg(any(feature = "alloc", feature = "std"))]
#[macro_export]
macro_rules! const_borrowed_slice {
    ($slice: expr) => {
        $crate::cow::CowSlice::borrowed_ok($slice).expect($crate::empty::EMPTY)
    };
}
