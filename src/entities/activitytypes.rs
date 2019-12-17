use serde::{Serialize, Deserialize};
use crate::entities::entity::{ActivityStreamEntityType, BoxedActivityStreamEntity, ActivityStreamEntity};
use crate::entities::collection::ActivityStreamCollection;
use crate::entities::activity::ActivityStreamActivity;
use crate::entities::intransitiveactivity::ActivityStreamIntransitiveActivity;

use crate::traits::properties::*;
use ambassador::Delegate;
use crate::{MaybeOptional, OneOrMultiple};
use url::Url;
use crate::content::*;
use chrono::{DateTime, Utc};

impl ActivityStreamAccept {
    pub fn create() -> Self {
        let mut activity = ActivityStreamAccept::default();
        activity.set_type(ActivityStreamEntityType::Accept);
        activity
    }
}

#[allow(non_snake_case)]
#[derive(Debug, Default, Delegate, Serialize, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
#[delegate(ActivityStreamActivityProperties, target = "_base")]
#[delegate(ActivityStreamObjectProperties, target = "_base")]
#[delegate(ActivityStreamEntityProperties, target = "_base")]
pub struct ActivityStreamAccept {
    #[serde(flatten)]
    _base: ActivityStreamActivity
}

impl ActivityStreamTentativeAccept {
    pub fn create() -> Self {
        let mut activity = ActivityStreamTentativeAccept::default();
        activity.set_type(ActivityStreamEntityType::TentativeAccept);
        activity
    }
}

#[allow(non_snake_case)]
#[derive(Debug, Default, Delegate, Serialize, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
#[delegate(ActivityStreamActivityProperties, target = "_base")]
#[delegate(ActivityStreamObjectProperties, target = "_base")]
#[delegate(ActivityStreamEntityProperties, target = "_base")]
pub struct ActivityStreamTentativeAccept {
    #[serde(flatten)]
    _base: ActivityStreamAccept
}

impl ActivityStreamAdd {
    pub fn create() -> Self {
        let mut activity = ActivityStreamAdd::default();
        activity.set_type(ActivityStreamEntityType::Add);
        activity
    }
}

#[allow(non_snake_case)]
#[derive(Debug, Default, Delegate, Serialize, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
#[delegate(ActivityStreamActivityProperties, target = "_base")]
#[delegate(ActivityStreamObjectProperties, target = "_base")]
#[delegate(ActivityStreamEntityProperties, target = "_base")]
pub struct ActivityStreamAdd {
    #[serde(flatten)]
    _base: ActivityStreamActivity
}

impl ActivityStreamArrive {
    pub fn create() -> Self {
        let mut activity = ActivityStreamArrive::default();
        activity.set_type(ActivityStreamEntityType::Arrive);
        activity
    }
}

#[allow(non_snake_case)]
#[derive(Debug, Default, Delegate, Serialize, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
#[delegate(ActivityStreamIntransitiveActivityProperties, target = "_base")]
#[delegate(ActivityStreamObjectProperties, target = "_base")]
#[delegate(ActivityStreamEntityProperties, target = "_base")]
pub struct ActivityStreamArrive {
    #[serde(flatten)]
    _base: ActivityStreamIntransitiveActivity
}

impl ActivityStreamCreate {
    pub fn create() -> Self {
        let mut activity = ActivityStreamCreate::default();
        activity.set_type(ActivityStreamEntityType::Create);
        activity
    }
}

#[allow(non_snake_case)]
#[derive(Debug, Default, Delegate, Serialize, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
#[delegate(ActivityStreamActivityProperties, target = "_base")]
#[delegate(ActivityStreamObjectProperties, target = "_base")]
#[delegate(ActivityStreamEntityProperties, target = "_base")]
pub struct ActivityStreamCreate {
    #[serde(flatten)]
    _base: ActivityStreamActivity
}

impl ActivityStreamDelete {
    pub fn create() -> Self {
        let mut activity = ActivityStreamDelete::default();
        activity.set_type(ActivityStreamEntityType::Delete);
        activity
    }
}

#[allow(non_snake_case)]
#[derive(Debug, Default, Delegate, Serialize, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
#[delegate(ActivityStreamActivityProperties, target = "_base")]
#[delegate(ActivityStreamObjectProperties, target = "_base")]
#[delegate(ActivityStreamEntityProperties, target = "_base")]
pub struct ActivityStreamDelete {
    #[serde(flatten)]
    _base: ActivityStreamActivity
}

