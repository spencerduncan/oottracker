//! OoT Flag Data Structures and Location Mappings
//!
//! This module provides the foundation for detecting location checks by mapping
//! location IDs to memory flag addresses. It defines flag types, scene IDs, and
//! stub mappings for all OoT locations imported from OoTMM world data.
//!
//! # Flag Structure (based on OoTMM research)
//!
//! OoT save data contains several types of flags stored in different memory regions:
//!
//! ## Scene Flags (per-scene, stored in save context at 0x00D4)
//!
//! Each scene has a 0x1C byte entry containing:
//! - `chests` (u32 at offset 0x00): Opened chest flags
//! - `switches` (u32 at offset 0x04): Switch/trigger flags
//! - `room_clear` (u32 at offset 0x08): Room clear flags
//! - `collectible` (u32 at offset 0x0C): Collectible item flags
//! - `unused` (u32 at offset 0x10): Unused flags (sometimes repurposed)
//! - `visited_rooms` (u32 at offset 0x14): Room visited flags
//! - `visited_floors` (u32 at offset 0x18): Floor visited flags
//!
//! ## Global Flags
//!
//! - Gold Skulltulas: Stored in save context at 0x0E9C (0x18 bytes)
//! - Event flags (event_chk_inf): Global event tracking at 0x0ED4 (0x1C bytes)
//! - Item get flags (item_get_inf): Item obtained tracking at 0x0EF0 (0x08 bytes)
//! - Info table (inf_table): NPC/misc flags at 0x0EF8 (0x3C bytes)
//!
//! # Usage
//!
//! ```ignore
//! use oottracker::flag_mapping::{FlagMapping, get_mapping, get_all_oot_mappings};
//!
//! // Get mapping for a specific location
//! if let Some(mapping) = get_mapping("oot_deku_tree_compass_chest") {
//!     println!("Location {} is in scene {:?}", mapping.location_id, mapping.scene_id);
//! }
//!
//! // Get all OoT location mappings
//! for mapping in get_all_oot_mappings() {
//!     println!("{}: {:?}", mapping.location_id, mapping.flag_type);
//! }
//! ```

use std::collections::HashMap;

use once_cell::sync::Lazy;

use ootmm::region::Game;
use ootmm::settings::RandomizerSettings;
use ootr::{
    model::{Dungeon, MainDungeon},
    region::Mq,
};

use crate::world_database;

// ============================================================================
// Flag Type Definition
// ============================================================================

/// Types of flags used to track location checks in OoT save data.
///
/// Each location in the game is tracked by one of these flag types,
/// stored in specific memory regions within the save context.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum FlagType {
    /// Chest opened flags (scene flags offset 0x00).
    /// Each bit represents a chest in the scene.
    Chest,

    /// Switch/trigger flags (scene flags offset 0x04).
    /// Includes crystal switches, floor switches, etc.
    Switch,

    /// Room clear flags (scene flags offset 0x08).
    /// Set when all enemies in a room are defeated.
    RoomClear,

    /// Collectible item flags (scene flags offset 0x0C).
    /// Freestanding items, rupees, hearts, etc.
    Collectible,

    /// Gold Skulltula flags (separate from scene flags).
    /// Stored in dedicated skulltula section at 0x0E9C.
    GoldSkulltula,

    /// Event check flags (event_chk_inf).
    /// Global events like cutscenes watched, NPCs talked to.
    EventChkInf,

    /// Item get flags (item_get_inf).
    /// Tracks specific item acquisitions.
    ItemGetInf,

    /// Info table flags (inf_table).
    /// NPC conversation flags, misc game state.
    InfTable,

    /// Shop item flags.
    /// Tracks purchased shop items.
    Shop,

    /// Scrub/merchant purchase flags.
    /// Business scrubs and other merchants.
    Scrub,

    /// Great Fairy reward flags.
    /// Tracks fairy fountain upgrades received.
    GreatFairy,

    /// Boss defeated flags.
    /// Typically stored in switch or event flags.
    Boss,

    /// Song learned flags.
    /// Stored in quest items bitfield.
    Song,

    /// Fishing pond flags.
    /// Special handling for fishing rewards.
    Fishing,

    /// Cow/milk flags.
    /// Playing Epona's Song to cows.
    Cow,

    /// Gossip stone flags.
    /// Hints from gossip stones (if shuffled).
    GossipStone,
}

impl FlagType {
    /// Returns the byte offset within scene flags for scene-based flag types.
    ///
    /// Returns `None` for global flag types that aren't stored per-scene.
    #[must_use]
    pub const fn scene_offset(&self) -> Option<usize> {
        match self {
            FlagType::Chest => Some(0x00),
            FlagType::Switch => Some(0x04),
            FlagType::RoomClear => Some(0x08),
            FlagType::Collectible => Some(0x0C),
            // These are global, not per-scene
            FlagType::GoldSkulltula
            | FlagType::EventChkInf
            | FlagType::ItemGetInf
            | FlagType::InfTable
            | FlagType::Shop
            | FlagType::Scrub
            | FlagType::GreatFairy
            | FlagType::Boss
            | FlagType::Song
            | FlagType::Fishing
            | FlagType::Cow
            | FlagType::GossipStone => None,
        }
    }

    /// Returns whether this flag type is stored per-scene.
    #[must_use]
    pub const fn is_scene_based(&self) -> bool {
        self.scene_offset().is_some()
    }
}

// ============================================================================
// Scene ID Constants
// ============================================================================

/// OoT Scene IDs.
///
/// Scene IDs correspond to the index in the scene flag array.
/// Reference: <https://wiki.cloudmodding.com/oot/Scene_Table/NTSC_1.0>
pub mod scene {
    // Dungeons
    pub const DEKU_TREE: u8 = 0x00;
    pub const DODONGOS_CAVERN: u8 = 0x01;
    pub const JABU_JABU: u8 = 0x02;
    pub const FOREST_TEMPLE: u8 = 0x03;
    pub const FIRE_TEMPLE: u8 = 0x04;
    pub const WATER_TEMPLE: u8 = 0x05;
    pub const SPIRIT_TEMPLE: u8 = 0x06;
    pub const SHADOW_TEMPLE: u8 = 0x07;
    pub const BOTTOM_OF_THE_WELL: u8 = 0x08;
    pub const ICE_CAVERN: u8 = 0x09;
    pub const GANONS_TOWER: u8 = 0x0A;
    pub const GERUDO_TRAINING_GROUND: u8 = 0x0B;
    pub const THIEVES_HIDEOUT: u8 = 0x0C;
    pub const GANONS_CASTLE: u8 = 0x0D;
    pub const GANONS_TOWER_COLLAPSING: u8 = 0x0E;
    pub const GANONS_CASTLE_COLLAPSING: u8 = 0x0F;
    pub const TREASURE_CHEST_GAME: u8 = 0x10;

    // Boss Rooms
    pub const DEKU_TREE_BOSS: u8 = 0x11;
    pub const DODONGOS_CAVERN_BOSS: u8 = 0x12;
    pub const JABU_JABU_BOSS: u8 = 0x13;
    pub const FOREST_TEMPLE_BOSS: u8 = 0x14;
    pub const FIRE_TEMPLE_BOSS: u8 = 0x15;
    pub const WATER_TEMPLE_BOSS: u8 = 0x16;
    pub const SPIRIT_TEMPLE_BOSS: u8 = 0x17;
    pub const SHADOW_TEMPLE_BOSS: u8 = 0x18;
    pub const GANONDORF_BOSS: u8 = 0x19;
    pub const GANON_BOSS: u8 = 0x1A;
    pub const TOWER_COLLAPSE_EXTERIOR: u8 = 0x1B;

