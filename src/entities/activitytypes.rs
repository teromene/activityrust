use crate::content::*;
use crate::entities::activity::ActivityStreamActivity;
use crate::entities::collection::ActivityStreamCollection;
use crate::entities::entity::{
    ActivityStreamEntity, ActivityStreamEntityType, BoxedActivityStreamEntity,
};
use crate::entities::intransitiveactivity::ActivityStreamIntransitiveActivity;
use crate::traits::properties::*;
use crate::{MaybeOptional, OneOrMultiple};
use ambassador::Delegate;
use chrono::{DateTime, Utc};
#[allow(non_snake_case)]
use serde::{Deserialize, Deserializer, Serialize};
use url::Url;

generate_basics!(ActivityStreamAccept, ActivityStreamEntityType::Accept);

#[derive(Debug, Default, Delegate, Serialize, Deserialize, PartialEq)]
#[delegate(ActivityStreamActivityProperties, target = "_base")]
#[delegate(ActivityStreamObjectProperties, target = "_base")]
pub struct ActivityStreamAccept {
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[serde(deserialize_with = "ActivityStreamAccept::deserialize_type")]
    r#type: Option<ActivityStreamEntityType>,
    #[serde(flatten)]
    _base: ActivityStreamActivity,
}

generate_basics!(
    ActivityStreamTentativeAccept,
    ActivityStreamEntityType::TentativeAccept
);

#[derive(Debug, Default, Delegate, Serialize, Deserialize, PartialEq)]
#[delegate(ActivityStreamActivityProperties, target = "_base")]
#[delegate(ActivityStreamObjectProperties, target = "_base")]
pub struct ActivityStreamTentativeAccept {
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[serde(deserialize_with = "ActivityStreamTentativeAccept::deserialize_type")]
    r#type: Option<ActivityStreamEntityType>,
    #[serde(flatten)]
    _base: ActivityStreamAccept,
}

generate_basics!(ActivityStreamAdd, ActivityStreamEntityType::Add);

#[derive(Debug, Default, Delegate, Serialize, Deserialize, PartialEq)]
#[delegate(ActivityStreamActivityProperties, target = "_base")]
#[delegate(ActivityStreamObjectProperties, target = "_base")]
pub struct ActivityStreamAdd {
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[serde(deserialize_with = "ActivityStreamAdd::deserialize_type")]
    r#type: Option<ActivityStreamEntityType>,
    #[serde(flatten)]
    _base: ActivityStreamActivity,
}

generate_basics!(ActivityStreamArrive, ActivityStreamEntityType::Arrive);

#[derive(Debug, Default, Delegate, Serialize, Deserialize, PartialEq)]
#[delegate(ActivityStreamIntransitiveActivityProperties, target = "_base")]
#[delegate(ActivityStreamObjectProperties, target = "_base")]
pub struct ActivityStreamArrive {
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[serde(deserialize_with = "ActivityStreamArrive::deserialize_type")]
    r#type: Option<ActivityStreamEntityType>,
    #[serde(flatten)]
    _base: ActivityStreamIntransitiveActivity,
}

generate_basics!(ActivityStreamCreate, ActivityStreamEntityType::Create);

#[derive(Debug, Default, Delegate, Serialize, Deserialize, PartialEq)]
#[delegate(ActivityStreamActivityProperties, target = "_base")]
#[delegate(ActivityStreamObjectProperties, target = "_base")]
pub struct ActivityStreamCreate {
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[serde(deserialize_with = "ActivityStreamCreate::deserialize_type")]
    r#type: Option<ActivityStreamEntityType>,
    #[serde(flatten)]
    _base: ActivityStreamActivity,
}

generate_basics!(ActivityStreamDelete, ActivityStreamEntityType::Delete);

