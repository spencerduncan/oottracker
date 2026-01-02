//! Logic evaluation integration tests for ootmm.
//!
//! Tests the complete parse -> evaluate pipeline for various game logic scenarios.

mod common;

use common::MockEvalContext;
use ootmm::expr::{eval_str, parse, Expr};

// =============================================================================
// OoT Forest Temple Access Tests
// =============================================================================

mod forest_temple_access {
    use super::*;

    /// Forest Temple requires being an adult with hookshot to access certain areas.
    /// This tests that adult Link with hookshot can access, but child Link cannot.
    #[test]
    fn adult_with_hookshot_can_access() {
        let ctx = MockEvalContext::adult().with_item("HOOKSHOT");

        let result = eval_str("is_adult && has(HOOKSHOT)", &ctx).unwrap();
        assert!(result, "Adult with hookshot should be able to access Forest Temple");
    }

    #[test]
    fn child_cannot_access() {
        let ctx = MockEvalContext::child().with_item("HOOKSHOT");

        let result = eval_str("is_adult && has(HOOKSHOT)", &ctx).unwrap();
        assert!(!result, "Child should not be able to access Forest Temple even with hookshot");
    }

    #[test]
    fn adult_without_hookshot_cannot_access() {
        let ctx = MockEvalContext::adult();

        let result = eval_str("is_adult && has(HOOKSHOT)", &ctx).unwrap();
        assert!(!result, "Adult without hookshot should not be able to access Forest Temple");
    }

    #[test]
    fn child_without_hookshot_cannot_access() {
        let ctx = MockEvalContext::child();

        let result = eval_str("is_adult && has(HOOKSHOT)", &ctx).unwrap();
        assert!(!result, "Child without hookshot should not be able to access Forest Temple");
    }
}

// =============================================================================
// Boolean AND Operator Tests
// =============================================================================

mod boolean_and_operator {
    use super::*;

    /// Tests has(A) && has(B) when both items are present.
    #[test]
    fn both_items_present_returns_true() {
        let ctx = MockEvalContext::new()
            .with_item("BOMBS")
            .with_item("BOW");

        let result = eval_str("has(BOMBS) && has(BOW)", &ctx).unwrap();
        assert!(result, "has(BOMBS) && has(BOW) should be true when both items are present");
    }

    /// Tests has(A) && has(B) when only A is present.
    #[test]
    fn only_first_item_present_returns_false() {
        let ctx = MockEvalContext::new().with_item("BOMBS");

        let result = eval_str("has(BOMBS) && has(BOW)", &ctx).unwrap();
        assert!(!result, "has(BOMBS) && has(BOW) should be false when only BOMBS is present");
    }

    /// Tests has(A) && has(B) when only B is present.
    #[test]
    fn only_second_item_present_returns_false() {
        let ctx = MockEvalContext::new().with_item("BOW");

        let result = eval_str("has(BOMBS) && has(BOW)", &ctx).unwrap();
        assert!(!result, "has(BOMBS) && has(BOW) should be false when only BOW is present");
    }

    /// Tests has(A) && has(B) when neither item is present.
    #[test]
    fn neither_item_present_returns_false() {
        let ctx = MockEvalContext::new();

        let result = eval_str("has(BOMBS) && has(BOW)", &ctx).unwrap();
        assert!(!result, "has(BOMBS) && has(BOW) should be false when neither item is present");
    }

    /// Tests chained AND operators.
    #[test]
    fn chained_and_operators() {
        let ctx = MockEvalContext::adult()
            .with_item("HOOKSHOT")
            .with_item("BOW")
            .with_item("BOMBS");

        let result = eval_str("is_adult && has(HOOKSHOT) && has(BOW) && has(BOMBS)", &ctx).unwrap();
        assert!(result, "Chained AND with all conditions met should be true");

        // Remove one item - should fail
        let ctx_missing = MockEvalContext::adult()
            .with_item("HOOKSHOT")
            .with_item("BOMBS");

        let result_missing = eval_str("is_adult && has(HOOKSHOT) && has(BOW) && has(BOMBS)", &ctx_missing).unwrap();
        assert!(!result_missing, "Chained AND with one condition not met should be false");
    }
}

