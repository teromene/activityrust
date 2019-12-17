use serde::{Serialize, Deserialize};
use crate::traits::properties::*;
use crate::entities::entity::{ActivityStreamEntityType, BoxedActivityStreamEntity, ActivityStreamEntity};
use crate::content::ActivityStreamMultilangString;
use crate::{MaybeOptional, OneOrMultiple};
use url::Url;

impl ActivityStreamEntityProperties for ActivityStreamLink {
    fn get_id(&self) -> &Option<Url> {
        &self.id
    }

    fn set_id<T: MaybeOptional<Url>>(&mut self, id: T) {
        self.id = id.get_optional();
    }

    fn get_type(&self) -> &ActivityStreamEntityType {
      if let Some(r#type) = &self.r#type {
        r#type
      } else {
        panic!("The entity type is null. This should not happen. Make sure that the object is created with the \"create\" method and not the \"default\" method");
      }
    }

    fn set_type(&mut self, r#type: ActivityStreamEntityType) {
      self.r#type = Some(r#type);
    }

    fn register_context(&mut self, new_context: Url) {
        if let Some(ref mut context) = self.context {
            context.append(new_context);
        } else {
            self.context = Some(OneOrMultiple::Element(new_context));
        }
    }

}

pub trait ActivityStreamLinkProperties {
    fn get_href(&self) -> &Option<Url>;
    fn set_href<T: MaybeOptional<Url>>(&mut self, href: T);
    fn get_hreflang(&self) -> &Option<String>;
    fn set_hreflang<T: MaybeOptional<String>>(&mut self, hreflang: T);
    fn get_media_type(&self) -> &Option<String>;
    fn set_media_type<T: MaybeOptional<String>>(&mut self, media_type: T);
    fn get_name(&self) -> &Option<ActivityStreamMultilangString>;
    fn set_name<S, T: MaybeOptional<S>>(&mut self, name: T) where ActivityStreamMultilangString: From<S>;
    fn get_height(&self) -> &Option<usize>;
    fn set_height<T: MaybeOptional<usize>>(&mut self, height: T);
    fn get_width(&self) -> &Option<usize>;
    fn set_width<T: MaybeOptional<usize>>(&mut self, width: T);
    fn get_preview(&self) -> &Option<BoxedActivityStreamEntity>;
    fn set_preview<S: ActivityStreamEntityProperties, T: MaybeOptional<S>>(&mut self, audience: T) where ActivityStreamEntity: From<S>;
}

impl ActivityStreamLinkProperties for ActivityStreamLink {
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

    fn set_name<S, T: MaybeOptional<S>>(&mut self, name: T) where ActivityStreamMultilangString: From<S> {
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

    fn set_preview<S: ActivityStreamEntityProperties, T: MaybeOptional<S>>(&mut self, preview: T) where ActivityStreamEntity: From<S> {
      if let Some(preview) = preview.get_optional() {
        self.preview = Some(Box::new(ActivityStreamEntity::from(preview)));
      }
    }
}

impl ActivityStreamLink {

  pub fn create() -> Self {

    let link_context = Url::parse("https://www.w3.org/ns/activitystreams").unwrap();

    let mut new_link = ActivityStreamLink::default();
    new_link.register_context(link_context);
    new_link.set_type(ActivityStreamEntityType::Link);
    new_link

  }

}

#[allow(non_snake_case)]
#[derive(Debug, Default, Serialize, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct ActivityStreamLink {
    #[serde(skip_serializing_if = "Option::is_none", default)]
    id: Option<Url>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
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