impl ActivityStreamFollow {
    pub fn create() -> Self {
        let mut activity = ActivityStreamFollow::default();
        activity.set_type(ActivityStreamEntityType::Follow);
        activity
    }
}

#[allow(non_snake_case)]
#[derive(Debug, Default, Delegate, Serialize, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
#[delegate(ActivityStreamActivityProperties, target = "_base")]
#[delegate(ActivityStreamObjectProperties, target = "_base")]
#[delegate(ActivityStreamEntityProperties, target = "_base")]
pub struct ActivityStreamFollow {
    #[serde(flatten)]
    _base: ActivityStreamActivity
}

impl ActivityStreamIgnore {
    pub fn create() -> Self {
        let mut activity = ActivityStreamIgnore::default();
        activity.set_type(ActivityStreamEntityType::Ignore);
        activity
    }
}

#[allow(non_snake_case)]
#[derive(Debug, Default, Delegate, Serialize, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
#[delegate(ActivityStreamActivityProperties, target = "_base")]
#[delegate(ActivityStreamObjectProperties, target = "_base")]
#[delegate(ActivityStreamEntityProperties, target = "_base")]
pub struct ActivityStreamIgnore {
    #[serde(flatten)]
    _base: ActivityStreamActivity
}

impl ActivityStreamJoin {
    pub fn create() -> Self {
        let mut activity = ActivityStreamJoin::default();
        activity.set_type(ActivityStreamEntityType::Join);
        activity
    }
}

#[allow(non_snake_case)]
#[derive(Debug, Default, Delegate, Serialize, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
#[delegate(ActivityStreamActivityProperties, target = "_base")]
#[delegate(ActivityStreamObjectProperties, target = "_base")]
#[delegate(ActivityStreamEntityProperties, target = "_base")]
pub struct ActivityStreamJoin {
    #[serde(flatten)]
    _base: ActivityStreamActivity
}

impl ActivityStreamLeave {
    pub fn create() -> Self {
        let mut activity = ActivityStreamLeave::default();
        activity.set_type(ActivityStreamEntityType::Leave);
        activity
    }
}

#[allow(non_snake_case)]
#[derive(Debug, Default, Delegate, Serialize, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
#[delegate(ActivityStreamActivityProperties, target = "_base")]
#[delegate(ActivityStreamObjectProperties, target = "_base")]
#[delegate(ActivityStreamEntityProperties, target = "_base")]
pub struct ActivityStreamLeave {
    #[serde(flatten)]
    _base: ActivityStreamActivity
}

impl ActivityStreamLike {
    pub fn create() -> Self {
        let mut activity = ActivityStreamLike::default();
        activity.set_type(ActivityStreamEntityType::Like);
        activity
    }
}

#[allow(non_snake_case)]
#[derive(Debug, Default, Delegate, Serialize, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
#[delegate(ActivityStreamActivityProperties, target = "_base")]
#[delegate(ActivityStreamObjectProperties, target = "_base")]
#[delegate(ActivityStreamEntityProperties, target = "_base")]
pub struct ActivityStreamLike {
    #[serde(flatten)]
    _base: ActivityStreamActivity
}

impl ActivityStreamOffer {
    pub fn create() -> Self {
        let mut activity = ActivityStreamOffer::default();
        activity.set_type(ActivityStreamEntityType::Offer);
        activity
    }
}

#[allow(non_snake_case)]
#[derive(Debug, Default, Delegate, Serialize, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
#[delegate(ActivityStreamActivityProperties, target = "_base")]
#[delegate(ActivityStreamObjectProperties, target = "_base")]
#[delegate(ActivityStreamEntityProperties, target = "_base")]
pub struct ActivityStreamOffer {
    #[serde(flatten)]
    _base: ActivityStreamActivity
}

impl ActivityStreamInvite {
    pub fn create() -> Self {
        let mut activity = ActivityStreamInvite::default();
        activity.set_type(ActivityStreamEntityType::Invite);
        activity
    }
}

