//! Region, location, and exit types for world graph representation.

use std::collections::HashMap;

use serde::Deserialize;

/// Which game a region belongs to.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Game {
    /// Ocarina of Time
    Oot,
    /// Majora's Mask
    Mm,
}

/// A game region/area containing locations and exits.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Region {
    /// Unique identifier for this region.
    pub id: String,
    /// Human-readable name.
    pub name: String,
    /// Which game this region belongs to.
    pub game: Game,
    /// Locations (item checks) within this region.
    #[serde(default)]
    pub locations: Vec<Location>,
    /// Exits leading to other regions.
    #[serde(default)]
    pub exits: Vec<Exit>,
    /// Events that can be triggered in this region.
    #[serde(default)]
    pub events: Vec<Event>,
}

impl Region {
    /// Create a new empty region.
    #[must_use]
    pub fn new(id: impl Into<String>, name: impl Into<String>, game: Game) -> Self {
        Self {
            id: id.into(),
            name: name.into(),
            game,
            locations: Vec::new(),
            exits: Vec::new(),
            events: Vec::new(),
        }
    }

    /// Add a location to this region.
    pub fn add_location(&mut self, location: Location) {
        self.locations.push(location);
    }

    /// Add an exit to this region.
    pub fn add_exit(&mut self, exit: Exit) {
        self.exits.push(exit);
    }

    /// Add an event to this region.
    pub fn add_event(&mut self, event: Event) {
        self.events.push(event);
    }
}

/// A specific location (item check) within a region.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Location {
    /// Unique identifier for this location.
    pub id: String,
    /// Human-readable name.
    pub name: String,
    /// The logic expression required to access this location (unparsed).
    #[serde(default)]
    pub logic: Option<String>,
    /// Type of location (chest, freestanding, NPC, etc.).
    #[serde(default)]
    pub location_type: LocationType,
}

impl Location {
    /// Create a new location.
    #[must_use]
    pub fn new(
        id: impl Into<String>,
        name: impl Into<String>,
        location_type: LocationType,
    ) -> Self {
        Self {
            id: id.into(),
            name: name.into(),
            logic: None,
            location_type,
        }
    }

    /// Set the logic expression for this location.
    #[must_use]
    pub fn with_logic(mut self, logic: impl Into<String>) -> Self {
        self.logic = Some(logic.into());
        self
    }
}

/// Types of locations.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum LocationType {
    /// Chest containing an item.
    #[default]
    Chest,
    /// Freestanding item.
    Freestanding,
    /// Item given by an NPC.
    Npc,
    /// Item from a minigame or event.
    Event,
    /// Song learned at a location.
    Song,
    /// Collectible (skulltula, heart piece, etc.).
    Collectible,
    /// Shop item.
    Shop,
    /// Scrub/merchant sale.
    Scrub,
    /// Gossip stone hint.
    GossipStone,
    /// Boss reward.
    Boss,
    /// Cow (gives milk bottle).
    Cow,
    /// Fishing pond.
    Fishing,
    /// Great Fairy fountain.
    Fairy,
}

/// An exit connecting one region to another.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Exit {
    /// The target region ID.
    pub target: String,
    /// The logic expression required to use this exit (unparsed).
    #[serde(default)]
    pub logic: Option<String>,
    /// Type of exit.
    #[serde(default)]
    pub exit_type: ExitType,
}

impl Exit {
    /// Create a new exit to a target region.
    #[must_use]
    pub fn new(target: impl Into<String>, exit_type: ExitType) -> Self {
        Self {
            target: target.into(),
            logic: None,
            exit_type,
        }
    }

    /// Set the logic expression for this exit.
    #[must_use]
    pub fn with_logic(mut self, logic: impl Into<String>) -> Self {
        self.logic = Some(logic.into());
        self
    }
}

/// Types of exits between regions.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum ExitType {
    /// Normal walking/climbing connection.
    #[default]
    Normal,
    /// One-way drop or ledge.
    OneWay,
    /// Door requiring a key.
    LockedDoor,
    /// Warp song destination.
    Warp,
    /// Owl flight.
    Owl,
    /// Entrance to a dungeon.
    Dungeon,
    /// Entrance to a grotto/cave.
    Grotto,
    /// Interior door.
    Interior,
    /// Overworld connection.
    Overworld,
}

/// An event that can be triggered in a region.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Event {
    /// Unique identifier for this event.
    pub id: String,
    /// Human-readable name.
    pub name: String,
    /// The logic expression required to trigger this event (unparsed).
    #[serde(default)]
    pub logic: Option<String>,
}

impl Event {
    /// Create a new event.
    #[must_use]
    pub fn new(id: impl Into<String>, name: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            name: name.into(),
            logic: None,
        }
    }

    /// Set the logic expression for this event.
    #[must_use]
    pub fn with_logic(mut self, logic: impl Into<String>) -> Self {
        self.logic = Some(logic.into());
        self
    }
}

/// A world containing all regions for both games.
#[derive(Debug, Clone, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct World {
    /// All regions indexed by ID.
    #[serde(default)]
    pub regions: HashMap<String, Region>,
}

