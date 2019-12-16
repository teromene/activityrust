#[macro_use]
pub mod properties {

    use ambassador::delegatable_trait;
    use crate::{MaybeOptional, OneOrMultiple};
    use crate::entities::entity::{ActivityStreamEntity, BoxedActivityStreamEntity, ActivityStreamEntityType};
    use crate::entities::object::ActivityStreamCollection;
    use std::fmt::Debug;
    use serde::{Serialize, Deserialize};
    use crate::content::*;
    use chrono::{DateTime, Utc};
    use url::Url;

    #[delegatable_trait]
    pub trait ActivityStreamObjectProperties {
        fn get_attachments(&self) -> &Option<Vec<ActivityStreamEntity>>;
        fn set_attachments<S: ActivityStreamEntityProperties, T: MaybeOptional<Vec<S>>>(
            &mut self,
            attachment: T,
        )
        where
            ActivityStreamEntity: From<S>;
        fn add_attachment<S: ActivityStreamEntityProperties, T: MaybeOptional<S>>(
            &mut self,
            attachment: T,
        )
        where
            ActivityStreamEntity: From<S>;

        fn get_attributed_to(&self) -> &Option<Vec<ActivityStreamEntity>>;
        fn set_attributed_to<S: ActivityStreamEntityProperties, T: MaybeOptional<Vec<S>>>(
            &mut self,
            attributed_to: T,
        )
        where
            ActivityStreamEntity: From<S>;

        fn add_attributed_to<S: ActivityStreamEntityProperties, T: MaybeOptional<S>>(
            &mut self,
            attributed_to: T,
        )
        where
            ActivityStreamEntity: From<S>;

        fn get_audience(&self) -> &Option<BoxedActivityStreamEntity>;
        fn set_audience<S: ActivityStreamEntityProperties, T: MaybeOptional<S>>(&mut self, audience: T) where ActivityStreamEntity: From<S>;
        fn get_content(&self) -> &Option<ActivityStreamMultilangString>;
        fn set_content<S, T: MaybeOptional<S>>(&mut self, content: T) where ActivityStreamMultilangString: From<S>;
        fn get_end_time(&self) -> &Option<DateTime<Utc>>;
        fn set_end_time<T: MaybeOptional<DateTime<Utc>>>(&mut self, end_time: T);
        fn get_generator(&self) -> &Option<BoxedActivityStreamEntity>;
        fn set_generator<S: ActivityStreamEntityProperties, T: MaybeOptional<S>>(&mut self, generator: T) where ActivityStreamEntity: From<S>;
        fn get_icon(&self) -> &Option<ActivityStreamLinkableImage>;
        fn set_icon<S, T: MaybeOptional<S>>(&mut self, icon: T) where ActivityStreamLinkableImage: From<S>;
        fn get_image(&self) -> &Option<ActivityStreamLinkableImage>;
        fn set_image<S, T: MaybeOptional<S>>(&mut self, image: T) where ActivityStreamLinkableImage: From<S>;
        fn get_in_reply_to(&self) -> &Option<Url>;
        fn set_in_reply_to<T: MaybeOptional<Url>>(&mut self, in_reply_to: T);
        fn get_location(&self) -> &Option<BoxedActivityStreamEntity>;
        fn set_location<S: ActivityStreamEntityProperties, T: MaybeOptional<S>>(&mut self, location: T) where ActivityStreamEntity: From<S>;
        fn get_preview(&self) -> &Option<BoxedActivityStreamEntity>;
        fn set_preview<S: ActivityStreamEntityProperties, T: MaybeOptional<S>>(&mut self, preview: T) where ActivityStreamEntity: From<S>;
        fn get_published(&self) -> &Option<DateTime<Utc>>;
        fn set_published<T: MaybeOptional<DateTime<Utc>>>(&mut self, published: T);
        fn get_replies(&self) -> &Option<ActivityStreamCollection>;
        fn set_replies<T: MaybeOptional<ActivityStreamCollection>>(&mut self, replies: T);
        fn get_start_time(&self) -> &Option<DateTime<Utc>>;
        fn set_start_time<T: MaybeOptional<DateTime<Utc>>>(&mut self, start_time: T);
        fn get_summary(&self) -> &Option<ActivityStreamMultilangString>;
        fn set_summary<S, T: MaybeOptional<S>>(&mut self, summary: T) where ActivityStreamMultilangString: From<S>;
        fn get_tags(&self) -> &Option<Vec<ActivityStreamEntity>>;
        fn set_tags<S: ActivityStreamEntityProperties, T: MaybeOptional<Vec<S>>>(&mut self, attachment: T) where ActivityStreamEntity: From<S>;
        fn add_tag<S: ActivityStreamEntityProperties, T: MaybeOptional<S>>(&mut self, attachment: T) where ActivityStreamEntity: From<S>;
        fn get_updated(&self) -> &Option<DateTime<Utc>>;
        fn set_updated<T: MaybeOptional<DateTime<Utc>>>(&mut self, updated: T);
        fn get_url(&self) -> &Option<ActivityStreamLinkableUrl>;
        fn set_url<S, T: MaybeOptional<S>>(&mut self, url: T) where ActivityStreamLinkableUrl: From<S>;
        fn get_to(&self) -> &Option<BoxedActivityStreamEntity>;
        fn set_to<S: ActivityStreamEntityProperties, T: MaybeOptional<S>>(&mut self, to: T) where ActivityStreamEntity: From<S>;
        fn get_bto(&self) -> &Option<BoxedActivityStreamEntity>;
        fn set_bto<S: ActivityStreamEntityProperties, T: MaybeOptional<S>>(&mut self, to: T) where ActivityStreamEntity: From<S>;
        fn get_cc(&self) -> &Option<BoxedActivityStreamEntity>;
        fn set_cc<S: ActivityStreamEntityProperties, T: MaybeOptional<S>>(&mut self, cc: T) where ActivityStreamEntity: From<S>;
        fn get_bcc(&self) -> &Option<Vec<ActivityStreamEntity>>;
        fn set_bcc<S: ActivityStreamEntityProperties, T: MaybeOptional<Vec<S>>>(&mut self, attachment: T) where ActivityStreamEntity: From<S>;
        fn add_bcc<S: ActivityStreamEntityProperties, T: MaybeOptional<S>>(&mut self, attachment: T) where ActivityStreamEntity: From<S>;
        fn get_media_type(&self) -> &Option<String>;
        fn set_media_type<T: MaybeOptional<String>>(&mut self, media_type: T);
        fn get_duration(&self) -> &Option<String>;
        fn set_duration<T: MaybeOptional<String>>(&mut self, duration: T);
    }