#[allow(non_snake_case)]
#[derive(Debug, Default, Delegate, Serialize, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
#[delegate(ActivityStreamActivityProperties, target = "_base")]
#[delegate(ActivityStreamObjectProperties, target = "_base")]
#[delegate(ActivityStreamEntityProperties, target = "_base")]
pub struct ActivityStreamInvite {
    #[serde(flatten)]
    _base: ActivityStreamOffer
}

impl ActivityStreamReject {
    pub fn create() -> Self {
        let mut activity = ActivityStreamReject::default();
        activity.set_type(ActivityStreamEntityType::Reject);
        activity
    }
}

#[allow(non_snake_case)]
#[derive(Debug, Default, Delegate, Serialize, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
#[delegate(ActivityStreamActivityProperties, target = "_base")]
#[delegate(ActivityStreamObjectProperties, target = "_base")]
#[delegate(ActivityStreamEntityProperties, target = "_base")]
pub struct ActivityStreamReject {
    #[serde(flatten)]
    _base: ActivityStreamActivity
}

impl ActivityStreamTentativeReject {
    pub fn create() -> Self {
        let mut activity = ActivityStreamTentativeReject::default();
        activity.set_type(ActivityStreamEntityType::TentativeReject);
        activity
    }
}

#[allow(non_snake_case)]
#[derive(Debug, Default, Delegate, Serialize, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
#[delegate(ActivityStreamActivityProperties, target = "_base")]
#[delegate(ActivityStreamObjectProperties, target = "_base")]
#[delegate(ActivityStreamEntityProperties, target = "_base")]
pub struct ActivityStreamTentativeReject {
    #[serde(flatten)]
    _base: ActivityStreamReject
}

impl ActivityStreamRemove {
    pub fn create() -> Self {
        let mut activity = ActivityStreamRemove::default();
        activity.set_type(ActivityStreamEntityType::Remove);
        activity
    }
}

#[allow(non_snake_case)]
#[derive(Debug, Default, Delegate, Serialize, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
#[delegate(ActivityStreamActivityProperties, target = "_base")]
#[delegate(ActivityStreamObjectProperties, target = "_base")]
#[delegate(ActivityStreamEntityProperties, target = "_base")]
pub struct ActivityStreamRemove {
    #[serde(flatten)]
    _base: ActivityStreamActivity
}

impl ActivityStreamUndo {
    pub fn create() -> Self {
        let mut activity = ActivityStreamUndo::default();
        activity.set_type(ActivityStreamEntityType::Undo);
        activity
    }
}

#[allow(non_snake_case)]
#[derive(Debug, Default, Delegate, Serialize, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
#[delegate(ActivityStreamActivityProperties, target = "_base")]
#[delegate(ActivityStreamObjectProperties, target = "_base")]
#[delegate(ActivityStreamEntityProperties, target = "_base")]
pub struct ActivityStreamUndo {
    #[serde(flatten)]
    _base: ActivityStreamActivity
}

impl ActivityStreamUpdate {
    pub fn create() -> Self {
        let mut activity = ActivityStreamUpdate::default();
        activity.set_type(ActivityStreamEntityType::Update);
        activity
    }
}

#[allow(non_snake_case)]
#[derive(Debug, Default, Delegate, Serialize, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
#[delegate(ActivityStreamActivityProperties, target = "_base")]
#[delegate(ActivityStreamObjectProperties, target = "_base")]
#[delegate(ActivityStreamEntityProperties, target = "_base")]
pub struct ActivityStreamUpdate {
    #[serde(flatten)]
    _base: ActivityStreamActivity
}

impl ActivityStreamView {
    pub fn create() -> Self {
        let mut activity = ActivityStreamView::default();
        activity.set_type(ActivityStreamEntityType::View);
        activity
    }
}

#[allow(non_snake_case)]
#[derive(Debug, Default, Delegate, Serialize, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
#[delegate(ActivityStreamActivityProperties, target = "_base")]
#[delegate(ActivityStreamObjectProperties, target = "_base")]
#[delegate(ActivityStreamEntityProperties, target = "_base")]
pub struct ActivityStreamView {
    #[serde(flatten)]
    _base: ActivityStreamActivity
}

impl ActivityStreamListen {
    pub fn create() -> Self {
        let mut activity = ActivityStreamListen::default();
        activity.set_type(ActivityStreamEntityType::Listen);
        activity
    }
}

