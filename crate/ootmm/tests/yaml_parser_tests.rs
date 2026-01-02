//! Comprehensive tests for YAML world data parsing.
//!
//! This module tests:
//! - Region loading from YAML strings
//! - Location parsing with various types and options
//! - Exit parsing with different exit types
//! - Event parsing
//! - Error handling for malformed YAML
//! - WorldDatabase integration

use ootmm::region::{Event, Exit, ExitType, Game, Location, LocationType, Region};
use ootmm::world_database::WorldDatabase;

// =============================================================================
// Region Loading Tests
// =============================================================================

#[test]
fn test_parse_single_region_minimal() {
    let yaml = r#"
id: "test_region"
name: "Test Region"
game: oot
"#;
    let region: Region = serde_yaml::from_str(yaml).unwrap();
    assert_eq!(region.id, "test_region");
    assert_eq!(region.name, "Test Region");
    assert_eq!(region.game, Game::Oot);
    assert!(region.locations.is_empty());
    assert!(region.exits.is_empty());
    assert!(region.events.is_empty());
}

#[test]
fn test_parse_region_with_all_fields() {
    let yaml = r#"
id: "complete_region"
name: "Complete Test Region"
game: mm
locations:
  - id: "loc1"
    name: "Location One"
    locationType: chest
    logic: "has(Hookshot)"
exits:
  - target: "other_region"
    exitType: dungeon
    logic: "has(BossKey)"
events:
  - id: "evt1"
    name: "Event One"
    logic: "has(Sword)"
"#;
    let region: Region = serde_yaml::from_str(yaml).unwrap();
    assert_eq!(region.id, "complete_region");
    assert_eq!(region.game, Game::Mm);
    assert_eq!(region.locations.len(), 1);
    assert_eq!(region.exits.len(), 1);
    assert_eq!(region.events.len(), 1);
}

#[test]
fn test_parse_oot_game_type() {
    let yaml = r#"
id: "oot_region"
name: "OoT Region"
game: oot
"#;
    let region: Region = serde_yaml::from_str(yaml).unwrap();
    assert_eq!(region.game, Game::Oot);
}

#[test]
fn test_parse_mm_game_type() {
    let yaml = r#"
id: "mm_region"
name: "MM Region"
game: mm
"#;
    let region: Region = serde_yaml::from_str(yaml).unwrap();
    assert_eq!(region.game, Game::Mm);
}

#[test]
fn test_parse_region_with_multiple_locations() {
    let yaml = r#"
id: "multi_loc_region"
name: "Multi-Location Region"
game: oot
locations:
  - id: "loc1"
    name: "First Location"
  - id: "loc2"
    name: "Second Location"
  - id: "loc3"
    name: "Third Location"
"#;
    let region: Region = serde_yaml::from_str(yaml).unwrap();
    assert_eq!(region.locations.len(), 3);
    assert_eq!(region.locations[0].id, "loc1");
    assert_eq!(region.locations[1].id, "loc2");
    assert_eq!(region.locations[2].id, "loc3");
}

#[test]
fn test_parse_region_with_multiple_exits() {
    let yaml = r#"
id: "multi_exit_region"
name: "Multi-Exit Region"
game: oot
exits:
  - target: "region_a"
    exitType: overworld
  - target: "region_b"
    exitType: dungeon
  - target: "region_c"
    exitType: grotto
"#;
    let region: Region = serde_yaml::from_str(yaml).unwrap();
    assert_eq!(region.exits.len(), 3);
    assert_eq!(region.exits[0].target, "region_a");
    assert_eq!(region.exits[1].target, "region_b");
    assert_eq!(region.exits[2].target, "region_c");
}

// =============================================================================
// Location Parsing Tests
// =============================================================================

#[test]
fn test_parse_location_minimal() {
    let yaml = r#"
id: "test_loc"
name: "Test Location"
"#;
    let loc: Location = serde_yaml::from_str(yaml).unwrap();
    assert_eq!(loc.id, "test_loc");
    assert_eq!(loc.name, "Test Location");
    assert!(loc.logic.is_none());
    assert_eq!(loc.location_type, LocationType::Chest); // Default
}