    // Overworld
    pub const MARKET_ENTRANCE_DAY: u8 = 0x1C;
    pub const MARKET_ENTRANCE_NIGHT: u8 = 0x1D;
    pub const MARKET_ENTRANCE_RUINS: u8 = 0x1E;
    pub const BACK_ALLEY_DAY: u8 = 0x1F;
    pub const BACK_ALLEY_NIGHT: u8 = 0x20;
    pub const MARKET_DAY: u8 = 0x21;
    pub const MARKET_NIGHT: u8 = 0x22;
    pub const MARKET_RUINS: u8 = 0x23;
    pub const TEMPLE_OF_TIME_EXTERIOR_DAY: u8 = 0x24;
    pub const TEMPLE_OF_TIME_EXTERIOR_NIGHT: u8 = 0x25;
    pub const TEMPLE_OF_TIME_EXTERIOR_RUINS: u8 = 0x26;
    pub const KNOW_IT_ALL_BROTHERS_HOUSE: u8 = 0x27;
    pub const MIDOS_HOUSE: u8 = 0x28;
    pub const SARIAS_HOUSE: u8 = 0x29;
    pub const TWINS_HOUSE: u8 = 0x2A;
    pub const LINKS_HOUSE: u8 = 0x2B;
    pub const KAKARIKO_HOUSE_1: u8 = 0x2C;
    pub const BACK_ALLEY_HOUSE: u8 = 0x2D;
    pub const BAZAAR: u8 = 0x2E;
    pub const KOKIRI_SHOP: u8 = 0x2F;
    pub const GORON_SHOP: u8 = 0x30;
    pub const ZORA_SHOP: u8 = 0x31;
    pub const KAKARIKO_POTION_SHOP: u8 = 0x32;
    pub const MARKET_POTION_SHOP: u8 = 0x33;
    pub const BOMBCHU_SHOP: u8 = 0x34;
    pub const HAPPY_MASK_SHOP: u8 = 0x35;
    pub const GERUDO_VALLEY_TENT: u8 = 0x36;
    pub const IMPAS_HOUSE: u8 = 0x37;
    pub const LAKESIDE_LABORATORY: u8 = 0x38;
    pub const CARPENTERS_TENT: u8 = 0x39;
    pub const GRAVEKEEPERS_HUT: u8 = 0x3A;
    pub const GREAT_FAIRY_FOUNTAIN_UPGRADES: u8 = 0x3B;
    pub const FAIRY_FOUNTAIN: u8 = 0x3C;
    pub const GREAT_FAIRY_FOUNTAIN_SPELLS: u8 = 0x3D;
    pub const GROTTOS: u8 = 0x3E;
    pub const GRAVE_HEART_PIECE: u8 = 0x3F;
    pub const GRAVE_SHIELD: u8 = 0x40;
    pub const ROYAL_FAMILYS_TOMB: u8 = 0x41;
    pub const SHOOTING_GALLERY: u8 = 0x42;
    pub const TEMPLE_OF_TIME: u8 = 0x43;
    pub const CHAMBER_OF_SAGES: u8 = 0x44;
    pub const CASTLE_HEDGE_MAZE_DAY: u8 = 0x45;
    pub const CASTLE_HEDGE_MAZE_NIGHT: u8 = 0x46;
    pub const CUTSCENE_MAP: u8 = 0x47;
    pub const WINDMILL_AND_DAMPES_GRAVE: u8 = 0x48;
    pub const FISHING_POND: u8 = 0x49;
    pub const CASTLE_COURTYARD: u8 = 0x4A;
    pub const BOMBCHU_BOWLING: u8 = 0x4B;
    pub const LON_LON_RANCH_TOWER: u8 = 0x4C;
    pub const LON_LON_RANCH_HOUSE: u8 = 0x4D;
    pub const GUARD_HOUSE: u8 = 0x4E;
    pub const KAKARIKO_HOUSE_2: u8 = 0x4F;
    pub const KAKARIKO_HOUSE_3: u8 = 0x50;

    // Main Overworld Areas
    pub const HYRULE_FIELD: u8 = 0x51;
    pub const KAKARIKO_VILLAGE: u8 = 0x52;
    pub const GRAVEYARD: u8 = 0x53;
    pub const ZORA_RIVER: u8 = 0x54;
    pub const KOKIRI_FOREST: u8 = 0x55;
    pub const SACRED_FOREST_MEADOW: u8 = 0x56;
    pub const LAKE_HYLIA: u8 = 0x57;
    pub const ZORAS_DOMAIN: u8 = 0x58;
    pub const ZORAS_FOUNTAIN: u8 = 0x59;
    pub const GERUDO_VALLEY: u8 = 0x5A;
    pub const LOST_WOODS: u8 = 0x5B;
    pub const DESERT_COLOSSUS: u8 = 0x5C;
    pub const GERUDO_FORTRESS: u8 = 0x5D;
    pub const HAUNTED_WASTELAND: u8 = 0x5E;
    pub const HYRULE_CASTLE: u8 = 0x5F;
    pub const DEATH_MOUNTAIN_TRAIL: u8 = 0x60;
    pub const DEATH_MOUNTAIN_CRATER: u8 = 0x61;
    pub const GORON_CITY: u8 = 0x62;
    pub const LON_LON_RANCH: u8 = 0x63;
    pub const OUTSIDE_GANONS_CASTLE: u8 = 0x64;

    /// Maximum scene ID for OoT.
    pub const MAX_SCENE_ID: u8 = 0x64;

    /// Number of scenes in OoT.
    pub const SCENE_COUNT: usize = 101;
}

// ============================================================================
// Flag Mapping Structure
// ============================================================================

/// Mapping from a location ID to its flag address in memory.
///
/// This struct represents either a complete mapping (with scene_id, flag_type,
/// and flag_bit populated) or a stub mapping (with all optional fields as None).
///
/// Stub mappings are generated for all locations from world data and serve as
/// placeholders until the actual flag addresses are researched and filled in.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FlagMapping {
    /// The unique location identifier from OoTMM world data.
    pub location_id: &'static str,

    /// The scene ID where this flag is stored (None for unmapped stubs or global flags).
    pub scene_id: Option<u8>,

    /// The type of flag used for this location (None for unmapped stubs).
    pub flag_type: Option<FlagType>,

    /// The bit position within the flag word (None for unmapped stubs).
    /// For 32-bit flag words, this is typically 0-31 representing a single bit,
    /// or a full bitmask for multi-bit values.
    pub flag_bit: Option<u32>,
}

impl FlagMapping {
    /// Creates a new unmapped stub mapping for a location.
    #[must_use]
    pub const fn stub(location_id: &'static str) -> Self {
        Self {
            location_id,
            scene_id: None,
            flag_type: None,
            flag_bit: None,
        }
    }

    /// Creates a new fully mapped location.
    #[must_use]
    pub const fn mapped(
        location_id: &'static str,
        scene_id: u8,
        flag_type: FlagType,
        flag_bit: u32,
    ) -> Self {
        Self {
            location_id,
            scene_id: Some(scene_id),
            flag_type: Some(flag_type),
            flag_bit: Some(flag_bit),
        }
    }

