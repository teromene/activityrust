use crate::entities::link::ActivityStreamLink;
use crate::entities::entity::ActivityStreamEntity;
use crate::entities::collection::ActivityStreamCollection;
use crate::entities::collectionpage::ActivityStreamCollectionPage;
use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use url::Url;
use chrono::{DateTime, Utc};

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
