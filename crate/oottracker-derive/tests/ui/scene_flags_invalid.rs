//! Test that scene_flags! macro produces correct error for invalid scene field kind.

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

// This should fail because "invalid_field" is not a valid scene field kind.
// Valid kinds are: chests, switches, room_clear, collectible, unused,
// visited_rooms, visited_floors, gold_skulltulas
scene_flags! {
    pub struct InvalidSceneFlags {
        0x00: "Test Scene" {
            invalid_field: {
                FLAG_A = 0x0001,
            },
        },
    }
}

fn main() {}
