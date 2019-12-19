use crate::entities::activity::ActivityStreamActivity;
use crate::entities::activitytypes::*;
use crate::entities::actortypes::*;
use crate::entities::collection::ActivityStreamCollection;
use crate::entities::collectionpage::ActivityStreamCollectionPage;
use crate::entities::intransitiveactivity::ActivityStreamIntransitiveActivity;
use crate::entities::link::ActivityStreamLink;
use crate::entities::linktypes::ActivityStreamMention;
use crate::entities::object::ActivityStreamObject;
use crate::entities::objecttypes::*;
use crate::entities::orderedcollection::ActivityStreamOrderedCollection;
use crate::entities::orderedcollectionpage::ActivityStreamOrderedCollectionPage;
use serde::{Deserialize, Serialize};
use url::Url;

//// Enum containing any valid ActivityStream Entity.
#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum ActivityStreamEntity {
    StreamLink(ActivityStreamLink),
    ActivityStreamActivity(ActivityStreamActivity),
    ActivityStreamIntransitiveActivity(ActivityStreamIntransitiveActivity),
    ActivityStreamCollection(ActivityStreamCollection),
    ActivityStreamOrderedCollection(ActivityStreamOrderedCollection),
    ActivityStreamCollectionPage(ActivityStreamCollectionPage),
    ActivityStreamOrderedCollectionPage(ActivityStreamOrderedCollectionPage),
    ActivityStreamAccept(ActivityStreamAccept),
    ActivityStreamAdd(ActivityStreamAdd),
    ActivityStreamAnnounce(ActivityStreamAnnounce),
    ActivityStreamArrive(ActivityStreamArrive),
    ActivityStreamBlock(ActivityStreamBlock),
    ActivityStreamCreate(ActivityStreamCreate),
    ActivityStreamDelete(ActivityStreamDelete),
    ActivityStreamDislike(ActivityStreamDislike),
    ActivityStreamFlag(ActivityStreamFlag),
    ActivityStreamFollow(ActivityStreamFollow),
    ActivityStreamIgnore(ActivityStreamIgnore),
    ActivityStreamInvite(ActivityStreamInvite),
    ActivityStreamJoin(ActivityStreamJoin),
    ActivityStreamLeave(ActivityStreamLeave),
    ActivityStreamLike(ActivityStreamLike),
    ActivityStreamListen(ActivityStreamListen),
    ActivityStreamMove(ActivityStreamMove),
    ActivityStreamOffer(ActivityStreamOffer),
    ActivityStreamQuestion(ActivityStreamQuestion),
    ActivityStreamReject(ActivityStreamReject),
    ActivityStreamRead(ActivityStreamRead),
    ActivityStreamRemove(ActivityStreamRemove),
    ActivityStreamTentativeReject(ActivityStreamTentativeReject),
    ActivityStreamTentativeAccept(ActivityStreamTentativeAccept),
    ActivityStreamTravel(ActivityStreamTravel),
    ActivityStreamUndo(ActivityStreamUndo),
    ActivityStreamUpdate(ActivityStreamUpdate),
    ActivityStreamView(ActivityStreamView),
    ActivityStreamApplication(ActivityStreamApplication),
    ActivityStreamGroup(ActivityStreamGroup),
    ActivityStreamOrganization(ActivityStreamOrganization),
    ActivityStreamPerson(ActivityStreamPerson),
    ActivityStreamService(ActivityStreamService),
    ActivityStreamArticle(ActivityStreamArticle),
    ActivityStreamAudio(ActivityStreamAudio),
    ActivityStreamDocument(ActivityStreamDocument),
    ActivityStreamEvent(ActivityStreamEvent),
    ActivityStreamImage(ActivityStreamImage),
    ActivityStreamNote(ActivityStreamNote),
    ActivityStreamPage(ActivityStreamPage),
    ActivityStreamPlace(ActivityStreamPlace),
    ActivityStreamProfile(ActivityStreamProfile),
    ActivityStreamRelationship(ActivityStreamRelationship),
    ActivityStreamTombstone(ActivityStreamTombstone),
    ActivityStreamVideo(ActivityStreamVideo),
    ActivityStreamMention(ActivityStreamMention),
    Object(ActivityStreamObject), //The object is at the back, and does not check type to get fallbacks
    Vec(Vec<ActivityStreamEntity>),
    #[serde(deserialize_with = "deserialize_link")]
    Link(Url),
}

fn deserialize_link<'de, D>(des: D) -> Result<Url, D::Error>
where
    D: serde::Deserializer<'de>,
{
    if let Ok(ax) = String::deserialize(des) {
        if let Ok(ax) = Url::parse(&ax) {
            let ax: Url = ax;
            return Ok(ax);
        }
    }
    Err(serde::de::Error::custom("Not an URL !"))
}

//// A Boxed version of an ActivityStreamEntity
pub type BoxedActivityStreamEntity = Box<ActivityStreamEntity>;

//// This enum describes the ActivityStream core types as defined in [Section 2 of the specification](https://www.w3.org/TR/activitystreams-vocabulary/#types)
#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq)]
pub enum ActivityStreamEntityType {
    Object,
    Link,
    Activity,
    IntransitiveActivity,
    Collection,
    OrderedCollection,
    CollectionPage,
    OrderedCollectionPage,
    //Types for Activity
    Accept,
    Add,
    Announce,
    Arrive,
    Block,
    Create,
    Delete,
    Dislike,
    Flag,
    Follow,
    Ignore,
    Invite,
    Join,
    Leave,
    Like,
    Listen,
    Move,
    Offer,
    Question,
    Reject,
    Read,
    Remove,
    TentativeReject,
    TentativeAccept,
    Travel,
    Undo,
    Update,
    View,
    //Types for Actor
    Application,
    Group,
    Organization,
    Person,
    Service,
    //Types for Object
    Article,
    Audio,
    Document,
    Event,
    Image,
    Note,
    Page,
    Place,
    Profile,
    Relationship,
    Tombstone,
    Video,
    //Types for Link
    Mention,
    #[serde(other)]
    Other,
}

impl Default for ActivityStreamEntityType {
    fn default() -> Self {
        ActivityStreamEntityType::Object
    }
}
