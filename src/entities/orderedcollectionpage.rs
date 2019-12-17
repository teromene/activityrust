use serde::{Serialize, Deserialize, Deserializer};
use crate::entities::entity::{ActivityStreamEntityType, BoxedActivityStreamEntity, ActivityStreamEntity};
use crate::entities::object::ActivityStreamObject;
use crate::entities::collection::ActivityStreamCollection;
use crate::entities::collectionpage::ActivityStreamCollectionPage;
use crate::traits::properties::*;
use ambassador::Delegate;
use crate::{MaybeOptional, OneOrMultiple};
use url::Url;
use crate::content::*;
use chrono::{DateTime, Utc};


impl ActivityStreamOrderedCollectionPageProperties for ActivityStreamOrderedCollectionPage {

  fn get_start_index(&self) -> &Option<usize> {
    &self.startIndex
  }

  fn set_start_index<T: MaybeOptional<usize>>(&mut self, start_index: T) {
    self.startIndex = start_index.get_optional();
  }

}

generate_basics!(ActivityStreamOrderedCollectionPage, ActivityStreamEntityType::OrderedCollectionPage);

#[derive(Debug, Default, Delegate, Serialize, Deserialize, PartialEq)]
#[delegate(ActivityStreamObjectProperties, target = "_base")]
#[delegate(ActivityStreamCollectionPageProperties, target = "_base")]
#[delegate(ActivityStreamCollectionProperties, target = "_base")]
pub struct ActivityStreamOrderedCollectionPage {
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[serde(deserialize_with = "ActivityStreamOrderedCollectionPage::deserialize_type")]
    r#type: Option<ActivityStreamEntityType>,
    #[serde(flatten)]
    _base: ActivityStreamCollectionPage,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    startIndex: Option<usize>,
}
