//! MM flag data structures and stub mappings.
//!
//! This module provides data structures for mapping MM in-game locations (chests,
//! collectibles, stray fairies, etc.) to their memory flag locations in save data.
//!
//! # Memory Layout
//!
//! MM save data base address: 0x1EF670
//!
//! Scene flags work similarly to OoT but with different offsets. Each scene has
//! flags for chests, switches, room clears, and collectibles.
//!
//! # Flag Types
//!
//! - **Chest flags**: Track which chests have been opened
//! - **Collectible flags**: Track collected items (skulltulas, heart pieces, etc.)
//! - **Special flags**: Track special collectibles like stray fairies

/// MM save data offsets for scene flags.
pub mod offsets {
    /// MM save data base address in RAM.
    pub const MM_SAVE_BASE: usize = 0x1EF670;

    /// Scene flags offset in save data.
    pub const SCENE_FLAGS: usize = 0x00;

    /// Size of each scene's flags (0x14 = 20 bytes).
    pub const SCENE_SIZE: usize = 0x14;

    /// Number of scenes in MM.
    pub const NUM_SCENES: usize = 0x78;

    /// Stray fairy counts offset in save data.
    pub const STRAY_FAIRY_COUNTS: usize = 0x0E94;

    /// Number of stray fairy count entries.
    pub const STRAY_FAIRY_COUNT_SIZE: usize = 10;
}

/// Type of flag in MM save data.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MmFlagType {
    /// Chest flag (offset 0x00 in scene flags).
    Chest,
    /// Switch flag (offset 0x04 in scene flags).
    Switch,
    /// Room clear flag (offset 0x08 in scene flags).
    RoomClear,
    /// Collectible flag (offset 0x0C in scene flags).
    Collectible,
    /// Special flags (offset 0x10 in scene flags).
    Special,
}

impl MmFlagType {
    /// Get the byte offset within a scene's flag block for this flag type.
    #[must_use]
    pub const fn offset(self) -> usize {
        match self {
            Self::Chest => 0x00,
            Self::Switch => 0x04,
            Self::RoomClear => 0x08,
            Self::Collectible => 0x0C,
            Self::Special => 0x10,
        }
    }
}

/// Memory flag location for MM scene-based flags.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MmSceneFlag {
    /// Scene ID (0x00-0x77).
    pub scene_id: u8,
    /// Type of flag (chest, collectible, etc.).
    pub flag_type: MmFlagType,
    /// Bit mask for the flag within the 4-byte flag word.
    pub bit_mask: u32,
}

impl MmSceneFlag {
    /// Create a new scene flag reference.
    #[must_use]
    pub const fn new(scene_id: u8, flag_type: MmFlagType, bit_mask: u32) -> Self {
        Self {
            scene_id,
            flag_type,
            bit_mask,
        }
    }

    /// Calculate the absolute offset in save data for this flag.
    #[must_use]
    pub const fn save_offset(&self) -> usize {
        offsets::SCENE_FLAGS
            + (self.scene_id as usize) * offsets::SCENE_SIZE
            + self.flag_type.offset()
    }
}

/// Mapping from a location name to its memory flag location.
///
/// This struct is used to map randomizer check locations to their corresponding
/// memory flags in save data, allowing the tracker to read game state.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MmFlagMapping {
    /// The name of the location as used in randomizer logic.
    pub location_name: String,
    /// The scene flag that tracks this location.
    pub flag: MmSceneFlag,
}

impl MmFlagMapping {
    /// Create a new flag mapping.
    #[must_use]
    pub fn new(location_name: impl Into<String>, flag: MmSceneFlag) -> Self {
        Self {
            location_name: location_name.into(),
            flag,
        }
    }
}

