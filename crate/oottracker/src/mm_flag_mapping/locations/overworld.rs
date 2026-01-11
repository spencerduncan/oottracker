//! Overworld chest and collectible mappings.
//!
//! Clock Town, Termina Field, and other overworld area chests.

use std::collections::HashMap;

use crate::mm_flag_mapping::{scenes, MmFlagMapping, MmFlagType};

/// Registers overworld mappings into the provided map.
pub fn register_overworld(map: &mut HashMap<&'static str, MmFlagMapping>) {
    register_clock_town(map);
    register_termina_field(map);
    register_swamp_area(map);
    register_mountain_area(map);
    register_great_bay_area(map);
    register_ikana_area(map);
    register_ranch_area(map);
    register_wonder_items(map);
    register_soil_items(map);
}

/// Clock Town area chests
fn register_clock_town(map: &mut HashMap<&'static str, MmFlagMapping>) {
    // Clock Town South (Scene 0x6C)
    add_chest(
        map,
        "mm_clock_town_south_chest_lower",
        scenes::CLOCK_TOWN_SOUTH,
        0x0000_0001,
    );
    add_chest(
        map,
        "mm_clock_town_south_chest_upper",
        scenes::CLOCK_TOWN_SOUTH,
        0x0000_0002,
    );

    // Clock Town East (Scene 0x6E)
    add_chest(
        map,
        "mm_clock_town_silver_rupee_chest",
        scenes::CLOCK_TOWN_EAST,
        0x0000_0001,
    );

    // Astral Observatory (Scene 0x52)
    add_chest(
        map,
        "mm_astral_observatory_passage_chest",
        scenes::ASTRAL_OBSERVATORY,
        0x0000_0001,
    );

    // Stock Pot Inn (Scene 0x4D)
    add_chest(
        map,
        "mm_stock_pot_inn_guest_room_chest",
        scenes::STOCK_POT_INN,
        0x0000_0001,
    );
    add_chest(
        map,
        "mm_stock_pot_inn_staff_room_chest",
        scenes::STOCK_POT_INN,
        0x0000_0002,
    );
}

/// Termina Field chests
fn register_termina_field(map: &mut HashMap<&'static str, MmFlagMapping>) {
    add_chest(
        map,
        "mm_termina_field_water_chest",
        scenes::TERMINA_FIELD,
        0x0000_0001,
    );
    add_chest(
        map,
        "mm_termina_field_tall_grass_chest",
        scenes::TERMINA_FIELD,
        0x0000_0002,
    );
    add_chest(
        map,
        "mm_termina_field_tree_stump_chest",
        scenes::TERMINA_FIELD,
        0x0000_0004,
    );
}

/// Southern Swamp area chests
fn register_swamp_area(map: &mut HashMap<&'static str, MmFlagMapping>) {
    // Deku Palace Grotto (Scene 0x59)
    add_chest(map, "mm_deku_palace_grotto_chest", 0x59, 0x0000_0001);

    // Woodfall (Scene 0x14)
    add_chest(map, "mm_woodfall_entrance_chest", 0x14, 0x0000_0001);
    add_chest(map, "mm_woodfall_near_owl_chest", 0x14, 0x0000_0002);
}

/// Mountain area chests
fn register_mountain_area(map: &mut HashMap<&'static str, MmFlagMapping>) {
    // Lone Peak Shrine / Lens of Truth Cave (Scene 0x17)
    add_chest(map, "mm_lone_peak_shrine_lens_chest", 0x17, 0x0000_0001);
    add_chest(map, "mm_lone_peak_shrine_boulder_chest", 0x17, 0x0000_0002);
    add_chest(
        map,
        "mm_lone_peak_shrine_invisible_chest",
        0x17,
        0x0000_0004,
    );

    // Mountain Village (Scene 0x65)
    add_chest(
        map,
        "mm_mountain_village_waterfall_chest",
        0x65,
        0x0000_0001,
    );

    // Twin Islands Spring (Scene 0x49)
    add_chest(map, "mm_twin_islands_underwater_chest_1", 0x49, 0x0000_0001);
    add_chest(map, "mm_twin_islands_underwater_chest_2", 0x49, 0x0000_0002);
    add_chest(map, "mm_twin_islands_ramp_grotto_chest", 0x49, 0x0000_0004);
    add_chest(
        map,
        "mm_twin_islands_frozen_grotto_chest",
        0x49,
        0x0000_0008,
    );
}

