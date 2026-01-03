//! Semantic equivalence tests for roundtrip operations.
//!
//! These tests go beyond structural equality to verify that roundtripped
//! data behaves identically to the original. This catches subtle bugs where
//! serialization/deserialization preserves structure but loses semantic meaning.
//!
//! Key improvements over structural-only roundtrip tests:
//! 1. Expression roundtrips are tested by evaluating against multiple contexts
//! 2. Item roundtrips verify functional properties are preserved
//! 3. Edge cases test normalization behavior (whitespace, parentheses, etc.)

mod common;

use common::MockEvalContext;
use ootmm::expr::{eval, parse, Expr};

// ============================================================================
// Test Context Generation
// ============================================================================

/// Generates a variety of test contexts to verify semantic equivalence.
/// Returns contexts that cover different combinations of game state.
fn test_contexts() -> Vec<MockEvalContext> {
    vec![
        // Empty context (baseline)
        MockEvalContext::new(),
        // Adult with common items
        MockEvalContext::adult()
            .with_items(["HOOKSHOT", "BOW", "BOMB"])
            .with_event("MIDO_MOVED"),
        // Child with child items
        MockEvalContext::child()
            .with_items(["BOOMERANG", "SLINGSHOT", "DEKU_STICK"])
            .with_event("MET_ZELDA"),
        // Adult with settings and tricks
        MockEvalContext::adult()
            .with_items(["HOOKSHOT", "LONGSHOT"])
            .with_setting("shuffle_scrubs")
            .with_trick("logic_deku_b1_skip"),
        // Context with many items
        MockEvalContext::adult()
            .with_items([
                "HOOKSHOT",
                "LONGSHOT",
                "BOW",
                "BOMB",
                "BOOMERANG",
                "SLINGSHOT",
                "MASTER_SWORD",
            ])
            .with_event("FOREST_TEMPLE_CLEAR")
            .with_event("WATER_TEMPLE_CLEAR")
            .with_setting("skip_child_zelda"),
        // Context with only events
        MockEvalContext::new()
            .with_event("MIDO_MOVED")
            .with_event("DEKU_TREE_CLEAR"),
        // Context with only settings
        MockEvalContext::new()
            .with_setting("shuffle_scrubs")
            .with_setting("skip_child_zelda"),
    ]
}

// ============================================================================
// Semantic Equivalence Helpers
// ============================================================================

/// Tests that two expressions are semantically equivalent by evaluating them
/// against all test contexts and verifying identical results.
fn assert_semantically_equivalent(expr1: &Expr, expr2: &Expr, description: &str) {
    for (i, ctx) in test_contexts().iter().enumerate() {
        let result1 = eval(expr1, ctx);
        let result2 = eval(expr2, ctx);

        match (&result1, &result2) {
            (Ok(v1), Ok(v2)) => {
                assert_eq!(
                    v1, v2,
                    "{}: expressions differ in context #{}: {:?} vs {:?}",
                    description, i, v1, v2
                );
            }
            (Err(_), Err(_)) => {
                // Both errored, consider equivalent for this context
            }
            _ => {
                panic!(
                    "{}: one expression errored in context #{}: {:?} vs {:?}",
                    description, i, result1, result2
                );
            }
        }
    }
}

/// Tests roundtrip with semantic equivalence verification.
/// Parses, displays, re-parses, then verifies both semantic and structural equivalence.
fn test_semantic_roundtrip(expr_str: &str) {
    let expr1 = parse(expr_str).unwrap_or_else(|e| {
        panic!("First parse of '{}' should succeed: {:?}", expr_str, e);
    });

    let displayed = expr1.to_string();

    let expr2 = parse(&displayed).unwrap_or_else(|e| {
        panic!(
            "Second parse of displayed '{}' (from '{}') should succeed: {:?}",
            displayed, expr_str, e
        );
    });

    // Structural equality check
    assert_eq!(
        expr1, expr2,
        "Roundtrip should preserve structure for '{}' (displayed as '{}')",
        expr_str, displayed
    );

    // Semantic equivalence check - verify both evaluate identically
    assert_semantically_equivalent(
        &expr1,
        &expr2,
        &format!("Roundtrip semantic check for '{}'", expr_str),
    );
}

// ============================================================================
// Expression Roundtrip with Semantic Verification
// ============================================================================

mod expression_roundtrip_semantic {
    use super::*;

    #[test]
    fn test_boolean_literals_semantic() {
        test_semantic_roundtrip("true");
        test_semantic_roundtrip("false");
    }

    #[test]
    fn test_identifiers_semantic() {
        test_semantic_roundtrip("is_adult");
        test_semantic_roundtrip("is_child");
    }

