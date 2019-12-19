#[macro_use]
pub mod properties {

    use crate::content::*;
    use crate::entities::collection::ActivityStreamCollection;
    use crate::entities::entity::{
        ActivityStreamEntity, ActivityStreamEntityType, BoxedActivityStreamEntity,
    };
    use crate::entities::orderedcollection::ActivityStreamOrderedCollection;
    use crate::MaybeOptional;
    use ambassador::delegatable_trait;
    use chrono::{DateTime, FixedOffset};
    use serde::Deserializer;
    use url::Url;

    #[delegatable_trait]
    pub trait ActivityStreamObjectProperties {
        fn get_id(&self) -> &Option<Url>;
        fn set_id<T: MaybeOptional<Url>>(&mut self, id: T);
        fn register_context(&mut self, context: Url);
        fn get_attachments(&self) -> &Option<Vec<ActivityStreamEntity>>;
        fn set_attachments<S, T: MaybeOptional<Vec<S>>>(
            &mut self,
            attachment: T,
        ) where
            ActivityStreamEntity: From<S>;
        fn add_attachment<S, T: MaybeOptional<S>>(
            &mut self,
            attachment: T,
        ) where
            ActivityStreamEntity: From<S>;

        fn get_attributed_to(&self) -> &Option<Vec<ActivityStreamEntity>>;
        fn set_attributed_to<S, T: MaybeOptional<Vec<S>>>(
            &mut self,
            attributed_to: T,
        ) where
            ActivityStreamEntity: From<S>;

        fn add_attributed_to<S, T: MaybeOptional<S>>(
            &mut self,
            attributed_to: T,
        ) where
            ActivityStreamEntity: From<S>;

        fn get_audience(&self) -> &Option<BoxedActivityStreamEntity>;
        fn set_audience<S, T: MaybeOptional<S>>(
            &mut self,
            audience: T,
        ) where
            ActivityStreamEntity: From<S>;
        fn get_content(&self) -> &Option<ActivityStreamMultilangString>;
        fn set_content<S, T: MaybeOptional<S>>(&mut self, content: T)
        where
            ActivityStreamMultilangString: From<S>;
        fn get_end_time(&self) -> &Option<DateTime<FixedOffset>>;
        fn set_end_time<T: MaybeOptional<DateTime<FixedOffset>>>(&mut self, end_time: T);
        fn get_generator(&self) -> &Option<BoxedActivityStreamEntity>;
        fn set_generator<S, T: MaybeOptional<S>>(
            &mut self,
            generator: T,
        ) where
            ActivityStreamEntity: From<S>;
        fn get_icon(&self) -> &Option<ActivityStreamLinkableImage>;
        fn set_icon<S, T: MaybeOptional<S>>(&mut self, icon: T)
        where
            ActivityStreamLinkableImage: From<S>;
        fn get_image(&self) -> &Option<ActivityStreamLinkableImage>;
        fn set_image<S, T: MaybeOptional<S>>(&mut self, image: T)
        where
            ActivityStreamLinkableImage: From<S>;
        fn get_in_reply_to(&self) -> &Option<BoxedActivityStreamEntity>;
        fn set_in_reply_to<S, T: MaybeOptional<S>>(
            &mut self,
            in_reply_to: T,
        ) where
            ActivityStreamEntity: From<S>;
        fn get_location(&self) -> &Option<BoxedActivityStreamEntity>;
        fn set_location<S, T: MaybeOptional<S>>(
            &mut self,
            location: T,
        ) where
            ActivityStreamEntity: From<S>;
        fn get_preview(&self) -> &Option<BoxedActivityStreamEntity>;
        fn set_preview<S, T: MaybeOptional<S>>(
            &mut self,
            preview: T,
        ) where
            ActivityStreamEntity: From<S>;
        fn get_published(&self) -> &Option<DateTime<FixedOffset>>;
        fn set_published<T: MaybeOptional<DateTime<FixedOffset>>>(&mut self, published: T);
        fn get_replies(&self) -> &Option<Box<ActivityStreamCollection>>;
        fn set_replies<T: MaybeOptional<ActivityStreamCollection>>(&mut self, replies: T);
        fn get_start_time(&self) -> &Option<DateTime<FixedOffset>>;
        fn set_start_time<T: MaybeOptional<DateTime<FixedOffset>>>(&mut self, start_time: T);
        fn get_summary(&self) -> &Option<ActivityStreamMultilangString>;
        fn set_summary<S, T: MaybeOptional<S>>(&mut self, summary: T)
        where
            ActivityStreamMultilangString: From<S>;
        fn get_tags(&self) -> &Option<Vec<ActivityStreamEntity>>;
        fn set_tags<S, T: MaybeOptional<Vec<S>>>(
            &mut self,
            attachment: T,
        ) where
            ActivityStreamEntity: From<S>;
        fn add_tag<S, T: MaybeOptional<S>>(
            &mut self,
            attachment: T,
        ) where
            ActivityStreamEntity: From<S>;
        fn get_updated(&self) -> &Option<DateTime<FixedOffset>>;
        fn set_updated<T: MaybeOptional<DateTime<FixedOffset>>>(&mut self, updated: T);
        fn get_url(&self) -> &Option<ActivityStreamLinkableUrl>;
        fn set_url<S, T: MaybeOptional<S>>(&mut self, url: T)
        where
            ActivityStreamLinkableUrl: From<S>;
        fn get_to(&self) -> &Option<BoxedActivityStreamEntity>;
        fn set_to<S, T: MaybeOptional<S>>(&mut self, to: T)
        where
            ActivityStreamEntity: From<S>;
        fn get_bto(&self) -> &Option<BoxedActivityStreamEntity>;
        fn set_bto<S, T: MaybeOptional<S>>(&mut self, to: T)
        where
            ActivityStreamEntity: From<S>;
        fn get_cc(&self) -> &Option<BoxedActivityStreamEntity>;
        fn set_cc<S, T: MaybeOptional<S>>(&mut self, cc: T)
        where
            ActivityStreamEntity: From<S>;
        fn get_bcc(&self) -> &Option<Vec<ActivityStreamEntity>>;
        fn set_bcc<S, T: MaybeOptional<Vec<S>>>(
            &mut self,
            attachment: T,
        ) where
            ActivityStreamEntity: From<S>;
        fn add_bcc<S, T: MaybeOptional<S>>(
            &mut self,
            attachment: T,
        ) where
            ActivityStreamEntity: From<S>;
        fn get_media_type(&self) -> &Option<String>;
        fn set_media_type<T: MaybeOptional<String>>(&mut self, media_type: T);
        fn get_duration(&self) -> &Option<String>;
        fn set_duration<T: MaybeOptional<String>>(&mut self, duration: T);
    }