/// Great Bay area chests
fn register_great_bay_area(map: &mut HashMap<&'static str, MmFlagMapping>) {
    // Great Bay Coast (Scene 0x37)
    add_chest(
        map,
        "mm_great_bay_coast_ledge_chest",
        scenes::GREAT_BAY_COAST,
        0x0000_0001,
    );

    // Zora Cape (Scene 0x38)
    add_chest(
        map,
        "mm_zora_cape_underwater_chest",
        scenes::ZORA_CAPE,
        0x0000_0001,
    );
    add_chest(
        map,
        "mm_zora_cape_ledge_chest_1",
        scenes::ZORA_CAPE,
        0x0000_0002,
    );
    add_chest(
        map,
        "mm_zora_cape_ledge_chest_2",
        scenes::ZORA_CAPE,
        0x0000_0004,
    );

    // Pinnacle Rock (Scene 0x3F)
    add_chest(
        map,
        "mm_pinnacle_rock_chest_1",
        scenes::PINNACLE_ROCK,
        0x0000_0001,
    );
    add_chest(
        map,
        "mm_pinnacle_rock_chest_2",
        scenes::PINNACLE_ROCK,
        0x0000_0002,
    );
}

/// Ikana area chests
fn register_ikana_area(map: &mut HashMap<&'static str, MmFlagMapping>) {
    // Road to Ikana (Scene 0x47)
    add_chest(
        map,
        "mm_road_to_ikana_chest",
        scenes::ROAD_TO_IKANA,
        0x0000_0001,
    );

    // Stone Tower Exterior Inverted (Scene 0x0F)
    add_chest(map, "mm_stone_tower_inverted_chest_1", 0x0F, 0x0000_0001);
    add_chest(map, "mm_stone_tower_inverted_chest_2", 0x0F, 0x0000_0002);
    add_chest(map, "mm_stone_tower_inverted_chest_3", 0x0F, 0x0000_0004);
}

/// Ranch area chests
fn register_ranch_area(map: &mut HashMap<&'static str, MmFlagMapping>) {
    // Doggy Racetrack (Scene 0x62)
    add_chest(map, "mm_doggy_racetrack_chest", 0x62, 0x0000_0001);
}

