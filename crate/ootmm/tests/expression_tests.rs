//! Integration tests for expression parsing and evaluation.
//!
//! These tests use YAML fixtures to define test cases, making it easy to add
//! new test cases without writing additional Rust code.

mod common;

use common::{load_expression_fixture, MockEvalContext};
use ootmm::expr::parse;

/// Test helper that runs all test cases from a fixture file.
fn run_fixture_tests(fixture_name: &str) {
    let suite = load_expression_fixture(fixture_name);

    for case in &suite.test_cases {
        let result = parse(&case.expression);

        if case.should_parse {
            assert!(
                result.is_ok(),
                "Test '{}' in suite '{}': expression '{}' should parse but got error: {:?}",
                case.name,
                suite.name,
                case.expression,
                result.err()
            );
        } else {
            assert!(
                result.is_err(),
                "Test '{}' in suite '{}': expression '{}' should fail to parse but succeeded",
                case.name,
                suite.name,
                case.expression
            );
        }
    }
}

// ============================================================================
// Fixture-based tests
// ============================================================================

#[test]
fn test_basic_expressions() {
    run_fixture_tests("basic.yaml");
}

#[test]
fn test_function_expressions() {
    run_fixture_tests("functions.yaml");
}

#[test]
fn test_precedence_expressions() {
    run_fixture_tests("precedence.yaml");
}

// ============================================================================
// Direct integration tests for expression parsing
// ============================================================================

mod parsing {
    use ootmm::expr::{parse, Expr};

    #[test]
    fn test_roundtrip_simple() {
        // Parse -> Display -> Parse should give equivalent result
        let original = "a && b || c";
        let expr1 = parse(original).expect("should parse");
        let displayed = expr1.to_string();
        let expr2 = parse(&displayed).expect("displayed should parse");

        // Note: Display adds parentheses, so we compare structure
        assert_eq!(expr1, expr2, "roundtrip should preserve structure");
    }

    #[test]
    fn test_roundtrip_complex() {
        let original = "(has(HOOKSHOT) || has(LONGSHOT)) && is_adult";
        let expr1 = parse(original).expect("should parse");
        let displayed = expr1.to_string();
        let expr2 = parse(&displayed).expect("displayed should parse");

        assert_eq!(expr1, expr2, "roundtrip should preserve structure");
    }

    #[test]
    fn test_whitespace_handling() {
        // All these should parse to the same expression
        let expressions = ["a&&b", "a && b", "a  &&  b", " a && b ", "a&&b ", " a&&b"];

        let expected = parse("a && b").expect("reference should parse");

        for expr_str in &expressions {
            let result =
                parse(expr_str).unwrap_or_else(|e| panic!("'{}' should parse: {:?}", expr_str, e));
            assert_eq!(
                result, expected,
                "whitespace variation '{}' should match reference",
                expr_str
            );
        }
    }

    #[test]
    fn test_complex_ootmm_expression() {
        // A realistic OoTMM logic expression
        let expr = parse(
            "is_adult && (has(HOOKSHOT) || has(LONGSHOT)) && \
             (event(WATER_TEMPLE_CLEAR) || setting(skip_water_temple))",
        )
        .expect("complex expression should parse");

        // Verify structure
        match expr {
            Expr::And(_, _) => {} // Expected top-level AND
            other => panic!("expected And at top level, got {:?}", other),
        }
    }

    #[test]
    fn test_deeply_nested_expression() {
        // Test parser handles deep nesting without stack overflow
        let expr = "((((a && b) || c) && d) || e)";
        let result = parse(expr);
        assert!(result.is_ok(), "deeply nested expression should parse");
    }

    #[test]
    fn test_many_function_calls() {
        let expr = "has(A) && has(B) && has(C) && has(D) && has(E)";
        let result = parse(expr);
        assert!(result.is_ok(), "many function calls should parse");
    }
}

// ============================================================================
// Mock context tests (for future evaluator integration)
// ============================================================================

mod context {
    use super::*;

