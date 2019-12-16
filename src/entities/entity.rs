use serde::{Serialize, Deserialize, Serializer, Deserializer};
use crate::entities::object::ActivityStreamObject;
use crate::entities::activity::{ActivityStreamActivity, ActivityStreamActivityType};
use crate::entities::intransitiveactivity::ActivityStreamIntransitiveActivity;
use crate::entities::link::ActivityStreamLink;
use url::Url;

//// Enum containing any valid ActivityStream Entity.
#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum ActivityStreamEntity {
    ActivityStreamObject(ActivityStreamObject),
    ActivityStreamLink(ActivityStreamLink),
    ActivityStreamActivity(ActivityStreamActivity),
    ActivityStreamIntransitiveActivity(ActivityStreamIntransitiveActivity),
    Link(Url),
}

//// A Boxed version of an ActivityStreamEntity
pub type BoxedActivityStreamEntity = Box<ActivityStreamEntity>;

//// This enum describes the ActivityStream core types as defined in [Section 2 of the specification](https://www.w3.org/TR/activitystreams-vocabulary/#types)
#[derive(Debug, Clone, Copy, PartialEq)]
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
    ActivityType(ActivityStreamActivityType)
}

impl Serialize for ActivityStreamEntityType {

    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer {

        match self {
          ActivityStreamEntityType::ActivityType(activity_type) => activity_type.serialize(serializer),
          any => {
            serializer.serialize_str(&format!("{:?}", any))
          }
        }

    }

}

impl<'de> Deserialize<'de> for ActivityStreamEntityType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de> {

      deserializer.deserialize_string(Visitor::new());
      /*Ok(if let Ok(string_value) = String::deserialize(deserializer) {
        match string_value.as_str() {
          "Object" => ActivityStreamEntityType::Object,
          "Link" => ActivityStreamEntityType::Link,
          "LinkObject" => ActivityStreamEntityType::LinkObject,
          "Activity" => ActivityStreamEntityType::Activity,
          "IntransitiveActivity" => ActivityStreamEntityType::IntransitiveActivity,
          "Collection" => ActivityStreamEntityType::Collection,
          "OrderedCollection" => ActivityStreamEntityType::OrderedCollection,
          "CollectionPage" => ActivityStreamEntityType::CollectionPage,
          "OrderedCollectionPage" => ActivityStreamEntityType::OrderedCollectionPage,
          _ => unreachable!("Invalid variant in ActivityStreamEntityType enum !")
        }
      } else if let Ok(activity_type) = ActivityStreamActivityType::deserialize(deserializer) {
        ActivityStreamEntityType::ActivityType(activity_type)
      } else {
        unreachable!("Invalid type in ActivityStreamEntityType num !");
      })*/
    }
}

impl Default for ActivityStreamEntityType {
  fn default() -> Self {
    ActivityStreamEntityType::Object
  }
}
