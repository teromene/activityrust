use chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize, Serializer};
use ambassador::delegatable_trait;
use url::Url;

use crate::content::{
    ActivityStreamMultilangString, ActivityStreamLinkableImage, ActivityStreamLinkableUrl,
};
use crate::entities::entity::{ActivityStreamEntityProperties, ActivityStreamEntityType, ActivityStreamEntity, BoxedActivityStreamEntity};
use crate::MaybeOptional;

impl ActivityStreamEntityProperties for ActivityStreamObject {
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

}

#[delegatable_trait]
pub trait ActivityStreamObjectProperties {
    fn get_attachments(&self) -> &Option<Vec<ActivityStreamEntity>>;
    fn set_attachments<S: ActivityStreamObjectProperties, T: MaybeOptional<Vec<S>>>(
        &mut self,
        attachment: T,
    )
    where
        ActivityStreamEntity: From<S>;
    fn add_attachment<S: ActivityStreamObjectProperties, T: MaybeOptional<S>>(
        &mut self,
        attachment: T,
    )
    where
        ActivityStreamEntity: From<S>;

    fn get_attributed_to(&self) -> &Option<Vec<ActivityStreamEntity>>;
    fn set_attributed_to<S: ActivityStreamObjectProperties, T: MaybeOptional<Vec<S>>>(
        &mut self,
        attributed_to: T,
    )
    where
        ActivityStreamEntity: From<S>;

    fn add_attributed_to<S: ActivityStreamObjectProperties, T: MaybeOptional<S>>(
        &mut self,
        attributed_to: T,
    )
    where
        ActivityStreamEntity: From<S>;

    fn get_audience(&self) -> &Option<BoxedActivityStreamEntity>;
    fn set_audience<S: ActivityStreamObjectProperties, T: MaybeOptional<S>>(&mut self, audience: T) where ActivityStreamEntity: From<S>;
    fn get_content(&self) -> &Option<ActivityStreamMultilangString>;
    fn set_content<S, T: MaybeOptional<S>>(&mut self, content: T) where ActivityStreamMultilangString: From<S>;
    fn get_context(&self) -> &Option<ActivityStreamMultilangString>;
    fn set_context<S, T: MaybeOptional<S>>(&mut self, context: T) where ActivityStreamMultilangString: From<S>;
    fn get_end_time(&self) -> &Option<DateTime<Utc>>;
    fn set_end_time<T: MaybeOptional<DateTime<Utc>>>(&mut self, end_time: T);
    fn get_generator(&self) -> &Option<BoxedActivityStreamEntity>;
    fn set_generator<S: ActivityStreamObjectProperties, T: MaybeOptional<S>>(&mut self, generator: T) where ActivityStreamEntity: From<S>;
    fn get_icon(&self) -> &Option<ActivityStreamLinkableImage>;
    fn set_icon<S, T: MaybeOptional<S>>(&mut self, icon: T) where ActivityStreamLinkableImage: From<S>;
    fn get_image(&self) -> &Option<ActivityStreamLinkableImage>;
    fn set_image<S, T: MaybeOptional<S>>(&mut self, image: T) where ActivityStreamLinkableImage: From<S>;
    fn get_in_reply_to(&self) -> &Option<Url>;
    fn set_in_reply_to<T: MaybeOptional<Url>>(&mut self, in_reply_to: T);
    fn get_location(&self) -> &Option<BoxedActivityStreamEntity>;
    fn set_location<S: ActivityStreamObjectProperties, T: MaybeOptional<S>>(&mut self, location: T) where ActivityStreamEntity: From<S>;
    fn get_preview(&self) -> &Option<BoxedActivityStreamEntity>;
    fn set_preview<S: ActivityStreamObjectProperties, T: MaybeOptional<S>>(&mut self, preview: T) where ActivityStreamEntity: From<S>;
    fn get_published(&self) -> &Option<DateTime<Utc>>;
    fn set_published<T: MaybeOptional<DateTime<Utc>>>(&mut self, published: T);
    fn get_replies(&self) -> &Option<ActivityStreamCollection>;
    fn set_replies<T: MaybeOptional<ActivityStreamCollection>>(&mut self, replies: T);
    fn get_start_time(&self) -> &Option<DateTime<Utc>>;
    fn set_start_time<T: MaybeOptional<DateTime<Utc>>>(&mut self, start_time: T);
    fn get_summary(&self) -> &Option<ActivityStreamMultilangString>;
    fn set_summary<S, T: MaybeOptional<S>>(&mut self, summary: T) where ActivityStreamMultilangString: From<S>;
    fn get_tags(&self) -> &Option<Vec<ActivityStreamEntity>>;
    fn set_tags<S: ActivityStreamObjectProperties, T: MaybeOptional<Vec<S>>>(&mut self, attachment: T) where ActivityStreamEntity: From<S>;
    fn add_tag<S: ActivityStreamObjectProperties, T: MaybeOptional<S>>(&mut self, attachment: T) where ActivityStreamEntity: From<S>;
    fn get_updated(&self) -> &Option<DateTime<Utc>>;
    fn set_updated<T: MaybeOptional<DateTime<Utc>>>(&mut self, updated: T);
    fn get_url(&self) -> &Option<ActivityStreamLinkableUrl>;
    fn set_url<S, T: MaybeOptional<S>>(&mut self, url: T) where ActivityStreamLinkableUrl: From<S>;
    fn get_to(&self) -> &Option<BoxedActivityStreamEntity>;
    fn set_to<S: ActivityStreamObjectProperties, T: MaybeOptional<S>>(&mut self, to: T) where ActivityStreamEntity: From<S>;
    fn get_bto(&self) -> &Option<BoxedActivityStreamEntity>;
    fn set_bto<S: ActivityStreamObjectProperties, T: MaybeOptional<S>>(&mut self, to: T) where ActivityStreamEntity: From<S>;
    fn get_cc(&self) -> &Option<BoxedActivityStreamEntity>;
    fn set_cc<S: ActivityStreamObjectProperties, T: MaybeOptional<S>>(&mut self, cc: T) where ActivityStreamEntity: From<S>;
    fn get_bcc(&self) -> &Option<Vec<ActivityStreamEntity>>;
    fn set_bcc<S: ActivityStreamObjectProperties, T: MaybeOptional<Vec<S>>>(&mut self, attachment: T) where ActivityStreamEntity: From<S>;
    fn add_bcc<S: ActivityStreamObjectProperties, T: MaybeOptional<S>>(&mut self, attachment: T) where ActivityStreamEntity: From<S>;
    fn get_media_type(&self) -> &Option<String>;
    fn set_media_type<T: MaybeOptional<String>>(&mut self, media_type: T);
    fn get_duration(&self) -> &Option<String>;
    fn set_duration<T: MaybeOptional<String>>(&mut self, duration: T);
}

