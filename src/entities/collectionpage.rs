use serde::{Serialize, Deserialize};
use crate::entities::entity::{ActivityStreamEntityType, BoxedActivityStreamEntity, ActivityStreamEntity};
use crate::entities::object::ActivityStreamObject;
use crate::entities::collection::ActivityStreamCollection;
use crate::traits::properties::*;
use ambassador::Delegate;
use crate::{MaybeOptional, OneOrMultiple};
use url::Url;
use crate::content::*;
use chrono::{DateTime, Utc};

impl ActivityStreamEntityProperties for ActivityStreamCollectionPage {
    fn get_id(&self) -> &Option<Url> {
        &self.id
    }

    fn set_id<T: MaybeOptional<Url>>(&mut self, id: T) {
        self.id = id.get_optional();
    }

    fn get_type(&self) -> &ActivityStreamEntityType {
      &self.r#type
    }

    fn set_type(&mut self, r#type: ActivityStreamEntityType) {
      self.r#type = r#type;
    }

    fn register_context(&mut self, new_context: Url) {
        if let Some(ref mut context) = self.context {
            context.append(new_context);
        } else {
            self.context = Some(OneOrMultiple::Element(new_context));
        }
    }

}

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

impl ActivityStreamCollectionPage {

  pub fn create() -> Self {

    let object_context = Url::parse("https://www.w3.org/ns/activitystreams").unwrap();

    let mut new_object = ActivityStreamCollectionPage::default();
    new_object.register_context(object_context);
    new_object.set_type(ActivityStreamEntityType::CollectionPage);
    new_object
  }

}


#[allow(non_snake_case)]
#[derive(Debug, Default, Delegate, Serialize, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
#[delegate(ActivityStreamObjectProperties, target = "_base")]
#[delegate(ActivityStreamCollectionProperties, target = "_basecollection")]
pub struct ActivityStreamCollectionPage {
    #[serde(skip_serializing_if = "Option::is_none", default)]
    id: Option<Url>,
    r#type: ActivityStreamEntityType,
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
