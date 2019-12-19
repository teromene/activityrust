use crate::content::*;
use crate::entities::collection::ActivityStreamCollection;
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

impl ActivityStreamIntransitiveActivityProperties for ActivityStreamIntransitiveActivity {
    fn get_actor(&self) -> &Option<BoxedActivityStreamEntity> {
        &self.actor
    }

    fn set_actor<S, T: MaybeOptional<S>>(&mut self, actor: T)
    where
        ActivityStreamEntity: From<S>,
    {
        if let Some(actor) = actor.get_optional() {
            self.actor = Some(Box::new(ActivityStreamEntity::from(actor)));
        }
    }

    fn get_targets(&self) -> &Option<Vec<ActivityStreamEntity>> {
        &self.target
    }

    fn set_targets<S, T: MaybeOptional<Vec<S>>>(&mut self, targets: T)
    where
        ActivityStreamEntity: From<S>,
    {
        if let Some(targets) = targets.get_optional() {
            let targets: Vec<ActivityStreamEntity> =
                targets.into_iter().map(ActivityStreamEntity::from).collect();
            self.target = Some(targets);
        }
    }

    fn add_target<S, T: MaybeOptional<S>>(&mut self, target: T)
    where
        ActivityStreamEntity: From<S>,
    {
        if let Some(target) = target.get_optional() {
            if self.target.is_none() {
                self.target = Some(Vec::new());
            }

            if let Some(ref mut target_internal) = self.target {
                target_internal.push(ActivityStreamEntity::from(target));
            }
        }
    }

    fn get_result(&self) -> &Option<BoxedActivityStreamEntity> {
        &self.result
    }

    fn set_result<S, T: MaybeOptional<S>>(&mut self, result: T)
    where
        ActivityStreamEntity: From<S>,
    {
        if let Some(result) = result.get_optional() {
            self.result = Some(Box::new(ActivityStreamEntity::from(result)));
        }
    }

    fn get_origin(&self) -> &Option<BoxedActivityStreamEntity> {
        &self.origin
    }

    fn set_origin<S, T: MaybeOptional<S>>(&mut self, origin: T)
    where
        ActivityStreamEntity: From<S>,
    {
        if let Some(origin) = origin.get_optional() {
            self.origin = Some(Box::new(ActivityStreamEntity::from(origin)));
        }
    }

    fn get_instrument(&self) -> &Option<BoxedActivityStreamEntity> {
        &self.instrument
    }

    fn set_instrument<S, T: MaybeOptional<S>>(
        &mut self,
        instrument: T,
    ) where
        ActivityStreamEntity: From<S>,
    {
        if let Some(instrument) = instrument.get_optional() {
            self.instrument = Some(Box::new(ActivityStreamEntity::from(instrument)));
        }
    }
}

generate_basics!(
    ActivityStreamIntransitiveActivity,
    ActivityStreamEntityType::IntransitiveActivity
);

#[derive(Debug, Default, Delegate, Serialize, Deserialize, PartialEq)]
#[delegate(ActivityStreamObjectProperties, target = "_base")]
pub struct ActivityStreamIntransitiveActivity {
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[serde(deserialize_with = "ActivityStreamIntransitiveActivity::deserialize_type")]
    r#type: Option<ActivityStreamEntityType>,
    #[serde(flatten)]
    _base: ActivityStreamObject,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    actor: Option<BoxedActivityStreamEntity>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    object: Option<BoxedActivityStreamEntity>,
    #[serde(skip_serializing_if = "Option::is_none", default, with = "crate::traits::vecserializer")]
    target: Option<Vec<ActivityStreamEntity>>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    result: Option<BoxedActivityStreamEntity>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    origin: Option<BoxedActivityStreamEntity>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    instrument: Option<BoxedActivityStreamEntity>,
}