/// Wonder item (target shooting) collectibles
fn register_wonder_items(map: &mut HashMap<&'static str, MmFlagMapping>) {
    // Clock Town South Wonder Items
    add_collectible(
        map,
        "mm_clock_town_south_wonder_item_1",
        scenes::CLOCK_TOWN_SOUTH,
        0x01,
    );
    add_collectible(
        map,
        "mm_clock_town_south_wonder_item_2",
        scenes::CLOCK_TOWN_SOUTH,
        0x02,
    );
    add_collectible(
        map,
        "mm_clock_town_south_wonder_item_3",
        scenes::CLOCK_TOWN_SOUTH,
        0x03,
    );

    // Clock Town East Wonder Items - Target Left
    add_collectible(
        map,
        "mm_clock_town_east_wonder_item_target_left_1",
        scenes::CLOCK_TOWN_EAST,
        0x01,
    );
    add_collectible(
        map,
        "mm_clock_town_east_wonder_item_target_left_2",
        scenes::CLOCK_TOWN_EAST,
        0x02,
    );
    add_collectible(
        map,
        "mm_clock_town_east_wonder_item_target_left_3",
        scenes::CLOCK_TOWN_EAST,
        0x03,
    );

    // Clock Town East Wonder Items - Target Right
    add_collectible(
        map,
        "mm_clock_town_east_wonder_item_target_right_1",
        scenes::CLOCK_TOWN_EAST,
        0x04,
    );
    add_collectible(
        map,
        "mm_clock_town_east_wonder_item_target_right_2",
        scenes::CLOCK_TOWN_EAST,
        0x05,
    );
    add_collectible(
        map,
        "mm_clock_town_east_wonder_item_target_right_3",
        scenes::CLOCK_TOWN_EAST,
        0x06,
    );

    // Clock Town East Wonder Items - Basket
    add_collectible(
        map,
        "mm_clock_town_east_wonder_item_basket_1",
        scenes::CLOCK_TOWN_EAST,
        0x07,
    );
    add_collectible(
        map,
        "mm_clock_town_east_wonder_item_basket_2",
        scenes::CLOCK_TOWN_EAST,
        0x08,
    );
    add_collectible(
        map,
        "mm_clock_town_east_wonder_item_basket_3",
        scenes::CLOCK_TOWN_EAST,
        0x09,
    );

    // Ikana Graveyard Wonder Items
    for i in 1..=12 {
        let location_id = match i {
            1 => "mm_ikana_graveyard_wonder_item_01",
            2 => "mm_ikana_graveyard_wonder_item_02",
            3 => "mm_ikana_graveyard_wonder_item_03",
            4 => "mm_ikana_graveyard_wonder_item_04",
            5 => "mm_ikana_graveyard_wonder_item_05",
            6 => "mm_ikana_graveyard_wonder_item_06",
            7 => "mm_ikana_graveyard_wonder_item_07",
            8 => "mm_ikana_graveyard_wonder_item_08",
            9 => "mm_ikana_graveyard_wonder_item_09",
            10 => "mm_ikana_graveyard_wonder_item_10",
            11 => "mm_ikana_graveyard_wonder_item_11",
            12 => "mm_ikana_graveyard_wonder_item_12",
            _ => unreachable!(),
        };
        add_collectible(map, location_id, scenes::IKANA_GRAVEYARD, i as u32);
    }

    // Romani Ranch Wonder Items - Fence
    for i in 1..=6 {
        let location_id = match i {
            1 => "mm_romani_ranch_wonder_item_fence_1",
            2 => "mm_romani_ranch_wonder_item_fence_2",
            3 => "mm_romani_ranch_wonder_item_fence_3",
            4 => "mm_romani_ranch_wonder_item_fence_4",
            5 => "mm_romani_ranch_wonder_item_fence_5",
            6 => "mm_romani_ranch_wonder_item_fence_6",
            _ => unreachable!(),
        };
        add_collectible(map, location_id, scenes::ROMANI_RANCH, i as u32);
    }

    // Romani Ranch Barn Wonder Items
    add_collectible(
        map,
        "mm_romani_ranch_barn_wonder_item_1",
        scenes::ROMANI_RANCH,
        0x10,
    );
    add_collectible(
        map,
        "mm_romani_ranch_barn_wonder_item_2",
        scenes::ROMANI_RANCH,
        0x11,
    );

    // Cucco Shack Wonder Items
    for i in 1..=6 {
        let location_id = match i {
            1 => "mm_cucco_shack_wonder_item_1",
            2 => "mm_cucco_shack_wonder_item_2",
            3 => "mm_cucco_shack_wonder_item_3",
            4 => "mm_cucco_shack_wonder_item_4",
            5 => "mm_cucco_shack_wonder_item_5",
            6 => "mm_cucco_shack_wonder_item_6",
            _ => unreachable!(),
        };
        add_collectible(map, location_id, scenes::CUCCO_SHACK, i as u32);
    }

    // Termina Field Wonder Items
    add_collectible(
        map,
        "mm_termina_field_wonder_item_hollow_trunk",
        scenes::TERMINA_FIELD,
        0x01,
    );
    add_collectible(
        map,
        "mm_termina_field_wonder_item_fountains_1",
        scenes::TERMINA_FIELD,
        0x02,
    );
    add_collectible(
        map,
        "mm_termina_field_wonder_item_fountains_2",
        scenes::TERMINA_FIELD,
        0x03,
    );
    add_collectible(
        map,
        "mm_termina_field_wonder_item_north_ramp",
        scenes::TERMINA_FIELD,
        0x04,
    );
    add_collectible(
        map,
        "mm_termina_field_wonder_item_west_ramp",
        scenes::TERMINA_FIELD,
        0x05,
    );
    add_collectible(
        map,
        "mm_termina_field_wonder_item_south_west_ramp",
        scenes::TERMINA_FIELD,
        0x06,
    );
    add_collectible(
        map,
        "mm_termina_field_wonder_item_shell_1",
        scenes::TERMINA_FIELD,
        0x07,
    );
    add_collectible(
        map,
        "mm_termina_field_wonder_item_shell_2",
        scenes::TERMINA_FIELD,
        0x08,
    );
    add_collectible(
        map,
        "mm_termina_field_wonder_item_shell_3",
        scenes::TERMINA_FIELD,
        0x09,
    );
    add_collectible(
        map,
        "mm_termina_field_wonder_item_shell_side_1",
        scenes::TERMINA_FIELD,
        0x0A,
    );
    add_collectible(
        map,
        "mm_termina_field_wonder_item_shell_side_2",
        scenes::TERMINA_FIELD,
        0x0B,
    );
    add_collectible(
        map,
        "mm_termina_field_wonder_item_shell_side_3",
        scenes::TERMINA_FIELD,
        0x0C,
    );
    add_collectible(
        map,
        "mm_termina_field_wonder_item_graffiti_1",
        scenes::TERMINA_FIELD,
        0x0D,
    );
    add_collectible(
        map,
        "mm_termina_field_wonder_item_graffiti_2",
        scenes::TERMINA_FIELD,
        0x0E,
    );
    add_collectible(
        map,
        "mm_termina_field_wonder_item_graffiti_3",
        scenes::TERMINA_FIELD,
        0x0F,
    );
}