/// Get MM chest flag mappings.
///
/// Returns a vector of mappings from chest location names to their scene flags.
/// Includes all main dungeons (Woodfall, Snowhead, Great Bay, Stone Tower),
/// mini-dungeons, and overworld chest locations.
#[must_use]
pub fn mm_chest_mappings() -> Vec<MmFlagMapping> {
    vec![
        // ========================================================================
        // WOODFALL TEMPLE (Scene 0x1F)
        // ========================================================================
        MmFlagMapping::new(
            "mm_woodfall_temple_map_chest",
            MmSceneFlag::new(0x1F, MmFlagType::Chest, 0x0000_0001),
        ),
        MmFlagMapping::new(
            "mm_woodfall_temple_compass_chest",
            MmSceneFlag::new(0x1F, MmFlagType::Chest, 0x0000_0002),
        ),
        MmFlagMapping::new(
            "mm_woodfall_temple_boss_key_chest",
            MmSceneFlag::new(0x1F, MmFlagType::Chest, 0x0000_0004),
        ),
        MmFlagMapping::new(
            "mm_woodfall_temple_small_key_chest",
            MmSceneFlag::new(0x1F, MmFlagType::Chest, 0x0000_0008),
        ),
        MmFlagMapping::new(
            "mm_woodfall_temple_heros_bow_chest",
            MmSceneFlag::new(0x1F, MmFlagType::Chest, 0x0000_0010),
        ),
        MmFlagMapping::new(
            "mm_woodfall_temple_entrance_chest",
            MmSceneFlag::new(0x1F, MmFlagType::Chest, 0x0000_0020),
        ),
        MmFlagMapping::new(
            "mm_woodfall_temple_water_chest",
            MmSceneFlag::new(0x1F, MmFlagType::Chest, 0x0000_0040),
        ),
        MmFlagMapping::new(
            "mm_woodfall_temple_dark_chest",
            MmSceneFlag::new(0x1F, MmFlagType::Chest, 0x0000_0080),
        ),
        MmFlagMapping::new(
            "mm_woodfall_temple_center_chest",
            MmSceneFlag::new(0x1F, MmFlagType::Chest, 0x0000_0100),
        ),
        // ========================================================================
        // SNOWHEAD TEMPLE (Scene 0x22)
        // ========================================================================
        MmFlagMapping::new(
            "mm_snowhead_temple_map_chest",
            MmSceneFlag::new(0x22, MmFlagType::Chest, 0x0000_0001),
        ),
        MmFlagMapping::new(
            "mm_snowhead_temple_compass_chest",
            MmSceneFlag::new(0x22, MmFlagType::Chest, 0x0000_0002),
        ),
        MmFlagMapping::new(
            "mm_snowhead_temple_boss_key_chest",
            MmSceneFlag::new(0x22, MmFlagType::Chest, 0x0000_0004),
        ),
        MmFlagMapping::new(
            "mm_snowhead_temple_fire_arrow_chest",
            MmSceneFlag::new(0x22, MmFlagType::Chest, 0x0000_0008),
        ),
        MmFlagMapping::new(
            "mm_snowhead_temple_block_room_chest",
            MmSceneFlag::new(0x22, MmFlagType::Chest, 0x0000_0010),
        ),
        MmFlagMapping::new(
            "mm_snowhead_temple_icicle_room_chest",
            MmSceneFlag::new(0x22, MmFlagType::Chest, 0x0000_0020),
        ),
        MmFlagMapping::new(
            "mm_snowhead_temple_bridge_room_chest",
            MmSceneFlag::new(0x22, MmFlagType::Chest, 0x0000_0040),
        ),
        // ========================================================================
        // GREAT BAY TEMPLE (Scene 0x1E)
        // ========================================================================
        MmFlagMapping::new(
            "mm_great_bay_temple_map_chest",
            MmSceneFlag::new(0x1E, MmFlagType::Chest, 0x0000_0001),
        ),
        MmFlagMapping::new(
            "mm_great_bay_temple_compass_chest",
            MmSceneFlag::new(0x1E, MmFlagType::Chest, 0x0000_0002),
        ),
        MmFlagMapping::new(
            "mm_great_bay_temple_boss_key_chest",
            MmSceneFlag::new(0x1E, MmFlagType::Chest, 0x0000_0004),
        ),
        MmFlagMapping::new(
            "mm_great_bay_temple_ice_arrow_chest",
            MmSceneFlag::new(0x1E, MmFlagType::Chest, 0x0000_0008),
        ),
        MmFlagMapping::new(
            "mm_great_bay_temple_hookshot_chest",
            MmSceneFlag::new(0x1E, MmFlagType::Chest, 0x0000_0010),
        ),
        MmFlagMapping::new(
            "mm_great_bay_temple_small_key_chest",
            MmSceneFlag::new(0x1E, MmFlagType::Chest, 0x0000_0020),
        ),
        MmFlagMapping::new(
            "mm_great_bay_temple_entrance_chest",
            MmSceneFlag::new(0x1E, MmFlagType::Chest, 0x0000_0040),
        ),
        MmFlagMapping::new(
            "mm_great_bay_temple_baba_chest",
            MmSceneFlag::new(0x1E, MmFlagType::Chest, 0x0000_0080),
        ),
        MmFlagMapping::new(
            "mm_great_bay_temple_green_pipe_1_chest",
            MmSceneFlag::new(0x1E, MmFlagType::Chest, 0x0000_0100),
        ),
        MmFlagMapping::new(
            "mm_great_bay_temple_green_pipe_2_lower_chest",
            MmSceneFlag::new(0x1E, MmFlagType::Chest, 0x0000_0200),
        ),
        MmFlagMapping::new(
            "mm_great_bay_temple_green_pipe_2_upper_chest",
            MmSceneFlag::new(0x1E, MmFlagType::Chest, 0x0000_0400),
        ),
        MmFlagMapping::new(
            "mm_great_bay_temple_green_pipe_3_chest",
            MmSceneFlag::new(0x1E, MmFlagType::Chest, 0x0000_0800),
        ),
        // ========================================================================
        // STONE TOWER TEMPLE (Scene 0x18)
        // ========================================================================
        MmFlagMapping::new(
            "mm_stone_tower_temple_map_chest",
            MmSceneFlag::new(0x18, MmFlagType::Chest, 0x0000_0001),
        ),
        MmFlagMapping::new(
            "mm_stone_tower_temple_compass_chest",
            MmSceneFlag::new(0x18, MmFlagType::Chest, 0x0000_0002),
        ),
        MmFlagMapping::new(
            "mm_stone_tower_temple_boss_key_chest",
            MmSceneFlag::new(0x18, MmFlagType::Chest, 0x0000_0004),
        ),
        MmFlagMapping::new(
            "mm_stone_tower_temple_light_arrow_chest",
            MmSceneFlag::new(0x18, MmFlagType::Chest, 0x0000_0008),
        ),
        MmFlagMapping::new(
            "mm_stone_tower_temple_giants_mask_chest",
            MmSceneFlag::new(0x18, MmFlagType::Chest, 0x0000_0010),
        ),
        MmFlagMapping::new(
            "mm_stone_tower_temple_entrance_chest",
            MmSceneFlag::new(0x18, MmFlagType::Chest, 0x0000_0020),
        ),
        MmFlagMapping::new(
            "mm_stone_tower_temple_entrance_switch_chest",
            MmSceneFlag::new(0x18, MmFlagType::Chest, 0x0000_0040),
        ),
        MmFlagMapping::new(
            "mm_stone_tower_temple_under_west_garden_ledge_chest",
            MmSceneFlag::new(0x18, MmFlagType::Chest, 0x0000_0080),
        ),
        MmFlagMapping::new(
            "mm_stone_tower_temple_under_west_garden_lava_chest",
            MmSceneFlag::new(0x18, MmFlagType::Chest, 0x0000_0100),
        ),
        MmFlagMapping::new(
            "mm_stone_tower_temple_center_sun_block_chest",
            MmSceneFlag::new(0x18, MmFlagType::Chest, 0x0000_0200),
        ),
        MmFlagMapping::new(
            "mm_stone_tower_temple_center_across_water_chest",
            MmSceneFlag::new(0x18, MmFlagType::Chest, 0x0000_0400),
        ),
        MmFlagMapping::new(
            "mm_stone_tower_temple_water_sun_switch_chest",
            MmSceneFlag::new(0x18, MmFlagType::Chest, 0x0000_0800),
        ),
        MmFlagMapping::new(
            "mm_stone_tower_temple_mirrors_room_center_chest",
            MmSceneFlag::new(0x18, MmFlagType::Chest, 0x0000_1000),
        ),
        MmFlagMapping::new(
            "mm_stone_tower_temple_mirrors_room_right_chest",
            MmSceneFlag::new(0x18, MmFlagType::Chest, 0x0000_2000),
        ),
        MmFlagMapping::new(
            "mm_stone_tower_temple_wind_room_ledge_chest",
            MmSceneFlag::new(0x18, MmFlagType::Chest, 0x0000_4000),
        ),
        MmFlagMapping::new(
            "mm_stone_tower_temple_wind_room_jail_chest",
            MmSceneFlag::new(0x18, MmFlagType::Chest, 0x0000_8000),
        ),
        MmFlagMapping::new(
            "mm_stone_tower_temple_before_water_bridge_chest",
            MmSceneFlag::new(0x18, MmFlagType::Chest, 0x0001_0000),
        ),
        MmFlagMapping::new(
            "mm_stone_tower_temple_water_bridge_chest",
            MmSceneFlag::new(0x18, MmFlagType::Chest, 0x0002_0000),
        ),
        // ========================================================================
        // STONE TOWER TEMPLE INVERTED (Scene 0x19)
        // ========================================================================
        MmFlagMapping::new(
            "mm_stone_tower_temple_inverted_entrance_chest",
            MmSceneFlag::new(0x19, MmFlagType::Chest, 0x0000_0001),
        ),
        MmFlagMapping::new(
            "mm_stone_tower_temple_inverted_east_lower_chest",
            MmSceneFlag::new(0x19, MmFlagType::Chest, 0x0000_0002),
        ),
        MmFlagMapping::new(
            "mm_stone_tower_temple_inverted_east_upper_chest",
            MmSceneFlag::new(0x19, MmFlagType::Chest, 0x0000_0004),
        ),
        MmFlagMapping::new(
            "mm_stone_tower_temple_inverted_east_middle_chest",
            MmSceneFlag::new(0x19, MmFlagType::Chest, 0x0000_0008),
        ),
        MmFlagMapping::new(
            "mm_stone_tower_temple_inverted_wizzrobe_chest",
            MmSceneFlag::new(0x19, MmFlagType::Chest, 0x0000_0010),
        ),
        MmFlagMapping::new(
            "mm_stone_tower_temple_inverted_death_armos_chest",
            MmSceneFlag::new(0x19, MmFlagType::Chest, 0x0000_0020),
        ),
        // ========================================================================
        // BENEATH THE WELL (Scene 0x1B)
        // ========================================================================
        MmFlagMapping::new(
            "mm_beneath_the_well_mirror_shield_chest",
            MmSceneFlag::new(0x1B, MmFlagType::Chest, 0x0000_0001),
        ),
        MmFlagMapping::new(
            "mm_beneath_the_well_compass_chest",
            MmSceneFlag::new(0x1B, MmFlagType::Chest, 0x0000_0002),
        ),
        MmFlagMapping::new(
            "mm_beneath_the_well_map_chest",
            MmSceneFlag::new(0x1B, MmFlagType::Chest, 0x0000_0004),
        ),
        MmFlagMapping::new(
            "mm_beneath_the_well_keese_chest",
            MmSceneFlag::new(0x1B, MmFlagType::Chest, 0x0000_0008),
        ),
        MmFlagMapping::new(
            "mm_beneath_the_well_skulltulla_chest",
            MmSceneFlag::new(0x1B, MmFlagType::Chest, 0x0000_0010),
        ),
        // ========================================================================
        // ANCIENT CASTLE OF IKANA (Scene 0x11)
        // ========================================================================
        MmFlagMapping::new(
            "mm_ancient_castle_of_ikana_powder_keg_chest",
            MmSceneFlag::new(0x11, MmFlagType::Chest, 0x0000_0001),
        ),
        MmFlagMapping::new(
            "mm_ancient_castle_of_ikana_compass_chest",
            MmSceneFlag::new(0x11, MmFlagType::Chest, 0x0000_0002),
        ),
        MmFlagMapping::new(
            "mm_ancient_castle_of_ikana_map_chest",
            MmSceneFlag::new(0x11, MmFlagType::Chest, 0x0000_0004),
        ),
        // ========================================================================
        // SECRET SHRINE (Scene 0x13)
        // ========================================================================
        MmFlagMapping::new(
            "mm_secret_shrine_hp_chest",
            MmSceneFlag::new(0x13, MmFlagType::Chest, 0x0000_0001),
        ),
        MmFlagMapping::new(
            "mm_secret_shrine_light_arrows_chest",
            MmSceneFlag::new(0x13, MmFlagType::Chest, 0x0000_0002),
        ),
        MmFlagMapping::new(
            "mm_secret_shrine_dinolfos_chest",
            MmSceneFlag::new(0x13, MmFlagType::Chest, 0x0000_0004),
        ),
        MmFlagMapping::new(
            "mm_secret_shrine_wizzrobe_chest",
            MmSceneFlag::new(0x13, MmFlagType::Chest, 0x0000_0008),
        ),
        MmFlagMapping::new(
            "mm_secret_shrine_wart_chest",
            MmSceneFlag::new(0x13, MmFlagType::Chest, 0x0000_0010),
        ),
        MmFlagMapping::new(
            "mm_secret_shrine_garo_master_chest",
            MmSceneFlag::new(0x13, MmFlagType::Chest, 0x0000_0020),
        ),
        // ========================================================================
        // PIRATES FORTRESS (Scene 0x29 = exterior, 0x2A = interior)
        // ========================================================================
        MmFlagMapping::new(
            "mm_pirate_fortress_entrance_chest_1",
            MmSceneFlag::new(0x29, MmFlagType::Chest, 0x0000_0001),
        ),
        MmFlagMapping::new(
            "mm_pirate_fortress_entrance_chest_2",
            MmSceneFlag::new(0x29, MmFlagType::Chest, 0x0000_0002),
        ),
        MmFlagMapping::new(
            "mm_pirate_fortress_entrance_chest_3",
            MmSceneFlag::new(0x29, MmFlagType::Chest, 0x0000_0004),
        ),
        MmFlagMapping::new(
            "mm_pirate_fortress_sewers_chest_1",
            MmSceneFlag::new(0x29, MmFlagType::Chest, 0x0000_0008),
        ),
        MmFlagMapping::new(
            "mm_pirate_fortress_sewers_chest_2",
            MmSceneFlag::new(0x29, MmFlagType::Chest, 0x0000_0010),
        ),
        MmFlagMapping::new(
            "mm_pirate_fortress_sewers_chest_3",
            MmSceneFlag::new(0x29, MmFlagType::Chest, 0x0000_0020),
        ),
        MmFlagMapping::new(
            "mm_pirate_fortress_interior_hookshot_chest",
            MmSceneFlag::new(0x2A, MmFlagType::Chest, 0x0000_0001),
        ),
        MmFlagMapping::new(
            "mm_pirate_fortress_interior_silver_rupee_chest_1",
            MmSceneFlag::new(0x2A, MmFlagType::Chest, 0x0000_0002),
        ),
        MmFlagMapping::new(
            "mm_pirate_fortress_interior_silver_rupee_chest_2",
            MmSceneFlag::new(0x2A, MmFlagType::Chest, 0x0000_0004),
        ),
        MmFlagMapping::new(
            "mm_pirate_fortress_interior_silver_rupee_chest_3",
            MmSceneFlag::new(0x2A, MmFlagType::Chest, 0x0000_0008),
        ),
        MmFlagMapping::new(
            "mm_pirate_fortress_interior_lower_chest",
            MmSceneFlag::new(0x2A, MmFlagType::Chest, 0x0000_0010),
        ),
        MmFlagMapping::new(
            "mm_pirate_fortress_interior_upper_chest",
            MmSceneFlag::new(0x2A, MmFlagType::Chest, 0x0000_0020),
        ),
        MmFlagMapping::new(
            "mm_pirate_fortress_interior_pot_chest_aquarium_1",
            MmSceneFlag::new(0x2A, MmFlagType::Chest, 0x0000_0040),
        ),
        MmFlagMapping::new(
            "mm_pirate_fortress_interior_pot_chest_aquarium_2",
            MmSceneFlag::new(0x2A, MmFlagType::Chest, 0x0000_0080),
        ),
        MmFlagMapping::new(
            "mm_pirate_fortress_interior_pot_chest_aquarium_3",
            MmSceneFlag::new(0x2A, MmFlagType::Chest, 0x0000_0100),
        ),
        // ========================================================================
        // CLOCK TOWN (Scenes 0x6C-0x6F)
        // ========================================================================
        MmFlagMapping::new(
            "mm_clock_town_south_chest_lower",
            MmSceneFlag::new(0x6C, MmFlagType::Chest, 0x0000_0001),
        ),
        MmFlagMapping::new(
            "mm_clock_town_south_chest_upper",
            MmSceneFlag::new(0x6C, MmFlagType::Chest, 0x0000_0002),
        ),
        MmFlagMapping::new(
            "mm_clock_town_east_chest",
            MmSceneFlag::new(0x6E, MmFlagType::Chest, 0x0000_0001),
        ),
        MmFlagMapping::new(
            "mm_clock_town_north_tree_chest",
            MmSceneFlag::new(0x6D, MmFlagType::Chest, 0x0000_0001),
        ),
        MmFlagMapping::new(
            "mm_clock_town_silver_rupee_chest",
            MmSceneFlag::new(0x6E, MmFlagType::Chest, 0x0000_0002),
        ),
        // ========================================================================
        // TREASURE CHEST SHOP (Scene 0x4C)
        // ========================================================================
        MmFlagMapping::new(
            "mm_chest_game_hp",
            MmSceneFlag::new(0x4C, MmFlagType::Chest, 0x0000_0001),
        ),
        // ========================================================================
        // MOON TRIALS (Scenes 0x66-0x69)
        // ========================================================================
        MmFlagMapping::new(
            "mm_moon_trial_deku_hp",
            MmSceneFlag::new(0x67, MmFlagType::Chest, 0x0000_0001),
        ),
        MmFlagMapping::new(
            "mm_moon_trial_goron_hp",
            MmSceneFlag::new(0x68, MmFlagType::Chest, 0x0000_0001),
        ),
        MmFlagMapping::new(
            "mm_moon_trial_zora_hp",
            MmSceneFlag::new(0x69, MmFlagType::Chest, 0x0000_0001),
        ),
        MmFlagMapping::new(
            "mm_moon_trial_link_hp",
            MmSceneFlag::new(0x66, MmFlagType::Chest, 0x0000_0001),
        ),
        MmFlagMapping::new(
            "mm_moon_trial_link_garo_master_chest",
            MmSceneFlag::new(0x66, MmFlagType::Chest, 0x0000_0002),
        ),
        MmFlagMapping::new(
            "mm_moon_trial_link_iron_knuckle_chest",
            MmSceneFlag::new(0x66, MmFlagType::Chest, 0x0000_0004),
        ),
        // ========================================================================
        // STOCK POT INN (Scene 0x4D)
        // ========================================================================
        MmFlagMapping::new(
            "mm_stock_pot_inn_guest_room_chest",
            MmSceneFlag::new(0x4D, MmFlagType::Chest, 0x0000_0001),
        ),
        MmFlagMapping::new(
            "mm_stock_pot_inn_staff_room_chest",
            MmSceneFlag::new(0x4D, MmFlagType::Chest, 0x0000_0002),
        ),
        // ========================================================================
        // ASTRAL OBSERVATORY (Scene 0x52)
        // ========================================================================
        MmFlagMapping::new(
            "mm_astral_observatory_passage_chest",
            MmSceneFlag::new(0x52, MmFlagType::Chest, 0x0000_0001),
        ),
        // ========================================================================
        // TERMINA FIELD (Scene 0x54)
        // ========================================================================
        MmFlagMapping::new(
            "mm_termina_field_water_chest",
            MmSceneFlag::new(0x54, MmFlagType::Chest, 0x0000_0001),
        ),
        MmFlagMapping::new(
            "mm_termina_field_tall_grass_chest",
            MmSceneFlag::new(0x54, MmFlagType::Chest, 0x0000_0002),
        ),
        MmFlagMapping::new(
            "mm_termina_field_tree_stump_chest",
            MmSceneFlag::new(0x54, MmFlagType::Chest, 0x0000_0004),
        ),
        // ========================================================================
        // WOODFALL (Scene 0x14)
        // ========================================================================
        MmFlagMapping::new(
            "mm_woodfall_entrance_chest",
            MmSceneFlag::new(0x14, MmFlagType::Chest, 0x0000_0001),
        ),
        MmFlagMapping::new(
            "mm_woodfall_hp_chest",
            MmSceneFlag::new(0x14, MmFlagType::Chest, 0x0000_0002),
        ),
        MmFlagMapping::new(
            "mm_woodfall_near_owl_chest",
            MmSceneFlag::new(0x14, MmFlagType::Chest, 0x0000_0004),
        ),
        // ========================================================================
        // MOUNTAIN VILLAGE (Scene 0x65)
        // ========================================================================
        MmFlagMapping::new(
            "mm_mountain_village_waterfall_chest",
            MmSceneFlag::new(0x65, MmFlagType::Chest, 0x0000_0001),
        ),
        // ========================================================================
        // TWIN ISLANDS (Scene 0x48/0x49)
        // ========================================================================
        MmFlagMapping::new(
            "mm_twin_islands_underwater_chest_1",
            MmSceneFlag::new(0x49, MmFlagType::Chest, 0x0000_0001),
        ),
        MmFlagMapping::new(
            "mm_twin_islands_underwater_chest_2",
            MmSceneFlag::new(0x49, MmFlagType::Chest, 0x0000_0002),
        ),
        MmFlagMapping::new(
            "mm_twin_islands_ramp_grotto_chest",
            MmSceneFlag::new(0x49, MmFlagType::Chest, 0x0000_0004),
        ),
        MmFlagMapping::new(
            "mm_twin_islands_frozen_grotto_chest",
            MmSceneFlag::new(0x49, MmFlagType::Chest, 0x0000_0008),
        ),
        // ========================================================================
        // GREAT BAY COAST (Scene 0x37)
        // ========================================================================
        MmFlagMapping::new(
            "mm_great_bay_coast_ledge_chest",
            MmSceneFlag::new(0x37, MmFlagType::Chest, 0x0000_0001),
        ),
        // ========================================================================
        // ZORA CAPE (Scene 0x38)
        // ========================================================================
        MmFlagMapping::new(
            "mm_zora_cape_underwater_chest",
            MmSceneFlag::new(0x38, MmFlagType::Chest, 0x0000_0001),
        ),
        MmFlagMapping::new(
            "mm_zora_cape_ledge_chest_1",
            MmSceneFlag::new(0x38, MmFlagType::Chest, 0x0000_0002),
        ),
        MmFlagMapping::new(
            "mm_zora_cape_ledge_chest_2",
            MmSceneFlag::new(0x38, MmFlagType::Chest, 0x0000_0004),
        ),
        // ========================================================================
        // PINNACLE ROCK (Scene 0x3F)
        // ========================================================================
        MmFlagMapping::new(
            "mm_pinnacle_rock_chest_1",
            MmSceneFlag::new(0x3F, MmFlagType::Chest, 0x0000_0001),
        ),
        MmFlagMapping::new(
            "mm_pinnacle_rock_chest_2",
            MmSceneFlag::new(0x3F, MmFlagType::Chest, 0x0000_0002),
        ),
        // ========================================================================
        // ROAD TO IKANA (Scene 0x47)
        // ========================================================================
        MmFlagMapping::new(
            "mm_road_to_ikana_chest",
            MmSceneFlag::new(0x47, MmFlagType::Chest, 0x0000_0001),
        ),
        // ========================================================================
        // DEKU PALACE GROTTO (Scene 0x59)
        // ========================================================================
        MmFlagMapping::new(
            "mm_deku_palace_grotto_chest",
            MmSceneFlag::new(0x59, MmFlagType::Chest, 0x0000_0001),
        ),
        // ========================================================================
        // BENEATH THE GRAVEYARD (Scene 0x07)
        // ========================================================================
        MmFlagMapping::new(
            "mm_beneath_the_graveyard_chest",
            MmSceneFlag::new(0x07, MmFlagType::Chest, 0x0000_0001),
        ),
        MmFlagMapping::new(
            "mm_beneath_the_graveyard_dampe_chest",
            MmSceneFlag::new(0x07, MmFlagType::Chest, 0x0000_0002),
        ),
        // ========================================================================
        // LENS OF TRUTH CAVE (Scene 0x17)
        // ========================================================================
        MmFlagMapping::new(
            "mm_lone_peak_shrine_lens_chest",
            MmSceneFlag::new(0x17, MmFlagType::Chest, 0x0000_0001),
        ),
        MmFlagMapping::new(
            "mm_lone_peak_shrine_boulder_chest",
            MmSceneFlag::new(0x17, MmFlagType::Chest, 0x0000_0002),
        ),
        MmFlagMapping::new(
            "mm_lone_peak_shrine_invisible_chest",
            MmSceneFlag::new(0x17, MmFlagType::Chest, 0x0000_0004),
        ),
        // ========================================================================
        // STONE TOWER EXTERIOR INVERTED (Scene 0x0F)
        // ========================================================================
        MmFlagMapping::new(
            "mm_stone_tower_inverted_chest_1",
            MmSceneFlag::new(0x0F, MmFlagType::Chest, 0x0000_0001),
        ),
        MmFlagMapping::new(
            "mm_stone_tower_inverted_chest_2",
            MmSceneFlag::new(0x0F, MmFlagType::Chest, 0x0000_0002),
        ),
        MmFlagMapping::new(
            "mm_stone_tower_inverted_chest_3",
            MmSceneFlag::new(0x0F, MmFlagType::Chest, 0x0000_0004),
        ),
        // ========================================================================
        // DOGGY RACETRACK (Scene 0x62)
        // ========================================================================
        MmFlagMapping::new(
            "mm_doggy_racetrack_chest",
            MmSceneFlag::new(0x62, MmFlagType::Chest, 0x0000_0001),
        ),
        // ========================================================================
        // OCEANSIDE SPIDER HOUSE (Scene 0x28)
        // ========================================================================
        MmFlagMapping::new(
            "mm_ocean_spider_house_chest_hp",
            MmSceneFlag::new(0x28, MmFlagType::Chest, 0x0000_0001),
        ),
    ]
}