#[test]
fn test_parse_location_with_logic() {
    let yaml = r#"
id: "guarded_loc"
name: "Guarded Location"
logic: "has(Hookshot) && has(Bow)"
"#;
    let loc: Location = serde_yaml::from_str(yaml).unwrap();
    assert_eq!(loc.logic, Some("has(Hookshot) && has(Bow)".to_string()));
}

#[test]
fn test_parse_location_type_chest() {
    let yaml = r#"
id: "chest_loc"
name: "Chest"
locationType: chest
"#;
    let loc: Location = serde_yaml::from_str(yaml).unwrap();
    assert_eq!(loc.location_type, LocationType::Chest);
}

#[test]
fn test_parse_location_type_freestanding() {
    let yaml = r#"
id: "free_loc"
name: "Freestanding"
locationType: freestanding
"#;
    let loc: Location = serde_yaml::from_str(yaml).unwrap();
    assert_eq!(loc.location_type, LocationType::Freestanding);
}

#[test]
fn test_parse_location_type_npc() {
    let yaml = r#"
id: "npc_loc"
name: "NPC"
locationType: npc
"#;
    let loc: Location = serde_yaml::from_str(yaml).unwrap();
    assert_eq!(loc.location_type, LocationType::Npc);
}

#[test]
fn test_parse_location_type_event() {
    let yaml = r#"
id: "event_loc"
name: "Event"
locationType: event
"#;
    let loc: Location = serde_yaml::from_str(yaml).unwrap();
    assert_eq!(loc.location_type, LocationType::Event);
}

#[test]
fn test_parse_location_type_song() {
    let yaml = r#"
id: "song_loc"
name: "Song"
locationType: song
"#;
    let loc: Location = serde_yaml::from_str(yaml).unwrap();
    assert_eq!(loc.location_type, LocationType::Song);
}

#[test]
fn test_parse_location_type_collectible() {
    let yaml = r#"
id: "coll_loc"
name: "Collectible"
locationType: collectible
"#;
    let loc: Location = serde_yaml::from_str(yaml).unwrap();
    assert_eq!(loc.location_type, LocationType::Collectible);
}

#[test]
fn test_parse_location_type_shop() {
    let yaml = r#"
id: "shop_loc"
name: "Shop"
locationType: shop
"#;
    let loc: Location = serde_yaml::from_str(yaml).unwrap();
    assert_eq!(loc.location_type, LocationType::Shop);
}

#[test]
fn test_parse_location_type_scrub() {
    let yaml = r#"
id: "scrub_loc"
name: "Scrub"
locationType: scrub
"#;
    let loc: Location = serde_yaml::from_str(yaml).unwrap();
    assert_eq!(loc.location_type, LocationType::Scrub);
}

#[test]
fn test_parse_location_type_gossip_stone() {
    let yaml = r#"
id: "gs_loc"
name: "Gossip Stone"
locationType: gossipStone
"#;
    let loc: Location = serde_yaml::from_str(yaml).unwrap();
    assert_eq!(loc.location_type, LocationType::GossipStone);
}

#[test]
fn test_parse_location_type_boss() {
    let yaml = r#"
id: "boss_loc"
name: "Boss"
locationType: boss
"#;
    let loc: Location = serde_yaml::from_str(yaml).unwrap();
    assert_eq!(loc.location_type, LocationType::Boss);
}

#[test]
fn test_parse_location_type_cow() {
    let yaml = r#"
id: "cow_loc"
name: "Cow"
locationType: cow
"#;
    let loc: Location = serde_yaml::from_str(yaml).unwrap();
    assert_eq!(loc.location_type, LocationType::Cow);
}

#[test]
fn test_parse_location_type_fishing() {
    let yaml = r#"
id: "fish_loc"
name: "Fishing"
locationType: fishing
"#;
    let loc: Location = serde_yaml::from_str(yaml).unwrap();
    assert_eq!(loc.location_type, LocationType::Fishing);
}

#[test]
fn test_parse_location_type_fairy() {
    let yaml = r#"
id: "fairy_loc"
name: "Fairy"
locationType: fairy
"#;
    let loc: Location = serde_yaml::from_str(yaml).unwrap();
    assert_eq!(loc.location_type, LocationType::Fairy);
}

