//! End-to-end tracker session tests.
//!
//! These tests simulate a complete tracker session, verifying that:
//! - Accessibility increases as items are collected
//! - Logic expressions are properly evaluated against game state
//! - The WorldDatabase and expression evaluator work together correctly

use ootmm::expr::{eval_str, Age, GameContext};
use ootmm::region::Location;
use ootmm::world_database::WorldDatabase;

/// Helper struct to track accessible locations during a session.
struct TrackerSession {
    db: WorldDatabase,
    ctx: GameContext,
}

impl TrackerSession {
    /// Creates a new tracker session with the given world database.
    fn new(db: WorldDatabase) -> Self {
        Self {
            db,
            ctx: GameContext::new(),
        }
    }

    /// Counts how many locations are currently accessible.
    fn count_accessible_locations(&self) -> usize {
        self.db
            .locations()
            .filter(|(location, _region_id)| self.is_location_accessible(location))
            .count()
    }

    /// Checks if a specific location is accessible with current items.
    fn is_location_accessible(&self, location: &Location) -> bool {
        match &location.logic {
            // Locations without logic are always accessible
            None => true,
            Some(logic) if logic == "true" => true,
            Some(logic) if logic == "false" => false,
            Some(logic) => {
                // Evaluate the logic expression against current context
                eval_str(logic, &self.ctx).unwrap_or(false)
            }
        }
    }

    /// Adds an item to the player's inventory.
    fn add_item(&mut self, item: &str) {
        self.ctx.set_item(item, 1);
    }

    /// Sets the player's age.
    fn set_age(&mut self, age: Age) {
        self.ctx.set_age(age);
    }

    /// Triggers an event.
    fn add_event(&mut self, event: &str) {
        self.ctx.add_event(event);
    }
}

/// Creates a test world database with Kokiri Forest and related areas.
fn create_kokiri_forest_world() -> WorldDatabase {
    let yaml = r#"
regions:
  - id: "kokiri_forest"
    name: "Kokiri Forest"
    game: oot
    locations:
      - id: "kf_midos_chest"
        name: "Mido's House Chest"
        locationType: chest
        logic: "true"
      - id: "kf_sword_chest"
        name: "Kokiri Sword Chest"
        locationType: chest
        logic: "true"
      - id: "kf_shop_item_1"
        name: "Kokiri Shop Item 1"
        locationType: shop
        logic: "true"
    exits:
      - target: "lost_woods"
        exitType: overworld
      - target: "deku_tree"
        exitType: dungeon
        logic: "has(KOKIRISWORD) && has(DEKUSHIELD)"
    events:
      - id: "showed_mido_sword_shield"
        name: "Showed Mido Sword and Shield"
        logic: "has(KOKIRISWORD) && has(DEKUSHIELD)"

  - id: "lost_woods"
    name: "Lost Woods"
    game: oot
    locations:
      - id: "lw_skull_kid_gift"
        name: "Skull Kid Gift"
        locationType: npc
        logic: "has(SLINGSHOT)"
      - id: "lw_target_practice"
        name: "Target Practice"
        locationType: npc
        logic: "has(SLINGSHOT)"
      - id: "lw_deku_scrub_grotto"
        name: "Deku Scrub Grotto"
        locationType: scrub
        logic: "true"

  - id: "deku_tree"
    name: "Inside the Deku Tree"
    game: oot
    locations:
      - id: "dt_map_chest"
        name: "Deku Tree Map Chest"
        locationType: chest
        logic: "has(KOKIRISWORD) && has(DEKUSHIELD)"
      - id: "dt_compass_chest"
        name: "Deku Tree Compass Chest"
        locationType: chest
        logic: "has(KOKIRISWORD) && has(DEKUSHIELD)"
      - id: "dt_slingshot_chest"
        name: "Deku Tree Slingshot Chest"
        locationType: chest
        logic: "has(KOKIRISWORD) && has(DEKUSHIELD)"
      - id: "dt_basement_chest"
        name: "Deku Tree Basement Chest"
        locationType: chest
        logic: "has(KOKIRISWORD) && has(DEKUSHIELD) && has(SLINGSHOT)"
      - id: "dt_gohma_heart"
        name: "Queen Gohma Heart Container"
        locationType: boss
        logic: "has(KOKIRISWORD) && has(DEKUSHIELD) && has(SLINGSHOT)"
      - id: "dt_kokiri_emerald"
        name: "Kokiri Emerald"
        locationType: boss
        logic: "has(KOKIRISWORD) && has(DEKUSHIELD) && has(SLINGSHOT)"

  - id: "hyrule_field"
    name: "Hyrule Field"
    game: oot
    locations:
      - id: "hf_ocarina_gift"
        name: "Ocarina of Time Gift"
        locationType: npc
        logic: "has(KOKIRI_EMERALD)"
      - id: "hf_grotto_item"
        name: "Hyrule Field Grotto Item"
        locationType: chest
        logic: "true"
"#;

    let mut db = WorldDatabase::new();
    db.load_from_str(yaml).unwrap();
    db
}

