extern crate activityrust;
use activityrust::entities::entity::ActivityStreamEntity;

use std::io::prelude::*;
use std::fs::File;
const TEST_COUNT: u32 = 159;


macro_rules! generate_test {
    ($testname: ident, $testid: expr) => {
        #[test]
        fn $testname() {
            let mut dtx = File::open(format!("tests/activitystream_tests/example_{}.json", $testid)).unwrap();
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
generate_test!(test_015, 15);
generate_test!(test_016, 16);
generate_test!(test_017, 17);
generate_test!(test_018, 18);
generate_test!(test_019, 19);
generate_test!(test_020, 20);
generate_test!(test_021, 21);
generate_test!(test_022, 22);
generate_test!(test_023, 23);
generate_test!(test_024, 24);
generate_test!(test_025, 25);
generate_test!(test_026, 26);
generate_test!(test_027, 27);
generate_test!(test_028, 28);
generate_test!(test_029, 29);
generate_test!(test_030, 30);
generate_test!(test_031, 31);
generate_test!(test_032, 32);
generate_test!(test_033, 33);
generate_test!(test_034, 34);
generate_test!(test_035, 35);
generate_test!(test_036, 36);
generate_test!(test_037, 37);
generate_test!(test_038, 38);
generate_test!(test_039, 39);
generate_test!(test_040, 40);
generate_test!(test_041, 41);
generate_test!(test_042, 42);
generate_test!(test_043, 43);
generate_test!(test_044, 44);
generate_test!(test_045, 45);
generate_test!(test_046, 46);
generate_test!(test_047, 47);
generate_test!(test_048, 48);
generate_test!(test_049, 49);
generate_test!(test_050, 50);
generate_test!(test_051, 51);
generate_test!(test_052, 52);
generate_test!(test_053, 53);
generate_test!(test_054, 54);
generate_test!(test_055, 55);
generate_test!(test_056, 56);
generate_test!(test_057, 57);
generate_test!(test_058, 58);
generate_test!(test_059, 59);
generate_test!(test_060, 60);
generate_test!(test_061, 61);
generate_test!(test_062, 62);
generate_test!(test_063, 63);
generate_test!(test_064, 64);
generate_test!(test_065, 65);
generate_test!(test_066, 66);
generate_test!(test_067, 67);
generate_test!(test_068, 68);
generate_test!(test_069, 69);
generate_test!(test_070, 70);
generate_test!(test_071, 71);
generate_test!(test_072, 72);
generate_test!(test_073, 73);
generate_test!(test_074, 74);
generate_test!(test_075, 75);
generate_test!(test_076, 76);
generate_test!(test_077, 77);
generate_test!(test_078, 78);
generate_test!(test_079, 79);
generate_test!(test_080, 80);
generate_test!(test_081, 81);
generate_test!(test_082, 82);
generate_test!(test_083, 83);
generate_test!(test_084, 84);
generate_test!(test_085, 85);
generate_test!(test_086, 86);
generate_test!(test_087, 87);
generate_test!(test_088, 88);
generate_test!(test_089, 89);
generate_test!(test_090, 90);
generate_test!(test_091, 91);
generate_test!(test_092, 92);
generate_test!(test_093, 93);
generate_test!(test_094, 94);
generate_test!(test_095, 95);
generate_test!(test_096, 96);
generate_test!(test_097, 97);
generate_test!(test_098, 98);
generate_test!(test_099, 99);
generate_test!(test_100, 100);
generate_test!(test_101, 101);
//generate_test!(test_102, 102); This test is skipped, as we do not support multi-typed elements
generate_test!(test_103, 103);
generate_test!(test_104, 104);
generate_test!(test_105, 105);
generate_test!(test_106, 106);
generate_test!(test_107, 107);
generate_test!(test_108, 108);
generate_test!(test_109, 109);
generate_test!(test_110, 110);
generate_test!(test_111, 111);
generate_test!(test_112, 112);
generate_test!(test_113, 113);
generate_test!(test_114, 114);
generate_test!(test_115, 115);
generate_test!(test_116, 116);
generate_test!(test_117, 117);
generate_test!(test_118, 118);
generate_test!(test_119, 119);
generate_test!(test_120, 120);
generate_test!(test_121, 121);
generate_test!(test_122, 122);
generate_test!(test_123, 123);
generate_test!(test_124, 124);
generate_test!(test_125, 125);
generate_test!(test_126, 126);
generate_test!(test_127, 127);
generate_test!(test_128, 128);
generate_test!(test_129, 129);
generate_test!(test_130, 130);
generate_test!(test_131, 131);
generate_test!(test_132, 132);
generate_test!(test_133, 133);
generate_test!(test_134, 134);
generate_test!(test_135, 135);
generate_test!(test_136, 136);
generate_test!(test_137, 137);
generate_test!(test_138, 138);
generate_test!(test_139, 139);
generate_test!(test_140, 140);
generate_test!(test_141, 141);
generate_test!(test_142, 142);
generate_test!(test_143, 143);
generate_test!(test_144, 144);
generate_test!(test_145, 145);
generate_test!(test_146, 146);
generate_test!(test_147, 147);
generate_test!(test_148, 148);
//generate_test!(test_149, 149); This test is skipped because the content of the example does not match the specification
generate_test!(test_150, 150);
generate_test!(test_151, 151);
generate_test!(test_152, 152);
generate_test!(test_153, 153);
generate_test!(test_154, 154);
generate_test!(test_155, 155);
generate_test!(test_156, 156);
generate_test!(test_157, 157);
generate_test!(test_158, 158);
