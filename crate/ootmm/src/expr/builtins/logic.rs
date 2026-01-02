//! Logic built-in functions (event, setting, trick).
//!
//! These functions check game events and randomizer settings for OoTMM logic expressions.

use crate::expr::{EvalContext, EvalError, Expr};

/// Evaluate the `event` built-in function.
///
/// Checks if a specific game event has occurred. Events represent game state
/// changes like completing objectives, defeating bosses, or triggering cutscenes.
///
/// # Syntax
/// - `event(EVENT_NAME)` - checks if the specified event has occurred
///
/// # Examples
/// - `event(MIDO_MOVED)` - true if Mido has moved from blocking the path
/// - `event(FOREST_TEMPLE_CLEAR)` - true if the Forest Temple has been cleared
/// - `event(ZELDA_FLED)` - true if Zelda has fled the castle
pub fn eval_event(args: &[Expr], ctx: &impl EvalContext) -> Result<bool, EvalError> {
    if args.len() != 1 {
        return Err(EvalError::Error(format!(
            "event() expects 1 argument, got {}",
            args.len()
        )));
    }

    let event_name = extract_name(&args[0])?;
    Ok(ctx.event(&event_name))
}

/// Evaluate the `setting` built-in function.
///
/// Checks if a specific randomizer setting is enabled. Settings control various
/// aspects of the randomizer logic, such as skipping certain sections or enabling
/// specific game mechanics.
///
/// # Syntax
/// - `setting(SETTING_NAME)` - checks if the specified setting is enabled
///
/// # Return Value
/// Returns `true` if the setting exists and is enabled, `false` otherwise.
/// If the setting doesn't exist in the context, this returns `false`.
///
/// # Examples
/// - `setting(skip_child_zelda)` - true if the skip child zelda setting is enabled
/// - `setting(shuffle_ocarinas)` - true if ocarina shuffle is enabled
/// - `setting(open_door_of_time)` - true if Door of Time is open by default
pub fn eval_setting(args: &[Expr], ctx: &impl EvalContext) -> Result<bool, EvalError> {
    if args.len() != 1 {
        return Err(EvalError::Error(format!(
            "setting() expects 1 argument, got {}",
            args.len()
        )));
    }

    let setting_name = extract_name(&args[0])?;
    // If setting doesn't exist, treat as false
    Ok(ctx.setting(&setting_name).unwrap_or(false))
}

