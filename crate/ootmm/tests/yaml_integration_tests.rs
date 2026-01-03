//! Integration tests for expression parsing against real OoTMM YAML logic expressions.
//!
//! These tests verify that the expression parser can handle authentic logic expressions
//! from the OoTMM randomizer, ensuring compatibility with real-world usage patterns.

mod common;

use common::load_expression_fixture;
use ootmm::expr::parse;
use ootmm::world_database::WorldDatabase;

// ============================================================================
// Real OoTMM expression tests from YAML fixture
// ============================================================================

#[test]
fn test_real_ootmm_expressions() {
    let suite = load_expression_fixture("ootmm_real.yaml");

    let mut passed = 0;
    let mut failed = 0;

    for case in &suite.test_cases {
        let result = parse(&case.expression);

        let success = if case.should_parse {
            result.is_ok()
        } else {
            result.is_err()
        };

        if success {
            passed += 1;
        } else {
            failed += 1;
            if case.should_parse {
                eprintln!(
                    "FAIL: '{}' - expression '{}' should parse but got error: {:?}",
                    case.name,
                    case.expression,
                    result.err()
                );
            } else {
                eprintln!(
                    "FAIL: '{}' - expression '{}' should fail but parsed successfully",
                    case.name, case.expression
                );
            }
        }
    }

    assert_eq!(
        failed,
        0,
        "Failed {} out of {} real OoTMM expression tests",
        failed,
        passed + failed
    );
}

// ============================================================================
// WorldDatabase integration with expression parsing
// ============================================================================

mod world_database_integration {
    use super::*;
    use ootmm::region::Game;
    use std::path::PathBuf;

    fn fixture_path(name: &str) -> PathBuf {
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("tests");
        path.push("fixtures");
        path.push(name);
        path
    }

    fn load_sample_world() -> WorldDatabase {
        let mut db = WorldDatabase::new();
        let path = fixture_path("world/sample_world.yaml");
        db.load_from_file(&path)
            .unwrap_or_else(|e| panic!("Failed to load sample world: {}", e));
        db
    }

    #[test]
    fn test_load_sample_world_succeeds() {
        let db = load_sample_world();

        // Verify exact counts from sample_world.yaml fixture
        assert_eq!(db.region_count(), 29, "Sample world fixture has 29 regions");
        assert_eq!(
            db.location_count(),
            48,
            "Sample world fixture has 48 locations"
        );
        assert_eq!(db.event_count(), 6, "Sample world fixture has 6 events");
        assert_eq!(db.exit_count(), 54, "Sample world fixture has 54 exits");
    }

    #[test]
    fn test_sample_world_regions_exist() {
        let db = load_sample_world();

        // OoT regions
        assert!(
            db.has_region("kokiri_forest"),
            "Should have kokiri_forest region"
        );
        assert!(db.has_region("deku_tree"), "Should have deku_tree region");
        assert!(db.has_region("lost_woods"), "Should have lost_woods region");

        // MM regions
        assert!(
            db.has_region("clock_town_south"),
            "Should have clock_town_south region"
        );
        assert!(
            db.has_region("termina_field"),
            "Should have termina_field region"
        );
        assert!(
            db.has_region("woodfall_temple"),
            "Should have woodfall_temple region"
        );
    }

    #[test]
    fn test_sample_world_locations_exist() {
        let db = load_sample_world();

        // OoT locations
        assert!(
            db.has_location("kf_kokiri_sword_chest"),
            "Should have KF sword chest"
        );
        assert!(
            db.has_location("deku_tree_map_chest"),
            "Should have Deku Tree map chest"
        );
        assert!(
            db.has_location("lw_skull_kid"),
            "Should have Lost Woods Skull Kid"
        );

        // MM locations
        assert!(
            db.has_location("ct_clock_tower_entrance"),
            "Should have Clock Tower entrance"
        );
        assert!(
            db.has_location("wt_odolwa_remains"),
            "Should have Odolwa remains"
        );
    }

