use url::Url;
use chrono::{DateTime, Utc};

use crate::activitystream::MaybeOptional;
use crate::activitystream::base::{ActivityStreamBase, ActivityStreamBaseType, ActivityStreamBase_};
use crate::activitystream::content::ActivityStreamContent;

impl ActivityStreamBase_ for ActivityStreamObject {
    fn get_id(&self) -> &Option<Url> {
        &self.id
    }

    fn set_id(&mut self, id: Option<Url>) {
        self.id = id;
    }

    fn get_type(&self) -> ActivityStreamBaseType {
        ActivityStreamBaseType::Object
    }
}

pub trait ActivityStreamObject_ {
    fn get_attachment(&self) -> Option<Vec<ActivityStreamBase>>;
    fn set_attachment<T: MaybeOptional<Vec<ActivityStreamBase>>>(&mut self, attachment: T) -> Self;
    fn add_attachment(&mut self, attachment: ActivityStreamBase) -> Self;
    fn get_attributed_to(&self) -> Option<Vec<ActivityStreamBase>>;
    fn set_attributed_to<T: MaybeOptional<Vec<ActivityStreamBase>>>(&mut self, attributed_to: T) -> Self;
    fn add_attributed_to(&mut self, attributed_to: ActivityStreamBase) -> Self;
    fn get_audience(&self) -> Option<ActivityStreamBase>;
    fn set_audience<T: MaybeOptional<Vec<ActivityStreamBase>>>(&mut self, audience: T) -> Self;
    fn get_content(&self) -> Option<ActivityStreamContent>;
    fn set_content<U: Into<ActivityStreamContent> + Sized, T: MaybeOptional<U>>(&mut self, content: T) -> Self where ActivityStreamContent: From<T>;
    fn get_context(&self) -> Option<String>;
    fn set_context<U: Into<ActivityStreamContent> + Sized, T: MaybeOptional<U>>(&mut self, content: T) -> Self where ActivityStreamContent: From<T>;
    fn get_end_time(&self) -> Option<DateTime<Utc>>;
    fn set_end_time(&mut self, end_time: Option<DateTime<Utc>>) -> Self;
    fn get_generator(&self) -> Option<ActivityStreamBase>;
    fn set_generator<T: MaybeOptional<ActivityStreamBase>>(&mut self, generator: T) -> Self;
}

#[allow(non_snake_case)]
struct ActivityStreamObject {
    id: Option<Url>,
    attachment: Option<Vec<ActivityStreamBase>>,
    attributedTo: Option<Vec<ActivityStreamBase>>,
    audience: Option<ActivityStreamBase>, //The specs say "one or more" but example 69 is only a dict
    content: Option<ActivityStreamContent>,
    context: Option<String>,
    name: Option<ActivityStreamContent>,
    endTime: Option<DateTime<Utc>>,
    generator: Option<ActivityStreamBase>,
    /*
    icon: Option<ActivityStreamIcon>,
    image: Option<ActivityStreamImage>,
    inReplyTo: Option<ActivityStreamInReplyTo>,
    location: Option<ActivityStreamLocation>,
    preview: Option<ActivityStreamPreview>,
    published: Option<ActivityStreamPublished>,
    replies: Option<ActivityStreamReplies>,
    startTime: Option<ActivityStreamStartTime>,
    summary: Option<ActivityStreamSummary>,
    tag: Option<ActivityStreamTag>,
    updated: Option<ActivityStreamUpdated>,
    url: Option<ActivityStreamUrl>,
    to: Option<ActivityStreamTo>,
    bto: Option<ActivityStreamBto>,
    cc: Option<ActivityStreamCc>,
    bcc: Option<ActivityStreamBcc>,
    mediaType: Option<ActivityStreamMediaType>,
    duration: Option<ActivityStreamDuration>,*/
}

//INFO: BaseType
struct ActivityStreamLink {
    
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
struct ActivityStreamActivity {
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
struct ActivityStreamTransitiveActivity {
    /*id: Option<Url>,
    const r#type:ActivityStreamCoreType = ActivityStreamCoreType::TransitiveActivity,
    */
}

//Extends ActivityStreamObject
struct ActivitySteamCollection {
    /*id: Option<Url>,
    const r#type:ActivityStreamCoreType = ActivityStreamCoreType::Collection,
    totalItems: Option<ActivityStreamTotalItems>,
    current: Option<ActivityStreamCurrent>,
    first: Option<ActivityStreamFirst>,
    last: Option<ActivityStreamLast>,
    items: Option<ActivityStreamItems>,*/
}

struct ActivitySteamOrderedCollection {
    /*id: Option<Url>,
    const r#type:ActivityStreamCoreType = ActivityStreamCoreType::OrderedCollection,
    totalItems: Option<ActivityStreamTotalItems>,
    current: Option<ActivityStreamCurrent>,
    first: Option<ActivityStreamFirst>,
    last: Option<ActivityStreamLast>,
    //FIXME: The page says items, but the example says orderedItems
    items: Option<ActivityStreamItems>,*/
}

struct ActivitySteamCollectionPage {
    /*id: Option<Url>,
    const r#type:ActivityStreamCoreType = ActivityStreamCoreType::CollectionPage,
    partOf: Option<ActivityStreamPartOf>,
    next: Option<ActivityStreamNext>,
    prev: Option<ActivityStreamPrev>,*/
}


//FIXME: Not done
struct ActivitySteamOrderedCollectionPage {
    //id: Option<Url>,
    //const r#type:ActivityStreamCoreType = ActivityStreamCoreType::CollectionPage,
    //startIndex: Option<ActivityStreamStartIndex>,
}
