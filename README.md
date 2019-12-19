# ActivityRust

## What is ActivityRust ?
ActivityRust is a crate that allows a user to manage the structures used by common implementations of the ActivityPub protocol. In order to do so, it tries to be compilant with:
 * The ActivityStream Vocabulary, as defined [here](https://www.w3.org/TR/activitystreams-vocabulary/)
 * The ActivityPub extensions, as defined [here](https://www.w3.org/TR/activitypub/)
 * Parts of the W3ID Security extension, as defined [here](https://w3id.org/)
 * The WebFinger schema
ActivityRust allows creation and edition of these structures, but can also serialize and deserialize them to json.

## How to use ActivityRust ?

### Creating an ActivityStream entity

```rust
extern crate activityrust;
// Import the required traits
use activityrust::traits::properties::*;

use activityrust::entities::actortypes::ActivityStreamPerson;
use url::Url;

fn main() {

  let mut activity = ActivityStreamPerson::create();
  let activity_url = Url::parse("http://test.test").unwrap();
  activity.set_id(activity_url);

}
```
As ActivityStream, ActivityRust supports setting null values for properties. In practice, it means that you can do this:
```rust
  activity.set_summary::<String, Option<String>>(None);
```

### Deserializing an ActivityStream entity

ActivityRust supports the `serde` module, and all entities can be deserialized to their proper types and serialized to JSON.