/// Get MM collectible flag mappings.
///
/// Returns a vector of mappings from collectible location names to their scene flags.
/// This includes skull tokens (spider houses), heart pieces, and other collectible items.
#[must_use]
pub fn mm_collectible_mappings() -> Vec<MmFlagMapping> {
    vec![
        // ========================================================================
        // SWAMP SPIDER HOUSE SKULLTULAS (Scene 0x27)
        // ========================================================================
        MmFlagMapping::new(
            "mm_swamp_skulltula_main_room_near_ceiling",
            MmSceneFlag::new(0x27, MmFlagType::Collectible, 0x0000_0001),
        ),
        MmFlagMapping::new(
            "mm_swamp_skulltula_main_room_lower_right_soft_soil",
            MmSceneFlag::new(0x27, MmFlagType::Collectible, 0x0000_0002),
        ),
        MmFlagMapping::new(
            "mm_swamp_skulltula_main_room_lower_left_soft_soil",
            MmSceneFlag::new(0x27, MmFlagType::Collectible, 0x0000_0004),
        ),
        MmFlagMapping::new(
            "mm_swamp_skulltula_main_room_upper_soft_soil",
            MmSceneFlag::new(0x27, MmFlagType::Collectible, 0x0000_0008),
        ),
        MmFlagMapping::new(
            "mm_swamp_skulltula_main_room_upper_pillar",
            MmSceneFlag::new(0x27, MmFlagType::Collectible, 0x0000_0010),
        ),
        MmFlagMapping::new(
            "mm_swamp_skulltula_main_room_pillar",
            MmSceneFlag::new(0x27, MmFlagType::Collectible, 0x0000_0020),
        ),
        MmFlagMapping::new(
            "mm_swamp_skulltula_main_room_water",
            MmSceneFlag::new(0x27, MmFlagType::Collectible, 0x0000_0040),
        ),
        MmFlagMapping::new(
            "mm_swamp_skulltula_main_room_jar",
            MmSceneFlag::new(0x27, MmFlagType::Collectible, 0x0000_0080),
        ),
        MmFlagMapping::new(
            "mm_swamp_skulltula_gold_room_hive",
            MmSceneFlag::new(0x27, MmFlagType::Collectible, 0x0000_0100),
        ),
        MmFlagMapping::new(
            "mm_swamp_skulltula_gold_room_near_ceiling",
            MmSceneFlag::new(0x27, MmFlagType::Collectible, 0x0000_0200),
        ),
        MmFlagMapping::new(
            "mm_swamp_skulltula_gold_room_pillar",
            MmSceneFlag::new(0x27, MmFlagType::Collectible, 0x0000_0400),
        ),
        MmFlagMapping::new(
            "mm_swamp_skulltula_gold_room_wall",
            MmSceneFlag::new(0x27, MmFlagType::Collectible, 0x0000_0800),
        ),
        MmFlagMapping::new(
            "mm_swamp_skulltula_tree_room_hive",
            MmSceneFlag::new(0x27, MmFlagType::Collectible, 0x0000_1000),
        ),
        MmFlagMapping::new(
            "mm_swamp_skulltula_tree_room_grass_1",
            MmSceneFlag::new(0x27, MmFlagType::Collectible, 0x0000_2000),
        ),
        MmFlagMapping::new(
            "mm_swamp_skulltula_tree_room_grass_2",
            MmSceneFlag::new(0x27, MmFlagType::Collectible, 0x0000_4000),
        ),
        MmFlagMapping::new(
            "mm_swamp_skulltula_tree_room_tree_1",
            MmSceneFlag::new(0x27, MmFlagType::Collectible, 0x0000_8000),
        ),
        MmFlagMapping::new(
            "mm_swamp_skulltula_tree_room_tree_2",
            MmSceneFlag::new(0x27, MmFlagType::Collectible, 0x0001_0000),
        ),
        MmFlagMapping::new(
            "mm_swamp_skulltula_tree_room_tree_3",
            MmSceneFlag::new(0x27, MmFlagType::Collectible, 0x0002_0000),
        ),
        MmFlagMapping::new(
            "mm_swamp_skulltula_monument_room_lower_wall",
            MmSceneFlag::new(0x27, MmFlagType::Collectible, 0x0004_0000),
        ),
        MmFlagMapping::new(
            "mm_swamp_skulltula_monument_room_on_monument",
            MmSceneFlag::new(0x27, MmFlagType::Collectible, 0x0008_0000),
        ),
        MmFlagMapping::new(
            "mm_swamp_skulltula_monument_room_crate_1",
            MmSceneFlag::new(0x27, MmFlagType::Collectible, 0x0010_0000),
        ),
        MmFlagMapping::new(
            "mm_swamp_skulltula_monument_room_crate_2",
            MmSceneFlag::new(0x27, MmFlagType::Collectible, 0x0020_0000),
        ),
        MmFlagMapping::new(
            "mm_swamp_skulltula_monument_room_torch",
            MmSceneFlag::new(0x27, MmFlagType::Collectible, 0x0040_0000),
        ),
        MmFlagMapping::new(
            "mm_swamp_skulltula_pot_room_hive_1",
            MmSceneFlag::new(0x27, MmFlagType::Collectible, 0x0080_0000),
        ),
        MmFlagMapping::new(
            "mm_swamp_skulltula_pot_room_hive_2",
            MmSceneFlag::new(0x27, MmFlagType::Collectible, 0x0100_0000),
        ),
        MmFlagMapping::new(
            "mm_swamp_skulltula_pot_room_behind_vines",
            MmSceneFlag::new(0x27, MmFlagType::Collectible, 0x0200_0000),
        ),
        MmFlagMapping::new(
            "mm_swamp_skulltula_pot_room_pot_1",
            MmSceneFlag::new(0x27, MmFlagType::Collectible, 0x0400_0000),
        ),
        MmFlagMapping::new(
            "mm_swamp_skulltula_pot_room_pot_2",
            MmSceneFlag::new(0x27, MmFlagType::Collectible, 0x0800_0000),
        ),
        MmFlagMapping::new(
            "mm_swamp_skulltula_pot_room_jar",
            MmSceneFlag::new(0x27, MmFlagType::Collectible, 0x1000_0000),
        ),
        MmFlagMapping::new(
            "mm_swamp_skulltula_pot_room_wall",
            MmSceneFlag::new(0x27, MmFlagType::Collectible, 0x2000_0000),
        ),
        // ========================================================================
        // OCEANSIDE SPIDER HOUSE SKULLTULAS (Scene 0x28)
        // ========================================================================
        MmFlagMapping::new(
            "mm_ocean_skulltula_entrance_right_wall",
            MmSceneFlag::new(0x28, MmFlagType::Collectible, 0x0000_0001),
        ),
        MmFlagMapping::new(
            "mm_ocean_skulltula_entrance_left_wall",
            MmSceneFlag::new(0x28, MmFlagType::Collectible, 0x0000_0002),
        ),
        MmFlagMapping::new(
            "mm_ocean_skulltula_entrance_web",
            MmSceneFlag::new(0x28, MmFlagType::Collectible, 0x0000_0004),
        ),
        MmFlagMapping::new(
            "mm_ocean_skulltula_2nd_room_ceiling_edge",
            MmSceneFlag::new(0x28, MmFlagType::Collectible, 0x0000_0008),
        ),
        MmFlagMapping::new(
            "mm_ocean_skulltula_2nd_room_ceiling_plank",
            MmSceneFlag::new(0x28, MmFlagType::Collectible, 0x0000_0010),
        ),
        MmFlagMapping::new(
            "mm_ocean_skulltula_2nd_room_jar",
            MmSceneFlag::new(0x28, MmFlagType::Collectible, 0x0000_0020),
        ),
        MmFlagMapping::new(
            "mm_ocean_skulltula_2nd_room_webbed_hole",
            MmSceneFlag::new(0x28, MmFlagType::Collectible, 0x0000_0040),
        ),
        MmFlagMapping::new(
            "mm_ocean_skulltula_2nd_room_behind_skull_1",
            MmSceneFlag::new(0x28, MmFlagType::Collectible, 0x0000_0080),
        ),
        MmFlagMapping::new(
            "mm_ocean_skulltula_2nd_room_behind_skull_2",
            MmSceneFlag::new(0x28, MmFlagType::Collectible, 0x0000_0100),
        ),
        MmFlagMapping::new(
            "mm_ocean_skulltula_2nd_room_webbed_pot",
            MmSceneFlag::new(0x28, MmFlagType::Collectible, 0x0000_0200),
        ),
        MmFlagMapping::new(
            "mm_ocean_skulltula_2nd_room_upper_pot",
            MmSceneFlag::new(0x28, MmFlagType::Collectible, 0x0000_0400),
        ),
        MmFlagMapping::new(
            "mm_ocean_skulltula_2nd_room_lower_pot",
            MmSceneFlag::new(0x28, MmFlagType::Collectible, 0x0000_0800),
        ),
        MmFlagMapping::new(
            "mm_ocean_skulltula_library_hole_behind_picture",
            MmSceneFlag::new(0x28, MmFlagType::Collectible, 0x0000_1000),
        ),
        MmFlagMapping::new(
            "mm_ocean_skulltula_library_hole_behind_cabinet",
            MmSceneFlag::new(0x28, MmFlagType::Collectible, 0x0000_2000),
        ),
        MmFlagMapping::new(
            "mm_ocean_skulltula_library_on_corner_bookshelf",
            MmSceneFlag::new(0x28, MmFlagType::Collectible, 0x0000_4000),
        ),
        MmFlagMapping::new(
            "mm_ocean_skulltula_library_behind_picture",
            MmSceneFlag::new(0x28, MmFlagType::Collectible, 0x0000_8000),
        ),
        MmFlagMapping::new(
            "mm_ocean_skulltula_library_behind_bookcase_1",
            MmSceneFlag::new(0x28, MmFlagType::Collectible, 0x0001_0000),
        ),
        MmFlagMapping::new(
            "mm_ocean_skulltula_library_behind_bookcase_2",
            MmSceneFlag::new(0x28, MmFlagType::Collectible, 0x0002_0000),
        ),
        MmFlagMapping::new(
            "mm_ocean_skulltula_library_ceiling_edge",
            MmSceneFlag::new(0x28, MmFlagType::Collectible, 0x0004_0000),
        ),
        MmFlagMapping::new(
            "mm_ocean_skulltula_colored_skulls_chandelier_1",
            MmSceneFlag::new(0x28, MmFlagType::Collectible, 0x0008_0000),
        ),
        MmFlagMapping::new(
            "mm_ocean_skulltula_colored_skulls_chandelier_2",
            MmSceneFlag::new(0x28, MmFlagType::Collectible, 0x0010_0000),
        ),
        MmFlagMapping::new(
            "mm_ocean_skulltula_colored_skulls_chandelier_3",
            MmSceneFlag::new(0x28, MmFlagType::Collectible, 0x0020_0000),
        ),
        MmFlagMapping::new(
            "mm_ocean_skulltula_colored_skulls_behind_picture",
            MmSceneFlag::new(0x28, MmFlagType::Collectible, 0x0040_0000),
        ),
        MmFlagMapping::new(
            "mm_ocean_skulltula_colored_skulls_pot",
            MmSceneFlag::new(0x28, MmFlagType::Collectible, 0x0080_0000),
        ),
        MmFlagMapping::new(
            "mm_ocean_skulltula_colored_skulls_ceiling_edge",
            MmSceneFlag::new(0x28, MmFlagType::Collectible, 0x0100_0000),
        ),
        MmFlagMapping::new(
            "mm_ocean_skulltula_storage_room_behind_boat",
            MmSceneFlag::new(0x28, MmFlagType::Collectible, 0x0200_0000),
        ),
        MmFlagMapping::new(
            "mm_ocean_skulltula_storage_room_ceiling_web",
            MmSceneFlag::new(0x28, MmFlagType::Collectible, 0x0400_0000),
        ),
        MmFlagMapping::new(
            "mm_ocean_skulltula_storage_room_behind_crate",
            MmSceneFlag::new(0x28, MmFlagType::Collectible, 0x0800_0000),
        ),
        MmFlagMapping::new(
            "mm_ocean_skulltula_storage_room_crate",
            MmSceneFlag::new(0x28, MmFlagType::Collectible, 0x1000_0000),
        ),
        MmFlagMapping::new(
            "mm_ocean_skulltula_storage_room_jar",
            MmSceneFlag::new(0x28, MmFlagType::Collectible, 0x2000_0000),
        ),
        // ========================================================================
        // HEART PIECES
        // ========================================================================
        // Clock Town Area
        MmFlagMapping::new(
            "mm_clock_town_platform_hp",
            MmSceneFlag::new(0x6C, MmFlagType::Collectible, 0x0000_0001),
        ),
        MmFlagMapping::new(
            "mm_clock_town_tree_hp",
            MmSceneFlag::new(0x6D, MmFlagType::Collectible, 0x0000_0001),
        ),
        MmFlagMapping::new(
            "mm_clock_town_keaton_hp",
            MmSceneFlag::new(0x6D, MmFlagType::Collectible, 0x0000_0002),
        ),
        MmFlagMapping::new(
            "mm_clock_town_rosa_sisters_hp",
            MmSceneFlag::new(0x6F, MmFlagType::Collectible, 0x0000_0001),
        ),
        MmFlagMapping::new(
            "mm_post_office_hp",
            MmSceneFlag::new(0x30, MmFlagType::Collectible, 0x0000_0001),
        ),
        MmFlagMapping::new(
            "mm_swordsman_school_hp",
            MmSceneFlag::new(0x26, MmFlagType::Collectible, 0x0000_0002),
        ),
        MmFlagMapping::new(
            "mm_mayors_office_hp",
            MmSceneFlag::new(0x4E, MmFlagType::Collectible, 0x0000_0001),
        ),
        MmFlagMapping::new(
            "mm_stock_pot_inn_grandma_hp_1",
            MmSceneFlag::new(0x4D, MmFlagType::Collectible, 0x0000_0001),
        ),
        MmFlagMapping::new(
            "mm_stock_pot_inn_grandma_hp_2",
            MmSceneFlag::new(0x4D, MmFlagType::Collectible, 0x0000_0002),
        ),
        MmFlagMapping::new(
            "mm_stock_pot_inn_hp",
            MmSceneFlag::new(0x4D, MmFlagType::Collectible, 0x0000_0004),
        ),
        // Termina Field / Overworld
        MmFlagMapping::new(
            "mm_termina_field_gossip_stones_hp",
            MmSceneFlag::new(0x54, MmFlagType::Collectible, 0x0000_0001),
        ),
        // Southern Swamp Area
        MmFlagMapping::new(
            "mm_deku_palace_hp",
            MmSceneFlag::new(0x59, MmFlagType::Collectible, 0x0000_0001),
        ),
        MmFlagMapping::new(
            "mm_southern_swamp_hp",
            MmSceneFlag::new(0x55, MmFlagType::Collectible, 0x0000_0001),
        ),
        MmFlagMapping::new(
            "mm_road_to_southern_swamp_hp",
            MmSceneFlag::new(0x0D, MmFlagType::Collectible, 0x0000_0001),
        ),
        // Mountain Area
        MmFlagMapping::new(
            "mm_goron_village_hp",
            MmSceneFlag::new(0x6A, MmFlagType::Collectible, 0x0000_0001),
        ),
        MmFlagMapping::new(
            "mm_mountain_village_frog_choir_hp",
            MmSceneFlag::new(0x65, MmFlagType::Collectible, 0x0000_0001),
        ),
        MmFlagMapping::new(
            "mm_path_to_snowhead_hp",
            MmSceneFlag::new(0x21, MmFlagType::Collectible, 0x0000_0001),
        ),
        // Great Bay Area
        MmFlagMapping::new(
            "mm_great_bay_coast_hp",
            MmSceneFlag::new(0x37, MmFlagType::Collectible, 0x0000_0001),
        ),
        MmFlagMapping::new(
            "mm_great_bay_coast_fisherman_hp",
            MmSceneFlag::new(0x37, MmFlagType::Collectible, 0x0000_0002),
        ),
        MmFlagMapping::new(
            "mm_laboratory_fish_hp",
            MmSceneFlag::new(0x37, MmFlagType::Collectible, 0x0000_0004),
        ),
        MmFlagMapping::new(
            "mm_pinnacle_rock_hp",
            MmSceneFlag::new(0x3F, MmFlagType::Collectible, 0x0000_0001),
        ),
        MmFlagMapping::new(
            "mm_zora_cape_waterfall_hp",
            MmSceneFlag::new(0x38, MmFlagType::Collectible, 0x0000_0001),
        ),
        MmFlagMapping::new(
            "mm_zora_hall_evan_hp",
            MmSceneFlag::new(0x39, MmFlagType::Collectible, 0x0000_0001),
        ),
        MmFlagMapping::new(
            "mm_zora_hall_scrub_hp",
            MmSceneFlag::new(0x39, MmFlagType::Collectible, 0x0000_0002),
        ),
        // Ikana Area
        MmFlagMapping::new(
            "mm_ikana_valley_scrub_hp",
            MmSceneFlag::new(0x46, MmFlagType::Collectible, 0x0000_0001),
        ),
        MmFlagMapping::new(
            "mm_ghost_hut_hp",
            MmSceneFlag::new(0x46, MmFlagType::Collectible, 0x0000_0002),
        ),
        MmFlagMapping::new(
            "mm_beneath_the_graveyard_hp",
            MmSceneFlag::new(0x07, MmFlagType::Collectible, 0x0000_0001),
        ),
        MmFlagMapping::new(
            "mm_ancient_castle_of_ikana_hp",
            MmSceneFlag::new(0x11, MmFlagType::Collectible, 0x0000_0001),
        ),
        // Romani Ranch Area
        MmFlagMapping::new(
            "mm_doggy_racetrack_hp",
            MmSceneFlag::new(0x62, MmFlagType::Collectible, 0x0000_0001),
        ),
        // Great Fairy Rewards (collectible-based)
        MmFlagMapping::new(
            "mm_clock_town_great_fairy_reward",
            MmSceneFlag::new(0x26, MmFlagType::Collectible, 0x0000_0001),
        ),
        MmFlagMapping::new(
            "mm_woodfall_great_fairy_reward",
            MmSceneFlag::new(0x20, MmFlagType::Collectible, 0x0000_0001),
        ),
        MmFlagMapping::new(
            "mm_snowhead_great_fairy_reward",
            MmSceneFlag::new(0x25, MmFlagType::Collectible, 0x0000_0001),
        ),
        MmFlagMapping::new(
            "mm_great_bay_great_fairy_reward",
            MmSceneFlag::new(0x60, MmFlagType::Collectible, 0x0000_0001),
        ),
        MmFlagMapping::new(
            "mm_ikana_great_fairy_reward",
            MmSceneFlag::new(0x15, MmFlagType::Collectible, 0x0000_0001),
        ),
        // Spider House Rewards
        MmFlagMapping::new(
            "mm_swamp_spider_house_reward",
            MmSceneFlag::new(0x27, MmFlagType::Collectible, 0x4000_0000),
        ),
        MmFlagMapping::new(
            "mm_ocean_spider_house_reward",
            MmSceneFlag::new(0x28, MmFlagType::Collectible, 0x4000_0000),
        ),
    ]
}

