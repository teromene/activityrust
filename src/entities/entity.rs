use serde::{Serialize, Deserialize};
use crate::entities::object::ActivityStreamObject;
use crate::MaybeOptional;
use url::Url;

//// Enum containing any valid ActivityStream Entity.
#[derive(Debug, Serialize, Deserialize)]
pub enum ActivityStreamEntity {
    ActivityStreamObject(ActivityStreamObject),
    Link(Url),
}

//// A Boxed version of an ActivityStreamEntity
pub type BoxedActivityStreamEntity = Box<ActivityStreamEntity>;


//// This enum describes the ActivityStream core types as defined in [Section 2 of the specification](https://www.w3.org/TR/activitystreams-vocabulary/#types)
#[derive(Debug, Serialize, Deserialize)]
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

//// This trait allows access to all the basic elements of a core type
pub trait ActivityStreamEntityProperties {
    fn get_id(&self) -> &Option<Url>;
    fn set_id<T: MaybeOptional<Url>>(&mut self, id: T);
    fn get_type(&self) -> &ActivityStreamEntityType;
    fn set_type(&mut self, r#type: ActivityStreamEntityType);
}