/// Soil (bug drop) items
fn register_soil_items(map: &mut HashMap<&'static str, MmFlagMapping>) {
    // Deku Palace Soil Items
    add_collectible(map, "mm_deku_palace_soil_item_1", scenes::DEKU_PALACE, 0x10);
    add_collectible(map, "mm_deku_palace_soil_item_2", scenes::DEKU_PALACE, 0x11);
    add_collectible(map, "mm_deku_palace_soil_item_3", scenes::DEKU_PALACE, 0x12);

    // Beans Grotto Soil Items
    add_collectible(
        map,
        "mm_beans_grotto_soil_item_1",
        scenes::TERMINA_FIELD,
        0x20,
    );
    add_collectible(
        map,
        "mm_beans_grotto_soil_item_2",
        scenes::TERMINA_FIELD,
        0x21,
    );
    add_collectible(
        map,
        "mm_beans_grotto_soil_item_3",
        scenes::TERMINA_FIELD,
        0x22,
    );

    // Romani Ranch Soil Items - Days 2-3
    add_collectible(
        map,
        "mm_romani_ranch_soil_days_2_3_item_1",
        scenes::ROMANI_RANCH,
        0x20,
    );
    add_collectible(
        map,
        "mm_romani_ranch_soil_days_2_3_item_2",
        scenes::ROMANI_RANCH,
        0x21,
    );
    add_collectible(
        map,
        "mm_romani_ranch_soil_days_2_3_item_3",
        scenes::ROMANI_RANCH,
        0x22,
    );

    // Termina Field Soil Items - Observatory Area
    add_collectible(
        map,
        "mm_termina_field_soil_observatory_item_1",
        scenes::TERMINA_FIELD,
        0x30,
    );
    add_collectible(
        map,
        "mm_termina_field_soil_observatory_item_2",
        scenes::TERMINA_FIELD,
        0x31,
    );
    add_collectible(
        map,
        "mm_termina_field_soil_observatory_item_3",
        scenes::TERMINA_FIELD,
        0x32,
    );

    // Termina Field Soil Items - Wall Area
    add_collectible(
        map,
        "mm_termina_field_soil_wall_item_1",
        scenes::TERMINA_FIELD,
        0x33,
    );
    add_collectible(
        map,
        "mm_termina_field_soil_wall_item_2",
        scenes::TERMINA_FIELD,
        0x34,
    );
    add_collectible(
        map,
        "mm_termina_field_soil_wall_item_3",
        scenes::TERMINA_FIELD,
        0x35,
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
