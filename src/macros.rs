//! Macros for creating non-empty vectors and slices.

#[doc(hidden)]
pub mod import {
    pub use core::compile_error;

    #[cfg(feature = "std")]
    pub use std::vec;

    #[cfg(all(not(feature = "std"), feature = "alloc"))]
    pub use alloc::vec;

    pub use non_zero_size::const_size;
}

/// Constructs [`NonEmptyVec<T>`] containing the provided arguments.
///
/// # Examples
///
/// Providing no arguments results in compile-time errors:
///
/// ```compile_fail
/// use non_empty_slice::non_empty_vec;
///
/// let never = non_empty_vec![];
/// ```
///
/// Providing one argument:
///
/// ```
/// use non_empty_slice::non_empty_vec;
///
/// let single = non_empty_vec![13];
/// ```
///
/// Providing argument implementing [`Clone`] and non-zero count of type [`Size`]:
///
/// ```
/// use non_empty_slice::non_empty_vec;
/// use non_zero_size::const_size;
///
/// let repeated = non_empty_vec![13; const_size!(42)];
/// ```
///
/// Equivalently, one can use:
///
/// ```
/// use non_empty_slice::non_empty_vec;
///
/// let repeated = non_empty_vec![13; const 42];
/// ```
///
/// Finally, providing multiple arguments:
///
/// ```
/// use non_empty_slice::non_empty_vec;
///
/// let nice = non_empty_vec![13, 42, 69];
/// ```
///
/// [`NonEmptyVec<T>`]: crate::vec::NonEmptyVec
/// [`Size`]: non_zero_size::Size
#[macro_export]
#[cfg(any(feature = "std", feature = "alloc"))]
macro_rules! non_empty_vec {
    () => {
        $crate::macros::import::compile_error!("expected non-empty arguments");
    };
    ($value: expr $(,)?) => {
        $crate::vec::NonEmptyVec::single($value)
    };
    ($value: expr; const $count: expr) => {
        $crate::non_empty_vec!($value; $crate::macros::import::const_size!($count))
    };
    ($value: expr; $count: expr) => {
        $crate::vec::NonEmptyVec::repeat($value, $count)
    };
    ($value: expr, $($rest: expr),+ $(,)?) => {{
        let vector = $crate::macros::import::vec![$value, $($rest),+];

        // SAFETY: the vector is guaranteed to be non-empty due to the macro pattern
        let output = unsafe { $crate::vec::NonEmptyVec::new_unchecked(vector) };

        output
    }};
}

/// Constructs [`NonEmptySlice`] from the given slice, panicking if it is empty.
///
/// [`NonEmptySlice`]: crate::slice::NonEmptySlice
#[macro_export]
macro_rules! non_empty_slice {
    ($slice: expr) => {
        $crate::slice::NonEmptySlice::from_slice($slice).expect($crate::slice::EMPTY_SLICE)
    };
}

/// Similar to [`non_empty_slice!`], but constructs [`NonEmptyBytes`].
///
/// # Examples
///
/// Simple usage:
///
/// ```
/// use non_empty_slice::non_empty_bytes;
///
/// let nekit = non_empty_bytes!(b"nekit");
/// ```
///
/// Panicking if the bytes are empty:
///
/// ```should_panic
/// use non_empty_slice::non_empty_bytes;
///
/// let never = non_empty_bytes!(b"");
/// ```
///
/// Compilation failure in `const` contexts:
///
/// ```compile_fail
/// use non_empty_slice::non_empty_bytes;
///
/// let never = const { non_empty_bytes!(b"") };
/// ```
///
/// [`NonEmptyBytes`]: crate::slice::NonEmptyBytes
#[macro_export]
macro_rules! non_empty_bytes {
    ($bytes: expr) => {
        $crate::slice::NonEmptyBytes::from_slice($bytes).expect($crate::slice::EMPTY_SLICE)
    };
}

/// Similar to [`non_empty_slice!`] but for `const` contexts.
///
/// Note that the provided expression must be const-evaluatable, else the compilation will fail.
#[macro_export]
macro_rules! const_non_empty_slice {
    ($slice: expr) => {
        const { $crate::non_empty_slice!($slice) }
    };
}

/// Similar to [`non_empty_bytes!`] but for `const` contexts.
///
/// Note that the provided expression must be const-evaluatable, else the compilation will fail.
///
/// # Examples
///
/// ```
/// use non_empty_slice::const_non_empty_bytes;
///
/// let message = const_non_empty_bytes!(b"Hello, world!");
/// ```
///
/// Failing compilation on empty bytes:
///
/// ```compile_fail
/// use non_empty_slice::const_non_empty_bytes;
///
/// let never = const_non_empty_bytes!(b"");
/// ```
#[macro_export]
macro_rules! const_non_empty_bytes {
    ($bytes: expr) => {
        const { $crate::non_empty_bytes!($bytes) }
    };
}