    pub trait ActivityStreamActivityProperties {
        fn get_actor(&self) -> &Option<BoxedActivityStreamEntity>;
        fn set_actor<S: ActivityStreamEntityProperties, T: MaybeOptional<S>>(&mut self, actor: T) where ActivityStreamEntity: From<S>;
        fn get_object(&self) -> &Option<BoxedActivityStreamEntity>;
        fn set_object<S: ActivityStreamEntityProperties, T: MaybeOptional<S>>(&mut self, object: T) where ActivityStreamEntity: From<S>;
        fn get_target(&self) -> &Option<BoxedActivityStreamEntity>;
        fn set_target<S: ActivityStreamEntityProperties, T: MaybeOptional<S>>(&mut self, target: T) where ActivityStreamEntity: From<S>;
        fn get_result(&self) -> &Option<BoxedActivityStreamEntity>;
        fn set_result<S: ActivityStreamEntityProperties, T: MaybeOptional<S>>(&mut self, result: T) where ActivityStreamEntity: From<S>;
        fn get_origin(&self) -> &Option<BoxedActivityStreamEntity>;
        fn set_origin<S: ActivityStreamEntityProperties, T: MaybeOptional<S>>(&mut self, origin: T) where ActivityStreamEntity: From<S>;
        fn get_instrument(&self) -> &Option<BoxedActivityStreamEntity>;
        fn set_instrument<S: ActivityStreamEntityProperties, T: MaybeOptional<S>>(&mut self, instrument: T) where ActivityStreamEntity: From<S>;
    }

    pub trait ActivityStreamIntransitiveActivityProperties {
        fn get_actor(&self) -> &Option<BoxedActivityStreamEntity>;
        fn set_actor<S: ActivityStreamEntityProperties, T: MaybeOptional<S>>(&mut self, actor: T) where ActivityStreamEntity: From<S>;
        fn get_target(&self) -> &Option<BoxedActivityStreamEntity>;
        fn set_target<S: ActivityStreamEntityProperties, T: MaybeOptional<S>>(&mut self, target: T) where ActivityStreamEntity: From<S>;
        fn get_result(&self) -> &Option<BoxedActivityStreamEntity>;
        fn set_result<S: ActivityStreamEntityProperties, T: MaybeOptional<S>>(&mut self, result: T) where ActivityStreamEntity: From<S>;
        fn get_origin(&self) -> &Option<BoxedActivityStreamEntity>;
        fn set_origin<S: ActivityStreamEntityProperties, T: MaybeOptional<S>>(&mut self, origin: T) where ActivityStreamEntity: From<S>;
        fn get_instrument(&self) -> &Option<BoxedActivityStreamEntity>;
        fn set_instrument<S: ActivityStreamEntityProperties, T: MaybeOptional<S>>(&mut self, instrument: T) where ActivityStreamEntity: From<S>;
    }

    //// This trait allows access to all the basic elements of a core type
    pub trait ActivityStreamEntityProperties {
        fn get_id(&self) -> &Option<Url>;
        fn set_id<T: MaybeOptional<Url>>(&mut self, id: T);
        fn get_type(&self) -> &ActivityStreamEntityType;
        fn set_type(&mut self, r#type: ActivityStreamEntityType);
        fn register_context(&mut self, context: Url);
    }

}

