use serde::{Serialize, Deserialize};
use crate::entities::entity::{ActivityStreamEntityType, BoxedActivityStreamEntity, ActivityStreamEntity};
use crate::entities::object::ActivityStreamObject;
use crate::traits::properties::*;
use ambassador::Delegate;
use crate::{MaybeOptional, OneOrMultiple};
use url::Url;
use crate::content::*;
use chrono::{DateTime, Utc};

impl ActivityStreamEntityProperties for ActivityStreamCollection {
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


impl ActivityStreamCollectionProperties for ActivityStreamCollection {

  fn get_total_items(&self) -> &Option<usize> {
    &self.totalItems
  }

  fn get_current(&self) -> &Option<BoxedActivityStreamEntity> {
    &self.current
  }

  fn set_current<S: ActivityStreamEntityProperties, T: MaybeOptional<S>>(&mut self, current: T) where ActivityStreamEntity: From<S> {
    if let Some(current) = current.get_optional() {
      self.current = Some(Box::new(ActivityStreamEntity::from(current)));
    }
  }

  fn get_first(&self) -> &Option<BoxedActivityStreamEntity> {
    &self.first
  }

  fn set_first<S: ActivityStreamEntityProperties, T: MaybeOptional<S>>(&mut self, first: T) where ActivityStreamEntity: From<S> {
    if let Some(first) = first.get_optional() {
      self.first = Some(Box::new(ActivityStreamEntity::from(first)));
    }
  }

  fn get_last(&self) -> &Option<BoxedActivityStreamEntity> {
    &self.last
  }

  fn set_last<S: ActivityStreamEntityProperties, T: MaybeOptional<S>>(&mut self, last: T) where ActivityStreamEntity: From<S> {
    if let Some(last) = last.get_optional() {
      self.last = Some(Box::new(ActivityStreamEntity::from(last)));
    }
  }

  fn get_items(&self) -> &Option<Vec<ActivityStreamEntity>> {
      &self.items
  }

  fn set_items<S: ActivityStreamEntityProperties, T: MaybeOptional<Vec<S>>>(
      &mut self,
      items: T,
  )
  where
      ActivityStreamEntity: From<S>,
  {
      if let Some(items) = items.get_optional() {
          let items: Vec<ActivityStreamEntity> = items
                  .into_iter()
                  .map(ActivityStreamEntity::from)
                  .collect();
          self.totalItems = Some(items.len());
          self.items = Some(items);
      }
  }

    fn add_item<S: ActivityStreamEntityProperties, T: MaybeOptional<S>>(
        &mut self,
        item: T,
    )
    where
        ActivityStreamEntity: From<S>,
    {
        if let Some(item) = item.get_optional() {
            if self.items.is_none() {
                self.items = Some(Vec::new());
            }

            if let Some(ref mut items_internal) = self.items {
                self.totalItems = Some(self.totalItems.unwrap_or(0) + 1);
                items_internal.push(ActivityStreamEntity::from(item));
            }
        }
    }

}

impl ActivityStreamCollection {

  pub fn create() -> Self {

    let object_context = Url::parse("https://www.w3.org/ns/activitystreams").unwrap();

    let mut new_object = ActivityStreamCollection::default();
    new_object.register_context(object_context);
    new_object.set_type(ActivityStreamEntityType::Collection);
    new_object
  }

}


#[allow(non_snake_case)]
#[derive(Debug, Default, Delegate, Serialize, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
#[delegate(ActivityStreamObjectProperties, target = "_base")]
pub struct ActivityStreamCollection {
    #[serde(skip_serializing_if = "Option::is_none", default)]
    id: Option<Url>,
    r#type: ActivityStreamEntityType,
    #[serde(rename = "@context")]
    context: Option<OneOrMultiple<Url>>,
    #[serde(flatten)]
    _base: ActivityStreamObject,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    totalItems: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    current: Option<BoxedActivityStreamEntity>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    first: Option<BoxedActivityStreamEntity>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    last: Option<BoxedActivityStreamEntity>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    origin: Option<BoxedActivityStreamEntity>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    items: Option<Vec<ActivityStreamEntity>>,
}