#[derive(Debug, Default, Delegate, Serialize, Deserialize, PartialEq)]
#[delegate(ActivityStreamActivityProperties, target = "_base")]
#[delegate(ActivityStreamObjectProperties, target = "_base")]
pub struct ActivityStreamDelete {
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[serde(deserialize_with = "ActivityStreamDelete::deserialize_type")]
    r#type: Option<ActivityStreamEntityType>,
    #[serde(flatten)]
    _base: ActivityStreamActivity,
}

generate_basics!(ActivityStreamFollow, ActivityStreamEntityType::Follow);

#[derive(Debug, Default, Delegate, Serialize, Deserialize, PartialEq)]
#[delegate(ActivityStreamActivityProperties, target = "_base")]
#[delegate(ActivityStreamObjectProperties, target = "_base")]
pub struct ActivityStreamFollow {
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[serde(deserialize_with = "ActivityStreamFollow::deserialize_type")]
    r#type: Option<ActivityStreamEntityType>,
    #[serde(flatten)]
    _base: ActivityStreamActivity,
}

generate_basics!(ActivityStreamIgnore, ActivityStreamEntityType::Ignore);

#[allow(non_snake_case)]
#[derive(Debug, Default, Delegate, Serialize, Deserialize, PartialEq)]
#[delegate(ActivityStreamActivityProperties, target = "_base")]
#[delegate(ActivityStreamObjectProperties, target = "_base")]
pub struct ActivityStreamIgnore {
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[serde(deserialize_with = "ActivityStreamIgnore::deserialize_type")]
    r#type: Option<ActivityStreamEntityType>,
    #[serde(flatten)]
    _base: ActivityStreamActivity,
}

generate_basics!(ActivityStreamJoin, ActivityStreamEntityType::Join);

#[derive(Debug, Default, Delegate, Serialize, Deserialize, PartialEq)]
#[delegate(ActivityStreamActivityProperties, target = "_base")]
#[delegate(ActivityStreamObjectProperties, target = "_base")]
pub struct ActivityStreamJoin {
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[serde(deserialize_with = "ActivityStreamJoin::deserialize_type")]
    r#type: Option<ActivityStreamEntityType>,
    #[serde(flatten)]
    _base: ActivityStreamActivity,
}

generate_basics!(ActivityStreamLeave, ActivityStreamEntityType::Leave);

#[derive(Debug, Default, Delegate, Serialize, Deserialize, PartialEq)]
#[delegate(ActivityStreamActivityProperties, target = "_base")]
#[delegate(ActivityStreamObjectProperties, target = "_base")]
pub struct ActivityStreamLeave {
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[serde(deserialize_with = "ActivityStreamLeave::deserialize_type")]
    r#type: Option<ActivityStreamEntityType>,
    #[serde(flatten)]
    _base: ActivityStreamActivity,
}

generate_basics!(ActivityStreamLike, ActivityStreamEntityType::Like);

#[derive(Debug, Default, Delegate, Serialize, Deserialize, PartialEq)]
#[delegate(ActivityStreamActivityProperties, target = "_base")]
#[delegate(ActivityStreamObjectProperties, target = "_base")]
pub struct ActivityStreamLike {
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[serde(deserialize_with = "ActivityStreamLike::deserialize_type")]
    r#type: Option<ActivityStreamEntityType>,
    #[serde(flatten)]
    _base: ActivityStreamActivity,
}

generate_basics!(ActivityStreamOffer, ActivityStreamEntityType::Offer);

#[derive(Debug, Default, Delegate, Serialize, Deserialize, PartialEq)]
#[delegate(ActivityStreamActivityProperties, target = "_base")]
#[delegate(ActivityStreamObjectProperties, target = "_base")]
pub struct ActivityStreamOffer {
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[serde(deserialize_with = "ActivityStreamOffer::deserialize_type")]
    r#type: Option<ActivityStreamEntityType>,
    #[serde(flatten)]
    _base: ActivityStreamActivity,
}

generate_basics!(ActivityStreamInvite, ActivityStreamEntityType::Invite);

