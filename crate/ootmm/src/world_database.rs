//! World database for loading and indexing region/location data from YAML files.

use std::collections::HashMap;
use std::fs;
use std::path::Path;

use serde::Deserialize;

use crate::error::{Error, Result};
use crate::region::{Event, Exit, Game, Location, Region};

/// A reference to a location within the world database.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LocationRef {
    /// The region ID containing this location.
    pub region_id: String,
    /// The index of this location within the region's location list.
    pub location_index: usize,
}

/// A reference to an event within the world database.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EventRef {
    /// The region ID containing this event.
    pub region_id: String,
    /// The index of this event within the region's event list.
    pub event_index: usize,
}

/// YAML structure for a world file containing multiple regions.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
struct WorldFile {
    /// List of regions in this file.
    #[serde(default)]
    regions: Vec<Region>,
}

/// A database for loading, storing, and querying world data.
///
/// The `WorldDatabase` provides efficient lookups for regions, locations,
/// and events loaded from YAML files. It maintains indices for quick access
/// by ID.
///
/// # Example
///
/// ```
/// use ootmm::world_database::WorldDatabase;
///
/// let yaml = r#"
/// regions:
///   - id: "kokiri_forest"
///     name: "Kokiri Forest"
///     game: oot
///     locations:
///       - id: "kf_chest"
///         name: "Kokiri Forest Chest"
/// "#;
///
/// let mut db = WorldDatabase::new();
/// db.load_from_str(yaml).unwrap();
///
/// assert!(db.get_region("kokiri_forest").is_some());
/// assert!(db.get_location("kf_chest").is_some());
/// ```
#[derive(Debug, Clone, Default)]
pub struct WorldDatabase {
    /// All regions indexed by their unique ID.
    regions: HashMap<String, Region>,
    /// Location index mapping location ID to (region_id, location_index).
    location_index: HashMap<String, LocationRef>,
    /// Event index mapping event ID to (region_id, event_index).
    event_index: HashMap<String, EventRef>,
}

impl WorldDatabase {
    /// Creates a new empty world database.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Loads regions from a YAML string.
    ///
    /// The YAML should have a `regions` array containing region definitions.
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - The YAML cannot be parsed
    /// - A duplicate region ID is encountered
    /// - A duplicate location ID is encountered
    pub fn load_from_str(&mut self, yaml: &str) -> Result<()> {
        let world_file: WorldFile = serde_yaml::from_str(yaml)?;
        self.add_regions(world_file.regions)
    }

    /// Loads regions from a YAML file.
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - The file cannot be read
    /// - The YAML cannot be parsed
    /// - A duplicate region or location ID is encountered
    pub fn load_from_file<P: AsRef<Path>>(&mut self, path: P) -> Result<()> {
        let content = fs::read_to_string(path.as_ref())?;
        self.load_from_str(&content)
    }

    /// Loads regions from all YAML files in a directory.
    ///
    /// This method reads all `.yaml` and `.yml` files in the specified directory
    /// and loads their region definitions.
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - The directory cannot be read
    /// - Any file cannot be read or parsed
    /// - Duplicate region or location IDs are encountered
    pub fn load_from_directory<P: AsRef<Path>>(&mut self, dir: P) -> Result<()> {
        let dir = dir.as_ref();

        let entries = fs::read_dir(dir).map_err(|e| {
            Error::world_load_with_source(format!("failed to read directory: {}", dir.display()), e)
        })?;

        for entry in entries {
            let entry = entry
                .map_err(|e| Error::world_load_with_source("failed to read directory entry", e))?;

            let path = entry.path();
            if let Some(ext) = path.extension() {
                if ext == "yaml" || ext == "yml" {
                    self.load_from_file(&path)?;
                }
            }
        }

        Ok(())
    }

    /// Adds a single region to the database.
    ///
    /// # Errors
    ///
    /// Returns an error if a region with the same ID already exists.
    pub fn add_region(&mut self, region: Region) -> Result<()> {
        if self.regions.contains_key(&region.id) {
            return Err(Error::world_load(format!(
                "duplicate region ID: {}",
                region.id
            )));
        }

        // Index all locations in this region
        for (idx, location) in region.locations.iter().enumerate() {
            if self.location_index.contains_key(&location.id) {
                return Err(Error::world_load(format!(
                    "duplicate location ID: {}",
                    location.id
                )));
            }
            self.location_index.insert(
                location.id.clone(),
                LocationRef {
                    region_id: region.id.clone(),
                    location_index: idx,
                },
            );
        }

        // Index all events in this region
        for (idx, event) in region.events.iter().enumerate() {
            if self.event_index.contains_key(&event.id) {
                return Err(Error::world_load(format!(
                    "duplicate event ID: {}",
                    event.id
                )));
            }
            self.event_index.insert(
                event.id.clone(),
                EventRef {
                    region_id: region.id.clone(),
                    event_index: idx,
                },
            );
        }

        self.regions.insert(region.id.clone(), region);
        Ok(())
    }