    /// Creates a new global flag mapping (no scene_id).
    #[must_use]
    pub const fn global(location_id: &'static str, flag_type: FlagType, flag_bit: u32) -> Self {
        Self {
            location_id,
            scene_id: None,
            flag_type: Some(flag_type),
            flag_bit: Some(flag_bit),
        }
    }

    /// Returns whether this is an unmapped stub.
    #[must_use]
    pub const fn is_stub(&self) -> bool {
        self.flag_type.is_none()
    }

    /// Returns whether this mapping is complete (has flag type and bit).
    #[must_use]
    pub const fn is_mapped(&self) -> bool {
        self.flag_type.is_some() && self.flag_bit.is_some()
    }
}

// ============================================================================
// Static Mapping Tables
// ============================================================================

/// All OoT location IDs extracted from embedded world data.
///
/// This list is generated at compile time from the OoTMM YAML files.
static OOT_LOCATION_IDS: Lazy<Vec<&'static str>> = Lazy::new(|| {
    let db = ootmm::embedded_data::create_world_database()
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
static OOT_MAPPINGS: Lazy<HashMap<&'static str, FlagMapping>> = Lazy::new(|| {
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

// ============================================================================
// Public API
// ============================================================================

/// Returns the flag mapping for a location ID, if it exists.
///
/// Returns `Some(mapping)` if the location exists (even if unmapped stub),
/// or `None` if the location ID is not recognized.
#[must_use]
pub fn get_mapping(location_id: &str) -> Option<&'static FlagMapping> {
    OOT_MAPPINGS.get(location_id)
}

/// Returns an iterator over all OoT location mappings.
///
/// This includes both mapped locations and unmapped stubs.
pub fn get_all_oot_mappings() -> impl Iterator<Item = &'static FlagMapping> {
    OOT_MAPPINGS.values()
}

/// Returns the count of all OoT locations.
#[must_use]
pub fn oot_location_count() -> usize {
    OOT_MAPPINGS.len()
}

/// Returns the count of mapped (non-stub) OoT locations.
#[must_use]
pub fn oot_mapped_count() -> usize {
    OOT_MAPPINGS.values().filter(|m| m.is_mapped()).count()
}

/// Returns the count of unmapped stub locations.
#[must_use]
pub fn oot_stub_count() -> usize {
    OOT_MAPPINGS.values().filter(|m| m.is_stub()).count()
}

/// Returns an iterator over only the mapped (non-stub) locations.
pub fn get_mapped_locations() -> impl Iterator<Item = &'static FlagMapping> {
    OOT_MAPPINGS.values().filter(|m| m.is_mapped())
}

/// Returns an iterator over only the stub locations.
pub fn get_stub_locations() -> impl Iterator<Item = &'static FlagMapping> {
    OOT_MAPPINGS.values().filter(|m| m.is_stub())
}

/// Returns all mappings for a specific scene.
pub fn get_mappings_for_scene(scene_id: u8) -> impl Iterator<Item = &'static FlagMapping> {
    OOT_MAPPINGS
        .values()
        .filter(move |m| m.scene_id == Some(scene_id))
}

/// Returns all mappings for a specific flag type.
pub fn get_mappings_by_flag_type(
    flag_type: FlagType,
) -> impl Iterator<Item = &'static FlagMapping> {
    OOT_MAPPINGS
        .values()
        .filter(move |m| m.flag_type == Some(flag_type))
}

/// Returns all OoT location IDs from the world data.
pub fn get_all_oot_location_ids() -> impl Iterator<Item = &'static str> {
    OOT_LOCATION_IDS.iter().copied()
}

// ============================================================================
// Master Quest Integration
// ============================================================================

// Re-export MqDungeon for convenience
pub use ootmm::settings::MqDungeon;

/// Determines if an MQ location should be active based on settings.
///
/// This function handles MQ locations that `MqDungeon::from_location_id` might not
/// recognize due to non-standard naming patterns (e.g., `mq_oot_mq_ganon_pot_*`
/// instead of `mq_oot_mq_ganon_castle_*`).
fn is_mq_location_active(location_id: &str, settings: &RandomizerSettings) -> bool {
    // First try the standard detection
    if let Some(dungeon) = MqDungeon::from_location_id(location_id) {
        return settings.is_dungeon_mq(dungeon);
    }

    // For MQ locations that from_location_id doesn't recognize,
    // try additional pattern matching based on dungeon names in the location ID
    // Pattern: mq_oot_mq_<dungeon_shortname>_*

    // Ganon's Castle locations (handles mq_oot_mq_ganon_pot_* pattern)
    if location_id.contains("_ganon_") {
        return settings.is_dungeon_mq(MqDungeon::GanonsCastle);
    }

    // If we can't determine the dungeon, the MQ location should not be active
    // with default settings (conservative approach)
    false
}

/// Returns an iterator over active location mappings based on MQ settings.
///
/// This filters out locations that belong to dungeons where the MQ setting
/// doesn't match the location type (vanilla vs MQ).
///
/// # Arguments
///
/// * `settings` - The randomizer settings containing MQ dungeon selections
///
/// # Example
///
/// ```ignore
/// use oottracker::flag_mapping::get_active_mappings;
/// use ootmm::settings::RandomizerSettings;
///
/// let settings = RandomizerSettings::default();
/// for mapping in get_active_mappings(&settings) {
///     println!("{}: {:?}", mapping.location_id, mapping.flag_type);
/// }
/// ```
pub fn get_active_mappings(
    settings: &RandomizerSettings,
) -> impl Iterator<Item = &'static FlagMapping> + '_ {
    OOT_MAPPINGS
        .values()
        .filter(move |m| is_location_active_for_settings(m.location_id, settings))
}

/// Returns the flag mapping for a location, considering MQ settings.
///
/// This function checks if the location is in an MQ-able dungeon and
/// returns the mapping only if the location matches the current MQ setting.
///
/// # Arguments
///
/// * `location_id` - The location ID to look up
/// * `settings` - The randomizer settings containing MQ dungeon selections
///
/// # Returns
///
/// `Some(mapping)` if the location exists and is active for current settings,
/// `None` if the location doesn't exist or is inactive due to MQ settings.
#[must_use]
pub fn get_active_mapping(
    location_id: &str,
    settings: &RandomizerSettings,
) -> Option<&'static FlagMapping> {
    let mapping = OOT_MAPPINGS.get(location_id)?;
    if is_location_active_for_settings(location_id, settings) {
        Some(mapping)
    } else {
        None
    }
}

/// Helper to check if a location is active for given settings.
fn is_location_active_for_settings(location_id: &str, settings: &RandomizerSettings) -> bool {
    if location_id.starts_with("mq_oot_") {
        is_mq_location_active(location_id, settings)
    } else {
        settings.is_location_active(location_id)
    }
}

/// Returns the count of active (non-MQ-filtered) locations for given settings.
#[must_use]
pub fn active_location_count(settings: &RandomizerSettings) -> usize {
    OOT_MAPPINGS
        .values()
        .filter(|m| is_location_active_for_settings(m.location_id, settings))
        .count()
}

