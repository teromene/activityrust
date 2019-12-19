use crate::content::*;
use crate::entities::entity::{
    ActivityStreamEntity, ActivityStreamEntityType, BoxedActivityStreamEntity,
};
use crate::entities::object::ActivityStreamObject;
use crate::traits::properties::*;
use crate::MaybeOptional;
use ambassador::Delegate;
use chrono::{DateTime, FixedOffset};
use serde::{Deserialize, Deserializer, Serialize};
use url::Url;

impl ActivityStreamCollectionProperties for ActivityStreamCollection {
    fn get_total_items(&self) -> &Option<usize> {
        &self.totalItems
    }

    fn get_current(&self) -> &Option<BoxedActivityStreamEntity> {
        &self.current
    }

    fn set_current<S, T: MaybeOptional<S>>(&mut self, current: T)
    where
        ActivityStreamEntity: From<S>,
    {
        if let Some(current) = current.get_optional() {
            self.current = Some(Box::new(ActivityStreamEntity::from(current)));
        }
    }

    fn get_first(&self) -> &Option<BoxedActivityStreamEntity> {
        &self.first
    }

    fn set_first<S, T: MaybeOptional<S>>(&mut self, first: T)
    where
        ActivityStreamEntity: From<S>,
    {
        if let Some(first) = first.get_optional() {
            self.first = Some(Box::new(ActivityStreamEntity::from(first)));
        }
    }

    fn get_last(&self) -> &Option<BoxedActivityStreamEntity> {
        &self.last
    }

    fn set_last<S, T: MaybeOptional<S>>(&mut self, last: T)
    where
        ActivityStreamEntity: From<S>,
    {
        if let Some(last) = last.get_optional() {
            self.last = Some(Box::new(ActivityStreamEntity::from(last)));
        }
    }

    fn get_items(&self) -> &Option<Vec<ActivityStreamEntity>> {
        &self.items
    }

    fn set_items<S, T: MaybeOptional<Vec<S>>>(&mut self, items: T)
    where
        ActivityStreamEntity: From<S>,
    {
        if let Some(items) = items.get_optional() {
            let items: Vec<ActivityStreamEntity> =
                items.into_iter().map(ActivityStreamEntity::from).collect();
            self.totalItems = Some(items.len());
            self.items = Some(items);
        }
    }

    fn add_item<S, T: MaybeOptional<S>>(&mut self, item: T)
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

generate_basics!(
    ActivityStreamCollection,
    ActivityStreamEntityType::Collection
);

#[derive(Debug, Default, Delegate, Serialize, Deserialize, PartialEq)]
#[delegate(ActivityStreamObjectProperties, target = "_base")]
pub struct ActivityStreamCollection {
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[serde(deserialize_with = "ActivityStreamCollection::deserialize_type")]
    r#type: Option<ActivityStreamEntityType>,
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