impl ActivityStreamObjectProperties for ActivityStreamObject {
    fn get_attachments(&self) -> &Option<Vec<ActivityStreamEntity>> {
        &self.attachment
    }

    fn set_attachments<S: ActivityStreamObjectProperties, T: MaybeOptional<Vec<S>>>(
        &mut self,
        attachment: T,
    )
    where
        ActivityStreamEntity: From<S>,
    {
        if let Some(attachment) = attachment.get_optional() {
            self.attachment = Some(
                attachment
                    .into_iter()
                    .map(ActivityStreamEntity::from)
                    .collect(),
            );
        }
    }

    fn add_attachment<S: ActivityStreamObjectProperties, T: MaybeOptional<S>>(
        &mut self,
        attachment: T,
    )
    where
        ActivityStreamEntity: From<S>,
    {
        if let Some(attachment) = attachment.get_optional() {
            if self.attachment.is_none() {
                self.attachment = Some(Vec::new());
            }

            if let Some(ref mut attachment_internal) = self.attachment {
                attachment_internal.push(ActivityStreamEntity::from(attachment));
            }
        }
    }

    fn get_attributed_to(&self) -> &Option<Vec<ActivityStreamEntity>> {
      &self.attributedTo
    }

    fn set_attributed_to<S: ActivityStreamObjectProperties, T: MaybeOptional<Vec<S>>>(
        &mut self,
        attributed_to: T,
    )
    where
        ActivityStreamEntity: From<S>,
    {
        if let Some(attributed_to) = attributed_to.get_optional() {
            self.attributedTo = Some(
                attributed_to
                    .into_iter()
                    .map(ActivityStreamEntity::from)
                    .collect(),
            );
        }
    }

    fn add_attributed_to<S: ActivityStreamObjectProperties, T: MaybeOptional<S>>(
        &mut self,
        attributed_to: T,
    )
    where
        ActivityStreamEntity: From<S>,
    {
        if let Some(attributed_to) = attributed_to.get_optional() {
            if self.attributedTo.is_none() {
                self.attributedTo = Some(Vec::new());
            }

            if let Some(ref mut attributed_to_internal) = self.attributedTo {
                attributed_to_internal.push(ActivityStreamEntity::from(attributed_to));
            }
        }

    }

    fn get_audience(&self) -> &Option<BoxedActivityStreamEntity> {
      &self.audience
    }

