//! Integration tests for test fixtures infrastructure.

mod common;

use common::{fixture_path, load_fixture, load_fixture_bytes};

#[test]
fn test_fixture_path_returns_valid_path() {
    let path = fixture_path("knowledge/default.json");
    assert!(path.ends_with("tests/fixtures/knowledge/default.json"));
}

#[test]
fn test_load_fixture_reads_json() {
    let content = load_fixture("knowledge/default.json");
    assert!(content.contains("progression_mode"));
}

#[test]
fn test_load_vanilla_fixture() {
    let content = load_fixture("knowledge/vanilla.json");
    // Verify it contains expected vanilla settings
    assert!(content.contains("open_door_of_time"));
    assert!(content.contains("Deku Tree"));
}

#[test]
fn test_fixture_parses_as_json() {
    let content = load_fixture("knowledge/default.json");
    let _: serde_json::Value = serde_json::from_str(&content).expect("should be valid JSON");
}

#[test]
fn test_load_fixture_bytes() {
    let bytes = load_fixture_bytes("knowledge/default.json");
    assert!(!bytes.is_empty());
    // Verify it starts with a JSON object ('{')
    assert_eq!(bytes[0], b'{');
}
