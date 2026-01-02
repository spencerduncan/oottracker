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

    #[test]
    fn test_game_enum_deserialization() {
        let yaml_oot = r#"
id: "test_oot"
name: "Test OoT"
game: oot
"#;
        let yaml_mm = r#"
id: "test_mm"
name: "Test MM"
game: mm
"#;
        let region_oot: Region = serde_yaml::from_str(yaml_oot).unwrap();
        let region_mm: Region = serde_yaml::from_str(yaml_mm).unwrap();
        assert_eq!(region_oot.game, Game::Oot);
        assert_eq!(region_mm.game, Game::Mm);
    }

    #[test]
    fn test_location_default_type() {
        // LocationType should default to Chest when not specified
        let yaml = r#"
id: "test"
name: "Test"
game: oot
locations:
  - id: "loc1"
    name: "Location Without Type"
"#;
        let region: Region = serde_yaml::from_str(yaml).unwrap();
        assert_eq!(region.locations[0].location_type, LocationType::Chest);
    }

    #[test]
    fn test_exit_default_type() {
        // ExitType should default to Normal when not specified
        let yaml = r#"
id: "test"
name: "Test"
game: oot
exits:
  - target: "other_region"
"#;
        let region: Region = serde_yaml::from_str(yaml).unwrap();
        assert_eq!(region.exits[0].exit_type, ExitType::Normal);
    }

    #[test]
    fn test_world_get_region() {
        let mut world = World::new();
        let region = Region::new("test_region", "Test Region", Game::Oot);
        world.add_region(region);

        // Test get_region
        let retrieved = world.get_region("test_region");
        assert!(retrieved.is_some());
        assert_eq!(retrieved.unwrap().name, "Test Region");

        // Test non-existent region
        assert!(world.get_region("nonexistent").is_none());
    }

    #[test]
    fn test_world_get_region_mut() {
        let mut world = World::new();
        let region = Region::new("mutable_region", "Mutable Region", Game::Mm);
        world.add_region(region);

        // Test get_region_mut and modify
        {
            let region_mut = world.get_region_mut("mutable_region");
            assert!(region_mut.is_some());
            let r = region_mut.unwrap();
            r.add_location(Location::new(
                "new_loc",
                "New Location",
                LocationType::Chest,
            ));
        }

        // Verify modification persisted
        let region = world.get_region("mutable_region").unwrap();
        assert_eq!(region.locations.len(), 1);
        assert_eq!(region.locations[0].id, "new_loc");
    }

    #[test]
    fn test_region_add_event() {
        let mut region = Region::new("event_region", "Event Region", Game::Oot);
        let event = Event::new("boss_defeated", "Boss Defeated").with_logic("has(BossKey)");
        region.add_event(event);

        assert_eq!(region.events.len(), 1);
        assert_eq!(region.events[0].id, "boss_defeated");
        assert_eq!(region.events[0].name, "Boss Defeated");
        assert_eq!(region.events[0].logic, Some("has(BossKey)".to_string()));
    }

    #[test]
    fn test_deserialize_events() {
        let yaml = r#"
id: "deku_tree"
name: "Deku Tree"
game: oot
events:
  - id: "deku_tree_clear"
    name: "Deku Tree Cleared"
    logic: "can_defeat(QueenGohma)"
  - id: "nuts_access"
    name: "Nuts Available"
"#;
        let region: Region = serde_yaml::from_str(yaml).unwrap();
        assert_eq!(region.events.len(), 2);
        assert_eq!(region.events[0].id, "deku_tree_clear");
        assert_eq!(
            region.events[0].logic,
            Some("can_defeat(QueenGohma)".to_string())
        );
        assert_eq!(region.events[1].id, "nuts_access");
        assert!(region.events[1].logic.is_none());
    }

    #[test]
    fn test_deserialize_all_exit_types() {
        let yaml = r#"
id: "test"
name: "Test"
game: oot
exits:
  - target: "a"
    exitType: normal
  - target: "b"
    exitType: oneWay
  - target: "c"
    exitType: lockedDoor
  - target: "d"
    exitType: warp
  - target: "e"
    exitType: owl
  - target: "f"
    exitType: dungeon
  - target: "g"
    exitType: grotto
  - target: "h"
    exitType: interior
  - target: "i"
    exitType: overworld
"#;
        let region: Region = serde_yaml::from_str(yaml).unwrap();
        assert_eq!(region.exits.len(), 9);
        assert_eq!(region.exits[0].exit_type, ExitType::Normal);
        assert_eq!(region.exits[1].exit_type, ExitType::OneWay);
        assert_eq!(region.exits[2].exit_type, ExitType::LockedDoor);
        assert_eq!(region.exits[3].exit_type, ExitType::Warp);
        assert_eq!(region.exits[4].exit_type, ExitType::Owl);
        assert_eq!(region.exits[5].exit_type, ExitType::Dungeon);
        assert_eq!(region.exits[6].exit_type, ExitType::Grotto);
        assert_eq!(region.exits[7].exit_type, ExitType::Interior);
        assert_eq!(region.exits[8].exit_type, ExitType::Overworld);
    }

    #[test]
    fn test_deserialize_all_location_types() {
        let yaml = r#"
id: "test"
name: "Test"
game: oot
locations:
  - id: "a"
    name: "Chest"
    locationType: chest
  - id: "b"
    name: "Freestanding"
    locationType: freestanding
  - id: "c"
    name: "NPC"
    locationType: npc
  - id: "d"
    name: "Event"
    locationType: event
  - id: "e"
    name: "Song"
    locationType: song
  - id: "f"
    name: "Collectible"
    locationType: collectible
  - id: "g"
    name: "Shop"
    locationType: shop
  - id: "h"
    name: "Scrub"
    locationType: scrub
  - id: "i"
    name: "Gossip Stone"
    locationType: gossipStone
  - id: "j"
    name: "Boss"
    locationType: boss
  - id: "k"
    name: "Cow"
    locationType: cow
  - id: "l"
    name: "Fishing"
    locationType: fishing
  - id: "m"
    name: "Fairy"
    locationType: fairy
"#;
        let region: Region = serde_yaml::from_str(yaml).unwrap();
        assert_eq!(region.locations.len(), 13);
        assert_eq!(region.locations[0].location_type, LocationType::Chest);
        assert_eq!(
            region.locations[1].location_type,
            LocationType::Freestanding
        );
        assert_eq!(region.locations[2].location_type, LocationType::Npc);
        assert_eq!(region.locations[3].location_type, LocationType::Event);
        assert_eq!(region.locations[4].location_type, LocationType::Song);
        assert_eq!(region.locations[5].location_type, LocationType::Collectible);
        assert_eq!(region.locations[6].location_type, LocationType::Shop);
        assert_eq!(region.locations[7].location_type, LocationType::Scrub);
        assert_eq!(region.locations[8].location_type, LocationType::GossipStone);
        assert_eq!(region.locations[9].location_type, LocationType::Boss);
        assert_eq!(region.locations[10].location_type, LocationType::Cow);
        assert_eq!(region.locations[11].location_type, LocationType::Fishing);
        assert_eq!(region.locations[12].location_type, LocationType::Fairy);
    }
}
