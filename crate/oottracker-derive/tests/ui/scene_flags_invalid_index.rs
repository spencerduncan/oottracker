//! Test that scene_flags! macro produces error for non-integer scene index.
//!
//! This test verifies that using a non-numeric scene index produces a parse error.

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

// This should fail because "invalid" cannot be parsed as a scene index
scene_flags! {
    pub struct InvalidSceneFlags {
        invalid: "Test Scene" {
            chests: {
                "Chest" = 0x0000_0001,
            },
        },
    }
}

fn main() {}
