use url::Url;

//// This enum describes the ActivityStream core types as defined in [Section 2 of the specification](https://www.w3.org/TR/activitystreams-vocabulary/#types)
pub enum ActivityStreamCoreType {
    Object,
    Link,
    Activity,
    IntransitiveActivity,
    Collection,
    OrderedCollection,
    CollectionPage,
    OrderedCollectionPage,
}

//// This trait allows access to all the basic elements of a core type
pub trait ActivityStreamCoreElement {
    fn get_id(&self) -> &Option<Url>;
    fn set_id(&mut self, id: Option<Url>);
    fn get_type(&self) -> ActivityStreamCoreType;
}

pub type ActivityStreamCoreElementBoxed = Box<dyn ActivityStreamCoreElement>;
pub type ActivityStreamCoreElementsBoxed = Box<Vec<dyn ActivityStreamCoreElement>>;