    /// Adds multiple regions to the database.
    ///
    /// # Errors
    ///
    /// Returns an error if any region or location has a duplicate ID.
    pub fn add_regions(&mut self, regions: Vec<Region>) -> Result<()> {
        for region in regions {
            self.add_region(region)?;
        }
        Ok(())
    }

    /// Gets a region by its ID.
    #[must_use]
    pub fn get_region(&self, id: &str) -> Option<&Region> {
        self.regions.get(id)
    }

    /// Gets a mutable region by its ID.
    pub fn get_region_mut(&mut self, id: &str) -> Option<&mut Region> {
        self.regions.get_mut(id)
    }

    /// Gets a location by its ID.
    ///
    /// Returns the location along with its containing region ID.
    #[must_use]
    pub fn get_location(&self, id: &str) -> Option<(&Location, &str)> {
        let location_ref = self.location_index.get(id)?;
        let region = self.regions.get(&location_ref.region_id)?;
        let location = region.locations.get(location_ref.location_index)?;
        Some((location, &location_ref.region_id))
    }

    /// Gets an event by its ID.
    ///
    /// Returns the event along with its containing region ID.
    #[must_use]
    pub fn get_event(&self, id: &str) -> Option<(&Event, &str)> {
        let event_ref = self.event_index.get(id)?;
        let region = self.regions.get(&event_ref.region_id)?;
        let event = region.events.get(event_ref.event_index)?;
        Some((event, &event_ref.region_id))
    }

    /// Returns an iterator over all regions.
    pub fn regions(&self) -> impl Iterator<Item = &Region> {
        self.regions.values()
    }

    /// Returns an iterator over all regions for a specific game.
    pub fn regions_for_game(&self, game: Game) -> impl Iterator<Item = &Region> {
        self.regions.values().filter(move |r| r.game == game)
    }

    /// Returns an iterator over all locations across all regions.
    pub fn locations(&self) -> impl Iterator<Item = (&Location, &str)> {
        self.regions
            .values()
            .flat_map(|r| r.locations.iter().map(move |l| (l, r.id.as_str())))
    }

    /// Returns an iterator over all locations for a specific game.
    pub fn locations_for_game(&self, game: Game) -> impl Iterator<Item = (&Location, &str)> {
        self.regions
            .values()
            .filter(move |r| r.game == game)
            .flat_map(|r| r.locations.iter().map(move |l| (l, r.id.as_str())))
    }

    /// Returns an iterator over all events across all regions.
    pub fn events(&self) -> impl Iterator<Item = (&Event, &str)> {
        self.regions
            .values()
            .flat_map(|r| r.events.iter().map(move |e| (e, r.id.as_str())))
    }

    /// Returns an iterator over all exits across all regions.
    pub fn exits(&self) -> impl Iterator<Item = (&Exit, &str)> {
        self.regions
            .values()
            .flat_map(|r| r.exits.iter().map(move |e| (e, r.id.as_str())))
    }

    /// Returns the total number of regions.
    #[must_use]
    pub fn region_count(&self) -> usize {
        self.regions.len()
    }

    /// Returns the total number of locations across all regions.
    #[must_use]
    pub fn location_count(&self) -> usize {
        self.location_index.len()
    }

    /// Returns the total number of events across all regions.
    #[must_use]
    pub fn event_count(&self) -> usize {
        self.event_index.len()
    }

    /// Returns the total number of exits across all regions.
    #[must_use]
    pub fn exit_count(&self) -> usize {
        self.regions.values().map(|r| r.exits.len()).sum()
    }

    /// Checks if the database contains a region with the given ID.
    #[must_use]
    pub fn has_region(&self, id: &str) -> bool {
        self.regions.contains_key(id)
    }

    /// Checks if the database contains a location with the given ID.
    #[must_use]
    pub fn has_location(&self, id: &str) -> bool {
        self.location_index.contains_key(id)
    }

    /// Checks if the database contains an event with the given ID.
    #[must_use]
    pub fn has_event(&self, id: &str) -> bool {
        self.event_index.contains_key(id)
    }

    /// Clears all data from the database.
    pub fn clear(&mut self) {
        self.regions.clear();
        self.location_index.clear();
        self.event_index.clear();
    }

