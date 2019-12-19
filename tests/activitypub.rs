extern crate activityrust;
use activityrust::entities::entity::ActivityStreamEntity;

use std::fs::File;
use std::io::prelude::*;

macro_rules! generate_test {
    ($testname: ident, $testid: expr) => {
        #[test]
        fn $testname() {
            let mut dtx =
                File::open(format!("tests/activitypub_tests/example_{}.json", $testid)).unwrap();
            let mut string = String::new();
            dtx.read_to_string(&mut string).unwrap();
            let decoded = serde_json::from_str::<ActivityStreamEntity>(&string).unwrap();
            //Let's re-encode it
            let encoded = serde_json::to_string(&decoded).unwrap();
            //Let's re-re decode it
            let redecoded = serde_json::from_str::<ActivityStreamEntity>(&encoded).unwrap();
            assert_eq!(decoded, redecoded);
        }
    };
}

generate_test!(test_000, 0);
generate_test!(test_001, 1);
generate_test!(test_002, 2);
generate_test!(test_003, 3);
generate_test!(test_004, 4);
generate_test!(test_005, 5);
generate_test!(test_006, 6);
generate_test!(test_007, 7);
generate_test!(test_008, 8);
generate_test!(test_009, 9);
generate_test!(test_010, 10);
generate_test!(test_011, 11);
generate_test!(test_012, 12);
generate_test!(test_013, 13);
generate_test!(test_014, 14);
