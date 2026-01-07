//! MM Flag Data Structures and Location Mappings
//!
//! This module provides the foundation for detecting location checks by mapping
//! location IDs to memory flag addresses for Majora's Mask. It defines flag types,
//! scene IDs, and stub mappings for all MM locations imported from OoTMM world data.
//!
//! # Flag Structure (based on OoTMM research)
//!
//! MM save data contains several types of flags stored in different memory regions:
//!
//! ## Scene Flags (per-scene, stored in permanent save at offset varies)
//!
//! Each scene has a 0x1C byte entry containing:
//! - `chest` (u32 at offset 0x00): Opened chest flags
//! - `switch0` (u32 at offset 0x04): Switch/trigger flags bank 0
//! - `switch1` (u32 at offset 0x08): Switch/trigger flags bank 1
//! - `cleared_room` (u32 at offset 0x0C): Room clear flags
//! - `collectible` (u32 at offset 0x10): Collectible item flags
//! - `cleared_floors` (u32 at offset 0x14): Floor clear flags
//! - `rooms` (u32 at offset 0x18): Room visited flags
//!
//! ## Global Flags
//!
//! - Gold Skulltulas: Swamp/Ocean spider house tokens
//! - Event flags: Global event tracking
//! - Week event flags: Cycle-persistent event flags
//! - Item get flags: Item obtained tracking
//!
//! # Usage
//!
//! ```ignore
//! use oottracker::mm_flag_mapping::{MmFlagMapping, get_mm_mapping, get_all_mm_mappings};
//!
//! // Get mapping for a specific location
//! if let Some(mapping) = get_mm_mapping("mm_woodfall_temple_compass_chest") {
//!     println!("Location {} is in scene {:?}", mapping.location_id, mapping.scene_id);
//! }
//!
//! // Get all MM location mappings
//! for mapping in get_all_mm_mappings() {
//!     println!("{}: {:?}", mapping.location_id, mapping.flag_type);
//! }
//! ```

use std::collections::HashMap;

use once_cell::sync::Lazy;

use ootmm::events::mm_flags::owl_bits;
use ootmm::region::Game;

// ============================================================================
// Flag Type Definition
// ============================================================================

/// Types of flags used to track location checks in MM save data.
///
/// Each location in the game is tracked by one of these flag types,
/// stored in specific memory regions within the save context.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MmFlagType {
    /// Chest opened flags (scene flags offset 0x00).
    /// Each bit represents a chest in the scene.
    Chest,

    /// Switch/trigger flags bank 0 (scene flags offset 0x04).
    /// Includes crystal switches, floor switches, etc.
    Switch0,

    /// Switch/trigger flags bank 1 (scene flags offset 0x08).
    /// Additional switch flags for complex scenes.
    Switch1,

    /// Room clear flags (scene flags offset 0x0C).
    /// Set when all enemies in a room are defeated.
    ClearedRoom,

    /// Collectible item flags (scene flags offset 0x10).
    /// Freestanding items, rupees, hearts, etc.
    Collectible,

    /// Gold Skulltula flags for spider houses.
    /// Swamp Spider House and Oceanside Spider House tokens.
    GoldSkulltula,

    /// Event flags for global game events.
    /// Tracks major story events and NPC interactions.
    EventInf,

    /// Week event flags that persist across cycles.
    /// Tracks events that should survive Song of Time.
    WeekEventReg,

    /// Item get flags.
    /// Tracks specific item acquisitions.
    ItemGetInf,

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
    /// Boss remains collected after defeating bosses.
    Boss,

    /// Song learned flags.
    /// Stored in quest items bitfield.
    Song,

    /// Cow/milk flags.
    /// Playing Epona's Song to cows.
    Cow,

    /// Stray fairy flags.
    /// Tracks stray fairies collected in dungeons.
    StrayFairy,

    /// Owl statue flags.
    /// Tracks activated owl statues.
    OwlStatue,

    /// Moon's Tear related flags.
    MoonsTear,

    /// Gossip stone hint flags.
    /// Hints from gossip stones (if shuffled).
    GossipStone,
}