    fn set_audience<S: ActivityStreamObjectProperties, T: MaybeOptional<S>>(&mut self, audience: T) where ActivityStreamEntity: From<S> {
      if let Some(audience) = audience.get_optional() {
        self.audience = Some(Box::new(ActivityStreamEntity::from(audience)));
      }
    }

    fn get_content(&self) -> &Option<ActivityStreamMultilangString> {
      &self.content
    }

    fn set_content<S, T: MaybeOptional<S>>(&mut self, content: T) where ActivityStreamMultilangString: From<S> {
      if let Some(content) = content.get_optional() {
        self.content = Some(ActivityStreamMultilangString::from(content));
      }
    }

    fn get_context(&self) -> &Option<ActivityStreamMultilangString> {
      &self.context
    }

    fn set_context<S, T: MaybeOptional<S>>(&mut self, context: T) where ActivityStreamMultilangString: From<S> {
      if let Some(context) = context.get_optional() {
        self.context = Some(ActivityStreamMultilangString::from(context));
      }
    }

    fn get_end_time(&self) -> &Option<DateTime<Utc>> {
      &self.endTime
    }

    fn set_end_time<T: MaybeOptional<DateTime<Utc>>>(&mut self, end_time: T) {
      self.endTime = end_time.get_optional();
    }

    fn get_generator(&self) -> &Option<BoxedActivityStreamEntity> {
      &self.generator
    }

    fn set_generator<S: ActivityStreamObjectProperties, T: MaybeOptional<S>>(&mut self, generator: T) where ActivityStreamEntity: From<S> {
      if let Some(generator) = generator.get_optional() {
        self.generator = Some(Box::new(ActivityStreamEntity::from(generator)));
      }
    }

    fn get_icon(&self) -> &Option<ActivityStreamLinkableImage> {
      &self.icon
    }

    fn set_icon<S, T: MaybeOptional<S>>(&mut self, icon: T) where ActivityStreamLinkableImage: From<S> {
      if let Some(icon) = icon.get_optional() {
        self.icon = Some(ActivityStreamLinkableImage::from(icon));
      }
    }

    fn get_image(&self) -> &Option<ActivityStreamLinkableImage> {
      &self.image
    }

    fn set_image<S, T: MaybeOptional<S>>(&mut self, image: T) where ActivityStreamLinkableImage: From<S> {
      if let Some(image) = image.get_optional() {
        self.image = Some(ActivityStreamLinkableImage::from(image));
      }
    }

    fn get_in_reply_to(&self) -> &Option<Url> {
      &self.inReplyTo
    }

    fn set_in_reply_to<T: MaybeOptional<Url>>(&mut self, in_reply_to: T) {
      self.inReplyTo = in_reply_to.get_optional();
    }

    fn get_location(&self) -> &Option<BoxedActivityStreamEntity> {
      &self.location
    }

    fn set_location<S: ActivityStreamObjectProperties, T: MaybeOptional<S>>(&mut self, location: T) where ActivityStreamEntity: From<S> {
      if let Some(location) = location.get_optional() {
        self.location = Some(Box::new(ActivityStreamEntity::from(location)));
      }
    }

    fn get_preview(&self) -> &Option<BoxedActivityStreamEntity> {
      &self.preview
    }

    fn set_preview<S: ActivityStreamObjectProperties, T: MaybeOptional<S>>(&mut self, preview: T) where ActivityStreamEntity: From<S> {
      if let Some(preview) = preview.get_optional() {
        self.preview = Some(Box::new(ActivityStreamEntity::from(preview)));
      }
    }

    fn get_published(&self) -> &Option<DateTime<Utc>> {
      &self.published
    }

    fn set_published<T: MaybeOptional<DateTime<Utc>>>(&mut self, published: T) {
      self.published = published.get_optional();
    }

    fn get_replies(&self) -> &Option<ActivityStreamCollection> {
      &self.replies
    }

    fn set_replies<T: MaybeOptional<ActivityStreamCollection>>(&mut self, replies: T) {
      self.replies = replies.get_optional();
    }

    fn get_start_time(&self) -> &Option<DateTime<Utc>> {
      &self.startTime
    }

    fn set_start_time<T: MaybeOptional<DateTime<Utc>>>(&mut self, start_time: T) {
      self.startTime = start_time.get_optional();
    }

    fn get_summary(&self) -> &Option<ActivityStreamMultilangString> {
      &self.summary
    }

    fn set_summary<S, T: MaybeOptional<S>>(&mut self, summary: T) where ActivityStreamMultilangString: From<S> {
      if let Some(summary) = summary.get_optional() {
        self.summary = Some(ActivityStreamMultilangString::from(summary));
      }
    }

