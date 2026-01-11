//! Heart piece location mappings.

use std::collections::HashMap;

use crate::mm_flag_mapping::{scenes, MmFlagMapping, MmFlagType};

/// Registers heart piece mappings into the provided map.
pub fn register_heart_pieces(map: &mut HashMap<&'static str, MmFlagMapping>) {
    register_clock_town_hp(map);
    register_termina_field_hp(map);
    register_swamp_area_hp(map);
    register_mountain_area_hp(map);
    register_great_bay_area_hp(map);
    register_ikana_area_hp(map);
    register_ranch_area_hp(map);
    register_moon_trial_hp(map);
}

/// Clock Town Heart Pieces
fn register_clock_town_hp(map: &mut HashMap<&'static str, MmFlagMapping>) {
    add_collectible(
        map,
        "mm_clock_town_platform_hp",
        scenes::CLOCK_TOWN_SOUTH,
        0x0000_0001,
    );
    add_collectible(
        map,
        "mm_clock_town_tree_hp",
        scenes::CLOCK_TOWN_NORTH,
        0x0000_0001,
    );
    add_collectible(
        map,
        "mm_clock_town_keaton_hp",
        scenes::CLOCK_TOWN_NORTH,
        0x0000_0002,
    );
    add_collectible(
        map,
        "mm_clock_town_rosa_sisters_hp",
        scenes::CLOCK_TOWN_WEST,
        0x0000_0001,
    );
    add_collectible(map, "mm_post_office_hp", scenes::POST_OFFICE, 0x0000_0001);
    add_collectible(map, "mm_swordsman_school_hp", 0x26, 0x0000_0002);
    add_collectible(
        map,
        "mm_mayors_office_hp",
        scenes::MAYORS_OFFICE,
        0x0000_0001,
    );
    add_collectible(
        map,
        "mm_chest_game_hp",
        scenes::TREASURE_CHEST_SHOP,
        0x0000_0001,
    );
    add_collectible(
        map,
        "mm_stock_pot_inn_grandma_hp_1",
        scenes::STOCK_POT_INN,
        0x0000_0001,
    );
    add_collectible(
        map,
        "mm_stock_pot_inn_grandma_hp_2",
        scenes::STOCK_POT_INN,
        0x0000_0002,
    );
    add_collectible(
        map,
        "mm_stock_pot_inn_hp",
        scenes::STOCK_POT_INN,
        0x0000_0004,
    );
}

/// Termina Field Heart Pieces
fn register_termina_field_hp(map: &mut HashMap<&'static str, MmFlagMapping>) {
    add_collectible(
        map,
        "mm_termina_field_gossip_stones_hp",
        scenes::TERMINA_FIELD,
        0x0000_0001,
    );
}

/// Southern Swamp Area Heart Pieces
fn register_swamp_area_hp(map: &mut HashMap<&'static str, MmFlagMapping>) {
    add_collectible(
        map,
        "mm_road_to_southern_swamp_hp",
        scenes::ROAD_TO_SOUTHERN_SWAMP,
        0x0000_0001,
    );
    add_collectible(map, "mm_deku_palace_hp", 0x59, 0x0000_0001);
    add_collectible(
        map,
        "mm_southern_swamp_hp",
        scenes::SOUTHERN_SWAMP,
        0x0000_0001,
    );
    add_collectible(map, "mm_woodfall_hp_chest", 0x14, 0x0000_0001);
}

/// Mountain Area Heart Pieces
fn register_mountain_area_hp(map: &mut HashMap<&'static str, MmFlagMapping>) {
    add_collectible(map, "mm_goron_village_hp", 0x6A, 0x0000_0001);
    add_collectible(map, "mm_mountain_village_frog_choir_hp", 0x65, 0x0000_0001);
    add_collectible(map, "mm_path_to_snowhead_hp", 0x21, 0x0000_0001);
}

/// Great Bay Area Heart Pieces
fn register_great_bay_area_hp(map: &mut HashMap<&'static str, MmFlagMapping>) {
    add_collectible(
        map,
        "mm_great_bay_coast_hp",
        scenes::GREAT_BAY_COAST,
        0x0000_0001,
    );
    add_collectible(
        map,
        "mm_great_bay_coast_fisherman_hp",
        scenes::GREAT_BAY_COAST,
        0x0000_0002,
    );
    add_collectible(
        map,
        "mm_laboratory_fish_hp",
        scenes::GREAT_BAY_COAST,
        0x0000_0004,
    );
    add_collectible(
        map,
        "mm_pinnacle_rock_hp",
        scenes::PINNACLE_ROCK,
        0x0000_0001,
    );
    add_collectible(
        map,
        "mm_zora_cape_waterfall_hp",
        scenes::ZORA_CAPE,
        0x0000_0001,
    );
    add_collectible(map, "mm_zora_hall_evan_hp", 0x39, 0x0000_0001);
    add_collectible(map, "mm_zora_hall_scrub_hp", 0x39, 0x0000_0002);
}

/// Ikana Area Heart Pieces
fn register_ikana_area_hp(map: &mut HashMap<&'static str, MmFlagMapping>) {
    add_collectible(map, "mm_ikana_valley_scrub_hp", 0x46, 0x0000_0001);
    add_collectible(map, "mm_ghost_hut_hp", 0x46, 0x0000_0002);
    add_collectible(
        map,
        "mm_beneath_the_graveyard_hp",
        scenes::BENEATH_THE_GRAVEYARD,
        0x0000_0001,
    );
}

/// Ranch Area Heart Pieces
fn register_ranch_area_hp(map: &mut HashMap<&'static str, MmFlagMapping>) {
    add_collectible(map, "mm_doggy_racetrack_hp", 0x62, 0x0000_0001);
}

/// Moon Trial Heart Pieces (stored as chest flags)
fn register_moon_trial_hp(map: &mut HashMap<&'static str, MmFlagMapping>) {
    add_chest(map, "mm_moon_trial_deku_hp", 0x67, 0x0000_0001);
    add_chest(map, "mm_moon_trial_goron_hp", 0x68, 0x0000_0001);
    add_chest(map, "mm_moon_trial_zora_hp", 0x69, 0x0000_0001);
    add_chest(map, "mm_moon_trial_link_hp", 0x66, 0x0000_0004);
}

fn add_collectible(
    map: &mut HashMap<&'static str, MmFlagMapping>,
    location_id: &'static str,
    scene_id: u8,
    flag_bit: u32,
) {
    map.insert(
        location_id,
        MmFlagMapping::mapped(location_id, scene_id, MmFlagType::Collectible, flag_bit),
    );
}

fn add_chest(
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