    #[test]
    fn test_sample_world_events_exist() {
        let db = load_sample_world();

        // OoT events
        assert!(
            db.has_event("SHOWED_MIDO_SWORD_SHIELD"),
            "Should have Mido event"
        );
        assert!(
            db.has_event("DEKU_TREE_CLEAR"),
            "Should have Deku Tree clear event"
        );

        // MM events
        assert!(
            db.has_event("CLOCK_TOWER_OPENED"),
            "Should have Clock Tower opened event"
        );
        assert!(
            db.has_event("WOODFALL_TEMPLE_CLEAR"),
            "Should have Woodfall Temple clear event"
        );
    }

    #[test]
    fn test_sample_world_game_separation() {
        let db = load_sample_world();

        let oot_regions: Vec<_> = db.regions_for_game(Game::Oot).collect();
        let mm_regions: Vec<_> = db.regions_for_game(Game::Mm).collect();

        assert!(!oot_regions.is_empty(), "Should have OoT regions");
        assert!(!mm_regions.is_empty(), "Should have MM regions");

        // Verify correct game assignment
        for region in &oot_regions {
            assert_eq!(region.game, Game::Oot, "Region {} should be OoT", region.id);
        }

        for region in &mm_regions {
            assert_eq!(region.game, Game::Mm, "Region {} should be MM", region.id);
        }
    }

    #[test]
    fn test_sample_world_validates() {
        let db = load_sample_world();
        assert!(db.validate().is_ok(), "Sample world should pass validation");
    }

    #[test]
    fn test_location_logic_expressions_parse() {
        let db = load_sample_world();

        let mut parsed = 0;
        let mut failed = 0;
        let mut failed_exprs = Vec::new();

        for (location, region_id) in db.locations() {
            if let Some(ref logic) = location.logic {
                match parse(logic) {
                    Ok(_) => parsed += 1,
                    Err(e) => {
                        failed += 1;
                        failed_exprs.push(format!(
                            "  {} ({}): '{}' - {}",
                            location.id, region_id, logic, e
                        ));
                    }
                }
            }
        }

        if failed > 0 {
            panic!(
                "Failed to parse {} location logic expressions:\n{}",
                failed,
                failed_exprs.join("\n")
            );
        }

        // Sample world has 48 locations all with logic expressions
        assert_eq!(
            parsed, 48,
            "Sample world fixture should have 48 locations with logic"
        );
    }

    #[test]
    fn test_exit_logic_expressions_parse() {
        let db = load_sample_world();

        let mut parsed = 0;
        let mut failed = 0;
        let mut failed_exprs = Vec::new();

        for (exit, region_id) in db.exits() {
            if let Some(ref logic) = exit.logic {
                match parse(logic) {
                    Ok(_) => parsed += 1,
                    Err(e) => {
                        failed += 1;
                        failed_exprs.push(format!(
                            "  {} -> {} ({}): '{}' - {}",
                            region_id, exit.target, region_id, logic, e
                        ));
                    }
                }
            }
        }

        if failed > 0 {
            panic!(
                "Failed to parse {} exit logic expressions:\n{}",
                failed,
                failed_exprs.join("\n")
            );
        }

        // Sample world has 54 exits all with logic expressions
        assert_eq!(
            parsed, 54,
            "Sample world fixture should have 54 exits with logic"
        );
    }

    #[test]
    fn test_event_logic_expressions_parse() {
        let db = load_sample_world();

        let mut parsed = 0;
        let mut failed = 0;
        let mut failed_exprs = Vec::new();

        for (event, region_id) in db.events() {
            if let Some(ref logic) = event.logic {
                match parse(logic) {
                    Ok(_) => parsed += 1,
                    Err(e) => {
                        failed += 1;
                        failed_exprs.push(format!(
                            "  {} ({}): '{}' - {}",
                            event.id, region_id, logic, e
                        ));
                    }
                }
            }
        }

        if failed > 0 {
            panic!(
                "Failed to parse {} event logic expressions:\n{}",
                failed,
                failed_exprs.join("\n")
            );
        }

        // Sample world has 6 events all with logic expressions
        assert_eq!(
            parsed, 6,
            "Sample world fixture should have 6 events with logic"
        );
    }