// =============================================================================
// Boolean OR Operator Tests
// =============================================================================

mod boolean_or_operator {
    use super::*;

    /// Tests has(A) || has(B) when only A is present.
    #[test]
    fn first_item_present_returns_true() {
        let ctx = MockEvalContext::new().with_item("HOOKSHOT");

        let result = eval_str("has(HOOKSHOT) || has(LONGSHOT)", &ctx).unwrap();
        assert!(result, "has(HOOKSHOT) || has(LONGSHOT) should be true when HOOKSHOT is present");
    }

    /// Tests has(A) || has(B) when only B is present.
    #[test]
    fn second_item_present_returns_true() {
        let ctx = MockEvalContext::new().with_item("LONGSHOT");

        let result = eval_str("has(HOOKSHOT) || has(LONGSHOT)", &ctx).unwrap();
        assert!(result, "has(HOOKSHOT) || has(LONGSHOT) should be true when LONGSHOT is present");
    }

    /// Tests has(A) || has(B) when both items are present.
    #[test]
    fn both_items_present_returns_true() {
        let ctx = MockEvalContext::new()
            .with_item("HOOKSHOT")
            .with_item("LONGSHOT");

        let result = eval_str("has(HOOKSHOT) || has(LONGSHOT)", &ctx).unwrap();
        assert!(result, "has(HOOKSHOT) || has(LONGSHOT) should be true when both items are present");
    }

    /// Tests has(A) || has(B) when neither item is present.
    #[test]
    fn neither_item_present_returns_false() {
        let ctx = MockEvalContext::new();

        let result = eval_str("has(HOOKSHOT) || has(LONGSHOT)", &ctx).unwrap();
        assert!(!result, "has(HOOKSHOT) || has(LONGSHOT) should be false when neither item is present");
    }

    /// Tests chained OR operators.
    #[test]
    fn chained_or_operators() {
        // Only one of many alternatives needed
        let ctx = MockEvalContext::new().with_item("BOMBS");

        let result = eval_str("has(HOOKSHOT) || has(LONGSHOT) || has(BOMBS) || has(BOW)", &ctx).unwrap();
        assert!(result, "Chained OR with one condition met should be true");
    }
}

// =============================================================================
// Mixed Boolean Operator Tests
// =============================================================================

mod mixed_operators {
    use super::*;

    /// Tests complex expression with both AND and OR.
    #[test]
    fn and_or_combination() {
        // Expression: is_adult && (has(HOOKSHOT) || has(LONGSHOT))
        // Adult with hookshot should pass
        let ctx = MockEvalContext::adult().with_item("HOOKSHOT");
        let result = eval_str("is_adult && (has(HOOKSHOT) || has(LONGSHOT))", &ctx).unwrap();
        assert!(result, "Adult with hookshot should satisfy the condition");

        // Adult with longshot should also pass
        let ctx2 = MockEvalContext::adult().with_item("LONGSHOT");
        let result2 = eval_str("is_adult && (has(HOOKSHOT) || has(LONGSHOT))", &ctx2).unwrap();
        assert!(result2, "Adult with longshot should satisfy the condition");

        // Child with hookshot should fail
        let ctx3 = MockEvalContext::child().with_item("HOOKSHOT");
        let result3 = eval_str("is_adult && (has(HOOKSHOT) || has(LONGSHOT))", &ctx3).unwrap();
        assert!(!result3, "Child with hookshot should not satisfy the condition");
    }

