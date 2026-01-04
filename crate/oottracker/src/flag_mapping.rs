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

    // Deku Tree chests
    add_mapping(
        &mut map,
        "oot_deku_tree_compass_side_chest",
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

    // Dodongo's Cavern chests
    add_mapping(
        &mut map,
        "oot_dodongo_cavern_end_of_bridge_chest",
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
        "oot_dodongo_cavern_bomb_flower_platform_chest",
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

    // Jabu Jabu's Belly chests
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

    // Forest Temple chests
    add_mapping(
        &mut map,
        "oot_forest_temple_blue_poe_chest",
        scene::FOREST_TEMPLE,
        FlagType::Chest,
        0x0000_8000,
    );
    add_mapping(
        &mut map,
        "oot_forest_temple_boss_key_chest",
        scene::FOREST_TEMPLE,
        FlagType::Chest,
        0x0000_4000,
    );
    add_mapping(
        &mut map,
        "oot_forest_temple_red_poe_chest",
        scene::FOREST_TEMPLE,
        FlagType::Chest,
        0x0000_2000,
    );
    add_mapping(
        &mut map,
        "oot_forest_temple_bow_chest",
        scene::FOREST_TEMPLE,
        FlagType::Chest,
        0x0000_1000,
    );
    add_mapping(
        &mut map,
        "oot_forest_temple_basement_chest",
        scene::FOREST_TEMPLE,
        FlagType::Chest,
        0x0000_0800,
    );
    add_mapping(
        &mut map,
        "oot_forest_temple_well_chest",
        scene::FOREST_TEMPLE,
        FlagType::Chest,
        0x0000_0200,
    );
    add_mapping(
        &mut map,
        "oot_forest_temple_map_chest",
        scene::FOREST_TEMPLE,
        FlagType::Chest,
        0x0000_0002,
    );
    add_mapping(
        &mut map,
        "oot_forest_temple_first_stalfos_chest",
        scene::FOREST_TEMPLE,
        FlagType::Chest,
        0x0000_0001,
    );

    // Kokiri Forest
    add_mapping(
        &mut map,
        "oot_kokiri_forest_kokiri_sword_chest",
        scene::KOKIRI_FOREST,
        FlagType::Chest,
        0x0000_0001,
    );

    // Mido's House chests
    add_mapping(
        &mut map,
        "oot_kf_midos_chest_bottom_right",
        scene::MIDOS_HOUSE,
        FlagType::Chest,
        0x0000_0008,
    );
    add_mapping(
        &mut map,
        "oot_kf_midos_chest_bottom_left",
        scene::MIDOS_HOUSE,
        FlagType::Chest,
        0x0000_0004,
    );
    add_mapping(
        &mut map,
        "oot_kf_midos_chest_top_right",
        scene::MIDOS_HOUSE,
        FlagType::Chest,
        0x0000_0002,
    );
    add_mapping(
        &mut map,
        "oot_kf_midos_chest_top_left",
        scene::MIDOS_HOUSE,
        FlagType::Chest,
        0x0000_0001,
    );

    // Death Mountain chests
    add_mapping(
        &mut map,
        "oot_death_mountain_trail_chest",
        scene::DEATH_MOUNTAIN_TRAIL,
        FlagType::Chest,
        0x0000_0002,
    );

    // Lake Hylia
    add_mapping(
        &mut map,
        "oot_lake_hylia_sun_chest",
        scene::LAKE_HYLIA,
        FlagType::Chest,
        0x0000_0001,
    );

    // Zora's Domain
    add_mapping(
        &mut map,
        "oot_zoras_domain_chest",
        scene::ZORAS_DOMAIN,
        FlagType::Chest,
        0x0000_0001,
    );

    // Gerudo Valley
    add_mapping(
        &mut map,
        "oot_gerudo_valley_chest",
        scene::GERUDO_VALLEY,
        FlagType::Chest,
        0x0000_0001,
    );

    // Goron City chests
    add_mapping(
        &mut map,
        "oot_goron_city_maze_center_chest",
        scene::GORON_CITY,
        FlagType::Chest,
        0x0000_0004,
    );
    add_mapping(
        &mut map,
        "oot_goron_city_maze_right_chest",
        scene::GORON_CITY,
        FlagType::Chest,
        0x0000_0002,
    );
    add_mapping(
        &mut map,
        "oot_goron_city_maze_left_chest",
        scene::GORON_CITY,
        FlagType::Chest,
        0x0000_0001,
    );

    // Haunted Wasteland
    add_mapping(
        &mut map,
        "oot_haunted_wasteland_chest",
        scene::HAUNTED_WASTELAND,
        FlagType::Chest,
        0x0000_0001,
    );

    // Desert Colossus (Spirit Temple exterior chests)
    add_mapping(
        &mut map,
        "oot_spirit_temple_silver_gauntlets_chest",
        scene::DESERT_COLOSSUS,
        FlagType::Chest,
        0x0000_0800,
    );
    add_mapping(
        &mut map,
        "oot_spirit_temple_mirror_shield_chest",
        scene::DESERT_COLOSSUS,
        FlagType::Chest,
        0x0000_0200,
    );

    // Gerudo Fortress
    add_mapping(
        &mut map,
        "oot_gerudo_fortress_chest",
        scene::GERUDO_FORTRESS,
        FlagType::Chest,
        0x0000_0001,
    );

    // Grottos (shared scene)
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

    // Graveyard graves
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
        "oot_graveyard_royal_familys_tomb_chest",
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

    // Gold Skulltulas - these use a different flag system
    // Forest Temple example
    add_global_mapping(
        &mut map,
        "oot_forest_temple_gs_level_island_courtyard",
        FlagType::GoldSkulltula,
        0x04,
    );

    map
});

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

/// Helper function to add a global mapping.
fn add_global_mapping(
    map: &mut HashMap<&'static str, FlagMapping>,
    location_id: &'static str,
    flag_type: FlagType,
    flag_bit: u32,
) {
    map.insert(
        location_id,
        FlagMapping::global(location_id, flag_type, flag_bit),
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
}