// =============================================================================
// Exit Parsing Tests
// =============================================================================

#[test]
fn test_parse_exit_minimal() {
    let yaml = r#"
target: "destination"
"#;
    let exit: Exit = serde_yaml::from_str(yaml).unwrap();
    assert_eq!(exit.target, "destination");
    assert!(exit.logic.is_none());
    assert_eq!(exit.exit_type, ExitType::Normal); // Default
}

#[test]
fn test_parse_exit_with_logic() {
    let yaml = r#"
target: "locked_area"
logic: "has(SmallKey, 3)"
"#;
    let exit: Exit = serde_yaml::from_str(yaml).unwrap();
    assert_eq!(exit.logic, Some("has(SmallKey, 3)".to_string()));
}

#[test]
fn test_parse_exit_type_normal() {
    let yaml = r#"
target: "next_room"
exitType: normal
"#;
    let exit: Exit = serde_yaml::from_str(yaml).unwrap();
    assert_eq!(exit.exit_type, ExitType::Normal);
}

#[test]
fn test_parse_exit_type_one_way() {
    let yaml = r#"
target: "lower_area"
exitType: oneWay
"#;
    let exit: Exit = serde_yaml::from_str(yaml).unwrap();
    assert_eq!(exit.exit_type, ExitType::OneWay);
}

#[test]
fn test_parse_exit_type_locked_door() {
    let yaml = r#"
target: "locked_room"
exitType: lockedDoor
"#;
    let exit: Exit = serde_yaml::from_str(yaml).unwrap();
    assert_eq!(exit.exit_type, ExitType::LockedDoor);
}

#[test]
fn test_parse_exit_type_warp() {
    let yaml = r#"
target: "warp_destination"
exitType: warp
"#;
    let exit: Exit = serde_yaml::from_str(yaml).unwrap();
    assert_eq!(exit.exit_type, ExitType::Warp);
}

#[test]
fn test_parse_exit_type_owl() {
    let yaml = r#"
target: "owl_destination"
exitType: owl
"#;
    let exit: Exit = serde_yaml::from_str(yaml).unwrap();
    assert_eq!(exit.exit_type, ExitType::Owl);
}

#[test]
fn test_parse_exit_type_dungeon() {
    let yaml = r#"
target: "dungeon_entrance"
exitType: dungeon
"#;
    let exit: Exit = serde_yaml::from_str(yaml).unwrap();
    assert_eq!(exit.exit_type, ExitType::Dungeon);
}

#[test]
fn test_parse_exit_type_grotto() {
    let yaml = r#"
target: "grotto_entrance"
exitType: grotto
"#;
    let exit: Exit = serde_yaml::from_str(yaml).unwrap();
    assert_eq!(exit.exit_type, ExitType::Grotto);
}

#[test]
fn test_parse_exit_type_interior() {
    let yaml = r#"
target: "building_interior"
exitType: interior
"#;
    let exit: Exit = serde_yaml::from_str(yaml).unwrap();
    assert_eq!(exit.exit_type, ExitType::Interior);
}

#[test]
fn test_parse_exit_type_overworld() {
    let yaml = r#"
target: "next_area"
exitType: overworld
"#;
    let exit: Exit = serde_yaml::from_str(yaml).unwrap();
    assert_eq!(exit.exit_type, ExitType::Overworld);
}

// =============================================================================
// Event Parsing Tests
// =============================================================================

#[test]
fn test_parse_event_minimal() {
    let yaml = r#"
id: "test_event"
name: "Test Event"
"#;
    let event: Event = serde_yaml::from_str(yaml).unwrap();
    assert_eq!(event.id, "test_event");
    assert_eq!(event.name, "Test Event");
    assert!(event.logic.is_none());
}

#[test]
fn test_parse_event_with_logic() {
    let yaml = r#"
id: "conditional_event"
name: "Conditional Event"
logic: "has(Sword) && has(Shield)"
"#;
    let event: Event = serde_yaml::from_str(yaml).unwrap();
    assert_eq!(event.logic, Some("has(Sword) && has(Shield)".to_string()));
}