impl MmFlagType {
    /// Returns the byte offset within scene flags for scene-based flag types.
    ///
    /// Returns `None` for global flag types that aren't stored per-scene.
    #[must_use]
    pub const fn scene_offset(&self) -> Option<usize> {
        match self {
            MmFlagType::Chest => Some(0x00),
            MmFlagType::Switch0 => Some(0x04),
            MmFlagType::Switch1 => Some(0x08),
            MmFlagType::ClearedRoom => Some(0x0C),
            MmFlagType::Collectible => Some(0x10),
            // These are global, not per-scene
            MmFlagType::GoldSkulltula
            | MmFlagType::EventInf
            | MmFlagType::WeekEventReg
            | MmFlagType::ItemGetInf
            | MmFlagType::Shop
            | MmFlagType::Scrub
            | MmFlagType::GreatFairy
            | MmFlagType::Boss
            | MmFlagType::Song
            | MmFlagType::Cow
            | MmFlagType::StrayFairy
            | MmFlagType::OwlStatue
            | MmFlagType::MoonsTear
            | MmFlagType::GossipStone => None,
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

/// MM Scene IDs.
///
/// Scene IDs correspond to the index in the scene flag array.
/// MM has 120 permanent scene flag slots.
pub mod mm_scene {
    // Main Dungeons
    pub const WOODFALL_TEMPLE: u8 = 0x1F;
    pub const SNOWHEAD_TEMPLE: u8 = 0x22;
    pub const GREAT_BAY_TEMPLE: u8 = 0x1E;
    pub const STONE_TOWER_TEMPLE: u8 = 0x18;
    pub const STONE_TOWER_TEMPLE_INVERTED: u8 = 0x19;

    // Dungeon Boss Rooms
    pub const WOODFALL_TEMPLE_BOSS: u8 = 0x1A;
    pub const SNOWHEAD_TEMPLE_BOSS: u8 = 0x24;
    pub const GREAT_BAY_TEMPLE_BOSS: u8 = 0x4F;
    pub const STONE_TOWER_TEMPLE_BOSS: u8 = 0x36;

    // Mini Dungeons
    pub const BENEATH_THE_WELL: u8 = 0x1B;
    pub const ANCIENT_CASTLE_OF_IKANA: u8 = 0x11;
    pub const IKANA_CANYON_SECRET_SHRINE: u8 = 0x13;
    pub const PIRATES_FORTRESS: u8 = 0x29;
    pub const PIRATES_FORTRESS_INTERIOR: u8 = 0x2A;
    pub const BENEATH_THE_GRAVEYARD: u8 = 0x07;

    // Spider Houses
    pub const SWAMP_SPIDER_HOUSE: u8 = 0x27;
    pub const OCEANSIDE_SPIDER_HOUSE: u8 = 0x28;

    // Clock Town Areas
    pub const CLOCK_TOWN_SOUTH: u8 = 0x6C;
    pub const CLOCK_TOWN_NORTH: u8 = 0x6D;
    pub const CLOCK_TOWN_EAST: u8 = 0x6E;
    pub const CLOCK_TOWN_WEST: u8 = 0x6F;
    pub const LAUNDRY_POOL: u8 = 0x70;
    pub const CLOCK_TOWER: u8 = 0x08;

    // Clock Town Buildings
    pub const STOCK_POT_INN: u8 = 0x4D;
    pub const STOCK_POT_INN_RESERVATION: u8 = 0x4B;
    pub const MILK_BAR: u8 = 0x51;
    pub const MAYORS_OFFICE: u8 = 0x4E;
    pub const POST_OFFICE: u8 = 0x30;
    pub const LOTTERY_SHOP: u8 = 0x42;
    pub const TRADING_POST: u8 = 0x4A;
    pub const BOMB_SHOP: u8 = 0x32;
    pub const CURIOSITY_SHOP: u8 = 0x33;
    pub const HONEY_AND_DARLING: u8 = 0x44;
    pub const TREASURE_CHEST_SHOP: u8 = 0x4C;
    pub const ASTRAL_OBSERVATORY: u8 = 0x52;
    pub const CLOCK_TOWN_GREAT_FAIRY: u8 = 0x26;

    // Termina Field and Roads
    pub const TERMINA_FIELD: u8 = 0x54;
    pub const ROAD_TO_SOUTHERN_SWAMP: u8 = 0x0D;
    pub const MILK_ROAD: u8 = 0x5E;
    pub const PATH_TO_MOUNTAIN_VILLAGE: u8 = 0x64;
    pub const ROAD_TO_IKANA: u8 = 0x47;
    pub const GREAT_BAY_COAST: u8 = 0x37;

    // Southern Swamp
    pub const SOUTHERN_SWAMP: u8 = 0x55;
    pub const SOUTHERN_SWAMP_CLEAR: u8 = 0x56;
    pub const SWAMP_TOURIST_CENTER: u8 = 0x57;
    pub const DEKU_PALACE: u8 = 0x14;
    pub const DEKU_PALACE_GARDEN: u8 = 0x4F;
    pub const WOODFALL: u8 = 0x20;
    pub const WOODFALL_GREAT_FAIRY: u8 = 0x26;

    // Snowhead Region
    pub const MOUNTAIN_VILLAGE: u8 = 0x5A;
    pub const MOUNTAIN_VILLAGE_SPRING: u8 = 0x5B;
    pub const GORON_VILLAGE: u8 = 0x5C;
    pub const GORON_VILLAGE_SPRING: u8 = 0x5D;
    pub const PATH_TO_SNOWHEAD: u8 = 0x5F;
    pub const SNOWHEAD: u8 = 0x23;
    pub const GORON_SHRINE: u8 = 0x58;
    pub const SNOWHEAD_GREAT_FAIRY: u8 = 0x26;

    // Great Bay Region
    pub const ZORA_CAPE: u8 = 0x38;
    pub const ZORA_HALL: u8 = 0x60;
    pub const PINNACLE_ROCK: u8 = 0x3F;
    pub const PIRATES_FORTRESS_EXTERIOR: u8 = 0x3B;
    pub const GREAT_BAY_GREAT_FAIRY: u8 = 0x26;

    // Ikana Region
    pub const IKANA_CANYON: u8 = 0x13;
    pub const IKANA_GRAVEYARD: u8 = 0x09;
    pub const STONE_TOWER: u8 = 0x17;
    pub const STONE_TOWER_INVERTED: u8 = 0x17;
    pub const IKANA_CASTLE: u8 = 0x11;
    pub const IKANA_GREAT_FAIRY: u8 = 0x26;

    // Ranch
    pub const ROMANI_RANCH: u8 = 0x64;
    pub const CUCCO_SHACK: u8 = 0x5F;
    pub const DOGGY_RACETRACK: u8 = 0x61;

    // The Moon
    pub const MOON: u8 = 0x08;
    pub const MOON_DEKU_TRIAL: u8 = 0x08;
    pub const MOON_GORON_TRIAL: u8 = 0x08;
    pub const MOON_ZORA_TRIAL: u8 = 0x08;
    pub const MOON_LINK_TRIAL: u8 = 0x08;

    /// Maximum scene ID for MM.
    pub const MAX_SCENE_ID: u8 = 0x78;

    /// Number of scenes in MM.
    pub const SCENE_COUNT: usize = 120;
}

// ============================================================================
// Flag Mapping Structure
// ============================================================================

/// Mapping from a location ID to its flag address in memory for MM.
///
/// This struct represents either a complete mapping (with scene_id, flag_type,
/// and flag_bit populated) or a stub mapping (with all optional fields as None).
///
/// Stub mappings are generated for all locations from world data and serve as
/// placeholders until the actual flag addresses are researched and filled in.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MmFlagMapping {
    /// The unique location identifier from OoTMM world data.
    pub location_id: &'static str,

    /// The scene ID where this flag is stored (None for unmapped stubs or global flags).
    pub scene_id: Option<u8>,

    /// The type of flag used for this location (None for unmapped stubs).
    pub flag_type: Option<MmFlagType>,

    /// The bit position within the flag word (None for unmapped stubs).
    /// For 32-bit flag words, this is typically 0-31 representing a single bit,
    /// or a full bitmask for multi-bit values.
    pub flag_bit: Option<u32>,
}

impl MmFlagMapping {
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
        flag_type: MmFlagType,
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
    pub const fn global(location_id: &'static str, flag_type: MmFlagType, flag_bit: u32) -> Self {
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

/// All MM location IDs extracted from embedded world data.
///
/// This list is generated at compile time from the OoTMM YAML files.
static MM_LOCATION_IDS: Lazy<Vec<&'static str>> = Lazy::new(|| {
    let db = ootmm::embedded_data::create_world_database()
        .expect("Failed to load world database for location extraction");

    db.locations_for_game(Game::Mm)
        .map(|(loc, _region_id)| {
            // Leak the string to get a 'static lifetime
            // This is intentional - these strings live for the program's lifetime
            Box::leak(loc.id.clone().into_boxed_str()) as &'static str
        })
        .collect()
});

/// HashMap of location ID to MmFlagMapping for fast lookups.
static MM_MAPPINGS: Lazy<HashMap<&'static str, MmFlagMapping>> = Lazy::new(|| {
    let mut map = HashMap::new();

    // First, add all stub mappings from world data
    for &loc_id in MM_LOCATION_IDS.iter() {
        map.insert(loc_id, MmFlagMapping::stub(loc_id));
    }

    // Then, add known mappings that override the stubs
    // These are derived from OoTMM research

    // ========================================================================
    // Owl Statue Mappings
    // ========================================================================
    // Owl statues are global flags stored in the quest status bitfield.
    // Each owl statue has a unique bit position (bits 20-29).
    add_mm_global_mapping(
        &mut map,
        "mm_clock_town_owl_statue",
        MmFlagType::OwlStatue,
        1 << owl_bits::OWL_CLOCK_TOWN,
    );
    add_mm_global_mapping(
        &mut map,
        "mm_milk_road_owl_statue",
        MmFlagType::OwlStatue,
        1 << owl_bits::OWL_MILK_ROAD,
    );
    add_mm_global_mapping(
        &mut map,
        "mm_southern_swamp_owl_statue",
        MmFlagType::OwlStatue,
        1 << owl_bits::OWL_SOUTHERN_SWAMP,
    );
    add_mm_global_mapping(
        &mut map,
        "mm_woodfall_owl_statue",
        MmFlagType::OwlStatue,
        1 << owl_bits::OWL_WOODFALL,
    );
    add_mm_global_mapping(
        &mut map,
        "mm_mountain_village_owl_statue",
        MmFlagType::OwlStatue,
        1 << owl_bits::OWL_MOUNTAIN_VILLAGE,
    );
    add_mm_global_mapping(
        &mut map,
        "mm_snowhead_owl_statue",
        MmFlagType::OwlStatue,
        1 << owl_bits::OWL_SNOWHEAD,
    );
    add_mm_global_mapping(
        &mut map,
        "mm_zora_cape_owl_statue",
        MmFlagType::OwlStatue,
        1 << owl_bits::OWL_ZORA_CAPE,
    );
    add_mm_global_mapping(
        &mut map,
        "mm_great_bay_coast_owl_statue",
        MmFlagType::OwlStatue,
        1 << owl_bits::OWL_GREAT_BAY,
    );
    add_mm_global_mapping(
        &mut map,
        "mm_ikana_canyon_owl_statue",
        MmFlagType::OwlStatue,
        1 << owl_bits::OWL_IKANA_CANYON,
    );
    add_mm_global_mapping(
        &mut map,
        "mm_stone_tower_owl_statue",
        MmFlagType::OwlStatue,
        1 << owl_bits::OWL_STONE_TOWER,
    );

    // ========================================================================
    // Stray Fairy Fountain Locations (15 total)
    // ========================================================================
    //
    // MM has stray fairies tracked via:
    // - Dedicated counters at save offset 0x00D0 (5 bytes: Clock Town, Woodfall,
    //   Snowhead, Great Bay, Stone Tower)
    // - Scene-based collectible flags for individual fairy pickups
    //
    // Great Fairy rewards are tracked by the StrayFairy counter:
    // - flag_bit 0: Clock Town (1 fairy needed)
    // - flag_bit 1: Woodfall (15 fairies needed)
    // - flag_bit 2: Snowhead (15 fairies needed)
    // - flag_bit 3: Great Bay (15 fairies needed)
    // - flag_bit 4: Stone Tower/Ikana (15 fairies needed)

    // Clock Town Great Fairy rewards (counter index 0)
    add_mm_global_mapping(
        &mut map,
        "mm_clock_town_great_fairy",
        MmFlagType::StrayFairy,
        0, // Clock Town counter
    );
    add_mm_global_mapping(
        &mut map,
        "mm_clock_town_great_fairy_alt",
        MmFlagType::StrayFairy,
        0, // Clock Town counter (alternate form reward)
    );

    // Woodfall Great Fairy reward (counter index 1)
    add_mm_global_mapping(
        &mut map,
        "mm_woodfall_great_fairy",
        MmFlagType::StrayFairy,
        1, // Woodfall counter
    );

    // Snowhead Great Fairy reward (counter index 2)
    add_mm_global_mapping(
        &mut map,
        "mm_snowhead_great_fairy",
        MmFlagType::StrayFairy,
        2, // Snowhead counter
    );

    // Great Bay Great Fairy reward (counter index 3)
    add_mm_global_mapping(
        &mut map,
        "mm_great_bay_great_fairy",
        MmFlagType::StrayFairy,
        3, // Great Bay counter
    );

    // Ikana (Stone Tower) Great Fairy reward (counter index 4)
    add_mm_global_mapping(
        &mut map,
        "mm_ikana_great_fairy",
        MmFlagType::StrayFairy,
        4, // Stone Tower counter
    );

    // Clock Town stray fairy (collectible in Laundry Pool)
    // This is the single stray fairy that appears in Clock Town during the day
    add_mm_mapping(
        &mut map,
        "mm_clock_town_stray_fairy",
        mm_scene::LAUNDRY_POOL,
        MmFlagType::Collectible,
        0x01, // Collectible flag bit
    );

    // Beneath the Well Fairy Fountain fairies (8 healing fairies)
    // These use scene collectible flags in Beneath the Well
    add_mm_mapping(
        &mut map,
        "mm_beneath_the_well_fairy_fountain_fairy_1",
        mm_scene::BENEATH_THE_WELL,
        MmFlagType::Collectible,
        0x01,
    );
    add_mm_mapping(
        &mut map,
        "mm_beneath_the_well_fairy_fountain_fairy_2",
        mm_scene::BENEATH_THE_WELL,
        MmFlagType::Collectible,
        0x02,
    );
    add_mm_mapping(
        &mut map,
        "mm_beneath_the_well_fairy_fountain_fairy_3",
        mm_scene::BENEATH_THE_WELL,
        MmFlagType::Collectible,
        0x04,
    );
    add_mm_mapping(
        &mut map,
        "mm_beneath_the_well_fairy_fountain_fairy_4",
        mm_scene::BENEATH_THE_WELL,
        MmFlagType::Collectible,
        0x08,
    );
    add_mm_mapping(
        &mut map,
        "mm_beneath_the_well_fairy_fountain_fairy_5",
        mm_scene::BENEATH_THE_WELL,
        MmFlagType::Collectible,
        0x10,
    );
    add_mm_mapping(
        &mut map,
        "mm_beneath_the_well_fairy_fountain_fairy_6",
        mm_scene::BENEATH_THE_WELL,
        MmFlagType::Collectible,
        0x20,
    );
    add_mm_mapping(
        &mut map,
        "mm_beneath_the_well_fairy_fountain_fairy_7",
        mm_scene::BENEATH_THE_WELL,
        MmFlagType::Collectible,
        0x40,
    );
    add_mm_mapping(
        &mut map,
        "mm_beneath_the_well_fairy_fountain_fairy_8",
        mm_scene::BENEATH_THE_WELL,
        MmFlagType::Collectible,
        0x80,
    );

    // ========================================================================
    // SONG LOCATIONS
    // ========================================================================
    // Songs are stored in quest_items bitfield at specific bit positions.
    // The MmFlagType::Song flag_bit represents the bit position in quest_items.
    //
    // Song bit positions:
    // - Sonata of Awakening: bit 6
    // - Goron Lullaby: bit 7
    // - New Wave Bossa Nova: bit 8
    // - Elegy of Emptiness: bit 9
    // - Oath to Order: bit 10
    // - Song of Time: bit 12
    // - Song of Healing: bit 13
    // - Epona's Song: bit 14
    // - Song of Soaring: bit 15
    // - Song of Storms: bit 16
    // - Lullaby Intro: bit 24

    // Song of Healing - learned from Happy Mask Salesman in Clock Tower
    add_mm_global_mapping(
        &mut map,
        "mm_initial_song_of_healing",
        MmFlagType::Song,
        1 << 13,
    );

    // Sonata of Awakening - learned from Deku Butler's Son in Deku Palace
    add_mm_global_mapping(
        &mut map,
        "mm_deku_palace_sonata_of_awakening",
        MmFlagType::Song,
        1 << 6,
    );

    // Goron Lullaby (Intro) - learned from crying Goron Baby
    add_mm_global_mapping(&mut map, "mm_goron_baby", MmFlagType::Song, 1 << 24);

    // Goron Lullaby (Full) - learned from Goron Elder
    add_mm_global_mapping(&mut map, "mm_goron_elder", MmFlagType::Song, 1 << 7);

    // New Wave Bossa Nova - learned at Marine Research Lab
    add_mm_global_mapping(
        &mut map,
        "mm_laboratory_zora_song",
        MmFlagType::Song,
        1 << 8,
    );

    // Elegy of Emptiness - learned from Igos du Ikana in Ancient Castle
    add_mm_global_mapping(
        &mut map,
        "mm_ancient_castle_of_ikana_elegy",
        MmFlagType::Song,
        1 << 9,
    );

    // Song of Storms - learned from Flat's ghost beneath the graveyard
    add_mm_global_mapping(
        &mut map,
        "mm_beneath_the_graveyard_song_of_storms",
        MmFlagType::Song,
        1 << 16,
    );

    // Epona's Song - learned from Romani at Romani Ranch
    add_mm_global_mapping(
        &mut map,
        "mm_romani_ranch_epona_song",
        MmFlagType::Song,
        1 << 14,
    );

    // Song of Soaring - learned from Owl in Southern Swamp
    add_mm_global_mapping(
        &mut map,
        "mm_southern_swamp_song_of_soaring",
        MmFlagType::Song,
        1 << 15,
    );

    // Oath to Order - learned after defeating all four bosses
    // This is typically obtained during the game's finale sequence

    // ========================================================================
    // NPC MASK REWARDS
    // ========================================================================
    // Masks obtained from NPCs are tracked by EventInf or WeekEventReg flags.
    // These flags are set when the player receives the mask.

    // Blast Mask - from old lady saved from Sakon (Night 1)
    add_mm_global_mapping(
        &mut map,
        "mm_clock_town_blast_mask",
        MmFlagType::WeekEventReg,
        0x01,
    );

    // Bremen Mask - from Guru-Guru in Laundry Pool
    add_mm_global_mapping(
        &mut map,
        "mm_clock_town_guru_guru_mask_bremen",
        MmFlagType::WeekEventReg,
        0x02,
    );

    // Kafei's Mask - from Madame Aroma in Mayor's Office
    add_mm_global_mapping(
        &mut map,
        "mm_mayors_office_kafeis_mask",
        MmFlagType::EventInf,
        0x01,
    );

    // Postman's Hat - from Postman after delivering mail
    add_mm_global_mapping(
        &mut map,
        "mm_clock_town_postman_hat",
        MmFlagType::WeekEventReg,
        0x04,
    );

    // All-Night Mask - from Curiosity Shop (expensive purchase)
    add_mm_global_mapping(
        &mut map,
        "mm_curiosity_shop_all_night_mask",
        MmFlagType::WeekEventReg,
        0x08,
    );

    // Troupe Leader's Mask - from Gorman Brothers at Milk Bar
    add_mm_global_mapping(
        &mut map,
        "mm_milk_bar_troupe_leader_mask",
        MmFlagType::WeekEventReg,
        0x10,
    );

    // Bunny Hood - from Grog at Cucco Shack
    add_mm_global_mapping(
        &mut map,
        "mm_cucco_shack_bunny_mask",
        MmFlagType::WeekEventReg,
        0x20,
    );

    // Don Gero's Mask - from hungry Goron in Mountain Village
    add_mm_global_mapping(
        &mut map,
        "mm_mountain_village_don_gero_mask",
        MmFlagType::WeekEventReg,
        0x40,
    );

    // Mask of Scents - from Deku Butler in Deku Shrine
    add_mm_global_mapping(
        &mut map,
        "mm_deku_shrine_mask_of_scents",
        MmFlagType::EventInf,
        0x02,
    );

    // Romani's Mask - from Cremia after escort mission
    add_mm_global_mapping(
        &mut map,
        "mm_romani_ranch_cremia_escort",
        MmFlagType::WeekEventReg,
        0x80,
    );

    // Garo's Mask - from Gorman Brothers at Gorman Track
    add_mm_global_mapping(
        &mut map,
        "mm_gorman_track_garo_mask",
        MmFlagType::EventInf,
        0x04,
    );

    // Captain's Hat - from Skull Keeta in Ikana Graveyard
    add_mm_global_mapping(
        &mut map,
        "mm_ikana_graveyard_captain_mask",
        MmFlagType::EventInf,
        0x08,
    );

    // Gibdo Mask - from Pamela's Father in Music Box House
    add_mm_global_mapping(
        &mut map,
        "mm_music_box_house_gibdo_mask",
        MmFlagType::EventInf,
        0x10,
    );

    // Stone Mask - from invisible soldier on Road to Ikana
    add_mm_global_mapping(
        &mut map,
        "mm_road_to_ikana_stone_mask",
        MmFlagType::EventInf,
        0x20,
    );

    // Kamaro's Mask - from Kamaro ghost in Termina Field
    add_mm_global_mapping(
        &mut map,
        "mm_termina_field_kamaro_mask",
        MmFlagType::EventInf,
        0x40,
    );

    // Goron Mask - from Darmani's ghost in Goron Graveyard
    add_mm_global_mapping(
        &mut map,
        "mm_goron_graveyard_mask",
        MmFlagType::EventInf,
        0x80,
    );

    // Zora Mask - from Mikau on Great Bay Coast
    add_mm_global_mapping(
        &mut map,
        "mm_great_bay_coast_zora_mask",
        MmFlagType::EventInf,
        0x0100,
    );

    // ========================================================================
    // NPC ITEM REWARDS
    // ========================================================================
    // Other important items obtained from NPCs.

    // Bomber's Notebook - from Jim after Hide and Seek
    add_mm_global_mapping(
        &mut map,
        "mm_clock_town_bomber_notebook",
        MmFlagType::EventInf,
        0x0200,
    );

    // Pendant of Memories - from Kafei in his hideout
    add_mm_global_mapping(
        &mut map,
        "mm_kafei_hideout_pendant_of_memories",
        MmFlagType::WeekEventReg,
        0x0100,
    );

    // Bottle from Madame Aroma - at Milk Bar
    add_mm_global_mapping(
        &mut map,
        "mm_milk_bar_madame_aroma_bottle",
        MmFlagType::WeekEventReg,
        0x0200,
    );

    // Post Box reward - delivering mail
    add_mm_global_mapping(
        &mut map,
        "mm_clock_town_post_box",
        MmFlagType::WeekEventReg,
        0x0400,
    );

    // Moon's Tear - from Astral Observatory
    add_mm_global_mapping(
        &mut map,
        "mm_astral_observatory_moon_tear",
        MmFlagType::MoonsTear,
        0x01,
    );

    // Pictobox - from Tourist Information
    add_mm_global_mapping(
        &mut map,
        "mm_tourist_information_pictobox",
        MmFlagType::EventInf,
        0x0400,
    );

    // Tingle Picture reward
    add_mm_global_mapping(
        &mut map,
        "mm_tourist_information_tingle_picture",
        MmFlagType::EventInf,
        0x0800,
    );

    // ========================================================================
    // MINIGAME REWARD LOCATIONS
    // ========================================================================

    // Romani Ranch - Aliens defense reward
    add_mm_global_mapping(
        &mut map,
        "mm_romani_ranch_aliens",
        MmFlagType::WeekEventReg,
        0x0800,
    );

    // Beaver Race rewards
    add_mm_global_mapping(
        &mut map,
        "mm_waterfall_rapids_beaver_race_1",
        MmFlagType::EventInf,
        0x8000,
    );
    add_mm_global_mapping(
        &mut map,
        "mm_waterfall_rapids_beaver_race_2",
        MmFlagType::ItemGetInf,
        0x01,
    );

    // Zora Hall Scene Lights minigame
    add_mm_global_mapping(
        &mut map,
        "mm_zora_hall_scene_lights",
        MmFlagType::WeekEventReg,
        0x1000,
    );

    // ========================================================================
    // GROTTO LOCATIONS
    // ========================================================================
    // Grottos typically contain collectibles or heart pieces.

    add_mm_mapping(
        &mut map,
        "mm_road_to_southern_swamp_grotto",
        mm_scene::ROAD_TO_SOUTHERN_SWAMP,
        MmFlagType::Collectible,
        0x01,
    );
    add_mm_mapping(
        &mut map,
        "mm_southern_swamp_grotto",
        mm_scene::SOUTHERN_SWAMP,
        MmFlagType::Collectible,
        0x01,
    );
    add_mm_mapping(
        &mut map,
        "mm_woods_of_mystery_grotto",
        0x59, // Woods of Mystery scene
        MmFlagType::Collectible,
        0x01,
    );
    add_mm_mapping(
        &mut map,
        "mm_mountain_village_tunnel_grotto",
        mm_scene::MOUNTAIN_VILLAGE,
        MmFlagType::Collectible,
        0x01,
    );
    add_mm_mapping(
        &mut map,
        "mm_path_to_snowhead_grotto",
        mm_scene::PATH_TO_SNOWHEAD,
        MmFlagType::Collectible,
        0x01,
    );
    add_mm_mapping(
        &mut map,
        "mm_zora_cape_grotto",
        mm_scene::ZORA_CAPE,
        MmFlagType::Collectible,
        0x01,
    );
    add_mm_mapping(
        &mut map,
        "mm_road_to_ikana_grotto",
        mm_scene::ROAD_TO_IKANA,
        MmFlagType::Collectible,
        0x01,
    );
    add_mm_mapping(
        &mut map,
        "mm_ikana_graveyard_grotto",
        mm_scene::IKANA_GRAVEYARD,
        MmFlagType::Collectible,
        0x01,
    );
    add_mm_mapping(
        &mut map,
        "mm_ikana_valley_grotto",
        mm_scene::IKANA_CANYON,
        MmFlagType::Collectible,
        0x02,
    );
    add_mm_mapping(
        &mut map,
        "mm_termina_field_peahat_grotto",
        mm_scene::TERMINA_FIELD,
        MmFlagType::Collectible,
        0x01,
    );
    add_mm_mapping(
        &mut map,
        "mm_termina_field_bio_baba_grotto",
        mm_scene::TERMINA_FIELD,
        MmFlagType::Collectible,
        0x02,
    );
    add_mm_mapping(
        &mut map,
        "mm_termina_field_dodongo_grotto",
        mm_scene::TERMINA_FIELD,
        MmFlagType::Collectible,
        0x04,
    );
    add_mm_mapping(
        &mut map,
        "mm_termina_field_pillar_grotto",
        mm_scene::TERMINA_FIELD,
        MmFlagType::Collectible,
        0x08,
    );

    // ========================================================================
    // BOSS REWARDS
    // ========================================================================
    // Boss remains are obtained after defeating temple bosses.

    add_mm_global_mapping(
        &mut map,
        "mm_woodfall_temple_boss_remains",
        MmFlagType::Boss,
        0x01,
    );
    add_mm_global_mapping(
        &mut map,
        "mm_snowhead_temple_boss_remains",
        MmFlagType::Boss,
        0x02,
    );
    add_mm_global_mapping(
        &mut map,
        "mm_great_bay_temple_boss_remains",
        MmFlagType::Boss,
        0x04,
    );
    add_mm_global_mapping(
        &mut map,
        "mm_stone_tower_temple_boss_remains",
        MmFlagType::Boss,
        0x08,
    );

    // ========================================================================
    // COW LOCATIONS
    // ========================================================================
    // Cows give milk when Epona's Song is played.

    add_mm_global_mapping(&mut map, "mm_romani_ranch_cow_1", MmFlagType::Cow, 0x01);
    add_mm_global_mapping(&mut map, "mm_romani_ranch_cow_2", MmFlagType::Cow, 0x02);
    add_mm_global_mapping(&mut map, "mm_romani_ranch_cow_3", MmFlagType::Cow, 0x04);
    add_mm_global_mapping(&mut map, "mm_great_bay_coast_cow", MmFlagType::Cow, 0x08);
    add_mm_global_mapping(&mut map, "mm_beneath_the_well_cow", MmFlagType::Cow, 0x10);

    // ========================================================================
    // DUNGEON CHESTS
    // ========================================================================

    // Woodfall Temple (Scene 0x1F)
    add_mm_mapping(
        &mut map,
        "mm_woodfall_temple_entrance_chest",
        mm_scene::WOODFALL_TEMPLE,
        MmFlagType::Chest,
        0x0000_0001,
    );
    add_mm_mapping(
        &mut map,
        "mm_woodfall_temple_water_chest",
        mm_scene::WOODFALL_TEMPLE,
        MmFlagType::Chest,
        0x0000_0002,
    );
    add_mm_mapping(
        &mut map,
        "mm_woodfall_temple_dark_chest",
        mm_scene::WOODFALL_TEMPLE,
        MmFlagType::Chest,
        0x0000_0004,
    );
    add_mm_mapping(
        &mut map,
        "mm_woodfall_temple_center_chest",
        mm_scene::WOODFALL_TEMPLE,
        MmFlagType::Chest,
        0x0000_0008,
    );
    add_mm_mapping(
        &mut map,
        "mm_woodfall_temple_boss_key_chest",
        mm_scene::WOODFALL_TEMPLE,
        MmFlagType::Chest,
        0x0000_0010,
    );

    // Snowhead Temple (Scene 0x22)
    add_mm_mapping(
        &mut map,
        "mm_snowhead_temple_map_chest",
        mm_scene::SNOWHEAD_TEMPLE,
        MmFlagType::Chest,
        0x0000_0001,
    );
    add_mm_mapping(
        &mut map,
        "mm_snowhead_temple_compass_chest",
        mm_scene::SNOWHEAD_TEMPLE,
        MmFlagType::Chest,
        0x0000_0002,
    );
    add_mm_mapping(
        &mut map,
        "mm_snowhead_temple_boss_key_chest",
        mm_scene::SNOWHEAD_TEMPLE,
        MmFlagType::Chest,
        0x0000_0004,
    );
    add_mm_mapping(
        &mut map,
        "mm_snowhead_temple_fire_arrow_chest",
        mm_scene::SNOWHEAD_TEMPLE,
        MmFlagType::Chest,
        0x0000_0008,
    );
    add_mm_mapping(
        &mut map,
        "mm_snowhead_temple_block_room_chest",
        mm_scene::SNOWHEAD_TEMPLE,
        MmFlagType::Chest,
        0x0000_0010,
    );
    add_mm_mapping(
        &mut map,
        "mm_snowhead_temple_icicle_room_chest",
        mm_scene::SNOWHEAD_TEMPLE,
        MmFlagType::Chest,
        0x0000_0020,
    );
    add_mm_mapping(
        &mut map,
        "mm_snowhead_temple_bridge_room_chest",
        mm_scene::SNOWHEAD_TEMPLE,
        MmFlagType::Chest,
        0x0000_0040,
    );

    // Great Bay Temple (Scene 0x1E)
    add_mm_mapping(
        &mut map,
        "mm_great_bay_temple_entrance_chest",
        mm_scene::GREAT_BAY_TEMPLE,
        MmFlagType::Chest,
        0x0000_0001,
    );
    add_mm_mapping(
        &mut map,
        "mm_great_bay_temple_baba_chest",
        mm_scene::GREAT_BAY_TEMPLE,
        MmFlagType::Chest,
        0x0000_0002,
    );
    add_mm_mapping(
        &mut map,
        "mm_great_bay_temple_green_pipe_1_chest",
        mm_scene::GREAT_BAY_TEMPLE,
        MmFlagType::Chest,
        0x0000_0004,
    );
    add_mm_mapping(
        &mut map,
        "mm_great_bay_temple_green_pipe_2_lower_chest",
        mm_scene::GREAT_BAY_TEMPLE,
        MmFlagType::Chest,
        0x0000_0008,
    );
    add_mm_mapping(
        &mut map,
        "mm_great_bay_temple_green_pipe_2_upper_chest",
        mm_scene::GREAT_BAY_TEMPLE,
        MmFlagType::Chest,
        0x0000_0010,
    );
    add_mm_mapping(
        &mut map,
        "mm_great_bay_temple_green_pipe_3_chest",
        mm_scene::GREAT_BAY_TEMPLE,
        MmFlagType::Chest,
        0x0000_0020,
    );
    add_mm_mapping(
        &mut map,
        "mm_great_bay_temple_map_chest",
        mm_scene::GREAT_BAY_TEMPLE,
        MmFlagType::Chest,
        0x0000_0040,
    );
    add_mm_mapping(
        &mut map,
        "mm_great_bay_temple_compass_chest",
        mm_scene::GREAT_BAY_TEMPLE,
        MmFlagType::Chest,
        0x0000_0080,
    );
    add_mm_mapping(
        &mut map,
        "mm_great_bay_temple_boss_key_chest",
        mm_scene::GREAT_BAY_TEMPLE,
        MmFlagType::Chest,
        0x0000_0100,
    );
    add_mm_mapping(
        &mut map,
        "mm_great_bay_temple_ice_arrow_chest",
        mm_scene::GREAT_BAY_TEMPLE,
        MmFlagType::Chest,
        0x0000_0200,
    );
    add_mm_mapping(
        &mut map,
        "mm_great_bay_temple_hookshot_chest",
        mm_scene::GREAT_BAY_TEMPLE,
        MmFlagType::Chest,
        0x0000_0400,
    );
    add_mm_mapping(
        &mut map,
        "mm_great_bay_temple_small_key_chest",
        mm_scene::GREAT_BAY_TEMPLE,
        MmFlagType::Chest,
        0x0000_0800,
    );

    // Stone Tower Temple (Scene 0x18)
    add_mm_mapping(
        &mut map,
        "mm_stone_tower_temple_entrance_chest",
        mm_scene::STONE_TOWER_TEMPLE,
        MmFlagType::Chest,
        0x0000_0001,
    );
    add_mm_mapping(
        &mut map,
        "mm_stone_tower_temple_entrance_switch_chest",
        mm_scene::STONE_TOWER_TEMPLE,
        MmFlagType::Chest,
        0x0000_0002,
    );
    add_mm_mapping(
        &mut map,
        "mm_stone_tower_temple_under_west_garden_ledge_chest",
        mm_scene::STONE_TOWER_TEMPLE,
        MmFlagType::Chest,
        0x0000_0004,
    );
    add_mm_mapping(
        &mut map,
        "mm_stone_tower_temple_under_west_garden_lava_chest",
        mm_scene::STONE_TOWER_TEMPLE,
        MmFlagType::Chest,
        0x0000_0008,
    );
    add_mm_mapping(
        &mut map,
        "mm_stone_tower_temple_center_sun_block_chest",
        mm_scene::STONE_TOWER_TEMPLE,
        MmFlagType::Chest,
        0x0000_0010,
    );
    add_mm_mapping(
        &mut map,
        "mm_stone_tower_temple_center_across_water_chest",
        mm_scene::STONE_TOWER_TEMPLE,
        MmFlagType::Chest,
        0x0000_0020,
    );
    add_mm_mapping(
        &mut map,
        "mm_stone_tower_temple_water_sun_switch_chest",
        mm_scene::STONE_TOWER_TEMPLE,
        MmFlagType::Chest,
        0x0000_0040,
    );
    add_mm_mapping(
        &mut map,
        "mm_stone_tower_temple_mirrors_room_center_chest",
        mm_scene::STONE_TOWER_TEMPLE,
        MmFlagType::Chest,
        0x0000_0080,
    );
    add_mm_mapping(
        &mut map,
        "mm_stone_tower_temple_mirrors_room_right_chest",
        mm_scene::STONE_TOWER_TEMPLE,
        MmFlagType::Chest,
        0x0000_0100,
    );
    add_mm_mapping(
        &mut map,
        "mm_stone_tower_temple_wind_room_ledge_chest",
        mm_scene::STONE_TOWER_TEMPLE,
        MmFlagType::Chest,
        0x0000_0200,
    );
    add_mm_mapping(
        &mut map,
        "mm_stone_tower_temple_wind_room_jail_chest",
        mm_scene::STONE_TOWER_TEMPLE,
        MmFlagType::Chest,
        0x0000_0400,
    );
    add_mm_mapping(
        &mut map,
        "mm_stone_tower_temple_before_water_bridge_chest",
        mm_scene::STONE_TOWER_TEMPLE,
        MmFlagType::Chest,
        0x0000_0800,
    );
    add_mm_mapping(
        &mut map,
        "mm_stone_tower_temple_water_bridge_chest",
        mm_scene::STONE_TOWER_TEMPLE,
        MmFlagType::Chest,
        0x0000_1000,
    );
    add_mm_mapping(
        &mut map,
        "mm_stone_tower_temple_map_chest",
        mm_scene::STONE_TOWER_TEMPLE,
        MmFlagType::Chest,
        0x0000_2000,
    );
    add_mm_mapping(
        &mut map,
        "mm_stone_tower_temple_compass_chest",
        mm_scene::STONE_TOWER_TEMPLE,
        MmFlagType::Chest,
        0x0000_4000,
    );
    add_mm_mapping(
        &mut map,
        "mm_stone_tower_temple_boss_key_chest",
        mm_scene::STONE_TOWER_TEMPLE,
        MmFlagType::Chest,
        0x0000_8000,
    );
    add_mm_mapping(
        &mut map,
        "mm_stone_tower_temple_light_arrow_chest",
        mm_scene::STONE_TOWER_TEMPLE,
        MmFlagType::Chest,
        0x0001_0000,
    );
    add_mm_mapping(
        &mut map,
        "mm_stone_tower_temple_giants_mask_chest",
        mm_scene::STONE_TOWER_TEMPLE,
        MmFlagType::Chest,
        0x0002_0000,
    );

    // Stone Tower Temple Inverted (Scene 0x19)
    add_mm_mapping(
        &mut map,
        "mm_stone_tower_temple_inverted_entrance_chest",
        mm_scene::STONE_TOWER_TEMPLE_INVERTED,
        MmFlagType::Chest,
        0x0000_0001,
    );
    add_mm_mapping(
        &mut map,
        "mm_stone_tower_temple_inverted_east_lower_chest",
        mm_scene::STONE_TOWER_TEMPLE_INVERTED,
        MmFlagType::Chest,
        0x0000_0002,
    );
    add_mm_mapping(
        &mut map,
        "mm_stone_tower_temple_inverted_east_upper_chest",
        mm_scene::STONE_TOWER_TEMPLE_INVERTED,
        MmFlagType::Chest,
        0x0000_0004,
    );
    add_mm_mapping(
        &mut map,
        "mm_stone_tower_temple_inverted_east_middle_chest",
        mm_scene::STONE_TOWER_TEMPLE_INVERTED,
        MmFlagType::Chest,
        0x0000_0008,
    );
    add_mm_mapping(
        &mut map,
        "mm_stone_tower_temple_inverted_wizzrobe_chest",
        mm_scene::STONE_TOWER_TEMPLE_INVERTED,
        MmFlagType::Chest,
        0x0000_0010,
    );
    add_mm_mapping(
        &mut map,
        "mm_stone_tower_temple_inverted_death_armos_chest",
        mm_scene::STONE_TOWER_TEMPLE_INVERTED,
        MmFlagType::Chest,
        0x0000_0020,
    );

    // ========================================================================
    // MINI-DUNGEON CHESTS
    // ========================================================================

    // Beneath the Well (Scene 0x1B)
    add_mm_mapping(
        &mut map,
        "mm_beneath_the_well_keese_chest",
        mm_scene::BENEATH_THE_WELL,
        MmFlagType::Chest,
        0x0000_0001,
    );
    add_mm_mapping(
        &mut map,
        "mm_beneath_the_well_skulltulla_chest",
        mm_scene::BENEATH_THE_WELL,
        MmFlagType::Chest,
        0x0000_0002,
    );
    add_mm_mapping(
        &mut map,
        "mm_beneath_the_well_mirror_shield_chest",
        mm_scene::BENEATH_THE_WELL,
        MmFlagType::Chest,
        0x0000_0004,
    );
    add_mm_mapping(
        &mut map,
        "mm_beneath_the_well_compass_chest",
        mm_scene::BENEATH_THE_WELL,
        MmFlagType::Chest,
        0x0000_0008,
    );
    add_mm_mapping(
        &mut map,
        "mm_beneath_the_well_map_chest",
        mm_scene::BENEATH_THE_WELL,
        MmFlagType::Chest,
        0x0000_0010,
    );

    // Ancient Castle of Ikana (Scene 0x11)
    add_mm_mapping(
        &mut map,
        "mm_ancient_castle_of_ikana_powder_keg_chest",
        mm_scene::ANCIENT_CASTLE_OF_IKANA,
        MmFlagType::Chest,
        0x0000_0001,
    );
    add_mm_mapping(
        &mut map,
        "mm_ancient_castle_of_ikana_compass_chest",
        mm_scene::ANCIENT_CASTLE_OF_IKANA,
        MmFlagType::Chest,
        0x0000_0002,
    );
    add_mm_mapping(
        &mut map,
        "mm_ancient_castle_of_ikana_map_chest",
        mm_scene::ANCIENT_CASTLE_OF_IKANA,
        MmFlagType::Chest,
        0x0000_0004,
    );

    // Secret Shrine (Scene 0x13)
    add_mm_mapping(
        &mut map,
        "mm_secret_shrine_dinolfos_chest",
        mm_scene::IKANA_CANYON_SECRET_SHRINE,
        MmFlagType::Chest,
        0x0000_0001,
    );
    add_mm_mapping(
        &mut map,
        "mm_secret_shrine_wizzrobe_chest",
        mm_scene::IKANA_CANYON_SECRET_SHRINE,
        MmFlagType::Chest,
        0x0000_0002,
    );
    add_mm_mapping(
        &mut map,
        "mm_secret_shrine_wart_chest",
        mm_scene::IKANA_CANYON_SECRET_SHRINE,
        MmFlagType::Chest,
        0x0000_0004,
    );
    add_mm_mapping(
        &mut map,
        "mm_secret_shrine_garo_master_chest",
        mm_scene::IKANA_CANYON_SECRET_SHRINE,
        MmFlagType::Chest,
        0x0000_0008,
    );

    // Pirates Fortress Exterior (Scene 0x29)
    add_mm_mapping(
        &mut map,
        "mm_pirate_fortress_entrance_chest_1",
        mm_scene::PIRATES_FORTRESS,
        MmFlagType::Chest,
        0x0000_0001,
    );
    add_mm_mapping(
        &mut map,
        "mm_pirate_fortress_entrance_chest_2",
        mm_scene::PIRATES_FORTRESS,
        MmFlagType::Chest,
        0x0000_0002,
    );
    add_mm_mapping(
        &mut map,
        "mm_pirate_fortress_entrance_chest_3",
        mm_scene::PIRATES_FORTRESS,
        MmFlagType::Chest,
        0x0000_0004,
    );
    add_mm_mapping(
        &mut map,
        "mm_pirate_fortress_sewers_chest_1",
        mm_scene::PIRATES_FORTRESS,
        MmFlagType::Chest,
        0x0000_0008,
    );
    add_mm_mapping(
        &mut map,
        "mm_pirate_fortress_sewers_chest_2",
        mm_scene::PIRATES_FORTRESS,
        MmFlagType::Chest,
        0x0000_0010,
    );
    add_mm_mapping(
        &mut map,
        "mm_pirate_fortress_sewers_chest_3",
        mm_scene::PIRATES_FORTRESS,
        MmFlagType::Chest,
        0x0000_0020,
    );

    // Pirates Fortress Interior (Scene 0x2A)
    add_mm_mapping(
        &mut map,
        "mm_pirate_fortress_interior_lower_chest",
        mm_scene::PIRATES_FORTRESS_INTERIOR,
        MmFlagType::Chest,
        0x0000_0001,
    );
    add_mm_mapping(
        &mut map,
        "mm_pirate_fortress_interior_upper_chest",
        mm_scene::PIRATES_FORTRESS_INTERIOR,
        MmFlagType::Chest,
        0x0000_0002,
    );
    add_mm_mapping(
        &mut map,
        "mm_pirate_fortress_interior_pot_chest_aquarium_1",
        mm_scene::PIRATES_FORTRESS_INTERIOR,
        MmFlagType::Chest,
        0x0000_0004,
    );
    add_mm_mapping(
        &mut map,
        "mm_pirate_fortress_interior_pot_chest_aquarium_2",
        mm_scene::PIRATES_FORTRESS_INTERIOR,
        MmFlagType::Chest,
        0x0000_0008,
    );
    add_mm_mapping(
        &mut map,
        "mm_pirate_fortress_interior_pot_chest_aquarium_3",
        mm_scene::PIRATES_FORTRESS_INTERIOR,
        MmFlagType::Chest,
        0x0000_0010,
    );
    add_mm_mapping(
        &mut map,
        "mm_pirate_fortress_interior_silver_rupee_chest",
        mm_scene::PIRATES_FORTRESS_INTERIOR,
        MmFlagType::Chest,
        0x0000_0020,
    );

    // Beneath the Graveyard (Scene 0x07)
    add_mm_mapping(
        &mut map,
        "mm_beneath_the_graveyard_chest",
        mm_scene::BENEATH_THE_GRAVEYARD,
        MmFlagType::Chest,
        0x0000_0001,
    );
    add_mm_mapping(
        &mut map,
        "mm_beneath_the_graveyard_dampe_chest",
        mm_scene::BENEATH_THE_GRAVEYARD,
        MmFlagType::Chest,
        0x0000_0002,
    );

    // ========================================================================
    // OVERWORLD CHESTS
    // ========================================================================

    // Clock Town South (Scene 0x6C)
    add_mm_mapping(
        &mut map,
        "mm_clock_town_south_chest_lower",
        mm_scene::CLOCK_TOWN_SOUTH,
        MmFlagType::Chest,
        0x0000_0001,
    );
    add_mm_mapping(
        &mut map,
        "mm_clock_town_south_chest_upper",
        mm_scene::CLOCK_TOWN_SOUTH,
        MmFlagType::Chest,
        0x0000_0002,
    );

    // Clock Town East (Scene 0x6E)
    add_mm_mapping(
        &mut map,
        "mm_clock_town_silver_rupee_chest",
        mm_scene::CLOCK_TOWN_EAST,
        MmFlagType::Chest,
        0x0000_0001,
    );

    // Astral Observatory (Scene 0x52)
    add_mm_mapping(
        &mut map,
        "mm_astral_observatory_passage_chest",
        mm_scene::ASTRAL_OBSERVATORY,
        MmFlagType::Chest,
        0x0000_0001,
    );

    // Stock Pot Inn (Scene 0x4D)
    add_mm_mapping(
        &mut map,
        "mm_stock_pot_inn_guest_room_chest",
        mm_scene::STOCK_POT_INN,
        MmFlagType::Chest,
        0x0000_0001,
    );
    add_mm_mapping(
        &mut map,
        "mm_stock_pot_inn_staff_room_chest",
        mm_scene::STOCK_POT_INN,
        MmFlagType::Chest,
        0x0000_0002,
    );

    // Deku Palace Grotto (Scene 0x59)
    add_mm_mapping(
        &mut map,
        "mm_deku_palace_grotto_chest",
        0x59,
        MmFlagType::Chest,
        0x0000_0001,
    );

    // Lone Peak Shrine / Lens of Truth Cave (Scene 0x17)
    add_mm_mapping(
        &mut map,
        "mm_lone_peak_shrine_lens_chest",
        0x17,
        MmFlagType::Chest,
        0x0000_0001,
    );
    add_mm_mapping(
        &mut map,
        "mm_lone_peak_shrine_boulder_chest",
        0x17,
        MmFlagType::Chest,
        0x0000_0002,
    );
    add_mm_mapping(
        &mut map,
        "mm_lone_peak_shrine_invisible_chest",
        0x17,
        MmFlagType::Chest,
        0x0000_0004,
    );

    // Mountain Village (Scene 0x65)
    add_mm_mapping(
        &mut map,
        "mm_mountain_village_waterfall_chest",
        0x65,
        MmFlagType::Chest,
        0x0000_0001,
    );

    // Twin Islands Spring (Scene 0x49)
    add_mm_mapping(
        &mut map,
        "mm_twin_islands_underwater_chest_1",
        0x49,
        MmFlagType::Chest,
        0x0000_0001,
    );
    add_mm_mapping(
        &mut map,
        "mm_twin_islands_underwater_chest_2",
        0x49,
        MmFlagType::Chest,
        0x0000_0002,
    );
    add_mm_mapping(
        &mut map,
        "mm_twin_islands_ramp_grotto_chest",
        0x49,
        MmFlagType::Chest,
        0x0000_0004,
    );
    add_mm_mapping(
        &mut map,
        "mm_twin_islands_frozen_grotto_chest",
        0x49,
        MmFlagType::Chest,
        0x0000_0008,
    );

    // Great Bay Coast (Scene 0x37)
    add_mm_mapping(
        &mut map,
        "mm_great_bay_coast_ledge_chest",
        mm_scene::GREAT_BAY_COAST,
        MmFlagType::Chest,
        0x0000_0001,
    );

    // Zora Cape (Scene 0x38)
    add_mm_mapping(
        &mut map,
        "mm_zora_cape_underwater_chest",
        mm_scene::ZORA_CAPE,
        MmFlagType::Chest,
        0x0000_0001,
    );
    add_mm_mapping(
        &mut map,
        "mm_zora_cape_ledge_chest_1",
        mm_scene::ZORA_CAPE,
        MmFlagType::Chest,
        0x0000_0002,
    );
    add_mm_mapping(
        &mut map,
        "mm_zora_cape_ledge_chest_2",
        mm_scene::ZORA_CAPE,
        MmFlagType::Chest,
        0x0000_0004,
    );

    // Pinnacle Rock (Scene 0x3F)
    add_mm_mapping(
        &mut map,
        "mm_pinnacle_rock_chest_1",
        mm_scene::PINNACLE_ROCK,
        MmFlagType::Chest,
        0x0000_0001,
    );
    add_mm_mapping(
        &mut map,
        "mm_pinnacle_rock_chest_2",
        mm_scene::PINNACLE_ROCK,
        MmFlagType::Chest,
        0x0000_0002,
    );

    // Road to Ikana (Scene 0x47)
    add_mm_mapping(
        &mut map,
        "mm_road_to_ikana_chest",
        mm_scene::ROAD_TO_IKANA,
        MmFlagType::Chest,
        0x0000_0001,
    );

    // Termina Field (Scene 0x54)
    add_mm_mapping(
        &mut map,
        "mm_termina_field_water_chest",
        mm_scene::TERMINA_FIELD,
        MmFlagType::Chest,
        0x0000_0001,
    );
    add_mm_mapping(
        &mut map,
        "mm_termina_field_tall_grass_chest",
        mm_scene::TERMINA_FIELD,
        MmFlagType::Chest,
        0x0000_0002,
    );
    add_mm_mapping(
        &mut map,
        "mm_termina_field_tree_stump_chest",
        mm_scene::TERMINA_FIELD,
        MmFlagType::Chest,
        0x0000_0004,
    );

    // Woodfall (Scene 0x14)
    add_mm_mapping(
        &mut map,
        "mm_woodfall_entrance_chest",
        0x14,
        MmFlagType::Chest,
        0x0000_0001,
    );
    add_mm_mapping(
        &mut map,
        "mm_woodfall_near_owl_chest",
        0x14,
        MmFlagType::Chest,
        0x0000_0002,
    );

    // Stone Tower Exterior Inverted (Scene 0x0F)
    add_mm_mapping(
        &mut map,
        "mm_stone_tower_inverted_chest_1",
        0x0F,
        MmFlagType::Chest,
        0x0000_0001,
    );
    add_mm_mapping(
        &mut map,
        "mm_stone_tower_inverted_chest_2",
        0x0F,
        MmFlagType::Chest,
        0x0000_0002,
    );
    add_mm_mapping(
        &mut map,
        "mm_stone_tower_inverted_chest_3",
        0x0F,
        MmFlagType::Chest,
        0x0000_0004,
    );

    // Doggy Racetrack (Scene 0x62)
    add_mm_mapping(
        &mut map,
        "mm_doggy_racetrack_chest",
        0x62,
        MmFlagType::Chest,
        0x0000_0001,
    );

    // ========================================================================
    // MOON TRIAL CHESTS
    // ========================================================================
    add_mm_mapping(
        &mut map,
        "mm_moon_trial_link_garo_master_chest",
        0x66,
        MmFlagType::Chest,
        0x0000_0001,
    );
    add_mm_mapping(
        &mut map,
        "mm_moon_trial_link_iron_knuckle_chest",
        0x66,
        MmFlagType::Chest,
        0x0000_0002,
    );

    // ========================================================================
    // HEART PIECES (Collectible flags)
    // ========================================================================

    // Clock Town Heart Pieces
    add_mm_mapping(
        &mut map,
        "mm_clock_town_platform_hp",
        mm_scene::CLOCK_TOWN_SOUTH,
        MmFlagType::Collectible,
        0x0000_0001,
    );
    add_mm_mapping(
        &mut map,
        "mm_clock_town_tree_hp",
        mm_scene::CLOCK_TOWN_NORTH,
        MmFlagType::Collectible,
        0x0000_0001,
    );
    add_mm_mapping(
        &mut map,
        "mm_clock_town_keaton_hp",
        mm_scene::CLOCK_TOWN_NORTH,
        MmFlagType::Collectible,
        0x0000_0002,
    );
    add_mm_mapping(
        &mut map,
        "mm_clock_town_rosa_sisters_hp",
        mm_scene::CLOCK_TOWN_WEST,
        MmFlagType::Collectible,
        0x0000_0001,
    );
    add_mm_mapping(
        &mut map,
        "mm_post_office_hp",
        mm_scene::POST_OFFICE,
        MmFlagType::Collectible,
        0x0000_0001,
    );
    add_mm_mapping(
        &mut map,
        "mm_swordsman_school_hp",
        0x26,
        MmFlagType::Collectible,
        0x0000_0002,
    );
    add_mm_mapping(
        &mut map,
        "mm_mayors_office_hp",
        mm_scene::MAYORS_OFFICE,
        MmFlagType::Collectible,
        0x0000_0001,
    );
    add_mm_mapping(
        &mut map,
        "mm_chest_game_hp",
        mm_scene::TREASURE_CHEST_SHOP,
        MmFlagType::Collectible,
        0x0000_0001,
    );
    add_mm_mapping(
        &mut map,
        "mm_stock_pot_inn_grandma_hp_1",
        mm_scene::STOCK_POT_INN,
        MmFlagType::Collectible,
        0x0000_0001,
    );
    add_mm_mapping(
        &mut map,
        "mm_stock_pot_inn_grandma_hp_2",
        mm_scene::STOCK_POT_INN,
        MmFlagType::Collectible,
        0x0000_0002,
    );
    add_mm_mapping(
        &mut map,
        "mm_stock_pot_inn_hp",
        mm_scene::STOCK_POT_INN,
        MmFlagType::Collectible,
        0x0000_0004,
    );

    // Termina Field Heart Pieces
    add_mm_mapping(
        &mut map,
        "mm_termina_field_gossip_stones_hp",
        mm_scene::TERMINA_FIELD,
        MmFlagType::Collectible,
        0x0000_0001,
    );

    // Southern Swamp Area Heart Pieces
    add_mm_mapping(
        &mut map,
        "mm_road_to_southern_swamp_hp",
        mm_scene::ROAD_TO_SOUTHERN_SWAMP,
        MmFlagType::Collectible,
        0x0000_0001,
    );
    add_mm_mapping(
        &mut map,
        "mm_deku_palace_hp",
        0x59,
        MmFlagType::Collectible,
        0x0000_0001,
    );
    add_mm_mapping(
        &mut map,
        "mm_southern_swamp_hp",
        mm_scene::SOUTHERN_SWAMP,
        MmFlagType::Collectible,
        0x0000_0001,
    );
    add_mm_mapping(
        &mut map,
        "mm_woodfall_hp_chest",
        0x14,
        MmFlagType::Collectible,
        0x0000_0001,
    );

    // Mountain Area Heart Pieces
    add_mm_mapping(
        &mut map,
        "mm_goron_village_hp",
        0x6A,
        MmFlagType::Collectible,
        0x0000_0001,
    );
    add_mm_mapping(
        &mut map,
        "mm_mountain_village_frog_choir_hp",
        0x65,
        MmFlagType::Collectible,
        0x0000_0001,
    );
    add_mm_mapping(
        &mut map,
        "mm_path_to_snowhead_hp",
        0x21,
        MmFlagType::Collectible,
        0x0000_0001,
    );

    // Great Bay Area Heart Pieces
    add_mm_mapping(
        &mut map,
        "mm_great_bay_coast_hp",
        mm_scene::GREAT_BAY_COAST,
        MmFlagType::Collectible,
        0x0000_0001,
    );
    add_mm_mapping(
        &mut map,
        "mm_great_bay_coast_fisherman_hp",
        mm_scene::GREAT_BAY_COAST,
        MmFlagType::Collectible,
        0x0000_0002,
    );
    add_mm_mapping(
        &mut map,
        "mm_laboratory_fish_hp",
        mm_scene::GREAT_BAY_COAST,
        MmFlagType::Collectible,
        0x0000_0004,
    );
    add_mm_mapping(
        &mut map,
        "mm_pinnacle_rock_hp",
        mm_scene::PINNACLE_ROCK,
        MmFlagType::Collectible,
        0x0000_0001,
    );
    add_mm_mapping(
        &mut map,
        "mm_zora_cape_waterfall_hp",
        mm_scene::ZORA_CAPE,
        MmFlagType::Collectible,
        0x0000_0001,
    );
    add_mm_mapping(
        &mut map,
        "mm_zora_hall_evan_hp",
        0x39,
        MmFlagType::Collectible,
        0x0000_0001,
    );
    add_mm_mapping(
        &mut map,
        "mm_zora_hall_scrub_hp",
        0x39,
        MmFlagType::Collectible,
        0x0000_0002,
    );

    // Ikana Area Heart Pieces
    add_mm_mapping(
        &mut map,
        "mm_ikana_valley_scrub_hp",
        0x46,
        MmFlagType::Collectible,
        0x0000_0001,
    );
    add_mm_mapping(
        &mut map,
        "mm_ghost_hut_hp",
        0x46,
        MmFlagType::Collectible,
        0x0000_0002,
    );
    add_mm_mapping(
        &mut map,
        "mm_beneath_the_graveyard_hp",
        mm_scene::BENEATH_THE_GRAVEYARD,
        MmFlagType::Collectible,
        0x0000_0001,
    );

    // Romani Ranch Area Heart Pieces
    add_mm_mapping(
        &mut map,
        "mm_doggy_racetrack_hp",
        0x62,
        MmFlagType::Collectible,
        0x0000_0001,
    );

    // Moon Trial Heart Pieces (stored as chest flags)
    add_mm_mapping(
        &mut map,
        "mm_moon_trial_deku_hp",
        0x67,
        MmFlagType::Chest,
        0x0000_0001,
    );
    add_mm_mapping(
        &mut map,
        "mm_moon_trial_goron_hp",
        0x68,
        MmFlagType::Chest,
        0x0000_0001,
    );
    add_mm_mapping(
        &mut map,
        "mm_moon_trial_zora_hp",
        0x69,
        MmFlagType::Chest,
        0x0000_0001,
    );
    add_mm_mapping(
        &mut map,
        "mm_moon_trial_link_hp",
        0x66,
        MmFlagType::Chest,
        0x0000_0004,
    );

    // ========================================================================
    // OCEANSIDE SPIDER HOUSE (Scene 0x28)
    // ========================================================================
    add_mm_mapping(
        &mut map,
        "mm_ocean_spider_house_chest_hp",
        mm_scene::OCEANSIDE_SPIDER_HOUSE,
        MmFlagType::Chest,
        0x0000_0001,
    );

    // Shop Item Mappings (Global Shop Flags)
    // ========================================================================
    // Shop items are tracked globally via shop purchase flags.
    // The flag_bit corresponds to the shop_price() index from OoTMM logic.

    // Bomb Shop (0x00-0x03)
    add_mm_global_mapping(&mut map, "mm_bomb_shop_item_1", MmFlagType::Shop, 0x00);
    add_mm_global_mapping(&mut map, "mm_bomb_shop_item_2", MmFlagType::Shop, 0x01);

    // Trading Post (0x05-0x0C)
    add_mm_global_mapping(&mut map, "mm_trading_post_item_1", MmFlagType::Shop, 0x05);
    add_mm_global_mapping(&mut map, "mm_trading_post_item_2", MmFlagType::Shop, 0x06);
    add_mm_global_mapping(&mut map, "mm_trading_post_item_3", MmFlagType::Shop, 0x07);
    add_mm_global_mapping(&mut map, "mm_trading_post_item_4", MmFlagType::Shop, 0x08);
    add_mm_global_mapping(&mut map, "mm_trading_post_item_5", MmFlagType::Shop, 0x09);
    add_mm_global_mapping(&mut map, "mm_trading_post_item_6", MmFlagType::Shop, 0x0A);
    add_mm_global_mapping(&mut map, "mm_trading_post_item_7", MmFlagType::Shop, 0x0B);
    add_mm_global_mapping(&mut map, "mm_trading_post_item_8", MmFlagType::Shop, 0x0C);

    // Goron Shop (0x10-0x12)
    add_mm_global_mapping(&mut map, "mm_goron_shop_item_1", MmFlagType::Shop, 0x10);
    add_mm_global_mapping(&mut map, "mm_goron_shop_item_2", MmFlagType::Shop, 0x11);
    add_mm_global_mapping(&mut map, "mm_goron_shop_item_3", MmFlagType::Shop, 0x12);

    // Zora Shop (0x13-0x15)
    add_mm_global_mapping(&mut map, "mm_zora_shop_item_1", MmFlagType::Shop, 0x13);
    add_mm_global_mapping(&mut map, "mm_zora_shop_item_2", MmFlagType::Shop, 0x14);
    add_mm_global_mapping(&mut map, "mm_zora_shop_item_3", MmFlagType::Shop, 0x15);

    // Milk Bar Purchases (tracked via week event flags)
    add_mm_global_mapping(
        &mut map,
        "mm_milk_bar_purchase_milk",
        MmFlagType::Shop,
        0x16,
    );
    add_mm_global_mapping(
        &mut map,
        "mm_milk_bar_purchase_chateau",
        MmFlagType::Shop,
        0x17,
    );

    // Gorman Track Milk Purchase
    add_mm_global_mapping(
        &mut map,
        "mm_gorman_track_milk_purchase",
        MmFlagType::Shop,
        0x18,
    );

    // ========================================================================
    // Business Scrub Mappings (Global Scrub Flags)
    // ========================================================================
    // Business scrubs are tracked globally via scrub purchase flags.

    // Clock Town Business Scrub - sells Moon's Tear trade item
    add_mm_global_mapping(
        &mut map,
        "mm_clock_town_business_scrub",
        MmFlagType::Scrub,
        0x00,
    );

    // Southern Swamp Scrubs
    add_mm_global_mapping(
        &mut map,
        "mm_southern_swamp_scrub_deed",
        MmFlagType::Scrub,
        0x01,
    );
    add_mm_global_mapping(
        &mut map,
        "mm_southern_swamp_scrub_shop",
        MmFlagType::Scrub,
        0x02,
    );

    // Goron Village Scrubs
    add_mm_global_mapping(
        &mut map,
        "mm_goron_village_scrub_deed",
        MmFlagType::Scrub,
        0x03,
    );
    add_mm_global_mapping(
        &mut map,
        "mm_goron_village_scrub_bomb_bag",
        MmFlagType::Scrub,
        0x04,
    );

    // Zora Hall Scrubs
    add_mm_global_mapping(&mut map, "mm_zora_hall_scrub_deed", MmFlagType::Scrub, 0x05);
    add_mm_global_mapping(&mut map, "mm_zora_hall_scrub_shop", MmFlagType::Scrub, 0x06);

    // Ikana Valley Scrub
    add_mm_global_mapping(
        &mut map,
        "mm_ikana_valley_scrub_shop",
        MmFlagType::Scrub,
        0x07,
    );

    // Termina Field Scrub Grotto
    add_mm_global_mapping(&mut map, "mm_termina_field_scrub", MmFlagType::Scrub, 0x08);
    add_mm_global_mapping(
        &mut map,
        "mm_termina_field_scrub_crate",
        MmFlagType::Scrub,
        0x09,
    );

    // ========================================================================
    // Wonder Item Mappings (Scene Collectible Flags)
    // ========================================================================
    // Wonder items are tracked per-scene as collectible flags.
    // These are special items that appear when hitting targets or other triggers.

    // Clock Town South Wonder Items (Scene: CLOCK_TOWN_SOUTH = 0x6C)
    add_mm_mapping(
        &mut map,
        "mm_clock_town_south_wonder_item_1",
        mm_scene::CLOCK_TOWN_SOUTH,
        MmFlagType::Collectible,
        0x01,
    );
    add_mm_mapping(
        &mut map,
        "mm_clock_town_south_wonder_item_2",
        mm_scene::CLOCK_TOWN_SOUTH,
        MmFlagType::Collectible,
        0x02,
    );
    add_mm_mapping(
        &mut map,
        "mm_clock_town_south_wonder_item_3",
        mm_scene::CLOCK_TOWN_SOUTH,
        MmFlagType::Collectible,
        0x03,
    );

    // Clock Town East Wonder Items - Target Left (Scene: CLOCK_TOWN_EAST = 0x6E)
    add_mm_mapping(
        &mut map,
        "mm_clock_town_east_wonder_item_target_left_1",
        mm_scene::CLOCK_TOWN_EAST,
        MmFlagType::Collectible,
        0x01,
    );
    add_mm_mapping(
        &mut map,
        "mm_clock_town_east_wonder_item_target_left_2",
        mm_scene::CLOCK_TOWN_EAST,
        MmFlagType::Collectible,
        0x02,
    );
    add_mm_mapping(
        &mut map,
        "mm_clock_town_east_wonder_item_target_left_3",
        mm_scene::CLOCK_TOWN_EAST,
        MmFlagType::Collectible,
        0x03,
    );

    // Clock Town East Wonder Items - Target Right
    add_mm_mapping(
        &mut map,
        "mm_clock_town_east_wonder_item_target_right_1",
        mm_scene::CLOCK_TOWN_EAST,
        MmFlagType::Collectible,
        0x04,
    );
    add_mm_mapping(
        &mut map,
        "mm_clock_town_east_wonder_item_target_right_2",
        mm_scene::CLOCK_TOWN_EAST,
        MmFlagType::Collectible,
        0x05,
    );
    add_mm_mapping(
        &mut map,
        "mm_clock_town_east_wonder_item_target_right_3",
        mm_scene::CLOCK_TOWN_EAST,
        MmFlagType::Collectible,
        0x06,
    );

    // Clock Town East Wonder Items - Basket
    add_mm_mapping(
        &mut map,
        "mm_clock_town_east_wonder_item_basket_1",
        mm_scene::CLOCK_TOWN_EAST,
        MmFlagType::Collectible,
        0x07,
    );
    add_mm_mapping(
        &mut map,
        "mm_clock_town_east_wonder_item_basket_2",
        mm_scene::CLOCK_TOWN_EAST,
        MmFlagType::Collectible,
        0x08,
    );
    add_mm_mapping(
        &mut map,
        "mm_clock_town_east_wonder_item_basket_3",
        mm_scene::CLOCK_TOWN_EAST,
        MmFlagType::Collectible,
        0x09,
    );

    // Ikana Graveyard Wonder Items (Scene: IKANA_GRAVEYARD = 0x09)
    add_mm_mapping(
        &mut map,
        "mm_ikana_graveyard_wonder_item_01",
        mm_scene::IKANA_GRAVEYARD,
        MmFlagType::Collectible,
        0x01,
    );
    add_mm_mapping(
        &mut map,
        "mm_ikana_graveyard_wonder_item_02",
        mm_scene::IKANA_GRAVEYARD,
        MmFlagType::Collectible,
        0x02,
    );
    add_mm_mapping(
        &mut map,
        "mm_ikana_graveyard_wonder_item_03",
        mm_scene::IKANA_GRAVEYARD,
        MmFlagType::Collectible,
        0x03,
    );
    add_mm_mapping(
        &mut map,
        "mm_ikana_graveyard_wonder_item_04",
        mm_scene::IKANA_GRAVEYARD,
        MmFlagType::Collectible,
        0x04,
    );
    add_mm_mapping(
        &mut map,
        "mm_ikana_graveyard_wonder_item_05",
        mm_scene::IKANA_GRAVEYARD,
        MmFlagType::Collectible,
        0x05,
    );
    add_mm_mapping(
        &mut map,
        "mm_ikana_graveyard_wonder_item_06",
        mm_scene::IKANA_GRAVEYARD,
        MmFlagType::Collectible,
        0x06,
    );
    add_mm_mapping(
        &mut map,
        "mm_ikana_graveyard_wonder_item_07",
        mm_scene::IKANA_GRAVEYARD,
        MmFlagType::Collectible,
        0x07,
    );
    add_mm_mapping(
        &mut map,
        "mm_ikana_graveyard_wonder_item_08",
        mm_scene::IKANA_GRAVEYARD,
        MmFlagType::Collectible,
        0x08,
    );
    add_mm_mapping(
        &mut map,
        "mm_ikana_graveyard_wonder_item_09",
        mm_scene::IKANA_GRAVEYARD,
        MmFlagType::Collectible,
        0x09,
    );
    add_mm_mapping(
        &mut map,
        "mm_ikana_graveyard_wonder_item_10",
        mm_scene::IKANA_GRAVEYARD,
        MmFlagType::Collectible,
        0x0A,
    );
    add_mm_mapping(
        &mut map,
        "mm_ikana_graveyard_wonder_item_11",
        mm_scene::IKANA_GRAVEYARD,
        MmFlagType::Collectible,
        0x0B,
    );
    add_mm_mapping(
        &mut map,
        "mm_ikana_graveyard_wonder_item_12",
        mm_scene::IKANA_GRAVEYARD,
        MmFlagType::Collectible,
        0x0C,
    );

    // Romani Ranch Wonder Items - Fence (Scene: ROMANI_RANCH = 0x64)
    add_mm_mapping(
        &mut map,
        "mm_romani_ranch_wonder_item_fence_1",
        mm_scene::ROMANI_RANCH,
        MmFlagType::Collectible,
        0x01,
    );
    add_mm_mapping(
        &mut map,
        "mm_romani_ranch_wonder_item_fence_2",
        mm_scene::ROMANI_RANCH,
        MmFlagType::Collectible,
        0x02,
    );
    add_mm_mapping(
        &mut map,
        "mm_romani_ranch_wonder_item_fence_3",
        mm_scene::ROMANI_RANCH,
        MmFlagType::Collectible,
        0x03,
    );
    add_mm_mapping(
        &mut map,
        "mm_romani_ranch_wonder_item_fence_4",
        mm_scene::ROMANI_RANCH,
        MmFlagType::Collectible,
        0x04,
    );
    add_mm_mapping(
        &mut map,
        "mm_romani_ranch_wonder_item_fence_5",
        mm_scene::ROMANI_RANCH,
        MmFlagType::Collectible,
        0x05,
    );
    add_mm_mapping(
        &mut map,
        "mm_romani_ranch_wonder_item_fence_6",
        mm_scene::ROMANI_RANCH,
        MmFlagType::Collectible,
        0x06,
    );

    // Romani Ranch Barn Wonder Items (Scene: uses ROMANI_RANCH for barn area)
    add_mm_mapping(
        &mut map,
        "mm_romani_ranch_barn_wonder_item_1",
        mm_scene::ROMANI_RANCH,
        MmFlagType::Collectible,
        0x10,
    );
    add_mm_mapping(
        &mut map,
        "mm_romani_ranch_barn_wonder_item_2",
        mm_scene::ROMANI_RANCH,
        MmFlagType::Collectible,
        0x11,
    );

    // Cucco Shack Wonder Items (Scene: CUCCO_SHACK = 0x5F)
    add_mm_mapping(
        &mut map,
        "mm_cucco_shack_wonder_item_1",
        mm_scene::CUCCO_SHACK,
        MmFlagType::Collectible,
        0x01,
    );
    add_mm_mapping(
        &mut map,
        "mm_cucco_shack_wonder_item_2",
        mm_scene::CUCCO_SHACK,
        MmFlagType::Collectible,
        0x02,
    );
    add_mm_mapping(
        &mut map,
        "mm_cucco_shack_wonder_item_3",
        mm_scene::CUCCO_SHACK,
        MmFlagType::Collectible,
        0x03,
    );
    add_mm_mapping(
        &mut map,
        "mm_cucco_shack_wonder_item_4",
        mm_scene::CUCCO_SHACK,
        MmFlagType::Collectible,
        0x04,
    );
    add_mm_mapping(
        &mut map,
        "mm_cucco_shack_wonder_item_5",
        mm_scene::CUCCO_SHACK,
        MmFlagType::Collectible,
        0x05,
    );
    add_mm_mapping(
        &mut map,
        "mm_cucco_shack_wonder_item_6",
        mm_scene::CUCCO_SHACK,
        MmFlagType::Collectible,
        0x06,
    );

    // Termina Field Wonder Items (Scene: TERMINA_FIELD = 0x54)
    add_mm_mapping(
        &mut map,
        "mm_termina_field_wonder_item_hollow_trunk",
        mm_scene::TERMINA_FIELD,
        MmFlagType::Collectible,
        0x01,
    );
    add_mm_mapping(
        &mut map,
        "mm_termina_field_wonder_item_fountains_1",
        mm_scene::TERMINA_FIELD,
        MmFlagType::Collectible,
        0x02,
    );
    add_mm_mapping(
        &mut map,
        "mm_termina_field_wonder_item_fountains_2",
        mm_scene::TERMINA_FIELD,
        MmFlagType::Collectible,
        0x03,
    );
    add_mm_mapping(
        &mut map,
        "mm_termina_field_wonder_item_north_ramp",
        mm_scene::TERMINA_FIELD,
        MmFlagType::Collectible,
        0x04,
    );
    add_mm_mapping(
        &mut map,
        "mm_termina_field_wonder_item_west_ramp",
        mm_scene::TERMINA_FIELD,
        MmFlagType::Collectible,
        0x05,
    );
    add_mm_mapping(
        &mut map,
        "mm_termina_field_wonder_item_south_west_ramp",
        mm_scene::TERMINA_FIELD,
        MmFlagType::Collectible,
        0x06,
    );
    add_mm_mapping(
        &mut map,
        "mm_termina_field_wonder_item_shell_1",
        mm_scene::TERMINA_FIELD,
        MmFlagType::Collectible,
        0x07,
    );
    add_mm_mapping(
        &mut map,
        "mm_termina_field_wonder_item_shell_2",
        mm_scene::TERMINA_FIELD,
        MmFlagType::Collectible,
        0x08,
    );
    add_mm_mapping(
        &mut map,
        "mm_termina_field_wonder_item_shell_3",
        mm_scene::TERMINA_FIELD,
        MmFlagType::Collectible,
        0x09,
    );
    add_mm_mapping(
        &mut map,
        "mm_termina_field_wonder_item_shell_side_1",
        mm_scene::TERMINA_FIELD,
        MmFlagType::Collectible,
        0x0A,
    );
    add_mm_mapping(
        &mut map,
        "mm_termina_field_wonder_item_shell_side_2",
        mm_scene::TERMINA_FIELD,
        MmFlagType::Collectible,
        0x0B,
    );
    add_mm_mapping(
        &mut map,
        "mm_termina_field_wonder_item_shell_side_3",
        mm_scene::TERMINA_FIELD,
        MmFlagType::Collectible,
        0x0C,
    );
    add_mm_mapping(
        &mut map,
        "mm_termina_field_wonder_item_graffiti_1",
        mm_scene::TERMINA_FIELD,
        MmFlagType::Collectible,
        0x0D,
    );
    add_mm_mapping(
        &mut map,
        "mm_termina_field_wonder_item_graffiti_2",
        mm_scene::TERMINA_FIELD,
        MmFlagType::Collectible,
        0x0E,
    );
    add_mm_mapping(
        &mut map,
        "mm_termina_field_wonder_item_graffiti_3",
        mm_scene::TERMINA_FIELD,
        MmFlagType::Collectible,
        0x0F,
    );

    // ========================================================================
    // Soil Item Mappings (Scene Collectible Flags)
    // ========================================================================
    // Soil items (bug drops) are tracked per-scene as collectible flags.

    // Deku Palace Soil Items (Scene: DEKU_PALACE = 0x14)
    add_mm_mapping(
        &mut map,
        "mm_deku_palace_soil_item_1",
        mm_scene::DEKU_PALACE,
        MmFlagType::Collectible,
        0x10,
    );
    add_mm_mapping(
        &mut map,
        "mm_deku_palace_soil_item_2",
        mm_scene::DEKU_PALACE,
        MmFlagType::Collectible,
        0x11,
    );
    add_mm_mapping(
        &mut map,
        "mm_deku_palace_soil_item_3",
        mm_scene::DEKU_PALACE,
        MmFlagType::Collectible,
        0x12,
    );

    // Beans Grotto Soil Items (Grotto scene - uses TERMINA_FIELD base)
    add_mm_mapping(
        &mut map,
        "mm_beans_grotto_soil_item_1",
        mm_scene::TERMINA_FIELD,
        MmFlagType::Collectible,
        0x20,
    );
    add_mm_mapping(
        &mut map,
        "mm_beans_grotto_soil_item_2",
        mm_scene::TERMINA_FIELD,
        MmFlagType::Collectible,
        0x21,
    );
    add_mm_mapping(
        &mut map,
        "mm_beans_grotto_soil_item_3",
        mm_scene::TERMINA_FIELD,
        MmFlagType::Collectible,
        0x22,
    );

    // Romani Ranch Soil Items - Days 2-3 (Scene: ROMANI_RANCH)
    add_mm_mapping(
        &mut map,
        "mm_romani_ranch_soil_days_2_3_item_1",
        mm_scene::ROMANI_RANCH,
        MmFlagType::Collectible,
        0x20,
    );
    add_mm_mapping(
        &mut map,
        "mm_romani_ranch_soil_days_2_3_item_2",
        mm_scene::ROMANI_RANCH,
        MmFlagType::Collectible,
        0x21,
    );
    add_mm_mapping(
        &mut map,
        "mm_romani_ranch_soil_days_2_3_item_3",
        mm_scene::ROMANI_RANCH,
        MmFlagType::Collectible,
        0x22,
    );

    // Termina Field Soil Items - Observatory Area
    add_mm_mapping(
        &mut map,
        "mm_termina_field_soil_observatory_item_1",
        mm_scene::TERMINA_FIELD,
        MmFlagType::Collectible,
        0x30,
    );
    add_mm_mapping(
        &mut map,
        "mm_termina_field_soil_observatory_item_2",
        mm_scene::TERMINA_FIELD,
        MmFlagType::Collectible,
        0x31,
    );
    add_mm_mapping(
        &mut map,
        "mm_termina_field_soil_observatory_item_3",
        mm_scene::TERMINA_FIELD,
        MmFlagType::Collectible,
        0x32,
    );

    // Termina Field Soil Items - Wall Area
    add_mm_mapping(
        &mut map,
        "mm_termina_field_soil_wall_item_1",
        mm_scene::TERMINA_FIELD,
        MmFlagType::Collectible,
        0x33,
    );
    add_mm_mapping(
        &mut map,
        "mm_termina_field_soil_wall_item_2",
        mm_scene::TERMINA_FIELD,
        MmFlagType::Collectible,
        0x34,
    );
    add_mm_mapping(
        &mut map,
        "mm_termina_field_soil_wall_item_3",
        mm_scene::TERMINA_FIELD,
        MmFlagType::Collectible,
        0x35,
    );

    map
});