    #[test]
    fn test_has_function_semantic() {
        test_semantic_roundtrip("has(HOOKSHOT)");
        test_semantic_roundtrip("has(BOW)");
        test_semantic_roundtrip("has(BOMB)");
        test_semantic_roundtrip("has(BOOMERANG)");
    }

    #[test]
    fn test_event_function_semantic() {
        test_semantic_roundtrip("event(MIDO_MOVED)");
        test_semantic_roundtrip("event(FOREST_TEMPLE_CLEAR)");
        test_semantic_roundtrip("event(DEKU_TREE_CLEAR)");
    }

    #[test]
    fn test_setting_function_semantic() {
        test_semantic_roundtrip("setting(shuffle_scrubs)");
        test_semantic_roundtrip("setting(skip_child_zelda)");
    }

    #[test]
    fn test_trick_function_semantic() {
        test_semantic_roundtrip("trick(logic_deku_b1_skip)");
    }

    #[test]
    fn test_and_expressions_semantic() {
        test_semantic_roundtrip("true && true");
        test_semantic_roundtrip("true && false");
        test_semantic_roundtrip("is_adult && has(HOOKSHOT)");
        test_semantic_roundtrip("has(HOOKSHOT) && has(BOW)");
    }

    #[test]
    fn test_or_expressions_semantic() {
        test_semantic_roundtrip("true || false");
        test_semantic_roundtrip("has(HOOKSHOT) || has(LONGSHOT)");
        test_semantic_roundtrip("is_adult || is_child");
    }

    #[test]
    fn test_not_expressions_semantic() {
        test_semantic_roundtrip("!true");
        test_semantic_roundtrip("!false");
        test_semantic_roundtrip("!is_adult");
        test_semantic_roundtrip("!has(HOOKSHOT)");
        test_semantic_roundtrip("!!true");
    }

    #[test]
    fn test_complex_expressions_semantic() {
        test_semantic_roundtrip("(has(HOOKSHOT) || has(LONGSHOT)) && is_adult");
        test_semantic_roundtrip("is_adult && has(HOOKSHOT) && event(MIDO_MOVED)");
        test_semantic_roundtrip("(is_adult && has(BOW)) || (is_child && has(SLINGSHOT))");
        test_semantic_roundtrip("event(FOREST_TEMPLE_CLEAR) || setting(skip_child_zelda)");
    }

    #[test]
    fn test_deeply_nested_semantic() {
        test_semantic_roundtrip("((a && b) || c) && d");
        test_semantic_roundtrip("(((true && false) || true) && false) || true");
    }

    #[test]
    fn test_real_ootmm_logic_semantic() {
        // Real expressions from OoTMM logic files
        test_semantic_roundtrip("is_child && has(BOOMERANG)");
        test_semantic_roundtrip("has(HOOKSHOT) || has(LONGSHOT)");
        test_semantic_roundtrip("(has(BOW) && is_adult) || trick(logic_deku_b1_skip)");
    }
}

// ============================================================================
// Whitespace Normalization Semantic Tests
// ============================================================================

mod whitespace_normalization {
    use super::*;

    /// Tests that different whitespace variations parse to semantically equivalent expressions.
    fn test_whitespace_equivalence(expressions: &[&str]) {
        let parsed: Vec<_> = expressions
            .iter()
            .map(|s| parse(s).expect(&format!("'{}' should parse", s)))
            .collect();

        // All should be structurally equal
        for (i, expr) in parsed.iter().enumerate().skip(1) {
            assert_eq!(
                &parsed[0], expr,
                "Whitespace variant {} '{}' should equal '{}' structurally",
                i, expressions[i], expressions[0]
            );
        }

        // All should be semantically equivalent
        for (i, expr) in parsed.iter().enumerate().skip(1) {
            assert_semantically_equivalent(
                &parsed[0],
                expr,
                &format!(
                    "Whitespace variant '{}' vs '{}'",
                    expressions[0], expressions[i]
                ),
            );
        }
    }

    #[test]
    fn test_and_whitespace_variations() {
        test_whitespace_equivalence(&["a && b", "a&&b", "a  &&  b", " a && b ", "a && b"]);
    }

    #[test]
    fn test_or_whitespace_variations() {
        test_whitespace_equivalence(&["a || b", "a||b", "a  ||  b", " a || b "]);
    }

    #[test]
    fn test_function_call_whitespace() {
        test_whitespace_equivalence(&["has(HOOKSHOT)", "has( HOOKSHOT )", "has(  HOOKSHOT  )"]);
    }

