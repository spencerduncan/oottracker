//! Tests verifying that tricks actually affect logic evaluation results.
//!
//! This module addresses GitHub issue #246: "Tricks are stored but never verified
//! to affect logic evaluation."
//!
//! These tests verify that:
//! 1. Enabling/disabling tricks changes evaluation results
//! 2. The `trick()` function correctly reflects enabled tricks
//! 3. Complex expressions combining tricks with other conditions work correctly

mod common;

use common::MockEvalContext;
use ootmm::expr::{eval_str, GameContext, GameContextBuilder};

// =============================================================================
// Basic Trick Evaluation Tests
// =============================================================================

mod basic_trick_evaluation {
    use super::*;

    /// Verifies that a simple trick expression evaluates to false when disabled.
    #[test]
    fn trick_disabled_evaluates_false() {
        let ctx = MockEvalContext::new();
        let result = eval_str("trick(logic_grottos_without_agony)", &ctx).unwrap();
        assert!(
            !result,
            "trick() should evaluate to false when trick is not enabled"
        );
    }

    /// Verifies that a simple trick expression evaluates to true when enabled.
    #[test]
    fn trick_enabled_evaluates_true() {
        let ctx = MockEvalContext::new().with_trick("logic_grottos_without_agony");
        let result = eval_str("trick(logic_grottos_without_agony)", &ctx).unwrap();
        assert!(
            result,
            "trick() should evaluate to true when trick is enabled"
        );
    }

    /// Verifies that enabling a trick changes the evaluation result from false to true.
    #[test]
    fn enabling_trick_changes_evaluation_result() {
        // Create context without trick
        let ctx_without_trick = MockEvalContext::new();

        // Create identical context with trick enabled
        let ctx_with_trick = MockEvalContext::new().with_trick("lens_skip");

        // Same expression, different results based on trick
        let expr = "trick(lens_skip)";

        let result_without = eval_str(expr, &ctx_without_trick).unwrap();
        let result_with = eval_str(expr, &ctx_with_trick).unwrap();

        assert!(!result_without, "Should be false without trick");
        assert!(result_with, "Should be true with trick");
        assert_ne!(
            result_without, result_with,
            "Enabling trick must change evaluation result"
        );
    }

    /// Verifies that different tricks are evaluated independently.
    #[test]
    fn different_tricks_evaluated_independently() {
        let ctx = MockEvalContext::new()
            .with_trick("trick_a")
            .with_trick("trick_b");

        assert!(eval_str("trick(trick_a)", &ctx).unwrap());
        assert!(eval_str("trick(trick_b)", &ctx).unwrap());
        assert!(
            !eval_str("trick(trick_c)", &ctx).unwrap(),
            "Unrelated trick should still be false"
        );
    }
}

// =============================================================================
// Trick Combinations with Boolean Operators
// =============================================================================

mod trick_boolean_combinations {
    use super::*;

    /// Tests trick combined with AND operator - both must be true.
    #[test]
    fn trick_and_item_both_required() {
        // Has item but no trick
        let ctx_item_only = MockEvalContext::new().with_item("HOOKSHOT");
        let result1 = eval_str("has(HOOKSHOT) && trick(hookshot_jump)", &ctx_item_only).unwrap();
        assert!(!result1, "Should fail without trick");

        // Has trick but no item
        let ctx_trick_only = MockEvalContext::new().with_trick("hookshot_jump");
        let result2 = eval_str("has(HOOKSHOT) && trick(hookshot_jump)", &ctx_trick_only).unwrap();
        assert!(!result2, "Should fail without item");

        // Has both
        let ctx_both = MockEvalContext::new()
            .with_item("HOOKSHOT")
            .with_trick("hookshot_jump");
        let result3 = eval_str("has(HOOKSHOT) && trick(hookshot_jump)", &ctx_both).unwrap();
        assert!(result3, "Should pass with both item and trick");
    }

    /// Tests trick combined with OR operator - either can satisfy.
    #[test]
    fn trick_or_item_either_satisfies() {
        let expr = "has(LONGSHOT) || trick(hookshot_jump)";

        // Has neither
        let ctx_neither = MockEvalContext::new();
        assert!(!eval_str(expr, &ctx_neither).unwrap());

        // Has item only
        let ctx_item = MockEvalContext::new().with_item("LONGSHOT");
        assert!(eval_str(expr, &ctx_item).unwrap());

        // Has trick only
        let ctx_trick = MockEvalContext::new().with_trick("hookshot_jump");
        assert!(eval_str(expr, &ctx_trick).unwrap());

        // Has both
        let ctx_both = MockEvalContext::new()
            .with_item("LONGSHOT")
            .with_trick("hookshot_jump");
        assert!(eval_str(expr, &ctx_both).unwrap());
    }

