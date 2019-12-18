use crate::entities::activity::ActivityStreamActivity;
use crate::entities::link::ActivityStreamLink;
use crate::entities::entity::{
    ActivityStreamEntity, ActivityStreamEntityType, BoxedActivityStreamEntity,
};
use serde::{Deserialize, Serialize};

use crate::content::*;
use crate::traits::properties::*;
use crate::MaybeOptional;
use ambassador::Delegate;
use url::Url;

generate_basics!(ActivityStreamMention, ActivityStreamEntityType::Mention);

#[derive(Debug, Default, Delegate, Serialize, Deserialize, PartialEq)]
#[delegate(ActivityStreamLinkProperties, target = "_base")]
pub struct ActivityStreamMention {
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[serde(deserialize_with = "ActivityStreamMention::deserialize_type")]
    r#type: Option<ActivityStreamEntityType>,
    #[serde(flatten)]
    _base: ActivityStreamLink,
}
