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

#[derive(Debug, PartialEq, Default)]
pub struct SingularVec<T>(Vec<T>);

impl<T> From<Vec<T>> for SingularVec<T> {
	fn from(vec: Vec<T>) -> SingularVec<T> {
		SingularVec(vec)
	}
}

impl<T> AsRef<Vec<T>> for SingularVec<T> {
  fn as_ref(&self) -> &Vec<T> {
    &self.0
  }
}

impl<T> AsMut<Vec<T>> for SingularVec<T> {
  fn as_mut(&mut self) -> &mut Vec<T> {
    &mut self.0
  }
}

impl<T> SingularVec<T> {
  fn push(&mut self, element: T) {
    self.0.push(element);
  }
}

impl<T: Serialize> Serialize for SingularVec<T> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer {

        if self.0.len() == 1 {
          let first_element = self.0.get(0).unwrap();
          return first_element.serialize(serializer);
        }

        return self.0.serialize(serializer);

    }

}

impl<'de, T: Deserialize<'de>> Deserialize<'de> for SingularVec<T> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de> {
      let dtx: SingularVec_helper<T> = SingularVec_helper::deserialize(deserializer)?;
      match dtx {
        SingularVec_helper::Element(elem) => {
          Ok(SingularVec(vec![elem]))
        }
        SingularVec_helper::Vec(elem) => {
          Ok(SingularVec(elem))
        }
      }
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
enum SingularVec_helper<T> {
  Element(T),
  Vec(Vec<T>),
}