    #[delegatable_trait]
    pub trait ActivityStreamLinkProperties {
        fn get_id(&self) -> &Option<Url>;
        fn set_id<T: MaybeOptional<Url>>(&mut self, id: T);
        fn register_context(&mut self, context: Url);
        fn get_href(&self) -> &Option<Url>;
        fn set_href<T: MaybeOptional<Url>>(&mut self, href: T);
        fn get_hreflang(&self) -> &Option<String>;
        fn set_hreflang<T: MaybeOptional<String>>(&mut self, hreflang: T);
        fn get_media_type(&self) -> &Option<String>;
        fn set_media_type<T: MaybeOptional<String>>(&mut self, media_type: T);
        fn get_name(&self) -> &Option<ActivityStreamMultilangString>;
        fn set_name<S, T: MaybeOptional<S>>(&mut self, name: T)
        where
            ActivityStreamMultilangString: From<S>;
        fn get_height(&self) -> &Option<usize>;
        fn set_height<T: MaybeOptional<usize>>(&mut self, height: T);
        fn get_width(&self) -> &Option<usize>;
        fn set_width<T: MaybeOptional<usize>>(&mut self, width: T);
        fn get_preview(&self) -> &Option<BoxedActivityStreamEntity>;
        fn set_preview<S, T: MaybeOptional<S>>(
            &mut self,
            audience: T,
        ) where
            ActivityStreamEntity: From<S>;
    }

    #[delegatable_trait]
    pub trait ActivityStreamActivityProperties {
        fn get_actor(&self) -> &Option<BoxedActivityStreamEntity>;
        fn set_actor<S, T: MaybeOptional<S>>(&mut self, actor: T)
        where
            ActivityStreamEntity: From<S>;
        fn get_object(&self) -> &Option<BoxedActivityStreamEntity>;
        fn set_object<S, T: MaybeOptional<S>>(&mut self, object: T)
        where
            ActivityStreamEntity: From<S>;
        fn get_targets(&self) -> &Option<Vec<ActivityStreamEntity>>;
        fn set_targets<S, T: MaybeOptional<Vec<S>>>(&mut self, targets: T)
        where
            ActivityStreamEntity: From<S>;
        fn add_target<S, T: MaybeOptional<S>>(&mut self, target: T)
        where
            ActivityStreamEntity: From<S>;
        fn get_result(&self) -> &Option<BoxedActivityStreamEntity>;
        fn set_result<S, T: MaybeOptional<S>>(&mut self, result: T)
        where
            ActivityStreamEntity: From<S>;
        fn get_origin(&self) -> &Option<BoxedActivityStreamEntity>;
        fn set_origin<S, T: MaybeOptional<S>>(&mut self, origin: T)
        where
            ActivityStreamEntity: From<S>;
        fn get_instrument(&self) -> &Option<BoxedActivityStreamEntity>;
        fn set_instrument<S, T: MaybeOptional<S>>(
            &mut self,
            instrument: T,
        ) where
            ActivityStreamEntity: From<S>;
    }