/// Returns the count of active mapped (non-stub) locations for given settings.
#[must_use]
pub fn active_mapped_count(settings: &RandomizerSettings) -> usize {
    OOT_MAPPINGS
        .values()
        .filter(|m| m.is_mapped() && is_location_active_for_settings(m.location_id, settings))
        .count()
}

/// Returns an iterator over active mapped (non-stub) locations for given settings.
pub fn get_active_mapped_locations(
    settings: &RandomizerSettings,
) -> impl Iterator<Item = &'static FlagMapping> + '_ {
    OOT_MAPPINGS
        .values()
        .filter(move |m| m.is_mapped() && is_location_active_for_settings(m.location_id, settings))
}

/// Returns all mappings for a specific dungeon based on its MQ status.
///
/// This returns either vanilla or MQ mappings depending on the dungeon's
/// setting in the provided settings.
pub fn get_dungeon_mappings(
    dungeon: MqDungeon,
    settings: &RandomizerSettings,
) -> impl Iterator<Item = &'static FlagMapping> + '_ {
    let prefix = settings.get_dungeon_location_prefix(dungeon);
    OOT_MAPPINGS
        .values()
        .filter(move |m| m.location_id.starts_with(prefix))
}

// ============================================================================
// MQ Settings Conversion from Knowledge
// ============================================================================

/// Converts a Dungeon enum to the corresponding MqDungeon enum.
///
/// This mapping is needed to bridge between the ootr Dungeon enum used in
/// Knowledge.mq and the ootmm MqDungeon enum used in RandomizerSettings.
fn dungeon_to_mq_dungeon(dungeon: &Dungeon) -> MqDungeon {
    match dungeon {
        Dungeon::Main(MainDungeon::DekuTree) => MqDungeon::DekuTree,
        Dungeon::Main(MainDungeon::DodongosCavern) => MqDungeon::DodongosCavern,
        Dungeon::Main(MainDungeon::JabuJabu) => MqDungeon::JabuJabu,
        Dungeon::Main(MainDungeon::ForestTemple) => MqDungeon::ForestTemple,
        Dungeon::Main(MainDungeon::FireTemple) => MqDungeon::FireTemple,
        Dungeon::Main(MainDungeon::WaterTemple) => MqDungeon::WaterTemple,
        Dungeon::Main(MainDungeon::ShadowTemple) => MqDungeon::ShadowTemple,
        Dungeon::Main(MainDungeon::SpiritTemple) => MqDungeon::SpiritTemple,
        Dungeon::IceCavern => MqDungeon::IceCavern,
        Dungeon::BottomOfTheWell => MqDungeon::BottomOfTheWell,
        Dungeon::GerudoTrainingGround => MqDungeon::GerudoTrainingGround,
        Dungeon::GanonsCastle => MqDungeon::GanonsCastle,
    }
}

/// Creates RandomizerSettings from Knowledge MQ settings.
///
/// This function converts the `Knowledge.mq` HashMap<Dungeon, Mq> into a
/// `RandomizerSettings` struct with the appropriate MQ dungeon selections.
///
/// # Arguments
///
/// * `mq_settings` - A HashMap mapping dungeons to their MQ status from Knowledge
///
/// # Returns
///
/// A `RandomizerSettings` with MQ dungeons configured according to the Knowledge.
///
/// # Example
///
/// ```ignore
/// use oottracker::flag_mapping::mq_settings_from_knowledge;
/// use oottracker::Knowledge;
///
/// let knowledge = Knowledge::default();
/// let settings = mq_settings_from_knowledge(&knowledge.mq);
/// ```
#[must_use]
pub fn mq_settings_from_knowledge(mq_settings: &HashMap<Dungeon, Mq>) -> RandomizerSettings {
    let mut settings = RandomizerSettings::new();

    for (dungeon, mq) in mq_settings {
        let mq_dungeon = dungeon_to_mq_dungeon(dungeon);
        match mq {
            Mq::Mq => settings.set_dungeon_mq(mq_dungeon),
            Mq::Vanilla => settings.set_dungeon_vanilla(mq_dungeon),
        }
    }

    settings
}

// ============================================================================
// Location Check Status
// ============================================================================

use crate::ModelState;

/// Status of a location check.
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum CheckStatus {
    /// Location has been checked (item collected).
    Checked,
    /// Location has not been checked yet.
    Unchecked,
    /// User has decided to skip this location (won't complete it).
    Skipped,
    /// Check status cannot be determined (unmapped or unknown).
    Unknown,
}

/// Result of checking a location.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct LocationCheckResult {
    /// The location ID.
    pub location_id: String,
    /// Whether the location has been checked.
    pub status: CheckStatus,
    /// Whether this location has a valid flag mapping.
    pub is_mapped: bool,
    /// Logic expression required to access this location (if available).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logic: Option<String>,
}

/// Returns the logic expression for a location from the world database.
///
/// # Arguments
///
/// * `location_id` - The location ID to look up
///
/// # Returns
///
/// The logic expression as a String if found, None otherwise.
pub fn get_location_logic(location_id: &str) -> Option<String> {
    let db = world_database();
    db.get_location(location_id)
        .and_then(|(location, _)| location.logic.clone())
}

/// Checks if a specific location has been checked based on the current game state.
///
/// # Arguments
///
/// * `mapping` - The flag mapping for the location
/// * `model` - The current model state containing game memory
///
/// # Returns
///
/// `CheckStatus::Skipped` if the user has marked this location as skipped,
/// `CheckStatus::Checked` if the location flag is set,
/// `CheckStatus::Unchecked` if the flag is not set,
/// `CheckStatus::Unknown` if the location is unmapped or cannot be determined.
#[must_use]
pub fn check_location_status(mapping: &FlagMapping, model: &ModelState) -> CheckStatus {
    // Check if user has marked this location as skipped
    if model.skipped_locations.contains(mapping.location_id) {
        return CheckStatus::Skipped;
    }

    // If the mapping is a stub (unmapped), we can't determine the status
    if mapping.is_stub() {
        return CheckStatus::Unknown;
    }

    let flag_type = match mapping.flag_type {
        Some(ft) => ft,
        None => return CheckStatus::Unknown,
    };

    let flag_bit = match mapping.flag_bit {
        Some(fb) => fb,
        None => return CheckStatus::Unknown,
    };

    match flag_type {
        FlagType::Chest => {
            if let Some(scene_id) = mapping.scene_id {
                let scene_flags = model.ram.scene_flags();
                let chests = scene_flags.get_chest_flags(scene_id);
                if chests & flag_bit != 0 {
                    CheckStatus::Checked
                } else {
                    CheckStatus::Unchecked
                }
            } else {
                CheckStatus::Unknown
            }
        }
        FlagType::Switch => {
            if let Some(scene_id) = mapping.scene_id {
                let scene_flags = model.ram.scene_flags();
                let switches = scene_flags.get_switch_flags(scene_id);
                if switches & flag_bit != 0 {
                    CheckStatus::Checked
                } else {
                    CheckStatus::Unchecked
                }
            } else {
                CheckStatus::Unknown
            }
        }
        FlagType::RoomClear => {
            if let Some(scene_id) = mapping.scene_id {
                let scene_flags = model.ram.scene_flags();
                let room_clear = scene_flags.get_room_clear_flags(scene_id);
                if room_clear & flag_bit != 0 {
                    CheckStatus::Checked
                } else {
                    CheckStatus::Unchecked
                }
            } else {
                CheckStatus::Unknown
            }
        }
        FlagType::Collectible => {
            if let Some(scene_id) = mapping.scene_id {
                let scene_flags = model.ram.scene_flags();
                let collectible = scene_flags.get_collectible_flags(scene_id);
                if collectible & flag_bit != 0 {
                    CheckStatus::Checked
                } else {
                    CheckStatus::Unchecked
                }
            } else {
                CheckStatus::Unknown
            }
        }
        FlagType::GoldSkulltula => {
            // Gold Skulltulas use a different storage mechanism
            // Check the gold_skulltulas field in save data
            let gs_flags = model.ram.save.gold_skulltulas.get_raw_flags();
            if gs_flags & flag_bit != 0 {
                CheckStatus::Checked
            } else {
                CheckStatus::Unchecked
            }
        }
        FlagType::EventChkInf => {
            let event_flags = model.ram.save.event_chk_inf.get_raw_flags();
            if event_flags & flag_bit != 0 {
                CheckStatus::Checked
            } else {
                CheckStatus::Unchecked
            }
        }
        FlagType::ItemGetInf => {
            let item_flags = model.ram.save.item_get_inf.get_raw_flags();
            if item_flags & flag_bit != 0 {
                CheckStatus::Checked
            } else {
                CheckStatus::Unchecked
            }
        }
        FlagType::InfTable => {
            let inf_flags = model.ram.save.inf_table.get_raw_flags();
            if inf_flags & flag_bit != 0 {
                CheckStatus::Checked
            } else {
                CheckStatus::Unchecked
            }
        }
        // These flag types need special handling or are not yet implemented
        FlagType::Shop
        | FlagType::Scrub
        | FlagType::GreatFairy
        | FlagType::Boss
        | FlagType::Song
        | FlagType::Fishing
        | FlagType::Cow
        | FlagType::GossipStone => CheckStatus::Unknown,
    }
}

