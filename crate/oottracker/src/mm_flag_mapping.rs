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
    // These are derived from OoTMM research and will be populated incrementally

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

        // All should be stubs initially since no mappings are added yet
        let stub_count = mm_stub_count();
        let mapped_count = mm_mapped_count();

        assert_eq!(
            stub_count + mapped_count,
            count,
            "stub + mapped should equal total"
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
        assert!(mm_scene::WOODFALL_TEMPLE < mm_scene::MAX_SCENE_ID);
        assert!(mm_scene::SNOWHEAD_TEMPLE < mm_scene::MAX_SCENE_ID);
        assert!(mm_scene::GREAT_BAY_TEMPLE < mm_scene::MAX_SCENE_ID);
        assert!(mm_scene::STONE_TOWER_TEMPLE < mm_scene::MAX_SCENE_ID);
        assert!(mm_scene::CLOCK_TOWN_SOUTH < mm_scene::MAX_SCENE_ID);
    }

    #[test]
    fn test_get_mm_mappings_by_flag_type() {
        // This should work even if all are stubs (returns empty iterator)
        let chest_mappings: Vec<_> = get_mm_mappings_by_flag_type(MmFlagType::Chest).collect();
        // Initially all are stubs, so this should be empty
        assert!(
            chest_mappings.is_empty() || chest_mappings.iter().all(|m| m.is_mapped()),
            "All returned mappings should be mapped"
        );
    }

    #[test]
    fn test_get_mm_mappings_for_scene() {
        // This should work even if all are stubs (returns empty iterator)
        let woodfall_mappings: Vec<_> =
            get_mm_mappings_for_scene(mm_scene::WOODFALL_TEMPLE).collect();
        // Initially all are stubs with no scene_id, so this should be empty
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
}