    #[delegatable_trait]
    pub trait ActivityStreamIntransitiveActivityProperties {
        fn get_actor(&self) -> &Option<BoxedActivityStreamEntity>;
        fn set_actor<S, T: MaybeOptional<S>>(&mut self, actor: T)
        where
            ActivityStreamEntity: From<S>;
        fn get_targets(&self) -> &Option<Vec<ActivityStreamEntity>>;
        fn set_targets<S, T: MaybeOptional<Vec<S>>>(&mut self, targets: T)
        where
            ActivityStreamEntity: From<S>;
        fn add_target<S, T: MaybeOptional<S>>(&mut self, target: T)
        where
            ActivityStreamEntity: From<S>;
        fn get_result(&self) -> &Option<BoxedActivityStreamEntity>;
        fn set_result<S, T: MaybeOptional<S>>(&mut self, result: T)
        where
            ActivityStreamEntity: From<S>;
        fn get_origin(&self) -> &Option<BoxedActivityStreamEntity>;
        fn set_origin<S, T: MaybeOptional<S>>(&mut self, origin: T)
        where
            ActivityStreamEntity: From<S>;
        fn get_instrument(&self) -> &Option<BoxedActivityStreamEntity>;
        fn set_instrument<S, T: MaybeOptional<S>>(
            &mut self,
            instrument: T,
        ) where
            ActivityStreamEntity: From<S>;
    }

    #[delegatable_trait]
    pub trait ActivityStreamCollectionProperties {
        fn get_total_items(&self) -> &Option<usize>;
        fn get_current(&self) -> &Option<BoxedActivityStreamEntity>;
        fn set_current<S, T: MaybeOptional<S>>(
            &mut self,
            current: T,
        ) where
            ActivityStreamEntity: From<S>;
        fn get_first(&self) -> &Option<BoxedActivityStreamEntity>;
        fn set_first<S, T: MaybeOptional<S>>(&mut self, first: T)
        where
            ActivityStreamEntity: From<S>;
        fn get_last(&self) -> &Option<BoxedActivityStreamEntity>;
        fn set_last<S, T: MaybeOptional<S>>(&mut self, last: T)
        where
            ActivityStreamEntity: From<S>;
        fn get_items(&self) -> &Option<Vec<ActivityStreamEntity>>;
        fn set_items<S, T: MaybeOptional<Vec<S>>>(
            &mut self,
            items: T,
        ) where
            ActivityStreamEntity: From<S>;
        fn add_item<S, T: MaybeOptional<S>>(&mut self, item: T)
        where
            ActivityStreamEntity: From<S>;
    }

    #[delegatable_trait]
    pub trait ActivityStreamCollectionPageProperties {
        fn get_part_of(&self) -> &Option<ActivityStreamLinkableCollection>;
        fn set_part_of<S, T: MaybeOptional<S>>(&mut self, part_of: T)
        where
            ActivityStreamLinkableCollection: From<S>;
        fn get_next(&self) -> &Option<BoxedActivityStreamEntity>;
        fn set_next<S, T: MaybeOptional<S>>(&mut self, next: T)
        where
            ActivityStreamEntity: From<S>;
        fn get_prev(&self) -> &Option<BoxedActivityStreamEntity>;
        fn set_prev<S, T: MaybeOptional<S>>(&mut self, prev: T)
        where
            ActivityStreamEntity: From<S>;
    }

    #[delegatable_trait]
    pub trait ActivityStreamQuestionProperties {
        fn get_one_of(&self) -> &Option<Vec<ActivityStreamEntity>>;
        fn set_one_of<S, T: MaybeOptional<Vec<S>>>(
            &mut self,
            one_of: T,
        ) where
            ActivityStreamEntity: From<S>;
        fn add_one_of<S, T: MaybeOptional<S>>(&mut self, one_of: T)
        where
            ActivityStreamEntity: From<S>;
        fn get_any_of(&self) -> &Option<Vec<ActivityStreamEntity>>;
        fn set_any_of<S, T: MaybeOptional<Vec<S>>>(
            &mut self,
            any_of: T,
        ) where
            ActivityStreamEntity: From<S>;
        fn add_any_of<S, T: MaybeOptional<S>>(&mut self, one_of: T)
        where
            ActivityStreamEntity: From<S>;
        fn get_closed(&self) -> &Option<Box<ActivityStreamQuestionClosed>>;
        fn set_closed<S, T: MaybeOptional<S>>(&mut self, closed: T)
        where
            ActivityStreamQuestionClosed: From<S>;
    }

