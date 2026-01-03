//! Integration tests for test fixtures infrastructure.

mod common;

use common::{fixture_path, load_fixture, load_fixture_bytes};
use serde_json::Value;

#[test]
fn test_fixture_path_returns_valid_path() {
    let path = fixture_path("knowledge/default.json");
    assert!(path.ends_with("tests/fixtures/knowledge/default.json"));
}

#[test]
fn test_load_fixture_reads_json() {
    let content = load_fixture("knowledge/default.json");
    let json: Value = serde_json::from_str(&content).expect("should be valid JSON");

    // Verify the expected field exists and has correct type
    assert!(
        json.get("progression_mode").is_some(),
        "default.json should have 'progression_mode' field"
    );
    assert!(
        json["progression_mode"].is_string(),
        "'progression_mode' should be a string"
    );
}

#[test]
fn test_load_vanilla_fixture() {
    let content = load_fixture("knowledge/vanilla.json");
    let json: Value = serde_json::from_str(&content).expect("should be valid JSON");

    // Verify settings structure and specific field values
    let settings = json
        .get("settings")
        .expect("vanilla.json should have 'settings' field");
    assert!(
        settings.get("open_door_of_time").is_some(),
        "settings should have 'open_door_of_time' field"
    );
    assert_eq!(
        settings["open_door_of_time"],
        Value::Bool(false),
        "open_door_of_time should be false in vanilla"
    );

    // Verify dungeons structure and specific dungeon
    let dungeons = json
        .get("dungeons")
        .expect("vanilla.json should have 'dungeons' field");
    assert!(
        dungeons.get("Deku Tree").is_some(),
        "dungeons should have 'Deku Tree' entry"
    );
    assert_eq!(
        dungeons["Deku Tree"],
        Value::String("vanilla".to_string()),
        "Deku Tree should be 'vanilla' in vanilla settings"
    );
}

#[test]
fn test_fixture_parses_as_json() {
    let content = load_fixture("knowledge/default.json");
    let json: Value = serde_json::from_str(&content).expect("should be valid JSON");

    // Verify the JSON has the expected top-level structure
    assert!(json.is_object(), "fixture should be a JSON object");
    let obj = json.as_object().unwrap();

    // Check for expected top-level keys
    let expected_keys = ["settings", "dungeons", "trials", "entrances", "locations"];
    for key in expected_keys {
        assert!(
            obj.contains_key(key),
            "default.json should have '{}' field",
            key
        );
    }
}

#[test]
fn test_load_fixture_bytes() {
    let bytes = load_fixture_bytes("knowledge/default.json");
    assert!(!bytes.is_empty());
    // Verify it starts with a JSON object ('{')
    assert_eq!(bytes[0], b'{');
}
