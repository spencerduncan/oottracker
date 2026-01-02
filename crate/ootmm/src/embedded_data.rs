//! Embedded world data loaded at compile time.
//!
//! This module provides access to YAML world data that is embedded into the
//! binary at compile time via the build script. This eliminates the need to
//! load data files from disk at runtime.
//!
//! # Example
//!
//! ```
//! use ootmm::embedded_data;
//! use ootmm::WorldDatabase;
//!
//! // Load all embedded world data into a database
//! let db = embedded_data::load_all_world_data().unwrap();
//! assert!(db.region_count() > 0);
//! ```

use crate::error::Result;
use crate::world_database::WorldDatabase;

// Include the generated code from build.rs
include!(concat!(env!("OUT_DIR"), "/embedded_world_data.rs"));

/// Returns an iterator over all embedded OoT world data files.
///
/// Each item is a tuple of (filename, yaml_content).
pub fn oot_world_data() -> impl Iterator<Item = (&'static str, &'static str)> {
    OOT_WORLD_DATA.iter().copied()
}

/// Returns an iterator over all embedded MM world data files.
///
/// Each item is a tuple of (filename, yaml_content).
pub fn mm_world_data() -> impl Iterator<Item = (&'static str, &'static str)> {
    MM_WORLD_DATA.iter().copied()
}

/// Returns an iterator over all embedded world data files (both OoT and MM).
///
/// Each item is a tuple of (filename, yaml_content).
pub fn all_world_data() -> impl Iterator<Item = (&'static str, &'static str)> {
    ALL_WORLD_DATA.iter().copied()
}

/// Returns the number of embedded OoT world data files.
#[must_use]
pub fn oot_file_count() -> usize {
    OOT_WORLD_DATA.len()
}

/// Returns the number of embedded MM world data files.
#[must_use]
pub fn mm_file_count() -> usize {
    MM_WORLD_DATA.len()
}

/// Returns the total number of embedded world data files.
#[must_use]
pub fn total_file_count() -> usize {
    ALL_WORLD_DATA.len()
}

/// Loads all embedded OoT world data into a new WorldDatabase.
///
/// # Errors
///
/// Returns an error if the YAML data cannot be parsed or contains
/// duplicate region/location IDs.
pub fn load_oot_world_data() -> Result<WorldDatabase> {
    let mut db = WorldDatabase::new();
    for (_filename, content) in oot_world_data() {
        db.load_from_str(content)?;
    }
    Ok(db)
}

/// Loads all embedded MM world data into a new WorldDatabase.
///
/// # Errors
///
/// Returns an error if the YAML data cannot be parsed or contains
/// duplicate region/location IDs.
pub fn load_mm_world_data() -> Result<WorldDatabase> {
    let mut db = WorldDatabase::new();
    for (_filename, content) in mm_world_data() {
        db.load_from_str(content)?;
    }
    Ok(db)
}

/// Loads all embedded world data (both OoT and MM) into a new WorldDatabase.
///
/// # Errors
///
/// Returns an error if the YAML data cannot be parsed or contains
/// duplicate region/location IDs.
pub fn load_all_world_data() -> Result<WorldDatabase> {
    let mut db = WorldDatabase::new();
    for (_filename, content) in all_world_data() {
        db.load_from_str(content)?;
    }
    Ok(db)
}

/// Loads all embedded world data into an existing WorldDatabase.
///
/// # Errors
///
/// Returns an error if the YAML data cannot be parsed or contains
/// duplicate region/location IDs.
pub fn load_into_database(db: &mut WorldDatabase) -> Result<()> {
    for (_filename, content) in all_world_data() {
        db.load_from_str(content)?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::region::Game;

    #[test]
    fn test_oot_world_data_not_empty() {
        assert!(oot_file_count() > 0, "No OoT world data files embedded");
    }

    #[test]
    fn test_mm_world_data_not_empty() {
        assert!(mm_file_count() > 0, "No MM world data files embedded");
    }

    #[test]
    fn test_total_file_count() {
        assert_eq!(
            total_file_count(),
            oot_file_count() + mm_file_count(),
            "Total file count should equal OoT + MM counts"
        );
    }

    #[test]
    fn test_load_oot_world_data() {
        let db = load_oot_world_data().expect("Failed to load OoT world data");
        assert!(db.region_count() > 0, "No OoT regions loaded");

        // All regions should be OoT
        for region in db.regions() {
            assert_eq!(region.game, Game::Oot);
        }
    }

    #[test]
    fn test_load_mm_world_data() {
        let db = load_mm_world_data().expect("Failed to load MM world data");
        assert!(db.region_count() > 0, "No MM regions loaded");

        // All regions should be MM
        for region in db.regions() {
            assert_eq!(region.game, Game::Mm);
        }
    }

    #[test]
    fn test_load_all_world_data() {
        let db = load_all_world_data().expect("Failed to load all world data");
        assert!(db.region_count() > 0, "No regions loaded");

        // Should have both OoT and MM regions
        let oot_count = db.regions_for_game(Game::Oot).count();
        let mm_count = db.regions_for_game(Game::Mm).count();
        assert!(oot_count > 0, "No OoT regions in combined database");
        assert!(mm_count > 0, "No MM regions in combined database");
    }

    #[test]
    fn test_load_into_existing_database() {
        let mut db = WorldDatabase::new();
        load_into_database(&mut db).expect("Failed to load into database");
        assert!(db.region_count() > 0);
    }

    #[test]
    fn test_embedded_data_has_locations() {
        let db = load_all_world_data().expect("Failed to load world data");
        assert!(db.location_count() > 0, "No locations in embedded data");
    }

    #[test]
    fn test_embedded_data_has_exits() {
        let db = load_all_world_data().expect("Failed to load world data");
        assert!(db.exit_count() > 0, "No exits in embedded data");
    }

    #[test]
    fn test_specific_oot_region_exists() {
        let db = load_oot_world_data().expect("Failed to load OoT world data");
        assert!(
            db.has_region("kokiri_forest"),
            "Kokiri Forest region not found"
        );
    }

    #[test]
    fn test_specific_mm_region_exists() {
        let db = load_mm_world_data().expect("Failed to load MM world data");
        assert!(
            db.has_region("clock_town_south"),
            "South Clock Town region not found"
        );
    }

    #[test]
    fn test_world_data_iterator_yields_valid_yaml() {
        for (filename, content) in all_world_data() {
            assert!(!filename.is_empty(), "Empty filename in embedded data");
            assert!(!content.is_empty(), "Empty content for file {}", filename);
            assert!(
                content.contains("regions:"),
                "File {} doesn't contain 'regions:' key",
                filename
            );
        }
    }
}