impl World {
    /// Create a new empty world.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Add a region to the world.
    pub fn add_region(&mut self, region: Region) {
        self.regions.insert(region.id.clone(), region);
    }

    /// Get a region by ID.
    #[must_use]
    pub fn get_region(&self, id: &str) -> Option<&Region> {
        self.regions.get(id)
    }

    /// Get a mutable region by ID.
    pub fn get_region_mut(&mut self, id: &str) -> Option<&mut Region> {
        self.regions.get_mut(id)
    }

    /// Get all regions for a specific game.
    pub fn regions_for_game(&self, game: Game) -> impl Iterator<Item = &Region> {
        self.regions.values().filter(move |r| r.game == game)
    }

    /// Count total locations across all regions.
    #[must_use]
    pub fn location_count(&self) -> usize {
        self.regions.values().map(|r| r.locations.len()).sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_region() {
        let region = Region::new("kokiri_forest", "Kokiri Forest", Game::Oot);
        assert_eq!(region.id, "kokiri_forest");
        assert_eq!(region.game, Game::Oot);
        assert!(region.locations.is_empty());
    }

    #[test]
    fn test_add_location() {
        let mut region = Region::new("kokiri_forest", "Kokiri Forest", Game::Oot);
        let location = Location::new("kf_chest", "Kokiri Forest Chest", LocationType::Chest)
            .with_logic("true");
        region.add_location(location);
        assert_eq!(region.locations.len(), 1);
        assert_eq!(region.locations[0].logic, Some("true".to_string()));
    }

    #[test]
    fn test_add_exit() {
        let mut region = Region::new("kokiri_forest", "Kokiri Forest", Game::Oot);
        let exit = Exit::new("lost_woods", ExitType::Overworld).with_logic("true");
        region.add_exit(exit);
        assert_eq!(region.exits.len(), 1);
        assert_eq!(region.exits[0].target, "lost_woods");
    }

    #[test]
    fn test_world() {
        let mut world = World::new();

        let mut kf = Region::new("kokiri_forest", "Kokiri Forest", Game::Oot);
        kf.add_location(Location::new("kf_chest", "Chest", LocationType::Chest));
        kf.add_location(Location::new(
            "kf_sword",
            "Sword",
            LocationType::Freestanding,
        ));

        let clock_town = Region::new("clock_town", "Clock Town", Game::Mm);

        world.add_region(kf);
        world.add_region(clock_town);

        assert_eq!(world.location_count(), 2);
        assert_eq!(world.regions_for_game(Game::Oot).count(), 1);
        assert_eq!(world.regions_for_game(Game::Mm).count(), 1);
    }

    #[test]
    fn test_event() {
        let event = Event::new("deku_tree_clear", "Deku Tree Cleared")
            .with_logic("has(KokiriSword) && has(DekuShield)");
        assert_eq!(event.id, "deku_tree_clear");
        assert!(event.logic.is_some());
    }

    #[test]
    fn test_deserialize_region() {
        let yaml = r#"
id: "test_region"
name: "Test Region"
game: oot
locations:
  - id: "test_loc"
    name: "Test Location"
"#;
        let region: Region = serde_yaml::from_str(yaml).unwrap();
        assert_eq!(region.id, "test_region");
        assert_eq!(region.name, "Test Region");
        assert_eq!(region.game, Game::Oot);
        assert_eq!(region.locations.len(), 1);
        assert_eq!(region.locations[0].name, "Test Location");
        assert!(region.exits.is_empty());
        assert!(region.events.is_empty());
    }

    #[test]
    fn test_deserialize_region_with_exits() {
        let yaml = r#"
id: "kokiri_forest"
name: "Kokiri Forest"
game: oot
exits:
  - target: "lost_woods"
    exitType: overworld
  - target: "deku_tree"
    exitType: dungeon
    logic: "has(KokiriSword)"
"#;
        let region: Region = serde_yaml::from_str(yaml).unwrap();
        assert_eq!(region.exits.len(), 2);
        assert_eq!(region.exits[0].target, "lost_woods");
        assert_eq!(region.exits[0].exit_type, ExitType::Overworld);
        assert_eq!(region.exits[1].exit_type, ExitType::Dungeon);
        assert_eq!(region.exits[1].logic, Some("has(KokiriSword)".to_string()));
    }

    #[test]
    fn test_deserialize_location_types() {
        let yaml = r#"
id: "test"
name: "Test"
game: mm
locations:
  - id: "chest1"
    name: "Chest"
    locationType: chest
  - id: "npc1"
    name: "NPC"
    locationType: npc
  - id: "gs1"
    name: "Gossip Stone"
    locationType: gossipStone
"#;
        let region: Region = serde_yaml::from_str(yaml).unwrap();
        assert_eq!(region.locations[0].location_type, LocationType::Chest);
        assert_eq!(region.locations[1].location_type, LocationType::Npc);
        assert_eq!(region.locations[2].location_type, LocationType::GossipStone);
    }
}
