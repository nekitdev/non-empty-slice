pub mod import {
    pub use core::fmt;
}

macro_rules! debug {
    ($name: ident, $field: ident) => {
        impl<T> $crate::format::import::fmt::Debug for $name<T> {
            fn fmt(
                &self,
                formatter: &mut $crate::format::import::fmt::Formatter<'_>,
            ) -> $crate::format::import::fmt::Result {
                struct DebugEmptySlice;

                impl $crate::format::import::fmt::Debug for DebugEmptySlice {
                    fn fmt(
                        &self,
                        formatter: &mut $crate::format::import::fmt::Formatter<'_>,
                    ) -> $crate::format::import::fmt::Result {
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

pub(crate) use debug;