    #[test]
    fn test_all_logic_expressions_parse() {
        let db = load_sample_world();

        let mut total = 0;
        let mut failed = 0;

        // Count all logic expressions
        for (location, _) in db.locations() {
            if location.logic.is_some() {
                total += 1;
            }
        }
        for (exit, _) in db.exits() {
            if exit.logic.is_some() {
                total += 1;
            }
        }
        for (event, _) in db.events() {
            if event.logic.is_some() {
                total += 1;
            }
        }

        // Verify all parse (detailed errors from individual tests)
        for (location, _) in db.locations() {
            if let Some(ref logic) = location.logic {
                if parse(logic).is_err() {
                    failed += 1;
                }
            }
        }
        for (exit, _) in db.exits() {
            if let Some(ref logic) = exit.logic {
                if parse(logic).is_err() {
                    failed += 1;
                }
            }
        }
        for (event, _) in db.events() {
            if let Some(ref logic) = event.logic {
                if parse(logic).is_err() {
                    failed += 1;
                }
            }
        }

        assert_eq!(
            failed, 0,
            "All {} logic expressions should parse successfully",
            total
        );
    }
}

// ============================================================================
// Expression structure verification tests
// ============================================================================

mod expression_structure {
    use super::*;
    use ootmm::expr::Expr;

    #[test]
    fn test_can_use_function_structure() {
        let expr = parse("can_use(Hookshot)").expect("should parse");

        match expr {
            Expr::Call { ref name, ref args } => {
                assert_eq!(name, "can_use");
                assert_eq!(args.len(), 1);
                assert!(matches!(&args[0], Expr::Ident(s) if s == "Hookshot"));
            }
            _ => panic!("Expected function call, got {:?}", expr),
        }
    }

    #[test]
    fn test_has_function_with_count_structure() {
        let expr = parse("has(Bottle, 2)").expect("should parse");

        match expr {
            Expr::Call { ref name, ref args } => {
                assert_eq!(name, "has");
                assert_eq!(args.len(), 2);
                assert!(matches!(&args[0], Expr::Ident(s) if s == "Bottle"));
                assert!(matches!(&args[1], Expr::Number(2)));
            }
            _ => panic!("Expected function call, got {:?}", expr),
        }
    }

    #[test]
    fn test_complex_expression_structure() {
        let expr = parse("(is_adult && has(Hookshot)) || (is_child && can_use(Boomerang))")
            .expect("should parse");

        // Top level should be Or
        match expr {
            Expr::Or(left, right) => {
                // Left side: (is_adult && has(Hookshot))
                match left.as_ref() {
                    Expr::And(adult, hookshot) => {
                        assert!(matches!(adult.as_ref(), Expr::Ident(s) if s == "is_adult"));
                        assert!(
                            matches!(hookshot.as_ref(), Expr::Call { name, .. } if name == "has")
                        );
                    }
                    _ => panic!("Expected And on left side"),
                }

                // Right side: (is_child && can_use(Boomerang))
                match right.as_ref() {
                    Expr::And(child, boomerang) => {
                        assert!(matches!(child.as_ref(), Expr::Ident(s) if s == "is_child"));
                        assert!(
                            matches!(boomerang.as_ref(), Expr::Call { name, .. } if name == "can_use")
                        );
                    }
                    _ => panic!("Expected And on right side"),
                }
            }
            _ => panic!("Expected Or at top level, got {:?}", expr),
        }
    }

    #[test]
    fn test_nested_function_calls() {
        // This tests a pattern that might appear in complex logic
        let expr = parse("has(Small_Key_Forest_Temple, 5)").expect("should parse");

        match expr {
            Expr::Call { ref name, ref args } => {
                assert_eq!(name, "has");
                assert_eq!(args.len(), 2);
                assert!(matches!(&args[0], Expr::Ident(s) if s == "Small_Key_Forest_Temple"));
                assert!(matches!(&args[1], Expr::Number(5)));
            }
            _ => panic!("Expected function call"),
        }
    }

    #[test]
    fn test_not_expression_structure() {
        let expr = parse("!setting(shuffle_scrubs)").expect("should parse");

        match expr {
            Expr::Not(inner) => {
                assert!(matches!(inner.as_ref(), Expr::Call { name, .. } if name == "setting"));
            }
            _ => panic!("Expected Not expression"),
        }
    }