/// Creates a more complex world for testing progressive item collection.
fn create_progressive_world() -> WorldDatabase {
    let yaml = r#"
regions:
  - id: "starting_area"
    name: "Starting Area"
    game: oot
    locations:
      - id: "start_chest_1"
        name: "Starting Chest 1"
        locationType: chest
        logic: "true"
      - id: "start_chest_2"
        name: "Starting Chest 2"
        locationType: chest
        logic: "true"
      - id: "start_npc"
        name: "Starting NPC"
        locationType: npc
        logic: "true"

  - id: "sword_required_area"
    name: "Sword Required Area"
    game: oot
    locations:
      - id: "sword_chest_1"
        name: "Sword Chest 1"
        locationType: chest
        logic: "has(KOKIRISWORD)"
      - id: "sword_chest_2"
        name: "Sword Chest 2"
        locationType: chest
        logic: "has(KOKIRISWORD)"

  - id: "shield_required_area"
    name: "Shield Required Area"
    game: oot
    locations:
      - id: "shield_chest_1"
        name: "Shield Chest 1"
        locationType: chest
        logic: "has(DEKUSHIELD)"

  - id: "sword_and_shield_area"
    name: "Sword and Shield Area"
    game: oot
    locations:
      - id: "both_chest_1"
        name: "Both Required Chest 1"
        locationType: chest
        logic: "has(KOKIRISWORD) && has(DEKUSHIELD)"
      - id: "both_chest_2"
        name: "Both Required Chest 2"
        locationType: chest
        logic: "has(KOKIRISWORD) && has(DEKUSHIELD)"
      - id: "both_chest_3"
        name: "Both Required Chest 3"
        locationType: chest
        logic: "has(KOKIRISWORD) && has(DEKUSHIELD)"

  - id: "slingshot_area"
    name: "Slingshot Area"
    game: oot
    locations:
      - id: "sling_chest_1"
        name: "Slingshot Chest 1"
        locationType: chest
        logic: "has(SLINGSHOT)"
      - id: "sling_chest_2"
        name: "Slingshot Chest 2"
        locationType: chest
        logic: "has(SLINGSHOT)"
      - id: "sling_chest_3"
        name: "Slingshot Chest 3"
        locationType: chest
        logic: "has(SLINGSHOT)"
      - id: "sling_chest_4"
        name: "Slingshot Chest 4"
        locationType: chest
        logic: "has(SLINGSHOT)"

  - id: "all_items_area"
    name: "All Items Area"
    game: oot
    locations:
      - id: "all_chest_1"
        name: "All Items Chest 1"
        locationType: chest
        logic: "has(KOKIRISWORD) && has(DEKUSHIELD) && has(SLINGSHOT)"
      - id: "all_chest_2"
        name: "All Items Chest 2"
        locationType: chest
        logic: "has(KOKIRISWORD) && has(DEKUSHIELD) && has(SLINGSHOT)"
"#;

    let mut db = WorldDatabase::new();
    db.load_from_str(yaml).unwrap();
    db
}

// ============================================================================
// E2E Tracker Session Tests
// ============================================================================

#[test]
fn test_e2e_empty_state_accessibility() {
    let db = create_kokiri_forest_world();
    let session = TrackerSession::new(db);

    // With no items, we should only be able to access locations with "true" logic
    let accessible = session.count_accessible_locations();

    // Expected accessible: kf_midos_chest, kf_sword_chest, kf_shop_item_1,
    // lw_deku_scrub_grotto, hf_grotto_item = 5 locations
    assert_eq!(
        accessible, 5,
        "With no items, should have exactly 5 accessible locations"
    );
}

