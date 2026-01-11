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

// Submodules
mod checker;
mod locations;
mod mapping;
mod queries;
pub mod scenes;
mod types;

#[cfg(test)]
mod tests;

// Re-exports for public API
pub use checker::{
    check_mm_location_status, get_all_mm_checked_locations, get_all_mm_locations_with_status,
};
pub use mapping::MmFlagMapping;
pub use queries::{
    get_all_mm_location_ids, get_all_mm_mappings, get_mm_mapped_locations, get_mm_mapping,
    get_mm_mappings_by_flag_type, get_mm_mappings_for_scene, get_mm_stub_locations,
    mm_location_count, mm_mapped_count, mm_stub_count,
};
pub use types::MmFlagType;

// For backwards compatibility, re-export mm_scene as a module
pub use scenes as mm_scene;

use std::collections::HashMap;

use once_cell::sync::Lazy;
use ootmm::region::Game;

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
    locations::register_all_locations(&mut map);

    map
});
