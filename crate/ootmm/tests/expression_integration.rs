//! Integration tests for the expression parser against real YAML world data.
//!
//! These tests verify that 100% of logic expressions in the world fixtures
//! can be successfully parsed by the expression parser.

use std::path::PathBuf;

use ootmm::expr::parse;
use ootmm::world_database::WorldDatabase;

/// Get the path to a fixture file.
fn fixture_path(name: &str) -> PathBuf {
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push("tests");
    path.push("fixtures");
    path.push(name);
    path
}

/// Load the sample world database from fixtures.
fn load_world_database() -> WorldDatabase {
    let mut db = WorldDatabase::new();
    let path = fixture_path("world");
    db.load_from_directory(&path)
        .unwrap_or_else(|e| panic!("Failed to load world database from {:?}: {}", path, e));
    db
}

/// Represents a parse failure for reporting.
#[derive(Debug)]
struct ParseFailure {
    /// The type of element (location, exit, or event).
    element_type: &'static str,
    /// The ID of the element.
    element_id: String,
    /// The region containing this element.
    region_id: String,
    /// The logic expression that failed to parse.
    logic: String,
    /// The parse error message.
    error: String,
}

impl std::fmt::Display for ParseFailure {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "  [{}] {} in region '{}': '{}' - {}",
            self.element_type, self.element_id, self.region_id, self.logic, self.error
        )
    }
}

/// Test that 100% of location logic expressions parse successfully.
#[test]
fn test_all_location_logic_expressions_parse() {
    let db = load_world_database();
    let mut failures: Vec<ParseFailure> = Vec::new();
    let mut total = 0;

    for (location, region_id) in db.locations() {
        if let Some(ref logic) = location.logic {
            total += 1;
            if let Err(e) = parse(logic) {
                failures.push(ParseFailure {
                    element_type: "location",
                    element_id: location.id.clone(),
                    region_id: region_id.to_string(),
                    logic: logic.clone(),
                    error: e.to_string(),
                });
            }
        }
    }

    if !failures.is_empty() {
        let failure_report: String = failures
            .iter()
            .map(|f| f.to_string())
            .collect::<Vec<_>>()
            .join("\n");
        panic!(
            "Failed to parse {} out of {} location logic expressions:\n{}",
            failures.len(),
            total,
            failure_report
        );
    }

    assert!(
        total > 0,
        "Expected at least one location with logic expression"
    );
    println!("Successfully parsed {} location logic expressions", total);
}

/// Test that 100% of exit logic expressions parse successfully.
#[test]
fn test_all_exit_logic_expressions_parse() {
    let db = load_world_database();
    let mut failures: Vec<ParseFailure> = Vec::new();
    let mut total = 0;

    for (exit, region_id) in db.exits() {
        if let Some(ref logic) = exit.logic {
            total += 1;
            if let Err(e) = parse(logic) {
                failures.push(ParseFailure {
                    element_type: "exit",
                    element_id: format!("{} -> {}", region_id, exit.target),
                    region_id: region_id.to_string(),
                    logic: logic.clone(),
                    error: e.to_string(),
                });
            }
        }
    }

    if !failures.is_empty() {
        let failure_report: String = failures
            .iter()
            .map(|f| f.to_string())
            .collect::<Vec<_>>()
            .join("\n");
        panic!(
            "Failed to parse {} out of {} exit logic expressions:\n{}",
            failures.len(),
            total,
            failure_report
        );
    }

    assert!(
        total > 0,
        "Expected at least one exit with logic expression"
    );
    println!("Successfully parsed {} exit logic expressions", total);
}

/// Test that 100% of event logic expressions parse successfully.
#[test]
fn test_all_event_logic_expressions_parse() {
    let db = load_world_database();
    let mut failures: Vec<ParseFailure> = Vec::new();
    let mut total = 0;

    for (event, region_id) in db.events() {
        if let Some(ref logic) = event.logic {
            total += 1;
            if let Err(e) = parse(logic) {
                failures.push(ParseFailure {
                    element_type: "event",
                    element_id: event.id.clone(),
                    region_id: region_id.to_string(),
                    logic: logic.clone(),
                    error: e.to_string(),
                });
            }
        }
    }

    if !failures.is_empty() {
        let failure_report: String = failures
            .iter()
            .map(|f| f.to_string())
            .collect::<Vec<_>>()
            .join("\n");
        panic!(
            "Failed to parse {} out of {} event logic expressions:\n{}",
            failures.len(),
            total,
            failure_report
        );
    }

    assert!(
        total > 0,
        "Expected at least one event with logic expression"
    );
    println!("Successfully parsed {} event logic expressions", total);
}