#[test]
fn test_e2e_sword_and_shield_increases_accessibility() {
    let db = create_kokiri_forest_world();
    let mut session = TrackerSession::new(db);

    // Count initial accessible locations
    let initial_accessible = session.count_accessible_locations();
    assert_eq!(initial_accessible, 5);

    // Add Kokiri Sword and Deku Shield
    session.add_item("KOKIRISWORD");
    session.add_item("DEKUSHIELD");

    // Now we should be able to access Deku Tree locations
    let after_sword_shield = session.count_accessible_locations();

    // Expected new accessible: dt_map_chest, dt_compass_chest, dt_slingshot_chest = 3 more
    assert!(
        after_sword_shield > initial_accessible,
        "Accessibility should increase after getting sword and shield"
    );
    assert_eq!(
        after_sword_shield, 8,
        "Should have 8 accessible locations after sword+shield"
    );
}

#[test]
fn test_e2e_slingshot_further_increases_accessibility() {
    let db = create_kokiri_forest_world();
    let mut session = TrackerSession::new(db);

    // Start with sword and shield
    session.add_item("KOKIRISWORD");
    session.add_item("DEKUSHIELD");
    let after_sword_shield = session.count_accessible_locations();

    // Add Slingshot
    session.add_item("SLINGSHOT");
    let after_slingshot = session.count_accessible_locations();

    // Expected new accessible: lw_skull_kid_gift, lw_target_practice,
    // dt_basement_chest, dt_gohma_heart, dt_kokiri_emerald = 5 more
    assert!(
        after_slingshot > after_sword_shield,
        "Accessibility should increase after getting slingshot"
    );
    assert_eq!(
        after_slingshot, 13,
        "Should have 13 accessible locations after sword+shield+slingshot"
    );
}

#[test]
fn test_e2e_progressive_item_collection() {
    let db = create_progressive_world();
    let mut session = TrackerSession::new(db);

    // Stage 1: Empty state
    let stage1 = session.count_accessible_locations();
    assert_eq!(
        stage1, 3,
        "Stage 1 (empty): Should have 3 accessible locations"
    );

    // Stage 2: Add Kokiri Sword
    session.add_item("KOKIRISWORD");
    let stage2 = session.count_accessible_locations();
    assert_eq!(
        stage2, 5,
        "Stage 2 (sword): Should have 5 accessible locations"
    );
    assert!(stage2 > stage1, "Stage 2 should have more than stage 1");

    // Stage 3: Add Deku Shield
    session.add_item("DEKUSHIELD");
    let stage3 = session.count_accessible_locations();
    assert_eq!(
        stage3, 9,
        "Stage 3 (sword+shield): Should have 9 accessible locations"
    );
    assert!(stage3 > stage2, "Stage 3 should have more than stage 2");

    // Stage 4: Add Slingshot
    session.add_item("SLINGSHOT");
    let stage4 = session.count_accessible_locations();
    assert_eq!(
        stage4, 15,
        "Stage 4 (all items): Should have 15 accessible locations"
    );
    assert!(stage4 > stage3, "Stage 4 should have more than stage 3");

    // Verify we've unlocked all locations
    assert_eq!(
        stage4,
        session.db.location_count(),
        "With all items, all locations should be accessible"
    );
}

#[test]
fn test_e2e_accessibility_monotonically_increases() {
    let db = create_progressive_world();
    let mut session = TrackerSession::new(db);

    let mut previous_count = session.count_accessible_locations();
    let items = ["KOKIRISWORD", "DEKUSHIELD", "SLINGSHOT"];

    for item in items {
        session.add_item(item);
        let current_count = session.count_accessible_locations();

        assert!(
            current_count >= previous_count,
            "Accessibility should never decrease when adding items. \
             After adding {}: {} (was {})",
            item,
            current_count,
            previous_count
        );

        previous_count = current_count;
    }
}

#[test]
fn test_e2e_specific_location_accessibility() {
    let db = create_kokiri_forest_world();
    let mut session = TrackerSession::new(db);

    // Helper to check location accessibility by ID
    fn check_accessible(session: &TrackerSession, loc_id: &str) -> bool {
        let (location, _) = session.db.get_location(loc_id).unwrap();
        session.is_location_accessible(location)
    }

    // Initially neither should be accessible
    assert!(
        !check_accessible(&session, "dt_slingshot_chest"),
        "Slingshot chest should not be accessible without sword+shield"
    );
    assert!(
        !check_accessible(&session, "dt_basement_chest"),
        "Basement chest should not be accessible without all items"
    );

    // Add sword and shield
    session.add_item("KOKIRISWORD");
    session.add_item("DEKUSHIELD");

    // Slingshot chest should now be accessible, but not basement
    assert!(
        check_accessible(&session, "dt_slingshot_chest"),
        "Slingshot chest should be accessible with sword+shield"
    );
    assert!(
        !check_accessible(&session, "dt_basement_chest"),
        "Basement chest should not be accessible without slingshot"
    );

    // Add slingshot
    session.add_item("SLINGSHOT");

    // Now both should be accessible
    assert!(
        check_accessible(&session, "dt_slingshot_chest"),
        "Slingshot chest should still be accessible"
    );
    assert!(
        check_accessible(&session, "dt_basement_chest"),
        "Basement chest should now be accessible with slingshot"
    );
}

