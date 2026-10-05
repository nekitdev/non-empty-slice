use core::fmt;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct DebugEmptySlice;

impl DebugEmptySlice {
    pub const fn new() -> Self {
        Self
    }

    pub const fn by_ref(&self) -> &Self {
        self
    }
}

impl fmt::Debug for DebugEmptySlice {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.debug_list().finish()
    }
}

pub mod import {
    pub use core::fmt;

    pub use super::DebugEmptySlice;
}

macro_rules! empty {
    ($name: ident => $field: ident) => {
        impl<T> $crate::debug::import::fmt::Debug for $name<T> {
            fn fmt(
                &self,
                formatter: &mut $crate::debug::import::fmt::Formatter<'_>,
            ) -> $crate::debug::import::fmt::Result {
                formatter
                    .debug_struct(stringify!($name))
                    .field(
                        stringify!($field),
                        $crate::debug::import::DebugEmptySlice::new().by_ref(),
                    )
                    .finish()
            }
        }
    };
}

pub(crate) use empty;
