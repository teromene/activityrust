use serde::{Serialize, Deserialize, Deserializer};
use crate::entities::entity::{ActivityStreamEntityType, BoxedActivityStreamEntity, ActivityStreamEntity};
use crate::entities::object::ActivityStreamObject;
use crate::entities::collection::ActivityStreamCollection;
use crate::traits::properties::*;
use ambassador::Delegate;
use crate::{MaybeOptional, OneOrMultiple};
use url::Url;
use crate::content::*;
use chrono::{DateTime, Utc};

impl ActivityStreamCollectionPageProperties for ActivityStreamCollectionPage {
  fn get_part_of(&self) -> &Option<ActivityStreamLinkableCollection> {
    &self.partOf
  }

  fn set_part_of<S, T: MaybeOptional<S>>(&mut self, part_of: T) where ActivityStreamLinkableCollection: From<S> {
    if let Some(part_of) = part_of.get_optional() {
      self.partOf = Some(ActivityStreamLinkableCollection::from(part_of));
    }
  }

  fn get_next(&self) -> &Option<Box<ActivityStreamLinkableCollectionPage>> {
    &self.next
  }

  fn set_next<S, T: MaybeOptional<S>>(&mut self, next: T) where ActivityStreamLinkableCollectionPage: From<S> {
    if let Some(next) = next.get_optional() {
      self.next = Some(Box::new(ActivityStreamLinkableCollectionPage::from(next)));
    }
  }

  fn get_prev(&self) -> &Option<Box<ActivityStreamLinkableCollectionPage>> {
    &self.prev
  }

  fn set_prev<S, T: MaybeOptional<S>>(&mut self, prev: T) where ActivityStreamLinkableCollectionPage: From<S> {
    if let Some(prev) = prev.get_optional() {
      self.prev = Some(Box::new(ActivityStreamLinkableCollectionPage::from(prev)));
    }
  }

}

generate_basics!(ActivityStreamCollectionPage, ActivityStreamEntityType::CollectionPage);

#[derive(Debug, Default, Delegate, Serialize, Deserialize, PartialEq)]
#[delegate(ActivityStreamObjectProperties, target = "_base")]
#[delegate(ActivityStreamCollectionProperties, target = "_basecollection")]
pub struct ActivityStreamCollectionPage {
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[serde(deserialize_with = "ActivityStreamCollectionPage::deserialize_type")]
    r#type: Option<ActivityStreamEntityType>,
    #[serde(rename = "@context")]
    context: Option<OneOrMultiple<Url>>,
    #[serde(flatten)]
    _base: ActivityStreamObject,
    #[serde(flatten)]
    _basecollection: ActivityStreamCollection,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    partOf: Option<ActivityStreamLinkableCollection>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    next: Option<Box<ActivityStreamLinkableCollectionPage>>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    prev: Option<Box<ActivityStreamLinkableCollectionPage>>,
}