/// Extract a name (event, setting, etc.) from an expression.
fn extract_name(expr: &Expr) -> Result<String, EvalError> {
    match expr {
        Expr::Ident(name) => Ok(name.clone()),
        Expr::String(s) => Ok(s.clone()),
        _ => Err(EvalError::Error(format!(
            "expected name (identifier or string), got {:?}",
            expr
        ))),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::{HashMap, HashSet};

    /// Mock context for testing logic built-in functions.
    struct MockContext {
        events: HashSet<String>,
        settings: HashMap<String, bool>,
    }

    impl MockContext {
        fn new() -> Self {
            Self {
                events: HashSet::new(),
                settings: HashMap::new(),
            }
        }

        fn with_event(mut self, event: &str) -> Self {
            self.events.insert(event.to_string());
            self
        }

        fn with_setting(mut self, setting: &str, value: bool) -> Self {
            self.settings.insert(setting.to_string(), value);
            self
        }
    }

    impl EvalContext for MockContext {
        fn has_item(&self, _item: &str, _count: u32) -> bool {
            false
        }

        fn event(&self, name: &str) -> bool {
            self.events.contains(name)
        }

        fn setting(&self, name: &str) -> Option<bool> {
            self.settings.get(name).copied()
        }

        fn trick(&self, _name: &str) -> bool {
            false
        }

        fn is_adult(&self) -> bool {
            true
        }

        fn is_child(&self) -> bool {
            false
        }

        fn mm_time(&self) -> u32 {
            0 // Default to time 0 for tests
        }
    }

    // --- event() tests ---

    #[test]
    fn test_event_exists() {
        let ctx = MockContext::new().with_event("MIDO_MOVED");
        let args = vec![Expr::Ident("MIDO_MOVED".into())];

        assert!(eval_event(&args, &ctx).unwrap());
    }

    #[test]
    fn test_event_not_exists() {
        let ctx = MockContext::new();
        let args = vec![Expr::Ident("MIDO_MOVED".into())];

        assert!(!eval_event(&args, &ctx).unwrap());
    }

    #[test]
    fn test_event_with_string_arg() {
        let ctx = MockContext::new().with_event("FOREST_TEMPLE_CLEAR");
        let args = vec![Expr::String("FOREST_TEMPLE_CLEAR".into())];

        assert!(eval_event(&args, &ctx).unwrap());
    }

    #[test]
    fn test_event_wrong_arg_count() {
        let ctx = MockContext::new();

        // No arguments
        let result = eval_event(&[], &ctx);
        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("expects 1 argument"));

        // Too many arguments
        let args = vec![Expr::Ident("EVENT1".into()), Expr::Ident("EVENT2".into())];
        let result = eval_event(&args, &ctx);
        assert!(result.is_err());
    }

    #[test]
    fn test_event_invalid_arg_type() {
        let ctx = MockContext::new();
        let args = vec![Expr::Number(42)];

        let result = eval_event(&args, &ctx);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("expected name"));
    }

    #[test]
    fn test_event_multiple_events() {
        let ctx = MockContext::new()
            .with_event("EVENT_A")
            .with_event("EVENT_B");

        let args_a = vec![Expr::Ident("EVENT_A".into())];
        let args_b = vec![Expr::Ident("EVENT_B".into())];
        let args_c = vec![Expr::Ident("EVENT_C".into())];

        assert!(eval_event(&args_a, &ctx).unwrap());
        assert!(eval_event(&args_b, &ctx).unwrap());
        assert!(!eval_event(&args_c, &ctx).unwrap());
    }

    // --- setting() tests ---

    #[test]
    fn test_setting_enabled() {
        let ctx = MockContext::new().with_setting("skip_child_zelda", true);
        let args = vec![Expr::Ident("skip_child_zelda".into())];

        assert!(eval_setting(&args, &ctx).unwrap());
    }

    #[test]
    fn test_setting_disabled() {
        let ctx = MockContext::new().with_setting("skip_child_zelda", false);
        let args = vec![Expr::Ident("skip_child_zelda".into())];

        assert!(!eval_setting(&args, &ctx).unwrap());
    }

    #[test]
    fn test_setting_not_exists() {
        let ctx = MockContext::new();
        let args = vec![Expr::Ident("unknown_setting".into())];

        // Non-existent settings should return false
        assert!(!eval_setting(&args, &ctx).unwrap());
    }

    #[test]
    fn test_setting_with_string_arg() {
        let ctx = MockContext::new().with_setting("shuffle_ocarinas", true);
        let args = vec![Expr::String("shuffle_ocarinas".into())];

        assert!(eval_setting(&args, &ctx).unwrap());
    }

    #[test]
    fn test_setting_wrong_arg_count() {
        let ctx = MockContext::new();

        // No arguments
        let result = eval_setting(&[], &ctx);
        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("expects 1 argument"));

        // Too many arguments
        let args = vec![
            Expr::Ident("setting1".into()),
            Expr::Ident("setting2".into()),
        ];
        let result = eval_setting(&args, &ctx);
        assert!(result.is_err());
    }

    #[test]
    fn test_setting_invalid_arg_type() {
        let ctx = MockContext::new();
        let args = vec![Expr::Bool(true)];

        let result = eval_setting(&args, &ctx);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("expected name"));
    }

    #[test]
    fn test_setting_multiple_settings() {
        let ctx = MockContext::new()
            .with_setting("setting_a", true)
            .with_setting("setting_b", false)
            .with_setting("setting_c", true);

        let args_a = vec![Expr::Ident("setting_a".into())];
        let args_b = vec![Expr::Ident("setting_b".into())];
        let args_c = vec![Expr::Ident("setting_c".into())];

        assert!(eval_setting(&args_a, &ctx).unwrap());
        assert!(!eval_setting(&args_b, &ctx).unwrap());
        assert!(eval_setting(&args_c, &ctx).unwrap());
    }

    // --- extract_name() tests ---

    #[test]
    fn test_extract_name_from_ident() {
        let expr = Expr::Ident("test_name".into());
        assert_eq!(extract_name(&expr).unwrap(), "test_name");
    }

    #[test]
    fn test_extract_name_from_string() {
        let expr = Expr::String("test_name".into());
        assert_eq!(extract_name(&expr).unwrap(), "test_name");
    }

    #[test]
    fn test_extract_name_from_invalid() {
        let expr = Expr::Number(42);
        assert!(extract_name(&expr).is_err());

        let expr = Expr::Bool(true);
        assert!(extract_name(&expr).is_err());
    }
}