// ============================================================================
// Helper Functions (for adding mappings)
// ============================================================================

/// Adds a scene-based mapping to the MM mappings table.
///
/// This is a helper function used during table initialization to add
/// fully mapped locations with scene-based flags.
#[allow(dead_code)]
fn add_mm_mapping(
    map: &mut HashMap<&'static str, MmFlagMapping>,
    location_id: &'static str,
    scene_id: u8,
    flag_type: MmFlagType,
    flag_bit: u32,
) {
    map.insert(
        location_id,
        MmFlagMapping::mapped(location_id, scene_id, flag_type, flag_bit),
    );
}

/// Adds a global flag mapping to the MM mappings table.
///
/// This is a helper function used during table initialization to add
/// mappings for global flags (not scene-specific).
#[allow(dead_code)]
fn add_mm_global_mapping(
    map: &mut HashMap<&'static str, MmFlagMapping>,
    location_id: &'static str,
    flag_type: MmFlagType,
    flag_bit: u32,
) {
    map.insert(
        location_id,
        MmFlagMapping::global(location_id, flag_type, flag_bit),
    );
}

// ============================================================================
// Public API
// ============================================================================

/// Returns the flag mapping for a MM location ID, if it exists.
///
/// Returns `Some(mapping)` if the location exists (even if unmapped stub),
/// or `None` if the location ID is not recognized.
#[must_use]
pub fn get_mm_mapping(location_id: &str) -> Option<&'static MmFlagMapping> {
    MM_MAPPINGS.get(location_id)
}