    /// Tests operator precedence: AND binds tighter than OR.
    #[test]
    fn operator_precedence() {
        // "a || b && c" should parse as "a || (b && c)"
        // With only 'a' true, the result should be true
        let ctx = MockEvalContext::adult();

        // is_adult || (has(HOOKSHOT) && has(BOW)) - adult but no items
        let result = eval_str("is_adult || has(HOOKSHOT) && has(BOW)", &ctx).unwrap();
        assert!(result, "is_adult should be enough due to OR short-circuit");

        // With child and only hookshot, should be false
        let ctx2 = MockEvalContext::child().with_item("HOOKSHOT");
        let result2 = eval_str("is_adult || has(HOOKSHOT) && has(BOW)", &ctx2).unwrap();
        assert!(!result2, "Child with only hookshot should fail (need both for AND)");

        // With child and both items, should be true
        let ctx3 = MockEvalContext::child()
            .with_item("HOOKSHOT")
            .with_item("BOW");
        let result3 = eval_str("is_adult || has(HOOKSHOT) && has(BOW)", &ctx3).unwrap();
        assert!(result3, "Child with both items should pass the AND part");
    }

    /// Tests NOT operator.
    #[test]
    fn not_operator() {
        let ctx = MockEvalContext::child();

        let result = eval_str("!is_adult", &ctx).unwrap();
        assert!(result, "!is_adult should be true for child");

        let ctx2 = MockEvalContext::adult();
        let result2 = eval_str("!is_adult", &ctx2).unwrap();
        assert!(!result2, "!is_adult should be false for adult");
    }

    /// Tests complex nested expression.
    #[test]
    fn complex_nested_expression() {
        // Complex check like: (is_adult && has(HOOKSHOT)) || (is_child && has(BOOMERANG))
        let adult_ctx = MockEvalContext::adult().with_item("HOOKSHOT");
        let result = eval_str("(is_adult && has(HOOKSHOT)) || (is_child && has(BOOMERANG))", &adult_ctx).unwrap();
        assert!(result, "Adult with hookshot should pass");

        let child_ctx = MockEvalContext::child().with_item("BOOMERANG");
        let result2 = eval_str("(is_adult && has(HOOKSHOT)) || (is_child && has(BOOMERANG))", &child_ctx).unwrap();
        assert!(result2, "Child with boomerang should pass");

        let wrong_items_ctx = MockEvalContext::adult().with_item("BOOMERANG");
        let result3 = eval_str("(is_adult && has(HOOKSHOT)) || (is_child && has(BOOMERANG))", &wrong_items_ctx).unwrap();
        assert!(!result3, "Adult with boomerang (wrong item) should fail");
    }
}

// =============================================================================
// Parse and Evaluate Pipeline Tests
// =============================================================================

mod parse_evaluate_pipeline {
    use super::*;

    /// Tests that parsing produces the expected AST structure.
    #[test]
    fn parse_and_expression() {
        let expr = parse("has(A) && has(B)").unwrap();
        assert!(matches!(expr, Expr::And(_, _)), "Should parse as AND expression");
    }

    /// Tests that parsing produces the expected AST structure for OR.
    #[test]
    fn parse_or_expression() {
        let expr = parse("has(A) || has(B)").unwrap();
        assert!(matches!(expr, Expr::Or(_, _)), "Should parse as OR expression");
    }

    /// Tests parsing complex expressions.
    #[test]
    fn parse_complex_expression() {
        let expr = parse("is_adult && (has(HOOKSHOT) || has(LONGSHOT)) && event(WATER_TEMPLE_CLEAR)").unwrap();
        assert!(matches!(expr, Expr::And(_, _)), "Top-level should be AND");
    }

    /// Tests that parsed expressions evaluate correctly.
    #[test]
    fn parse_then_evaluate() {
        let expr = parse("is_adult && has(HOOKSHOT)").unwrap();
        let ctx = MockEvalContext::adult().with_item("HOOKSHOT");

        let result = ootmm::expr::eval(&expr, &ctx).unwrap();
        assert!(result, "Parsed expression should evaluate to true");
    }
}