    /// Validates the database for consistency.
    ///
    /// Checks that all exit targets reference valid regions.
    ///
    /// # Errors
    ///
    /// Returns an error if any exit targets an unknown region.
    pub fn validate(&self) -> Result<()> {
        for region in self.regions.values() {
            for exit in &region.exits {
                if !self.regions.contains_key(&exit.target) {
                    return Err(Error::world_load(format!(
                        "exit in region '{}' references unknown region '{}'",
                        region.id, exit.target
                    )));
                }
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::region::{ExitType, LocationType};

    fn sample_yaml() -> &'static str {
        r#"
regions:
  - id: "kokiri_forest"
    name: "Kokiri Forest"
    game: oot
    locations:
      - id: "kf_midos_chest"
        name: "Mido's House Chest"
        locationType: chest
      - id: "kf_sword"
        name: "Kokiri Sword Chest"
        locationType: chest
        logic: "true"
    exits:
      - target: "lost_woods"
        exitType: overworld
      - target: "deku_tree"
        exitType: dungeon
        logic: "has(DekuShield)"
    events:
      - id: "kf_showed_mido_sword"
        name: "Showed Mido Sword and Shield"
        logic: "has(KokiriSword) && has(DekuShield)"
  - id: "lost_woods"
    name: "Lost Woods"
    game: oot
    locations:
      - id: "lw_skull_kid"
        name: "Skull Kid Gift"
        locationType: npc
    exits:
      - target: "kokiri_forest"
        exitType: overworld
  - id: "deku_tree"
    name: "Inside the Deku Tree"
    game: oot
"#
    }

    #[test]
    fn test_new_database_is_empty() {
        let db = WorldDatabase::new();
        assert_eq!(db.region_count(), 0);
        assert_eq!(db.location_count(), 0);
        assert_eq!(db.event_count(), 0);
    }

    #[test]
    fn test_load_from_str() {
        let mut db = WorldDatabase::new();
        db.load_from_str(sample_yaml()).unwrap();

        assert_eq!(db.region_count(), 3);
        assert_eq!(db.location_count(), 3);
        assert_eq!(db.event_count(), 1);
        assert_eq!(db.exit_count(), 3);
    }

    #[test]
    fn test_get_region() {
        let mut db = WorldDatabase::new();
        db.load_from_str(sample_yaml()).unwrap();

        let region = db.get_region("kokiri_forest").unwrap();
        assert_eq!(region.name, "Kokiri Forest");
        assert_eq!(region.game, Game::Oot);
        assert_eq!(region.locations.len(), 2);
        assert_eq!(region.exits.len(), 2);
    }

    #[test]
    fn test_get_location() {
        let mut db = WorldDatabase::new();
        db.load_from_str(sample_yaml()).unwrap();

        let (location, region_id) = db.get_location("kf_midos_chest").unwrap();
        assert_eq!(location.name, "Mido's House Chest");
        assert_eq!(location.location_type, LocationType::Chest);
        assert_eq!(region_id, "kokiri_forest");

        let (location2, region_id2) = db.get_location("lw_skull_kid").unwrap();
        assert_eq!(location2.name, "Skull Kid Gift");
        assert_eq!(region_id2, "lost_woods");
    }

    #[test]
    fn test_get_event() {
        let mut db = WorldDatabase::new();
        db.load_from_str(sample_yaml()).unwrap();

        let (event, region_id) = db.get_event("kf_showed_mido_sword").unwrap();
        assert_eq!(event.name, "Showed Mido Sword and Shield");
        assert_eq!(region_id, "kokiri_forest");
        assert!(event.logic.is_some());
    }

    #[test]
    fn test_has_methods() {
        let mut db = WorldDatabase::new();
        db.load_from_str(sample_yaml()).unwrap();

        assert!(db.has_region("kokiri_forest"));
        assert!(db.has_region("lost_woods"));
        assert!(!db.has_region("nonexistent"));

        assert!(db.has_location("kf_sword"));
        assert!(!db.has_location("nonexistent"));

        assert!(db.has_event("kf_showed_mido_sword"));
        assert!(!db.has_event("nonexistent"));
    }

    #[test]
    fn test_regions_iterator() {
        let mut db = WorldDatabase::new();
        db.load_from_str(sample_yaml()).unwrap();

        let region_ids: Vec<_> = db.regions().map(|r| r.id.as_str()).collect();
        assert_eq!(region_ids.len(), 3);
        assert!(region_ids.contains(&"kokiri_forest"));
        assert!(region_ids.contains(&"lost_woods"));
        assert!(region_ids.contains(&"deku_tree"));
    }

    #[test]
    fn test_regions_for_game() {
        let mut db = WorldDatabase::new();
        let yaml = r#"
regions:
  - id: "kf"
    name: "Kokiri Forest"
    game: oot
  - id: "ct"
    name: "Clock Town"
    game: mm
"#;
        db.load_from_str(yaml).unwrap();

        let oot_regions: Vec<_> = db.regions_for_game(Game::Oot).collect();
        assert_eq!(oot_regions.len(), 1);
        assert_eq!(oot_regions[0].id, "kf");

        let mm_regions: Vec<_> = db.regions_for_game(Game::Mm).collect();
        assert_eq!(mm_regions.len(), 1);
        assert_eq!(mm_regions[0].id, "ct");
    }

    #[test]
    fn test_locations_iterator() {
        let mut db = WorldDatabase::new();
        db.load_from_str(sample_yaml()).unwrap();

        let locations: Vec<_> = db.locations().collect();
        assert_eq!(locations.len(), 3);
    }

    #[test]
    fn test_exits_iterator() {
        let mut db = WorldDatabase::new();
        db.load_from_str(sample_yaml()).unwrap();

        let exits: Vec<_> = db.exits().collect();
        assert_eq!(exits.len(), 3);

        // Check that we have expected exit types
        let has_dungeon_exit = exits.iter().any(|(e, _)| e.exit_type == ExitType::Dungeon);
        assert!(has_dungeon_exit);
    }

    #[test]
    fn test_duplicate_region_id_error() {
        let mut db = WorldDatabase::new();
        let yaml = r#"
regions:
  - id: "same_id"
    name: "First Region"
    game: oot
  - id: "same_id"
    name: "Second Region"
    game: oot
"#;
        let result = db.load_from_str(yaml);
        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("duplicate region ID"));
    }

    #[test]
    fn test_duplicate_location_id_error() {
        let mut db = WorldDatabase::new();
        let yaml = r#"
regions:
  - id: "region1"
    name: "Region One"
    game: oot
    locations:
      - id: "same_loc"
        name: "Location 1"
  - id: "region2"
    name: "Region Two"
    game: oot
    locations:
      - id: "same_loc"
        name: "Location 2"
"#;
        let result = db.load_from_str(yaml);
        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("duplicate location ID"));
    }