#[test]
fn test_e2e_or_logic_accessibility() {
    // Test locations with OR logic (accessible via multiple paths)
    let yaml = r#"
regions:
  - id: "test_region"
    name: "Test Region"
    game: oot
    locations:
      - id: "or_location"
        name: "OR Logic Location"
        locationType: chest
        logic: "has(HOOKSHOT) || has(LONGSHOT)"
      - id: "and_or_location"
        name: "AND/OR Logic Location"
        locationType: chest
        logic: "(has(BOW) || has(SLINGSHOT)) && has(BOMB)"
"#;

    let mut db = WorldDatabase::new();
    db.load_from_str(yaml).unwrap();
    let mut session = TrackerSession::new(db);

    // Helper to check location accessibility by ID
    fn check_accessible(session: &TrackerSession, loc_id: &str) -> bool {
        let (location, _) = session.db.get_location(loc_id).unwrap();
        session.is_location_accessible(location)
    }

    // Initially neither accessible
    assert!(!check_accessible(&session, "or_location"));
    assert!(!check_accessible(&session, "and_or_location"));

    // Hookshot alone satisfies OR location
    session.add_item("HOOKSHOT");
    assert!(check_accessible(&session, "or_location"));
    assert!(!check_accessible(&session, "and_or_location"));

    // Slingshot alone doesn't satisfy AND/OR location (needs bomb too)
    session.add_item("SLINGSHOT");
    assert!(!check_accessible(&session, "and_or_location"));

    // Adding bomb completes the AND/OR requirement
    session.add_item("BOMB");
    assert!(check_accessible(&session, "and_or_location"));
}

#[test]
fn test_e2e_age_based_accessibility() {
    let yaml = r#"
regions:
  - id: "age_test_region"
    name: "Age Test Region"
    game: oot
    locations:
      - id: "child_only_location"
        name: "Child Only Location"
        locationType: chest
        logic: "is_child"
      - id: "adult_only_location"
        name: "Adult Only Location"
        locationType: chest
        logic: "is_adult"
      - id: "child_with_item"
        name: "Child with Slingshot"
        locationType: chest
        logic: "is_child && has(SLINGSHOT)"
      - id: "adult_with_item"
        name: "Adult with Hookshot"
        locationType: chest
        logic: "is_adult && has(HOOKSHOT)"
"#;

    let mut db = WorldDatabase::new();
    db.load_from_str(yaml).unwrap();
    let mut session = TrackerSession::new(db);

    // Default is child
    session.set_age(Age::Child);
    let child_accessible = session.count_accessible_locations();
    assert_eq!(
        child_accessible, 1,
        "Child should access 1 location initially"
    );

    // Add slingshot as child
    session.add_item("SLINGSHOT");
    let child_with_sling = session.count_accessible_locations();
    assert_eq!(
        child_with_sling, 2,
        "Child with slingshot should access 2 locations"
    );

    // Switch to adult
    session.set_age(Age::Adult);
    let adult_accessible = session.count_accessible_locations();
    assert_eq!(
        adult_accessible, 1,
        "Adult should access 1 location initially"
    );

    // Add hookshot as adult
    session.add_item("HOOKSHOT");
    let adult_with_hook = session.count_accessible_locations();
    assert_eq!(
        adult_with_hook, 2,
        "Adult with hookshot should access 2 locations"
    );
}