    #[delegatable_trait]
    pub trait ActivityStreamActorProperties {
      fn get_inbox(&self) -> &Option<ActivityStreamLinkableOrderedCollection>;
      fn set_inbox<S, T: MaybeOptional<S>>(&mut self, inbox: T) where ActivityStreamLinkableOrderedCollection: From<S>;
      fn get_outbox(&self) -> &Option<ActivityStreamLinkableOrderedCollection>;
      fn set_outbox<S, T: MaybeOptional<S>>(&mut self, outbox: T) where ActivityStreamLinkableOrderedCollection: From<S>;
      fn get_following(&self) -> &Option<ActivityStreamLinkableCollection>;
      fn set_following<S, T: MaybeOptional<S>>(&mut self, following: T) where ActivityStreamLinkableCollection: From<S>;
      fn get_followers(&self) -> &Option<ActivityStreamLinkableCollection>;
      fn set_followers<S, T: MaybeOptional<S>>(&mut self, followers: T) where ActivityStreamLinkableCollection: From<S>;
      fn get_liked(&self) -> &Option<ActivityStreamLinkableCollection>;
      fn set_liked<S, T: MaybeOptional<S>>(&mut self, liked: T) where ActivityStreamLinkableCollection: From<S>;
    }

    pub trait ActivityStreamPlaceProperties {
      fn get_accuracy(&self) -> &Option<f64>;
      fn set_accuracy<T: MaybeOptional<f64>>(&mut self, accuracy: T);
      fn get_altitude(&self) -> &Option<f64>;
      fn set_altitude<T: MaybeOptional<f64>>(&mut self, altitude: T);
      fn get_latitude(&self) -> &Option<f64>;
      fn set_latitude<T: MaybeOptional<f64>>(&mut self, latitude: T);
      fn get_longitude(&self) -> &Option<f64>;
      fn set_longitude<T: MaybeOptional<f64>>(&mut self, longitude: T);
      fn get_radius(&self) -> &Option<f64>;
      fn set_radius<T: MaybeOptional<f64>>(&mut self, radius: T);
      fn get_units(&self) -> &Option<ActivityStreamUnit>;
      fn set_units<T: MaybeOptional<ActivityStreamUnit>>(&mut self, units: T);
    }

    pub trait ActivityStreamProfileProperties {
      fn get_describes(&self) -> &Option<BoxedActivityStreamEntity>;

      fn set_describes<S, T: MaybeOptional<S>>(
        &mut self,
        describes: T,
    ) where
        ActivityStreamEntity: From<S>;
    }

    pub trait ActivityStreamTombstoneProperties {
      fn get_former_type(&self) -> &Option<ActivityStreamEntityType>;

      fn set_former_type<T: MaybeOptional<ActivityStreamEntityType>>(
        &mut self,
        former_type: T,
    ) ;

      fn get_deleted(&self) -> &Option<DateTime<FixedOffset>>;
      fn set_deleted<T: MaybeOptional<DateTime<FixedOffset>>>(&mut self, deleted: T);
    }

    #[delegatable_trait]
    pub trait ActivityStreamOrderedCollectionPageProperties {
        fn get_start_index(&self) -> &Option<usize>;
        fn set_start_index<T: MaybeOptional<usize>>(&mut self, start_index: T);
    }

    //// This trait allows access to all the basic elements of a core type
    #[delegatable_trait]
    pub trait ActivityStreamEntityProperties {
        //FIXME: Return option, add function is_of_type
        fn get_type(&self) -> &Option<ActivityStreamEntityType>;
        fn set_type(&mut self, r#type: ActivityStreamEntityType);
    }

    pub trait ActivityStreamRelationshipProperties {
        fn get_subject(&self) -> &Option<BoxedActivityStreamEntity>;
        fn set_subject<S, T: MaybeOptional<S>>(&mut self, subject: T)
        where
            ActivityStreamEntity: From<S>;

        fn get_object(&self) -> &Option<BoxedActivityStreamEntity>;

        fn set_object<S, T: MaybeOptional<S>>(&mut self, object: T)
        where
            ActivityStreamEntity: From<S>;

        fn get_relationship(&self) -> &Option<Box<ActivityStreamLinkableRelationship>>;

