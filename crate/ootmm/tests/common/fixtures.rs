//! Test fixture utilities for ootmm integration tests.

use std::path::PathBuf;

/// Returns the path to a fixture file relative to the tests/fixtures directory.
///
/// # Arguments
///
/// * `name` - The name of the fixture file (relative to the fixtures directory)
///
/// # Example
///
/// ```ignore
/// let path = fixture_path("expressions/basic.yaml");
/// ```
#[must_use]
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
/// let content = load_fixture("expressions/basic.yaml");
/// ```
pub fn load_fixture(name: &str) -> String {
    let path = fixture_path(name);
    std::fs::read_to_string(&path)
        .unwrap_or_else(|e| panic!("Failed to load fixture '{}' at {:?}: {}", name, path, e))
}

/// A test case for expression parsing and/or evaluation.
///
/// This struct is used to define test cases in YAML fixtures.
#[derive(Debug, Clone, serde::Deserialize)]
pub struct ExpressionTestCase {
    /// Human-readable name for the test case.
    pub name: String,
    /// The expression string to parse.
    pub expression: String,
    /// Whether parsing should succeed.
    #[serde(default = "default_true")]
    pub should_parse: bool,
    /// Expected result when evaluated (if applicable).
    /// Reserved for future evaluation tests.
    #[serde(default)]
    #[allow(dead_code)]
    pub expected_result: Option<bool>,
    /// Description of what this test case is checking.
    #[serde(default)]
    #[allow(dead_code)]
    pub description: Option<String>,
}

fn default_true() -> bool {
    true
}

/// Container for a collection of expression test cases.
#[derive(Debug, serde::Deserialize)]
pub struct ExpressionTestSuite {
    /// Name of the test suite.
    pub name: String,
    /// Description of what this suite tests.
    #[serde(default)]
    #[allow(dead_code)]
    pub description: Option<String>,
    /// The test cases in this suite.
    pub test_cases: Vec<ExpressionTestCase>,
}

/// Loads an expression test suite from a YAML fixture file.
///
/// # Arguments
///
/// * `name` - The name of the fixture file (relative to fixtures/expressions/)
///
/// # Panics
///
/// Panics if the fixture file cannot be loaded or parsed.
///
/// # Example
///
/// ```ignore
/// let suite = load_expression_fixture("basic.yaml");
/// for case in &suite.test_cases {
///     let result = parse(&case.expression);
///     assert_eq!(result.is_ok(), case.should_parse);
/// }
/// ```
pub fn load_expression_fixture(name: &str) -> ExpressionTestSuite {
    let path = format!("expressions/{}", name);
    let content = load_fixture(&path);
    serde_yaml::from_str(&content)
        .unwrap_or_else(|e| panic!("Failed to parse expression fixture '{}': {}", name, e))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fixture_path() {
        let path = fixture_path("expressions/basic.yaml");
        assert!(path.ends_with("tests/fixtures/expressions/basic.yaml"));
    }

    #[test]
    fn test_fixture_path_is_absolute_or_relative_to_manifest() {
        let path = fixture_path("test.yaml");
        // Path should contain CARGO_MANIFEST_DIR
        let manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        assert!(path.starts_with(&manifest_dir));
    }
}