#[test]
fn test_parse_event_with_complex_logic() {
    let yaml = r#"
id: "complex_event"
name: "Complex Event"
logic: "(has(Hookshot) || has(Longshot)) && (age(adult) || has(Boomerang))"
"#;
    let event: Event = serde_yaml::from_str(yaml).unwrap();
    assert_eq!(
        event.logic,
        Some("(has(Hookshot) || has(Longshot)) && (age(adult) || has(Boomerang))".to_string())
    );
}

// =============================================================================
// Error Handling Tests
// =============================================================================

#[test]
fn test_parse_invalid_yaml_syntax() {
    let yaml = r#"
id: "broken
name: "Missing Quote"
game: oot
"#;
    let result: Result<Region, _> = serde_yaml::from_str(yaml);
    assert!(result.is_err());
}

#[test]
fn test_parse_missing_required_field_id() {
    let yaml = r#"
name: "No ID Region"
game: oot
"#;
    let result: Result<Region, _> = serde_yaml::from_str(yaml);
    assert!(result.is_err());
}

#[test]
fn test_parse_missing_required_field_name() {
    let yaml = r#"
id: "no_name_region"
game: oot
"#;
    let result: Result<Region, _> = serde_yaml::from_str(yaml);
    assert!(result.is_err());
}

#[test]
fn test_parse_missing_required_field_game() {
    let yaml = r#"
id: "no_game_region"
name: "No Game Region"
"#;
    let result: Result<Region, _> = serde_yaml::from_str(yaml);
    assert!(result.is_err());
}

#[test]
fn test_parse_invalid_game_type() {
    let yaml = r#"
id: "invalid_game_region"
name: "Invalid Game Region"
game: invalid_game
"#;
    let result: Result<Region, _> = serde_yaml::from_str(yaml);
    assert!(result.is_err());
}

#[test]
fn test_parse_invalid_location_type() {
    let yaml = r#"
id: "invalid_loc"
name: "Invalid Location"
locationType: invalidType
"#;
    let result: Result<Location, _> = serde_yaml::from_str(yaml);
    assert!(result.is_err());
}

#[test]
fn test_parse_invalid_exit_type() {
    let yaml = r#"
target: "somewhere"
exitType: invalidExitType
"#;
    let result: Result<Exit, _> = serde_yaml::from_str(yaml);
    assert!(result.is_err());
}

#[test]
fn test_parse_exit_missing_target() {
    let yaml = r#"
exitType: normal
"#;
    let result: Result<Exit, _> = serde_yaml::from_str(yaml);
    assert!(result.is_err());
}

#[test]
fn test_parse_location_missing_id() {
    let yaml = r#"
name: "No ID Location"
"#;
    let result: Result<Location, _> = serde_yaml::from_str(yaml);
    assert!(result.is_err());
}

#[test]
fn test_parse_event_missing_id() {
    let yaml = r#"
name: "No ID Event"
"#;
    let result: Result<Event, _> = serde_yaml::from_str(yaml);
    assert!(result.is_err());
}

// =============================================================================
// WorldDatabase Integration Tests
// =============================================================================

