//! Item-related built-in functions (has, can_use).
//!
//! These functions check inventory and item usability conditions for OoTMM logic expressions.

use crate::expr::{EvalContext, EvalError, Expr};

/// Evaluate the `has` built-in function.
///
/// Checks if the player has a specific item, optionally with a minimum count.
///
/// # Syntax
/// - `has(ITEM)` - checks if player has at least 1 of the item
/// - `has(ITEM, count)` - checks if player has at least `count` of the item
///
/// # Examples
/// - `has(HOOKSHOT)` - true if player has the hookshot
/// - `has(BOMBCHU, 10)` - true if player has at least 10 bombchus
pub fn eval_has(args: &[Expr], ctx: &impl EvalContext) -> Result<bool, EvalError> {
    match args.len() {
        1 => {
            let item_name = extract_item_name(&args[0])?;
            Ok(ctx.has_item(&item_name, 1))
        }
        2 => {
            let item_name = extract_item_name(&args[0])?;
            let count = extract_count(&args[1])?;
            Ok(ctx.has_item(&item_name, count))
        }
        _ => Err(EvalError::Error(format!(
            "has() expects 1 or 2 arguments, got {}",
            args.len()
        ))),
    }
}

/// Evaluate the `can_use` built-in function.
///
/// Checks if the player can use a specific item, considering both possession and
/// age-related restrictions. Some items can only be used by Adult Link or Child Link.
///
/// # Syntax
/// - `can_use(ITEM)` - checks if player has the item and meets age requirements
///
/// # Age Restrictions
/// Items that require Adult Link:
/// - HOOKSHOT, LONGSHOT, BOW, HAMMER, IRON_BOOTS, HOVER_BOOTS
///
/// Items that require Child Link:
/// - SLINGSHOT, BOOMERANG, KOKIRI_SWORD
///
/// Items usable by either:
/// - BOMBS, BOMBCHU, DEKU_STICK, DEKU_NUT, OCARINA, MAGIC_BEAN, etc.
///
/// # Examples
/// - `can_use(HOOKSHOT)` - true if player has hookshot AND is adult
/// - `can_use(BOOMERANG)` - true if player has boomerang AND is child
/// - `can_use(BOMBS)` - true if player has bombs (no age restriction)
pub fn eval_can_use(args: &[Expr], ctx: &impl EvalContext) -> Result<bool, EvalError> {
    if args.len() != 1 {
        return Err(EvalError::Error(format!(
            "can_use() expects 1 argument, got {}",
            args.len()
        )));
    }

    let item_name = extract_item_name(&args[0])?;

    // First check if player has the item
    if !ctx.has_item(&item_name, 1) {
        return Ok(false);
    }

    // Then check age restrictions
    match get_age_restriction(&item_name) {
        AgeRestriction::AdultOnly => Ok(ctx.is_adult()),
        AgeRestriction::ChildOnly => Ok(ctx.is_child()),
        AgeRestriction::None => Ok(true),
    }
}

/// Age restriction for item usage.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum AgeRestriction {
    /// Item can only be used by Adult Link.
    AdultOnly,
    /// Item can only be used by Child Link.
    ChildOnly,
    /// Item can be used by either Adult or Child Link.
    None,
}

/// Get the age restriction for an item.
fn get_age_restriction(item: &str) -> AgeRestriction {
    // Normalize to uppercase for comparison
    let item_upper = item.to_uppercase();

    // Adult-only items
    const ADULT_ONLY_ITEMS: &[&str] = &[
        "HOOKSHOT",
        "LONGSHOT",
        "BOW",
        "HAMMER",
        "MEGATON_HAMMER",
        "IRON_BOOTS",
        "HOVER_BOOTS",
        "GORON_TUNIC",
        "ZORA_TUNIC",
        "MIRROR_SHIELD",
        "BIGGORON_SWORD",
        "GIANTS_KNIFE",
    ];

    // Child-only items
    const CHILD_ONLY_ITEMS: &[&str] = &[
        "SLINGSHOT",
        "FAIRY_SLINGSHOT",
        "BOOMERANG",
        "KOKIRI_SWORD",
        "DEKU_SHIELD",
    ];

    if ADULT_ONLY_ITEMS.contains(&item_upper.as_str()) {
        AgeRestriction::AdultOnly
    } else if CHILD_ONLY_ITEMS.contains(&item_upper.as_str()) {
        AgeRestriction::ChildOnly
    } else {
        AgeRestriction::None
    }
}

