extern crate activityrust;
use activityrust::entities::entity::ActivityStreamEntity;

use std::io::prelude::*;
use std::fs::File;
const TEST_COUNT: u32 = 159;

#[test]
fn test_object() {

  for i in 0..TEST_COUNT {
    let mut dtx = File::open(format!("tests/activitystream_tests/example_{}.json", i)).unwrap();
    let mut string = String::new();
    dtx.read_to_string(&mut string).unwrap();
    let ent: ActivityStreamEntity = serde_json::from_str(&string).unwrap();
  }

}
