#[macro_use]
pub mod traits;
pub mod content;
pub mod entities;

use serde::{Deserialize, Serialize, Serializer, Deserializer};

pub trait MaybeOptional<T> {
    fn get_optional(self) -> Option<T>;
    fn get_optional_boxed(self) -> Option<Box<T>>;
}

impl<T> MaybeOptional<T> for T {
    fn get_optional(self) -> Option<T> {
        Some(self)
    }

    fn get_optional_boxed(self) -> Option<Box<T>> {
        Some(Box::new(self))
    }
}

impl<T> MaybeOptional<T> for Option<T> {
    fn get_optional(self) -> Option<T> {
        self
    }

    fn get_optional_boxed(self) -> Option<Box<T>> {
        if let Some(inner) = self {
            Some(Box::new(inner))
        } else {
            None
        }
    }
}