/// Extract an item name from an expression.
fn extract_item_name(expr: &Expr) -> Result<String, EvalError> {
    match expr {
        Expr::Ident(name) => Ok(name.clone()),
        Expr::String(s) => Ok(s.clone()),
        _ => Err(EvalError::Error(format!(
            "expected item name (identifier or string), got {:?}",
            expr
        ))),
    }
}

/// Extract a count value from an expression.
fn extract_count(expr: &Expr) -> Result<u32, EvalError> {
    match expr {
        Expr::Number(n) => {
            if *n < 0 {
                Err(EvalError::Error(format!(
                    "count must be non-negative, got {}",
                    n
                )))
            } else {
                Ok(*n as u32)
            }
        }
        _ => Err(EvalError::Error(format!(
            "expected count (number), got {:?}",
            expr
        ))),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Mock context for testing built-in functions.
    struct MockContext {
        items: std::collections::HashMap<String, u32>,
        is_adult: bool,
    }

    impl MockContext {
        fn new() -> Self {
            Self {
                items: std::collections::HashMap::new(),
                is_adult: true,
            }
        }

        fn with_item(mut self, item: &str, count: u32) -> Self {
            self.items.insert(item.to_uppercase(), count);
            self
        }

        fn with_child_age(mut self) -> Self {
            self.is_adult = false;
            self
        }

        fn with_adult_age(mut self) -> Self {
            self.is_adult = true;
            self
        }
    }

    impl EvalContext for MockContext {
        fn has_item(&self, item: &str, count: u32) -> bool {
            self.items
                .get(&item.to_uppercase())
                .map(|&c| c >= count)
                .unwrap_or(false)
        }

        fn event(&self, _name: &str) -> bool {
            false
        }

        fn setting(&self, _name: &str) -> Option<bool> {
            None
        }

        fn trick(&self, _name: &str) -> bool {
            false
        }

        fn is_adult(&self) -> bool {
            self.is_adult
        }

        fn is_child(&self) -> bool {
            !self.is_adult
        }

        fn mm_time(&self) -> u32 {
            0 // Default to dawn for item tests
        }
    }

    // --- has() tests ---

    #[test]
    fn test_has_single_item() {
        let ctx = MockContext::new().with_item("HOOKSHOT", 1);
        let args = vec![Expr::Ident("HOOKSHOT".into())];

        assert!(eval_has(&args, &ctx).unwrap());
    }

    #[test]
    fn test_has_missing_item() {
        let ctx = MockContext::new();
        let args = vec![Expr::Ident("HOOKSHOT".into())];

        assert!(!eval_has(&args, &ctx).unwrap());
    }

    #[test]
    fn test_has_with_count() {
        let ctx = MockContext::new().with_item("BOMBCHU", 15);

        // Has at least 10
        let args = vec![Expr::Ident("BOMBCHU".into()), Expr::Number(10)];
        assert!(eval_has(&args, &ctx).unwrap());

        // Has at least 20 (false - only has 15)
        let args = vec![Expr::Ident("BOMBCHU".into()), Expr::Number(20)];
        assert!(!eval_has(&args, &ctx).unwrap());
    }

    #[test]
    fn test_has_string_item_name() {
        let ctx = MockContext::new().with_item("HOOKSHOT", 1);
        let args = vec![Expr::String("HOOKSHOT".into())];

        assert!(eval_has(&args, &ctx).unwrap());
    }

    #[test]
    fn test_has_wrong_arg_count() {
        let ctx = MockContext::new();

        // No arguments
        let result = eval_has(&[], &ctx);
        assert!(result.is_err());

        // Too many arguments
        let args = vec![
            Expr::Ident("HOOKSHOT".into()),
            Expr::Number(1),
            Expr::Number(2),
        ];
        let result = eval_has(&args, &ctx);
        assert!(result.is_err());
    }

    #[test]
    fn test_has_invalid_item_name() {
        let ctx = MockContext::new();
        let args = vec![Expr::Number(42)];

        let result = eval_has(&args, &ctx);
        assert!(result.is_err());
    }

    #[test]
    fn test_has_negative_count() {
        let ctx = MockContext::new().with_item("BOMBS", 5);
        let args = vec![Expr::Ident("BOMBS".into()), Expr::Number(-1)];

        let result = eval_has(&args, &ctx);
        assert!(result.is_err());
    }

    // --- can_use() tests ---

    #[test]
    fn test_can_use_adult_item_as_adult() {
        let ctx = MockContext::new().with_item("HOOKSHOT", 1).with_adult_age();
        let args = vec![Expr::Ident("HOOKSHOT".into())];

        assert!(eval_can_use(&args, &ctx).unwrap());
    }

    #[test]
    fn test_can_use_adult_item_as_child() {
        let ctx = MockContext::new().with_item("HOOKSHOT", 1).with_child_age();
        let args = vec![Expr::Ident("HOOKSHOT".into())];

        assert!(!eval_can_use(&args, &ctx).unwrap());
    }

    #[test]
    fn test_can_use_child_item_as_child() {
        let ctx = MockContext::new()
            .with_item("BOOMERANG", 1)
            .with_child_age();
        let args = vec![Expr::Ident("BOOMERANG".into())];

        assert!(eval_can_use(&args, &ctx).unwrap());
    }

    #[test]
    fn test_can_use_child_item_as_adult() {
        let ctx = MockContext::new()
            .with_item("BOOMERANG", 1)
            .with_adult_age();
        let args = vec![Expr::Ident("BOOMERANG".into())];

        assert!(!eval_can_use(&args, &ctx).unwrap());
    }

    #[test]
    fn test_can_use_any_age_item() {
        let ctx_adult = MockContext::new().with_item("BOMBS", 5).with_adult_age();
        let ctx_child = MockContext::new().with_item("BOMBS", 5).with_child_age();
        let args = vec![Expr::Ident("BOMBS".into())];

        assert!(eval_can_use(&args, &ctx_adult).unwrap());
        assert!(eval_can_use(&args, &ctx_child).unwrap());
    }

    #[test]
    fn test_can_use_missing_item() {
        let ctx = MockContext::new().with_adult_age();
        let args = vec![Expr::Ident("HOOKSHOT".into())];

        assert!(!eval_can_use(&args, &ctx).unwrap());
    }

    #[test]
    fn test_can_use_wrong_arg_count() {
        let ctx = MockContext::new();

        // No arguments
        let result = eval_can_use(&[], &ctx);
        assert!(result.is_err());

        // Too many arguments
        let args = vec![Expr::Ident("HOOKSHOT".into()), Expr::Number(1)];
        let result = eval_can_use(&args, &ctx);
        assert!(result.is_err());
    }

    #[test]
    fn test_can_use_case_insensitive() {
        let ctx = MockContext::new().with_item("hookshot", 1).with_adult_age();
        let args = vec![Expr::Ident("HOOKSHOT".into())];

        // Item stored lowercase, queried uppercase
        assert!(eval_can_use(&args, &ctx).unwrap());
    }

    // --- Age restriction tests ---

    #[test]
    fn test_age_restrictions() {
        assert_eq!(get_age_restriction("HOOKSHOT"), AgeRestriction::AdultOnly);
        assert_eq!(get_age_restriction("BOW"), AgeRestriction::AdultOnly);
        assert_eq!(get_age_restriction("HAMMER"), AgeRestriction::AdultOnly);
        assert_eq!(get_age_restriction("SLINGSHOT"), AgeRestriction::ChildOnly);
        assert_eq!(get_age_restriction("BOOMERANG"), AgeRestriction::ChildOnly);
        assert_eq!(get_age_restriction("BOMBS"), AgeRestriction::None);
        assert_eq!(get_age_restriction("UNKNOWN_ITEM"), AgeRestriction::None);
    }

    #[test]
    fn test_age_restriction_case_insensitive() {
        assert_eq!(get_age_restriction("hookshot"), AgeRestriction::AdultOnly);
        assert_eq!(get_age_restriction("Hookshot"), AgeRestriction::AdultOnly);
        assert_eq!(get_age_restriction("HOOKSHOT"), AgeRestriction::AdultOnly);
    }

    // ===== Additional built-in function tests =====

    #[test]
    fn test_has_with_zero_count() {
        let ctx = MockContext::new().with_item("BOMBS", 5);
        let args = vec![Expr::Ident("BOMBS".into()), Expr::Number(0)];
        // Has at least 0 should always be true if item exists
        assert!(eval_has(&args, &ctx).unwrap());
    }

    #[test]
    fn test_has_exact_count() {
        let ctx = MockContext::new().with_item("ARROWS", 30);
        // Exactly 30
        let args = vec![Expr::Ident("ARROWS".into()), Expr::Number(30)];
        assert!(eval_has(&args, &ctx).unwrap());
        // 31 should fail
        let args = vec![Expr::Ident("ARROWS".into()), Expr::Number(31)];
        assert!(!eval_has(&args, &ctx).unwrap());
    }

    #[test]
    fn test_has_multiple_items() {
        let ctx = MockContext::new()
            .with_item("HOOKSHOT", 1)
            .with_item("BOMBS", 20)
            .with_item("ARROWS", 50);

        assert!(eval_has(&[Expr::Ident("HOOKSHOT".into())], &ctx).unwrap());
        assert!(eval_has(&[Expr::Ident("BOMBS".into())], &ctx).unwrap());
        assert!(eval_has(&[Expr::Ident("ARROWS".into())], &ctx).unwrap());
        assert!(!eval_has(&[Expr::Ident("BOW".into())], &ctx).unwrap());
    }

    #[test]
    fn test_has_with_invalid_count_type() {
        let ctx = MockContext::new().with_item("BOMBS", 5);
        // String instead of number for count
        let args = vec![Expr::Ident("BOMBS".into()), Expr::String("10".into())];
        let result = eval_has(&args, &ctx);
        assert!(result.is_err());
    }

    #[test]
    fn test_has_with_bool_item_name() {
        let ctx = MockContext::new();
        // Bool instead of identifier
        let args = vec![Expr::Bool(true)];
        let result = eval_has(&args, &ctx);
        assert!(result.is_err());
    }

    #[test]
    fn test_can_use_all_adult_only_items() {
        let adult_items = [
            "HOOKSHOT",
            "LONGSHOT",
            "BOW",
            "HAMMER",
            "MEGATON_HAMMER",
            "IRON_BOOTS",
            "HOVER_BOOTS",
            "GORON_TUNIC",
            "ZORA_TUNIC",
            "MIRROR_SHIELD",
            "BIGGORON_SWORD",
            "GIANTS_KNIFE",
        ];

        for item in &adult_items {
            let ctx_adult = MockContext::new().with_item(item, 1).with_adult_age();
            let ctx_child = MockContext::new().with_item(item, 1).with_child_age();
            let args = vec![Expr::Ident(item.to_string())];

            assert!(
                eval_can_use(&args, &ctx_adult).unwrap(),
                "Adult should be able to use {}",
                item
            );
            assert!(
                !eval_can_use(&args, &ctx_child).unwrap(),
                "Child should not be able to use {}",
                item
            );
        }
    }

    #[test]
    fn test_can_use_all_child_only_items() {
        let child_items = [
            "SLINGSHOT",
            "FAIRY_SLINGSHOT",
            "BOOMERANG",
            "KOKIRI_SWORD",
            "DEKU_SHIELD",
        ];

        for item in &child_items {
            let ctx_adult = MockContext::new().with_item(item, 1).with_adult_age();
            let ctx_child = MockContext::new().with_item(item, 1).with_child_age();
            let args = vec![Expr::Ident(item.to_string())];

            assert!(
                !eval_can_use(&args, &ctx_adult).unwrap(),
                "Adult should not be able to use {}",
                item
            );
            assert!(
                eval_can_use(&args, &ctx_child).unwrap(),
                "Child should be able to use {}",
                item
            );
        }
    }

    #[test]
    fn test_can_use_any_age_items() {
        let any_age_items = [
            "BOMBS",
            "BOMBCHU",
            "DEKU_STICK",
            "DEKU_NUT",
            "OCARINA",
            "MAGIC_BEAN",
            "LENS_OF_TRUTH",
            "DINS_FIRE",
            "FARORES_WIND",
            "NAYRUS_LOVE",
        ];

        for item in &any_age_items {
            let ctx_adult = MockContext::new().with_item(item, 1).with_adult_age();
            let ctx_child = MockContext::new().with_item(item, 1).with_child_age();
            let args = vec![Expr::Ident(item.to_string())];

            assert!(
                eval_can_use(&args, &ctx_adult).unwrap(),
                "Adult should be able to use {}",
                item
            );
            assert!(
                eval_can_use(&args, &ctx_child).unwrap(),
                "Child should be able to use {}",
                item
            );
        }
    }

    #[test]
    fn test_can_use_with_string_arg() {
        let ctx = MockContext::new().with_item("HOOKSHOT", 1).with_adult_age();
        let args = vec![Expr::String("HOOKSHOT".into())];
        assert!(eval_can_use(&args, &ctx).unwrap());
    }

    #[test]
    fn test_can_use_invalid_arg_type() {
        let ctx = MockContext::new();
        let args = vec![Expr::Number(42)];
        let result = eval_can_use(&args, &ctx);
        assert!(result.is_err());
    }

    #[test]
    fn test_can_use_with_call_expr_arg() {
        let ctx = MockContext::new();
        // Passing a call expression as argument (invalid)
        let args = vec![Expr::call("inner", vec![])];
        let result = eval_can_use(&args, &ctx);
        assert!(result.is_err());
    }

    #[test]
    fn test_has_with_call_expr_arg() {
        let ctx = MockContext::new();
        // Passing a call expression as argument (invalid)
        let args = vec![Expr::call("inner", vec![])];
        let result = eval_has(&args, &ctx);
        assert!(result.is_err());
    }

    #[test]
    fn test_has_case_insensitive_matching() {
        // Item stored as lowercase
        let ctx = MockContext::new().with_item("hookshot", 1);
        // Queried as uppercase
        let args = vec![Expr::Ident("HOOKSHOT".into())];
        assert!(eval_has(&args, &ctx).unwrap());
    }

    #[test]
    fn test_extract_item_name_edge_cases() {
        // Valid identifier
        assert_eq!(
            extract_item_name(&Expr::Ident("ITEM".into())).unwrap(),
            "ITEM"
        );
        // Valid string
        assert_eq!(
            extract_item_name(&Expr::String("ITEM".into())).unwrap(),
            "ITEM"
        );
        // Invalid: boolean
        assert!(extract_item_name(&Expr::Bool(true)).is_err());
        // Invalid: number
        assert!(extract_item_name(&Expr::Number(42)).is_err());
        // Invalid: and expression
        assert!(extract_item_name(&Expr::and(Expr::Bool(true), Expr::Bool(false))).is_err());
    }

    #[test]
    fn test_extract_count_edge_cases() {
        // Valid positive
        assert_eq!(extract_count(&Expr::Number(10)).unwrap(), 10);
        // Valid zero
        assert_eq!(extract_count(&Expr::Number(0)).unwrap(), 0);
        // Invalid: negative
        assert!(extract_count(&Expr::Number(-5)).is_err());
        // Invalid: string
        assert!(extract_count(&Expr::String("10".into())).is_err());
        // Invalid: identifier
        assert!(extract_count(&Expr::Ident("count".into())).is_err());
    }

    #[test]
    fn test_age_restriction_all_adult_items() {
        assert_eq!(get_age_restriction("HOOKSHOT"), AgeRestriction::AdultOnly);
        assert_eq!(get_age_restriction("LONGSHOT"), AgeRestriction::AdultOnly);
        assert_eq!(get_age_restriction("BOW"), AgeRestriction::AdultOnly);
        assert_eq!(get_age_restriction("HAMMER"), AgeRestriction::AdultOnly);
        assert_eq!(
            get_age_restriction("MEGATON_HAMMER"),
            AgeRestriction::AdultOnly
        );
        assert_eq!(get_age_restriction("IRON_BOOTS"), AgeRestriction::AdultOnly);
        assert_eq!(
            get_age_restriction("HOVER_BOOTS"),
            AgeRestriction::AdultOnly
        );
        assert_eq!(
            get_age_restriction("GORON_TUNIC"),
            AgeRestriction::AdultOnly
        );
        assert_eq!(get_age_restriction("ZORA_TUNIC"), AgeRestriction::AdultOnly);
        assert_eq!(
            get_age_restriction("MIRROR_SHIELD"),
            AgeRestriction::AdultOnly
        );
        assert_eq!(
            get_age_restriction("BIGGORON_SWORD"),
            AgeRestriction::AdultOnly
        );
        assert_eq!(
            get_age_restriction("GIANTS_KNIFE"),
            AgeRestriction::AdultOnly
        );
    }

    #[test]
    fn test_age_restriction_all_child_items() {
        assert_eq!(get_age_restriction("SLINGSHOT"), AgeRestriction::ChildOnly);
        assert_eq!(
            get_age_restriction("FAIRY_SLINGSHOT"),
            AgeRestriction::ChildOnly
        );
        assert_eq!(get_age_restriction("BOOMERANG"), AgeRestriction::ChildOnly);
        assert_eq!(
            get_age_restriction("KOKIRI_SWORD"),
            AgeRestriction::ChildOnly
        );
        assert_eq!(
            get_age_restriction("DEKU_SHIELD"),
            AgeRestriction::ChildOnly
        );
    }

    #[test]
    fn test_age_restriction_none_default() {
        assert_eq!(get_age_restriction("BOMBS"), AgeRestriction::None);
        assert_eq!(
            get_age_restriction("TOTALLY_MADE_UP_ITEM"),
            AgeRestriction::None
        );
        assert_eq!(get_age_restriction(""), AgeRestriction::None);
    }

    #[test]
    fn test_has_empty_inventory() {
        let ctx = MockContext::new();
        // No items in inventory
        assert!(!eval_has(&[Expr::Ident("ANYTHING".into())], &ctx).unwrap());
    }

    #[test]
    fn test_can_use_without_item() {
        // Test that can_use returns false even with correct age if item not present
        let ctx = MockContext::new().with_adult_age();
        let args = vec![Expr::Ident("HOOKSHOT".into())];
        assert!(!eval_can_use(&args, &ctx).unwrap());
    }

    #[test]
    fn test_mock_context_builder_chain() {
        let ctx = MockContext::new()
            .with_item("A", 1)
            .with_item("B", 2)
            .with_item("C", 3)
            .with_child_age()
            .with_adult_age(); // Last one wins

        assert!(ctx.is_adult());
        assert!(!ctx.is_child());
        assert!(ctx.has_item("A", 1));
        assert!(ctx.has_item("B", 2));
        assert!(ctx.has_item("C", 3));
    }

    #[test]
    fn test_mock_context_event_always_false() {
        let ctx = MockContext::new();
        assert!(!ctx.event("ANY_EVENT"));
        assert!(!ctx.event("BOSS_DEFEATED"));
    }

    #[test]
    fn test_mock_context_setting_always_none() {
        let ctx = MockContext::new();
        assert!(ctx.setting("any_setting").is_none());
    }

    #[test]
    fn test_mock_context_trick_always_false() {
        let ctx = MockContext::new();
        assert!(!ctx.trick("any_trick"));
    }
}
