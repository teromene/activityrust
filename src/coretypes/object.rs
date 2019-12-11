use url::Url;
use chrono::{DateTime, Utc};

use crate::MaybeOptional;
use crate::coretypes::base::{ActivityStreamCoreType, ActivityStreamCoreElement, ActivityStreamCoreElementBoxed, ActivityStreamCoreElementsBoxed};
use crate::content::{ActivityStreamContent, ActivityStreamLinkableImage, ActivityStreamLinkableUrl};

impl ActivityStreamCoreElement for ActivityStreamCoreTypeObject {
    fn get_id(&self) -> &Option<Url> {
        &self.id
    }

    fn set_id(&mut self, id: Option<Url>) {
        self.id = id;
    }

    fn get_type(&self) -> ActivityStreamCoreType {
        ActivityStreamCoreType::Object
    }
}

pub trait ActivityStreamCoreTypeObject_ {
    fn get_attachment(&self) -> Option<ActivityStreamCoreElementsBoxed>;
    fn set_attachment<S: ActivityStreamCoreElement, T: MaybeOptional<Vec<S>>>(&mut self, attachment: T) -> Self;
    fn add_attachment<T: ActivityStreamCoreElement>(&mut self, attachment: T) -> Self;
    fn get_attributed_to<T: ActivityStreamCoreElement>(&self) -> Option<Vec<T>>;
    fn set_attributed_to<S: ActivityStreamCoreElement, T: MaybeOptional<Vec<S>>>(&mut self, attributed_to: T) -> Self;
    fn add_attributed_to<T: ActivityStreamCoreElement>(&mut self, attributed_to: T) -> Self;
    fn get_audience<T: ActivityStreamCoreElement>(&self) -> Option<T>;
    fn set_audience<S: ActivityStreamCoreElement, T: MaybeOptional<Vec<T>>>(&mut self, audience: T) -> Self;
    fn get_content<T: ActivityStreamCoreElement>(&self) -> Option<T>;
    fn set_content<S: ActivityStreamCoreElement, U: Into<S> + Sized, T: MaybeOptional<U>>(&mut self, content: T) -> Self where ActivityStreamContent: From<T>;
    fn get_context(&self) -> Option<String>;
    fn set_context<S: ActivityStreamCoreElement, U: Into<S> + Sized, T: MaybeOptional<U>>(&mut self, content: T) -> Self where ActivityStreamContent: From<T>;
    fn get_end_time(&self) -> Option<DateTime<Utc>>;
    fn set_end_time(&mut self, end_time: Option<DateTime<Utc>>) -> Self;
    fn get_generator<T: ActivityStreamCoreElement>(&self) -> Option<T>;
    fn set_generator<S: ActivityStreamCoreElement, T: MaybeOptional<S>>(&mut self, generator: T) -> Self;
    fn get_icon(&self) -> Option<ActivityStreamLinkableImage>;
    fn set_icon<T: MaybeOptional<ActivityStreamLinkableImage>>(&self, icon: T) -> Self;
    fn get_image(&self) -> Option<ActivityStreamLinkableImage>;
    fn set_image<T: MaybeOptional<ActivityStreamLinkableImage>>(&self, image: T) -> Self;
    fn get_in_reply_to(&self) -> Option<Url>;
    fn set_in_reply_to<T: MaybeOptional<Url>>(&mut self, in_reply_to: T) -> Self;
    fn get_location<T: ActivityStreamCoreElement>(&self) -> Option<T>;
    fn set_location<S: ActivityStreamCoreElement, T: MaybeOptional<S>>(&mut self, location: T) -> Self;
    fn get_preview<T: ActivityStreamCoreElement>(&self) -> Option<T>;
    fn set_preview<S: ActivityStreamCoreElement, T: MaybeOptional<S>>(&mut self, preview: T) -> Self;
    fn get_published(&self) -> Option<DateTime<Utc>>;
    fn set_published(&mut self, start_time: Option<DateTime<Utc>>) -> Self;
    fn get_replies(&self) -> Option<ActivityStreamCoreTypeCollection>;
    fn set_replies<T: MaybeOptional<ActivityStreamCoreTypeCollection>>(&mut self, replies: T) -> Self;
    fn get_start_time(&self) -> Option<DateTime<Utc>>;
    fn set_start_time(&mut self, start_time: Option<DateTime<Utc>>) -> Self;
    fn get_summary<T: ActivityStreamCoreElement>(&self) -> Option<T>;
    fn set_summary<S: ActivityStreamCoreElement, U: Into<S> + Sized, T: MaybeOptional<U>>(&mut self, summary: T) -> Self where ActivityStreamContent: From<T>;
    fn get_tag<T: ActivityStreamCoreElement>(&self) -> Option<Vec<T>>;
    fn set_tag<S: ActivityStreamCoreElement, T: MaybeOptional<Vec<S>>>(&mut self, tag: T) -> Self;
    fn get_updated(&self) -> Option<DateTime<Utc>>;
    fn set_updated(&mut self, updated: Option<DateTime<Utc>>) -> Self;
    fn get_url(&self) -> Option<ActivityStreamLinkableUrl>;
    fn set_url<T: MaybeOptional<ActivityStreamLinkableUrl>>(&mut self, url: T) -> Self;
    fn get_to<T: ActivityStreamCoreElement>(&self) -> Option<T>;
    fn set_to<S: ActivityStreamCoreElement, T: MaybeOptional<T>>(&mut self, to: T) -> Self;
    fn get_bto<T: ActivityStreamCoreElement>(&self) -> Option<T>;
    fn set_bto<S: ActivityStreamCoreElement, T: MaybeOptional<T>>(&mut self, bto: T) -> Self;
    fn get_cc<T: ActivityStreamCoreElement>(&self) -> Option<T>;
    fn set_cc<S: ActivityStreamCoreElement, T: MaybeOptional<T>>(&mut self, cc: T) -> Self;
    fn get_bcc<T: ActivityStreamCoreElement>(&self) -> Option<T>;
    fn set_bcc<S: ActivityStreamCoreElement, T: MaybeOptional<Vec<T>>>(&mut self, bcc: T) -> Self;
    fn get_media_type<T: ActivityStreamCoreElement>(&self) -> Option<T>;
    fn set_media_type<S: ActivityStreamCoreElement, U: Into<S> + Sized, T: MaybeOptional<U>>(&mut self, media_type: T) -> Self where ActivityStreamContent: From<T>;
    fn get_duration(&self) -> Option<String>;
    fn set_duration<T: MaybeOptional<String>>(&mut self, duration: T) -> Self;
}

