use crate::entities::collection::ActivityStreamCollection;
use crate::entities::orderedcollection::ActivityStreamOrderedCollection;
use crate::entities::collectionpage::ActivityStreamCollectionPage;
use crate::entities::entity::ActivityStreamEntity;
use crate::entities::link::ActivityStreamLink;
use crate::entities::objecttypes::*;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use url::Url;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ActivityStreamMultilangString {
    String(String),
    LanguageMap(HashMap<String, String>),
}

impl From<String> for ActivityStreamMultilangString {
    fn from(string: String) -> Self {
        ActivityStreamMultilangString::String(string)
    }
}

impl From<&str> for ActivityStreamMultilangString {
    fn from(string: &str) -> Self {
        ActivityStreamMultilangString::String(String::from(string))
    }
}

impl From<HashMap<String, String>> for ActivityStreamMultilangString {
    fn from(string: HashMap<String, String>) -> Self {
        ActivityStreamMultilangString::LanguageMap(string)
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum ActivityStreamLinkableImage {
    Link(ActivityStreamLink),
    //FIXME: Image(ActivityStreamImage)
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ActivityStreamLinkableUrl {
    Url(Url),
    Link(ActivityStreamLink),
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ActivityStreamLinkableCollection {
    Url(Url),
    Collection(ActivityStreamCollection),
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ActivityStreamLinkableOrderedCollection {
    Url(Url),
    OrderedCollection(ActivityStreamOrderedCollection),
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ActivityStreamLinkableCollectionPage {
    Url(Url),
    CollectionPage(ActivityStreamCollectionPage),
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ActivityStreamQuestionClosed {
    Entity(ActivityStreamEntity),
    Bool(bool),
    Date(DateTime<Utc>),
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ActivityStreamLinkableRelationship {
    Url(Url),
    Relationship(ActivityStreamRelationship),
}

//FIXME: URL type
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum ActivityStreamUnit {
  #[serde(rename = "cm")]
  Centimeter,
  #[serde(rename = "feet")]
  Feet,
  #[serde(rename = "inches")]
  Inches,
  #[serde(rename = "km")]
  Kilometer,
  #[serde(rename = "m")]
  Meter,
  #[serde(rename = "miles")]
  Miles,
}