    /// Tests NOT operator with tricks.
    #[test]
    fn not_trick_inverts_result() {
        let ctx_without = MockEvalContext::new();
        let ctx_with = MockEvalContext::new().with_trick("some_trick");

        assert!(
            eval_str("!trick(some_trick)", &ctx_without).unwrap(),
            "NOT trick should be true when trick disabled"
        );
        assert!(
            !eval_str("!trick(some_trick)", &ctx_with).unwrap(),
            "NOT trick should be false when trick enabled"
        );
    }

    /// Tests multiple tricks combined with AND.
    #[test]
    fn multiple_tricks_with_and() {
        let expr = "trick(trick_a) && trick(trick_b) && trick(trick_c)";

        // Only one trick enabled
        let ctx_one = MockEvalContext::new().with_trick("trick_a");
        assert!(!eval_str(expr, &ctx_one).unwrap());

        // Two tricks enabled
        let ctx_two = MockEvalContext::new()
            .with_trick("trick_a")
            .with_trick("trick_b");
        assert!(!eval_str(expr, &ctx_two).unwrap());

        // All three tricks enabled
        let ctx_all = MockEvalContext::new()
            .with_trick("trick_a")
            .with_trick("trick_b")
            .with_trick("trick_c");
        assert!(eval_str(expr, &ctx_all).unwrap());
    }

    /// Tests multiple tricks combined with OR.
    #[test]
    fn multiple_tricks_with_or() {
        let expr = "trick(trick_a) || trick(trick_b) || trick(trick_c)";

        // No tricks enabled
        let ctx_none = MockEvalContext::new();
        assert!(!eval_str(expr, &ctx_none).unwrap());

        // One trick enabled
        let ctx_one = MockEvalContext::new().with_trick("trick_b");
        assert!(eval_str(expr, &ctx_one).unwrap());

        // All tricks enabled
        let ctx_all = MockEvalContext::new()
            .with_trick("trick_a")
            .with_trick("trick_b")
            .with_trick("trick_c");
        assert!(eval_str(expr, &ctx_all).unwrap());
    }
}

// =============================================================================
// Complex Real-World Trick Scenarios
// =============================================================================

mod real_world_trick_scenarios {
    use super::*;

    /// Tests a scenario where a trick allows bypassing an item requirement.
    ///
    /// Example: Shadow Temple normally requires Lens of Truth, but with
    /// `logic_lens_shadow` trick, you can navigate without it.
    #[test]
    fn lens_trick_bypasses_item_requirement() {
        // Normal path: need Lens of Truth
        // Trick path: can skip Lens with trick
        let expr = "has(LENS_OF_TRUTH) || trick(logic_lens_shadow)";

        // Without item or trick - cannot access
        let ctx_nothing = MockEvalContext::adult();
        assert!(!eval_str(expr, &ctx_nothing).unwrap());

        // With Lens - can access normally
        let ctx_lens = MockEvalContext::adult().with_item("LENS_OF_TRUTH");
        assert!(eval_str(expr, &ctx_lens).unwrap());

        // With trick but no Lens - can still access
        let ctx_trick = MockEvalContext::adult().with_trick("logic_lens_shadow");
        assert!(eval_str(expr, &ctx_trick).unwrap());
    }

    /// Tests a scenario where multiple tricks provide alternative routes.
    #[test]
    fn multiple_trick_alternatives() {
        // Spirit Temple: Can either have Hover Boots OR use a hover boost trick
        // OR use a bomb boost trick to reach an area
        let expr =
            "has(HOVER_BOOTS) || trick(logic_spirit_wall_hover) || trick(logic_spirit_wall_bomb)";

        let ctx_nothing = MockEvalContext::adult();
        assert!(!eval_str(expr, &ctx_nothing).unwrap());

        let ctx_boots = MockEvalContext::adult().with_item("HOVER_BOOTS");
        assert!(eval_str(expr, &ctx_boots).unwrap());

        let ctx_hover_trick = MockEvalContext::adult().with_trick("logic_spirit_wall_hover");
        assert!(eval_str(expr, &ctx_hover_trick).unwrap());

        let ctx_bomb_trick = MockEvalContext::adult().with_trick("logic_spirit_wall_bomb");
        assert!(eval_str(expr, &ctx_bomb_trick).unwrap());
    }