    fn get_tags(&self) -> &Option<Vec<ActivityStreamEntity>> {
        &self.tag
    }

    fn set_tags<S: ActivityStreamObjectProperties, T: MaybeOptional<Vec<S>>>(
        &mut self,
        tag: T,
    )
    where
        ActivityStreamEntity: From<S>,
    {
        if let Some(tag) = tag.get_optional() {
            self.tag = Some(
                tag
                    .into_iter()
                    .map(ActivityStreamEntity::from)
                    .collect(),
            );
        }
    }

    fn add_tag<S: ActivityStreamObjectProperties, T: MaybeOptional<S>>(
        &mut self,
        tag: T,
    )
    where
        ActivityStreamEntity: From<S>,
    {
        if let Some(tag) = tag.get_optional() {
            if self.tag.is_none() {
                self.tag = Some(Vec::new());
            }

            if let Some(ref mut tag_internal) = self.tag {
                tag_internal.push(ActivityStreamEntity::from(tag));
            }
        }
    }

    fn get_updated(&self) -> &Option<DateTime<Utc>> {
      &self.updated
    }

    fn set_updated<T: MaybeOptional<DateTime<Utc>>>(&mut self, updated: T) {
      self.updated = updated.get_optional();
    }

    fn get_url(&self) -> &Option<ActivityStreamLinkableUrl> {
      &self.url
    }

    fn set_url<S, T: MaybeOptional<S>>(&mut self, url: T) where ActivityStreamLinkableUrl: From<S> {
      if let Some(url) = url.get_optional() {
        self.url = Some(ActivityStreamLinkableUrl::from(url));
      }
    }

    fn get_to(&self) -> &Option<BoxedActivityStreamEntity> {
      &self.to
    }

    fn set_to<S: ActivityStreamObjectProperties, T: MaybeOptional<S>>(&mut self, to: T) where ActivityStreamEntity: From<S> {
      if let Some(to) = to.get_optional() {
        self.to = Some(Box::new(ActivityStreamEntity::from(to)));
      }
    }

    fn get_bto(&self) -> &Option<BoxedActivityStreamEntity> {
      &self.bto
    }

    fn set_bto<S: ActivityStreamObjectProperties, T: MaybeOptional<S>>(&mut self, bto: T) where ActivityStreamEntity: From<S> {
      if let Some(bto) = bto.get_optional() {
        self.bto = Some(Box::new(ActivityStreamEntity::from(bto)));
      }
    }

    fn get_cc(&self) -> &Option<BoxedActivityStreamEntity> {
      &self.cc
    }

    fn set_cc<S: ActivityStreamObjectProperties, T: MaybeOptional<S>>(&mut self, cc: T) where ActivityStreamEntity: From<S> {
      if let Some(cc) = cc.get_optional() {
        self.cc = Some(Box::new(ActivityStreamEntity::from(cc)));
      }
    }

    fn get_bcc(&self) -> &Option<Vec<ActivityStreamEntity>> {
        &self.bcc
    }

    fn set_bcc<S: ActivityStreamObjectProperties, T: MaybeOptional<Vec<S>>>(
        &mut self,
        bcc: T,
    )
    where
        ActivityStreamEntity: From<S>,
    {
        if let Some(bcc) = bcc.get_optional() {
            self.bcc = Some(
                bcc
                    .into_iter()
                    .map(ActivityStreamEntity::from)
                    .collect(),
            );
        }
    }

    fn add_bcc<S: ActivityStreamObjectProperties, T: MaybeOptional<S>>(
        &mut self,
        bcc: T,
    )
    where
        ActivityStreamEntity: From<S>,
    {
        if let Some(bcc) = bcc.get_optional() {
            if self.bcc.is_none() {
                self.bcc = Some(Vec::new());
            }

            if let Some(ref mut bcc_internal) = self.bcc {
                bcc_internal.push(ActivityStreamEntity::from(bcc));
            }
        }
    }

    fn get_media_type(&self) -> &Option<String> {
      &self.mediaType
    }

    fn set_media_type<T: MaybeOptional<String>>(&mut self, media_type: T) {
      self.mediaType = media_type.get_optional();
    }

    fn get_duration(&self) -> &Option<String> {
      &self.duration
    }

    fn set_duration<T: MaybeOptional<String>>(&mut self, duration: T) {
      self.duration = duration.get_optional();
    }

}

impl ActivityStreamObject {

  pub fn create() -> Self {
    let mut new_object = ActivityStreamObject::default();
    new_object.set_type(ActivityStreamEntityType::Object);
    new_object
  }

}

