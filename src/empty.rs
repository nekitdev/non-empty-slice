//! Emptiness errors.

#[cfg(feature = "diagnostics")]
use miette::Diagnostic;

use thiserror::Error;

/// The message for errors returned on empty slices.
pub const EMPTY: &str = "the slice is empty";

/// Represents errors that occur when the input slice is empty.
#[derive(Debug, Error)]
#[error("the slice is empty")]
#[cfg_attr(
    feature = "diagnostics",
    derive(Diagnostic),
    diagnostic(code(non_empty_str::empty), help("make sure the slice is non-empty"))
)]
pub struct Empty;