    /// Tests age-restricted trick (trick only works for adult).
    #[test]
    fn age_restricted_trick() {
        // Adult-only trick: Hover boots trick requires being adult
        let expr = "is_adult && trick(logic_shadow_fire_arrow_entry)";

        // Adult with trick
        let ctx_adult_trick = MockEvalContext::adult().with_trick("logic_shadow_fire_arrow_entry");
        assert!(eval_str(expr, &ctx_adult_trick).unwrap());

        // Child with trick (should fail - child can't do adult tricks)
        let ctx_child_trick = MockEvalContext::child().with_trick("logic_shadow_fire_arrow_entry");
        assert!(!eval_str(expr, &ctx_child_trick).unwrap());

        // Adult without trick
        let ctx_adult_no_trick = MockEvalContext::adult();
        assert!(!eval_str(expr, &ctx_adult_no_trick).unwrap());
    }

    /// Tests trick combined with multiple items.
    #[test]
    fn trick_with_item_requirements() {
        // Ganon's Castle skip requires bombs AND hookshot AND the trick
        let expr = "has(BOMBS) && has(HOOKSHOT) && trick(logic_ganon_skip)";

        // All items but no trick
        let ctx_items = MockEvalContext::adult()
            .with_item("BOMBS")
            .with_item("HOOKSHOT");
        assert!(!eval_str(expr, &ctx_items).unwrap());

        // Trick but missing items
        let ctx_partial = MockEvalContext::adult()
            .with_item("BOMBS")
            .with_trick("logic_ganon_skip");
        assert!(!eval_str(expr, &ctx_partial).unwrap());

        // All requirements met
        let ctx_all = MockEvalContext::adult()
            .with_item("BOMBS")
            .with_item("HOOKSHOT")
            .with_trick("logic_ganon_skip");
        assert!(eval_str(expr, &ctx_all).unwrap());
    }

    /// Tests complex nested expression with tricks.
    #[test]
    fn complex_nested_trick_expression() {
        // Complex logic: (adult with hookshot) OR (child with boomerang AND trick)
        let expr = "(is_adult && has(HOOKSHOT)) || (is_child && has(BOOMERANG) && trick(logic_child_deadhand))";

        // Adult with hookshot - passes first branch
        let ctx1 = MockEvalContext::adult().with_item("HOOKSHOT");
        assert!(eval_str(expr, &ctx1).unwrap());

        // Child with boomerang but no trick - fails second branch
        let ctx2 = MockEvalContext::child().with_item("BOOMERANG");
        assert!(!eval_str(expr, &ctx2).unwrap());

        // Child with boomerang and trick - passes second branch
        let ctx3 = MockEvalContext::child()
            .with_item("BOOMERANG")
            .with_trick("logic_child_deadhand");
        assert!(eval_str(expr, &ctx3).unwrap());

        // Child with trick but no boomerang - fails second branch
        let ctx4 = MockEvalContext::child().with_trick("logic_child_deadhand");
        assert!(!eval_str(expr, &ctx4).unwrap());
    }
}

// =============================================================================
// GameContext Trick Evaluation Tests
// =============================================================================

mod game_context_trick_evaluation {
    use super::*;

    /// Tests that GameContext tricks affect evaluation (not just MockEvalContext).
    #[test]
    fn game_context_trick_affects_evaluation() {
        let mut ctx = GameContext::new();

        // Without trick
        assert!(!eval_str("trick(test_trick)", &ctx).unwrap());

        // Add trick
        ctx.add_trick("test_trick");
        assert!(eval_str("trick(test_trick)", &ctx).unwrap());

        // Remove trick
        ctx.remove_trick("test_trick");
        assert!(!eval_str("trick(test_trick)", &ctx).unwrap());
    }

    /// Tests GameContextBuilder with tricks.
    #[test]
    fn game_context_builder_trick_affects_evaluation() {
        let ctx = GameContextBuilder::new()
            .with_trick("builder_trick")
            .build();

        assert!(eval_str("trick(builder_trick)", &ctx).unwrap());
        assert!(!eval_str("trick(other_trick)", &ctx).unwrap());
    }

