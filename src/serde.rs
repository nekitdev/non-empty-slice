#[cfg(not(feature = "serde"))]
compile_error!("expected `serde` to be enabled");

use serde::{Serialize, Serializer};

use crate::slice::NonEmptySlice;

impl<T: Serialize> Serialize for NonEmptySlice<T> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        self.as_slice().serialize(serializer)
    }
}

#[cfg(any(feature = "std", feature = "alloc"))]
mod std_or_alloc {
    cfg_select! {
        feature = "std" => {}
        feature = "alloc" => {
            use alloc::vec::Vec;
        }
        _ => {
            compile_error!("expected either `std` or `alloc` to be enabled");
        }
    }

    use serde::{Deserialize, Deserializer, Serialize, Serializer, de::Error};

    use crate::{boxed::NonEmptyBoxedSlice, vec::NonEmptyVec};

    impl<T: Serialize> Serialize for NonEmptyVec<T> {
        fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
            self.as_vec().serialize(serializer)
        }
    }

    impl<'de, T: Deserialize<'de>> Deserialize<'de> for NonEmptyVec<T> {
        fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
            let maybe_empty = Vec::deserialize(deserializer)?;

            Self::new(maybe_empty).map_err(D::Error::custom)
        }
    }

    // NOTE: `Serialize` is implemented for `Box<U>`, provided `U: Serialize`
    // if `T` is `Serialize`, NonEmptySlice<T>` is `Serialize`, therefore
    // `NonEmptyBoxedSlice<T>` is as well

    impl<'de, T: Deserialize<'de>> Deserialize<'de> for NonEmptyBoxedSlice<T> {
        fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
            let non_empty_vec = NonEmptyVec::deserialize(deserializer)?;

            Ok(non_empty_vec.into_non_empty_boxed_slice())
        }
    }
}