#[derive(Debug, Default, Delegate, Serialize, Deserialize, PartialEq)]
#[delegate(ActivityStreamActivityProperties, target = "_base")]
#[delegate(ActivityStreamObjectProperties, target = "_base")]
pub struct ActivityStreamInvite {
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[serde(deserialize_with = "ActivityStreamInvite::deserialize_type")]
    r#type: Option<ActivityStreamEntityType>,
    #[serde(flatten)]
    _base: ActivityStreamOffer,
}

generate_basics!(ActivityStreamReject, ActivityStreamEntityType::Reject);

#[derive(Debug, Default, Delegate, Serialize, Deserialize, PartialEq)]
#[delegate(ActivityStreamActivityProperties, target = "_base")]
#[delegate(ActivityStreamObjectProperties, target = "_base")]
pub struct ActivityStreamReject {
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[serde(deserialize_with = "ActivityStreamReject::deserialize_type")]
    r#type: Option<ActivityStreamEntityType>,
    #[serde(flatten)]
    _base: ActivityStreamActivity,
}

generate_basics!(
    ActivityStreamTentativeReject,
    ActivityStreamEntityType::TentativeReject
);

#[derive(Debug, Default, Delegate, Serialize, Deserialize, PartialEq)]
#[delegate(ActivityStreamActivityProperties, target = "_base")]
#[delegate(ActivityStreamObjectProperties, target = "_base")]
pub struct ActivityStreamTentativeReject {
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[serde(deserialize_with = "ActivityStreamTentativeReject::deserialize_type")]
    r#type: Option<ActivityStreamEntityType>,
    #[serde(flatten)]
    _base: ActivityStreamReject,
}

generate_basics!(ActivityStreamRemove, ActivityStreamEntityType::Remove);

#[derive(Debug, Default, Delegate, Serialize, Deserialize, PartialEq)]
#[delegate(ActivityStreamActivityProperties, target = "_base")]
#[delegate(ActivityStreamObjectProperties, target = "_base")]
pub struct ActivityStreamRemove {
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[serde(deserialize_with = "ActivityStreamRemove::deserialize_type")]
    r#type: Option<ActivityStreamEntityType>,
    #[serde(flatten)]
    _base: ActivityStreamActivity,
}

generate_basics!(ActivityStreamUndo, ActivityStreamEntityType::Undo);

#[derive(Debug, Default, Delegate, Serialize, Deserialize, PartialEq)]
#[delegate(ActivityStreamActivityProperties, target = "_base")]
#[delegate(ActivityStreamObjectProperties, target = "_base")]
pub struct ActivityStreamUndo {
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[serde(deserialize_with = "ActivityStreamUndo::deserialize_type")]
    r#type: Option<ActivityStreamEntityType>,
    #[serde(flatten)]
    _base: ActivityStreamActivity,
}

generate_basics!(ActivityStreamUpdate, ActivityStreamEntityType::Update);

#[derive(Debug, Default, Delegate, Serialize, Deserialize, PartialEq)]
#[delegate(ActivityStreamActivityProperties, target = "_base")]
#[delegate(ActivityStreamObjectProperties, target = "_base")]
pub struct ActivityStreamUpdate {
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[serde(deserialize_with = "ActivityStreamUpdate::deserialize_type")]
    r#type: Option<ActivityStreamEntityType>,
    #[serde(flatten)]
    _base: ActivityStreamActivity,
}

generate_basics!(ActivityStreamView, ActivityStreamEntityType::View);

#[derive(Debug, Default, Delegate, Serialize, Deserialize, PartialEq)]
#[delegate(ActivityStreamActivityProperties, target = "_base")]
#[delegate(ActivityStreamObjectProperties, target = "_base")]
pub struct ActivityStreamView {
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[serde(deserialize_with = "ActivityStreamView::deserialize_type")]
    r#type: Option<ActivityStreamEntityType>,
    #[serde(flatten)]
    _base: ActivityStreamActivity,
}

generate_basics!(ActivityStreamListen, ActivityStreamEntityType::Listen);