/// Returns all checked locations for the current game state.
///
/// This function returns check status for all OoT locations. For combo randomizer
/// tracking that includes MM locations, use `get_all_checked_locations_combo`.
///
/// # Arguments
///
/// * `model` - The current model state containing game memory
///
/// # Returns
///
/// A vector of `LocationCheckResult` for all mapped OoT locations.
pub fn get_all_checked_locations(model: &ModelState) -> Vec<LocationCheckResult> {
    get_mapped_locations()
        .map(|mapping| LocationCheckResult {
            location_id: mapping.location_id.to_string(),
            status: check_location_status(mapping, model),
            is_mapped: mapping.is_mapped(),
            logic: get_location_logic(mapping.location_id),
        })
        .collect()
}

/// Returns all checked locations for both OoT and MM (combo randomizer).
///
/// This function combines OoT and MM location checks for combo randomizer tracking.
/// MM locations are only included if `model.ram.mm_save` is Some.
///
/// # Arguments
///
/// * `model` - The current model state containing game memory
///
/// # Returns
///
/// A vector of `LocationCheckResult` for all OoT locations and MM locations (if available).
pub fn get_all_checked_locations_combo(model: &ModelState) -> Vec<LocationCheckResult> {
    use crate::mm_flag_mapping::{check_mm_location_status, get_mm_mapped_locations};

    // Get all OoT locations
    let mut results: Vec<LocationCheckResult> = get_mapped_locations()
        .map(|mapping| LocationCheckResult {
            location_id: mapping.location_id.to_string(),
            status: check_location_status(mapping, model),
            is_mapped: mapping.is_mapped(),
            logic: get_location_logic(mapping.location_id),
        })
        .collect();

    // Add MM locations if MM save data is available
    if let Some(ref mm_save) = model.ram.mm_save {
        let mm_results: Vec<LocationCheckResult> = get_mm_mapped_locations()
            .map(|mapping| LocationCheckResult {
                location_id: mapping.location_id.to_string(),
                status: check_mm_location_status(mapping, mm_save),
                is_mapped: mapping.is_mapped(),
                logic: get_location_logic(mapping.location_id),
            })
            .collect();
        results.extend(mm_results);
    }

    results
}

/// Returns all checked locations as a HashMap for efficient lookup.
///
/// # Arguments
///
/// * `model` - The current model state containing game memory
///
/// # Returns
///
/// A HashMap mapping location_id to CheckStatus.
pub fn get_checked_locations_map(model: &ModelState) -> HashMap<String, CheckStatus> {
    get_mapped_locations()
        .map(|mapping| {
            (
                mapping.location_id.to_string(),
                check_location_status(mapping, model),
            )
        })
        .collect()
}

/// Summary of checked locations.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CheckedLocationsSummary {
    /// Total number of mapped locations.
    pub total_mapped: usize,
    /// Number of checked locations.
    pub checked_count: usize,
    /// Number of unchecked locations.
    pub unchecked_count: usize,
    /// Number of skipped locations.
    pub skipped_count: usize,
    /// Number of locations with unknown status.
    pub unknown_count: usize,
    /// List of location check results.
    pub locations: Vec<LocationCheckResult>,
}

/// Returns a summary of checked locations for the current game state.
///
/// This function includes both OoT and MM locations when MM save data is available,
/// making it suitable for combo randomizer tracking.
///
/// # Arguments
///
/// * `model` - The current model state containing game memory
///
/// # Returns
///
/// A `CheckedLocationsSummary` containing counts and individual location statuses.
pub fn get_checked_locations_summary(model: &ModelState) -> CheckedLocationsSummary {
    // Use combo version to include both OoT and MM locations
    let locations = get_all_checked_locations_combo(model);
    let total_mapped = locations.len();
    let checked_count = locations
        .iter()
        .filter(|l| l.status == CheckStatus::Checked)
        .count();
    let unchecked_count = locations
        .iter()
        .filter(|l| l.status == CheckStatus::Unchecked)
        .count();
    let skipped_count = locations
        .iter()
        .filter(|l| l.status == CheckStatus::Skipped)
        .count();
    let unknown_count = locations
        .iter()
        .filter(|l| l.status == CheckStatus::Unknown)
        .count();

    CheckedLocationsSummary {
        total_mapped,
        checked_count,
        unchecked_count,
        skipped_count,
        unknown_count,
        locations,
    }
}

/// Returns all checked locations for the current game state, filtered by MQ settings.
///
/// This function filters out locations that don't match the current MQ/vanilla
/// settings for each dungeon as stored in the model's Knowledge.
///
/// # Arguments
///
/// * `model` - The current model state containing game memory and knowledge
///
/// # Returns
///
/// A vector of `LocationCheckResult` for all active locations based on MQ settings.
pub fn get_all_checked_locations_filtered(model: &ModelState) -> Vec<LocationCheckResult> {
    let settings = mq_settings_from_knowledge(&model.knowledge.mq);
    get_active_mapped_locations(&settings)
        .map(|mapping| LocationCheckResult {
            location_id: mapping.location_id.to_string(),
            status: check_location_status(mapping, model),
            is_mapped: mapping.is_mapped(),
            logic: get_location_logic(mapping.location_id),
        })
        .collect()
}