    #[test]
    fn test_mock_context_basic() {
        let ctx = MockEvalContext::adult()
            .with_items(["HOOKSHOT", "BOW", "BOMB"])
            .with_event("MIDO_MOVED")
            .with_setting("shuffle_scrubs");

        // Verify context is set up correctly
        use ootmm::expr::EvalContext;
        assert!(ctx.is_adult());
        assert!(!ctx.is_child());
        assert!(ctx.has_item("HOOKSHOT", 1));
        assert!(ctx.has_item("BOW", 1));
        assert!(!ctx.has_item("SLINGSHOT", 1));
        assert!(ctx.event("MIDO_MOVED"));
        assert!(!ctx.event("OTHER_EVENT"));
        assert_eq!(ctx.setting("shuffle_scrubs"), Some(true));
        assert_eq!(ctx.setting("unknown_setting"), None);
    }

    #[test]
    fn test_mock_context_child() {
        let ctx = MockEvalContext::child().with_items(["SLINGSHOT", "BOOMERANG", "DEKU_STICK"]);

        use ootmm::expr::EvalContext;
        assert!(!ctx.is_adult());
        assert!(ctx.is_child());
        assert!(ctx.has_item("SLINGSHOT", 1));
        assert!(ctx.has_item("BOOMERANG", 1));
    }

    #[test]
    fn test_mock_context_item_counts() {
        let mut ctx = MockEvalContext::new();
        ctx.add_item("BOTTLE", 4);

        use ootmm::expr::EvalContext;
        assert!(ctx.has_item("BOTTLE", 1));
        assert!(ctx.has_item("BOTTLE", 2));
        assert!(ctx.has_item("BOTTLE", 3));
        assert!(ctx.has_item("BOTTLE", 4));
        assert!(!ctx.has_item("BOTTLE", 5));
    }
}

// ============================================================================
// Item integration tests
// ============================================================================

mod items {
    use ootmm::{Item, MmItem, OotItem};

    #[test]
    fn test_item_lookup_in_expression_context() {
        // These are items commonly referenced in logic expressions
        let oot_items = ["Hookshot", "Bow", "Bomb", "Boomerang", "MasterSword"];
        // Note: Some masks exist in both games, so use MM-unique items
        let mm_unique_items = [
            "OdolwaRemains",
            "GohtRemains",
            "GyorgRemains",
            "TwinmoldRemains",
        ];

        for name in oot_items {
            let item = Item::by_name(name);
            assert!(item.is_some(), "OoT item '{}' should be found", name);
            assert!(
                matches!(item, Some(Item::Oot(_))),
                "'{}' should be OoT item",
                name
            );
        }

        for name in mm_unique_items {
            let item = Item::by_name(name);
            assert!(item.is_some(), "MM item '{}' should be found", name);
            assert!(
                matches!(item, Some(Item::Mm(_))),
                "'{}' should be MM item",
                name
            );
        }
    }

    #[test]
    fn test_mm_specific_items_via_direct_lookup() {
        // MM items that also exist in OoT can be looked up directly
        let mm_masks = ["DekuMask", "GoronMask", "ZoraMask", "FierceDeityMask"];

        for name in mm_masks {
            let item = MmItem::by_name(name);
            assert!(
                item.is_some(),
                "MM mask '{}' should be found via direct lookup",
                name
            );
        }
    }

    #[test]
    fn test_item_snake_case_lookup() {
        // Logic files often use snake_case
        assert!(Item::by_name("master_sword").is_some());
        assert!(Item::by_name("deku_mask").is_some());
        assert!(Item::by_name("fierce_deity_mask").is_some());
    }

    #[test]
    fn test_item_display_for_expressions() {
        // Ensure items can be displayed in expressions
        let oot = OotItem::Hookshot;
        let mm = MmItem::DekuMask;

        // These should be usable in expressions like has(HOOKSHOT)
        assert!(!format!("{:?}", oot).is_empty());
        assert!(!format!("{:?}", mm).is_empty());
    }
}
