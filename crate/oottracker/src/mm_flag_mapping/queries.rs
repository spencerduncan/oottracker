//! Query API for MM location mappings.
//!
//! This module provides functions to query the MM location mappings.

use super::{MmFlagMapping, MmFlagType, MM_LOCATION_IDS, MM_MAPPINGS};

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
