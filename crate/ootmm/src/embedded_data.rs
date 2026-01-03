//! Embedded static data module.
//!
//! This module provides access to static data files that are embedded into the binary
//! at compile time using `include_str!`. The data files are processed by `build.rs`.
//!
//! # Example
//!
//! ```
//! use ootmm::embedded_data;
//!
//! // Get all embedded world files
//! for (name, content) in embedded_data::world::all() {
//!     println!("World file: {} ({} bytes)", name, content.len());
//! }
//!
//! // Get a specific world file by name
//! if let Some(yaml) = embedded_data::get_world_data("oot_kokiri") {
//!     println!("Found OoT Kokiri data");
//! }
//! ```

// Include the generated code from build.rs
include!(concat!(env!("OUT_DIR"), "/embedded_data.rs"));

use crate::error::Result;
use crate::world_database::WorldDatabase;

/// Creates a [`WorldDatabase`] pre-loaded with all embedded world data.
///
/// This is a convenience function that loads all embedded YAML world files
/// into a new `WorldDatabase` instance.
///
/// # Errors
///
/// Returns an error if any embedded YAML file fails to parse or contains
/// duplicate region/location IDs.
///
/// # Example
///
/// ```
/// use ootmm::embedded_data;
///
/// let db = embedded_data::create_world_database().expect("Failed to load world data");
/// println!("Loaded {} regions", db.region_count());
/// ```
pub fn create_world_database() -> Result<WorldDatabase> {
    let mut db = WorldDatabase::new();

    for (name, content) in world::all() {
        db.load_from_str(content).map_err(|e| {
            crate::error::Error::world_load(format!(
                "failed to load embedded world '{}': {}",
                name, e
            ))
        })?;
    }

    Ok(db)
}

/// Creates a [`WorldDatabase`] from specific embedded world files.
///
/// This function allows selective loading of world data by name.
///
/// # Arguments
///
/// * `names` - An iterator of world file names (without extension)
///
/// # Errors
///
/// Returns an error if:
/// - A requested world file is not found
/// - Any embedded YAML file fails to parse
/// - Duplicate region/location IDs are encountered
///
/// # Example
///
/// ```
/// use ootmm::embedded_data;
///
/// // Load only OoT data
/// let db = embedded_data::create_world_database_from(["oot_kokiri"])
///     .expect("Failed to load world data");
/// ```
pub fn create_world_database_from<'a>(
    names: impl IntoIterator<Item = &'a str>,
) -> Result<WorldDatabase> {
    let mut db = WorldDatabase::new();

    for name in names {
        let content = get_world_data(name).ok_or_else(|| {
            crate::error::Error::world_load(format!("embedded world '{}' not found", name))
        })?;

        db.load_from_str(content).map_err(|e| {
            crate::error::Error::world_load(format!(
                "failed to load embedded world '{}': {}",
                name, e
            ))
        })?;
    }

    Ok(db)
}

/// Returns a list of all embedded world file names.
///
/// # Example
///
/// ```
/// use ootmm::embedded_data;
///
/// for name in embedded_data::world_names() {
///     println!("Available world: {}", name);
/// }
/// ```
pub fn world_names() -> impl Iterator<Item = &'static str> {
    world::all().iter().map(|(name, _)| *name)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_world_file_count() {
        // We should have at least our 2 test files
        assert!(world_file_count() >= 2);
    }

    #[test]
    fn test_get_world_data_exists() {
        let data = get_world_data("oot_kokiri");
        assert!(data.is_some());
        let content = data.unwrap();
        assert!(content.contains("kokiri_forest"));
        assert!(content.contains("regions:"));
    }

    #[test]
    fn test_get_world_data_not_found() {
        let data = get_world_data("nonexistent_world");
        assert!(data.is_none());
    }

    #[test]
    fn test_world_all() {
        let all = world::all();
        assert!(!all.is_empty());

        // Check that all entries have non-empty content
        for (name, content) in all {
            assert!(!name.is_empty(), "World name should not be empty");
            assert!(!content.is_empty(), "World content should not be empty");
            assert!(
                content.contains("regions:"),
                "World content should contain 'regions:'"
            );
        }
    }

    #[test]
    fn test_world_names() {
        let names: Vec<_> = world_names().collect();
        assert!(!names.is_empty());
        assert!(names.contains(&"oot_kokiri"));
        assert!(names.contains(&"mm_clock_town"));
    }

    #[test]
    fn test_create_world_database() {
        let db = create_world_database().expect("Failed to create world database");
        // Embedded data has 4 regions: kokiri_forest, lost_woods (OoT) + clock_town_south, termina_field (MM)
        assert_eq!(db.region_count(), 4, "Embedded data has 4 regions");
        assert!(db.has_region("kokiri_forest"));
        assert!(db.has_region("clock_town_south"));
    }

    #[test]
    fn test_create_world_database_from_specific() {
        let db =
            create_world_database_from(["oot_kokiri"]).expect("Failed to create world database");

        // Should have OoT regions
        assert!(db.has_region("kokiri_forest"));

        // Should NOT have MM regions
        assert!(!db.has_region("clock_town_south"));
    }

    #[test]
    fn test_create_world_database_from_not_found() {
        let result = create_world_database_from(["nonexistent"]);
        assert!(result.is_err());
    }

    #[test]
    fn test_embedded_oot_data_content() {
        let content = world::OOT_KOKIRI;
        assert!(content.contains("kokiri_forest"));
        assert!(content.contains("lost_woods"));
        assert!(content.contains("kf_midos_chest_top_left"));
    }

    #[test]
    fn test_embedded_mm_data_content() {
        let content = world::MM_CLOCK_TOWN;
        assert!(content.contains("clock_town_south"));
        assert!(content.contains("termina_field"));
        assert!(content.contains("ct_bank_reward_1"));
    }
}
