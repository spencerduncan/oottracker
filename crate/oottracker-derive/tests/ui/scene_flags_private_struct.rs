//! Test that scene_flags! macro works correctly with private (non-pub) structs.
//!
//! This is a passing test that verifies private structs are handled correctly.

use oottracker_derive::scene_flags;

extern crate bitflags;
extern crate byteorder;
extern crate ootr;
extern crate itertools;

pub(crate) trait FlagsScene {
    fn set_chests(&mut self, chests: u32);
    fn set_switches(&mut self, switches: u32);
    fn set_room_clear(&mut self, room_clear: u32);
}

pub(crate) struct Scene(pub(crate) &'static str);

pub(crate) mod region {
    pub trait RegionLookup {}
}

// Private struct should work correctly
scene_flags! {
    struct PrivateSceneFlags {
        0x00: "Test Area" {
            chests: {
                "Test Chest" = 0x0000_0001,
            },
            switches: {
                SWITCH_A = 0x0000_0001,
            },
        },
    }
}

fn main() {
    // Test that the private struct can be default-constructed
    let scene_flags = PrivateSceneFlags::default();

    // Test that TryFrom<Vec<u8>> works
    let data: Vec<u8> = vec![0; 0x1c * 0x65]; // scene_size * num_scenes
    let result = PrivateSceneFlags::try_from(data);
    assert!(result.is_ok());

    // Test serialization
    let bytes: Vec<u8> = (&scene_flags).into();
    assert_eq!(bytes.len(), 0x1c * 0x65);

    // Test that scene type is generated
    let _ = TestArea::default();

    // Test that chest flag type is generated
    let _ = TestAreaChests::empty();
}
