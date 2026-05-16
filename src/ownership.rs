#[cfg(not(feature = "ownership"))]
compile_error!("expected `ownership` to be enabled");

#[cfg(any(feature = "std", feature = "alloc"))]
mod impl_into_owned {
    use ownership::IntoOwned;

    use crate::{boxed::NonEmptyBoxedSlice, vec::NonEmptyVec};

    impl<T: IntoOwned> IntoOwned for NonEmptyVec<T> {
        type Owned = NonEmptyVec<T::Owned>;

        fn into_owned(self) -> Self::Owned {
            // SAFETY: `into_owned` can not make the vector empty
            unsafe { Self::Owned::new_unchecked(self.into_vec().into_owned()) }
        }
    }

    impl<T: IntoOwned> IntoOwned for NonEmptyBoxedSlice<T> {
        type Owned = NonEmptyBoxedSlice<T::Owned>;

        fn into_owned(self) -> Self::Owned {
            self.into_non_empty_vec()
                .into_owned()
                .into_non_empty_boxed_slice()
        }
    }
}