#[test]
fn test_e2e_event_based_accessibility() {
    let yaml = r#"
regions:
  - id: "event_test_region"
    name: "Event Test Region"
    game: oot
    locations:
      - id: "base_location"
        name: "Base Location"
        locationType: chest
        logic: "true"
      - id: "event_location"
        name: "Event Required Location"
        locationType: chest
        logic: "event(DEKU_TREE_CLEAR)"
      - id: "event_and_item_location"
        name: "Event and Item Location"
        locationType: chest
        logic: "event(DEKU_TREE_CLEAR) && has(HOOKSHOT)"
"#;

    let mut db = WorldDatabase::new();
    db.load_from_str(yaml).unwrap();
    let mut session = TrackerSession::new(db);

    assert_eq!(session.count_accessible_locations(), 1);

    // Trigger event
    session.add_event("DEKU_TREE_CLEAR");
    assert_eq!(session.count_accessible_locations(), 2);

    // Add required item
    session.add_item("HOOKSHOT");
    assert_eq!(session.count_accessible_locations(), 3);
}

#[test]
fn test_e2e_full_tracker_simulation() {
    // Simulate a realistic early game tracker session
    let db = create_kokiri_forest_world();
    let mut session = TrackerSession::new(db);

    // Track accessibility at each step
    let mut accessibility_history = Vec::new();

    // Initial state
    accessibility_history.push(("Start", session.count_accessible_locations()));

    // Player finds Kokiri Sword
    session.add_item("KOKIRISWORD");
    accessibility_history.push(("Got Kokiri Sword", session.count_accessible_locations()));

    // Player buys Deku Shield from shop
    session.add_item("DEKUSHIELD");
    accessibility_history.push(("Got Deku Shield", session.count_accessible_locations()));

    // Player enters Deku Tree and finds Slingshot
    session.add_item("SLINGSHOT");
    accessibility_history.push(("Got Slingshot", session.count_accessible_locations()));

    // Player defeats Gohma and gets Kokiri Emerald
    session.add_item("KOKIRI_EMERALD");
    accessibility_history.push(("Got Kokiri Emerald", session.count_accessible_locations()));

    // Verify accessibility increased at each step
    for i in 1..accessibility_history.len() {
        let (prev_name, prev_count) = &accessibility_history[i - 1];
        let (curr_name, curr_count) = &accessibility_history[i];
        assert!(
            curr_count >= prev_count,
            "Accessibility should not decrease: {} ({}) -> {} ({})",
            prev_name,
            prev_count,
            curr_name,
            curr_count
        );
    }

    // Verify final state
    let (final_name, final_count) = accessibility_history.last().unwrap();
    assert_eq!(
        *final_count, 14,
        "Final accessibility for {} should be 14",
        final_name
    );
}

// ============================================================================
// WorldDatabase and Expression Integration Tests
// ============================================================================

#[test]
fn test_world_database_location_iteration() {
    let db = create_kokiri_forest_world();

    // Verify we can iterate all locations
    let location_count = db.location_count();
    let iterated_count = db.locations().count();

    assert_eq!(location_count, iterated_count);
    // create_kokiri_forest_world() defines 14 locations across 4 regions
    assert_eq!(location_count, 14, "Kokiri test world has 14 locations");
}

#[test]
fn test_expression_evaluation_with_game_context() {
    let mut ctx = GameContext::new();

    // Test basic has() evaluation
    assert!(!eval_str("has(KOKIRISWORD)", &ctx).unwrap());

    ctx.set_item("KOKIRISWORD", 1);
    assert!(eval_str("has(KOKIRISWORD)", &ctx).unwrap());

    // Test AND evaluation
    assert!(!eval_str("has(KOKIRISWORD) && has(DEKUSHIELD)", &ctx).unwrap());

    ctx.set_item("DEKUSHIELD", 1);
    assert!(eval_str("has(KOKIRISWORD) && has(DEKUSHIELD)", &ctx).unwrap());

    // Test OR evaluation
    assert!(eval_str("has(KOKIRISWORD) || has(HOOKSHOT)", &ctx).unwrap());
    assert!(!eval_str("has(BOMBS) || has(HOOKSHOT)", &ctx).unwrap());
}

#[test]
fn test_parse_and_evaluate_location_logic() {
    let db = create_kokiri_forest_world();
    let mut ctx = GameContext::new();

    // Get a location with logic
    let (location, _) = db.get_location("dt_basement_chest").unwrap();
    let logic = location.logic.as_ref().unwrap();

    // Should not be accessible initially
    assert!(!eval_str(logic, &ctx).unwrap());

    // Add required items one by one
    ctx.set_item("KOKIRISWORD", 1);
    assert!(!eval_str(logic, &ctx).unwrap());

    ctx.set_item("DEKUSHIELD", 1);
    assert!(!eval_str(logic, &ctx).unwrap());

    ctx.set_item("SLINGSHOT", 1);
    assert!(eval_str(logic, &ctx).unwrap());
}
