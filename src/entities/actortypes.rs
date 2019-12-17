use serde::{Serialize, Deserialize};
use crate::entities::entity::{ActivityStreamEntityType, BoxedActivityStreamEntity, ActivityStreamEntity};
use crate::entities::collection::ActivityStreamCollection;
use crate::entities::orderedcollection::ActivityStreamOrderedCollection;
use crate::entities::activity::ActivityStreamActivity;
use crate::entities::intransitiveactivity::ActivityStreamIntransitiveActivity;

use crate::traits::properties::*;
use ambassador::Delegate;
use crate::{MaybeOptional, OneOrMultiple};
use url::Url;
use crate::content::*;
use chrono::{DateTime, Utc};

/*
        fn get_inbox(&self) -> &ActivityStreamOrderedCollection;
        fn set_inbox(&mut self, inbox: ActivityStreamOrderedCollection);
        fn get_outbox(&self) -> &ActivityStreamOrderedCollection;
        fn set_outbox(&mut self, outbox: ActivityStreamOrderedCollection);
        fn get_following(&self) -> &Option<ActivityStreamCollection>;
        fn set_following<S, T: MaybeOptional<S>>(&mut self, following: T) where ActivityStreamCollection: From<S>;
        fn get_followers(&self) -> &Option<ActivityStreamCollection>;
        fn set_followers<S, T: MaybeOptional<S>>(&mut self, followers: T) where ActivityStreamCollection: From<S>;
        fn get_liked(&self) -> &Option<ActivityStreamCollection>;
        fn set_liked<S, T: MaybeOptional<S>>(&mut self, liked: T) where ActivityStreamCollection: From<S>;
*/
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
#[allow(non_snake_case)]
#[derive(Debug, Default, Serialize, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
struct ActivityStreamActor_ {
    #[serde(flatten)]
    _base: ActivityStreamActivity,
    inbox: ActivityStreamOrderedCollection,
    outbox: ActivityStreamOrderedCollection,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    following: Option<ActivityStreamCollection>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    followers: Option<ActivityStreamCollection>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    liked: Option<ActivityStreamCollection>,
}