impl ActivityStreamCoreTypeObject_ for ActivityStreamCoreTypeObject {
    fn get_attachment(&self) -> Option<Vec<ActivityStreamCoreElementBoxed>> {
        self.attachment
    }

    fn set_attachment<S: ActivityStreamCoreElement, T: MaybeOptional<Vec<S>>>(&mut self, attachment: T) -> Self {
        self.attachment = attachment.get_optional_boxed();
    }
}

#[allow(non_snake_case)]
pub struct ActivityStreamCoreTypeObject {
    id: Option<Url>,
    attachment: Option<ActivityStreamCoreElementsBoxed>,
    attributedTo: Option<Vec<ActivityStreamCoreElementBoxed>>,
    audience: Option<ActivityStreamCoreElementBoxed>, //The specs say "one or more" but example 69 is only a dict
    content: Option<ActivityStreamContent>,
    context: Option<String>,
    name: Option<ActivityStreamContent>,
    endTime: Option<DateTime<Utc>>,
    generator: Option<ActivityStreamCoreElementBoxed>,
    icon: Option<ActivityStreamLinkableImage>,
    image: Option<ActivityStreamLinkableImage>,
    inReplyTo: Option<Url>, //While the specs say "one or more entities", it is implemented in Mastodon and others as an URI
    location: Option<ActivityStreamCoreElementBoxed>,
    preview: Option<ActivityStreamCoreElementBoxed>,
    published: Option<DateTime<Utc>>,
    replies: Option<ActivityStreamCoreTypeCollection>,
    startTime: Option<DateTime<Utc>>,
    summary: Option<ActivityStreamContent>,
    tag: Option<Vec<ActivityStreamCoreElementBoxed>>,
    updated: Option<DateTime<Utc>>,
    url: Option<ActivityStreamLinkableUrl>,
    to: Option<ActivityStreamCoreElementBoxed>, //FIXME: Test examples
    bto: Option<ActivityStreamCoreElementBoxed>, //FIXME: Test examples
    cc: Option<ActivityStreamCoreElementBoxed>, //FIXME: Test examples
    bcc: Option<Vec<ActivityStreamCoreElementBoxed>>, //FIXME: Test examples
    mediaType: Option<ActivityStreamContent>,
    duration: Option<String>, //FIXME: Duration not implemented as a valid type
}

