use serde::{Serialize, Deserialize};
use crate::entities::object::ActivityStreamObject;
use crate::entities::link::ActivityStreamLink;
use crate::MaybeOptional;
use url::Url;

//// Enum containing any valid ActivityStream Entity.
#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum ActivityStreamEntity {
    ActivityStreamObject(ActivityStreamObject),
    ActivityStreamLink(ActivityStreamLink),
    Link(Url),
}

//// A Boxed version of an ActivityStreamEntity
pub type BoxedActivityStreamEntity = Box<ActivityStreamEntity>;


//// This enum describes the ActivityStream core types as defined in [Section 2 of the specification](https://www.w3.org/TR/activitystreams-vocabulary/#types)
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub enum ActivityStreamEntityType {
    Object,
    Link,
    LinkObject,
    Activity,
    IntransitiveActivity,
    Collection,
    OrderedCollection,
    CollectionPage,
    OrderedCollectionPage,
}

impl Default for ActivityStreamEntityType {
  fn default() -> Self {
    ActivityStreamEntityType::Object
  }
}