/// Returns an iterator over all MM location mappings.
///
/// This includes both mapped locations and unmapped stubs.
pub fn get_all_mm_mappings() -> impl Iterator<Item = &'static MmFlagMapping> {
    MM_MAPPINGS.values()
}

/// Returns the count of all MM locations.
#[must_use]
pub fn mm_location_count() -> usize {
    MM_MAPPINGS.len()
}

/// Returns the count of mapped (non-stub) MM locations.
#[must_use]
pub fn mm_mapped_count() -> usize {
    MM_MAPPINGS.values().filter(|m| m.is_mapped()).count()
}

/// Returns the count of unmapped stub locations.
#[must_use]
pub fn mm_stub_count() -> usize {
    MM_MAPPINGS.values().filter(|m| m.is_stub()).count()
}

/// Returns an iterator over only the mapped (non-stub) locations.
pub fn get_mm_mapped_locations() -> impl Iterator<Item = &'static MmFlagMapping> {
    MM_MAPPINGS.values().filter(|m| m.is_mapped())
}

/// Returns an iterator over only the stub locations.
pub fn get_mm_stub_locations() -> impl Iterator<Item = &'static MmFlagMapping> {
    MM_MAPPINGS.values().filter(|m| m.is_stub())
}

/// Returns all mappings for a specific scene.
pub fn get_mm_mappings_for_scene(scene_id: u8) -> impl Iterator<Item = &'static MmFlagMapping> {
    MM_MAPPINGS
        .values()
        .filter(move |m| m.scene_id == Some(scene_id))
}