/// Get MM special flag mappings.
///
/// Returns a vector of mappings for special collectibles that don't fit into
/// the standard chest/collectible categories. This includes:
/// - Stray fairies (dungeon and overworld)
/// - Other unique collectibles
///
/// # Note
///
/// This is a stub function that returns an empty vector. Actual special mappings
/// will be populated in a future issue.
#[must_use]
pub fn mm_special_mappings() -> Vec<MmFlagMapping> {
    // TODO: Populate with actual special mappings (stray fairies, etc.)
    Vec::new()
}

/// Get all MM flag mappings combined.
///
/// Returns a vector containing all chest, collectible, and special mappings.
#[must_use]
pub fn mm_all_mappings() -> Vec<MmFlagMapping> {
    let mut all = mm_chest_mappings();
    all.extend(mm_collectible_mappings());
    all.extend(mm_special_mappings());
    all
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_flag_type_offsets() {
        assert_eq!(MmFlagType::Chest.offset(), 0x00);
        assert_eq!(MmFlagType::Switch.offset(), 0x04);
        assert_eq!(MmFlagType::RoomClear.offset(), 0x08);
        assert_eq!(MmFlagType::Collectible.offset(), 0x0C);
        assert_eq!(MmFlagType::Special.offset(), 0x10);
    }

    #[test]
    fn test_scene_flag_save_offset() {
        // Scene 0, chest flag
        let flag = MmSceneFlag::new(0, MmFlagType::Chest, 0x01);
        assert_eq!(flag.save_offset(), 0x00);

        // Scene 1, collectible flag
        let flag = MmSceneFlag::new(1, MmFlagType::Collectible, 0x01);
        // Scene 1 offset = 0x14, collectible offset = 0x0C
        assert_eq!(flag.save_offset(), 0x14 + 0x0C);

        // Scene 0x6C (Clock Town), chest flag
        let flag = MmSceneFlag::new(0x6C, MmFlagType::Chest, 0x01);
        assert_eq!(flag.save_offset(), 0x6C * 0x14);
    }

    #[test]
    fn test_flag_mapping_creation() {
        let mapping = MmFlagMapping::new(
            "Test Location",
            MmSceneFlag::new(0x6C, MmFlagType::Chest, 0x01),
        );
        assert_eq!(mapping.location_name, "Test Location");
        assert_eq!(mapping.flag.scene_id, 0x6C);
        assert_eq!(mapping.flag.flag_type, MmFlagType::Chest);
        assert_eq!(mapping.flag.bit_mask, 0x01);
    }

    #[test]
    fn test_chest_mappings_populated() {
        let chests = mm_chest_mappings();
        // Should have mappings for all 4 main dungeons plus overworld
        assert!(!chests.is_empty(), "Chest mappings should not be empty");
        // Verify we have Woodfall Temple chests
        assert!(
            chests
                .iter()
                .any(|m| m.location_name.contains("woodfall_temple")),
            "Should have Woodfall Temple chests"
        );
        // Verify we have Snowhead Temple chests
        assert!(
            chests
                .iter()
                .any(|m| m.location_name.contains("snowhead_temple")),
            "Should have Snowhead Temple chests"
        );
        // Verify we have Great Bay Temple chests
        assert!(
            chests
                .iter()
                .any(|m| m.location_name.contains("great_bay_temple")),
            "Should have Great Bay Temple chests"
        );
        // Verify we have Stone Tower Temple chests
        assert!(
            chests
                .iter()
                .any(|m| m.location_name.contains("stone_tower_temple")),
            "Should have Stone Tower Temple chests"
        );
    }

    #[test]
    fn test_collectible_mappings_populated() {
        let collectibles = mm_collectible_mappings();
        // Should have skulltulas and heart pieces
        assert!(
            !collectibles.is_empty(),
            "Collectible mappings should not be empty"
        );
        // Verify we have swamp spider house skulltulas
        assert!(
            collectibles
                .iter()
                .any(|m| m.location_name.contains("swamp_skulltula")),
            "Should have Swamp Spider House skulltulas"
        );
        // Verify we have ocean spider house skulltulas
        assert!(
            collectibles
                .iter()
                .any(|m| m.location_name.contains("ocean_skulltula")),
            "Should have Ocean Spider House skulltulas"
        );
        // Verify we have heart pieces
        assert!(
            collectibles.iter().any(|m| m.location_name.contains("_hp")),
            "Should have heart pieces"
        );
    }

    #[test]
    fn test_special_mappings_stub() {
        // Special mappings (stray fairies, etc.) are still a stub
        assert!(mm_special_mappings().is_empty());
    }

    #[test]
    fn test_all_mappings_combined() {
        let all = mm_all_mappings();
        let chests = mm_chest_mappings();
        let collectibles = mm_collectible_mappings();
        let special = mm_special_mappings();

        assert_eq!(all.len(), chests.len() + collectibles.len() + special.len());
        assert!(!all.is_empty(), "All mappings should not be empty");
    }

    #[test]
    fn test_offsets_constants() {
        assert_eq!(offsets::MM_SAVE_BASE, 0x1EF670);
        assert_eq!(offsets::SCENE_SIZE, 0x14);
        assert_eq!(offsets::NUM_SCENES, 0x78);
    }
}
