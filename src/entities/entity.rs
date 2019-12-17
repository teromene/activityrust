use serde::{Serialize, Deserialize};
use crate::entities::object::ActivityStreamObject;
use crate::entities::activity::ActivityStreamActivity;
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
#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq)]
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
    //Types for Activity
    Accept,
    Add,
    Announce,
    Arrive,
    Block,
    Create,
    Delete,
    Dislike,
    Flag,
    Follow,
    Ignore,
    Invite,
    Join,
    Leave,
    Like,
    Listen,
    Move,
    Offer,
    Question,
    Reject,
    Read,
    Remove,
    TentativeReject,
    TentativeAccept,
    Travel,
    Undo,
    Update,
    View,
    //Types for Actor
    Application,
    Group,
    Organization,
    Person,
    Service,
    //Types for Object
    Article,
    Audio,
    Document,
    Event,
    Image,
    Note,
    Page,
    Place,
    Profile,
    Relationship,
    Tombstone,
    Video,
    //Types for Link
    Mention,
}


impl Default for ActivityStreamEntityType {
  fn default() -> Self {
    ActivityStreamEntityType::Object
  }
}