#[derive(Debug, Default, Delegate, Serialize, Deserialize, PartialEq)]
#[delegate(ActivityStreamActivityProperties, target = "_base")]
#[delegate(ActivityStreamObjectProperties, target = "_base")]
pub struct ActivityStreamListen {
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[serde(deserialize_with = "ActivityStreamListen::deserialize_type")]
    r#type: Option<ActivityStreamEntityType>,
    #[serde(flatten)]
    _base: ActivityStreamActivity,
}

generate_basics!(ActivityStreamRead, ActivityStreamEntityType::Read);

#[derive(Debug, Default, Delegate, Serialize, Deserialize, PartialEq)]
#[delegate(ActivityStreamActivityProperties, target = "_base")]
#[delegate(ActivityStreamObjectProperties, target = "_base")]
pub struct ActivityStreamRead {
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[serde(deserialize_with = "ActivityStreamRead::deserialize_type")]
    r#type: Option<ActivityStreamEntityType>,
    #[serde(flatten)]
    _base: ActivityStreamActivity,
}

generate_basics!(ActivityStreamMove, ActivityStreamEntityType::Move);

#[derive(Debug, Default, Delegate, Serialize, Deserialize, PartialEq)]
#[delegate(ActivityStreamActivityProperties, target = "_base")]
#[delegate(ActivityStreamObjectProperties, target = "_base")]
pub struct ActivityStreamMove {
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[serde(deserialize_with = "ActivityStreamMove::deserialize_type")]
    r#type: Option<ActivityStreamEntityType>,
    #[serde(flatten)]
    _base: ActivityStreamActivity,
}

generate_basics!(ActivityStreamTravel, ActivityStreamEntityType::Travel);

#[derive(Debug, Default, Delegate, Serialize, Deserialize, PartialEq)]
#[delegate(ActivityStreamIntransitiveActivityProperties, target = "_base")]
#[delegate(ActivityStreamObjectProperties, target = "_base")]
pub struct ActivityStreamTravel {
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[serde(deserialize_with = "ActivityStreamTravel::deserialize_type")]
    r#type: Option<ActivityStreamEntityType>,
    #[serde(flatten)]
    _base: ActivityStreamIntransitiveActivity,
}

generate_basics!(ActivityStreamAnnounce, ActivityStreamEntityType::Announce);

#[derive(Debug, Default, Delegate, Serialize, Deserialize, PartialEq)]
#[delegate(ActivityStreamActivityProperties, target = "_base")]
#[delegate(ActivityStreamObjectProperties, target = "_base")]
pub struct ActivityStreamAnnounce {
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[serde(deserialize_with = "ActivityStreamAnnounce::deserialize_type")]
    r#type: Option<ActivityStreamEntityType>,
    #[serde(flatten)]
    _base: ActivityStreamActivity,
}

generate_basics!(ActivityStreamBlock, ActivityStreamEntityType::Block);

#[derive(Debug, Default, Delegate, Serialize, Deserialize, PartialEq)]
#[delegate(ActivityStreamActivityProperties, target = "_base")]
#[delegate(ActivityStreamObjectProperties, target = "_base")]
pub struct ActivityStreamBlock {
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[serde(deserialize_with = "ActivityStreamBlock::deserialize_type")]
    r#type: Option<ActivityStreamEntityType>,
    #[serde(flatten)]
    _base: ActivityStreamIgnore,
}

generate_basics!(ActivityStreamFlag, ActivityStreamEntityType::Flag);

#[derive(Debug, Default, Delegate, Serialize, Deserialize, PartialEq)]
#[delegate(ActivityStreamActivityProperties, target = "_base")]
#[delegate(ActivityStreamObjectProperties, target = "_base")]
pub struct ActivityStreamFlag {
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[serde(deserialize_with = "ActivityStreamFlag::deserialize_type")]
    r#type: Option<ActivityStreamEntityType>,
    #[serde(flatten)]
    _base: ActivityStreamActivity,
}

