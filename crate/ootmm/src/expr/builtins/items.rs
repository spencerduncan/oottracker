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
}
