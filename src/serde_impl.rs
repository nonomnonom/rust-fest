use alloc::{string::String, vec::Vec};
use serde::{Deserialize, Deserializer, de::Error};

use crate::{NonEmptyString, NonEmptyVec};

impl<'de, T: Deserialize<'de>> Deserialize<'de> for NonEmptyVec<T> {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        Self::try_from(Vec::<T>::deserialize(deserializer)?).map_err(D::Error::custom)
    }
}

impl<'de> Deserialize<'de> for NonEmptyString {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        Self::try_from(String::deserialize(deserializer)?).map_err(D::Error::custom)
    }
}
