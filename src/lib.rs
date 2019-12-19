//!# ActivityRust
//!
//!## What is ActivityRust ?
//!ActivityRust is a crate that allows a user to manage the structures used by common implementations of the ActivityPub protocol. In order to do so, it tries to be compilant with:
//! * The ActivityStream Vocabulary, as defined [here](https://www.w3.org/TR/activitystreams-vocabulary/)
//! * The ActivityPub extensions, as defined [here](https://www.w3.org/TR/activitypub/)
//! * Parts of the W3ID Security extension, as defined [here](https://w3id.org/)
//! * The WebFinger schema
//!ActivityRust allows creation and edition of these structures, but can also serialize and deserialize them to json.
//!
//!## How to use ActivityRust ?
//!
//!### Creating an ActivityStream entity
//!
//!```rust
//!extern crate activityrust;
//!// Import the required traits
//!use activityrust::traits::properties::*;
//!
//!use activityrust::entities::actortypes::ActivityStreamPerson;
//!use url::Url;
//!
//!fn main() {
//!
//!  let mut activity = ActivityStreamPerson::create();
//!  let activity_url = Url::parse("http://test.test").unwrap();
//!  activity.set_id(activity_url);
//!
//!}
//!```
//!As ActivityStream, ActivityRust supports setting null values for properties. In practice, it means that you can do this:
//!```rust
//!  activity.set_summary::<String, Option<String>>(None);
//!```
//!
//!### Deserializing/Serializing an ActivityStream entity
//!
//! ActivityRust supports the `serde` module, and all entities can be deserialized to their proper types and serialized to JSON.
//!
//!## Compatibility
//!
//!ActivityRust is tested against all the examples in `ActivityStream` and in `ActivityPub`, except examples `102` and `149`.
//!Indeed, example 102 uses multiple object types, that we don't support and example 149 because the content of the example does not match the specification
//!
//!We do have the following limitations:
//!  * The `type` property can only have a single value
//!  * The W3ID Security extension is only partially supported
#[macro_use]
pub mod traits;
pub mod content;
pub mod entities;

pub trait MaybeOptional<T> {
    fn get_optional(self) -> Option<T>;
}

impl<T> MaybeOptional<T> for T {
    fn get_optional(self) -> Option<T> {
        Some(self)
    }
}

impl<T> MaybeOptional<T> for Option<T> {
    fn get_optional(self) -> Option<T> {
        self
    }
}
