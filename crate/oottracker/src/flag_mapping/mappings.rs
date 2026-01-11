//! Static mapping tables for OoT locations.
//!
//! This module contains the large static data tables mapping location IDs
//! to their flag addresses. The data is lazily initialized at runtime.

use std::collections::HashMap;

use once_cell::sync::Lazy;
use ootmm::embedded_data;
use ootmm::region::Game;

use super::scenes as scene;
use super::types::{FlagMapping, FlagType};

// ============================================================================

/// All OoT location IDs extracted from embedded world data.
///
/// This list is generated at compile time from the OoTMM YAML files.
pub(super) static OOT_LOCATION_IDS: Lazy<Vec<&'static str>> = Lazy::new(|| {
    let db = embedded_data::create_world_database()
        .expect("Failed to load world database for location extraction");

    db.locations_for_game(Game::Oot)
        .map(|(loc, _region_id)| {
            // Leak the string to get a 'static lifetime
            // This is intentional - these strings live for the program's lifetime
            Box::leak(loc.id.clone().into_boxed_str()) as &'static str
        })
        .collect()
});

/// HashMap of location ID to FlagMapping for fast lookups.
pub(super) static OOT_MAPPINGS: Lazy<HashMap<&'static str, FlagMapping>> = Lazy::new(|| {
    let mut map = HashMap::new();

    // First, add all stub mappings from world data
    for &loc_id in OOT_LOCATION_IDS.iter() {
        map.insert(loc_id, FlagMapping::stub(loc_id));
    }

    // Then, add known mappings that override the stubs
    // These are derived from the existing scene.rs definitions and OoTMM research

    // ========================================================================
    // DEKU TREE (Scene 0x00)
    // ========================================================================
    add_mapping(
        &mut map,
        "oot_deku_tree_compass_room_side_chest",
        scene::DEKU_TREE,
        FlagType::Chest,
        0x0000_0040,
    );
    add_mapping(
        &mut map,
        "oot_deku_tree_slingshot_side_chest",
        scene::DEKU_TREE,
        FlagType::Chest,
        0x0000_0020,
    );
    add_mapping(
        &mut map,
        "oot_deku_tree_basement_chest",
        scene::DEKU_TREE,
        FlagType::Chest,
        0x0000_0010,
    );
    add_mapping(
        &mut map,
        "oot_deku_tree_map_chest",
        scene::DEKU_TREE,
        FlagType::Chest,
        0x0000_0008,
    );
    add_mapping(
        &mut map,
        "oot_deku_tree_compass_chest",
        scene::DEKU_TREE,
        FlagType::Chest,
        0x0000_0004,
    );
    add_mapping(
        &mut map,
        "oot_deku_tree_slingshot_chest",
        scene::DEKU_TREE,
        FlagType::Chest,
        0x0000_0002,
    );

    // ========================================================================
    // DODONGO'S CAVERN (Scene 0x01)
    // ========================================================================
    add_mapping(
        &mut map,
        "oot_dodongo_cavern_bridge_chest",
        scene::DODONGOS_CAVERN,
        FlagType::Chest,
        0x0000_0400,
    );
    add_mapping(
        &mut map,
        "oot_dodongo_cavern_map_chest",
        scene::DODONGOS_CAVERN,
        FlagType::Chest,
        0x0000_0100,
    );
    add_mapping(
        &mut map,
        "oot_dodongo_cavern_bomb_bag_side_chest",
        scene::DODONGOS_CAVERN,
        FlagType::Chest,
        0x0000_0040,
    );
    add_mapping(
        &mut map,
        "oot_dodongo_cavern_compass_chest",
        scene::DODONGOS_CAVERN,
        FlagType::Chest,
        0x0000_0020,
    );
    add_mapping(
        &mut map,
        "oot_dodongo_cavern_bomb_bag_chest",
        scene::DODONGOS_CAVERN,
        FlagType::Chest,
        0x0000_0010,
    );

    // ========================================================================
    // JABU JABU'S BELLY (Scene 0x02)
    // ========================================================================
    add_mapping(
        &mut map,
        "oot_jabu_jabu_compass_chest",
        scene::JABU_JABU,
        FlagType::Chest,
        0x0000_0010,
    );
    add_mapping(
        &mut map,
        "oot_jabu_jabu_map_chest",
        scene::JABU_JABU,
        FlagType::Chest,
        0x0000_0004,
    );
    add_mapping(
        &mut map,
        "oot_jabu_jabu_boomerang_chest",
        scene::JABU_JABU,
        FlagType::Chest,
        0x0000_0002,
    );

    // ========================================================================
    // FOREST TEMPLE (Scene 0x03)
    // ========================================================================
    add_mapping(
        &mut map,
        "oot_forest_temple_compass",
        scene::FOREST_TEMPLE,
        FlagType::Chest,
        0x0000_8000,
    );
    add_mapping(
        &mut map,
        "oot_forest_temple_boss_key",
        scene::FOREST_TEMPLE,
        FlagType::Chest,
        0x0000_4000,
    );
    add_mapping(
        &mut map,
        "oot_forest_temple_poe_key",
        scene::FOREST_TEMPLE,
        FlagType::Chest,
        0x0000_2000,
    );
    add_mapping(
        &mut map,
        "oot_forest_temple_bow",
        scene::FOREST_TEMPLE,
        FlagType::Chest,
        0x0000_1000,
    );
    add_mapping(
        &mut map,
        "oot_forest_temple_checkerboard",
        scene::FOREST_TEMPLE,
        FlagType::Chest,
        0x0000_0800,
    );
    add_mapping(
        &mut map,
        "oot_forest_temple_well",
        scene::FOREST_TEMPLE,
        FlagType::Chest,
        0x0000_0200,
    );
    add_mapping(
        &mut map,
        "oot_forest_temple_antichamber",
        scene::FOREST_TEMPLE,
        FlagType::Chest,
        0x0000_0080,
    );
    add_mapping(
        &mut map,
        "oot_forest_temple_garden",
        scene::FOREST_TEMPLE,
        FlagType::Chest,
        0x0000_0020,
    );
    add_mapping(
        &mut map,
        "oot_forest_temple_maze",
        scene::FOREST_TEMPLE,
        FlagType::Chest,
        0x0000_0010,
    );
    add_mapping(
        &mut map,
        "oot_forest_temple_tree_small_key",
        scene::FOREST_TEMPLE,
        FlagType::Chest,
        0x0000_0008,
    );
    add_mapping(
        &mut map,
        "oot_forest_temple_floormaster",
        scene::FOREST_TEMPLE,
        FlagType::Chest,
        0x0000_0004,
    );
    add_mapping(
        &mut map,
        "oot_forest_temple_map",
        scene::FOREST_TEMPLE,
        FlagType::Chest,
        0x0000_0002,
    );
    add_mapping(
        &mut map,
        "oot_forest_temple_mini_boss_key",
        scene::FOREST_TEMPLE,
        FlagType::Chest,
        0x0000_0001,
    );

    // ========================================================================
    // FIRE TEMPLE (Scene 0x04)
    // ========================================================================
    add_mapping(
        &mut map,
        "oot_fire_temple_scarecrow_chest",
        scene::FIRE_TEMPLE,
        FlagType::Chest,
        0x0000_2000,
    );
    add_mapping(
        &mut map,
        "oot_fire_temple_boss_key_chest",
        scene::FIRE_TEMPLE,
        FlagType::Chest,
        0x0000_1000,
    );
    add_mapping(
        &mut map,
        "oot_fire_temple_above_maze_chest",
        scene::FIRE_TEMPLE,
        FlagType::Chest,
        0x0000_0800,
    );
    add_mapping(
        &mut map,
        "oot_fire_temple_lava_room_south_jail_chest",
        scene::FIRE_TEMPLE,
        FlagType::Chest,
        0x0000_0400,
    );
    add_mapping(
        &mut map,
        "oot_fire_temple_jail_1_chest",
        scene::FIRE_TEMPLE,
        FlagType::Chest,
        0x0000_0200,
    );
    add_mapping(
        &mut map,
        "oot_fire_temple_maze_jail_chest",
        scene::FIRE_TEMPLE,
        FlagType::Chest,
        0x0000_0100,
    );
    add_mapping(
        &mut map,
        "oot_fire_temple_lava_room_north_jail_chest",
        scene::FIRE_TEMPLE,
        FlagType::Chest,
        0x0000_0080,
    );
    add_mapping(
        &mut map,
        "oot_fire_temple_below_maze_chest",
        scene::FIRE_TEMPLE,
        FlagType::Chest,
        0x0000_0040,
    );
    add_mapping(
        &mut map,
        "oot_fire_temple_maze_chest",
        scene::FIRE_TEMPLE,
        FlagType::Chest,
        0x0000_0020,
    );
    add_mapping(
        &mut map,
        "oot_fire_temple_boss_key_side_chest",
        scene::FIRE_TEMPLE,
        FlagType::Chest,
        0x0000_0010,
    );

    // ========================================================================
    // WATER TEMPLE (Scene 0x05)
    // ========================================================================
    add_mapping(
        &mut map,
        "oot_water_temple_dragon_chest",
        scene::WATER_TEMPLE,
        FlagType::Chest,
        0x0000_0400,
    );
    add_mapping(
        &mut map,
        "oot_water_temple_shell_chest",
        scene::WATER_TEMPLE,
        FlagType::Chest,
        0x0000_0200,
    );
    add_mapping(
        &mut map,
        "oot_water_temple_corridor_chest",
        scene::WATER_TEMPLE,
        FlagType::Chest,
        0x0000_0100,
    );
    add_mapping(
        &mut map,
        "oot_water_temple_bombable_chest",
        scene::WATER_TEMPLE,
        FlagType::Chest,
        0x0000_0080,
    );
    add_mapping(
        &mut map,
        "oot_water_temple_boss_key_chest",
        scene::WATER_TEMPLE,
        FlagType::Chest,
        0x0000_0040,
    );
    // Central Pillar Chest - 0x0000_0020 (Boss Key in scene.rs, but different in YAML)
    add_mapping(
        &mut map,
        "oot_water_temple_river_chest",
        scene::WATER_TEMPLE,
        FlagType::Chest,
        0x0000_0008,
    );

    // ========================================================================
    // SPIRIT TEMPLE (Scene 0x06)
    // ========================================================================
    add_mapping(
        &mut map,
        "oot_spirit_temple_adult_invisible_1",
        scene::SPIRIT_TEMPLE,
        FlagType::Chest,
        0x0020_0000,
    );
    add_mapping(
        &mut map,
        "oot_spirit_temple_adult_invisible_2",
        scene::SPIRIT_TEMPLE,
        FlagType::Chest,
        0x0010_0000,
    );
    add_mapping(
        &mut map,
        "oot_spirit_temple_adult_topmost_sun_on_wall",
        scene::SPIRIT_TEMPLE,
        FlagType::Chest,
        0x0004_0000,
    );
    add_mapping(
        &mut map,
        "oot_spirit_temple_statue_upper_right",
        scene::SPIRIT_TEMPLE,
        FlagType::Chest,
        0x0000_8000,
    );
    add_mapping(
        &mut map,
        "oot_spirit_temple_adult_suns_on_wall_1",
        scene::SPIRIT_TEMPLE,
        FlagType::Chest,
        0x0000_4000,
    );
    add_mapping(
        &mut map,
        "oot_spirit_temple_adult_suns_on_wall_2",
        scene::SPIRIT_TEMPLE,
        FlagType::Chest,
        0x0000_2000,
    );
    add_mapping(
        &mut map,
        "oot_spirit_temple_child_climb_2",
        scene::SPIRIT_TEMPLE,
        FlagType::Chest,
        0x0000_1000,
    );
    add_mapping(
        &mut map,
        "oot_spirit_temple_adult_boss_key_chest",
        scene::SPIRIT_TEMPLE,
        FlagType::Chest,
        0x0000_0400,
    );
    add_mapping(
        &mut map,
        "oot_spirit_temple_child_second_chest",
        scene::SPIRIT_TEMPLE,
        FlagType::Chest,
        0x0000_0100,
    );
    add_mapping(
        &mut map,
        "oot_spirit_temple_adult_lullaby",
        scene::SPIRIT_TEMPLE,
        FlagType::Chest,
        0x0000_0080,
    );
    add_mapping(
        &mut map,
        "oot_spirit_temple_child_climb_1",
        scene::SPIRIT_TEMPLE,
        FlagType::Chest,
        0x0000_0040,
    );
    add_mapping(
        &mut map,
        "oot_spirit_temple_statue_hands",
        scene::SPIRIT_TEMPLE,
        FlagType::Chest,
        0x0000_0020,
    );
    add_mapping(
        &mut map,
        "oot_spirit_temple_adult_silver_rupees",
        scene::SPIRIT_TEMPLE,
        FlagType::Chest,
        0x0000_0010,
    );
    add_mapping(
        &mut map,
        "oot_spirit_temple_statue_base",
        scene::SPIRIT_TEMPLE,
        FlagType::Chest,
        0x0000_0008,
    );
    add_mapping(
        &mut map,
        "oot_spirit_temple_sun_block_room_torches",
        scene::SPIRIT_TEMPLE,
        FlagType::Chest,
        0x0000_0004,
    );
    add_mapping(
        &mut map,
        "oot_spirit_temple_child_first_chest",
        scene::SPIRIT_TEMPLE,
        FlagType::Chest,
        0x0000_0002,
    );

    // ========================================================================
    // SHADOW TEMPLE (Scene 0x07)
    // ========================================================================
    add_mapping(
        &mut map,
        "oot_shadow_temple_spinning_blades_invisible",
        scene::SHADOW_TEMPLE,
        FlagType::Chest,
        0x0040_0000,
    );
    add_mapping(
        &mut map,
        "oot_shadow_temple_wind_room_hint",
        scene::SHADOW_TEMPLE,
        FlagType::Chest,
        0x0020_0000,
    );
    add_mapping(
        &mut map,
        "oot_shadow_temple_after_wind_invisible",
        scene::SHADOW_TEMPLE,
        FlagType::Chest,
        0x0010_0000,
    );
    add_mapping(
        &mut map,
        "oot_shadow_temple_invisible_floormaster",
        scene::SHADOW_TEMPLE,
        FlagType::Chest,
        0x0000_2000,
    );
    add_mapping(
        &mut map,
        "oot_shadow_temple_spinning_blades_visible",
        scene::SHADOW_TEMPLE,
        FlagType::Chest,
        0x0000_1000,
    );
    add_mapping(
        &mut map,
        "oot_shadow_temple_boss_key_room_2",
        scene::SHADOW_TEMPLE,
        FlagType::Chest,
        0x0000_0800,
    );
    add_mapping(
        &mut map,
        "oot_shadow_temple_boss_key_room_1",
        scene::SHADOW_TEMPLE,
        FlagType::Chest,
        0x0000_0400,
    );
    add_mapping(
        &mut map,
        "oot_shadow_temple_invisible_spike_room",
        scene::SHADOW_TEMPLE,
        FlagType::Chest,
        0x0000_0200,
    );
    add_mapping(
        &mut map,
        "oot_shadow_temple_after_wind",
        scene::SHADOW_TEMPLE,
        FlagType::Chest,
        0x0000_0100,
    );
    add_mapping(
        &mut map,
        "oot_shadow_temple_hover_boots",
        scene::SHADOW_TEMPLE,
        FlagType::Chest,
        0x0000_0080,
    );
    add_mapping(
        &mut map,
        "oot_shadow_temple_falling_spikes_upper_1",
        scene::SHADOW_TEMPLE,
        FlagType::Chest,
        0x0000_0040,
    );
    add_mapping(
        &mut map,
        "oot_shadow_temple_falling_spikes_lower",
        scene::SHADOW_TEMPLE,
        FlagType::Chest,
        0x0000_0020,
    );
    add_mapping(
        &mut map,
        "oot_shadow_temple_falling_spikes_upper_2",
        scene::SHADOW_TEMPLE,
        FlagType::Chest,
        0x0000_0010,
    );
    add_mapping(
        &mut map,
        "oot_shadow_temple_compass",
        scene::SHADOW_TEMPLE,
        FlagType::Chest,
        0x0000_0008,
    );
    add_mapping(
        &mut map,
        "oot_shadow_temple_silver_rupees",
        scene::SHADOW_TEMPLE,
        FlagType::Chest,
        0x0000_0004,
    );
    add_mapping(
        &mut map,
        "oot_shadow_temple_map",
        scene::SHADOW_TEMPLE,
        FlagType::Chest,
        0x0000_0002,
    );

    // ========================================================================
    // BOTTOM OF THE WELL (Scene 0x08)
    // ========================================================================
    add_mapping(
        &mut map,
        "oot_bottom_of_the_well_blood_chest",
        scene::BOTTOM_OF_THE_WELL,
        FlagType::Chest,
        0x0010_0000,
    );
    add_mapping(
        &mut map,
        "oot_bottom_of_the_well_underwater_2",
        scene::BOTTOM_OF_THE_WELL,
        FlagType::Chest,
        0x0001_0000,
    );
    add_mapping(
        &mut map,
        "oot_bottom_of_the_well_east_cage",
        scene::BOTTOM_OF_THE_WELL,
        FlagType::Chest,
        0x0000_4000,
    );
    add_mapping(
        &mut map,
        "oot_bottom_of_the_well_pits",
        scene::BOTTOM_OF_THE_WELL,
        FlagType::Chest,
        0x0000_1000,
    );
    add_mapping(
        &mut map,
        "oot_bottom_of_the_well_east",
        scene::BOTTOM_OF_THE_WELL,
        FlagType::Chest,
        0x0000_0400,
    );
    add_mapping(
        &mut map,
        "oot_bottom_of_the_well_underwater",
        scene::BOTTOM_OF_THE_WELL,
        FlagType::Chest,
        0x0000_0200,
    );
    add_mapping(
        &mut map,
        "oot_bottom_of_the_well_front_west",
        scene::BOTTOM_OF_THE_WELL,
        FlagType::Chest,
        0x0000_0100,
    );
    add_mapping(
        &mut map,
        "oot_bottom_of_the_well_map",
        scene::BOTTOM_OF_THE_WELL,
        FlagType::Chest,
        0x0000_0080,
    );
    add_mapping(
        &mut map,
        "oot_bottom_of_the_well_back_west",
        scene::BOTTOM_OF_THE_WELL,
        FlagType::Chest,
        0x0000_0020,
    );
    add_mapping(
        &mut map,
        "oot_bottom_of_the_well_under_debris",
        scene::BOTTOM_OF_THE_WELL,
        FlagType::Chest,
        0x0000_0010,
    );
    add_mapping(
        &mut map,
        "oot_bottom_of_the_well_lens",
        scene::BOTTOM_OF_THE_WELL,
        FlagType::Chest,
        0x0000_0008,
    );
    add_mapping(
        &mut map,
        "oot_bottom_of_the_well_lens_side_chest",
        scene::BOTTOM_OF_THE_WELL,
        FlagType::Chest,
        0x0000_0004,
    );
    add_mapping(
        &mut map,
        "oot_bottom_of_the_well_compass",
        scene::BOTTOM_OF_THE_WELL,
        FlagType::Chest,
        0x0000_0002,
    );

    // ========================================================================
    // ICE CAVERN (Scene 0x09)
    // ========================================================================
    add_mapping(
        &mut map,
        "oot_ice_cavern_rupee_ice",
        scene::ICE_CAVERN,
        FlagType::Chest,
        0x0000_0004,
    );
    add_mapping(
        &mut map,
        "oot_ice_cavern_compass",
        scene::ICE_CAVERN,
        FlagType::Chest,
        0x0000_0002,
    );
    add_mapping(
        &mut map,
        "oot_ice_cavern_map",
        scene::ICE_CAVERN,
        FlagType::Chest,
        0x0000_0001,
    );

    // ========================================================================
    // GANON'S CASTLE TOWER (Scene 0x0A)
    // ========================================================================
    // Boss Key chest - scene.rs shows 0x0000_0800

    // ========================================================================
    // GERUDO TRAINING GROUND (Scene 0x0B)
    // ========================================================================
    add_mapping(
        &mut map,
        "oot_gerudo_training_maze_chest_4",
        scene::GERUDO_TRAINING_GROUND,
        FlagType::Chest,
        0x0010_0000,
    );
    add_mapping(
        &mut map,
        "oot_gerudo_training_maze_side_chest_1",
        scene::GERUDO_TRAINING_GROUND,
        FlagType::Chest,
        0x0008_0000,
    );
    add_mapping(
        &mut map,
        "oot_gerudo_training_maze_chest_3",
        scene::GERUDO_TRAINING_GROUND,
        FlagType::Chest,
        0x0004_0000,
    );
    add_mapping(
        &mut map,
        "oot_gerudo_training_maze_chest_2",
        scene::GERUDO_TRAINING_GROUND,
        FlagType::Chest,
        0x0002_0000,
    );
    add_mapping(
        &mut map,
        "oot_gerudo_training_maze_side_chest_2",
        scene::GERUDO_TRAINING_GROUND,
        FlagType::Chest,
        0x0001_0000,
    );
    add_mapping(
        &mut map,
        "oot_gerudo_training_maze_chest_1",
        scene::GERUDO_TRAINING_GROUND,
        FlagType::Chest,
        0x0000_1000,
    );

    // ========================================================================
    // DODONGO'S CAVERN BOSS (Scene 0x12)
    // ========================================================================
    add_mapping(
        &mut map,
        "oot_dodongo_cavern_boss_chest",
        scene::DODONGOS_CAVERN_BOSS,
        FlagType::Chest,
        0x0000_0001,
    );

    // ========================================================================
    // GANON'S CASTLE (Scene 0x0D)
    // ========================================================================
    add_mapping(
        &mut map,
        "oot_ganon_castle_spirit_chest_2",
        scene::GANONS_CASTLE,
        FlagType::Chest,
        0x0010_0000,
    );
    add_mapping(
        &mut map,
        "oot_ganon_castle_spirit_chest_1",
        scene::GANONS_CASTLE,
        FlagType::Chest,
        0x0004_0000,
    );
    add_mapping(
        &mut map,
        "oot_ganon_castle_light_chest_lullaby",
        scene::GANONS_CASTLE,
        FlagType::Chest,
        0x0002_0000,
    );
    add_mapping(
        &mut map,
        "oot_ganon_castle_light_chest_center",
        scene::GANONS_CASTLE,
        FlagType::Chest,
        0x0001_0000,
    );
    add_mapping(
        &mut map,
        "oot_ganon_castle_light_chest_around_3",
        scene::GANONS_CASTLE,
        FlagType::Chest,
        0x0000_8000,
    );
    add_mapping(
        &mut map,
        "oot_ganon_castle_light_chest_around_1",
        scene::GANONS_CASTLE,
        FlagType::Chest,
        0x0000_4000,
    );
    add_mapping(
        &mut map,
        "oot_ganon_castle_light_chest_around_6",
        scene::GANONS_CASTLE,
        FlagType::Chest,
        0x0000_2000,
    );
    add_mapping(
        &mut map,
        "oot_ganon_castle_light_chest_around_4",
        scene::GANONS_CASTLE,
        FlagType::Chest,
        0x0000_1000,
    );
    add_mapping(
        &mut map,
        "oot_ganon_castle_light_chest_around_5",
        scene::GANONS_CASTLE,
        FlagType::Chest,
        0x0000_0800,
    );
    add_mapping(
        &mut map,
        "oot_ganon_castle_light_chest_around_2",
        scene::GANONS_CASTLE,
        FlagType::Chest,
        0x0000_0400,
    );
    add_mapping(
        &mut map,
        "oot_ganon_castle_forest_chest",
        scene::GANONS_CASTLE,
        FlagType::Chest,
        0x0000_0200,
    );
    add_mapping(
        &mut map,
        "oot_ganon_castle_shadow_chest_1",
        scene::GANONS_CASTLE,
        FlagType::Chest,
        0x0000_0100,
    );
    add_mapping(
        &mut map,
        "oot_ganon_castle_water_chest_1",
        scene::GANONS_CASTLE,
        FlagType::Chest,
        0x0000_0080,
    );
    add_mapping(
        &mut map,
        "oot_ganon_castle_water_chest_2",
        scene::GANONS_CASTLE,
        FlagType::Chest,
        0x0000_0040,
    );
    add_mapping(
        &mut map,
        "oot_ganon_castle_shadow_chest_2",
        scene::GANONS_CASTLE,
        FlagType::Chest,
        0x0000_0020,
    );

    // ========================================================================
    // TREASURE CHEST GAME (Scene 0x10)
    // ========================================================================
    // The Treasure Chest Game has 5 rooms with 2 chests each, plus the final HP chest.
    // Flags 0x01-0x200 are for room chests, 0x400 is the HP reward.
    // Note: The "buy_key" location is handled separately (shop/NPC interaction).
    add_mapping(
        &mut map,
        "oot_treasure_chest_game_room_1_chest_left",
        scene::TREASURE_CHEST_GAME,
        FlagType::Chest,
        0x0000_0001,
    );
    add_mapping(
        &mut map,
        "oot_treasure_chest_game_room_1_chest_right",
        scene::TREASURE_CHEST_GAME,
        FlagType::Chest,
        0x0000_0002,
    );
    add_mapping(
        &mut map,
        "oot_treasure_chest_game_room_2_chest_left",
        scene::TREASURE_CHEST_GAME,
        FlagType::Chest,
        0x0000_0004,
    );
    add_mapping(
        &mut map,
        "oot_treasure_chest_game_room_2_chest_right",
        scene::TREASURE_CHEST_GAME,
        FlagType::Chest,
        0x0000_0008,
    );
    add_mapping(
        &mut map,
        "oot_treasure_chest_game_room_3_chest_left",
        scene::TREASURE_CHEST_GAME,
        FlagType::Chest,
        0x0000_0010,
    );
    add_mapping(
        &mut map,
        "oot_treasure_chest_game_room_3_chest_right",
        scene::TREASURE_CHEST_GAME,
        FlagType::Chest,
        0x0000_0020,
    );
    add_mapping(
        &mut map,
        "oot_treasure_chest_game_room_4_chest_left",
        scene::TREASURE_CHEST_GAME,
        FlagType::Chest,
        0x0000_0040,
    );
    add_mapping(
        &mut map,
        "oot_treasure_chest_game_room_4_chest_right",
        scene::TREASURE_CHEST_GAME,
        FlagType::Chest,
        0x0000_0080,
    );
    add_mapping(
        &mut map,
        "oot_treasure_chest_game_room_5_chest_left",
        scene::TREASURE_CHEST_GAME,
        FlagType::Chest,
        0x0000_0100,
    );
    add_mapping(
        &mut map,
        "oot_treasure_chest_game_room_5_chest_right",
        scene::TREASURE_CHEST_GAME,
        FlagType::Chest,
        0x0000_0200,
    );
    add_mapping(
        &mut map,
        "oot_treasure_chest_game_hp",
        scene::TREASURE_CHEST_GAME,
        FlagType::Chest,
        0x0000_0400,
    );

    // ========================================================================
    // KF MIDO'S HOUSE (Scene 0x28)
    // ========================================================================
    // Mido's House chest IDs - checking YAML for exact naming

    // ========================================================================
    // GROTTOS (Scene 0x3E) - Shared scene for all grottos
    // ========================================================================
    add_mapping(
        &mut map,
        "oot_dmc_upper_grotto_chest",
        scene::GROTTOS,
        FlagType::Chest,
        0x0400_0000,
    );
    add_mapping(
        &mut map,
        "oot_dmt_storms_grotto_chest",
        scene::GROTTOS,
        FlagType::Chest,
        0x0040_0000,
    );
    add_mapping(
        &mut map,
        "oot_lw_near_shortcuts_grotto_chest",
        scene::GROTTOS,
        FlagType::Chest,
        0x0010_0000,
    );
    add_mapping(
        &mut map,
        "oot_sfm_wolfos_grotto_chest",
        scene::GROTTOS,
        FlagType::Chest,
        0x0002_0000,
    );
    add_mapping(
        &mut map,
        "oot_kf_storms_grotto_chest",
        scene::GROTTOS,
        FlagType::Chest,
        0x0000_1000,
    );
    add_mapping(
        &mut map,
        "oot_kak_redead_grotto_chest",
        scene::GROTTOS,
        FlagType::Chest,
        0x0000_0400,
    );
    add_mapping(
        &mut map,
        "oot_zr_open_grotto_chest",
        scene::GROTTOS,
        FlagType::Chest,
        0x0000_0200,
    );
    add_mapping(
        &mut map,
        "oot_kak_open_grotto_chest",
        scene::GROTTOS,
        FlagType::Chest,
        0x0000_0100,
    );
    add_mapping(
        &mut map,
        "oot_hf_open_grotto_chest",
        scene::GROTTOS,
        FlagType::Chest,
        0x0000_0008,
    );
    add_mapping(
        &mut map,
        "oot_hf_southeast_grotto_chest",
        scene::GROTTOS,
        FlagType::Chest,
        0x0000_0004,
    );
    add_mapping(
        &mut map,
        "oot_hf_near_market_grotto_chest",
        scene::GROTTOS,
        FlagType::Chest,
        0x0000_0001,
    );

    // ========================================================================
    // GRAVEYARD GRAVES (Scenes 0x3F, 0x40, 0x41, 0x48)
    // ========================================================================
    add_mapping(
        &mut map,
        "oot_graveyard_heart_piece_grave_chest",
        scene::GRAVE_HEART_PIECE,
        FlagType::Chest,
        0x0000_0001,
    );
    add_mapping(
        &mut map,
        "oot_graveyard_shield_grave_chest",
        scene::GRAVE_SHIELD,
        FlagType::Chest,
        0x0000_0001,
    );
    add_mapping(
        &mut map,
        "oot_graveyard_royal_tomb_chest",
        scene::ROYAL_FAMILYS_TOMB,
        FlagType::Chest,
        0x0000_0001,
    );
    add_mapping(
        &mut map,
        "oot_graveyard_hookshot_chest",
        scene::WINDMILL_AND_DAMPES_GRAVE,
        FlagType::Chest,
        0x0000_0001,
    );

    // ========================================================================
    // KOKIRI FOREST (Scene 0x55)
    // ========================================================================
    add_mapping(
        &mut map,
        "oot_kokiri_forest_kokiri_sword_chest",
        scene::KOKIRI_FOREST,
        FlagType::Chest,
        0x0000_0001,
    );

    // ========================================================================
    // LAKE HYLIA (Scene 0x57)
    // ========================================================================
    // LH Sun chest - scene 0x57

    // ========================================================================
    // ZORA'S DOMAIN (Scene 0x58)
    // ========================================================================
    add_mapping(
        &mut map,
        "oot_zora_domain_waterfall_chest",
        scene::ZORAS_DOMAIN,
        FlagType::Chest,
        0x0000_0001,
    );

    // ========================================================================
    // GERUDO VALLEY (Scene 0x5A)
    // ========================================================================
    add_mapping(
        &mut map,
        "oot_gerudo_valley_chest",
        scene::GERUDO_VALLEY,
        FlagType::Chest,
        0x0000_0001,
    );

    // ========================================================================
    // DESERT COLOSSUS (Scene 0x5C) - Spirit Temple exterior chests
    // ========================================================================
    add_mapping(
        &mut map,
        "oot_spirit_temple_silver_gauntlets",
        scene::DESERT_COLOSSUS,
        FlagType::Chest,
        0x0000_0800,
    );
    add_mapping(
        &mut map,
        "oot_spirit_temple_mirror_shield",
        scene::DESERT_COLOSSUS,
        FlagType::Chest,
        0x0000_0200,
    );

    // ========================================================================
    // GERUDO FORTRESS (Scene 0x5D)
    // ========================================================================
    add_mapping(
        &mut map,
        "oot_gerudo_fortress_chest",
        scene::GERUDO_FORTRESS,
        FlagType::Chest,
        0x0000_0001,
    );

    // ========================================================================
    // HAUNTED WASTELAND (Scene 0x5E)
    // ========================================================================
    add_mapping(
        &mut map,
        "oot_haunted_wasteland_chest",
        scene::HAUNTED_WASTELAND,
        FlagType::Chest,
        0x0000_0001,
    );

    // ========================================================================
    // DEATH MOUNTAIN TRAIL (Scene 0x60)
    // ========================================================================
    add_mapping(
        &mut map,
        "oot_death_mountain_trail_chest",
        scene::DEATH_MOUNTAIN_TRAIL,
        FlagType::Chest,
        0x0000_0002,
    );

    // ========================================================================
    // GORON CITY (Scene 0x62)
    // ========================================================================
    // Goron City doesn't have maze chests in YAML - different naming

    // ========================================================================
    // Gold Skulltulas
    // ========================================================================
    //
    // Gold Skulltula flags are stored in a 24-byte (0x18) section at save
    // offset 0x0E9C. Each scene that has skulltulas uses a specific byte,
    // and each skulltula is a single bit within that byte.
    //
    // The byte offset within the GS section is calculated from scene ID:
    //   byte_offset = (scene_id + 3) - 2 * (scene_id % 4)
    //
    // For GoldSkulltula mappings, scene_id identifies the area/dungeon,
    // and flag_bit is the bit position (0x01, 0x02, 0x04, etc.) within
    // that area's byte.

    // --- Deku Tree (Scene 0x00) ---
    add_skulltula_mapping(&mut map, "oot_deku_tree_gs_compass", scene::DEKU_TREE, 0x01);
    add_skulltula_mapping(
        &mut map,
        "oot_deku_tree_gs_basement_gate",
        scene::DEKU_TREE,
        0x02,
    );
    add_skulltula_mapping(
        &mut map,
        "oot_deku_tree_gs_basement_vines",
        scene::DEKU_TREE,
        0x04,
    );
    add_skulltula_mapping(
        &mut map,
        "oot_deku_tree_gs_basement_back_room",
        scene::DEKU_TREE,
        0x08,
    );

    // --- Dodongo's Cavern (Scene 0x01) ---
    add_skulltula_mapping(
        &mut map,
        "oot_dodongo_cavern_gs_side_room",
        scene::DODONGOS_CAVERN,
        0x01,
    );
    add_skulltula_mapping(
        &mut map,
        "oot_dodongo_cavern_gs_stairs_vines",
        scene::DODONGOS_CAVERN,
        0x02,
    );
    add_skulltula_mapping(
        &mut map,
        "oot_dodongo_cavern_gs_stairs_top",
        scene::DODONGOS_CAVERN,
        0x04,
    );
    add_skulltula_mapping(
        &mut map,
        "oot_dodongo_cavern_gs_scarecrow",
        scene::DODONGOS_CAVERN,
        0x08,
    );
    add_skulltula_mapping(
        &mut map,
        "oot_dodongo_cavern_gs_near_boss",
        scene::DODONGOS_CAVERN,
        0x10,
    );

    // --- Jabu Jabu's Belly (Scene 0x02) ---
    add_skulltula_mapping(
        &mut map,
        "oot_jabu_jabu_gs_bottom_lower",
        scene::JABU_JABU,
        0x01,
    );
    add_skulltula_mapping(
        &mut map,
        "oot_jabu_jabu_gs_bottom_upper",
        scene::JABU_JABU,
        0x02,
    );
    add_skulltula_mapping(
        &mut map,
        "oot_jabu_jabu_gs_water_switch",
        scene::JABU_JABU,
        0x04,
    );
    add_skulltula_mapping(
        &mut map,
        "oot_jabu_jabu_gs_near_boss",
        scene::JABU_JABU,
        0x08,
    );

    // --- Forest Temple (Scene 0x03) ---
    add_skulltula_mapping(
        &mut map,
        "oot_forest_temple_gs_entrance",
        scene::FOREST_TEMPLE,
        0x01,
    );
    add_skulltula_mapping(
        &mut map,
        "oot_forest_temple_gs_main",
        scene::FOREST_TEMPLE,
        0x02,
    );
    add_skulltula_mapping(
        &mut map,
        "oot_forest_temple_gs_garden_west",
        scene::FOREST_TEMPLE,
        0x04,
    );
    add_skulltula_mapping(
        &mut map,
        "oot_forest_temple_gs_garden_east",
        scene::FOREST_TEMPLE,
        0x08,
    );
    add_skulltula_mapping(
        &mut map,
        "oot_forest_temple_gs_antichamber",
        scene::FOREST_TEMPLE,
        0x10,
    );

    // --- Fire Temple (Scene 0x04) ---
    add_skulltula_mapping(
        &mut map,
        "oot_fire_temple_gs_hammer_statues",
        scene::FIRE_TEMPLE,
        0x01,
    );
    add_skulltula_mapping(
        &mut map,
        "oot_fire_temple_gs_lava_side_room",
        scene::FIRE_TEMPLE,
        0x02,
    );
    add_skulltula_mapping(
        &mut map,
        "oot_fire_temple_gs_maze",
        scene::FIRE_TEMPLE,
        0x04,
    );
    add_skulltula_mapping(
        &mut map,
        "oot_fire_temple_gs_scarecrow_wall",
        scene::FIRE_TEMPLE,
        0x08,
    );
    add_skulltula_mapping(
        &mut map,
        "oot_fire_temple_gs_scarecrow_top",
        scene::FIRE_TEMPLE,
        0x10,
    );

    // --- Water Temple (Scene 0x05) ---
    add_skulltula_mapping(
        &mut map,
        "oot_water_temple_gs_center",
        scene::WATER_TEMPLE,
        0x01,
    );
    add_skulltula_mapping(
        &mut map,
        "oot_water_temple_gs_waterfalls",
        scene::WATER_TEMPLE,
        0x02,
    );
    add_skulltula_mapping(
        &mut map,
        "oot_water_temple_gs_large_pit",
        scene::WATER_TEMPLE,
        0x04,
    );
    add_skulltula_mapping(
        &mut map,
        "oot_water_temple_gs_river",
        scene::WATER_TEMPLE,
        0x08,
    );
    add_skulltula_mapping(
        &mut map,
        "oot_water_temple_gs_cage",
        scene::WATER_TEMPLE,
        0x10,
    );

    // --- Spirit Temple (Scene 0x06) ---
    add_skulltula_mapping(
        &mut map,
        "oot_spirit_temple_gs_child_fence",
        scene::SPIRIT_TEMPLE,
        0x01,
    );
    add_skulltula_mapping(
        &mut map,
        "oot_spirit_temple_gs_child_climb",
        scene::SPIRIT_TEMPLE,
        0x02,
    );
    add_skulltula_mapping(
        &mut map,
        "oot_spirit_temple_gs_iron_knuckle",
        scene::SPIRIT_TEMPLE,
        0x04,
    );
    add_skulltula_mapping(
        &mut map,
        "oot_spirit_temple_gs_boulders",
        scene::SPIRIT_TEMPLE,
        0x08,
    );
    add_skulltula_mapping(
        &mut map,
        "oot_spirit_temple_gs_statue",
        scene::SPIRIT_TEMPLE,
        0x10,
    );

    // --- Shadow Temple (Scene 0x07) ---
    add_skulltula_mapping(
        &mut map,
        "oot_shadow_temple_gs_invisible_scythe",
        scene::SHADOW_TEMPLE,
        0x01,
    );
    add_skulltula_mapping(
        &mut map,
        "oot_shadow_temple_gs_falling_spikes",
        scene::SHADOW_TEMPLE,
        0x02,
    );
    add_skulltula_mapping(
        &mut map,
        "oot_shadow_temple_gs_skull_pot",
        scene::SHADOW_TEMPLE,
        0x04,
    );
    add_skulltula_mapping(
        &mut map,
        "oot_shadow_temple_gs_near_boat",
        scene::SHADOW_TEMPLE,
        0x08,
    );
    add_skulltula_mapping(
        &mut map,
        "oot_shadow_temple_gs_triple_skull_pot",
        scene::SHADOW_TEMPLE,
        0x10,
    );

    // --- Bottom of the Well (Scene 0x08) ---
    add_skulltula_mapping(
        &mut map,
        "oot_bottom_of_the_well_gs_east_cage",
        scene::BOTTOM_OF_THE_WELL,
        0x01,
    );
    add_skulltula_mapping(
        &mut map,
        "oot_bottom_of_the_well_gs_inner_west",
        scene::BOTTOM_OF_THE_WELL,
        0x02,
    );
    add_skulltula_mapping(
        &mut map,
        "oot_bottom_of_the_well_gs_inner_east",
        scene::BOTTOM_OF_THE_WELL,
        0x04,
    );

    // --- Ice Cavern (Scene 0x09) ---
    add_skulltula_mapping(
        &mut map,
        "oot_ice_cavern_gs_scythe_room",
        scene::ICE_CAVERN,
        0x01,
    );
    add_skulltula_mapping(
        &mut map,
        "oot_ice_cavern_gs_hp_room",
        scene::ICE_CAVERN,
        0x02,
    );
    add_skulltula_mapping(
        &mut map,
        "oot_ice_cavern_gs_block_room",
        scene::ICE_CAVERN,
        0x04,
    );

    // ========================================================================
    // Overworld Gold Skulltulas
    // ========================================================================
    // Overworld skulltulas use scene IDs 0x0A-0x15 for GS flag bytes.
    // These map to actual overworld areas via a special mapping.

    // --- Hyrule Field area (GS Scene 0x0A) ---
    // Note: Overworld GS use a remapped scene ID system for the GS byte offset
    add_skulltula_mapping(
        &mut map,
        "oot_hyrule_castle_gs_tree",
        scene::HYRULE_CASTLE,
        0x01,
    );
    add_skulltula_mapping(
        &mut map,
        "oot_hyrule_castle_gs_grotto",
        scene::HYRULE_CASTLE,
        0x02,
    );

    // --- Kakariko Village area (GS Scene 0x10) ---
    add_skulltula_mapping(
        &mut map,
        "oot_kakariko_gs_shooting_gallery",
        scene::KAKARIKO_VILLAGE,
        0x01,
    );
    add_skulltula_mapping(
        &mut map,
        "oot_kakariko_gs_tree",
        scene::KAKARIKO_VILLAGE,
        0x02,
    );
    add_skulltula_mapping(
        &mut map,
        "oot_kakariko_gs_house_of_skulltula",
        scene::KAKARIKO_VILLAGE,
        0x04,
    );
    add_skulltula_mapping(
        &mut map,
        "oot_kakariko_gs_bazaar",
        scene::KAKARIKO_VILLAGE,
        0x08,
    );
    add_skulltula_mapping(
        &mut map,
        "oot_kakariko_gs_ladder",
        scene::KAKARIKO_VILLAGE,
        0x10,
    );
    add_skulltula_mapping(
        &mut map,
        "oot_kakariko_gs_roof",
        scene::KAKARIKO_VILLAGE,
        0x20,
    );

    // --- Graveyard (GS Scene 0x10) ---
    add_skulltula_mapping(&mut map, "oot_graveyard_gs_soil", scene::GRAVEYARD, 0x01);
    add_skulltula_mapping(&mut map, "oot_graveyard_gs_wall", scene::GRAVEYARD, 0x02);

    // --- Death Mountain Trail (GS Scene 0x0F) ---
    add_skulltula_mapping(
        &mut map,
        "oot_death_mountain_trail_gs_entrance",
        scene::DEATH_MOUNTAIN_TRAIL,
        0x01,
    );
    add_skulltula_mapping(
        &mut map,
        "oot_death_mountain_trail_gs_soil",
        scene::DEATH_MOUNTAIN_TRAIL,
        0x02,
    );
    add_skulltula_mapping(
        &mut map,
        "oot_death_mountain_trail_gs_above_dodongo",
        scene::DEATH_MOUNTAIN_TRAIL,
        0x04,
    );
    add_skulltula_mapping(
        &mut map,
        "oot_death_mountain_trail_gs_before_climb",
        scene::DEATH_MOUNTAIN_TRAIL,
        0x08,
    );

    // --- Death Mountain Crater (GS Scene 0x0F) ---
    add_skulltula_mapping(
        &mut map,
        "oot_death_mountain_crater_gs_crate",
        scene::DEATH_MOUNTAIN_CRATER,
        0x01,
    );
    add_skulltula_mapping(
        &mut map,
        "oot_death_mountain_crater_gs_soil",
        scene::DEATH_MOUNTAIN_CRATER,
        0x02,
    );

    // --- Goron City (GS Scene 0x0F) ---
    add_skulltula_mapping(
        &mut map,
        "oot_goron_city_gs_platform",
        scene::GORON_CITY,
        0x01,
    );
    add_skulltula_mapping(&mut map, "oot_goron_city_gs_maze", scene::GORON_CITY, 0x02);

    // --- Kokiri Forest (GS Scene 0x0C) ---
    add_skulltula_mapping(
        &mut map,
        "oot_kokiri_forest_gs_soil",
        scene::KOKIRI_FOREST,
        0x01,
    );
    add_skulltula_mapping(
        &mut map,
        "oot_kokiri_forest_gs_night_adult",
        scene::KOKIRI_FOREST,
        0x02,
    );
    add_skulltula_mapping(
        &mut map,
        "oot_kokiri_forest_gs_night_child",
        scene::KOKIRI_FOREST,
        0x04,
    );

    // --- Lost Woods (GS Scene 0x0D) ---
    add_skulltula_mapping(
        &mut map,
        "oot_lost_woods_gs_soil_bridge",
        scene::LOST_WOODS,
        0x01,
    );
    add_skulltula_mapping(
        &mut map,
        "oot_lost_woods_gs_soil_theater",
        scene::LOST_WOODS,
        0x02,
    );
    add_skulltula_mapping(
        &mut map,
        "oot_lost_woods_gs_bean_ride",
        scene::LOST_WOODS,
        0x04,
    );

    // --- Sacred Forest Meadow (GS Scene 0x0D) ---
    add_skulltula_mapping(
        &mut map,
        "oot_sacred_meadow_gs_night_adult",
        scene::SACRED_FOREST_MEADOW,
        0x01,
    );

    // --- Lake Hylia (GS Scene 0x12) ---
    add_skulltula_mapping(
        &mut map,
        "oot_lake_hylia_gs_lab_wall",
        scene::LAKE_HYLIA,
        0x01,
    );
    add_skulltula_mapping(
        &mut map,
        "oot_lake_hylia_gs_island",
        scene::LAKE_HYLIA,
        0x02,
    );
    add_skulltula_mapping(&mut map, "oot_lake_hylia_gs_soil", scene::LAKE_HYLIA, 0x04);
    add_skulltula_mapping(
        &mut map,
        "oot_lake_hylia_gs_big_tree",
        scene::LAKE_HYLIA,
        0x08,
    );

    // --- Lakeside Laboratory (same scene as Lake Hylia) ---
    add_skulltula_mapping(
        &mut map,
        "oot_laboratory_gs_crate",
        scene::LAKESIDE_LABORATORY,
        0x01,
    );

    // --- Zora River (GS Scene 0x11) ---
    add_skulltula_mapping(&mut map, "oot_zora_river_gs_tree", scene::ZORA_RIVER, 0x01);
    add_skulltula_mapping(
        &mut map,
        "oot_zora_river_gs_ladder",
        scene::ZORA_RIVER,
        0x02,
    );
    add_skulltula_mapping(
        &mut map,
        "oot_zora_river_gs_near_grotto",
        scene::ZORA_RIVER,
        0x04,
    );
    add_skulltula_mapping(
        &mut map,
        "oot_zora_river_gs_near_bridge",
        scene::ZORA_RIVER,
        0x08,
    );

    // --- Zora's Domain (GS Scene 0x11) ---
    add_skulltula_mapping(
        &mut map,
        "oot_zora_domain_gs_waterfall",
        scene::ZORAS_DOMAIN,
        0x01,
    );

    // --- Zora's Fountain (GS Scene 0x11) ---
    add_skulltula_mapping(
        &mut map,
        "oot_zora_fountain_gs_wall",
        scene::ZORAS_FOUNTAIN,
        0x01,
    );
    add_skulltula_mapping(
        &mut map,
        "oot_zora_fountain_gs_tree",
        scene::ZORAS_FOUNTAIN,
        0x02,
    );
    add_skulltula_mapping(
        &mut map,
        "oot_zora_fountain_gs_upper",
        scene::ZORAS_FOUNTAIN,
        0x04,
    );

    // --- Gerudo Valley (GS Scene 0x13) ---
    add_skulltula_mapping(
        &mut map,
        "oot_gerudo_valley_gs_soil",
        scene::GERUDO_VALLEY,
        0x01,
    );
    add_skulltula_mapping(
        &mut map,
        "oot_gerudo_valley_gs_wall",
        scene::GERUDO_VALLEY,
        0x02,
    );
    add_skulltula_mapping(
        &mut map,
        "oot_gerudo_valley_gs_tent",
        scene::GERUDO_VALLEY,
        0x04,
    );
    add_skulltula_mapping(
        &mut map,
        "oot_gerudo_valley_gs_pillar",
        scene::GERUDO_VALLEY,
        0x08,
    );

    // --- Gerudo Fortress (GS Scene 0x14) ---
    add_skulltula_mapping(
        &mut map,
        "oot_gerudo_fortress_gs_target",
        scene::GERUDO_FORTRESS,
        0x01,
    );
    add_skulltula_mapping(
        &mut map,
        "oot_gerudo_fortress_gs_wall",
        scene::GERUDO_FORTRESS,
        0x02,
    );

    // --- Desert Colossus (GS Scene 0x15) ---
    add_skulltula_mapping(
        &mut map,
        "oot_desert_colossus_gs_soil",
        scene::DESERT_COLOSSUS,
        0x01,
    );
    add_skulltula_mapping(
        &mut map,
        "oot_desert_colossus_gs_tree",
        scene::DESERT_COLOSSUS,
        0x02,
    );
    add_skulltula_mapping(
        &mut map,
        "oot_desert_colossus_gs_plateau",
        scene::DESERT_COLOSSUS,
        0x04,
    );

    // --- Lon Lon Ranch (GS Scene 0x0B) ---
    add_skulltula_mapping(
        &mut map,
        "oot_lon_lon_ranch_gs_tree",
        scene::LON_LON_RANCH,
        0x01,
    );
    add_skulltula_mapping(
        &mut map,
        "oot_lon_lon_ranch_gs_house",
        scene::LON_LON_RANCH,
        0x02,
    );
    add_skulltula_mapping(
        &mut map,
        "oot_lon_lon_ranch_gs_rain_shed",
        scene::LON_LON_RANCH,
        0x04,
    );
    add_skulltula_mapping(
        &mut map,
        "oot_lon_lon_ranch_gs_back_wall",
        scene::LON_LON_RANCH,
        0x08,
    );

    // ========================================================================
    // Heart Pieces / Freestanding Collectibles
    // ========================================================================
    //
    // Heart Pieces and other freestanding collectibles are tracked via the
    // scene collectible flags (offset 0x0C within each scene's flag block).
    // Each bit represents a collected item in that scene.

    // --- Ice Cavern (Scene 0x09) ---
    add_mapping(
        &mut map,
        "oot_ice_cavern_hp",
        scene::ICE_CAVERN,
        FlagType::Collectible,
        0x0000_0002,
    );

    // --- Kakariko / Windmill area (Scene 0x48) ---
    add_mapping(
        &mut map,
        "oot_kakariko_impa_house_hp",
        scene::IMPAS_HOUSE,
        FlagType::Collectible,
        0x0000_0002,
    );
    add_mapping(
        &mut map,
        "oot_windmill_hp",
        scene::WINDMILL_AND_DAMPES_GRAVE,
        FlagType::Collectible,
        0x0000_0002,
    );

    // --- Lon Lon Ranch Tower (Scene 0x4C) ---
    add_mapping(
        &mut map,
        "oot_lon_lon_ranch_silo_hp",
        scene::LON_LON_RANCH_TOWER,
        FlagType::Collectible,
        0x0000_0002,
    );

    // --- Graveyard (Scene 0x53) ---
    add_mapping(
        &mut map,
        "oot_graveyard_crate_hp",
        scene::GRAVEYARD,
        FlagType::Collectible,
        0x0000_0010,
    );

    // --- Zora River (Scene 0x54) ---
    add_mapping(
        &mut map,
        "oot_zora_river_hp_pillar",
        scene::ZORA_RIVER,
        FlagType::Collectible,
        0x0000_0800,
    );
    add_mapping(
        &mut map,
        "oot_zora_river_hp_platform",
        scene::ZORA_RIVER,
        FlagType::Collectible,
        0x0000_0010,
    );

    // --- Lake Hylia (Scene 0x57) ---
    add_mapping(
        &mut map,
        "oot_lake_hylia_hp",
        scene::LAKE_HYLIA,
        FlagType::Collectible,
        0x4000_0000,
    );

    // --- Zora's Fountain (Scene 0x59) ---
    add_mapping(
        &mut map,
        "oot_zora_fountain_iceberg_hp",
        scene::ZORAS_FOUNTAIN,
        FlagType::Collectible,
        0x0000_0002,
    );
    add_mapping(
        &mut map,
        "oot_zora_fountain_bottom_hp",
        scene::ZORAS_FOUNTAIN,
        FlagType::Collectible,
        0x0010_0000,
    );

    // --- Gerudo Valley (Scene 0x5A) ---
    add_mapping(
        &mut map,
        "oot_gerudo_valley_crate_hp",
        scene::GERUDO_VALLEY,
        FlagType::Collectible,
        0x0000_0004,
    );
    add_mapping(
        &mut map,
        "oot_gerudo_valley_waterfall_hp",
        scene::GERUDO_VALLEY,
        FlagType::Collectible,
        0x0000_0002,
    );

    // --- Desert Colossus (Scene 0x5C) ---
    add_mapping(
        &mut map,
        "oot_desert_colossus_hp",
        scene::DESERT_COLOSSUS,
        FlagType::Collectible,
        0x0000_2000,
    );

    // --- Death Mountain Trail (Scene 0x60) ---
    add_mapping(
        &mut map,
        "oot_death_mountain_trail_hp",
        scene::DEATH_MOUNTAIN_TRAIL,
        FlagType::Collectible,
        0x4000_0000,
    );

    // --- Death Mountain Crater (Scene 0x61) ---
    add_mapping(
        &mut map,
        "oot_death_mountain_crater_volcano_hp",
        scene::DEATH_MOUNTAIN_CRATER,
        FlagType::Collectible,
        0x0000_0100,
    );
    add_mapping(
        &mut map,
        "oot_death_mountain_crater_alcove_hp",
        scene::DEATH_MOUNTAIN_CRATER,
        FlagType::Collectible,
        0x0000_0004,
    );

    // --- Goron City (Scene 0x62) ---
    add_mapping(
        &mut map,
        "oot_goron_city_big_pot_hp",
        scene::GORON_CITY,
        FlagType::Collectible,
        0x8000_0000,
    );

    // --- Grottos (Scene 0x3E) - Hyrule Field Tektite Grotto ---
    add_mapping(
        &mut map,
        "oot_hyrule_field_grotto_tektite_hp",
        scene::GROTTOS,
        FlagType::Collectible,
        0x0000_0002,
    );

    // ========================================================================
    // MASTER QUEST DUNGEON MAPPINGS
    // ========================================================================
    //
    // Master Quest dungeons use the same scene IDs as vanilla but have
    // different chest/collectible layouts with different flag bits.
    // Location IDs use the "mq_oot_mq_" prefix to distinguish from vanilla.

    // ========================================================================
    // MQ DEKU TREE (Scene 0x00)
    // ========================================================================
    add_mapping(
        &mut map,
        "mq_oot_mq_deku_tree_map_chest",
        scene::DEKU_TREE,
        FlagType::Chest,
        0x0000_0001,
    );
    add_mapping(
        &mut map,
        "mq_oot_mq_deku_tree_compass_chest",
        scene::DEKU_TREE,
        FlagType::Chest,
        0x0000_0002,
    );
    add_mapping(
        &mut map,
        "mq_oot_mq_deku_tree_slingshot_chest",
        scene::DEKU_TREE,
        FlagType::Chest,
        0x0000_0008,
    );
    add_mapping(
        &mut map,
        "mq_oot_mq_deku_tree_slingshot_room_far_chest",
        scene::DEKU_TREE,
        FlagType::Chest,
        0x0000_0010,
    );
    add_mapping(
        &mut map,
        "mq_oot_mq_deku_tree_basement_chest",
        scene::DEKU_TREE,
        FlagType::Chest,
        0x0000_0004,
    );
    add_mapping(
        &mut map,
        "mq_oot_mq_deku_tree_before_water_platform_chest",
        scene::DEKU_TREE,
        FlagType::Chest,
        0x0000_0020,
    );
    add_mapping(
        &mut map,
        "mq_oot_mq_deku_tree_after_water_platform_chest",
        scene::DEKU_TREE,
        FlagType::Chest,
        0x0000_0040,
    );

    // MQ Deku Tree Gold Skulltulas
    add_skulltula_mapping(
        &mut map,
        "mq_oot_mq_deku_tree_gs_lobby_crate",
        scene::DEKU_TREE,
        0x01,
    );
    add_skulltula_mapping(
        &mut map,
        "mq_oot_mq_deku_tree_gs_compass_room",
        scene::DEKU_TREE,
        0x02,
    );
    add_skulltula_mapping(
        &mut map,
        "mq_oot_mq_deku_tree_gs_song_of_time_blocks",
        scene::DEKU_TREE,
        0x04,
    );
    add_skulltula_mapping(
        &mut map,
        "mq_oot_mq_deku_tree_gs_back_room",
        scene::DEKU_TREE,
        0x08,
    );

    // ========================================================================
    // MQ DODONGO'S CAVERN (Scene 0x01)
    // ========================================================================
    add_mapping(
        &mut map,
        "mq_oot_mq_dodongo_cavern_map_chest",
        scene::DODONGOS_CAVERN,
        FlagType::Chest,
        0x0000_0001,
    );
    add_mapping(
        &mut map,
        "mq_oot_mq_dodongo_cavern_compass_chest",
        scene::DODONGOS_CAVERN,
        FlagType::Chest,
        0x0000_0020,
    );
    add_mapping(
        &mut map,
        "mq_oot_mq_dodongo_cavern_bomb_bag_chest",
        scene::DODONGOS_CAVERN,
        FlagType::Chest,
        0x0000_0010,
    );
    add_mapping(
        &mut map,
        "mq_oot_mq_dodongo_cavern_larvae_room_chest",
        scene::DODONGOS_CAVERN,
        FlagType::Chest,
        0x0000_0002,
    );
    add_mapping(
        &mut map,
        "mq_oot_mq_dodongo_cavern_upper_ledge_chest",
        scene::DODONGOS_CAVERN,
        FlagType::Chest,
        0x0000_0040,
    );
    add_mapping(
        &mut map,
        "mq_oot_mq_dodongo_cavern_chest_under_grave",
        scene::DODONGOS_CAVERN,
        FlagType::Chest,
        0x0000_0004,
    );

    // MQ Dodongo's Cavern Gold Skulltulas
    add_skulltula_mapping(
        &mut map,
        "mq_oot_mq_dodongo_cavern_gs_time_blocks",
        scene::DODONGOS_CAVERN,
        0x01,
    );
    add_skulltula_mapping(
        &mut map,
        "mq_oot_mq_dodongo_cavern_gs_larve_room",
        scene::DODONGOS_CAVERN,
        0x02,
    );
    add_skulltula_mapping(
        &mut map,
        "mq_oot_mq_dodongo_cavern_gs_upper_lizalfos",
        scene::DODONGOS_CAVERN,
        0x04,
    );
    add_skulltula_mapping(
        &mut map,
        "mq_oot_mq_dodongo_cavern_gs_poe_room_side",
        scene::DODONGOS_CAVERN,
        0x08,
    );
    add_skulltula_mapping(
        &mut map,
        "mq_oot_mq_dodongo_cavern_gs_near_boss",
        scene::DODONGOS_CAVERN,
        0x10,
    );

    // ========================================================================
    // MQ JABU JABU'S BELLY (Scene 0x02)
    // ========================================================================
    add_mapping(
        &mut map,
        "mq_oot_mq_jabu_jabu_map_chest",
        scene::JABU_JABU,
        FlagType::Chest,
        0x0000_0001,
    );
    add_mapping(
        &mut map,
        "mq_oot_mq_jabu_jabu_entry_chest",
        scene::JABU_JABU,
        FlagType::Chest,
        0x0000_0080,
    );
    add_mapping(
        &mut map,
        "mq_oot_mq_jabu_jabu_second_room_b1_chest",
        scene::JABU_JABU,
        FlagType::Chest,
        0x0000_0002,
    );
    add_mapping(
        &mut map,
        "mq_oot_mq_jabu_jabu_compass_chest",
        scene::JABU_JABU,
        FlagType::Chest,
        0x0000_0008,
    );
    add_mapping(
        &mut map,
        "mq_oot_mq_jabu_jabu_second_room_1f_chest",
        scene::JABU_JABU,
        FlagType::Chest,
        0x0000_0100,
    );
    add_mapping(
        &mut map,
        "mq_oot_mq_jabu_jabu_third_room_west_chest",
        scene::JABU_JABU,
        FlagType::Chest,
        0x0000_0200,
    );
    add_mapping(
        &mut map,
        "mq_oot_mq_jabu_jabu_third_room_east_chest",
        scene::JABU_JABU,
        FlagType::Chest,
        0x0000_0400,
    );
    add_mapping(
        &mut map,
        "mq_oot_mq_jabu_jabu_boomerang_chest",
        scene::JABU_JABU,
        FlagType::Chest,
        0x0000_0004,
    );
    add_mapping(
        &mut map,
        "mq_oot_mq_jabu_jabu_sot_room_lower_chest",
        scene::JABU_JABU,
        FlagType::Chest,
        0x0000_0800,
    );
    add_mapping(
        &mut map,
        "mq_oot_mq_jabu_jabu_back_chest",
        scene::JABU_JABU,
        FlagType::Chest,
        0x0000_0020,
    );
    add_mapping(
        &mut map,
        "mq_oot_mq_jabu_jabu_pre_boss_chest",
        scene::JABU_JABU,
        FlagType::Chest,
        0x0000_0040,
    );

    // MQ Jabu Jabu Gold Skulltulas
    add_skulltula_mapping(
        &mut map,
        "mq_oot_mq_jabu_jabu_gs_sot_block",
        scene::JABU_JABU,
        0x01,
    );
    add_skulltula_mapping(
        &mut map,
        "mq_oot_mq_jabu_jabu_gs_back",
        scene::JABU_JABU,
        0x02,
    );
    add_skulltula_mapping(
        &mut map,
        "mq_oot_mq_jabu_jabu_gs_basement_side_room",
        scene::JABU_JABU,
        0x04,
    );
    add_skulltula_mapping(
        &mut map,
        "mq_oot_mq_jabu_jabu_gs_pre_boss",
        scene::JABU_JABU,
        0x08,
    );

    // ========================================================================
    // MQ FOREST TEMPLE (Scene 0x03)
    // ========================================================================
    add_mapping(
        &mut map,
        "mq_oot_mq_forest_temple_first_room_chest",
        scene::FOREST_TEMPLE,
        FlagType::Chest,
        0x0000_0008,
    );
    add_mapping(
        &mut map,
        "mq_oot_mq_forest_temple_wolfos_chest",
        scene::FOREST_TEMPLE,
        FlagType::Chest,
        0x0000_0001,
    );
    add_mapping(
        &mut map,
        "mq_oot_mq_forest_temple_boss_key_chest",
        scene::FOREST_TEMPLE,
        FlagType::Chest,
        0x0000_4000,
    );
    add_mapping(
        &mut map,
        "mq_oot_mq_forest_temple_redead_chest",
        scene::FOREST_TEMPLE,
        FlagType::Chest,
        0x0000_0004,
    );
    add_mapping(
        &mut map,
        "mq_oot_mq_forest_temple_well_chest",
        scene::FOREST_TEMPLE,
        FlagType::Chest,
        0x0000_0200,
    );
    add_mapping(
        &mut map,
        "mq_oot_mq_forest_temple_east_garden_high_ledge_chest",
        scene::FOREST_TEMPLE,
        FlagType::Chest,
        0x0000_0010,
    );
    add_mapping(
        &mut map,
        "mq_oot_mq_forest_temple_east_garden_ledge_chest",
        scene::FOREST_TEMPLE,
        FlagType::Chest,
        0x0000_0020,
    );
    add_mapping(
        &mut map,
        "mq_oot_mq_forest_temple_map_chest",
        scene::FOREST_TEMPLE,
        FlagType::Chest,
        0x0000_0002,
    );
    add_mapping(
        &mut map,
        "mq_oot_mq_forest_temple_bow_chest",
        scene::FOREST_TEMPLE,
        FlagType::Chest,
        0x0000_1000,
    );
    add_mapping(
        &mut map,
        "mq_oot_mq_forest_temple_compass_chest",
        scene::FOREST_TEMPLE,
        FlagType::Chest,
        0x0000_8000,
    );
    add_mapping(
        &mut map,
        "mq_oot_mq_forest_temple_falling_ceiling_chest",
        scene::FOREST_TEMPLE,
        FlagType::Chest,
        0x0000_0040,
    );

    // MQ Forest Temple Gold Skulltulas
    add_skulltula_mapping(
        &mut map,
        "mq_oot_mq_forest_temple_gs_entryway",
        scene::FOREST_TEMPLE,
        0x01,
    );
    add_skulltula_mapping(
        &mut map,
        "mq_oot_mq_forest_temple_gs_climb_room",
        scene::FOREST_TEMPLE,
        0x02,
    );
    add_skulltula_mapping(
        &mut map,
        "mq_oot_mq_forest_temple_gs_west_garden",
        scene::FOREST_TEMPLE,
        0x04,
    );
    add_skulltula_mapping(
        &mut map,
        "mq_oot_mq_forest_temple_gs_east_garden",
        scene::FOREST_TEMPLE,
        0x08,
    );
    add_skulltula_mapping(
        &mut map,
        "mq_oot_mq_forest_temple_gs_well",
        scene::FOREST_TEMPLE,
        0x10,
    );

    // ========================================================================
    // MQ FIRE TEMPLE (Scene 0x04)
    // ========================================================================
    add_mapping(
        &mut map,
        "mq_oot_mq_fire_temple_early_lower_left_chest",
        scene::FIRE_TEMPLE,
        FlagType::Chest,
        0x0000_0001,
    );
    add_mapping(
        &mut map,
        "mq_oot_mq_fire_temple_map_chest",
        scene::FIRE_TEMPLE,
        FlagType::Chest,
        0x0000_0002,
    );
    add_mapping(
        &mut map,
        "mq_oot_mq_fire_temple_pre_boss_chest",
        scene::FIRE_TEMPLE,
        FlagType::Chest,
        0x0000_0080,
    );
    add_mapping(
        &mut map,
        "mq_oot_mq_fire_temple_hammer_chest",
        scene::FIRE_TEMPLE,
        FlagType::Chest,
        0x0000_0008,
    );
    add_mapping(
        &mut map,
        "mq_oot_mq_fire_temple_boss_key_chest",
        scene::FIRE_TEMPLE,
        FlagType::Chest,
        0x0000_1000,
    );
    add_mapping(
        &mut map,
        "mq_oot_mq_fire_temple_1f_lava_room_goron_chest",
        scene::FIRE_TEMPLE,
        FlagType::Chest,
        0x0000_0004,
    );
    add_mapping(
        &mut map,
        "mq_oot_mq_fire_temple_compass_chest",
        scene::FIRE_TEMPLE,
        FlagType::Chest,
        0x0000_0040,
    );
    add_mapping(
        &mut map,
        "mq_oot_mq_fire_temple_maze_lower_chest",
        scene::FIRE_TEMPLE,
        FlagType::Chest,
        0x0000_0010,
    );
    add_mapping(
        &mut map,
        "mq_oot_mq_fire_temple_maze_upper_chest",
        scene::FIRE_TEMPLE,
        FlagType::Chest,
        0x0000_0020,
    );
    add_mapping(
        &mut map,
        "mq_oot_mq_fire_temple_maze_side_room_chest",
        scene::FIRE_TEMPLE,
        FlagType::Chest,
        0x0000_0100,
    );
    add_mapping(
        &mut map,
        "mq_oot_mq_fire_temple_topmost_chest",
        scene::FIRE_TEMPLE,
        FlagType::Chest,
        0x0000_2000,
    );

    // MQ Fire Temple Gold Skulltulas
    add_skulltula_mapping(
        &mut map,
        "mq_oot_mq_fire_temple_gs_1f_lava_room",
        scene::FIRE_TEMPLE,
        0x01,
    );
    add_skulltula_mapping(
        &mut map,
        "mq_oot_mq_fire_temple_gs_burning_block",
        scene::FIRE_TEMPLE,
        0x02,
    );
    add_skulltula_mapping(
        &mut map,
        "mq_oot_mq_fire_temple_gs_fire_walls_side_room",
        scene::FIRE_TEMPLE,
        0x04,
    );
    add_skulltula_mapping(
        &mut map,
        "mq_oot_mq_fire_temple_gs_fire_walls_middle",
        scene::FIRE_TEMPLE,
        0x08,
    );
    add_skulltula_mapping(
        &mut map,
        "mq_oot_mq_fire_temple_gs_topmost",
        scene::FIRE_TEMPLE,
        0x10,
    );

    // ========================================================================
    // MQ WATER TEMPLE (Scene 0x05)
    // ========================================================================
    add_mapping(
        &mut map,
        "mq_oot_mq_water_temple_compass_chest",
        scene::WATER_TEMPLE,
        FlagType::Chest,
        0x0000_0001,
    );
    add_mapping(
        &mut map,
        "mq_oot_mq_water_temple_longshot_chest",
        scene::WATER_TEMPLE,
        FlagType::Chest,
        0x0000_0002,
    );
    add_mapping(
        &mut map,
        "mq_oot_mq_water_temple_map_chest",
        scene::WATER_TEMPLE,
        FlagType::Chest,
        0x0000_0004,
    );
    add_mapping(
        &mut map,
        "mq_oot_mq_water_temple_boss_key_chest",
        scene::WATER_TEMPLE,
        FlagType::Chest,
        0x0000_0040,
    );
    add_mapping(
        &mut map,
        "mq_oot_mq_water_temple_central_pillar_chest",
        scene::WATER_TEMPLE,
        FlagType::Chest,
        0x0000_0020,
    );

    // MQ Water Temple Gold Skulltulas
    add_skulltula_mapping(
        &mut map,
        "mq_oot_mq_water_temple_gs_river",
        scene::WATER_TEMPLE,
        0x01,
    );
    add_skulltula_mapping(
        &mut map,
        "mq_oot_mq_water_temple_gs_three_torch",
        scene::WATER_TEMPLE,
        0x02,
    );
    add_skulltula_mapping(
        &mut map,
        "mq_oot_mq_water_temple_gs_side_loop",
        scene::WATER_TEMPLE,
        0x04,
    );
    add_skulltula_mapping(
        &mut map,
        "mq_oot_mq_water_temple_gs_lizalfos_hallway",
        scene::WATER_TEMPLE,
        0x08,
    );
    add_skulltula_mapping(
        &mut map,
        "mq_oot_mq_water_temple_gs_high_water_changer",
        scene::WATER_TEMPLE,
        0x10,
    );

    // ========================================================================
    // MQ SPIRIT TEMPLE (Scene 0x06)
    // ========================================================================
    add_mapping(
        &mut map,
        "mq_oot_mq_spirit_temple_entrance_initial_chest",
        scene::SPIRIT_TEMPLE,
        FlagType::Chest,
        0x0000_0001,
    );
    add_mapping(
        &mut map,
        "mq_oot_mq_spirit_temple_lobby_back_left_chest",
        scene::SPIRIT_TEMPLE,
        FlagType::Chest,
        0x0000_0002,
    );
    add_mapping(
        &mut map,
        "mq_oot_mq_spirit_temple_lobby_back_right_chest",
        scene::SPIRIT_TEMPLE,
        FlagType::Chest,
        0x0000_0004,
    );
    add_mapping(
        &mut map,
        "mq_oot_mq_spirit_temple_compass_chest",
        scene::SPIRIT_TEMPLE,
        FlagType::Chest,
        0x0000_0008,
    );
    add_mapping(
        &mut map,
        "mq_oot_mq_spirit_temple_sun_block_room_chest",
        scene::SPIRIT_TEMPLE,
        FlagType::Chest,
        0x0000_0010,
    );
    add_mapping(
        &mut map,
        "mq_oot_mq_spirit_temple_lobby_front_right_chest",
        scene::SPIRIT_TEMPLE,
        FlagType::Chest,
        0x0000_0020,
    );
    add_mapping(
        &mut map,
        "mq_oot_mq_spirit_temple_map_chest",
        scene::SPIRIT_TEMPLE,
        FlagType::Chest,
        0x0000_0040,
    );
    add_mapping(
        &mut map,
        "mq_oot_mq_spirit_temple_map_room_back_chest",
        scene::SPIRIT_TEMPLE,
        FlagType::Chest,
        0x0000_0080,
    );
    add_mapping(
        &mut map,
        "mq_oot_mq_spirit_temple_paradox_chest",
        scene::SPIRIT_TEMPLE,
        FlagType::Chest,
        0x0000_0100,
    );
    add_mapping(
        &mut map,
        "mq_oot_mq_spirit_temple_child_upper_ground_chest",
        scene::SPIRIT_TEMPLE,
        FlagType::Chest,
        0x0000_0200,
    );
    add_mapping(
        &mut map,
        "mq_oot_mq_spirit_temple_child_upper_ledge_chest",
        scene::SPIRIT_TEMPLE,
        FlagType::Chest,
        0x0000_0400,
    );
    add_mapping(
        &mut map,
        "mq_oot_mq_spirit_temple_silver_block_room_target_chest",
        scene::SPIRIT_TEMPLE,
        FlagType::Chest,
        0x0000_0800,
    );
    add_mapping(
        &mut map,
        "mq_oot_mq_spirit_temple_chest_in_box",
        scene::SPIRIT_TEMPLE,
        FlagType::Chest,
        0x0000_1000,
    );
    add_mapping(
        &mut map,
        "mq_oot_mq_spirit_temple_statue_room_ledge_chest",
        scene::SPIRIT_TEMPLE,
        FlagType::Chest,
        0x0000_2000,
    );
    add_mapping(
        &mut map,
        "mq_oot_mq_spirit_temple_purple_leever_chest",
        scene::SPIRIT_TEMPLE,
        FlagType::Chest,
        0x0000_4000,
    );
    add_mapping(
        &mut map,
        "mq_oot_mq_spirit_temple_symphony_room_chest",
        scene::SPIRIT_TEMPLE,
        FlagType::Chest,
        0x0000_8000,
    );
    add_mapping(
        &mut map,
        "mq_oot_mq_spirit_temple_beamos_room_chest",
        scene::SPIRIT_TEMPLE,
        FlagType::Chest,
        0x0001_0000,
    );
    add_mapping(
        &mut map,
        "mq_oot_mq_spirit_temple_dinolfos_room_chest",
        scene::SPIRIT_TEMPLE,
        FlagType::Chest,
        0x0002_0000,
    );
    add_mapping(
        &mut map,
        "mq_oot_mq_spirit_temple_boss_key_chest",
        scene::SPIRIT_TEMPLE,
        FlagType::Chest,
        0x0004_0000,
    );
    add_mapping(
        &mut map,
        "mq_oot_mq_spirit_temple_topmost_chest",
        scene::SPIRIT_TEMPLE,
        FlagType::Chest,
        0x0008_0000,
    );

    // MQ Spirit Temple Gold Skulltulas
    add_skulltula_mapping(
        &mut map,
        "mq_oot_mq_spirit_temple_gs_sun_block_room",
        scene::SPIRIT_TEMPLE,
        0x01,
    );
    add_skulltula_mapping(
        &mut map,
        "mq_oot_mq_spirit_temple_gs_leever_room",
        scene::SPIRIT_TEMPLE,
        0x02,
    );
    add_skulltula_mapping(
        &mut map,
        "mq_oot_mq_spirit_temple_gs_symphony_room",
        scene::SPIRIT_TEMPLE,
        0x04,
    );
    add_skulltula_mapping(
        &mut map,
        "mq_oot_mq_spirit_temple_gs_top_floor_left_wall",
        scene::SPIRIT_TEMPLE,
        0x08,
    );
    add_skulltula_mapping(
        &mut map,
        "mq_oot_mq_spirit_temple_gs_top_floor_back_wall",
        scene::SPIRIT_TEMPLE,
        0x10,
    );

    // ========================================================================
    // MQ SHADOW TEMPLE (Scene 0x07)
    // ========================================================================
    add_mapping(
        &mut map,
        "mq_oot_mq_shadow_temple_compass_chest",
        scene::SHADOW_TEMPLE,
        FlagType::Chest,
        0x0000_0008,
    );
    add_mapping(
        &mut map,
        "mq_oot_mq_shadow_temple_hover_boots_chest",
        scene::SHADOW_TEMPLE,
        FlagType::Chest,
        0x0000_0080,
    );
    add_mapping(
        &mut map,
        "mq_oot_mq_shadow_temple_first_gibdos_chest",
        scene::SHADOW_TEMPLE,
        FlagType::Chest,
        0x0000_0001,
    );
    add_mapping(
        &mut map,
        "mq_oot_mq_shadow_temple_map_chest",
        scene::SHADOW_TEMPLE,
        FlagType::Chest,
        0x0000_0002,
    );
    add_mapping(
        &mut map,
        "mq_oot_mq_shadow_temple_boat_passage_chest",
        scene::SHADOW_TEMPLE,
        FlagType::Chest,
        0x0000_0004,
    );
    add_mapping(
        &mut map,
        "mq_oot_mq_shadow_temple_second_silver_rupee_visible_chest",
        scene::SHADOW_TEMPLE,
        FlagType::Chest,
        0x0000_0010,
    );
    add_mapping(
        &mut map,
        "mq_oot_mq_shadow_temple_second_silver_rupee_invisible_chest",
        scene::SHADOW_TEMPLE,
        FlagType::Chest,
        0x0000_0020,
    );
    add_mapping(
        &mut map,
        "mq_oot_mq_shadow_temple_huge_pit_silver_rupee_chest",
        scene::SHADOW_TEMPLE,
        FlagType::Chest,
        0x0000_0040,
    );
    add_mapping(
        &mut map,
        "mq_oot_mq_shadow_temple_spike_curtain_ground_chest",
        scene::SHADOW_TEMPLE,
        FlagType::Chest,
        0x0000_0100,
    );
    add_mapping(
        &mut map,
        "mq_oot_mq_shadow_temple_spike_curtain_upper_cage_chest",
        scene::SHADOW_TEMPLE,
        FlagType::Chest,
        0x0000_0200,
    );
    add_mapping(
        &mut map,
        "mq_oot_mq_shadow_temple_spike_curtain_upper_switch_chest",
        scene::SHADOW_TEMPLE,
        FlagType::Chest,
        0x0000_0400,
    );
    add_mapping(
        &mut map,
        "mq_oot_mq_shadow_temple_invisible_spike_floor_chest",
        scene::SHADOW_TEMPLE,
        FlagType::Chest,
        0x0000_0800,
    );
    add_mapping(
        &mut map,
        "mq_oot_mq_shadow_temple_stalfos_room_chest",
        scene::SHADOW_TEMPLE,
        FlagType::Chest,
        0x0000_1000,
    );
    add_mapping(
        &mut map,
        "mq_oot_mq_shadow_temple_wind_hint_chest",
        scene::SHADOW_TEMPLE,
        FlagType::Chest,
        0x0020_0000,
    );
    add_mapping(
        &mut map,
        "mq_oot_mq_shadow_temple_after_wind_gibdos_chest",
        scene::SHADOW_TEMPLE,
        FlagType::Chest,
        0x0000_2000,
    );
    add_mapping(
        &mut map,
        "mq_oot_mq_shadow_temple_after_wind_bomb_chest",
        scene::SHADOW_TEMPLE,
        FlagType::Chest,
        0x0010_0000,
    );
    add_mapping(
        &mut map,
        "mq_oot_mq_shadow_temple_hidden_dead_hand_chest",
        scene::SHADOW_TEMPLE,
        FlagType::Chest,
        0x0000_4000,
    );
    add_mapping(
        &mut map,
        "mq_oot_mq_shadow_temple_crushing_wall_left_chest",
        scene::SHADOW_TEMPLE,
        FlagType::Chest,
        0x0000_8000,
    );
    add_mapping(
        &mut map,
        "mq_oot_mq_shadow_temple_boss_key_chest",
        scene::SHADOW_TEMPLE,
        FlagType::Chest,
        0x0001_0000,
    );

    // MQ Shadow Temple Gold Skulltulas
    add_skulltula_mapping(
        &mut map,
        "mq_oot_mq_shadow_temple_gs_spike_curtain",
        scene::SHADOW_TEMPLE,
        0x01,
    );
    add_skulltula_mapping(
        &mut map,
        "mq_oot_mq_shadow_temple_gs_wind_hint",
        scene::SHADOW_TEMPLE,
        0x02,
    );
    add_skulltula_mapping(
        &mut map,
        "mq_oot_mq_shadow_temple_gs_after_wind_bomb",
        scene::SHADOW_TEMPLE,
        0x04,
    );
    add_skulltula_mapping(
        &mut map,
        "mq_oot_mq_shadow_temple_gs_after_boat",
        scene::SHADOW_TEMPLE,
        0x08,
    );
    add_skulltula_mapping(
        &mut map,
        "mq_oot_mq_shadow_temple_gs_pre_boss",
        scene::SHADOW_TEMPLE,
        0x10,
    );

    // ========================================================================
    // MQ BOTTOM OF THE WELL (Scene 0x08)
    // ========================================================================
    add_mapping(
        &mut map,
        "mq_oot_mq_bottom_of_the_well_map_chest",
        scene::BOTTOM_OF_THE_WELL,
        FlagType::Chest,
        0x0000_0001,
    );
    add_mapping(
        &mut map,
        "mq_oot_mq_bottom_of_the_well_lens_chest",
        scene::BOTTOM_OF_THE_WELL,
        FlagType::Chest,
        0x0000_0008,
    );
    add_mapping(
        &mut map,
        "mq_oot_mq_bottom_of_the_well_compass_chest",
        scene::BOTTOM_OF_THE_WELL,
        FlagType::Chest,
        0x0000_0002,
    );

    // MQ Bottom of the Well Gold Skulltulas
    add_skulltula_mapping(
        &mut map,
        "mq_oot_mq_bottom_of_the_well_gs_basement",
        scene::BOTTOM_OF_THE_WELL,
        0x01,
    );
    add_skulltula_mapping(
        &mut map,
        "mq_oot_mq_bottom_of_the_well_gs_west_middle_room",
        scene::BOTTOM_OF_THE_WELL,
        0x02,
    );
    add_skulltula_mapping(
        &mut map,
        "mq_oot_mq_bottom_of_the_well_gs_coffin_room",
        scene::BOTTOM_OF_THE_WELL,
        0x04,
    );

    // ========================================================================
    // MQ ICE CAVERN (Scene 0x09)
    // ========================================================================
    add_mapping(
        &mut map,
        "mq_oot_mq_ice_cavern_map_chest",
        scene::ICE_CAVERN,
        FlagType::Chest,
        0x0000_0001,
    );
    add_mapping(
        &mut map,
        "mq_oot_mq_ice_cavern_compass_chest",
        scene::ICE_CAVERN,
        FlagType::Chest,
        0x0000_0002,
    );

    // MQ Ice Cavern Gold Skulltulas
    add_skulltula_mapping(
        &mut map,
        "mq_oot_mq_ice_cavern_gs_compass_room",
        scene::ICE_CAVERN,
        0x01,
    );
    add_skulltula_mapping(
        &mut map,
        "mq_oot_mq_ice_cavern_gs_clear_blocks",
        scene::ICE_CAVERN,
        0x02,
    );
    add_skulltula_mapping(
        &mut map,
        "mq_oot_mq_ice_cavern_gs_scarecrow",
        scene::ICE_CAVERN,
        0x04,
    );

    // ========================================================================
    // MQ GERUDO TRAINING GROUND (Scene 0x0B)
    // ========================================================================
    add_mapping(
        &mut map,
        "mq_oot_mq_gerudo_training_grounds_entryway_left_chest",
        scene::GERUDO_TRAINING_GROUND,
        FlagType::Chest,
        0x0000_0001,
    );
    add_mapping(
        &mut map,
        "mq_oot_mq_gerudo_training_grounds_entryway_right_chest",
        scene::GERUDO_TRAINING_GROUND,
        FlagType::Chest,
        0x0000_0002,
    );
    add_mapping(
        &mut map,
        "mq_oot_mq_gerudo_training_grounds_maze_first_chest",
        scene::GERUDO_TRAINING_GROUND,
        FlagType::Chest,
        0x0000_1000,
    );
    add_mapping(
        &mut map,
        "mq_oot_mq_gerudo_training_grounds_maze_second_chest",
        scene::GERUDO_TRAINING_GROUND,
        FlagType::Chest,
        0x0002_0000,
    );
    add_mapping(
        &mut map,
        "mq_oot_mq_gerudo_training_grounds_maze_third_chest",
        scene::GERUDO_TRAINING_GROUND,
        FlagType::Chest,
        0x0004_0000,
    );
    add_mapping(
        &mut map,
        "mq_oot_mq_gerudo_training_grounds_maze_fourth_chest",
        scene::GERUDO_TRAINING_GROUND,
        FlagType::Chest,
        0x0010_0000,
    );
    add_mapping(
        &mut map,
        "mq_oot_mq_gerudo_training_grounds_maze_right_side_middle_chest",
        scene::GERUDO_TRAINING_GROUND,
        FlagType::Chest,
        0x0000_0004,
    );
    add_mapping(
        &mut map,
        "mq_oot_mq_gerudo_training_grounds_maze_right_side_right_chest",
        scene::GERUDO_TRAINING_GROUND,
        FlagType::Chest,
        0x0000_0008,
    );
    add_mapping(
        &mut map,
        "mq_oot_mq_gerudo_training_grounds_right_side_dinolfos_chest",
        scene::GERUDO_TRAINING_GROUND,
        FlagType::Chest,
        0x0000_0010,
    );
    add_mapping(
        &mut map,
        "mq_oot_mq_gerudo_training_grounds_water_room_chest",
        scene::GERUDO_TRAINING_GROUND,
        FlagType::Chest,
        0x0000_0020,
    );
    add_mapping(
        &mut map,
        "mq_oot_mq_gerudo_training_grounds_left_side_iron_knuckle_chest",
        scene::GERUDO_TRAINING_GROUND,
        FlagType::Chest,
        0x0000_0040,
    );
    add_mapping(
        &mut map,
        "mq_oot_mq_gerudo_training_grounds_stalfos_room_chest",
        scene::GERUDO_TRAINING_GROUND,
        FlagType::Chest,
        0x0000_0080,
    );
    add_mapping(
        &mut map,
        "mq_oot_mq_gerudo_training_grounds_silver_block_room_chest",
        scene::GERUDO_TRAINING_GROUND,
        FlagType::Chest,
        0x0000_0100,
    );
    add_mapping(
        &mut map,
        "mq_oot_mq_gerudo_training_grounds_ice_arrows_chest",
        scene::GERUDO_TRAINING_GROUND,
        FlagType::Chest,
        0x0000_0200,
    );
    add_mapping(
        &mut map,
        "mq_oot_mq_gerudo_training_grounds_spinning_statue_chest",
        scene::GERUDO_TRAINING_GROUND,
        FlagType::Chest,
        0x0000_0400,
    );
    add_mapping(
        &mut map,
        "mq_oot_mq_gerudo_training_grounds_torch_slug_room_clear_chest",
        scene::GERUDO_TRAINING_GROUND,
        FlagType::Chest,
        0x0000_0800,
    );
    add_mapping(
        &mut map,
        "mq_oot_mq_gerudo_training_grounds_torch_slug_room_switch_chest",
        scene::GERUDO_TRAINING_GROUND,
        FlagType::Chest,
        0x0008_0000,
    );

    // ========================================================================
    // MQ GANON'S CASTLE (Scene 0x0D)
    // ========================================================================
    add_mapping(
        &mut map,
        "mq_oot_mq_ganon_castle_light_trial_chest",
        scene::GANONS_CASTLE,
        FlagType::Chest,
        0x0001_0000,
    );
    add_mapping(
        &mut map,
        "mq_oot_mq_ganon_castle_forest_trial_first_chest",
        scene::GANONS_CASTLE,
        FlagType::Chest,
        0x0000_0200,
    );
    add_mapping(
        &mut map,
        "mq_oot_mq_ganon_castle_forest_trial_second_chest",
        scene::GANONS_CASTLE,
        FlagType::Chest,
        0x0000_0001,
    );
    add_mapping(
        &mut map,
        "mq_oot_mq_ganon_castle_water_trial_chest",
        scene::GANONS_CASTLE,
        FlagType::Chest,
        0x0000_0080,
    );
    add_mapping(
        &mut map,
        "mq_oot_mq_ganon_castle_spirit_trial_first_chest",
        scene::GANONS_CASTLE,
        FlagType::Chest,
        0x0004_0000,
    );
    add_mapping(
        &mut map,
        "mq_oot_mq_ganon_castle_spirit_trial_second_chest",
        scene::GANONS_CASTLE,
        FlagType::Chest,
        0x0010_0000,
    );
    add_mapping(
        &mut map,
        "mq_oot_mq_ganon_castle_spirit_trial_back_right_sun_chest",
        scene::GANONS_CASTLE,
        FlagType::Chest,
        0x0000_0002,
    );
    add_mapping(
        &mut map,
        "mq_oot_mq_ganon_castle_spirit_trial_back_left_sun_chest",
        scene::GANONS_CASTLE,
        FlagType::Chest,
        0x0000_0004,
    );
    add_mapping(
        &mut map,
        "mq_oot_mq_ganon_castle_spirit_trial_front_left_sun_chest",
        scene::GANONS_CASTLE,
        FlagType::Chest,
        0x0000_0008,
    );
    add_mapping(
        &mut map,
        "mq_oot_mq_ganon_castle_spirit_trial_gold_gauntlets_chest",
        scene::GANONS_CASTLE,
        FlagType::Chest,
        0x0000_0010,
    );
    add_mapping(
        &mut map,
        "mq_oot_mq_ganon_castle_shadow_trial_bomb_flower_chest",
        scene::GANONS_CASTLE,
        FlagType::Chest,
        0x0000_0100,
    );
    add_mapping(
        &mut map,
        "mq_oot_mq_ganon_castle_shadow_trial_switch_chest",
        scene::GANONS_CASTLE,
        FlagType::Chest,
        0x0000_0020,
    );

    map
});

/// Helper function to add a Gold Skulltula mapping.
///
/// Gold Skulltulas use a separate flag section from scene flags.
/// The scene_id identifies which area/dungeon the skulltula is in,
/// and flag_bit is the bit position within that area's byte.
fn add_skulltula_mapping(
    map: &mut HashMap<&'static str, FlagMapping>,
    location_id: &'static str,
    scene_id: u8,
    flag_bit: u32,
) {
    map.insert(
        location_id,
        FlagMapping::mapped(location_id, scene_id, FlagType::GoldSkulltula, flag_bit),
    );
}

/// Helper function to add a scene-based mapping.
fn add_mapping(
    map: &mut HashMap<&'static str, FlagMapping>,
    location_id: &'static str,
    scene_id: u8,
    flag_type: FlagType,
    flag_bit: u32,
) {
    map.insert(
        location_id,
        FlagMapping::mapped(location_id, scene_id, flag_type, flag_bit),
    );
}
