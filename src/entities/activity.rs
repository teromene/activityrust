use serde::{Serialize, Deserialize};
use crate::entities::entity::{ActivityStreamEntityType, BoxedActivityStreamEntity, ActivityStreamEntity};
use crate::entities::object::ActivityStreamObject;
use crate::entities::collection::ActivityStreamCollection;
use crate::traits::properties::*;
use ambassador::Delegate;
use crate::{MaybeOptional, OneOrMultiple};
use url::Url;
use crate::content::*;
use chrono::{DateTime, Utc};

impl ActivityStreamEntityProperties for ActivityStreamActivity {
    fn get_id(&self) -> &Option<Url> {
        &self.id
    }

    fn set_id<T: MaybeOptional<Url>>(&mut self, id: T) {
        self.id = id.get_optional();
    }

    fn get_type(&self) -> &ActivityStreamEntityType {
      &self.r#type
    }

    fn set_type(&mut self, r#type: ActivityStreamEntityType) {
      self.r#type = r#type;
    }

    fn register_context(&mut self, new_context: Url) {
        if let Some(ref mut context) = self.context {
            context.append(new_context);
        } else {
            self.context = Some(OneOrMultiple::Element(new_context));
        }
    }

}

impl ActivityStreamActivityProperties for ActivityStreamActivity {
    fn get_actor(&self) -> &Option<BoxedActivityStreamEntity> {
        &self.actor
    }

    fn set_actor<S: ActivityStreamEntityProperties, T: MaybeOptional<S>>(&mut self, actor: T) where ActivityStreamEntity: From<S> {
        if let Some(actor) = actor.get_optional() {
            self.actor = Some(Box::new(ActivityStreamEntity::from(actor)));
        }
    }

    fn get_object(&self) -> &Option<BoxedActivityStreamEntity> {
        &self.object
    }

    fn set_object<S: ActivityStreamEntityProperties, T: MaybeOptional<S>>(&mut self, object: T) where ActivityStreamEntity: From<S> {
        if let Some(object) = object.get_optional() {
            self.object = Some(Box::new(ActivityStreamEntity::from(object)));
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

impl ActivityStreamActivity {

  pub fn create() -> Self {

    let object_context = Url::parse("https://www.w3.org/ns/activitystreams").unwrap();

    let mut new_object = ActivityStreamActivity::default();
    new_object.register_context(object_context);
    new_object.set_type(ActivityStreamEntityType::Activity);
    new_object
  }

}

#[allow(non_snake_case)]
#[derive(Debug, Default, Delegate, Serialize, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
#[delegate(ActivityStreamObjectProperties, target = "_base")]
pub struct ActivityStreamActivity {
    #[serde(skip_serializing_if = "Option::is_none", default)]
    id: Option<Url>,
    r#type: ActivityStreamEntityType,
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