    #[test]
    fn test_chained_and_structure() {
        let expr = parse("a && b && c && d").expect("should parse");

        // Should be left-associative: ((a && b) && c) && d
        match expr {
            Expr::And(left, d) => {
                assert!(matches!(d.as_ref(), Expr::Ident(s) if s == "d"));
                match left.as_ref() {
                    Expr::And(left2, c) => {
                        assert!(matches!(c.as_ref(), Expr::Ident(s) if s == "c"));
                        match left2.as_ref() {
                            Expr::And(a, b) => {
                                assert!(matches!(a.as_ref(), Expr::Ident(s) if s == "a"));
                                assert!(matches!(b.as_ref(), Expr::Ident(s) if s == "b"));
                            }
                            _ => panic!("Expected innermost And"),
                        }
                    }
                    _ => panic!("Expected middle And"),
                }
            }
            _ => panic!("Expected outer And"),
        }
    }

    #[test]
    fn test_mixed_precedence_structure() {
        // a || b && c should be a || (b && c) because && has higher precedence
        let expr = parse("a || b && c").expect("should parse");

        match expr {
            Expr::Or(a, bc) => {
                assert!(matches!(a.as_ref(), Expr::Ident(s) if s == "a"));
                match bc.as_ref() {
                    Expr::And(b, c) => {
                        assert!(matches!(b.as_ref(), Expr::Ident(s) if s == "b"));
                        assert!(matches!(c.as_ref(), Expr::Ident(s) if s == "c"));
                    }
                    _ => panic!("Expected And on right side of Or"),
                }
            }
            _ => panic!("Expected Or at top level"),
        }
    }
}

// ============================================================================
// Roundtrip tests - parse -> display -> parse
// ============================================================================

mod roundtrip {
    use super::*;

    fn test_roundtrip(expr_str: &str) {
        let expr1 =
            parse(expr_str).expect(&format!("First parse of '{}' should succeed", expr_str));
        let displayed = expr1.to_string();
        let expr2 = parse(&displayed).expect(&format!(
            "Second parse of displayed '{}' (from '{}') should succeed",
            displayed, expr_str
        ));

        assert_eq!(
            expr1, expr2,
            "Roundtrip should preserve structure for '{}' (displayed as '{}')",
            expr_str, displayed
        );
    }

    #[test]
    fn test_roundtrip_simple_identifiers() {
        test_roundtrip("is_adult");
        test_roundtrip("is_child");
        test_roundtrip("Hookshot");
    }

    #[test]
    fn test_roundtrip_literals() {
        test_roundtrip("true");
        test_roundtrip("false");
        test_roundtrip("42");
    }

    #[test]
    fn test_roundtrip_function_calls() {
        test_roundtrip("has(Hookshot)");
        test_roundtrip("can_use(Bow)");
        test_roundtrip("event(MIDO_MOVED)");
        test_roundtrip("setting(shuffle_scrubs)");
        test_roundtrip("trick(logic_deku_b1_skip)");
    }

    #[test]
    fn test_roundtrip_function_with_multiple_args() {
        test_roundtrip("has(Bottle, 2)");
        test_roundtrip("has(Small_Key_Forest_Temple, 5)");
    }

    #[test]
    fn test_roundtrip_binary_expressions() {
        test_roundtrip("a && b");
        test_roundtrip("a || b");
        test_roundtrip("a && b && c");
        test_roundtrip("a || b || c");
    }

    #[test]
    fn test_roundtrip_not_expressions() {
        test_roundtrip("!a");
        test_roundtrip("!!a");
        test_roundtrip("!is_adult");
    }

    #[test]
    fn test_roundtrip_complex_expressions() {
        test_roundtrip("(is_adult && has(Hookshot)) || (is_child && can_use(Boomerang))");
        test_roundtrip("has(Zora_Tunic) && has(Iron_Boots) && (has(Hookshot) || has(Longshot))");
        test_roundtrip("event(DEKU_TREE_CLEAR) || setting(skip_child_zelda)");
    }

    #[test]
    fn test_roundtrip_real_logic_expressions() {
        // Real expressions from OoTMM logic files
        test_roundtrip("is_child && at_night && can_child_attack");
        test_roundtrip("has(Deku_Mask) || has(Hookshot)");
        test_roundtrip("has(Goron_Mask) || (has(Bow) && has(Fire_Arrows))");
        test_roundtrip("has(Boss_Key_Woodfall_Temple) && (has(Bow) || has(Deku_Mask))");
    }
}