/// Returns all mappings for a specific flag type.
pub fn get_mm_mappings_by_flag_type(
    flag_type: MmFlagType,
) -> impl Iterator<Item = &'static MmFlagMapping> {
    MM_MAPPINGS
        .values()
        .filter(move |m| m.flag_type == Some(flag_type))
}

/// Returns all MM location IDs from the world data.
pub fn get_all_mm_location_ids() -> impl Iterator<Item = &'static str> {
    MM_LOCATION_IDS.iter().copied()
}

// ============================================================================
// Location Status Checking
// ============================================================================

use crate::flag_mapping::{CheckStatus, LocationCheckResult};
use crate::mm_save::MmSave;

/// Checks if a specific MM location has been checked based on the save data.
///
/// # Arguments
///
/// * `mapping` - The flag mapping for the location
/// * `mm_save` - The MM save data
///
/// # Returns
///
/// `CheckStatus::Checked` if the location flag is set,
/// `CheckStatus::Unchecked` if the flag is not set,
/// `CheckStatus::Unknown` if the location is unmapped or cannot be determined.
#[must_use]
pub fn check_mm_location_status(mapping: &MmFlagMapping, mm_save: &MmSave) -> CheckStatus {
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
        MmFlagType::Chest => {
            if let Some(scene_id) = mapping.scene_id {
                if let Some(scene_flags) = mm_save.permanent_scene_flags.get(scene_id as usize) {
                    if scene_flags.chest & flag_bit != 0 {
                        CheckStatus::Checked
                    } else {
                        CheckStatus::Unchecked
                    }
                } else {
                    CheckStatus::Unknown
                }
            } else {
                CheckStatus::Unknown
            }
        }
        MmFlagType::Switch0 => {
            if let Some(scene_id) = mapping.scene_id {
                if let Some(scene_flags) = mm_save.permanent_scene_flags.get(scene_id as usize) {
                    if scene_flags.switch0 & flag_bit != 0 {
                        CheckStatus::Checked
                    } else {
                        CheckStatus::Unchecked
                    }
                } else {
                    CheckStatus::Unknown
                }
            } else {
                CheckStatus::Unknown
            }
        }
        MmFlagType::Switch1 => {
            if let Some(scene_id) = mapping.scene_id {
                if let Some(scene_flags) = mm_save.permanent_scene_flags.get(scene_id as usize) {
                    if scene_flags.switch1 & flag_bit != 0 {
                        CheckStatus::Checked
                    } else {
                        CheckStatus::Unchecked
                    }
                } else {
                    CheckStatus::Unknown
                }
            } else {
                CheckStatus::Unknown
            }
        }
        MmFlagType::ClearedRoom => {
            if let Some(scene_id) = mapping.scene_id {
                if let Some(scene_flags) = mm_save.permanent_scene_flags.get(scene_id as usize) {
                    if scene_flags.cleared_room & flag_bit != 0 {
                        CheckStatus::Checked
                    } else {
                        CheckStatus::Unchecked
                    }
                } else {
                    CheckStatus::Unknown
                }
            } else {
                CheckStatus::Unknown
            }
        }
        MmFlagType::Collectible => {
            if let Some(scene_id) = mapping.scene_id {
                if let Some(scene_flags) = mm_save.permanent_scene_flags.get(scene_id as usize) {
                    if scene_flags.collectible & flag_bit != 0 {
                        CheckStatus::Checked
                    } else {
                        CheckStatus::Unchecked
                    }
                } else {
                    CheckStatus::Unknown
                }
            } else {
                CheckStatus::Unknown
            }
        }
        // These flag types need special handling or are not yet implemented
        MmFlagType::GoldSkulltula
        | MmFlagType::EventInf
        | MmFlagType::WeekEventReg
        | MmFlagType::ItemGetInf
        | MmFlagType::Shop
        | MmFlagType::Scrub
        | MmFlagType::GreatFairy
        | MmFlagType::Boss
        | MmFlagType::Song
        | MmFlagType::Cow
        | MmFlagType::StrayFairy
        | MmFlagType::OwlStatue
        | MmFlagType::MoonsTear
        | MmFlagType::GossipStone => CheckStatus::Unknown,
    }
}

