use url::Url;
use std::collections::HashMap;
use crate::coretypes::object::ActivityStreamLink;

pub enum ActivityStreamContent {
    String(String),
    LanguageMap(HashMap<String, String>),
}

impl From<String> for ActivityStreamContent {
    fn from(string: String) -> Self {
        ActivityStreamContent::String(string)
    }
}

impl From<HashMap<String, String>> for ActivityStreamContent {
    fn from(string: HashMap<String, String>) -> Self {
        ActivityStreamContent::LanguageMap(string)
    }
}


pub enum ActivityStreamLinkableImage {
    Link(ActivityStreamLink),
    //FIXME: Image(ActivityStreamImage)
}

pub enum ActivityStreamLinkableUrl {
    Url(Url),
    Link(ActivityStreamLink),
}
