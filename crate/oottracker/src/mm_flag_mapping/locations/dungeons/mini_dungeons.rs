//! Mini dungeon chest mappings.
//!
//! Beneath the Well, Ancient Castle of Ikana, Secret Shrine,
//! Pirates Fortress, Beneath the Graveyard, and Spider Houses.

use std::collections::HashMap;

use crate::mm_flag_mapping::{scenes, MmFlagMapping, MmFlagType};

/// Registers mini dungeon chest mappings into the provided map.
pub fn register_mini_dungeons(map: &mut HashMap<&'static str, MmFlagMapping>) {
    register_beneath_the_well(map);
    register_ancient_castle_of_ikana(map);
    register_secret_shrine(map);
    register_pirates_fortress(map);
    register_beneath_the_graveyard(map);
    register_spider_houses(map);
    register_moon_trials(map);
}

/// Beneath the Well (Scene 0x1B)
fn register_beneath_the_well(map: &mut HashMap<&'static str, MmFlagMapping>) {
    add(
        map,
        "mm_beneath_the_well_keese_chest",
        scenes::BENEATH_THE_WELL,
        0x0000_0001,
    );
    add(
        map,
        "mm_beneath_the_well_skulltulla_chest",
        scenes::BENEATH_THE_WELL,
        0x0000_0002,
    );
    add(
        map,
        "mm_beneath_the_well_mirror_shield_chest",
        scenes::BENEATH_THE_WELL,
        0x0000_0004,
    );
    add(
        map,
        "mm_beneath_the_well_compass_chest",
        scenes::BENEATH_THE_WELL,
        0x0000_0008,
    );
    add(
        map,
        "mm_beneath_the_well_map_chest",
        scenes::BENEATH_THE_WELL,
        0x0000_0010,
    );
}

/// Ancient Castle of Ikana (Scene 0x11)
fn register_ancient_castle_of_ikana(map: &mut HashMap<&'static str, MmFlagMapping>) {
    add(
        map,
        "mm_ancient_castle_of_ikana_powder_keg_chest",
        scenes::ANCIENT_CASTLE_OF_IKANA,
        0x0000_0001,
    );
    add(
        map,
        "mm_ancient_castle_of_ikana_compass_chest",
        scenes::ANCIENT_CASTLE_OF_IKANA,
        0x0000_0002,
    );
    add(
        map,
        "mm_ancient_castle_of_ikana_map_chest",
        scenes::ANCIENT_CASTLE_OF_IKANA,
        0x0000_0004,
    );
}

/// Secret Shrine (Scene 0x13)
fn register_secret_shrine(map: &mut HashMap<&'static str, MmFlagMapping>) {
    add(
        map,
        "mm_secret_shrine_dinolfos_chest",
        scenes::IKANA_CANYON_SECRET_SHRINE,
        0x0000_0001,
    );
    add(
        map,
        "mm_secret_shrine_wizzrobe_chest",
        scenes::IKANA_CANYON_SECRET_SHRINE,
        0x0000_0002,
    );
    add(
        map,
        "mm_secret_shrine_wart_chest",
        scenes::IKANA_CANYON_SECRET_SHRINE,
        0x0000_0004,
    );
    add(
        map,
        "mm_secret_shrine_garo_master_chest",
        scenes::IKANA_CANYON_SECRET_SHRINE,
        0x0000_0008,
    );
}

/// Pirates Fortress (Scenes 0x29, 0x2A)
fn register_pirates_fortress(map: &mut HashMap<&'static str, MmFlagMapping>) {
    // Exterior (Scene 0x29)
    add(
        map,
        "mm_pirate_fortress_entrance_chest_1",
        scenes::PIRATES_FORTRESS,
        0x0000_0001,
    );
    add(
        map,
        "mm_pirate_fortress_entrance_chest_2",
        scenes::PIRATES_FORTRESS,
        0x0000_0002,
    );
    add(
        map,
        "mm_pirate_fortress_entrance_chest_3",
        scenes::PIRATES_FORTRESS,
        0x0000_0004,
    );
    add(
        map,
        "mm_pirate_fortress_sewers_chest_1",
        scenes::PIRATES_FORTRESS,
        0x0000_0008,
    );
    add(
        map,
        "mm_pirate_fortress_sewers_chest_2",
        scenes::PIRATES_FORTRESS,
        0x0000_0010,
    );
    add(
        map,
        "mm_pirate_fortress_sewers_chest_3",
        scenes::PIRATES_FORTRESS,
        0x0000_0020,
    );

    // Interior (Scene 0x2A)
    add(
        map,
        "mm_pirate_fortress_interior_lower_chest",
        scenes::PIRATES_FORTRESS_INTERIOR,
        0x0000_0001,
    );
    add(
        map,
        "mm_pirate_fortress_interior_upper_chest",
        scenes::PIRATES_FORTRESS_INTERIOR,
        0x0000_0002,
    );
    add(
        map,
        "mm_pirate_fortress_interior_pot_chest_aquarium_1",
        scenes::PIRATES_FORTRESS_INTERIOR,
        0x0000_0004,
    );
    add(
        map,
        "mm_pirate_fortress_interior_pot_chest_aquarium_2",
        scenes::PIRATES_FORTRESS_INTERIOR,
        0x0000_0008,
    );
    add(
        map,
        "mm_pirate_fortress_interior_pot_chest_aquarium_3",
        scenes::PIRATES_FORTRESS_INTERIOR,
        0x0000_0010,
    );
    add(
        map,
        "mm_pirate_fortress_interior_silver_rupee_chest",
        scenes::PIRATES_FORTRESS_INTERIOR,
        0x0000_0020,
    );
}

/// Beneath the Graveyard (Scene 0x07)
fn register_beneath_the_graveyard(map: &mut HashMap<&'static str, MmFlagMapping>) {
    add(
        map,
        "mm_beneath_the_graveyard_chest",
        scenes::BENEATH_THE_GRAVEYARD,
        0x0000_0001,
    );
    add(
        map,
        "mm_beneath_the_graveyard_dampe_chest",
        scenes::BENEATH_THE_GRAVEYARD,
        0x0000_0002,
    );
}

/// Spider Houses (Scenes 0x27, 0x28)
fn register_spider_houses(map: &mut HashMap<&'static str, MmFlagMapping>) {
    // Oceanside Spider House
    add(
        map,
        "mm_ocean_spider_house_chest_hp",
        scenes::OCEANSIDE_SPIDER_HOUSE,
        0x0000_0001,
    );
}

/// Moon Trial chests
fn register_moon_trials(map: &mut HashMap<&'static str, MmFlagMapping>) {
    add(
        map,
        "mm_moon_trial_link_garo_master_chest",
        0x66, // Moon Link Trial scene
        0x0000_0001,
    );
    add(
        map,
        "mm_moon_trial_link_iron_knuckle_chest",
        0x66,
        0x0000_0002,
    );
}

fn add(
    map: &mut HashMap<&'static str, MmFlagMapping>,
    location_id: &'static str,
    scene_id: u8,
    flag_bit: u32,
) {
    map.insert(
        location_id,
        MmFlagMapping::mapped(location_id, scene_id, MmFlagType::Chest, flag_bit),
    );
}