        fn set_relationship<S, T: MaybeOptional<S>>(&mut self, relationship: T)
        where
            ActivityStreamLinkableRelationship: From<S>;
    }

    pub trait DeserializeType {
        fn deserialize_type<'de, D>(des: D) -> Result<Option<ActivityStreamEntityType>, D::Error>
        where
            D: Deserializer<'de>;
    }

    macro_rules! generate_basics {
        ($objname:ty, $objtype:expr) => {
            impl ActivityStreamEntityProperties for $objname {
                fn get_type(&self) -> &Option<ActivityStreamEntityType> {
                    &self.r#type
                }

                fn set_type(&mut self, r#type: ActivityStreamEntityType) {
                    self.r#type = Some(r#type);
                }
            }

            impl $objname {
                pub fn create() -> Self {
                    let object_context =
                        Url::parse("https://www.w3.org/ns/activitystreams").unwrap();

                    let mut new_object = <$objname>::default();
                    new_object.register_context(object_context);
                    new_object.set_type($objtype);
                    new_object
                }
            }

            impl DeserializeType for $objname {
                fn deserialize_type<'de, D>(
                    des: D,
                ) -> Result<Option<ActivityStreamEntityType>, D::Error>
                where
                    D: serde::Deserializer<'de>,
                {
                    if let Ok(ax) = ActivityStreamEntityType::deserialize(des) {
                        if ax == $objtype {
                            return Ok(Some(ax));
                        }
                    }
                    Err(serde::de::Error::custom("Invalid constant !"))
                }
            }
        };
    }
}

pub mod vecserializer {

    use serde::{Serialize, Deserialize, Serializer, Deserializer};

    pub fn serialize<S, T: Serialize>(element: &Option<Vec<T>>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer {

        match element {
            None => element.serialize(serializer),
            Some(element) => {
                    if element.len() == 1 {
                      let first_element = element.get(0).unwrap();
                      first_element.serialize(serializer)
                    } else {
                        element.serialize(serializer)
                    }
            }
        }

    }

    pub fn deserialize<'de, D, T: Deserialize<'de>>(deserializer: D) -> Result<Option<Vec<T>>, D::Error>
    where
        D: Deserializer<'de> {
      let dtx: Option<SingularVecSerializer_helper<T>> = Option::<SingularVecSerializer_helper<T>>::deserialize(deserializer)?;
      if let Some(dtx) = dtx {
          match dtx {
            SingularVecSerializer_helper::Element(elem) => {
              Ok(Some(vec![elem]))
            }
            SingularVecSerializer_helper::Vec(elem) => {
              Ok(Some(elem))
            }
          }
      } else {
        Ok(None)
      }
    }

    #[derive(Debug, PartialEq, Serialize, Deserialize)]
    #[serde(untagged)]
    enum SingularVecSerializer_helper<T> {
      Element(T),
      Vec(Vec<T>),
    }

}

pub mod optionaldateserializer {

    use serde::{Serialize, Deserialize, Serializer, Deserializer};
    use chrono::{DateTime, FixedOffset};
    use chrono::offset::TimeZone;
    pub fn serialize<S>(element: &Option<DateTime<FixedOffset>>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer {

        match element {
            None => {
                let element: Option<String> = None;
                element.serialize(serializer)
            }
            Some(element) => {
                Option::Some(element.to_rfc3339()).serialize(serializer)
            }
        }

    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<DateTime<FixedOffset>>, D::Error>
    where
        D: Deserializer<'de> {
      let dtx: Option<String> = Option::<String>::deserialize(deserializer)?;
      if let Some(dtx) = dtx {

        if let Ok(date) = DateTime::parse_from_rfc3339(&dtx) {
            Ok(Some(date))
        } else if let Ok(date) = chrono::Utc.datetime_from_str(&dtx, "%FT%T") {
            Ok(Some(DateTime::<FixedOffset>::from(date)))
        } else {
            Err(serde::de::Error::custom("Invalid DateTime"))
        }
      } else {
        Ok(None)
      }
    }
}

pub mod dateserializer {

    use serde::{Serialize, Deserialize, Serializer, Deserializer};
    use chrono::{DateTime, FixedOffset};

    pub fn serialize<S>(element: &DateTime<FixedOffset>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer {

        element.to_rfc3339().serialize(serializer)

    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<DateTime<FixedOffset>, D::Error>
    where
        D: Deserializer<'de> {
      let dtx = String::deserialize(deserializer)?;
        if let Ok(date) = DateTime::parse_from_rfc3339(&dtx) {
            return Ok(date);
        }
    return Err(serde::de::Error::custom("Invalid DateTime"));
}
}