/// Comprehensive test that verifies 100% of ALL logic expressions parse.
///
/// This test aggregates locations, exits, and events and reports all failures
/// in a single comprehensive report.
#[test]
fn test_all_logic_expressions_parse() {
    let db = load_world_database();
    let mut failures: Vec<ParseFailure> = Vec::new();
    let mut total = 0;

    // Collect all location logic expressions
    for (location, region_id) in db.locations() {
        if let Some(ref logic) = location.logic {
            total += 1;
            if let Err(e) = parse(logic) {
                failures.push(ParseFailure {
                    element_type: "location",
                    element_id: location.id.clone(),
                    region_id: region_id.to_string(),
                    logic: logic.clone(),
                    error: e.to_string(),
                });
            }
        }
    }

    // Collect all exit logic expressions
    for (exit, region_id) in db.exits() {
        if let Some(ref logic) = exit.logic {
            total += 1;
            if let Err(e) = parse(logic) {
                failures.push(ParseFailure {
                    element_type: "exit",
                    element_id: format!("{} -> {}", region_id, exit.target),
                    region_id: region_id.to_string(),
                    logic: logic.clone(),
                    error: e.to_string(),
                });
            }
        }
    }

    // Collect all event logic expressions
    for (event, region_id) in db.events() {
        if let Some(ref logic) = event.logic {
            total += 1;
            if let Err(e) = parse(logic) {
                failures.push(ParseFailure {
                    element_type: "event",
                    element_id: event.id.clone(),
                    region_id: region_id.to_string(),
                    logic: logic.clone(),
                    error: e.to_string(),
                });
            }
        }
    }

    // Report results
    let passed = total - failures.len();
    let pass_rate = if total > 0 {
        (passed as f64 / total as f64) * 100.0
    } else {
        0.0
    };

    if !failures.is_empty() {
        let failure_report: String = failures
            .iter()
            .map(|f| f.to_string())
            .collect::<Vec<_>>()
            .join("\n");
        panic!(
            "Expression parser integration test failed!\n\
             Pass rate: {:.1}% ({}/{} expressions)\n\
             \n\
             Failed expressions:\n{}",
            pass_rate, passed, total, failure_report
        );
    }

    assert!(
        total > 0,
        "Expected at least one logic expression in the world database"
    );
    println!(
        "All {} logic expressions parsed successfully (100% pass rate)",
        total
    );
}

/// Test that the world database loads correctly.
#[test]
fn test_world_database_loads() {
    let db = load_world_database();

    // Verify we have data
    assert!(
        db.region_count() > 0,
        "Should have loaded at least one region"
    );
    assert!(
        db.location_count() > 0,
        "Should have loaded at least one location"
    );

    println!(
        "Loaded world database: {} regions, {} locations, {} exits, {} events",
        db.region_count(),
        db.location_count(),
        db.exit_count(),
        db.event_count()
    );
}

/// Test expression parsing statistics.
#[test]
fn test_expression_statistics() {
    let db = load_world_database();

    let mut location_logic_count = 0;
    let mut exit_logic_count = 0;
    let mut event_logic_count = 0;

    for (location, _) in db.locations() {
        if location.logic.is_some() {
            location_logic_count += 1;
        }
    }

    for (exit, _) in db.exits() {
        if exit.logic.is_some() {
            exit_logic_count += 1;
        }
    }

    for (event, _) in db.events() {
        if event.logic.is_some() {
            event_logic_count += 1;
        }
    }

    let total = location_logic_count + exit_logic_count + event_logic_count;

    println!("Expression statistics:");
    println!("  Location logic expressions: {}", location_logic_count);
    println!("  Exit logic expressions: {}", exit_logic_count);
    println!("  Event logic expressions: {}", event_logic_count);
    println!("  Total logic expressions: {}", total);

    assert!(
        location_logic_count > 0,
        "Should have locations with logic expressions"
    );
    assert!(
        exit_logic_count > 0,
        "Should have exits with logic expressions"
    );
    assert!(
        event_logic_count > 0,
        "Should have events with logic expressions"
    );
}
