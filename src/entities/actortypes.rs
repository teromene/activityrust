use crate::entities::activity::ActivityStreamActivity;
use crate::entities::collection::ActivityStreamCollection;
use crate::entities::entity::{
    ActivityStreamEntity, ActivityStreamEntityType, BoxedActivityStreamEntity,
};
use crate::entities::intransitiveactivity::ActivityStreamIntransitiveActivity;
use crate::entities::orderedcollection::ActivityStreamOrderedCollection;
use serde::{Deserialize, Deserializer, Serialize};

use crate::content::*;
use crate::traits::properties::*;
use crate::{MaybeOptional, OneOrMultiple};
use ambassador::Delegate;
use chrono::{DateTime, Utc};
use url::Url;

impl ActivityStreamActorProperties for ActivityStreamActor_ {
    fn get_inbox(&self) -> &ActivityStreamOrderedCollection {
        &self.inbox
    }

    fn set_inbox(&mut self, inbox: ActivityStreamOrderedCollection) {
        self.inbox = inbox;
    }

    fn get_outbox(&self) -> &ActivityStreamOrderedCollection {
        &self.outbox
    }

    fn set_outbox(&mut self, outbox: ActivityStreamOrderedCollection) {
        self.outbox = outbox;
    }

    fn get_following(&self) -> &Option<ActivityStreamCollection> {
        &self.following
    }

    fn set_following<T: MaybeOptional<ActivityStreamCollection>>(&mut self, following: T) {
        self.following = following.get_optional();
    }

    fn get_followers(&self) -> &Option<ActivityStreamCollection> {
        &self.followers
    }

    fn set_followers<T: MaybeOptional<ActivityStreamCollection>>(&mut self, followers: T) {
        self.followers = followers.get_optional();
    }

    fn get_liked(&self) -> &Option<ActivityStreamCollection> {
        &self.liked
    }

    fn set_liked<T: MaybeOptional<ActivityStreamCollection>>(&mut self, liked: T) {
        self.liked = liked.get_optional();
    }
}

//// Type for the Actor data
#[derive(Debug, Default, Serialize, Deserialize, PartialEq)]
struct ActivityStreamActor_ {
    #[serde(flatten)]
    inbox: ActivityStreamOrderedCollection,
    outbox: ActivityStreamOrderedCollection,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    following: Option<ActivityStreamCollection>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    followers: Option<ActivityStreamCollection>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    liked: Option<ActivityStreamCollection>,
}

generate_basics!(
    ActivityStreamApplication,
    ActivityStreamEntityType::Application
);

#[derive(Debug, Default, Delegate, Serialize, Deserialize, PartialEq)]
#[delegate(ActivityStreamActivityProperties, target = "_base")]
#[delegate(ActivityStreamObjectProperties, target = "_base")]
#[delegate(ActivityStreamActorProperties, target = "_actorbase")]
pub struct ActivityStreamApplication {
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[serde(deserialize_with = "ActivityStreamApplication::deserialize_type")]
    r#type: Option<ActivityStreamEntityType>,
    #[serde(flatten)]
    _base: ActivityStreamActivity,
    #[serde(flatten)]
    _actorbase: ActivityStreamActor_,
}

generate_basics!(ActivityStreamGroup, ActivityStreamEntityType::Group);

#[derive(Debug, Default, Delegate, Serialize, Deserialize, PartialEq)]
#[delegate(ActivityStreamActivityProperties, target = "_base")]
#[delegate(ActivityStreamObjectProperties, target = "_base")]
#[delegate(ActivityStreamActorProperties, target = "_actorbase")]
pub struct ActivityStreamGroup {
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[serde(deserialize_with = "ActivityStreamGroup::deserialize_type")]
    r#type: Option<ActivityStreamEntityType>,
    #[serde(flatten)]
    _base: ActivityStreamActivity,
    #[serde(flatten)]
    _actorbase: ActivityStreamActor_,
}

generate_basics!(
    ActivityStreamOrganization,
    ActivityStreamEntityType::Organization
);

#[derive(Debug, Default, Delegate, Serialize, Deserialize, PartialEq)]
#[delegate(ActivityStreamActivityProperties, target = "_base")]
#[delegate(ActivityStreamObjectProperties, target = "_base")]
#[delegate(ActivityStreamActorProperties, target = "_actorbase")]
pub struct ActivityStreamOrganization {
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[serde(deserialize_with = "ActivityStreamOrganization::deserialize_type")]
    r#type: Option<ActivityStreamEntityType>,
    #[serde(flatten)]
    _base: ActivityStreamActivity,
    #[serde(flatten)]
    _actorbase: ActivityStreamActor_,
}

generate_basics!(ActivityStreamPerson, ActivityStreamEntityType::Person);

#[derive(Debug, Default, Delegate, Serialize, Deserialize, PartialEq)]
#[delegate(ActivityStreamActivityProperties, target = "_base")]
#[delegate(ActivityStreamObjectProperties, target = "_base")]
#[delegate(ActivityStreamActorProperties, target = "_actorbase")]
pub struct ActivityStreamPerson {
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[serde(deserialize_with = "ActivityStreamPerson::deserialize_type")]
    r#type: Option<ActivityStreamEntityType>,
    #[serde(flatten)]
    _base: ActivityStreamActivity,
    #[serde(flatten)]
    _actorbase: ActivityStreamActor_,
}

generate_basics!(ActivityStreamService, ActivityStreamEntityType::Service);

#[derive(Debug, Default, Delegate, Serialize, Deserialize, PartialEq)]
#[delegate(ActivityStreamActivityProperties, target = "_base")]
#[delegate(ActivityStreamObjectProperties, target = "_base")]
#[delegate(ActivityStreamActorProperties, target = "_actorbase")]
pub struct ActivityStreamService {
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[serde(deserialize_with = "ActivityStreamService::deserialize_type")]
    r#type: Option<ActivityStreamEntityType>,
    #[serde(flatten)]
    _base: ActivityStreamActivity,
    #[serde(flatten)]
    _actorbase: ActivityStreamActor_,
}