    #[test]
    fn test_validate_success() {
        let mut db = WorldDatabase::new();
        db.load_from_str(sample_yaml()).unwrap();
        assert!(db.validate().is_ok());
    }

    #[test]
    fn test_validate_invalid_exit_target() {
        let mut db = WorldDatabase::new();
        let yaml = r#"
regions:
  - id: "region1"
    name: "Region One"
    game: oot
    exits:
      - target: "nonexistent_region"
"#;
        db.load_from_str(yaml).unwrap();
        let result = db.validate();
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("unknown region"));
    }

    #[test]
    fn test_clear() {
        let mut db = WorldDatabase::new();
        db.load_from_str(sample_yaml()).unwrap();

        assert!(db.region_count() > 0);
        db.clear();
        assert_eq!(db.region_count(), 0);
        assert_eq!(db.location_count(), 0);
        assert_eq!(db.event_count(), 0);
    }

    #[test]
    fn test_add_region_directly() {
        let mut db = WorldDatabase::new();

        let mut region = Region::new("test_region", "Test Region", Game::Oot);
        region.add_location(Location::new(
            "test_loc",
            "Test Location",
            LocationType::Chest,
        ));

        db.add_region(region).unwrap();

        assert!(db.has_region("test_region"));
        assert!(db.has_location("test_loc"));
    }

    #[test]
    fn test_get_region_mut() {
        let mut db = WorldDatabase::new();
        db.load_from_str(sample_yaml()).unwrap();

        let region = db.get_region_mut("kokiri_forest").unwrap();
        region.add_location(Location::new("new_loc", "New Location", LocationType::Npc));

        // Note: manually added locations won't be indexed
        let region = db.get_region("kokiri_forest").unwrap();
        assert_eq!(region.locations.len(), 3);
    }

    #[test]
    fn test_empty_yaml() {
        let mut db = WorldDatabase::new();
        let yaml = "regions: []";
        db.load_from_str(yaml).unwrap();
        assert_eq!(db.region_count(), 0);
    }

    #[test]
    fn test_locations_for_game() {
        let mut db = WorldDatabase::new();
        let yaml = r#"
regions:
  - id: "kf"
    name: "Kokiri Forest"
    game: oot
    locations:
      - id: "kf_loc"
        name: "KF Location"
  - id: "ct"
    name: "Clock Town"
    game: mm
    locations:
      - id: "ct_loc1"
        name: "CT Location 1"
      - id: "ct_loc2"
        name: "CT Location 2"
"#;
        db.load_from_str(yaml).unwrap();

        let oot_locs: Vec<_> = db.locations_for_game(Game::Oot).collect();
        assert_eq!(oot_locs.len(), 1);
        assert_eq!(oot_locs[0].0.id, "kf_loc");

        let mm_locs: Vec<_> = db.locations_for_game(Game::Mm).collect();
        assert_eq!(mm_locs.len(), 2);
    }
}
