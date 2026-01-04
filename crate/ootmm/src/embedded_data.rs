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
//! if let Some(yaml) = embedded_data::get_world_data("oot_overworld") {
//!     println!("Found OoT overworld data");
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
/// // Load only OoT overworld data
/// let db = embedded_data::create_world_database_from(["oot_overworld"])
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
        // We should have all our comprehensive world files
        assert!(
            world_file_count() >= 8,
            "Expected at least 8 world files, got {}",
            world_file_count()
        );
    }

    #[test]
    fn test_get_world_data_exists() {
        let data = get_world_data("oot_overworld");
        assert!(data.is_some());
        let content = data.unwrap();
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
        assert!(names.contains(&"oot_overworld"));
        assert!(names.contains(&"oot_dungeons"));
        assert!(names.contains(&"mm_overworld"));
        assert!(names.contains(&"mm_dungeons"));
    }

    #[test]
    fn test_create_world_database() {
        let db = create_world_database().expect("Failed to create world database");
        // Comprehensive import has 1000+ regions from both games
        assert!(
            db.region_count() > 1000,
            "Expected 1000+ regions, got {}",
            db.region_count()
        );
        // Check some representative regions from OoT overworld (prefixed with oot_)
        assert!(
            db.has_region("oot_kokiri_forest"),
            "Missing oot_kokiri_forest"
        );
        assert!(db.has_region("oot_lost_woods"), "Missing oot_lost_woods");
        assert!(
            db.has_region("oot_death_mountain"),
            "Missing oot_death_mountain"
        );
        // Check some representative regions from MM overworld (prefixed with mm_)
        assert!(
            db.has_region("mm_clock_town_south"),
            "Missing mm_clock_town_south"
        );
        assert!(
            db.has_region("mm_termina_field"),
            "Missing mm_termina_field"
        );
    }

    #[test]
    fn test_create_world_database_from_specific() {
        let db =
            create_world_database_from(["oot_overworld"]).expect("Failed to create world database");

        // Should have OoT overworld regions (prefixed with oot_)
        assert!(db.has_region("oot_kokiri_forest"));
        assert!(db.has_region("oot_lost_woods"));

        // Should NOT have MM regions (from mm_overworld)
        assert!(!db.has_region("mm_clock_town_south"));
    }

    #[test]
    fn test_create_world_database_from_not_found() {
        let result = create_world_database_from(["nonexistent"]);
        assert!(result.is_err());
    }

    #[test]
    fn test_embedded_oot_overworld_content() {
        let content = world::OOT_OVERWORLD;
        assert!(content.contains("oot_kokiri_forest"));
        assert!(content.contains("oot_lost_woods"));
        assert!(content.contains("oot_death_mountain"));
    }

    #[test]
    fn test_embedded_oot_dungeons_content() {
        let content = world::OOT_DUNGEONS;
        assert!(content.contains("oot_deku_tree"));
        assert!(content.contains("oot_dodongo"));
        assert!(content.contains("oot_forest_temple"));
    }

    #[test]
    fn test_embedded_mm_overworld_content() {
        let content = world::MM_OVERWORLD;
        assert!(content.contains("mm_clock_town_south"));
        assert!(content.contains("mm_termina_field"));
        assert!(content.contains("mm_clock_town_north"));
    }

    #[test]
    fn test_embedded_mm_dungeons_content() {
        let content = world::MM_DUNGEONS;
        assert!(content.contains("mm_woodfall_temple"));
        assert!(content.contains("mm_snowhead_temple"));
        assert!(content.contains("mm_great_bay_temple"));
    }

    #[test]
    fn test_region_count_by_game() {
        let db = create_world_database().expect("Failed to create world database");

        let oot_count = db.regions_for_game(crate::region::Game::Oot).count();
        let mm_count = db.regions_for_game(crate::region::Game::Mm).count();

        assert!(
            oot_count > 400,
            "Expected 400+ OoT regions, got {}",
            oot_count
        );
        assert!(mm_count > 400, "Expected 400+ MM regions, got {}", mm_count);
    }

    #[test]
    fn test_location_count() {
        let db = create_world_database().expect("Failed to create world database");

        let loc_count = db.location_count();
        assert!(
            loc_count > 2000,
            "Expected 2000+ locations, got {}",
            loc_count
        );
    }
}
