use crate::content::*;
use crate::entities::activity::ActivityStreamActivity;
use crate::entities::collection::ActivityStreamCollection;
use crate::entities::entity::{
    ActivityStreamEntity, ActivityStreamEntityType, BoxedActivityStreamEntity,
};
use crate::entities::intransitiveactivity::ActivityStreamIntransitiveActivity;
use crate::entities::object::ActivityStreamObject;
use crate::traits::properties::*;
use crate::{MaybeOptional, OneOrMultiple};
use ambassador::Delegate;
use chrono::{DateTime, Utc};
#[allow(non_snake_case)]
use serde::{Deserialize, Deserializer, Serialize};
use url::Url;

generate_basics!(
    ActivityStreamRelationship,
    ActivityStreamEntityType::Relationship
);

impl ActivityStreamRelationshipProperties for ActivityStreamRelationship {
    fn get_subject(&self) -> &Option<ActivityStreamEntity> {
        &self.subject
    }

    fn set_subject<S, T: MaybeOptional<S>>(&mut self, subject: T)
    where
        ActivityStreamEntity: From<S>,
    {
        if let Some(subject) = subject.get_optional() {
            self.subject = Some(ActivityStreamEntity::from(subject));
        }
    }

    fn get_object(&self) -> &Option<ActivityStreamEntity> {
        &self.object
    }

    fn set_object<S, T: MaybeOptional<S>>(&mut self, object: T)
    where
        ActivityStreamEntity: From<S>,
    {
        if let Some(object) = object.get_optional() {
            self.object = Some(ActivityStreamEntity::from(object));
        }
    }

    fn get_relationship(&self) -> &Option<Box<ActivityStreamLinkableRelationship>> {
        &self.relationship
    }

    fn set_relationship<S, T: MaybeOptional<S>>(&mut self, relationship: T)
    where
        ActivityStreamLinkableRelationship: From<S>,
    {
        if let Some(relationship) = relationship.get_optional() {
            self.relationship = Some(Box::new(ActivityStreamLinkableRelationship::from(
                relationship,
            )));
        }
    }
}

#[derive(Debug, Default, Delegate, Serialize, Deserialize, PartialEq)]
#[delegate(ActivityStreamObjectProperties, target = "_base")]
pub struct ActivityStreamRelationship {
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[serde(deserialize_with = "ActivityStreamRelationship::deserialize_type")]
    r#type: Option<ActivityStreamEntityType>,
    #[serde(flatten)]
    _base: ActivityStreamObject,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    subject: Option<ActivityStreamEntity>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    object: Option<ActivityStreamEntity>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    relationship: Option<Box<ActivityStreamLinkableRelationship>>,
}

generate_basics!(
    ActivityStreamArticle,
    ActivityStreamEntityType::Article
);

#[derive(Debug, Default, Delegate, Serialize, Deserialize, PartialEq)]
#[delegate(ActivityStreamObjectProperties, target = "_base")]
pub struct ActivityStreamArticle {
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[serde(deserialize_with = "ActivityStreamArticle::deserialize_type")]
    r#type: Option<ActivityStreamEntityType>,
    #[serde(flatten)]
    _base: ActivityStreamObject,
}

generate_basics!(
    ActivityStreamDocument,
    ActivityStreamEntityType::Document
);

#[derive(Debug, Default, Delegate, Serialize, Deserialize, PartialEq)]
#[delegate(ActivityStreamObjectProperties, target = "_base")]
pub struct ActivityStreamDocument {
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[serde(deserialize_with = "ActivityStreamDocument::deserialize_type")]
    r#type: Option<ActivityStreamEntityType>,
    #[serde(flatten)]
    _base: ActivityStreamObject,
}

generate_basics!(
    ActivityStreamAudio,
    ActivityStreamEntityType::Audio
);

#[derive(Debug, Default, Delegate, Serialize, Deserialize, PartialEq)]
#[delegate(ActivityStreamObjectProperties, target = "_base")]
pub struct ActivityStreamAudio {
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[serde(deserialize_with = "ActivityStreamAudio::deserialize_type")]
    r#type: Option<ActivityStreamEntityType>,
    #[serde(flatten)]
    _base: ActivityStreamDocument,
}

generate_basics!(
    ActivityStreamImage,
    ActivityStreamEntityType::Image
);

#[derive(Debug, Default, Delegate, Serialize, Deserialize, PartialEq)]
#[delegate(ActivityStreamObjectProperties, target = "_base")]
pub struct ActivityStreamImage {
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[serde(deserialize_with = "ActivityStreamImage::deserialize_type")]
    r#type: Option<ActivityStreamEntityType>,
    #[serde(flatten)]
    _base: ActivityStreamDocument,
}

generate_basics!(
    ActivityStreamVideo,
    ActivityStreamEntityType::Video
);

#[derive(Debug, Default, Delegate, Serialize, Deserialize, PartialEq)]
#[delegate(ActivityStreamObjectProperties, target = "_base")]
pub struct ActivityStreamVideo {
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[serde(deserialize_with = "ActivityStreamVideo::deserialize_type")]
    r#type: Option<ActivityStreamEntityType>,
    #[serde(flatten)]
    _base: ActivityStreamDocument,
}

generate_basics!(
    ActivityStreamNote,
    ActivityStreamEntityType::Note
);

#[derive(Debug, Default, Delegate, Serialize, Deserialize, PartialEq)]
#[delegate(ActivityStreamObjectProperties, target = "_base")]
pub struct ActivityStreamNote {
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[serde(deserialize_with = "ActivityStreamNote::deserialize_type")]
    r#type: Option<ActivityStreamEntityType>,
    #[serde(flatten)]
    _base: ActivityStreamObject,
}

generate_basics!(
    ActivityStreamPage,
    ActivityStreamEntityType::Page
);

#[derive(Debug, Default, Delegate, Serialize, Deserialize, PartialEq)]
#[delegate(ActivityStreamObjectProperties, target = "_base")]
pub struct ActivityStreamPage {
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[serde(deserialize_with = "ActivityStreamPage::deserialize_type")]
    r#type: Option<ActivityStreamEntityType>,
    #[serde(flatten)]
    _base: ActivityStreamDocument,
}

generate_basics!(
    ActivityStreamEvent,
    ActivityStreamEntityType::Event
);

#[derive(Debug, Default, Delegate, Serialize, Deserialize, PartialEq)]
#[delegate(ActivityStreamObjectProperties, target = "_base")]
pub struct ActivityStreamEvent {
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[serde(deserialize_with = "ActivityStreamEvent::deserialize_type")]
    r#type: Option<ActivityStreamEntityType>,
    #[serde(flatten)]
    _base: ActivityStreamObject,
}