//INFO: BaseType
pub struct ActivityStreamLink {

    /*id: Option<Url>,
    const r#type:ActivityStreamCoreType = ActivityStreamCoreType::Link,
    href: Option<ActivityStreamHref>,
    rel: Option<ActivityStreamRel>,
    mediaType: Option<ActivityStreamMediaType>,
    name: Option<ActivityStreamName>,
    hreflang: Option<ActivityStreamHreflang>,
    height: Option<Option<ActivityStreamHeight>,
    width: Option<ActivityStreamWidth>,
    preview: Option<ActivityStreamPreview>*/
}

//Extends Object
pub struct ActivityStreamActivity {
    /*id: Option<Url>,
    const r#type:ActivityStreamCoreType = ActivityStreamCoreType::Activity,
    actor: Option<Vec<ActivityStreamActor>,
    object: Option<ActivityStreamObject>,
    target: Option<ActivityStreamTarget>,
    result: Option<ActivityStreamResult>,
    origin: Option<ActivityStreamOrigin>,
    instrument: Option<ActivityStreamInstrument>,*/
}

//Extends ActivityStreamActivity, without access to the object type
pub struct ActivityStreamTransitiveActivity {
    /*id: Option<Url>,
    const r#type:ActivityStreamCoreType = ActivityStreamCoreType::TransitiveActivity,
    */
}

//Extends ActivityStreamObject
pub struct ActivityStreamCoreTypeCollection {
    /*id: Option<Url>,
    const r#type:ActivityStreamCoreType = ActivityStreamCoreType::Collection,
    totalItems: Option<ActivityStreamTotalItems>,
    current: Option<ActivityStreamCurrent>,
    first: Option<ActivityStreamFirst>,
    last: Option<ActivityStreamLast>,
    items: Option<ActivityStreamItems>,*/
}

pub struct ActivitySteamOrderedCollection {
    /*id: Option<Url>,
    const r#type:ActivityStreamCoreType = ActivityStreamCoreType::OrderedCollection,
    totalItems: Option<ActivityStreamTotalItems>,
    current: Option<ActivityStreamCurrent>,
    first: Option<ActivityStreamFirst>,
    last: Option<ActivityStreamLast>,
    //FIXME: The page says items, but the example says orderedItems
    items: Option<ActivityStreamItems>,*/
}

pub struct ActivitySteamCollectionPage {
    /*id: Option<Url>,
    const r#type:ActivityStreamCoreType = ActivityStreamCoreType::CollectionPage,
    partOf: Option<ActivityStreamPartOf>,
    next: Option<ActivityStreamNext>,
    prev: Option<ActivityStreamPrev>,*/
}


//FIXME: Not done
pub struct ActivitySteamOrderedCollectionPage {
    //id: Option<Url>,
    //const r#type:ActivityStreamCoreType = ActivityStreamCoreType::CollectionPage,
    //startIndex: Option<ActivityStreamStartIndex>,
}
