//! Test that scene_flags! macro generates correct scene structures
//! and region lookup implementation.

use oottracker_derive::scene_flags;

extern crate bitflags;
extern crate byteorder;
extern crate ootr;
extern crate itertools;

// Stub implementation of FlagsScene trait required by generated code
pub(crate) trait FlagsScene {
    fn set_chests(&mut self, chests: u32);
    fn set_switches(&mut self, switches: u32);
    fn set_room_clear(&mut self, room_clear: u32);
}

// Stub Scene type required by generated code
pub(crate) struct Scene(pub(crate) &'static str);

// Stub RegionLookup trait required by generated code
pub(crate) mod region {
    pub trait RegionLookup {}
}

scene_flags! {
    pub struct TestSceneFlags {
        0x00: "Test Dungeon" {
            chests: {
                "Test Dungeon Chest A" = 0x0000_0001,
                "Test Dungeon Chest B" = 0x0000_0002,
            },
            switches: {
                SWITCH_A = 0x0000_0001,
                event "Test Event" = 0x0000_0002,
            },
            gold_skulltulas: {
                "Test Dungeon GS" = 0x01,
            },
        },
        0x01: TestArea {
            region_name: "Test Region",
            room_clear: {
                ROOM_CLEARED = 0x0000_0001,
            },
            collectible: {
                "Test Area Collectible" = 0x0000_0001,
            },
        },
    }
}

fn main() {
    // Test that the main struct can be default-constructed
    let scene_flags = TestSceneFlags::default();

    // Test that TryFrom<Vec<u8>> is implemented
    // Scene size is 0x1c (28) bytes, num_scenes is 0x65 (101)
    let data: Vec<u8> = vec![0; 0x1c * 0x65];
    let result = TestSceneFlags::try_from(data);
    assert!(result.is_ok());

    // Test that From<&TestSceneFlags> for Vec<u8> is implemented
    let bytes: Vec<u8> = (&scene_flags).into();
    assert_eq!(bytes.len(), 0x1c * 0x65);

    // Test that GoldSkulltulas struct is generated
    let _skulls = GoldSkulltulas::default();
    let skull_data: Vec<u8> = vec![0; 0x18];
    let skull_result = GoldSkulltulas::try_from(skull_data);
    assert!(skull_result.is_ok());

    // Test that individual scene types are generated
    let _ = TestDungeon::default();
    let _ = TestArea::default();

    // Test that chest flag types are generated
    let _ = TestDungeonChests::empty();
}
