use crate::content::ActivityStreamMultilangString;
use crate::entities::entity::{
    ActivityStreamEntity, ActivityStreamEntityType, BoxedActivityStreamEntity,
};
use crate::traits::properties::*;
use crate::{MaybeOptional, OneOrMultiple};
use serde::{Deserialize, Deserializer, Serialize};
use url::Url;

impl ActivityStreamLinkProperties for ActivityStreamLink {
    fn get_id(&self) -> &Option<Url> {
        &self.id
    }

    fn set_id<T: MaybeOptional<Url>>(&mut self, id: T) {
        self.id = id.get_optional();
    }

    fn register_context(&mut self, new_context: Url) {
        if let Some(ref mut context) = self.context {
            context.append(new_context);
        } else {
            self.context = Some(OneOrMultiple::Element(new_context));
        }
    }

    fn get_href(&self) -> &Option<Url> {
        &self.href
    }

    fn set_href<T: MaybeOptional<Url>>(&mut self, href: T) {
        self.href = href.get_optional();
    }

    fn get_hreflang(&self) -> &Option<String> {
        &self.hreflang
    }

    fn set_hreflang<T: MaybeOptional<String>>(&mut self, hreflang: T) {
        self.hreflang = hreflang.get_optional();
    }

    fn get_media_type(&self) -> &Option<String> {
        &self.mediaType
    }

    fn set_media_type<T: MaybeOptional<String>>(&mut self, media_type: T) {
        self.mediaType = media_type.get_optional();
    }

    fn get_name(&self) -> &Option<ActivityStreamMultilangString> {
        &self.name
    }

    fn set_name<S, T: MaybeOptional<S>>(&mut self, name: T)
    where
        ActivityStreamMultilangString: From<S>,
    {
        if let Some(name) = name.get_optional() {
            self.name = Some(ActivityStreamMultilangString::from(name));
        }
    }

    fn get_height(&self) -> &Option<usize> {
        &self.height
    }

    fn set_height<T: MaybeOptional<usize>>(&mut self, height: T) {
        self.height = height.get_optional();
    }

    fn get_width(&self) -> &Option<usize> {
        &self.width
    }

    fn set_width<T: MaybeOptional<usize>>(&mut self, width: T) {
        self.width = width.get_optional();
    }

    fn get_preview(&self) -> &Option<BoxedActivityStreamEntity> {
        &self.preview
    }

    fn set_preview<S, T: MaybeOptional<S>>(&mut self, preview: T)
    where
        ActivityStreamEntity: From<S>,
    {
        if let Some(preview) = preview.get_optional() {
            self.preview = Some(Box::new(ActivityStreamEntity::from(preview)));
        }
    }
}

generate_basics!(ActivityStreamLink, ActivityStreamEntityType::Link);

#[allow(non_snake_case)]
#[derive(Debug, Default, Serialize, Deserialize, PartialEq)]
pub struct ActivityStreamLink {
    #[serde(skip_serializing_if = "Option::is_none", default)]
    id: Option<Url>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[serde(deserialize_with = "ActivityStreamLink::deserialize_type")]
    r#type: Option<ActivityStreamEntityType>,
    #[serde(rename = "@context")]
    context: Option<OneOrMultiple<Url>>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    href: Option<Url>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    hreflang: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    mediaType: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    name: Option<ActivityStreamMultilangString>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    height: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    width: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    preview: Option<BoxedActivityStreamEntity>,
}
