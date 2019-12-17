extern crate activityrust;
use activityrust::content::ActivityStreamMultilangString;
use activityrust::entities::entity::ActivityStreamEntityProperties;
use activityrust::entities::object::ActivityStreamObject;
use activityrust::entities::object::ActivityStreamObjectProperties;
use std::str::FromStr;
use url::Url;

#[test]
fn test_object() {
    let mut activitystream_object = ActivityStreamObject::default();
    assert_eq!(*activitystream_object.get_id(), None);

    let id = Url::from_str(&"https://test.com").unwrap();
    activitystream_object.set_id(id.clone());
    assert_eq!(*activitystream_object.get_id(), Some(id));

    let content = String::from("ActivityTest-Content");
    activitystream_object.set_content(content.clone());
    assert_eq!(
        *activitystream_object.get_content(),
        Some(ActivityStreamMultilangString::String(content))
    );

    let context = String::from("ActivityTest-Context");
    activitystream_object.set_context(context.clone());
    assert_eq!(
        *activitystream_object.get_context(),
        Some(ActivityStreamMultilangString::String(context))
    );
}
