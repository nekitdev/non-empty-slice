use core::cmp::Ordering;

use crate::slice::NonEmptySlice;

// NonEmptySlice <-> NonEmptySlice

impl<T: PartialEq<U>, U> PartialEq<NonEmptySlice<U>> for NonEmptySlice<T> {
    fn eq(&self, other: &NonEmptySlice<U>) -> bool {
        self == other.as_slice()
    }
}

impl<T: Eq> Eq for NonEmptySlice<T> {}

impl<T: PartialOrd> PartialOrd for NonEmptySlice<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.partial_cmp(other.as_slice())
    }
}

impl<T: Ord> Ord for NonEmptySlice<T> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.as_slice().cmp(other.as_slice())
    }
}

// NonEmptySlice <-> Slice

impl<T: PartialEq<U>, U> PartialEq<[U]> for NonEmptySlice<T> {
    fn eq(&self, other: &[U]) -> bool {
        self.as_slice() == other
    }
}

impl<T: PartialOrd> PartialOrd<[T]> for NonEmptySlice<T> {
    fn partial_cmp(&self, other: &[T]) -> Option<Ordering> {
        self.as_slice().partial_cmp(other)
    }
}

impl<T: PartialEq<U>, U> PartialEq<NonEmptySlice<U>> for [T] {
    fn eq(&self, other: &NonEmptySlice<U>) -> bool {
        self == other.as_slice()
    }
}

impl<T: PartialOrd> PartialOrd<NonEmptySlice<T>> for [T] {
    fn partial_cmp(&self, other: &NonEmptySlice<T>) -> Option<Ordering> {
        self.partial_cmp(other.as_slice())
    }
}

#[cfg(any(feature = "std", feature = "alloc"))]
mod std_or_alloc {
    cfg_select! {
        feature = "std" => {
            use std::borrow::Cow;
        }
        feature = "alloc" => {
            use alloc::{borrow::Cow, boxed::Box, vec::Vec};
        }
        _ => {
            compile_error!("expected either `std` or `alloc` to be enabled");
        }
    }

    use core::cmp::Ordering;

    use crate::{
        boxed::NonEmptyBoxedSlice, cow::NonEmptyCowSlice, slice::NonEmptySlice, vec::NonEmptyVec,
    };

    // NonEmptySlice <-> BoxedSlice

    impl<T: PartialEq<U>, U> PartialEq<Box<[U]>> for NonEmptySlice<T> {
        fn eq(&self, other: &Box<[U]>) -> bool {
            self == other.as_ref()
        }
    }

    impl<T: PartialOrd> PartialOrd<Box<[T]>> for NonEmptySlice<T> {
        fn partial_cmp(&self, other: &Box<[T]>) -> Option<Ordering> {
            self.partial_cmp(other.as_ref())
        }
    }

    impl<T: PartialEq<U>, U> PartialEq<NonEmptySlice<U>> for Box<[T]> {
        fn eq(&self, other: &NonEmptySlice<U>) -> bool {
            self.as_ref() == other
        }
    }

    impl<T: PartialOrd> PartialOrd<NonEmptySlice<T>> for Box<[T]> {
        fn partial_cmp(&self, other: &NonEmptySlice<T>) -> Option<Ordering> {
            self.as_ref().partial_cmp(other)
        }
    }

    // NonEmptySlice <-> CowSlice

