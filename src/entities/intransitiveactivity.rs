use serde::{Serialize, Deserialize, Deserializer};
use crate::entities::entity::{ActivityStreamEntityType, BoxedActivityStreamEntity, ActivityStreamEntity};
use crate::entities::object::ActivityStreamObject;
use crate::traits::properties::*;
use ambassador::Delegate;
use crate::{MaybeOptional, OneOrMultiple};
use url::Url;
use crate::entities::collection::ActivityStreamCollection;
use crate::content::*;
use chrono::{DateTime, Utc};

impl ActivityStreamIntransitiveActivityProperties for ActivityStreamIntransitiveActivity {
    fn get_actor(&self) -> &Option<BoxedActivityStreamEntity> {
        &self.actor
    }

    fn set_actor<S: ActivityStreamEntityProperties, T: MaybeOptional<S>>(&mut self, actor: T) where ActivityStreamEntity: From<S> {
        if let Some(actor) = actor.get_optional() {
            self.actor = Some(Box::new(ActivityStreamEntity::from(actor)));
        }
    }

    fn get_target(&self) -> &Option<BoxedActivityStreamEntity> {
        &self.target
    }

    fn set_target<S: ActivityStreamEntityProperties, T: MaybeOptional<S>>(&mut self, target: T) where ActivityStreamEntity: From<S> {
        if let Some(target) = target.get_optional() {
            self.target = Some(Box::new(ActivityStreamEntity::from(target)));
        }
    }

    fn get_result(&self) -> &Option<BoxedActivityStreamEntity> {
        &self.result
    }

    fn set_result<S: ActivityStreamEntityProperties, T: MaybeOptional<S>>(&mut self, result: T) where ActivityStreamEntity: From<S> {
        if let Some(result) = result.get_optional() {
            self.result = Some(Box::new(ActivityStreamEntity::from(result)));
        }
    }

    fn get_origin(&self) -> &Option<BoxedActivityStreamEntity> {
        &self.origin
    }

    fn set_origin<S: ActivityStreamEntityProperties, T: MaybeOptional<S>>(&mut self, origin: T) where ActivityStreamEntity: From<S> {
        if let Some(origin) = origin.get_optional() {
            self.origin = Some(Box::new(ActivityStreamEntity::from(origin)));
        }
    }

    fn get_instrument(&self) -> &Option<BoxedActivityStreamEntity> {
        &self.instrument
    }

    fn set_instrument<S: ActivityStreamEntityProperties, T: MaybeOptional<S>>(&mut self, instrument: T) where ActivityStreamEntity: From<S> {
        if let Some(instrument) = instrument.get_optional() {
            self.instrument = Some(Box::new(ActivityStreamEntity::from(instrument)));
        }
    }
}

generate_basics!(ActivityStreamIntransitiveActivity, ActivityStreamEntityType::IntransitiveActivity);

#[derive(Debug, Default, Delegate, Serialize, Deserialize, PartialEq)]
#[delegate(ActivityStreamObjectProperties, target = "_base")]
pub struct ActivityStreamIntransitiveActivity {
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[serde(deserialize_with = "ActivityStreamIntransitiveActivity::deserialize_type")]
    r#type: Option<ActivityStreamEntityType>,
    #[serde(rename = "@context")]
    context: Option<OneOrMultiple<Url>>,
    #[serde(flatten)]
    _base: ActivityStreamObject,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    actor: Option<BoxedActivityStreamEntity>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    object: Option<BoxedActivityStreamEntity>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    target: Option<BoxedActivityStreamEntity>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    result: Option<BoxedActivityStreamEntity>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    origin: Option<BoxedActivityStreamEntity>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    instrument: Option<BoxedActivityStreamEntity>,
}
