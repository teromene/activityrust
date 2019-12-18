use crate::entities::activity::ActivityStreamActivity;
use crate::entities::collection::ActivityStreamCollection;
use crate::entities::orderedcollection::ActivityStreamOrderedCollection;
use crate::entities::entity::{
    ActivityStreamEntity, ActivityStreamEntityType, BoxedActivityStreamEntity,
};
use serde::{Deserialize, Serialize};

use crate::content::*;
use crate::traits::properties::*;
use crate::{MaybeOptional};
use ambassador::Delegate;
use chrono::{DateTime, Utc};
use url::Url;

impl ActivityStreamActorProperties for ActivityStreamActor_ {
    fn get_inbox(&self) -> &Option<ActivityStreamLinkableOrderedCollection> {
        &self.inbox
    }

    fn set_inbox<S, T: MaybeOptional<S>>(&mut self, inbox: T) where ActivityStreamLinkableOrderedCollection: From<S> {
      if let Some(inbox) = inbox.get_optional() {
        self.inbox = Some(ActivityStreamLinkableOrderedCollection::from(inbox));
      }
    }

    fn get_outbox(&self) -> &Option<ActivityStreamLinkableOrderedCollection> {
        &self.outbox
    }

    fn set_outbox<S, T: MaybeOptional<S>>(&mut self, outbox: T) where ActivityStreamLinkableOrderedCollection: From<S> {
      if let Some(outbox) = outbox.get_optional() {
        self.outbox = Some(ActivityStreamLinkableOrderedCollection::from(outbox));
      }
    }

    fn get_following(&self) -> &Option<ActivityStreamLinkableCollection> {
        &self.following
    }

    fn set_following<S, T: MaybeOptional<S>>(&mut self, following: T) where ActivityStreamLinkableCollection: From<S> {
        if let Some(following) = following.get_optional() {
          self.following = Some(ActivityStreamLinkableCollection::from(following));
        }
    }

    fn get_followers(&self) -> &Option<ActivityStreamLinkableCollection> {
        &self.followers
    }

    fn set_followers<S, T: MaybeOptional<S>>(&mut self, followers: T) where ActivityStreamLinkableCollection: From<S> {
        if let Some(followers) = followers.get_optional() {
          self.followers = Some(ActivityStreamLinkableCollection::from(followers));
        }
    }

    fn get_liked(&self) -> &Option<ActivityStreamLinkableCollection> {
        &self.liked
    }

    fn set_liked<S, T: MaybeOptional<S>>(&mut self, liked: T) where ActivityStreamLinkableCollection: From<S> {
        if let Some(liked) = liked.get_optional() {
          self.liked = Some(ActivityStreamLinkableCollection::from(liked));
        }
    }
}

//// Type for the Actor data
#[derive(Debug, Default, Serialize, Deserialize, PartialEq)]
struct ActivityStreamActor_ {
    #[serde(flatten)]
    inbox: Option<ActivityStreamLinkableOrderedCollection>,
    outbox: Option<ActivityStreamLinkableOrderedCollection>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    following: Option<ActivityStreamLinkableCollection>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    followers: Option<ActivityStreamLinkableCollection>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    liked: Option<ActivityStreamLinkableCollection>,
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