#[allow(non_snake_case)]
#[derive(Debug, Default, Delegate, Serialize, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
#[delegate(ActivityStreamActivityProperties, target = "_base")]
#[delegate(ActivityStreamObjectProperties, target = "_base")]
#[delegate(ActivityStreamEntityProperties, target = "_base")]
pub struct ActivityStreamListen {
    #[serde(flatten)]
    _base: ActivityStreamActivity
}

impl ActivityStreamRead {
    pub fn create() -> Self {
        let mut activity = ActivityStreamRead::default();
        activity.set_type(ActivityStreamEntityType::Read);
        activity
    }
}

#[allow(non_snake_case)]
#[derive(Debug, Default, Delegate, Serialize, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
#[delegate(ActivityStreamActivityProperties, target = "_base")]
#[delegate(ActivityStreamObjectProperties, target = "_base")]
#[delegate(ActivityStreamEntityProperties, target = "_base")]
pub struct ActivityStreamRead {
    #[serde(flatten)]
    _base: ActivityStreamActivity
}

impl ActivityStreamMove {
    pub fn create() -> Self {
        let mut activity = ActivityStreamMove::default();
        activity.set_type(ActivityStreamEntityType::Move);
        activity
    }
}

#[allow(non_snake_case)]
#[derive(Debug, Default, Delegate, Serialize, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
#[delegate(ActivityStreamActivityProperties, target = "_base")]
#[delegate(ActivityStreamObjectProperties, target = "_base")]
#[delegate(ActivityStreamEntityProperties, target = "_base")]
pub struct ActivityStreamMove {
    #[serde(flatten)]
    _base: ActivityStreamActivity
}

impl ActivityStreamTravel {
    pub fn create() -> Self {
        let mut activity = ActivityStreamTravel::default();
        activity.set_type(ActivityStreamEntityType::Travel);
        activity
    }
}

#[allow(non_snake_case)]
#[derive(Debug, Default, Delegate, Serialize, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
#[delegate(ActivityStreamIntransitiveActivityProperties, target = "_base")]
#[delegate(ActivityStreamObjectProperties, target = "_base")]
#[delegate(ActivityStreamEntityProperties, target = "_base")]
pub struct ActivityStreamTravel {
    #[serde(flatten)]
    _base: ActivityStreamIntransitiveActivity
}

impl ActivityStreamAnnounce {
    pub fn create() -> Self {
        let mut activity = ActivityStreamAnnounce::default();
        activity.set_type(ActivityStreamEntityType::Announce);
        activity
    }
}

#[allow(non_snake_case)]
#[derive(Debug, Default, Delegate, Serialize, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
#[delegate(ActivityStreamActivityProperties, target = "_base")]
#[delegate(ActivityStreamObjectProperties, target = "_base")]
#[delegate(ActivityStreamEntityProperties, target = "_base")]
pub struct ActivityStreamAnnounce {
    #[serde(flatten)]
    _base: ActivityStreamActivity
}

impl ActivityStreamBlock {
    pub fn create() -> Self {
        let mut activity = ActivityStreamBlock::default();
        activity.set_type(ActivityStreamEntityType::Block);
        activity
    }
}

#[allow(non_snake_case)]
#[derive(Debug, Default, Delegate, Serialize, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
#[delegate(ActivityStreamActivityProperties, target = "_base")]
#[delegate(ActivityStreamObjectProperties, target = "_base")]
#[delegate(ActivityStreamEntityProperties, target = "_base")]
pub struct ActivityStreamBlock {
    #[serde(flatten)]
    _base: ActivityStreamIgnore
}

impl ActivityStreamFlag {
    pub fn create() -> Self {
        let mut activity = ActivityStreamFlag::default();
        activity.set_type(ActivityStreamEntityType::Flag);
        activity
    }
}

#[allow(non_snake_case)]
#[derive(Debug, Default, Delegate, Serialize, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
#[delegate(ActivityStreamActivityProperties, target = "_base")]
#[delegate(ActivityStreamObjectProperties, target = "_base")]
#[delegate(ActivityStreamEntityProperties, target = "_base")]
pub struct ActivityStreamFlag {
    #[serde(flatten)]
    _base: ActivityStreamActivity
}

impl ActivityStreamDislike {
    pub fn create() -> Self {
        let mut activity = ActivityStreamDislike::default();
        activity.set_type(ActivityStreamEntityType::Dislike);
        activity
    }
}