    #[test]
    fn test_complex_whitespace_variations() {
        test_whitespace_equivalence(&[
            "has(HOOKSHOT) && is_adult",
            "has(HOOKSHOT)&&is_adult",
            "has( HOOKSHOT ) && is_adult",
            "  has(HOOKSHOT)  &&  is_adult  ",
        ]);
    }
}

// ============================================================================
// Parentheses Normalization Tests
// ============================================================================

mod parentheses_normalization {
    use super::*;

    /// Tests that expressions with different (but logically valid) parenthesization
    /// are semantically equivalent.
    #[test]
    fn test_extra_parentheses_semantic() {
        let base = parse("a && b").unwrap();
        let extra = parse("(a && b)").unwrap();
        let double = parse("((a && b))").unwrap();

        assert_semantically_equivalent(&base, &extra, "extra parens around and");
        assert_semantically_equivalent(&base, &double, "double extra parens around and");
    }

    #[test]
    fn test_nested_parentheses_roundtrip() {
        // Display adds parentheses, but semantic meaning should be preserved
        let original = "a && b || c";
        let expr1 = parse(original).unwrap();
        let displayed = expr1.to_string();
        let expr2 = parse(&displayed).unwrap();

        assert_semantically_equivalent(
            &expr1,
            &expr2,
            "parentheses normalization preserves semantics",
        );
    }

    #[test]
    fn test_redundant_parentheses_semantic() {
        let expr1 = parse("(true)").unwrap();
        let expr2 = parse("true").unwrap();
        assert_semantically_equivalent(&expr1, &expr2, "redundant parens on literal");
    }
}

// ============================================================================
// Edge Cases and Boundary Conditions
// ============================================================================

mod edge_cases {
    use super::*;

    #[test]
    fn test_single_variable_semantic() {
        // Single variables should roundtrip and maintain evaluation behavior
        for var in ["x", "foo", "is_adult", "is_child", "unknown_var"] {
            test_semantic_roundtrip(var);
        }
    }

    #[test]
    fn test_number_literals_semantic() {
        test_semantic_roundtrip("42");
        test_semantic_roundtrip("0");
        test_semantic_roundtrip("999999");
    }

    #[test]
    fn test_deeply_nested_and_chain_semantic() {
        test_semantic_roundtrip("a && b && c && d && e");
    }

    #[test]
    fn test_deeply_nested_or_chain_semantic() {
        test_semantic_roundtrip("a || b || c || d || e");
    }

    #[test]
    fn test_mixed_operators_semantic() {
        test_semantic_roundtrip("a && b || c && d");
        test_semantic_roundtrip("(a || b) && (c || d)");
        test_semantic_roundtrip("!a && !b || c");
    }

    #[test]
    fn test_negation_of_complex_expressions_semantic() {
        test_semantic_roundtrip("!(a && b)");
        test_semantic_roundtrip("!(a || b)");
        test_semantic_roundtrip("!(!a)");
        test_semantic_roundtrip("!(has(HOOKSHOT) && is_adult)");
    }

    #[test]
    fn test_function_with_multiple_args_semantic() {
        test_semantic_roundtrip("has(BOTTLE, 2)");
        test_semantic_roundtrip("has(Small_Key_Forest_Temple, 5)");
    }

    #[test]
    fn test_cond_function_semantic() {
        test_semantic_roundtrip("cond(true, true, false)");
        test_semantic_roundtrip("cond(is_adult, has(HOOKSHOT), has(BOOMERANG))");
    }
}

// ============================================================================
// Evaluation Context Sensitivity Tests
// ============================================================================

mod context_sensitivity {
    use super::*;

    /// Tests that an expression evaluates differently in different contexts
    /// (proving our semantic tests are meaningful).
    #[test]
    fn test_context_actually_matters() {
        let expr = parse("is_adult && has(HOOKSHOT)").unwrap();

        // Context 1: adult with hookshot - should be true
        let ctx1 = MockEvalContext::adult().with_item("HOOKSHOT");
        assert!(eval(&expr, &ctx1).unwrap());

        // Context 2: child with hookshot - should be false
        let ctx2 = MockEvalContext::child().with_item("HOOKSHOT");
        assert!(!eval(&expr, &ctx2).unwrap());

        // Context 3: adult without hookshot - should be false
        let ctx3 = MockEvalContext::adult();
        assert!(!eval(&expr, &ctx3).unwrap());
    }

    #[test]
    fn test_events_are_context_sensitive() {
        let expr = parse("event(MIDO_MOVED)").unwrap();

        let with_event = MockEvalContext::new().with_event("MIDO_MOVED");
        let without_event = MockEvalContext::new();

        assert!(eval(&expr, &with_event).unwrap());
        assert!(!eval(&expr, &without_event).unwrap());
    }