#[test]
fn test_world_database_load_complex_world() {
    let yaml = r#"
regions:
  - id: "hyrule_field"
    name: "Hyrule Field"
    game: oot
    locations:
      - id: "hf_fairy"
        name: "Hyrule Field Fairy"
        locationType: fairy
      - id: "hf_skulltula"
        name: "Hyrule Field Skulltula"
        locationType: collectible
        logic: "has(Boomerang) || has(Hookshot)"
    exits:
      - target: "kokiri_forest"
        exitType: overworld
      - target: "kakariko_village"
        exitType: overworld
      - target: "lake_hylia"
        exitType: overworld
    events:
      - id: "hf_big_poe_hunt"
        name: "Big Poe Hunt Complete"
        logic: "has(Bow) && has(Epona)"
  - id: "kokiri_forest"
    name: "Kokiri Forest"
    game: oot
    locations:
      - id: "kf_midos_chest"
        name: "Mido's House Chest"
        locationType: chest
    exits:
      - target: "hyrule_field"
        exitType: overworld
      - target: "deku_tree"
        exitType: dungeon
  - id: "kakariko_village"
    name: "Kakariko Village"
    game: oot
    locations:
      - id: "kak_bottle_merchant"
        name: "Kakariko Bottle Merchant"
        locationType: npc
    exits:
      - target: "hyrule_field"
        exitType: overworld
      - target: "graveyard"
        exitType: overworld
  - id: "lake_hylia"
    name: "Lake Hylia"
    game: oot
    exits:
      - target: "hyrule_field"
        exitType: overworld
  - id: "deku_tree"
    name: "Inside the Deku Tree"
    game: oot
    exits:
      - target: "kokiri_forest"
        exitType: dungeon
  - id: "graveyard"
    name: "Graveyard"
    game: oot
    exits:
      - target: "kakariko_village"
        exitType: overworld
"#;

    let mut db = WorldDatabase::new();
    db.load_from_str(yaml).unwrap();

    assert_eq!(db.region_count(), 6);
    assert_eq!(db.location_count(), 4);
    assert_eq!(db.event_count(), 1);
    assert_eq!(db.exit_count(), 10);

    // Verify region data
    let hf = db.get_region("hyrule_field").unwrap();
    assert_eq!(hf.name, "Hyrule Field");
    assert_eq!(hf.locations.len(), 2);
    assert_eq!(hf.exits.len(), 3);

    // Verify location lookup
    let (loc, region_id) = db.get_location("hf_skulltula").unwrap();
    assert_eq!(loc.location_type, LocationType::Collectible);
    assert_eq!(region_id, "hyrule_field");

    // Verify event lookup
    let (evt, region_id) = db.get_event("hf_big_poe_hunt").unwrap();
    assert_eq!(evt.name, "Big Poe Hunt Complete");
    assert_eq!(region_id, "hyrule_field");

    // Validate all exit targets exist
    assert!(db.validate().is_ok());
}

#[test]
fn test_world_database_events_iterator() {
    let yaml = r#"
regions:
  - id: "region1"
    name: "Region One"
    game: oot
    events:
      - id: "evt1"
        name: "Event 1"
      - id: "evt2"
        name: "Event 2"
  - id: "region2"
    name: "Region Two"
    game: mm
    events:
      - id: "evt3"
        name: "Event 3"
"#;

    let mut db = WorldDatabase::new();
    db.load_from_str(yaml).unwrap();

    let events: Vec<_> = db.events().collect();
    assert_eq!(events.len(), 3);

    let event_ids: Vec<_> = events.iter().map(|(e, _)| e.id.as_str()).collect();
    assert!(event_ids.contains(&"evt1"));
    assert!(event_ids.contains(&"evt2"));
    assert!(event_ids.contains(&"evt3"));
}

#[test]
fn test_world_database_duplicate_event_id_error() {
    let yaml = r#"
regions:
  - id: "region1"
    name: "Region One"
    game: oot
    events:
      - id: "same_event"
        name: "Event in Region 1"
  - id: "region2"
    name: "Region Two"
    game: oot
    events:
      - id: "same_event"
        name: "Event in Region 2"
"#;

    let mut db = WorldDatabase::new();
    let result = db.load_from_str(yaml);
    assert!(result.is_err());
    assert!(result
        .unwrap_err()
        .to_string()
        .contains("duplicate event ID"));
}

#[test]
fn test_world_database_mixed_games() {
    let yaml = r#"
regions:
  - id: "kokiri_forest"
    name: "Kokiri Forest"
    game: oot
    locations:
      - id: "kf_chest"
        name: "KF Chest"
  - id: "clock_town_south"
    name: "South Clock Town"
    game: mm
    locations:
      - id: "ct_chest1"
        name: "CT Chest 1"
      - id: "ct_chest2"
        name: "CT Chest 2"
  - id: "woodfall"
    name: "Woodfall"
    game: mm
    locations:
      - id: "wf_chest"
        name: "WF Chest"
"#;

    let mut db = WorldDatabase::new();
    db.load_from_str(yaml).unwrap();

    let oot_regions: Vec<_> = db.regions_for_game(Game::Oot).collect();
    assert_eq!(oot_regions.len(), 1);
    assert_eq!(oot_regions[0].id, "kokiri_forest");

    let mm_regions: Vec<_> = db.regions_for_game(Game::Mm).collect();
    assert_eq!(mm_regions.len(), 2);

    let oot_locs: Vec<_> = db.locations_for_game(Game::Oot).collect();
    assert_eq!(oot_locs.len(), 1);

    let mm_locs: Vec<_> = db.locations_for_game(Game::Mm).collect();
    assert_eq!(mm_locs.len(), 3);
}

