pub mod import {
    pub use core::fmt;
}

macro_rules! debug_empty {
    ($name: ident, $field: ident) => {
        impl<T> $crate::internals::import::fmt::Debug for $name<T> {
            fn fmt(
                &self,
                formatter: &mut $crate::internals::import::fmt::Formatter<'_>,
            ) -> $crate::internals::import::fmt::Result {
                struct DebugEmptySlice;

                impl $crate::internals::import::fmt::Debug for DebugEmptySlice {
                    fn fmt(
                        &self,
                        formatter: &mut $crate::internals::import::fmt::Formatter<'_>,
                    ) -> $crate::internals::import::fmt::Result {
                        formatter.debug_list().finish()
                    }
                }

                formatter
                    .debug_struct(stringify!($name))
                    .field(stringify!($field), &DebugEmptySlice)
                    .finish()
            }
        }
    };
}

pub(crate) use debug_empty;