    #[test]
    fn test_settings_are_context_sensitive() {
        let expr = parse("setting(skip_child_zelda)").unwrap();

        let with_setting = MockEvalContext::new().with_setting("skip_child_zelda");
        let without_setting = MockEvalContext::new();

        assert!(eval(&expr, &with_setting).unwrap());
        assert!(!eval(&expr, &without_setting).unwrap());
    }
}

// ============================================================================
// Double Roundtrip Tests
// ============================================================================

mod double_roundtrip {
    use super::*;

    /// Tests that multiple roundtrips preserve semantic equivalence.
    fn test_double_semantic_roundtrip(expr_str: &str) {
        let original = parse(expr_str).unwrap();

        // First roundtrip
        let displayed1 = original.to_string();
        let roundtrip1 = parse(&displayed1).unwrap();

        // Second roundtrip
        let displayed2 = roundtrip1.to_string();
        let roundtrip2 = parse(&displayed2).unwrap();

        // All three should be semantically equivalent
        assert_semantically_equivalent(&original, &roundtrip1, "first roundtrip");
        assert_semantically_equivalent(&original, &roundtrip2, "second roundtrip");
        assert_semantically_equivalent(&roundtrip1, &roundtrip2, "between roundtrips");

        // After first roundtrip, structural equality should be stable
        assert_eq!(
            roundtrip1, roundtrip2,
            "multiple roundtrips should stabilize structure"
        );
    }

    #[test]
    fn test_double_roundtrip_simple() {
        test_double_semantic_roundtrip("true");
        test_double_semantic_roundtrip("a && b");
        test_double_semantic_roundtrip("a || b");
    }

    #[test]
    fn test_double_roundtrip_complex() {
        test_double_semantic_roundtrip("(has(HOOKSHOT) || has(LONGSHOT)) && is_adult");
        test_double_semantic_roundtrip("event(MIDO_MOVED) || setting(skip_child_zelda)");
    }

    #[test]
    fn test_double_roundtrip_with_normalization() {
        // Expression that gets normalized on first roundtrip
        test_double_semantic_roundtrip("a && b || c");
    }
}

// ============================================================================
// Specific Bug Prevention Tests
// ============================================================================

mod bug_prevention {
    use super::*;

    /// Ensures that operator precedence is correctly preserved through roundtrip.
    #[test]
    fn test_precedence_preserved_semantically() {
        // && has higher precedence than ||
        // "a && b || c" means "(a && b) || c", not "a && (b || c)"
        let expr = parse("a && b || c").unwrap();

        // Test with context where the difference matters
        // If a=false, b=true, c=true:
        //   (a && b) || c = (false && true) || true = false || true = true
        //   a && (b || c) = false && (true || true) = false && true = false
        let ctx = MockEvalContext::new()
            .with_event("b")
            .with_event("c")
            // 'a' is not set, so it evaluates to false

        ;
        // After roundtrip, should behave the same
        let displayed = expr.to_string();
        let roundtrip = parse(&displayed).unwrap();

        let original_result = eval(&expr, &ctx);
        let roundtrip_result = eval(&roundtrip, &ctx);

        assert_eq!(
            original_result.ok(),
            roundtrip_result.ok(),
            "Operator precedence must be preserved through roundtrip"
        );
    }

    /// Ensures that short-circuit evaluation is preserved.
    #[test]
    fn test_short_circuit_preserved() {
        // In "false && x", x should not be evaluated (short-circuit)
        // This is semantic behavior that must be preserved
        let expr1 = parse("false && unknown_func()").unwrap();
        let displayed = expr1.to_string();
        let expr2 = parse(&displayed).unwrap();

        let ctx = MockEvalContext::new();

        // Both should succeed without error (due to short-circuit)
        // unknown_func would cause an error if evaluated
        let result1 = eval(&expr1, &ctx).ok();
        let result2 = eval(&expr2, &ctx).ok();

        assert_eq!(result1, result2);
        assert_eq!(result1, Some(false));
    }

    /// Ensures that double negation works correctly through roundtrip.
    #[test]
    fn test_double_negation_semantic() {
        let expr = parse("!!true").unwrap();
        let displayed = expr.to_string();
        let roundtrip = parse(&displayed).unwrap();

        let ctx = MockEvalContext::new();

        assert_eq!(eval(&expr, &ctx).ok(), Some(true));
        assert_eq!(eval(&roundtrip, &ctx).ok(), Some(true));
    }
}
