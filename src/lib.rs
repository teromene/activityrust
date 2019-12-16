#[macro_use]
pub mod traits;
pub mod content;
pub mod entities;

use serde::{Serialize, Deserialize};
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

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum OneOrMultiple<T> {
    Vec(Vec<T>),
    Element(T),
}

impl<T> From<T> for OneOrMultiple<T> {
    fn from(element: T) -> OneOrMultiple<T> {
        OneOrMultiple::Element(element)
    }
}

impl<T> OneOrMultiple<T> {
    fn append(&mut self, mut new_element: T) {
        match self {
            OneOrMultiple::Element(element) => {
                OneOrMultiple::Vec(vec![element, &mut new_element]);
            }
            OneOrMultiple::Vec(element) => {
                element.push(new_element);
            }
        }
    }
}