#[allow(non_snake_case)]
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct ActivityStreamObject {
    #[serde(skip_serializing_if = "Option::is_none", default)]
    id: Option<Url>,
    r#type: ActivityStreamEntityType,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    attachment: Option<Vec<ActivityStreamEntity>>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    attributedTo: Option<Vec<ActivityStreamEntity>>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    audience: Option<BoxedActivityStreamEntity>, //The specs say "one or more" but example 69 is only a dict
    #[serde(skip_serializing_if = "Option::is_none", default)]
    content: Option<ActivityStreamMultilangString>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    context: Option<ActivityStreamMultilangString>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    name: Option<ActivityStreamMultilangString>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    endTime: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    generator: Option<BoxedActivityStreamEntity>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    icon: Option<ActivityStreamLinkableImage>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    image: Option<ActivityStreamLinkableImage>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    inReplyTo: Option<Url>, //While the specs say "one or more entities", it is implemented in Mastodon and others as an URI
    #[serde(skip_serializing_if = "Option::is_none", default)]
    location: Option<BoxedActivityStreamEntity>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    preview: Option<BoxedActivityStreamEntity>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    published: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    replies: Option<ActivityStreamCollection>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    startTime: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    summary: Option<ActivityStreamMultilangString>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    tag: Option<Vec<ActivityStreamEntity>>, //Singular in the specs
    #[serde(skip_serializing_if = "Option::is_none", default)]
    updated: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    url: Option<ActivityStreamLinkableUrl>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    to: Option<BoxedActivityStreamEntity>, //FIXME: Test examples
    #[serde(skip_serializing_if = "Option::is_none", default)]
    bto: Option<BoxedActivityStreamEntity>, //FIXME: Test examples
    #[serde(skip_serializing_if = "Option::is_none", default)]
    cc: Option<BoxedActivityStreamEntity>, //FIXME: Test examples
    #[serde(skip_serializing_if = "Option::is_none", default)]
    bcc: Option<Vec<ActivityStreamEntity>>, //FIXME: Test examples
    #[serde(skip_serializing_if = "Option::is_none", default)]
    mediaType: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    duration: Option<String>, //FIXME: Duration not implemented as a valid type
}

//Extends Object
#[derive(Debug, Serialize, Deserialize)]
pub struct ActivityStreamActivity {
    /*id: Option<Url>,
const r#type:ActivityStreamCoreType = ActivityStreamCoreType::Activity,
actor: Option<Vec<ActivityStreamActor>,
object: Option<ActivityStreamObject>,
target: Option<ActivityStreamTarget>,
result: Option<ActivityStreamResult>,
origin: Option<ActivityStreamOrigin>,
instrument: Option<ActivityStreamInstrument>,*/}

//Extends ActivityStreamActivity, without access to the object type
#[derive(Debug, Serialize, Deserialize)]
pub struct ActivityStreamTransitiveActivity {
    /*id: Option<Url>,
const r#type:ActivityStreamCoreType = ActivityStreamCoreType::TransitiveActivity,
*/}

//Extends ActivityStreamObject
#[derive(Debug, Serialize, Deserialize)]
pub struct ActivityStreamCollection {
    /*id: Option<Url>,
const r#type:ActivityStreamCoreType = ActivityStreamCoreType::Collection,
totalItems: Option<ActivityStreamTotalItems>,
current: Option<ActivityStreamCurrent>,
first: Option<ActivityStreamFirst>,
last: Option<ActivityStreamLast>,
items: Option<ActivityStreamItems>,*/}

pub struct ActivitySteamOrderedCollection {
    /*id: Option<Url>,
const r#type:ActivityStreamCoreType = ActivityStreamCoreType::OrderedCollection,
totalItems: Option<ActivityStreamTotalItems>,
current: Option<ActivityStreamCurrent>,
first: Option<ActivityStreamFirst>,
last: Option<ActivityStreamLast>,
//FIXME: The page says items, but the example says orderedItems
items: Option<ActivityStreamItems>,*/}

#[derive(Debug, Serialize, Deserialize)]
pub struct ActivitySteamCollectionPage {
    /*id: Option<Url>,
const r#type:ActivityStreamCoreType = ActivityStreamCoreType::CollectionPage,
partOf: Option<ActivityStreamPartOf>,
next: Option<ActivityStreamNext>,
prev: Option<ActivityStreamPrev>,*/}

//FIXME: Not done
#[derive(Debug, Serialize, Deserialize)]
pub struct ActivitySteamOrderedCollectionPage {
    //id: Option<Url>,
//const r#type:ActivityStreamCoreType = ActivityStreamCoreType::CollectionPage,
//startIndex: Option<ActivityStreamStartIndex>,
}