#[allow(non_snake_case)]
#[derive(Debug, Default, Delegate, Serialize, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
#[delegate(ActivityStreamActivityProperties, target = "_base")]
#[delegate(ActivityStreamObjectProperties, target = "_base")]
#[delegate(ActivityStreamEntityProperties, target = "_base")]
pub struct ActivityStreamDislike {
    #[serde(flatten)]
    _base: ActivityStreamActivity
}

impl ActivityStreamQuestion {
    pub fn create() -> Self {
        let mut activity = ActivityStreamQuestion::default();
        activity.set_type(ActivityStreamEntityType::Question);
        activity
    }
}

impl ActivityStreamQuestionProperties for ActivityStreamQuestion {
/*
        fn set_one_of<S: ActivityStreamEntityProperties, T: MaybeOptional<Vec<S>>>(&mut self, one_of: T) where ActivityStreamEntity: From<S>;
        fn add_one_of<S: ActivityStreamEntityProperties, T: MaybeOptional<S>>(&mut self, one_of: T) where ActivityStreamEntity: From<S>;
        fn get_any_of(&self) -> &Option<Vec<ActivityStreamEntity>>;
        fn set_any_of<S: ActivityStreamEntityProperties, T: MaybeOptional<Vec<S>>>(&mut self, any_of: T) where ActivityStreamEntity: From<S>;
        fn add_any_of<S: ActivityStreamEntityProperties, T: MaybeOptional<S>>(&mut self, one_of: T) where ActivityStreamEntity: From<S>;
        fn get_closed(&self) -> &Option<Vec<ActivityStreamQuestionClosed>>;
        fn set_closed<S, T: MaybeOptional<S>>(&mut self, any_of: T) where ActivityStreamQuestionClosed: From<S>;
*/

  fn get_one_of(&self) -> &Option<Vec<ActivityStreamEntity>> {
      &self.oneOf
  }

  fn set_one_of<S: ActivityStreamEntityProperties, T: MaybeOptional<Vec<S>>>(
      &mut self,
      one_of: T,
  )
  where
      ActivityStreamEntity: From<S>,
  {
      if let Some(one_of) = one_of.get_optional() {
          let one_of: Vec<ActivityStreamEntity> = one_of
                  .into_iter()
                  .map(ActivityStreamEntity::from)
                  .collect();
          self.oneOf = Some(one_of);
      }
  }

    fn add_one_of<S: ActivityStreamEntityProperties, T: MaybeOptional<S>>(
        &mut self,
        one_of: T,
    )
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

  fn set_any_of<S: ActivityStreamEntityProperties, T: MaybeOptional<Vec<S>>>(
      &mut self,
      any_of: T,
  )
  where
      ActivityStreamEntity: From<S>,
  {
      if let Some(any_of) = any_of.get_optional() {
          let any_of: Vec<ActivityStreamEntity> = any_of
                  .into_iter()
                  .map(ActivityStreamEntity::from)
                  .collect();
          self.anyOf = Some(any_of);
      }
  }

    fn add_any_of<S: ActivityStreamEntityProperties, T: MaybeOptional<S>>(
        &mut self,
        any_of: T,
    )
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

    fn get_closed(&self) -> &Option<ActivityStreamQuestionClosed> {
        &self.closed
    }

    fn set_closed<S, T: MaybeOptional<S>>(&mut self, closed: T) where ActivityStreamQuestionClosed: From<S> {
        if let Some(closed) = closed.get_optional() {
            self.closed = Some(ActivityStreamQuestionClosed::from(closed));
        }
    }

}

#[allow(non_snake_case)]
#[derive(Debug, Default, Delegate, Serialize, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
#[delegate(ActivityStreamIntransitiveActivityProperties, target = "_base")]
#[delegate(ActivityStreamObjectProperties, target = "_base")]
#[delegate(ActivityStreamEntityProperties, target = "_base")]
pub struct ActivityStreamQuestion {
    #[serde(flatten)]
    _base: ActivityStreamIntransitiveActivity,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    oneOf: Option<Vec<ActivityStreamEntity>>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    anyOf: Option<Vec<ActivityStreamEntity>>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    closed: Option<ActivityStreamQuestionClosed>,
}