    /// Tests clearing tricks affects evaluation.
    #[test]
    fn clearing_tricks_affects_evaluation() {
        let mut ctx = GameContext::new();
        ctx.add_trick("trick1");
        ctx.add_trick("trick2");

        assert!(eval_str("trick(trick1) && trick(trick2)", &ctx).unwrap());

        ctx.clear_tricks();

        assert!(!eval_str("trick(trick1)", &ctx).unwrap());
        assert!(!eval_str("trick(trick2)", &ctx).unwrap());
    }
}

// =============================================================================
// Trick State Change Tests
// =============================================================================

mod trick_state_changes {
    use super::*;

    /// Verifies that the same context object can have tricks toggled
    /// and evaluation results change accordingly.
    #[test]
    fn mutable_trick_toggle_changes_evaluation() {
        let mut ctx = GameContext::new();
        let expr = "trick(toggle_trick)";

        // Initially disabled
        assert!(!eval_str(expr, &ctx).unwrap());

        // Enable
        ctx.add_trick("toggle_trick");
        assert!(eval_str(expr, &ctx).unwrap());

        // Disable
        ctx.remove_trick("toggle_trick");
        assert!(!eval_str(expr, &ctx).unwrap());

        // Re-enable
        ctx.add_trick("toggle_trick");
        assert!(eval_str(expr, &ctx).unwrap());
    }

    /// Tests that trick changes don't affect other context state.
    #[test]
    fn trick_changes_isolated_from_other_state() {
        let mut ctx = GameContext::new();
        ctx.set_item("HOOKSHOT", 1);
        ctx.add_event("TEST_EVENT");

        // Add and remove trick
        ctx.add_trick("test_trick");
        ctx.remove_trick("test_trick");

        // Other state should be unaffected
        assert!(eval_str("has(HOOKSHOT)", &ctx).unwrap());
        assert!(eval_str("event(TEST_EVENT)", &ctx).unwrap());
    }
}

// =============================================================================
// Edge Cases
// =============================================================================

mod edge_cases {
    use super::*;

    /// Tests trick names with special characters/underscores.
    #[test]
    fn trick_names_with_underscores() {
        let ctx = MockEvalContext::new()
            .with_trick("logic_grottos_without_agony")
            .with_trick("logic_lens_botw")
            .with_trick("logic_dc_jump");

        assert!(eval_str("trick(logic_grottos_without_agony)", &ctx).unwrap());
        assert!(eval_str("trick(logic_lens_botw)", &ctx).unwrap());
        assert!(eval_str("trick(logic_dc_jump)", &ctx).unwrap());
    }

    /// Tests that trick evaluation is case-sensitive.
    #[test]
    fn trick_names_are_case_sensitive() {
        let ctx = MockEvalContext::new().with_trick("MyTrick");

        assert!(eval_str("trick(MyTrick)", &ctx).unwrap());
        // Different case should NOT match
        assert!(!eval_str("trick(mytrick)", &ctx).unwrap());
        assert!(!eval_str("trick(MYTRICK)", &ctx).unwrap());
    }

    /// Tests trick combined with setting.
    #[test]
    fn trick_combined_with_setting() {
        let expr = "setting(shuffle_songs) || trick(skip_shuffle)";

        // Neither setting nor trick
        let ctx1 = MockEvalContext::new();
        assert!(!eval_str(expr, &ctx1).unwrap());

        // Setting enabled
        let ctx2 = MockEvalContext::new().with_setting("shuffle_songs");
        assert!(eval_str(expr, &ctx2).unwrap());

        // Trick enabled
        let ctx3 = MockEvalContext::new().with_trick("skip_shuffle");
        assert!(eval_str(expr, &ctx3).unwrap());
    }

    /// Tests trick combined with event.
    #[test]
    fn trick_combined_with_event() {
        let expr = "event(BOSS_DEFEATED) && trick(boss_skip)";

        // Event but no trick
        let ctx1 = MockEvalContext::new().with_event("BOSS_DEFEATED");
        assert!(!eval_str(expr, &ctx1).unwrap());

        // Trick but no event
        let ctx2 = MockEvalContext::new().with_trick("boss_skip");
        assert!(!eval_str(expr, &ctx2).unwrap());

        // Both
        let ctx3 = MockEvalContext::new()
            .with_event("BOSS_DEFEATED")
            .with_trick("boss_skip");
        assert!(eval_str(expr, &ctx3).unwrap());
    }
}