// ============================================================================
// Statistics and coverage tests
// ============================================================================

mod statistics {
    use super::*;
    use ootmm::region::Game;

    #[test]
    fn test_expression_fixture_coverage() {
        let suite = load_expression_fixture("ootmm_real.yaml");

        // Count expression types
        let mut simple_idents = 0;
        let mut function_calls = 0;
        let mut compound_expressions = 0;
        let mut error_cases = 0;

        for case in &suite.test_cases {
            if !case.should_parse {
                error_cases += 1;
            } else if case.expression.contains('(') && case.expression.contains(')') {
                if case.expression.contains("&&") || case.expression.contains("||") {
                    compound_expressions += 1;
                } else {
                    function_calls += 1;
                }
            } else if case.expression.contains("&&") || case.expression.contains("||") {
                compound_expressions += 1;
            } else {
                simple_idents += 1;
            }
        }

        let total = suite.test_cases.len();

        // Verify exact counts from ootmm_real.yaml fixture (87 total test cases)
        assert_eq!(
            simple_idents, 16,
            "ootmm_real.yaml fixture has 16 simple identifier tests"
        );
        assert_eq!(
            function_calls, 33,
            "ootmm_real.yaml fixture has 33 function call tests"
        );
        assert_eq!(
            compound_expressions, 29,
            "ootmm_real.yaml fixture has 29 compound expression tests"
        );
        assert_eq!(
            error_cases, 9,
            "ootmm_real.yaml fixture has 9 error case tests"
        );

        // Verify all categories sum correctly
        let categorized = simple_idents + function_calls + compound_expressions + error_cases;
        assert_eq!(
            categorized, total,
            "All test cases should be categorized: {} categorized vs {} total",
            categorized, total
        );
    }

    #[test]
    fn test_world_database_statistics() {
        let mut db = WorldDatabase::new();
        let path = {
            let mut p = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));
            p.push("tests/fixtures/world/sample_world.yaml");
            p
        };
        db.load_from_file(&path).unwrap();

        // Verify structural counts
        assert!(
            db.region_count() >= 10,
            "Should have at least 10 regions, got {}",
            db.region_count()
        );
        assert!(
            db.location_count() >= 20,
            "Should have at least 20 locations, got {}",
            db.location_count()
        );
        assert!(
            db.event_count() >= 5,
            "Should have at least 5 events, got {}",
            db.event_count()
        );
        assert!(
            db.exit_count() >= 10,
            "Should have at least 10 exits, got {}",
            db.exit_count()
        );

        // Verify game distribution
        let oot_count = db.regions_for_game(Game::Oot).count();
        let mm_count = db.regions_for_game(Game::Mm).count();
        assert!(
            oot_count >= 3,
            "Should have at least 3 OoT regions, got {}",
            oot_count
        );
        assert!(
            mm_count >= 3,
            "Should have at least 3 MM regions, got {}",
            mm_count
        );
        assert_eq!(
            oot_count + mm_count,
            db.region_count(),
            "All regions should be assigned to OoT or MM"
        );

        // Verify logic expression coverage
        let loc_with_logic = db.locations().filter(|(l, _)| l.logic.is_some()).count();
        let exit_with_logic = db.exits().filter(|(e, _)| e.logic.is_some()).count();
        let event_with_logic = db.events().filter(|(e, _)| e.logic.is_some()).count();

        println!("  Locations with logic: {}", loc_with_logic);
        println!("  Exits with logic: {}", exit_with_logic);
        println!("  Events with logic: {}", event_with_logic);

        // Verify exact counts from sample_world.yaml fixture
        assert_eq!(db.region_count(), 29, "Sample world fixture has 29 regions");
        assert_eq!(
            db.location_count(),
            48,
            "Sample world fixture has 48 locations"
        );
        assert_eq!(
            loc_with_logic, 48,
            "Sample world fixture has 48 locations with logic"
        );
        assert_eq!(
            exit_with_logic, 54,
            "Sample world fixture has 54 exits with logic"
        );
        assert_eq!(
            event_with_logic, 6,
            "Sample world fixture has 6 events with logic"
        );
    }
}