/// Returns all checked MM locations for the current save data.
///
/// # Arguments
///
/// * `mm_save` - The MM save data
///
/// # Returns
///
/// A vector of `LocationCheckResult` for all mapped MM locations.
pub fn get_all_mm_checked_locations(mm_save: &MmSave) -> Vec<LocationCheckResult> {
    get_mm_mapped_locations()
        .map(|mapping| LocationCheckResult {
            location_id: mapping.location_id.to_string(),
            status: check_mm_location_status(mapping, mm_save),
            is_mapped: mapping.is_mapped(),
        })
        .collect()
}

/// Returns all MM locations (mapped and unmapped) with their check status.
///
/// # Arguments
///
/// * `mm_save` - Optional MM save data
///
/// # Returns
///
/// A vector of `LocationCheckResult` for all MM locations.
/// If mm_save is None, all locations will have Unknown status.
pub fn get_all_mm_locations_with_status(mm_save: Option<&MmSave>) -> Vec<LocationCheckResult> {
    match mm_save {
        Some(save) => get_all_mm_mappings()
            .map(|mapping| LocationCheckResult {
                location_id: mapping.location_id.to_string(),
                status: check_mm_location_status(mapping, save),
                is_mapped: mapping.is_mapped(),
            })
            .collect(),
        None => get_all_mm_mappings()
            .map(|mapping| LocationCheckResult {
                location_id: mapping.location_id.to_string(),
                status: CheckStatus::Unknown,
                is_mapped: mapping.is_mapped(),
            })
            .collect(),
    }
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mm_flag_type_scene_offset() {
        assert_eq!(MmFlagType::Chest.scene_offset(), Some(0x00));
        assert_eq!(MmFlagType::Switch0.scene_offset(), Some(0x04));
        assert_eq!(MmFlagType::Switch1.scene_offset(), Some(0x08));
        assert_eq!(MmFlagType::ClearedRoom.scene_offset(), Some(0x0C));
        assert_eq!(MmFlagType::Collectible.scene_offset(), Some(0x10));

        // Global flags should return None
        assert_eq!(MmFlagType::GoldSkulltula.scene_offset(), None);
        assert_eq!(MmFlagType::EventInf.scene_offset(), None);
        assert_eq!(MmFlagType::WeekEventReg.scene_offset(), None);
        assert_eq!(MmFlagType::StrayFairy.scene_offset(), None);
    }

    #[test]
    fn test_mm_flag_type_is_scene_based() {
        assert!(MmFlagType::Chest.is_scene_based());
        assert!(MmFlagType::Switch0.is_scene_based());
        assert!(MmFlagType::Switch1.is_scene_based());
        assert!(MmFlagType::ClearedRoom.is_scene_based());
        assert!(MmFlagType::Collectible.is_scene_based());

        assert!(!MmFlagType::GoldSkulltula.is_scene_based());
        assert!(!MmFlagType::EventInf.is_scene_based());
        assert!(!MmFlagType::Boss.is_scene_based());
    }

    #[test]
    fn test_mm_flag_mapping_stub() {
        let stub = MmFlagMapping::stub("test_location");
        assert!(stub.is_stub());
        assert!(!stub.is_mapped());
        assert_eq!(stub.location_id, "test_location");
        assert_eq!(stub.scene_id, None);
        assert_eq!(stub.flag_type, None);
        assert_eq!(stub.flag_bit, None);
    }

    #[test]
    fn test_mm_flag_mapping_mapped() {
        let mapped = MmFlagMapping::mapped(
            "test_location",
            mm_scene::WOODFALL_TEMPLE,
            MmFlagType::Chest,
            0x01,
        );
        assert!(!mapped.is_stub());
        assert!(mapped.is_mapped());
        assert_eq!(mapped.location_id, "test_location");
        assert_eq!(mapped.scene_id, Some(mm_scene::WOODFALL_TEMPLE));
        assert_eq!(mapped.flag_type, Some(MmFlagType::Chest));
        assert_eq!(mapped.flag_bit, Some(0x01));
    }

    #[test]
    fn test_mm_flag_mapping_global() {
        let global = MmFlagMapping::global("test_global", MmFlagType::EventInf, 0x10);
        assert!(!global.is_stub());
        assert!(global.is_mapped());
        assert_eq!(global.scene_id, None);
        assert_eq!(global.flag_type, Some(MmFlagType::EventInf));
    }

    #[test]
    fn test_mm_mappings_loaded() {
        // Verify that MM_MAPPINGS is populated from world data
        let count = mm_location_count();
        assert!(
            count > 0,
            "MM_MAPPINGS should have locations from world data"
        );

        // Verify stub + mapped counts match total
        let stub_count = mm_stub_count();
        let mapped_count = mm_mapped_count();

        assert_eq!(
            stub_count + mapped_count,
            count,
            "stub + mapped should equal total"
        );

        // Verify we have some mapped chest and collectible locations
        assert!(
            mapped_count > 100,
            "Should have over 100 mapped locations (chests + heart pieces)"
        );
    }

    #[test]
    fn test_mm_location_ids_loaded() {
        // Verify that MM_LOCATION_IDS is populated
        let ids: Vec<_> = get_all_mm_location_ids().collect();
        assert!(!ids.is_empty(), "Should have MM location IDs");

        // All IDs should be unique
        let mut unique_ids = ids.clone();
        unique_ids.sort();
        unique_ids.dedup();
        assert_eq!(
            ids.len(),
            unique_ids.len(),
            "All location IDs should be unique"
        );
    }

    #[test]
    fn test_get_mm_mapping() {
        // Try to get a mapping - we know at least some MM locations exist
        let ids: Vec<_> = get_all_mm_location_ids().collect();
        if !ids.is_empty() {
            let first_id = ids[0];
            let mapping = get_mm_mapping(first_id);
            assert!(
                mapping.is_some(),
                "Should find mapping for known location ID"
            );
            assert_eq!(mapping.unwrap().location_id, first_id);
        }

        // Non-existent location should return None
        assert!(get_mm_mapping("non_existent_location_xyz").is_none());
    }

    #[test]
    fn test_scene_constants() {
        // Verify scene constants are reasonable
        // Note: Using runtime check to avoid clippy's const evaluation warnings
        let scenes: [u8; 5] = [
            mm_scene::WOODFALL_TEMPLE,
            mm_scene::SNOWHEAD_TEMPLE,
            mm_scene::GREAT_BAY_TEMPLE,
            mm_scene::STONE_TOWER_TEMPLE,
            mm_scene::CLOCK_TOWN_SOUTH,
        ];
        for scene in scenes {
            assert!(scene < mm_scene::MAX_SCENE_ID);
        }
    }

    #[test]
    fn test_get_mm_mappings_by_flag_type() {
        // Get all chest mappings
        let chest_mappings: Vec<_> = get_mm_mappings_by_flag_type(MmFlagType::Chest).collect();
        // All returned mappings should be properly mapped
        assert!(!chest_mappings.is_empty(), "Should have chest mappings");
        assert!(
            chest_mappings.iter().all(|m| m.is_mapped()),
            "All returned mappings should be mapped"
        );
    }

    #[test]
    fn test_get_mm_mappings_for_scene() {
        // Get Woodfall Temple mappings
        let woodfall_mappings: Vec<_> =
            get_mm_mappings_for_scene(mm_scene::WOODFALL_TEMPLE).collect();
        // Should have Woodfall Temple chest mappings
        assert!(
            !woodfall_mappings.is_empty(),
            "Should have Woodfall Temple mappings"
        );
        for mapping in &woodfall_mappings {
            assert_eq!(mapping.scene_id, Some(mm_scene::WOODFALL_TEMPLE));
        }
    }

    #[test]
    fn test_owl_statue_mappings() {
        // All 10 owl statues should be mapped
        let owl_mappings: Vec<_> = get_mm_mappings_by_flag_type(MmFlagType::OwlStatue).collect();
        assert_eq!(owl_mappings.len(), 10, "Should have 10 owl statue mappings");

        // Verify all owl statues are properly mapped
        let owl_ids = [
            "mm_clock_town_owl_statue",
            "mm_milk_road_owl_statue",
            "mm_southern_swamp_owl_statue",
            "mm_woodfall_owl_statue",
            "mm_mountain_village_owl_statue",
            "mm_snowhead_owl_statue",
            "mm_zora_cape_owl_statue",
            "mm_great_bay_coast_owl_statue",
            "mm_ikana_canyon_owl_statue",
            "mm_stone_tower_owl_statue",
        ];

        for owl_id in owl_ids {
            let mapping = get_mm_mapping(owl_id);
            assert!(mapping.is_some(), "Should find mapping for {}", owl_id);

            let m = mapping.unwrap();
            assert!(m.is_mapped(), "{} should be mapped", owl_id);
            assert_eq!(
                m.flag_type,
                Some(MmFlagType::OwlStatue),
                "{} should have OwlStatue flag type",
                owl_id
            );
            assert!(
                m.scene_id.is_none(),
                "{} should be a global flag (no scene_id)",
                owl_id
            );
        }
    }

    #[test]
    fn test_owl_statue_flag_bits() {
        // Verify each owl statue has a unique, correct bit mask
        let expected_bits = [
            ("mm_clock_town_owl_statue", 1 << owl_bits::OWL_CLOCK_TOWN),
            ("mm_milk_road_owl_statue", 1 << owl_bits::OWL_MILK_ROAD),
            (
                "mm_southern_swamp_owl_statue",
                1 << owl_bits::OWL_SOUTHERN_SWAMP,
            ),
            ("mm_woodfall_owl_statue", 1 << owl_bits::OWL_WOODFALL),
            (
                "mm_mountain_village_owl_statue",
                1 << owl_bits::OWL_MOUNTAIN_VILLAGE,
            ),
            ("mm_snowhead_owl_statue", 1 << owl_bits::OWL_SNOWHEAD),
            ("mm_zora_cape_owl_statue", 1 << owl_bits::OWL_ZORA_CAPE),
            (
                "mm_great_bay_coast_owl_statue",
                1 << owl_bits::OWL_GREAT_BAY,
            ),
            (
                "mm_ikana_canyon_owl_statue",
                1 << owl_bits::OWL_IKANA_CANYON,
            ),
            ("mm_stone_tower_owl_statue", 1 << owl_bits::OWL_STONE_TOWER),
        ];

        for (loc_id, expected_bit) in expected_bits {
            let mapping = get_mm_mapping(loc_id).expect("Mapping should exist");
            assert_eq!(
                mapping.flag_bit,
                Some(expected_bit),
                "{} should have flag_bit {}",
                loc_id,
                expected_bit
            );
        }
    }

    #[test]
    fn test_shop_mappings() {
        // Verify shop mappings are populated
        let shop_mappings: Vec<_> = get_mm_mappings_by_flag_type(MmFlagType::Shop).collect();

        // We should have 19 shop mappings:
        // - 2 bomb shop items
        // - 8 trading post items
        // - 3 goron shop items
        // - 3 zora shop items
        // - 2 milk bar purchases
        // - 1 gorman track milk purchase
        assert_eq!(shop_mappings.len(), 19, "Should have 19 shop mappings");

        // Verify all shop mappings are properly mapped (not stubs)
        for mapping in &shop_mappings {
            assert!(mapping.is_mapped(), "Shop mapping should not be a stub");
            assert_eq!(
                mapping.flag_type,
                Some(MmFlagType::Shop),
                "Should be Shop flag type"
            );
        }

        // Verify specific shop items exist
        assert!(get_mm_mapping("mm_bomb_shop_item_1").is_some());
        assert!(get_mm_mapping("mm_trading_post_item_1").is_some());
        assert!(get_mm_mapping("mm_goron_shop_item_1").is_some());
        assert!(get_mm_mapping("mm_zora_shop_item_1").is_some());
        assert!(get_mm_mapping("mm_milk_bar_purchase_milk").is_some());
        assert!(get_mm_mapping("mm_gorman_track_milk_purchase").is_some());
    }

    #[test]
    fn test_scrub_mappings() {
        // Verify scrub mappings are populated
        let scrub_mappings: Vec<_> = get_mm_mappings_by_flag_type(MmFlagType::Scrub).collect();

        // We should have 10 scrub mappings:
        // - 1 Clock Town business scrub
        // - 2 Southern Swamp scrubs
        // - 2 Goron Village scrubs
        // - 2 Zora Hall scrubs
        // - 1 Ikana Valley scrub
        // - 2 Termina Field scrub grotto
        assert_eq!(scrub_mappings.len(), 10, "Should have 10 scrub mappings");

        // Verify all scrub mappings are properly mapped (not stubs)
        for mapping in &scrub_mappings {
            assert!(mapping.is_mapped(), "Scrub mapping should not be a stub");
            assert_eq!(
                mapping.flag_type,
                Some(MmFlagType::Scrub),
                "Should be Scrub flag type"
            );
        }

        // Verify specific scrub locations exist
        assert!(get_mm_mapping("mm_clock_town_business_scrub").is_some());
        assert!(get_mm_mapping("mm_southern_swamp_scrub_deed").is_some());
        assert!(get_mm_mapping("mm_goron_village_scrub_deed").is_some());
        assert!(get_mm_mapping("mm_zora_hall_scrub_deed").is_some());
        assert!(get_mm_mapping("mm_ikana_valley_scrub_shop").is_some());
        assert!(get_mm_mapping("mm_termina_field_scrub").is_some());
    }

    #[test]
    fn test_wonder_item_mappings() {
        // Verify wonder item mappings (using Collectible flag type)
        let collectible_mappings: Vec<_> =
            get_mm_mappings_by_flag_type(MmFlagType::Collectible).collect();

        // We should have many wonder items and soil items mapped as collectibles
        // Clock Town South: 3, Clock Town East: 9, Ikana Graveyard: 12,
        // Romani Ranch Fence: 6, Romani Ranch Barn: 2, Cucco Shack: 6,
        // Termina Field: 15, Deku Palace Soil: 3, Beans Grotto Soil: 3,
        // Romani Ranch Soil: 3, Termina Field Soil: 6
        // Total: 68 collectible mappings
        assert!(
            collectible_mappings.len() >= 68,
            "Should have at least 68 collectible (wonder/soil) mappings, got {}",
            collectible_mappings.len()
        );

        // Verify specific wonder items exist
        assert!(get_mm_mapping("mm_clock_town_south_wonder_item_1").is_some());
        assert!(get_mm_mapping("mm_clock_town_east_wonder_item_target_left_1").is_some());
        assert!(get_mm_mapping("mm_ikana_graveyard_wonder_item_01").is_some());
        assert!(get_mm_mapping("mm_romani_ranch_wonder_item_fence_1").is_some());
        assert!(get_mm_mapping("mm_cucco_shack_wonder_item_1").is_some());
        assert!(get_mm_mapping("mm_termina_field_wonder_item_hollow_trunk").is_some());

        // Verify specific soil items exist
        assert!(get_mm_mapping("mm_deku_palace_soil_item_1").is_some());
        assert!(get_mm_mapping("mm_beans_grotto_soil_item_1").is_some());
        assert!(get_mm_mapping("mm_romani_ranch_soil_days_2_3_item_1").is_some());
        assert!(get_mm_mapping("mm_termina_field_soil_observatory_item_1").is_some());
    }

    #[test]
    fn test_total_shop_scrub_mappings() {
        // Verify total number of mapped (non-stub) locations
        let mapped_count = mm_mapped_count();

        // We added:
        // - 19 shop mappings
        // - 10 scrub mappings
        // - 68 wonder/soil item mappings (collectibles)
        // Total: 97 mappings
        assert!(
            mapped_count >= 97,
            "Should have at least 97 mapped locations, got {}",
            mapped_count
        );
    }

    #[test]
    fn test_stray_fairy_great_fairy_mappings() {
        // Test that Great Fairy reward locations are mapped with StrayFairy type
        let great_fairy_locations = [
            ("mm_clock_town_great_fairy", 0),
            ("mm_clock_town_great_fairy_alt", 0),
            ("mm_woodfall_great_fairy", 1),
            ("mm_snowhead_great_fairy", 2),
            ("mm_great_bay_great_fairy", 3),
            ("mm_ikana_great_fairy", 4),
        ];

        for (location_id, expected_bit) in great_fairy_locations {
            let mapping = get_mm_mapping(location_id);
            assert!(
                mapping.is_some(),
                "Great Fairy location {} should exist",
                location_id
            );
            let mapping = mapping.unwrap();
            assert!(
                mapping.is_mapped(),
                "Great Fairy location {} should be mapped",
                location_id
            );
            assert_eq!(
                mapping.flag_type,
                Some(MmFlagType::StrayFairy),
                "Great Fairy location {} should use StrayFairy flag type",
                location_id
            );
            assert_eq!(
                mapping.flag_bit,
                Some(expected_bit),
                "Great Fairy location {} should have flag_bit {}",
                location_id,
                expected_bit
            );
            // Great Fairy rewards are global (no scene_id)
            assert_eq!(
                mapping.scene_id, None,
                "Great Fairy location {} should be global (no scene_id)",
                location_id
            );
        }
    }

    #[test]
    fn test_stray_fairy_collectible_mappings() {
        // Test Clock Town stray fairy
        let clock_town_fairy = get_mm_mapping("mm_clock_town_stray_fairy");
        assert!(
            clock_town_fairy.is_some(),
            "Clock Town stray fairy should exist"
        );
        let mapping = clock_town_fairy.unwrap();
        assert!(
            mapping.is_mapped(),
            "Clock Town stray fairy should be mapped"
        );
        assert_eq!(
            mapping.flag_type,
            Some(MmFlagType::Collectible),
            "Clock Town stray fairy should use Collectible flag type"
        );
        assert_eq!(
            mapping.scene_id,
            Some(mm_scene::LAUNDRY_POOL),
            "Clock Town stray fairy should be in Laundry Pool scene"
        );
    }

    #[test]
    fn test_beneath_the_well_fairy_mappings() {
        // Test all 8 Beneath the Well fairy fountain fairies
        for i in 1..=8 {
            let location_id = format!("mm_beneath_the_well_fairy_fountain_fairy_{}", i);
            let mapping = get_mm_mapping(&location_id);
            assert!(
                mapping.is_some(),
                "Beneath the Well fairy {} should exist",
                i
            );
            let mapping = mapping.unwrap();
            assert!(
                mapping.is_mapped(),
                "Beneath the Well fairy {} should be mapped",
                i
            );
            assert_eq!(
                mapping.flag_type,
                Some(MmFlagType::Collectible),
                "Beneath the Well fairy {} should use Collectible flag type",
                i
            );
            assert_eq!(
                mapping.scene_id,
                Some(mm_scene::BENEATH_THE_WELL),
                "Beneath the Well fairy {} should be in Beneath the Well scene",
                i
            );
            // Each fairy should have a unique bit (power of 2)
            let expected_bit = 1u32 << (i - 1);
            assert_eq!(
                mapping.flag_bit,
                Some(expected_bit),
                "Beneath the Well fairy {} should have flag_bit 0x{:02X}",
                i,
                expected_bit
            );
        }
    }

    #[test]
    fn test_stray_fairy_mappings_by_flag_type() {
        // Verify we have StrayFairy mappings now
        let stray_fairy_mappings: Vec<_> =
            get_mm_mappings_by_flag_type(MmFlagType::StrayFairy).collect();
        assert_eq!(
            stray_fairy_mappings.len(),
            6,
            "Should have 6 StrayFairy mappings (Great Fairy rewards)"
        );

        // Verify all are properly mapped
        for mapping in &stray_fairy_mappings {
            assert!(
                mapping.is_mapped(),
                "All StrayFairy mappings should be complete"
            );
            assert_eq!(
                mapping.scene_id, None,
                "StrayFairy mappings should be global"
            );
        }
    }

    #[test]
    fn test_fairy_mappings_count() {
        // Total fairy mappings: 6 Great Fairy rewards + 1 Clock Town stray + 8 Beneath Well = 15
        let mapped_count = mm_mapped_count();
        assert!(
            mapped_count >= 15,
            "Should have at least 15 mapped locations (stray fairy fountains)"
        );
    }

    // ========================================================================
    // check_mm_location_status tests
    // ========================================================================

    #[test]
    fn test_check_mm_location_status_stub_returns_unknown() {
        // Create a stub mapping (unmapped location)
        let stub = MmFlagMapping::stub("test_unmapped_location");

        // Create a default MmSave
        let mm_save = MmSave::default();

        // Stub mappings should return Unknown
        let status = check_mm_location_status(&stub, &mm_save);
        assert_eq!(
            status,
            CheckStatus::Unknown,
            "Stub mapping should return Unknown status"
        );
    }

    #[test]
    fn test_check_mm_location_status_chest_checked() {
        use crate::mm_save::MmPermanentSceneFlags;

        // Create a mapped chest location for Woodfall Temple
        let mapping = MmFlagMapping::mapped(
            "test_chest_location",
            mm_scene::WOODFALL_TEMPLE,
            MmFlagType::Chest,
            0x01, // bit 0
        );

        // Create MmSave with the chest flag set
        let mut mm_save = MmSave::default();
        // Ensure we have enough scene flag entries
        while mm_save.permanent_scene_flags.len() <= mm_scene::WOODFALL_TEMPLE as usize {
            mm_save
                .permanent_scene_flags
                .push(MmPermanentSceneFlags::default());
        }
        // Set the chest flag bit
        mm_save.permanent_scene_flags[mm_scene::WOODFALL_TEMPLE as usize].chest = 0x01;

        // Status should be Checked
        let status = check_mm_location_status(&mapping, &mm_save);
        assert_eq!(
            status,
            CheckStatus::Checked,
            "Chest with flag set should return Checked status"
        );
    }

    #[test]
    fn test_check_mm_location_status_chest_unchecked() {
        use crate::mm_save::MmPermanentSceneFlags;

        // Create a mapped chest location for Woodfall Temple
        let mapping = MmFlagMapping::mapped(
            "test_chest_location",
            mm_scene::WOODFALL_TEMPLE,
            MmFlagType::Chest,
            0x01, // bit 0
        );

        // Create MmSave with the chest flag NOT set
        let mut mm_save = MmSave::default();
        // Ensure we have enough scene flag entries
        while mm_save.permanent_scene_flags.len() <= mm_scene::WOODFALL_TEMPLE as usize {
            mm_save
                .permanent_scene_flags
                .push(MmPermanentSceneFlags::default());
        }
        // Chest flags are 0 (not set)
        mm_save.permanent_scene_flags[mm_scene::WOODFALL_TEMPLE as usize].chest = 0x00;

        // Status should be Unchecked
        let status = check_mm_location_status(&mapping, &mm_save);
        assert_eq!(
            status,
            CheckStatus::Unchecked,
            "Chest with flag not set should return Unchecked status"
        );
    }

    #[test]
    fn test_check_mm_location_status_out_of_bounds_scene_returns_unknown() {
        // Create a mapped location with an out-of-bounds scene ID
        let mapping = MmFlagMapping::mapped(
            "test_oob_location",
            255, // Very high scene ID that won't exist
            MmFlagType::Chest,
            0x01,
        );

        // Create MmSave with default (small) permanent_scene_flags
        let mm_save = MmSave::default();

        // Out of bounds scene should return Unknown
        let status = check_mm_location_status(&mapping, &mm_save);
        assert_eq!(
            status,
            CheckStatus::Unknown,
            "Out of bounds scene_id should return Unknown status"
        );
    }

    #[test]
    fn test_check_mm_location_status_collectible_checked() {
        use crate::mm_save::MmPermanentSceneFlags;

        // Test collectible flag type as well
        let mapping = MmFlagMapping::mapped(
            "test_collectible_location",
            mm_scene::TERMINA_FIELD,
            MmFlagType::Collectible,
            0x04, // bit 2
        );

        // Create MmSave with the collectible flag set
        let mut mm_save = MmSave::default();
        while mm_save.permanent_scene_flags.len() <= mm_scene::TERMINA_FIELD as usize {
            mm_save
                .permanent_scene_flags
                .push(MmPermanentSceneFlags::default());
        }
        mm_save.permanent_scene_flags[mm_scene::TERMINA_FIELD as usize].collectible = 0x04;

        let status = check_mm_location_status(&mapping, &mm_save);
        assert_eq!(
            status,
            CheckStatus::Checked,
            "Collectible with flag set should return Checked status"
        );
    }

    #[test]
    fn test_check_mm_location_status_switch0_checked() {
        use crate::mm_save::MmPermanentSceneFlags;

        // Test switch0 flag type
        let mapping = MmFlagMapping::mapped(
            "test_switch0_location",
            mm_scene::SNOWHEAD_TEMPLE,
            MmFlagType::Switch0,
            0x08, // bit 3
        );

        let mut mm_save = MmSave::default();
        while mm_save.permanent_scene_flags.len() <= mm_scene::SNOWHEAD_TEMPLE as usize {
            mm_save
                .permanent_scene_flags
                .push(MmPermanentSceneFlags::default());
        }
        mm_save.permanent_scene_flags[mm_scene::SNOWHEAD_TEMPLE as usize].switch0 = 0x08;

        let status = check_mm_location_status(&mapping, &mm_save);
        assert_eq!(
            status,
            CheckStatus::Checked,
            "Switch0 with flag set should return Checked status"
        );
    }

    #[test]
    fn test_check_mm_location_status_global_flag_returns_unknown() {
        // Global flags like EventInf are not yet implemented, should return Unknown
        let mapping = MmFlagMapping::global("test_event_location", MmFlagType::EventInf, 0x01);

        let mm_save = MmSave::default();

        let status = check_mm_location_status(&mapping, &mm_save);
        assert_eq!(
            status,
            CheckStatus::Unknown,
            "Unimplemented global flags should return Unknown status"
        );
    }
}