generate_basics!(ActivityStreamDislike, ActivityStreamEntityType::Dislike);

#[derive(Debug, Default, Delegate, Serialize, Deserialize, PartialEq)]
#[delegate(ActivityStreamActivityProperties, target = "_base")]
#[delegate(ActivityStreamObjectProperties, target = "_base")]
pub struct ActivityStreamDislike {
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[serde(deserialize_with = "ActivityStreamDislike::deserialize_type")]
    r#type: Option<ActivityStreamEntityType>,
    #[serde(flatten)]
    _base: ActivityStreamActivity,
}

generate_basics!(ActivityStreamQuestion, ActivityStreamEntityType::Question);

impl ActivityStreamQuestionProperties for ActivityStreamQuestion {
    fn get_one_of(&self) -> &Option<Vec<ActivityStreamEntity>> {
        &self.oneOf
    }

    fn set_one_of<S, T: MaybeOptional<Vec<S>>>(&mut self, one_of: T)
    where
        ActivityStreamEntity: From<S>,
    {
        if let Some(one_of) = one_of.get_optional() {
            let one_of: Vec<ActivityStreamEntity> =
                one_of.into_iter().map(ActivityStreamEntity::from).collect();
            self.oneOf = Some(one_of);
        }
    }

    fn add_one_of<S, T: MaybeOptional<S>>(&mut self, one_of: T)
    where
        ActivityStreamEntity: From<S>,
    {
        if let Some(one_of) = one_of.get_optional() {
            if self.oneOf.is_none() {
                self.oneOf = Some(Vec::new());
            }

            if let Some(ref mut one_of_internal) = self.oneOf {
                one_of_internal.push(ActivityStreamEntity::from(one_of));
            }
        }
    }

    fn get_any_of(&self) -> &Option<Vec<ActivityStreamEntity>> {
        &self.anyOf
    }

    fn set_any_of<S, T: MaybeOptional<Vec<S>>>(&mut self, any_of: T)
    where
        ActivityStreamEntity: From<S>,
    {
        if let Some(any_of) = any_of.get_optional() {
            let any_of: Vec<ActivityStreamEntity> =
                any_of.into_iter().map(ActivityStreamEntity::from).collect();
            self.anyOf = Some(any_of);
        }
    }

    fn add_any_of<S, T: MaybeOptional<S>>(&mut self, any_of: T)
    where
        ActivityStreamEntity: From<S>,
    {
        if let Some(any_of) = any_of.get_optional() {
            if self.anyOf.is_none() {
                self.anyOf = Some(Vec::new());
            }

            if let Some(ref mut any_of_internal) = self.anyOf {
                any_of_internal.push(ActivityStreamEntity::from(any_of));
            }
        }
    }

    fn get_closed(&self) -> &Option<Box<ActivityStreamQuestionClosed>> {
        &self.closed
    }

    fn set_closed<S, T: MaybeOptional<S>>(&mut self, closed: T)
    where
        ActivityStreamQuestionClosed: From<S>,
    {
        if let Some(closed) = closed.get_optional() {
            self.closed = Some(Box::new(ActivityStreamQuestionClosed::from(closed)));
        }
    }
}

#[derive(Debug, Default, Delegate, Serialize, Deserialize, PartialEq)]
#[delegate(ActivityStreamIntransitiveActivityProperties, target = "_base")]
#[delegate(ActivityStreamObjectProperties, target = "_base")]
pub struct ActivityStreamQuestion {
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[serde(deserialize_with = "ActivityStreamQuestion::deserialize_type")]
    r#type: Option<ActivityStreamEntityType>,
    #[serde(flatten)]
    _base: ActivityStreamIntransitiveActivity,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    oneOf: Option<Vec<ActivityStreamEntity>>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    anyOf: Option<Vec<ActivityStreamEntity>>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    closed: Option<Box<ActivityStreamQuestionClosed>>,
}
