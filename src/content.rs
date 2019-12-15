use crate::entities::link::ActivityStreamLink;
use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use url::Url;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum ActivityStreamMultilangString {
    String(String),
    LanguageMap(HashMap<String, String>),
}

impl From<String> for ActivityStreamMultilangString {
    fn from(string: String) -> Self {
        ActivityStreamMultilangString::String(string)
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
pub enum ActivityStreamLinkableUrl {
    Url(Url),
    Link(ActivityStreamLink),
}