#[test]
fn test_world_database_empty_regions_array() {
    let yaml = "regions: []";
    let mut db = WorldDatabase::new();
    db.load_from_str(yaml).unwrap();
    assert_eq!(db.region_count(), 0);
    assert_eq!(db.location_count(), 0);
    assert_eq!(db.exit_count(), 0);
    assert_eq!(db.event_count(), 0);
}

#[test]
fn test_world_database_region_with_empty_arrays() {
    let yaml = r#"
regions:
  - id: "empty_region"
    name: "Empty Region"
    game: oot
    locations: []
    exits: []
    events: []
"#;

    let mut db = WorldDatabase::new();
    db.load_from_str(yaml).unwrap();
    assert_eq!(db.region_count(), 1);
    assert_eq!(db.location_count(), 0);
    assert_eq!(db.exit_count(), 0);
    assert_eq!(db.event_count(), 0);

    let region = db.get_region("empty_region").unwrap();
    assert!(region.locations.is_empty());
    assert!(region.exits.is_empty());
    assert!(region.events.is_empty());
}

#[test]
fn test_world_database_nonexistent_lookups() {
    let yaml = r#"
regions:
  - id: "test_region"
    name: "Test Region"
    game: oot
"#;

    let mut db = WorldDatabase::new();
    db.load_from_str(yaml).unwrap();

    assert!(db.get_region("nonexistent").is_none());
    assert!(db.get_location("nonexistent").is_none());
    assert!(db.get_event("nonexistent").is_none());
    assert!(!db.has_region("nonexistent"));
    assert!(!db.has_location("nonexistent"));
    assert!(!db.has_event("nonexistent"));
}

#[test]
fn test_parse_region_preserves_location_order() {
    let yaml = r#"
id: "ordered_region"
name: "Ordered Region"
game: oot
locations:
  - id: "first"
    name: "First"
  - id: "second"
    name: "Second"
  - id: "third"
    name: "Third"
  - id: "fourth"
    name: "Fourth"
"#;

    let region: Region = serde_yaml::from_str(yaml).unwrap();
    assert_eq!(region.locations[0].id, "first");
    assert_eq!(region.locations[1].id, "second");
    assert_eq!(region.locations[2].id, "third");
    assert_eq!(region.locations[3].id, "fourth");
}

#[test]
fn test_parse_region_preserves_exit_order() {
    let yaml = r#"
id: "ordered_region"
name: "Ordered Region"
game: oot
exits:
  - target: "area_a"
  - target: "area_b"
  - target: "area_c"
"#;

    let region: Region = serde_yaml::from_str(yaml).unwrap();
    assert_eq!(region.exits[0].target, "area_a");
    assert_eq!(region.exits[1].target, "area_b");
    assert_eq!(region.exits[2].target, "area_c");
}

#[test]
fn test_parse_location_with_empty_logic_string() {
    let yaml = r#"
id: "empty_logic_loc"
name: "Empty Logic Location"
logic: ""
"#;

    let loc: Location = serde_yaml::from_str(yaml).unwrap();
    assert_eq!(loc.logic, Some(String::new()));
}

#[test]
fn test_world_database_validate_circular_exits() {
    let yaml = r#"
regions:
  - id: "region_a"
    name: "Region A"
    game: oot
    exits:
      - target: "region_b"
  - id: "region_b"
    name: "Region B"
    game: oot
    exits:
      - target: "region_a"
"#;

    let mut db = WorldDatabase::new();
    db.load_from_str(yaml).unwrap();
    assert!(db.validate().is_ok());
}

#[test]
fn test_world_database_validate_self_referencing_exit() {
    let yaml = r#"
regions:
  - id: "self_ref_region"
    name: "Self-Referencing Region"
    game: oot
    exits:
      - target: "self_ref_region"
"#;

    let mut db = WorldDatabase::new();
    db.load_from_str(yaml).unwrap();
    assert!(db.validate().is_ok());
}
