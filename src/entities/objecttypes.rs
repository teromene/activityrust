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
#[allow(non_snake_case)]
use serde::{Deserialize, Serialize};
use url::Url;

generate_basics!(
    ActivityStreamRelationship,
    ActivityStreamEntityType::Relationship
);

impl ActivityStreamRelationshipProperties for ActivityStreamRelationship {
    fn get_subject(&self) -> &Option<BoxedActivityStreamEntity> {
        &self.subject
    }

    fn set_subject<S, T: MaybeOptional<S>>(&mut self, subject: T)
    where
        ActivityStreamEntity: From<S>,
    {
        if let Some(subject) = subject.get_optional() {
            self.subject = Some(Box::new(ActivityStreamEntity::from(subject)));
        }
    }

    fn get_object(&self) -> &Option<BoxedActivityStreamEntity> {
        &self.object
    }

    fn set_object<S, T: MaybeOptional<S>>(&mut self, object: T)
    where
        ActivityStreamEntity: From<S>,
    {
        if let Some(object) = object.get_optional() {
            self.object = Some(Box::new(ActivityStreamEntity::from(object)));
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

#[allow(non_snake_case)]
#[derive(Debug, Default, Delegate, Serialize, Deserialize, PartialEq)]
#[delegate(ActivityStreamObjectProperties, target = "_base")]
pub struct ActivityStreamRelationship {
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[serde(deserialize_with = "ActivityStreamRelationship::deserialize_type")]
    r#type: Option<ActivityStreamEntityType>,
    #[serde(flatten)]
    _base: ActivityStreamObject,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    subject: Option<BoxedActivityStreamEntity>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    object: Option<BoxedActivityStreamEntity>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    relationship: Option<Box<ActivityStreamLinkableRelationship>>,
}

generate_basics!(ActivityStreamArticle, ActivityStreamEntityType::Article);

#[derive(Debug, Default, Delegate, Serialize, Deserialize, PartialEq)]
#[delegate(ActivityStreamObjectProperties, target = "_base")]
pub struct ActivityStreamArticle {
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[serde(deserialize_with = "ActivityStreamArticle::deserialize_type")]
    r#type: Option<ActivityStreamEntityType>,
    #[serde(flatten)]
    _base: ActivityStreamObject,
}

generate_basics!(ActivityStreamDocument, ActivityStreamEntityType::Document);

#[derive(Debug, Default, Delegate, Serialize, Deserialize, PartialEq)]
#[delegate(ActivityStreamObjectProperties, target = "_base")]
pub struct ActivityStreamDocument {
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[serde(deserialize_with = "ActivityStreamDocument::deserialize_type")]
    r#type: Option<ActivityStreamEntityType>,
    #[serde(flatten)]
    _base: ActivityStreamObject,
}

generate_basics!(ActivityStreamAudio, ActivityStreamEntityType::Audio);

#[derive(Debug, Default, Delegate, Serialize, Deserialize, PartialEq)]
#[delegate(ActivityStreamObjectProperties, target = "_base")]
pub struct ActivityStreamAudio {
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[serde(deserialize_with = "ActivityStreamAudio::deserialize_type")]
    r#type: Option<ActivityStreamEntityType>,
    #[serde(flatten)]
    _base: ActivityStreamDocument,
}

generate_basics!(ActivityStreamImage, ActivityStreamEntityType::Image);

#[derive(Debug, Default, Delegate, Serialize, Deserialize, PartialEq)]
#[delegate(ActivityStreamObjectProperties, target = "_base")]
pub struct ActivityStreamImage {
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[serde(deserialize_with = "ActivityStreamImage::deserialize_type")]
    r#type: Option<ActivityStreamEntityType>,
    #[serde(flatten)]
    _base: ActivityStreamDocument,
}

generate_basics!(ActivityStreamVideo, ActivityStreamEntityType::Video);

#[derive(Debug, Default, Delegate, Serialize, Deserialize, PartialEq)]
#[delegate(ActivityStreamObjectProperties, target = "_base")]
pub struct ActivityStreamVideo {
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[serde(deserialize_with = "ActivityStreamVideo::deserialize_type")]
    r#type: Option<ActivityStreamEntityType>,
    #[serde(flatten)]
    _base: ActivityStreamDocument,
}

generate_basics!(ActivityStreamNote, ActivityStreamEntityType::Note);

#[derive(Debug, Default, Delegate, Serialize, Deserialize, PartialEq)]
#[delegate(ActivityStreamObjectProperties, target = "_base")]
pub struct ActivityStreamNote {
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[serde(deserialize_with = "ActivityStreamNote::deserialize_type")]
    r#type: Option<ActivityStreamEntityType>,
    #[serde(flatten)]
    _base: ActivityStreamObject,
}

generate_basics!(ActivityStreamPage, ActivityStreamEntityType::Page);

#[derive(Debug, Default, Delegate, Serialize, Deserialize, PartialEq)]
#[delegate(ActivityStreamObjectProperties, target = "_base")]
pub struct ActivityStreamPage {
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[serde(deserialize_with = "ActivityStreamPage::deserialize_type")]
    r#type: Option<ActivityStreamEntityType>,
    #[serde(flatten)]
    _base: ActivityStreamDocument,
}

generate_basics!(ActivityStreamEvent, ActivityStreamEntityType::Event);

#[derive(Debug, Default, Delegate, Serialize, Deserialize, PartialEq)]
#[delegate(ActivityStreamObjectProperties, target = "_base")]
pub struct ActivityStreamEvent {
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[serde(deserialize_with = "ActivityStreamEvent::deserialize_type")]
    r#type: Option<ActivityStreamEntityType>,
    #[serde(flatten)]
    _base: ActivityStreamObject,
}

generate_basics!(ActivityStreamPlace, ActivityStreamEntityType::Place);

impl ActivityStreamPlaceProperties for ActivityStreamPlace {
    fn get_accuracy(&self) -> &Option<f64> {
        &self.accuracy
    }

    fn set_accuracy<T: MaybeOptional<f64>>(&mut self, accuracy: T) {
        self.accuracy = accuracy.get_optional();
    }

    fn get_altitude(&self) -> &Option<f64> {
        &self.altitude
    }

    fn set_altitude<T: MaybeOptional<f64>>(&mut self, altitude: T) {
        self.altitude = altitude.get_optional();
    }

    fn get_latitude(&self) -> &Option<f64> {
        &self.latitude
    }

    fn set_latitude<T: MaybeOptional<f64>>(&mut self, latitude: T) {
        self.latitude = latitude.get_optional();
    }

    fn get_longitude(&self) -> &Option<f64> {
        &self.longitude
    }

    fn set_longitude<T: MaybeOptional<f64>>(&mut self, longitude: T) {
        self.longitude = longitude.get_optional();
    }

    fn get_radius(&self) -> &Option<f64> {
        &self.radius
    }

    fn set_radius<T: MaybeOptional<f64>>(&mut self, radius: T) {
        self.radius = radius.get_optional();
    }

    fn get_units(&self) -> &Option<ActivityStreamUnit> {
        &self.units
    }

    fn set_units<T: MaybeOptional<ActivityStreamUnit>>(&mut self, units: T) {
        self.units = units.get_optional();
    }
}

#[derive(Debug, Default, Delegate, Serialize, Deserialize, PartialEq)]
#[delegate(ActivityStreamObjectProperties, target = "_base")]
pub struct ActivityStreamPlace {
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[serde(deserialize_with = "ActivityStreamPlace::deserialize_type")]
    r#type: Option<ActivityStreamEntityType>,
    #[serde(flatten)]
    _base: ActivityStreamObject,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    accuracy: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    altitude: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    latitude: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    longitude: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    radius: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    units: Option<ActivityStreamUnit>,
}

generate_basics!(ActivityStreamProfile, ActivityStreamEntityType::Profile);

impl ActivityStreamProfileProperties for ActivityStreamProfile {
    fn get_describes(&self) -> &Option<BoxedActivityStreamEntity> {
        &self.describes
    }

    fn set_describes<S, T: MaybeOptional<S>>(&mut self, describes: T)
    where
        ActivityStreamEntity: From<S>,
    {
        if let Some(describes) = describes.get_optional() {
            self.describes = Some(Box::new(ActivityStreamEntity::from(describes)));
        }
    }
}

#[derive(Debug, Default, Delegate, Serialize, Deserialize, PartialEq)]
#[delegate(ActivityStreamObjectProperties, target = "_base")]
pub struct ActivityStreamProfile {
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[serde(deserialize_with = "ActivityStreamProfile::deserialize_type")]
    r#type: Option<ActivityStreamEntityType>,
    #[serde(flatten)]
    _base: ActivityStreamObject,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    describes: Option<BoxedActivityStreamEntity>,
}

generate_basics!(ActivityStreamTombstone, ActivityStreamEntityType::Tombstone);

impl ActivityStreamTombstoneProperties for ActivityStreamTombstone {
    fn get_former_type(&self) -> &Option<ActivityStreamEntityType> {
        &self.formerType
    }

    fn set_former_type<T: MaybeOptional<ActivityStreamEntityType>>(&mut self, former_type: T) {
        self.formerType = former_type.get_optional();
    }

    fn get_deleted(&self) -> &Option<DateTime<FixedOffset>> {
        &self.deleted
    }

    fn set_deleted<T: MaybeOptional<DateTime<FixedOffset>>>(&mut self, deleted: T) {
        self.deleted = deleted.get_optional();
    }
}

#[allow(non_snake_case)]
#[derive(Debug, Default, Delegate, Serialize, Deserialize, PartialEq)]
#[delegate(ActivityStreamObjectProperties, target = "_base")]
pub struct ActivityStreamTombstone {
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[serde(deserialize_with = "ActivityStreamTombstone::deserialize_type")]
    r#type: Option<ActivityStreamEntityType>,
    #[serde(flatten)]
    _base: ActivityStreamObject,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    formerType: Option<ActivityStreamEntityType>,
    #[serde(
        skip_serializing_if = "Option::is_none",
        default,
        with = "crate::traits::optionaldateserializer"
    )]
    deleted: Option<DateTime<FixedOffset>>,
}
