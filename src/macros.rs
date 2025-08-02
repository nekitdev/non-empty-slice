//! Macros for creating non-empty slices in `const` contexts.

/// Constantly constructs [`Slice`] from the given slice, failing compilation if the slice is empty.
///
/// [`Slice`]: crate::slice::Slice
#[macro_export]
macro_rules! const_slice {
    ($slice: expr) => {
        const { $crate::slice::Slice::from_slice($slice).expect($crate::slice::EMPTY) }
    };
}

/// Similar to [`const_slice!`], but constructs [`Bytes`].
///
/// # Examples
///
/// Simple usage:
///
/// ```
/// use non_empty_slice::const_bytes;
///
/// let nekit = const_bytes!(b"nekit");
/// ```
///
/// Compilation failure if the bytes are empty:
///
/// ```compile_fail
/// use non_empty_slice::const_bytes;
///
/// let empty = const_bytes!(b"");
/// ```
///
/// [`Bytes`]: crate::slice::Bytes
#[macro_export]
macro_rules! const_bytes {
    ($bytes: expr) => {
        const { $crate::slice::Bytes::from_slice($bytes).expect($crate::slice::EMPTY) }
    };
}