/// Returns a summary of checked locations filtered by MQ settings from Knowledge.
///
/// This function filters out locations that don't match the current MQ/vanilla
/// settings for each dungeon. For example, if the Deku Tree is set to vanilla
/// in Knowledge.mq, then Master Quest Deku Tree locations will be excluded.
///
/// # Arguments
///
/// * `model` - The current model state containing game memory and knowledge
///
/// # Returns
///
/// A `CheckedLocationsSummary` containing counts and individual location statuses,
/// filtered to only include locations matching the current MQ settings.
///
/// # Example
///
/// ```ignore
/// use oottracker::flag_mapping::get_checked_locations_summary_filtered;
/// use oottracker::ModelState;
///
/// let model = ModelState::default();
/// let summary = get_checked_locations_summary_filtered(&model);
/// // summary.locations will only include vanilla dungeon checks
/// // since Knowledge.default() sets all dungeons to vanilla
/// ```
pub fn get_checked_locations_summary_filtered(model: &ModelState) -> CheckedLocationsSummary {
    let locations = get_all_checked_locations_filtered(model);
    let total_mapped = locations.len();
    let checked_count = locations
        .iter()
        .filter(|l| l.status == CheckStatus::Checked)
        .count();
    let unchecked_count = locations
        .iter()
        .filter(|l| l.status == CheckStatus::Unchecked)
        .count();
    let skipped_count = locations
        .iter()
        .filter(|l| l.status == CheckStatus::Skipped)
        .count();
    let unknown_count = locations
        .iter()
        .filter(|l| l.status == CheckStatus::Unknown)
        .count();

    CheckedLocationsSummary {
        total_mapped,
        checked_count,
        unchecked_count,
        skipped_count,
        unknown_count,
        locations,
    }
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_flag_type_scene_offset() {
        assert_eq!(FlagType::Chest.scene_offset(), Some(0x00));
        assert_eq!(FlagType::Switch.scene_offset(), Some(0x04));
        assert_eq!(FlagType::RoomClear.scene_offset(), Some(0x08));
        assert_eq!(FlagType::Collectible.scene_offset(), Some(0x0C));
        assert_eq!(FlagType::GoldSkulltula.scene_offset(), None);
        assert_eq!(FlagType::EventChkInf.scene_offset(), None);
    }

    #[test]
    fn test_flag_type_is_scene_based() {
        assert!(FlagType::Chest.is_scene_based());
        assert!(FlagType::Switch.is_scene_based());
        assert!(!FlagType::GoldSkulltula.is_scene_based());
        assert!(!FlagType::EventChkInf.is_scene_based());
    }

    #[test]
    fn test_flag_mapping_stub() {
        let stub = FlagMapping::stub("test_location");
        assert!(stub.is_stub());
        assert!(!stub.is_mapped());
        assert_eq!(stub.location_id, "test_location");
        assert!(stub.scene_id.is_none());
        assert!(stub.flag_type.is_none());
        assert!(stub.flag_bit.is_none());
    }

    #[test]
    fn test_flag_mapping_mapped() {
        let mapped = FlagMapping::mapped("test_chest", scene::DEKU_TREE, FlagType::Chest, 0x01);
        assert!(!mapped.is_stub());
        assert!(mapped.is_mapped());
        assert_eq!(mapped.scene_id, Some(scene::DEKU_TREE));
        assert_eq!(mapped.flag_type, Some(FlagType::Chest));
        assert_eq!(mapped.flag_bit, Some(0x01));
    }

    #[test]
    fn test_flag_mapping_global() {
        let global = FlagMapping::global("test_skulltula", FlagType::GoldSkulltula, 0x04);
        assert!(!global.is_stub());
        assert!(global.is_mapped());
        assert!(global.scene_id.is_none());
        assert_eq!(global.flag_type, Some(FlagType::GoldSkulltula));
    }

    #[test]
    fn test_get_mapping_exists() {
        // This should exist from the world data
        let mappings_count = oot_location_count();
        assert!(
            mappings_count > 0,
            "Should have at least some OoT locations"
        );
    }

    #[test]
    fn test_get_mapping_not_found() {
        let result = get_mapping("nonexistent_location_xyz");
        assert!(result.is_none());
    }

    #[test]
    fn test_location_counts() {
        let total = oot_location_count();
        let mapped = oot_mapped_count();
        let stubs = oot_stub_count();

        assert_eq!(total, mapped + stubs);
        // We should have mostly stubs since only some are mapped
        assert!(stubs > 0, "Should have some stub locations");
    }

    #[test]
    fn test_get_mappings_for_scene() {
        let deku_tree_mappings: Vec<_> = get_mappings_for_scene(scene::DEKU_TREE).collect();
        // We added several Deku Tree chest mappings
        assert!(
            !deku_tree_mappings.is_empty(),
            "Should have Deku Tree mappings"
        );
        for mapping in &deku_tree_mappings {
            assert_eq!(mapping.scene_id, Some(scene::DEKU_TREE));
        }
    }

    #[test]
    fn test_get_mappings_by_flag_type() {
        let chest_mappings: Vec<_> = get_mappings_by_flag_type(FlagType::Chest).collect();
        assert!(!chest_mappings.is_empty(), "Should have chest mappings");
        for mapping in &chest_mappings {
            assert_eq!(mapping.flag_type, Some(FlagType::Chest));
        }
    }

    #[test]
    fn test_scene_constants() {
        assert_eq!(scene::DEKU_TREE, 0x00);
        assert_eq!(scene::DODONGOS_CAVERN, 0x01);
        assert_eq!(scene::KOKIRI_FOREST, 0x55);
        assert_eq!(scene::MAX_SCENE_ID, 0x64);
        assert_eq!(scene::SCENE_COUNT, 101);
    }

    #[test]
    fn test_all_location_ids_loaded() {
        let ids: Vec<_> = get_all_oot_location_ids().collect();
        assert!(!ids.is_empty(), "Should have loaded OoT location IDs");
        // All IDs should start with "oot_" or "mq_oot_" (for Master Quest variants)
        for id in &ids {
            assert!(
                id.starts_with("oot_") || id.starts_with("mq_oot_"),
                "OoT location ID should start with 'oot_' or 'mq_oot_': {}",
                id
            );
        }
    }

    // === Master Quest Integration Tests ===

    #[test]
    fn test_mq_active_mappings_default() {
        use ootmm::settings::RandomizerSettings;

        let settings = RandomizerSettings::default();

        // With default settings (no MQ), vanilla locations should be active
        let active: Vec<_> = get_active_mappings(&settings).collect();
        assert!(!active.is_empty());

        // All active locations should be vanilla (not MQ)
        for mapping in &active {
            assert!(
                !mapping.location_id.starts_with("mq_oot_"),
                "Default settings should not include MQ locations: {}",
                mapping.location_id
            );
        }
    }

    #[test]
    fn test_mq_active_mapping_filters() {
        use ootmm::settings::RandomizerSettings;

        let mut settings = RandomizerSettings::default();

        // Vanilla Deku Tree location should be active by default
        assert!(get_active_mapping("oot_deku_tree_compass_chest", &settings).is_some());

        // MQ Deku Tree location should NOT be active by default
        assert!(get_active_mapping("mq_oot_mq_deku_tree_compass_chest", &settings).is_none());

        // Set Deku Tree to MQ
        settings.set_dungeon_mq(MqDungeon::DekuTree);

        // Now vanilla should be inactive and MQ should be active
        assert!(get_active_mapping("oot_deku_tree_compass_chest", &settings).is_none());
        assert!(get_active_mapping("mq_oot_mq_deku_tree_compass_chest", &settings).is_some());
    }

    #[test]
    fn test_mq_dungeon_mappings() {
        use ootmm::settings::RandomizerSettings;

        let mut settings = RandomizerSettings::default();

        // Get vanilla Deku Tree mappings
        let vanilla_mappings: Vec<_> =
            get_dungeon_mappings(MqDungeon::DekuTree, &settings).collect();
        assert!(!vanilla_mappings.is_empty());
        for mapping in &vanilla_mappings {
            assert!(
                mapping.location_id.starts_with("oot_deku_tree_"),
                "Expected vanilla Deku Tree location: {}",
                mapping.location_id
            );
        }

        // Set to MQ and get MQ mappings
        settings.set_dungeon_mq(MqDungeon::DekuTree);
        let mq_mappings: Vec<_> = get_dungeon_mappings(MqDungeon::DekuTree, &settings).collect();
        assert!(!mq_mappings.is_empty());
        for mapping in &mq_mappings {
            assert!(
                mapping.location_id.starts_with("mq_oot_mq_deku_tree_"),
                "Expected MQ Deku Tree location: {}",
                mapping.location_id
            );
        }
    }

    #[test]
    fn test_active_counts_change_with_mq() {
        use ootmm::settings::RandomizerSettings;

        let mut settings = RandomizerSettings::default();

        let vanilla_count = active_location_count(&settings);
        assert!(vanilla_count > 0);

        // Set all dungeons to MQ
        settings.set_all_dungeons_mq();
        let mq_count = active_location_count(&settings);
        assert!(mq_count > 0);

        // Counts might differ since MQ dungeons have different check counts
        // The important thing is that we get different locations
        let vanilla_settings = RandomizerSettings::default();
        let vanilla_locs: std::collections::HashSet<_> = get_active_mappings(&vanilla_settings)
            .map(|m| m.location_id)
            .collect();
        let mq_locs: std::collections::HashSet<_> = get_active_mappings(&settings)
            .map(|m| m.location_id)
            .collect();

        // The sets should be different (vanilla vs MQ dungeon locations)
        assert_ne!(vanilla_locs, mq_locs);
    }

    // ========================================================================
    // get_all_checked_locations_combo integration tests
    // ========================================================================

    #[test]
    fn test_get_all_checked_locations_combo_oot_only() {
        // Test that without MM save data, only OoT locations are returned
        let model = ModelState::default();

        // Ensure mm_save is None (it should be by default)
        assert!(
            model.ram.mm_save.is_none(),
            "Default model should have no MM save data"
        );

        let locations = get_all_checked_locations_combo(&model);

        // Should have OoT locations
        assert!(!locations.is_empty(), "Should have some locations");

        // All locations should be OoT (start with "oot_" or "mq_oot_" for Master Quest)
        // No MM locations should be present
        for loc in &locations {
            let is_oot =
                loc.location_id.starts_with("oot_") || loc.location_id.starts_with("mq_oot_");
            assert!(
                is_oot,
                "Without MM save, all locations should be OoT: {}",
                loc.location_id
            );
            assert!(
                !loc.location_id.starts_with("mm_"),
                "Without MM save, no MM locations should be present: {}",
                loc.location_id
            );
        }
    }

    #[test]
    fn test_get_all_checked_locations_combo_includes_mm_when_present() {
        use crate::mm_save::MmSave;

        // Create model with MM save data
        let mut model = ModelState::default();
        model.ram.mm_save = Some(MmSave::default());

        let locations = get_all_checked_locations_combo(&model);

        // Should have locations
        assert!(!locations.is_empty(), "Should have some locations");

        // Should have both OoT and MM locations
        let has_oot = locations.iter().any(|l| l.location_id.starts_with("oot_"));
        let has_mm = locations.iter().any(|l| l.location_id.starts_with("mm_"));

        assert!(has_oot, "Should have OoT locations");
        assert!(has_mm, "Should have MM locations when mm_save is present");
    }

    #[test]
    fn test_get_all_checked_locations_combo_more_locations_with_mm() {
        use crate::mm_save::MmSave;

        // Get OoT-only count
        let model_oot_only = ModelState::default();
        let oot_only_count = get_all_checked_locations_combo(&model_oot_only).len();

        // Get OoT+MM count
        let mut model_with_mm = ModelState::default();
        model_with_mm.ram.mm_save = Some(MmSave::default());
        let combo_count = get_all_checked_locations_combo(&model_with_mm).len();

        // Combo should have more locations than OoT-only
        assert!(
            combo_count > oot_only_count,
            "Combo locations ({}) should be more than OoT-only ({})",
            combo_count,
            oot_only_count
        );
    }

    #[test]
    fn test_get_checked_locations_summary_includes_mm() {
        use crate::mm_save::MmSave;

        // Create model with MM save data
        let mut model = ModelState::default();
        model.ram.mm_save = Some(MmSave::default());

        let summary = get_checked_locations_summary(&model);

        // Summary should include both OoT and MM locations
        let has_oot = summary
            .locations
            .iter()
            .any(|l| l.location_id.starts_with("oot_"));
        let has_mm = summary
            .locations
            .iter()
            .any(|l| l.location_id.starts_with("mm_"));

        assert!(has_oot, "Summary should include OoT locations");
        assert!(
            has_mm,
            "Summary should include MM locations when mm_save is present"
        );

        // Total should match location count
        assert_eq!(
            summary.total_mapped,
            summary.locations.len(),
            "total_mapped should match locations count"
        );

        // Counts should add up
        assert_eq!(
            summary.checked_count + summary.unchecked_count + summary.unknown_count,
            summary.total_mapped,
            "Status counts should sum to total_mapped"
        );
    }

    // === dungeon_to_mq_dungeon Tests ===

    #[test]
    fn test_dungeon_to_mq_dungeon_main_dungeons() {
        // Test all main dungeon conversions
        assert_eq!(
            dungeon_to_mq_dungeon(&Dungeon::Main(MainDungeon::DekuTree)),
            MqDungeon::DekuTree
        );
        assert_eq!(
            dungeon_to_mq_dungeon(&Dungeon::Main(MainDungeon::DodongosCavern)),
            MqDungeon::DodongosCavern
        );
        assert_eq!(
            dungeon_to_mq_dungeon(&Dungeon::Main(MainDungeon::JabuJabu)),
            MqDungeon::JabuJabu
        );
        assert_eq!(
            dungeon_to_mq_dungeon(&Dungeon::Main(MainDungeon::ForestTemple)),
            MqDungeon::ForestTemple
        );
        assert_eq!(
            dungeon_to_mq_dungeon(&Dungeon::Main(MainDungeon::FireTemple)),
            MqDungeon::FireTemple
        );
        assert_eq!(
            dungeon_to_mq_dungeon(&Dungeon::Main(MainDungeon::WaterTemple)),
            MqDungeon::WaterTemple
        );
        assert_eq!(
            dungeon_to_mq_dungeon(&Dungeon::Main(MainDungeon::ShadowTemple)),
            MqDungeon::ShadowTemple
        );
        assert_eq!(
            dungeon_to_mq_dungeon(&Dungeon::Main(MainDungeon::SpiritTemple)),
            MqDungeon::SpiritTemple
        );
    }

    #[test]
    fn test_dungeon_to_mq_dungeon_mini_dungeons() {
        // Test mini dungeon conversions
        assert_eq!(
            dungeon_to_mq_dungeon(&Dungeon::IceCavern),
            MqDungeon::IceCavern
        );
        assert_eq!(
            dungeon_to_mq_dungeon(&Dungeon::BottomOfTheWell),
            MqDungeon::BottomOfTheWell
        );
        assert_eq!(
            dungeon_to_mq_dungeon(&Dungeon::GerudoTrainingGround),
            MqDungeon::GerudoTrainingGround
        );
        assert_eq!(
            dungeon_to_mq_dungeon(&Dungeon::GanonsCastle),
            MqDungeon::GanonsCastle
        );
    }

    #[test]
    fn test_dungeon_to_mq_dungeon_exhaustive() {
        // Ensure all Dungeon variants can be converted
        use enum_iterator::all;

        for main_dungeon in all::<MainDungeon>() {
            let dungeon = Dungeon::Main(main_dungeon);
            // Should not panic
            let _ = dungeon_to_mq_dungeon(&dungeon);
        }

        // Test non-main dungeons explicitly
        let _ = dungeon_to_mq_dungeon(&Dungeon::IceCavern);
        let _ = dungeon_to_mq_dungeon(&Dungeon::BottomOfTheWell);
        let _ = dungeon_to_mq_dungeon(&Dungeon::GerudoTrainingGround);
        let _ = dungeon_to_mq_dungeon(&Dungeon::GanonsCastle);
    }

    // === mq_settings_from_knowledge Tests ===

    #[test]
    fn test_mq_settings_from_knowledge_empty() {
        // Empty HashMap should result in all-vanilla settings
        let mq_settings: HashMap<Dungeon, Mq> = HashMap::new();
        let settings = mq_settings_from_knowledge(&mq_settings);

        // All dungeons should be vanilla
        assert!(!settings.is_dungeon_mq(MqDungeon::DekuTree));
        assert!(!settings.is_dungeon_mq(MqDungeon::ForestTemple));
        assert!(!settings.is_dungeon_mq(MqDungeon::GanonsCastle));
    }

    #[test]
    fn test_mq_settings_from_knowledge_single_mq() {
        let mut mq_settings: HashMap<Dungeon, Mq> = HashMap::new();
        mq_settings.insert(Dungeon::Main(MainDungeon::ForestTemple), Mq::Mq);

        let settings = mq_settings_from_knowledge(&mq_settings);

        // Only Forest Temple should be MQ
        assert!(settings.is_dungeon_mq(MqDungeon::ForestTemple));
        assert!(!settings.is_dungeon_mq(MqDungeon::DekuTree));
        assert!(!settings.is_dungeon_mq(MqDungeon::FireTemple));
    }

    #[test]
    fn test_mq_settings_from_knowledge_multiple_mq() {
        let mut mq_settings: HashMap<Dungeon, Mq> = HashMap::new();
        mq_settings.insert(Dungeon::Main(MainDungeon::DekuTree), Mq::Mq);
        mq_settings.insert(Dungeon::Main(MainDungeon::ForestTemple), Mq::Mq);
        mq_settings.insert(Dungeon::GanonsCastle, Mq::Mq);
        mq_settings.insert(Dungeon::Main(MainDungeon::WaterTemple), Mq::Vanilla);

        let settings = mq_settings_from_knowledge(&mq_settings);

        // Check MQ dungeons
        assert!(settings.is_dungeon_mq(MqDungeon::DekuTree));
        assert!(settings.is_dungeon_mq(MqDungeon::ForestTemple));
        assert!(settings.is_dungeon_mq(MqDungeon::GanonsCastle));

        // Check vanilla dungeon
        assert!(!settings.is_dungeon_mq(MqDungeon::WaterTemple));

        // Check unspecified dungeons (should be vanilla)
        assert!(!settings.is_dungeon_mq(MqDungeon::FireTemple));
    }

    #[test]
    fn test_mq_settings_from_knowledge_all_vanilla() {
        let mut mq_settings: HashMap<Dungeon, Mq> = HashMap::new();
        // Explicitly set everything to vanilla
        mq_settings.insert(Dungeon::Main(MainDungeon::DekuTree), Mq::Vanilla);
        mq_settings.insert(Dungeon::Main(MainDungeon::ForestTemple), Mq::Vanilla);
        mq_settings.insert(Dungeon::GanonsCastle, Mq::Vanilla);

        let settings = mq_settings_from_knowledge(&mq_settings);

        // All should be vanilla
        assert!(!settings.is_dungeon_mq(MqDungeon::DekuTree));
        assert!(!settings.is_dungeon_mq(MqDungeon::ForestTemple));
        assert!(!settings.is_dungeon_mq(MqDungeon::GanonsCastle));
    }

    #[test]
    fn test_mq_settings_from_knowledge_all_mq() {
        use enum_iterator::all;

        let mut mq_settings: HashMap<Dungeon, Mq> = HashMap::new();

        // Set all main dungeons to MQ
        for main_dungeon in all::<MainDungeon>() {
            mq_settings.insert(Dungeon::Main(main_dungeon), Mq::Mq);
        }
        // Set mini dungeons to MQ
        mq_settings.insert(Dungeon::IceCavern, Mq::Mq);
        mq_settings.insert(Dungeon::BottomOfTheWell, Mq::Mq);
        mq_settings.insert(Dungeon::GerudoTrainingGround, Mq::Mq);
        mq_settings.insert(Dungeon::GanonsCastle, Mq::Mq);

        let settings = mq_settings_from_knowledge(&mq_settings);

        // All should be MQ
        assert!(settings.is_dungeon_mq(MqDungeon::DekuTree));
        assert!(settings.is_dungeon_mq(MqDungeon::DodongosCavern));
        assert!(settings.is_dungeon_mq(MqDungeon::JabuJabu));
        assert!(settings.is_dungeon_mq(MqDungeon::ForestTemple));
        assert!(settings.is_dungeon_mq(MqDungeon::FireTemple));
        assert!(settings.is_dungeon_mq(MqDungeon::WaterTemple));
        assert!(settings.is_dungeon_mq(MqDungeon::ShadowTemple));
        assert!(settings.is_dungeon_mq(MqDungeon::SpiritTemple));
        assert!(settings.is_dungeon_mq(MqDungeon::IceCavern));
        assert!(settings.is_dungeon_mq(MqDungeon::BottomOfTheWell));
        assert!(settings.is_dungeon_mq(MqDungeon::GerudoTrainingGround));
        assert!(settings.is_dungeon_mq(MqDungeon::GanonsCastle));
    }

    #[test]
    fn test_mq_settings_from_knowledge_integration_with_filtering() {
        // Test that mq_settings_from_knowledge integrates properly with location filtering
        let mut mq_settings: HashMap<Dungeon, Mq> = HashMap::new();
        mq_settings.insert(Dungeon::Main(MainDungeon::DekuTree), Mq::Mq);

        let settings = mq_settings_from_knowledge(&mq_settings);

        // Vanilla Deku Tree should be inactive
        assert!(get_active_mapping("oot_deku_tree_compass_chest", &settings).is_none());

        // MQ Deku Tree should be active
        assert!(get_active_mapping("mq_oot_mq_deku_tree_compass_chest", &settings).is_some());

        // Other vanilla dungeons should still be active
        assert!(get_active_mapping("oot_forest_temple_compass", &settings).is_some());
    }
}
