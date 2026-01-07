//! Common test utilities for oottracker integration tests.
//!
//! This module provides helper functions for loading and working with test fixtures.

use std::path::PathBuf;

/// Returns the path to a fixture file.
///
/// # Arguments
///
/// * `name` - The name of the fixture file (relative to the fixtures directory)
///
/// # Example
///
/// ```ignore
/// let path = fixture_path("knowledge/default.json");
/// ```
pub fn fixture_path(name: &str) -> PathBuf {
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push("tests");
    path.push("fixtures");
    path.push(name);
    path
}

/// Loads a fixture file and returns its contents as a String.
///
/// # Arguments
///
/// * `name` - The name of the fixture file (relative to the fixtures directory)
///
/// # Panics
///
/// Panics if the fixture file does not exist or cannot be read.
///
/// # Example
///
/// ```ignore
/// let content = load_fixture("knowledge/default.json");
/// let knowledge: Knowledge = serde_json::from_str(&content).unwrap();
/// ```
#[allow(dead_code)]
pub fn load_fixture(name: &str) -> String {
    let path = fixture_path(name);
    std::fs::read_to_string(&path)
        .unwrap_or_else(|e| panic!("Failed to load fixture '{}' at {:?}: {}", name, path, e))
}

/// Loads a fixture file and returns its contents as bytes.
///
/// Useful for binary fixtures like save data.
///
/// # Arguments
///
/// * `name` - The name of the fixture file (relative to the fixtures directory)
///
/// # Panics
///
/// Panics if the fixture file does not exist or cannot be read.
#[allow(dead_code)]
pub fn load_fixture_bytes(name: &str) -> Vec<u8> {
    let path = fixture_path(name);
    std::fs::read(&path)
        .unwrap_or_else(|e| panic!("Failed to load fixture '{}' at {:?}: {}", name, path, e))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fixture_path() {
        let path = fixture_path("test.json");
        assert!(path.ends_with("tests/fixtures/test.json"));
        assert!(path.is_absolute() || path.starts_with(env!("CARGO_MANIFEST_DIR")));
    }
}
