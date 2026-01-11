//! Query functions for flag mappings.

use crate::world_database;

use super::mappings::{OOT_LOCATION_IDS, OOT_MAPPINGS};
use super::types::{FlagMapping, FlagType};

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