    impl<T: PartialEq<U>, U: Clone> PartialEq<Cow<'_, [U]>> for NonEmptySlice<T> {
        fn eq(&self, other: &Cow<'_, [U]>) -> bool {
            self == other.as_ref()
        }
    }

    impl<T: PartialOrd + Clone> PartialOrd<Cow<'_, [T]>> for NonEmptySlice<T> {
        fn partial_cmp(&self, other: &Cow<'_, [T]>) -> Option<Ordering> {
            self.partial_cmp(other.as_ref())
        }
    }

    impl<T: PartialEq<U> + Clone, U> PartialEq<NonEmptySlice<U>> for Cow<'_, [T]> {
        fn eq(&self, other: &NonEmptySlice<U>) -> bool {
            self.as_ref() == other
        }
    }

    impl<T: PartialOrd + Clone> PartialOrd<NonEmptySlice<T>> for Cow<'_, [T]> {
        fn partial_cmp(&self, other: &NonEmptySlice<T>) -> Option<Ordering> {
            self.as_ref().partial_cmp(other)
        }
    }

    // NonEmptySlice <-> Vec

    impl<T: PartialEq<U>, U> PartialEq<Vec<U>> for NonEmptySlice<T> {
        fn eq(&self, other: &Vec<U>) -> bool {
            self == other.as_slice()
        }
    }

    impl<T: PartialOrd> PartialOrd<Vec<T>> for NonEmptySlice<T> {
        fn partial_cmp(&self, other: &Vec<T>) -> Option<Ordering> {
            self.partial_cmp(other.as_slice())
        }
    }

    impl<T: PartialEq<U>, U> PartialEq<NonEmptySlice<U>> for Vec<T> {
        fn eq(&self, other: &NonEmptySlice<U>) -> bool {
            self.as_slice() == other
        }
    }

    impl<T: PartialOrd> PartialOrd<NonEmptySlice<T>> for Vec<T> {
        fn partial_cmp(&self, other: &NonEmptySlice<T>) -> Option<Ordering> {
            self.as_slice().partial_cmp(other)
        }
    }

    // NonEmptySlice <-> NonEmptyCowSlice

    impl<T: PartialEq<U>, U: Clone> PartialEq<NonEmptyCowSlice<'_, U>> for NonEmptySlice<T> {
        fn eq(&self, other: &NonEmptyCowSlice<'_, U>) -> bool {
            self == other.as_ref()
        }
    }

    impl<T: PartialOrd + Clone> PartialOrd<NonEmptyCowSlice<'_, T>> for NonEmptySlice<T> {
        fn partial_cmp(&self, other: &NonEmptyCowSlice<'_, T>) -> Option<Ordering> {
            self.partial_cmp(other.as_ref())
        }
    }

    impl<T: PartialEq<U> + Clone, U> PartialEq<NonEmptySlice<U>> for NonEmptyCowSlice<'_, T> {
        fn eq(&self, other: &NonEmptySlice<U>) -> bool {
            self.as_ref() == other
        }
    }

    impl<T: PartialOrd + Clone> PartialOrd<NonEmptySlice<T>> for NonEmptyCowSlice<'_, T> {
        fn partial_cmp(&self, other: &NonEmptySlice<T>) -> Option<Ordering> {
            self.as_ref().partial_cmp(other)
        }
    }

    // NonEmptyVec <-> NonEmptyVec

    impl<T: PartialEq<U>, U> PartialEq<NonEmptyVec<U>> for NonEmptyVec<T> {
        fn eq(&self, other: &NonEmptyVec<U>) -> bool {
            self == other.as_vec()
        }
    }

    impl<T: Eq> Eq for NonEmptyVec<T> {}

    impl<T: PartialOrd> PartialOrd for NonEmptyVec<T> {
        fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
            self.partial_cmp(other.as_vec())
        }
    }

    impl<T: Ord> Ord for NonEmptyVec<T> {
        fn cmp(&self, other: &Self) -> Ordering {
            self.as_vec().cmp(other.as_vec())
        }
    }

    // NonEmptyVec <-> Vec

    impl<T: PartialEq<U>, U> PartialEq<Vec<U>> for NonEmptyVec<T> {
        fn eq(&self, other: &Vec<U>) -> bool {
            self.as_vec() == other
        }
    }

    impl<T: PartialOrd> PartialOrd<Vec<T>> for NonEmptyVec<T> {
        fn partial_cmp(&self, other: &Vec<T>) -> Option<Ordering> {
            self.as_vec().partial_cmp(other)
        }
    }

    impl<T: PartialEq<U>, U> PartialEq<NonEmptyVec<U>> for Vec<T> {
        fn eq(&self, other: &NonEmptyVec<U>) -> bool {
            self == other.as_vec()
        }
    }

    impl<T: PartialOrd> PartialOrd<NonEmptyVec<T>> for Vec<T> {
        fn partial_cmp(&self, other: &NonEmptyVec<T>) -> Option<Ordering> {
            self.partial_cmp(other.as_vec())
        }
    }

    // NonEmptyVec <-> NonEmptySlice

    impl<T: PartialEq<U>, U> PartialEq<NonEmptySlice<U>> for NonEmptyVec<T> {
        fn eq(&self, other: &NonEmptySlice<U>) -> bool {
            self.as_non_empty_slice() == other
        }
    }

    impl<T: PartialOrd> PartialOrd<NonEmptySlice<T>> for NonEmptyVec<T> {
        fn partial_cmp(&self, other: &NonEmptySlice<T>) -> Option<Ordering> {
            self.as_non_empty_slice().partial_cmp(other)
        }
    }

    impl<T: PartialEq<U>, U> PartialEq<NonEmptyVec<U>> for NonEmptySlice<T> {
        fn eq(&self, other: &NonEmptyVec<U>) -> bool {
            self == other.as_non_empty_slice()
        }
    }

    impl<T: PartialOrd> PartialOrd<NonEmptyVec<T>> for NonEmptySlice<T> {
        fn partial_cmp(&self, other: &NonEmptyVec<T>) -> Option<Ordering> {
            self.partial_cmp(other.as_non_empty_slice())
        }
    }

    // NonEmptyVec <-> Slice

    impl<T: PartialEq<U>, U> PartialEq<[U]> for NonEmptyVec<T> {
        fn eq(&self, other: &[U]) -> bool {
            self.as_slice() == other
        }
    }

    impl<T: PartialOrd> PartialOrd<[T]> for NonEmptyVec<T> {
        fn partial_cmp(&self, other: &[T]) -> Option<Ordering> {
            self.as_slice().partial_cmp(other)
        }
    }

    impl<T: PartialEq<U>, U> PartialEq<NonEmptyVec<U>> for [T] {
        fn eq(&self, other: &NonEmptyVec<U>) -> bool {
            self == other.as_slice()
        }
    }

    impl<T: PartialOrd> PartialOrd<NonEmptyVec<T>> for [T] {
        fn partial_cmp(&self, other: &NonEmptyVec<T>) -> Option<Ordering> {
            self.partial_cmp(other.as_slice())
        }
    }

    // NonEmptyVec <-> NonEmptyCowSlice

    impl<T: PartialEq<U>, U: Clone> PartialEq<NonEmptyCowSlice<'_, U>> for NonEmptyVec<T> {
        fn eq(&self, other: &NonEmptyCowSlice<'_, U>) -> bool {
            self == other.as_ref()
        }
    }

    impl<T: PartialOrd + Clone> PartialOrd<NonEmptyCowSlice<'_, T>> for NonEmptyVec<T> {
        fn partial_cmp(&self, other: &NonEmptyCowSlice<'_, T>) -> Option<Ordering> {
            self.partial_cmp(other.as_ref())
        }
    }

    impl<T: PartialEq<U> + Clone, U> PartialEq<NonEmptyVec<U>> for NonEmptyCowSlice<'_, T> {
        fn eq(&self, other: &NonEmptyVec<U>) -> bool {
            self.as_ref() == other
        }
    }

    impl<T: PartialOrd + Clone> PartialOrd<NonEmptyVec<T>> for NonEmptyCowSlice<'_, T> {
        fn partial_cmp(&self, other: &NonEmptyVec<T>) -> Option<Ordering> {
            self.as_ref().partial_cmp(other)
        }
    }

    // NonEmptyVec <-> CowSlice

    impl<T: PartialEq<U>, U: Clone> PartialEq<Cow<'_, [U]>> for NonEmptyVec<T> {
        fn eq(&self, other: &Cow<'_, [U]>) -> bool {
            self == other.as_ref()
        }
    }

    impl<T: PartialOrd + Clone> PartialOrd<Cow<'_, [T]>> for NonEmptyVec<T> {
        fn partial_cmp(&self, other: &Cow<'_, [T]>) -> Option<Ordering> {
            self.partial_cmp(other.as_ref())
        }
    }

    impl<T: PartialEq<U> + Clone, U> PartialEq<NonEmptyVec<U>> for Cow<'_, [T]> {
        fn eq(&self, other: &NonEmptyVec<U>) -> bool {
            self.as_ref() == other
        }
    }

    impl<T: PartialOrd + Clone> PartialOrd<NonEmptyVec<T>> for Cow<'_, [T]> {
        fn partial_cmp(&self, other: &NonEmptyVec<T>) -> Option<Ordering> {
            self.as_ref().partial_cmp(other)
        }
    }

    // NonEmptyVec <-> NonEmptyBoxedSlice

    impl<T: PartialEq<U>, U> PartialEq<NonEmptyBoxedSlice<U>> for NonEmptyVec<T> {
        fn eq(&self, other: &NonEmptyBoxedSlice<U>) -> bool {
            self == other.as_ref()
        }
    }

    impl<T: PartialOrd> PartialOrd<NonEmptyBoxedSlice<T>> for NonEmptyVec<T> {
        fn partial_cmp(&self, other: &NonEmptyBoxedSlice<T>) -> Option<Ordering> {
            self.partial_cmp(other.as_ref())
        }
    }

    impl<T: PartialEq<U>, U> PartialEq<NonEmptyVec<U>> for NonEmptyBoxedSlice<T> {
        fn eq(&self, other: &NonEmptyVec<U>) -> bool {
            self.as_ref() == other
        }
    }

    impl<T: PartialOrd> PartialOrd<NonEmptyVec<T>> for NonEmptyBoxedSlice<T> {
        fn partial_cmp(&self, other: &NonEmptyVec<T>) -> Option<Ordering> {
            self.as_ref().partial_cmp(other)
        }
    }

    // NonEmptyVec <-> BoxedSlice

    impl<T: PartialEq<U>, U> PartialEq<Box<[U]>> for NonEmptyVec<T> {
        fn eq(&self, other: &Box<[U]>) -> bool {
            self == other.as_ref()
        }
    }

    impl<T: PartialOrd> PartialOrd<Box<[T]>> for NonEmptyVec<T> {
        fn partial_cmp(&self, other: &Box<[T]>) -> Option<Ordering> {
            self.partial_cmp(other.as_ref())
        }
    }

    impl<T: PartialEq<U>, U> PartialEq<NonEmptyVec<U>> for Box<[T]> {
        fn eq(&self, other: &NonEmptyVec<U>) -> bool {
            self.as_ref() == other.as_slice()
        }
    }

    impl<T: PartialOrd> PartialOrd<NonEmptyVec<T>> for Box<[T]> {
        fn partial_cmp(&self, other: &NonEmptyVec<T>) -> Option<Ordering> {
            self.as_ref().partial_cmp(other.as_slice())
        }
    }

    // NonEmptyBoxedSlice <-> NonEmptySlice

    impl<T: PartialEq<U>, U> PartialEq<NonEmptySlice<U>> for NonEmptyBoxedSlice<T> {
        fn eq(&self, other: &NonEmptySlice<U>) -> bool {
            self.as_ref() == other
        }
    }

    impl<T: PartialOrd> PartialOrd<NonEmptySlice<T>> for NonEmptyBoxedSlice<T> {
        fn partial_cmp(&self, other: &NonEmptySlice<T>) -> Option<Ordering> {
            self.as_ref().partial_cmp(other)
        }
    }

    impl<T: PartialEq<U>, U> PartialEq<NonEmptyBoxedSlice<U>> for NonEmptySlice<T> {
        fn eq(&self, other: &NonEmptyBoxedSlice<U>) -> bool {
            self == other.as_ref()
        }
    }

    impl<T: PartialOrd> PartialOrd<NonEmptyBoxedSlice<T>> for NonEmptySlice<T> {
        fn partial_cmp(&self, other: &NonEmptyBoxedSlice<T>) -> Option<Ordering> {
            self.partial_cmp(other.as_ref())
        }
    }

    // NonEmptyBoxedSlice <-> Slice

    impl<T: PartialEq<U>, U> PartialEq<[U]> for NonEmptyBoxedSlice<T> {
        fn eq(&self, other: &[U]) -> bool {
            self.as_ref() == other
        }
    }

    impl<T: PartialOrd> PartialOrd<[T]> for NonEmptyBoxedSlice<T> {
        fn partial_cmp(&self, other: &[T]) -> Option<Ordering> {
            self.as_ref().partial_cmp(other)
        }
    }

    impl<T: PartialEq<U>, U> PartialEq<NonEmptyBoxedSlice<U>> for [T] {
        fn eq(&self, other: &NonEmptyBoxedSlice<U>) -> bool {
            self == other.as_ref()
        }
    }

    impl<T: PartialOrd> PartialOrd<NonEmptyBoxedSlice<T>> for [T] {
        fn partial_cmp(&self, other: &NonEmptyBoxedSlice<T>) -> Option<Ordering> {
            self.partial_cmp(other.as_ref())
        }
    }

    // NonEmptyBoxedSlice <-> BoxedSlice

    impl<T: PartialEq<U>, U> PartialEq<Box<[U]>> for NonEmptyBoxedSlice<T> {
        fn eq(&self, other: &Box<[U]>) -> bool {
            self.as_ref() == other
        }
    }

    impl<T: PartialOrd> PartialOrd<Box<[T]>> for NonEmptyBoxedSlice<T> {
        fn partial_cmp(&self, other: &Box<[T]>) -> Option<Ordering> {
            self.as_ref().partial_cmp(other)
        }
    }

    impl<T: PartialEq<U>, U> PartialEq<NonEmptyBoxedSlice<U>> for Box<[T]> {
        fn eq(&self, other: &NonEmptyBoxedSlice<U>) -> bool {
            self == other.as_ref()
        }
    }

    impl<T: PartialOrd> PartialOrd<NonEmptyBoxedSlice<T>> for Box<[T]> {
        fn partial_cmp(&self, other: &NonEmptyBoxedSlice<T>) -> Option<Ordering> {
            self.partial_cmp(other.as_ref())
        }
    }

    // NonEmptyBoxedSlice <-> NonEmptyCowSlice

    impl<T: PartialEq<U>, U: Clone> PartialEq<NonEmptyCowSlice<'_, U>> for NonEmptyBoxedSlice<T> {
        fn eq(&self, other: &NonEmptyCowSlice<'_, U>) -> bool {
            self == other.as_ref()
        }
    }

    impl<T: PartialOrd + Clone> PartialOrd<NonEmptyCowSlice<'_, T>> for NonEmptyBoxedSlice<T> {
        fn partial_cmp(&self, other: &NonEmptyCowSlice<'_, T>) -> Option<Ordering> {
            self.partial_cmp(other.as_ref())
        }
    }

    impl<T: PartialEq<U> + Clone, U> PartialEq<NonEmptyBoxedSlice<U>> for NonEmptyCowSlice<'_, T> {
        fn eq(&self, other: &NonEmptyBoxedSlice<U>) -> bool {
            self.as_ref() == other
        }
    }

    impl<T: PartialOrd + Clone> PartialOrd<NonEmptyBoxedSlice<T>> for NonEmptyCowSlice<'_, T> {
        fn partial_cmp(&self, other: &NonEmptyBoxedSlice<T>) -> Option<Ordering> {
            self.as_ref().partial_cmp(other)
        }
    }

    // NonEmptyBoxedSlice <-> CowSlice

    impl<T: PartialEq<U>, U: Clone> PartialEq<Cow<'_, [U]>> for NonEmptyBoxedSlice<T> {
        fn eq(&self, other: &Cow<'_, [U]>) -> bool {
            self == other.as_ref()
        }
    }

    impl<T: PartialOrd + Clone> PartialOrd<Cow<'_, [T]>> for NonEmptyBoxedSlice<T> {
        fn partial_cmp(&self, other: &Cow<'_, [T]>) -> Option<Ordering> {
            self.partial_cmp(other.as_ref())
        }
    }

    impl<T: PartialEq<U> + Clone, U> PartialEq<NonEmptyBoxedSlice<U>> for Cow<'_, [T]> {
        fn eq(&self, other: &NonEmptyBoxedSlice<U>) -> bool {
            self.as_ref() == other
        }
    }

    impl<T: PartialOrd + Clone> PartialOrd<NonEmptyBoxedSlice<T>> for Cow<'_, [T]> {
        fn partial_cmp(&self, other: &NonEmptyBoxedSlice<T>) -> Option<Ordering> {
            self.as_ref().partial_cmp(other)
        }
    }

    // NonEmptyBoxedSlice <-> Vec

    impl<T: PartialEq<U>, U> PartialEq<Vec<U>> for NonEmptyBoxedSlice<T> {
        fn eq(&self, other: &Vec<U>) -> bool {
            self.as_ref() == other
        }
    }

    impl<T: PartialOrd> PartialOrd<Vec<T>> for NonEmptyBoxedSlice<T> {
        fn partial_cmp(&self, other: &Vec<T>) -> Option<Ordering> {
            self.as_ref().partial_cmp(other)
        }
    }

    impl<T: PartialEq<U>, U> PartialEq<NonEmptyBoxedSlice<U>> for Vec<T> {
        fn eq(&self, other: &NonEmptyBoxedSlice<U>) -> bool {
            self == other.as_ref()
        }
    }

    impl<T: PartialOrd> PartialOrd<NonEmptyBoxedSlice<T>> for Vec<T> {
        fn partial_cmp(&self, other: &NonEmptyBoxedSlice<T>) -> Option<Ordering> {
            self.partial_cmp(other.as_ref())
        }
    }
}
