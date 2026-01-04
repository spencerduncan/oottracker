//! Expression evaluator.
//!
//! This module provides the core expression evaluator that ties together the lexer, parser,
//! and built-in functions to evaluate OoTMM condition expressions.

use crate::expr::builtins::{eval_can_use, eval_has};
use crate::expr::{parse, Expr, ParseError};
use thiserror::Error;

/// Error type for expression evaluation.
#[derive(Debug, Error)]
pub enum EvalError {
    /// Generic evaluation error.
    #[error("evaluation error: {0}")]
    Error(String),

    /// Unknown function called.
    #[error("unknown function: {0}")]
    UnknownFunction(String),

    /// Type error during evaluation.
    #[error("type error: expected {expected}, got {got}")]
    TypeError { expected: String, got: String },

    /// Unknown identifier encountered.
    #[error("unknown identifier: {0}")]
    UnknownIdent(String),

    /// Parse error when evaluating a string expression.
    #[error("parse error: {0}")]
    Parse(#[from] ParseError),
}

/// Context for expression evaluation.
///
/// Implement this trait to provide the game state that expressions are evaluated against.
pub trait EvalContext {
    /// Check if the player has at least `count` of the specified item.
    fn has_item(&self, item: &str, count: u32) -> bool;

    /// Check if a game event has occurred.
    fn event(&self, name: &str) -> bool;

    /// Get the value of a boolean setting. Returns `None` if the setting doesn't exist.
    ///
    /// This is used for `setting(name)` expressions that check if a setting is enabled.
    fn setting(&self, name: &str) -> Option<bool>;

    /// Check if a setting has a specific value.
    ///
    /// This is used for `setting(name, value)` expressions that check if a setting
    /// matches a specific value (e.g., `setting(openDungeonsOot, DC)`).
    ///
    /// Default implementation returns false for all queries.
    fn setting_value(&self, name: &str, value: &str) -> bool {
        let _ = (name, value);
        false
    }

    /// Check if a trick is enabled.
    fn trick(&self, name: &str) -> bool;

    /// Check if the player is currently Adult Link.
    fn is_adult(&self) -> bool;

    /// Check if the player is currently Child Link.
    fn is_child(&self) -> bool;

    // MM time-related methods

    /// Get the current MM time as a numeric value.
    /// Time is represented as minutes since the start of Day 1 at 6:00 AM.
    /// Each in-game day is 24 hours (1440 minutes), with the full 3-day cycle
    /// spanning 0 to 4319 (72 hours = 4320 minutes).
    fn mm_time(&self) -> u32;

    /// Check if it's currently daytime (6:00 AM to 6:00 PM).
    fn is_day(&self) -> bool {
        let time_in_day = self.mm_time() % 1440; // Minutes within current day
        time_in_day < 720 // 0-719 is day (6 AM to 6 PM)
    }

    /// Check if it's currently nighttime (6:00 PM to 6:00 AM).
    fn is_night(&self) -> bool {
        !self.is_day()
    }
}

/// Expression evaluator that ties together lexer, parser, and built-in functions.
///
/// The `Evaluator` provides the core evaluation logic for OoTMM condition expressions.
/// It recursively evaluates AST nodes against a provided context.
///
/// # Example
///
/// ```ignore
/// use ootmm::expr::{Evaluator, EvalContext};
///
/// let evaluator = Evaluator::new(&ctx);
/// let result = evaluator.eval_str("has(HOOKSHOT) && is_adult")?;
/// ```
pub struct Evaluator<'a, C: EvalContext> {
    ctx: &'a C,
}

impl<'a, C: EvalContext> Evaluator<'a, C> {
    /// Create a new evaluator with the given context.
    pub fn new(ctx: &'a C) -> Self {
        Self { ctx }
    }

    /// Parse and evaluate an expression string.
    ///
    /// This is a convenience method that combines parsing and evaluation.
    pub fn eval_str(&self, input: &str) -> Result<bool, EvalError> {
        let expr = parse(input)?;
        self.eval(&expr)
    }

    /// Evaluate an expression AST node.
    ///
    /// Recursively evaluates the expression tree against the context.
    pub fn eval(&self, expr: &Expr) -> Result<bool, EvalError> {
        match expr {
            Expr::Bool(b) => Ok(*b),

            Expr::Number(n) => {
                // Non-zero numbers are truthy
                Ok(*n != 0)
            }

            Expr::String(s) => {
                // Non-empty strings are truthy
                Ok(!s.is_empty())
            }

            Expr::Ident(name) => self.eval_ident(name),

            Expr::And(left, right) => {
                // Short-circuit AND: only evaluate right if left is true
                if self.eval(left)? {
                    self.eval(right)
                } else {
                    Ok(false)
                }
            }

            Expr::Or(left, right) => {
                // Short-circuit OR: only evaluate right if left is false
                if self.eval(left)? {
                    Ok(true)
                } else {
                    self.eval(right)
                }
            }

            Expr::Not(inner) => Ok(!self.eval(inner)?),

            Expr::Call { name, args } => self.eval_call(name, args),
        }
    }

    /// Evaluate an identifier.
    ///
    /// Special identifiers are handled directly:
    /// - `is_adult`: true if the player is Adult Link
    /// - `is_child`: true if the player is Child Link
    /// - `is_human`: always true (player can always be in human form in MM tracker)
    /// - `true`: always true
    /// - `false`: always false
    fn eval_ident(&self, name: &str) -> Result<bool, EvalError> {
        match name {
            "is_adult" => Ok(self.ctx.is_adult()),
            "is_child" => Ok(self.ctx.is_child()),
            "is_human" => Ok(true),
            "true" => Ok(true),
            "false" => Ok(false),
            _ => {
                // Check if it's an event or setting
                if self.ctx.event(name) {
                    return Ok(true);
                }
                if let Some(val) = self.ctx.setting(name) {
                    return Ok(val);
                }
                // Unknown identifier - could be an item check shorthand
                // For now, treat unknown identifiers as events that haven't occurred
                Ok(false)
            }
        }
    }

    /// Evaluate a function call.
    fn eval_call(&self, name: &str, args: &[Expr]) -> Result<bool, EvalError> {
        match name {
            "has" => eval_has(args, self.ctx),
            "can_use" => eval_can_use(args, self.ctx),
            "event" => self.eval_event(args),
            "setting" => self.eval_setting(args),
            "trick" => self.eval_trick(args),
            "cond" => self.eval_cond(args),
            _ => Err(EvalError::UnknownFunction(name.to_string())),
        }
    }

    /// Evaluate the `event` built-in function.
    ///
    /// Checks if a game event has occurred.
    ///
    /// # Syntax
    /// - `event(EVENT_NAME)` - true if the event has occurred
    fn eval_event(&self, args: &[Expr]) -> Result<bool, EvalError> {
        if args.len() != 1 {
            return Err(EvalError::Error(format!(
                "event() expects 1 argument, got {}",
                args.len()
            )));
        }

        let event_name = self.extract_name(&args[0])?;
        Ok(self.ctx.event(&event_name))
    }

    /// Evaluate the `setting` built-in function.
    ///
    /// Checks the value of a game setting.
    ///
    /// # Syntax
    /// - `setting(SETTING_NAME)` - true if the boolean setting is enabled
    /// - `setting(SETTING_NAME, VALUE)` - true if the setting has the specified value
    fn eval_setting(&self, args: &[Expr]) -> Result<bool, EvalError> {
        match args.len() {
            1 => {
                // Boolean setting check: setting(name)
                let setting_name = self.extract_name(&args[0])?;
                Ok(self.ctx.setting(&setting_name).unwrap_or(false))
            }
            2 => {
                // Value setting check: setting(name, value)
                let setting_name = self.extract_name(&args[0])?;
                let setting_value = self.extract_name(&args[1])?;
                Ok(self.ctx.setting_value(&setting_name, &setting_value))
            }
            _ => Err(EvalError::Error(format!(
                "setting() expects 1 or 2 arguments, got {}",
                args.len()
            ))),
        }
    }

    /// Evaluate the `trick` built-in function.
    ///
    /// Checks if a trick is enabled.
    ///
    /// # Syntax
    /// - `trick(TRICK_NAME)` - true if the trick is enabled
    fn eval_trick(&self, args: &[Expr]) -> Result<bool, EvalError> {
        if args.len() != 1 {
            return Err(EvalError::Error(format!(
                "trick() expects 1 argument, got {}",
                args.len()
            )));
        }

        let trick_name = self.extract_name(&args[0])?;
        Ok(self.ctx.trick(&trick_name))
    }

    /// Evaluate the `cond` built-in function.
    ///
    /// Conditional expression: `cond(condition, then_value, else_value)`
    ///
    /// # Syntax
    /// - `cond(test, if_true, if_false)` - returns if_true if test is true, else if_false
    fn eval_cond(&self, args: &[Expr]) -> Result<bool, EvalError> {
        if args.len() != 3 {
            return Err(EvalError::Error(format!(
                "cond() expects 3 arguments, got {}",
                args.len()
            )));
        }

        if self.eval(&args[0])? {
            self.eval(&args[1])
        } else {
            self.eval(&args[2])
        }
    }

    /// Extract a name (identifier or string) from an expression.
    fn extract_name(&self, expr: &Expr) -> Result<String, EvalError> {
        match expr {
            Expr::Ident(name) => Ok(name.clone()),
            Expr::String(s) => Ok(s.clone()),
            _ => Err(EvalError::TypeError {
                expected: "identifier or string".to_string(),
                got: format!("{:?}", expr),
            }),
        }
    }
}

/// Evaluate an expression against a context.
///
/// This is a convenience function that creates an `Evaluator` and evaluates the expression.
pub fn eval(expr: &Expr, ctx: &impl EvalContext) -> Result<bool, EvalError> {
    let evaluator = Evaluator::new(ctx);
    evaluator.eval(expr)
}

/// Parse and evaluate an expression string against a context.
///
/// This is a convenience function that combines parsing and evaluation.
pub fn eval_str(input: &str, ctx: &impl EvalContext) -> Result<bool, EvalError> {
    let evaluator = Evaluator::new(ctx);
    evaluator.eval_str(input)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::{HashMap, HashSet};

    /// Mock context for testing the evaluator.
    struct MockContext {
        items: HashMap<String, u32>,
        events: HashSet<String>,
        settings: HashMap<String, bool>,
        /// Settings with values: key is "settingName:value", value is true if that combination is valid
        setting_values: HashSet<String>,
        tricks: HashSet<String>,
        is_adult: bool,
    }

    impl MockContext {
        fn new() -> Self {
            Self {
                items: HashMap::new(),
                events: HashSet::new(),
                settings: HashMap::new(),
                setting_values: HashSet::new(),
                tricks: HashSet::new(),
                is_adult: true,
            }
        }

        fn with_item(mut self, item: &str, count: u32) -> Self {
            self.items.insert(item.to_uppercase(), count);
            self
        }

        fn with_event(mut self, event: &str) -> Self {
            self.events.insert(event.to_string());
            self
        }

        fn with_setting(mut self, setting: &str, value: bool) -> Self {
            self.settings.insert(setting.to_string(), value);
            self
        }

        /// Sets a setting to have a specific value (for 2-argument setting checks).
        fn with_setting_value(mut self, setting: &str, value: &str) -> Self {
            self.setting_values.insert(format!("{}:{}", setting, value));
            self
        }

        fn with_trick(mut self, trick: &str) -> Self {
            self.tricks.insert(trick.to_string());
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

        fn event(&self, name: &str) -> bool {
            self.events.contains(name)
        }

        fn setting(&self, name: &str) -> Option<bool> {
            self.settings.get(name).copied()
        }

        fn setting_value(&self, name: &str, value: &str) -> bool {
            self.setting_values.contains(&format!("{}:{}", name, value))
        }

        fn trick(&self, name: &str) -> bool {
            self.tricks.contains(name)
        }

        fn is_adult(&self) -> bool {
            self.is_adult
        }

        fn is_child(&self) -> bool {
            !self.is_adult
        }

        fn mm_time(&self) -> u32 {
            0 // Default to time 0 for tests
        }
    }

    // --- Boolean literal tests ---

    #[test]
    fn test_eval_true() {
        let ctx = MockContext::new();
        assert!(eval_str("true", &ctx).unwrap());
    }

    #[test]
    fn test_eval_false() {
        let ctx = MockContext::new();
        assert!(!eval_str("false", &ctx).unwrap());
    }

    // --- Identifier tests ---

    #[test]
    fn test_eval_is_adult() {
        let ctx = MockContext::new().with_adult_age();
        assert!(eval_str("is_adult", &ctx).unwrap());

        let ctx = MockContext::new().with_child_age();
        assert!(!eval_str("is_adult", &ctx).unwrap());
    }

    #[test]
    fn test_eval_is_child() {
        let ctx = MockContext::new().with_child_age();
        assert!(eval_str("is_child", &ctx).unwrap());

        let ctx = MockContext::new().with_adult_age();
        assert!(!eval_str("is_child", &ctx).unwrap());
    }

    #[test]
    fn test_eval_is_human() {
        // is_human always returns true for MM tracker purposes
        let ctx = MockContext::new();
        assert!(eval_str("is_human", &ctx).unwrap());

        // Should work in combination with other conditions
        let ctx = MockContext::new().with_item("HOOKSHOT", 1);
        assert!(eval_str("is_human && has(HOOKSHOT)", &ctx).unwrap());
    }

    #[test]
    fn test_eval_unknown_ident() {
        let ctx = MockContext::new();
        // Unknown identifiers default to false
        assert!(!eval_str("unknown_thing", &ctx).unwrap());
    }

    // --- Logical operator tests ---

    #[test]
    fn test_eval_and_true_true() {
        let ctx = MockContext::new();
        assert!(eval_str("true && true", &ctx).unwrap());
    }

    #[test]
    fn test_eval_and_true_false() {
        let ctx = MockContext::new();
        assert!(!eval_str("true && false", &ctx).unwrap());
    }

    #[test]
    fn test_eval_and_false_true() {
        let ctx = MockContext::new();
        assert!(!eval_str("false && true", &ctx).unwrap());
    }

    #[test]
    fn test_eval_and_false_false() {
        let ctx = MockContext::new();
        assert!(!eval_str("false && false", &ctx).unwrap());
    }

    #[test]
    fn test_eval_or_true_true() {
        let ctx = MockContext::new();
        assert!(eval_str("true || true", &ctx).unwrap());
    }

    #[test]
    fn test_eval_or_true_false() {
        let ctx = MockContext::new();
        assert!(eval_str("true || false", &ctx).unwrap());
    }

    #[test]
    fn test_eval_or_false_true() {
        let ctx = MockContext::new();
        assert!(eval_str("false || true", &ctx).unwrap());
    }

    #[test]
    fn test_eval_or_false_false() {
        let ctx = MockContext::new();
        assert!(!eval_str("false || false", &ctx).unwrap());
    }

    #[test]
    fn test_eval_not_true() {
        let ctx = MockContext::new();
        assert!(!eval_str("!true", &ctx).unwrap());
    }

    #[test]
    fn test_eval_not_false() {
        let ctx = MockContext::new();
        assert!(eval_str("!false", &ctx).unwrap());
    }

    #[test]
    fn test_eval_double_not() {
        let ctx = MockContext::new();
        assert!(eval_str("!!true", &ctx).unwrap());
        assert!(!eval_str("!!false", &ctx).unwrap());
    }

    // --- Short-circuit evaluation tests ---

    #[test]
    fn test_and_short_circuit() {
        // When left side is false, right side should not be evaluated
        let ctx = MockContext::new();
        // This would fail if unknown_func was evaluated, but it shouldn't be
        // because false && anything = false
        let expr = parse("false && unknown_func()").unwrap();
        assert!(!eval(&expr, &ctx).unwrap());
    }

    #[test]
    fn test_or_short_circuit() {
        // When left side is true, right side should not be evaluated
        let ctx = MockContext::new();
        // This would fail if unknown_func was evaluated, but it shouldn't be
        // because true || anything = true
        let expr = parse("true || unknown_func()").unwrap();
        assert!(eval(&expr, &ctx).unwrap());
    }

    // --- has() function tests ---

    #[test]
    fn test_eval_has_item() {
        let ctx = MockContext::new().with_item("HOOKSHOT", 1);
        assert!(eval_str("has(HOOKSHOT)", &ctx).unwrap());
    }

    #[test]
    fn test_eval_has_missing_item() {
        let ctx = MockContext::new();
        assert!(!eval_str("has(HOOKSHOT)", &ctx).unwrap());
    }

    #[test]
    fn test_eval_has_with_count() {
        let ctx = MockContext::new().with_item("BOMBS", 20);
        assert!(eval_str("has(BOMBS, 10)", &ctx).unwrap());
        assert!(eval_str("has(BOMBS, 20)", &ctx).unwrap());
        assert!(!eval_str("has(BOMBS, 30)", &ctx).unwrap());
    }

    // --- can_use() function tests ---

    #[test]
    fn test_eval_can_use_adult_item_as_adult() {
        let ctx = MockContext::new().with_item("HOOKSHOT", 1).with_adult_age();
        assert!(eval_str("can_use(HOOKSHOT)", &ctx).unwrap());
    }

    #[test]
    fn test_eval_can_use_adult_item_as_child() {
        let ctx = MockContext::new().with_item("HOOKSHOT", 1).with_child_age();
        assert!(!eval_str("can_use(HOOKSHOT)", &ctx).unwrap());
    }

    #[test]
    fn test_eval_can_use_child_item_as_child() {
        let ctx = MockContext::new()
            .with_item("BOOMERANG", 1)
            .with_child_age();
        assert!(eval_str("can_use(BOOMERANG)", &ctx).unwrap());
    }

    #[test]
    fn test_eval_can_use_child_item_as_adult() {
        let ctx = MockContext::new()
            .with_item("BOOMERANG", 1)
            .with_adult_age();
        assert!(!eval_str("can_use(BOOMERANG)", &ctx).unwrap());
    }

    // --- event() function tests ---

    #[test]
    fn test_eval_event_occurred() {
        let ctx = MockContext::new().with_event("MIDO_MOVED");
        assert!(eval_str("event(MIDO_MOVED)", &ctx).unwrap());
    }

    #[test]
    fn test_eval_event_not_occurred() {
        let ctx = MockContext::new();
        assert!(!eval_str("event(MIDO_MOVED)", &ctx).unwrap());
    }

    // --- setting() function tests ---

    #[test]
    fn test_eval_setting_enabled() {
        let ctx = MockContext::new().with_setting("skip_child_zelda", true);
        assert!(eval_str("setting(skip_child_zelda)", &ctx).unwrap());
    }

    #[test]
    fn test_eval_setting_disabled() {
        let ctx = MockContext::new().with_setting("skip_child_zelda", false);
        assert!(!eval_str("setting(skip_child_zelda)", &ctx).unwrap());
    }

    #[test]
    fn test_eval_setting_missing() {
        let ctx = MockContext::new();
        assert!(!eval_str("setting(nonexistent)", &ctx).unwrap());
    }

    // --- trick() function tests ---

    #[test]
    fn test_eval_trick_enabled() {
        let ctx = MockContext::new().with_trick("hover_boost");
        assert!(eval_str("trick(hover_boost)", &ctx).unwrap());
    }

    #[test]
    fn test_eval_trick_disabled() {
        let ctx = MockContext::new();
        assert!(!eval_str("trick(hover_boost)", &ctx).unwrap());
    }

    // --- cond() function tests ---

    #[test]
    fn test_eval_cond_true() {
        let ctx = MockContext::new();
        assert!(eval_str("cond(true, true, false)", &ctx).unwrap());
    }

    #[test]
    fn test_eval_cond_false() {
        let ctx = MockContext::new();
        assert!(!eval_str("cond(false, true, false)", &ctx).unwrap());
    }

    #[test]
    fn test_eval_cond_with_expressions() {
        let ctx = MockContext::new()
            .with_item("HOOKSHOT", 1)
            .with_event("BOSS_DEFEATED");
        assert!(eval_str("cond(has(HOOKSHOT), event(BOSS_DEFEATED), false)", &ctx).unwrap());
    }

    // --- Complex expression tests ---

    #[test]
    fn test_eval_complex_and_or() {
        let ctx = MockContext::new()
            .with_item("HOOKSHOT", 1)
            .with_item("BOW", 1);

        assert!(eval_str("has(HOOKSHOT) && has(BOW)", &ctx).unwrap());
        assert!(eval_str("has(HOOKSHOT) || has(BOMBS)", &ctx).unwrap());
        assert!(!eval_str("has(HOOKSHOT) && has(BOMBS)", &ctx).unwrap());
    }

    #[test]
    fn test_eval_complex_with_not() {
        let ctx = MockContext::new().with_item("HOOKSHOT", 1);

        assert!(eval_str("!has(BOMBS)", &ctx).unwrap());
        assert!(!eval_str("!has(HOOKSHOT)", &ctx).unwrap());
        assert!(eval_str("has(HOOKSHOT) && !has(BOMBS)", &ctx).unwrap());
    }

    #[test]
    fn test_eval_nested_parentheses() {
        let ctx = MockContext::new()
            .with_item("HOOKSHOT", 1)
            .with_event("MIDO_MOVED");

        assert!(eval_str("(has(HOOKSHOT) && event(MIDO_MOVED))", &ctx).unwrap());
        assert!(eval_str("((has(HOOKSHOT)))", &ctx).unwrap());
    }

    #[test]
    fn test_eval_real_world_expression() {
        // A realistic OoTMM logic expression
        let ctx = MockContext::new()
            .with_item("HOOKSHOT", 1)
            .with_adult_age()
            .with_event("FOREST_TEMPLE_CLEAR");

        assert!(eval_str(
            "is_adult && has(HOOKSHOT) && event(FOREST_TEMPLE_CLEAR)",
            &ctx
        )
        .unwrap());
    }

    #[test]
    fn test_eval_or_with_setting() {
        let ctx = MockContext::new()
            .with_setting("skip_child_zelda", true)
            .with_event("MET_ZELDA");

        assert!(eval_str("event(MET_ZELDA) || setting(skip_child_zelda)", &ctx).unwrap());

        let ctx2 = MockContext::new().with_setting("skip_child_zelda", true);
        assert!(eval_str("event(MET_ZELDA) || setting(skip_child_zelda)", &ctx2).unwrap());

        let ctx3 = MockContext::new();
        assert!(!eval_str("event(MET_ZELDA) || setting(skip_child_zelda)", &ctx3).unwrap());
    }

    // --- Error handling tests ---

    #[test]
    fn test_eval_unknown_function() {
        let ctx = MockContext::new();
        let result = eval_str("unknown_func()", &ctx);
        assert!(matches!(result, Err(EvalError::UnknownFunction(_))));
    }

    #[test]
    fn test_eval_event_wrong_args() {
        let ctx = MockContext::new();
        let result = eval_str("event()", &ctx);
        assert!(result.is_err());

        let result = eval_str("event(a, b)", &ctx);
        assert!(result.is_err());
    }

    #[test]
    fn test_eval_setting_wrong_args() {
        let ctx = MockContext::new();
        let result = eval_str("setting()", &ctx);
        assert!(result.is_err());
    }

    #[test]
    fn test_eval_trick_wrong_args() {
        let ctx = MockContext::new();
        let result = eval_str("trick()", &ctx);
        assert!(result.is_err());
    }

    #[test]
    fn test_eval_cond_wrong_args() {
        let ctx = MockContext::new();
        let result = eval_str("cond(true, false)", &ctx);
        assert!(result.is_err());
    }

    #[test]
    fn test_eval_parse_error() {
        let ctx = MockContext::new();
        let result = eval_str("has(HOOKSHOT &&", &ctx);
        assert!(matches!(result, Err(EvalError::Parse(_))));
    }

    // --- Evaluator struct tests ---

    #[test]
    fn test_evaluator_new() {
        let ctx = MockContext::new();
        let evaluator = Evaluator::new(&ctx);
        assert!(evaluator.eval_str("true").unwrap());
    }

    #[test]
    fn test_evaluator_eval_expr() {
        let ctx = MockContext::new().with_item("HOOKSHOT", 1);
        let evaluator = Evaluator::new(&ctx);
        let expr = parse("has(HOOKSHOT)").unwrap();
        assert!(evaluator.eval(&expr).unwrap());
    }

    // --- Number and String truthiness tests ---

    #[test]
    fn test_eval_number_truthiness() {
        let ctx = MockContext::new();
        let evaluator = Evaluator::new(&ctx);

        // Non-zero numbers are truthy
        assert!(evaluator.eval(&Expr::Number(1)).unwrap());
        assert!(evaluator.eval(&Expr::Number(42)).unwrap());
        assert!(evaluator.eval(&Expr::Number(-1)).unwrap());

        // Zero is falsy
        assert!(!evaluator.eval(&Expr::Number(0)).unwrap());
    }

    #[test]
    fn test_eval_string_truthiness() {
        let ctx = MockContext::new();
        let evaluator = Evaluator::new(&ctx);

        // Non-empty strings are truthy
        assert!(evaluator.eval(&Expr::String("hello".to_string())).unwrap());
        assert!(evaluator.eval(&Expr::String(" ".to_string())).unwrap());

        // Empty string is falsy
        assert!(!evaluator.eval(&Expr::String("".to_string())).unwrap());
    }

    // --- Integration tests ---

    #[test]
    fn test_full_evaluation_pipeline() {
        // Test the complete pipeline: string -> parse -> eval
        let ctx = MockContext::new()
            .with_item("HOOKSHOT", 1)
            .with_item("BOW", 1)
            .with_adult_age()
            .with_event("FOREST_TEMPLE_CLEAR")
            .with_setting("shuffle_songs", true);

        let expressions = vec![
            ("true", true),
            ("false", false),
            ("is_adult", true),
            ("is_child", false),
            ("has(HOOKSHOT)", true),
            ("has(BOMBS)", false),
            ("can_use(HOOKSHOT)", true),
            ("event(FOREST_TEMPLE_CLEAR)", true),
            ("setting(shuffle_songs)", true),
            ("has(HOOKSHOT) && has(BOW) && is_adult", true),
            ("event(FOREST_TEMPLE_CLEAR) || has(BOMBS)", true),
            ("!(has(BOMBS))", true),
        ];

        for (expr_str, expected) in expressions {
            let result = eval_str(expr_str, &ctx).unwrap();
            assert_eq!(result, expected, "Expression '{}' failed", expr_str);
        }
    }

    // --- Two-argument setting() tests ---

    #[test]
    fn test_eval_setting_two_args_matches() {
        let ctx = MockContext::new()
            .with_setting_value("openDungeonsOot", "DC")
            .with_setting_value("openDungeonsOot", "BotW");

        assert!(eval_str("setting(openDungeonsOot, DC)", &ctx).unwrap());
        assert!(eval_str("setting(openDungeonsOot, BotW)", &ctx).unwrap());
    }

    #[test]
    fn test_eval_setting_two_args_no_match() {
        let ctx = MockContext::new().with_setting_value("openDungeonsOot", "DC");

        assert!(!eval_str("setting(openDungeonsOot, Shadow)", &ctx).unwrap());
        assert!(!eval_str("setting(openDungeonsMm, ST)", &ctx).unwrap());
    }

    #[test]
    fn test_eval_setting_two_args_enum_value() {
        let ctx = MockContext::new()
            .with_setting_value("ganonBossKey", "removed")
            .with_setting_value("ageChange", "none");

        assert!(eval_str("setting(ganonBossKey, removed)", &ctx).unwrap());
        assert!(!eval_str("setting(ganonBossKey, custom)", &ctx).unwrap());
        assert!(eval_str("setting(ageChange, none)", &ctx).unwrap());
    }

    #[test]
    fn test_eval_setting_two_args_in_complex_expr() {
        let ctx = MockContext::new()
            .with_setting_value("openDungeonsOot", "DC")
            .with_setting_value("beneathWell", "open")
            .with_item("HOOKSHOT", 1);

        // Test setting in AND expression
        assert!(eval_str("setting(openDungeonsOot, DC) && has(HOOKSHOT)", &ctx).unwrap());
        assert!(!eval_str("setting(openDungeonsOot, Shadow) && has(HOOKSHOT)", &ctx).unwrap());

        // Test setting in OR expression
        assert!(eval_str("setting(beneathWell, open) || has(BOMBS)", &ctx).unwrap());

        // Test negated setting
        assert!(eval_str("!setting(openDungeonsOot, Shadow)", &ctx).unwrap());
        assert!(!eval_str("!setting(openDungeonsOot, DC)", &ctx).unwrap());
    }

    #[test]
    fn test_eval_setting_two_args_like_world_data() {
        // Test expressions that match patterns from real world data files
        let ctx = MockContext::new()
            .with_setting_value("climbMostSurfacesOot", "off")
            .with_setting_value("hookshotAnywhereOot", "off")
            .with_setting_value("ageChange", "none");

        // Pattern: !setting(climbMostSurfacesOot, off)
        assert!(!eval_str("!setting(climbMostSurfacesOot, off)", &ctx).unwrap());

        // Pattern: !setting(hookshotAnywhereOot, off) && !setting(ageChange, none)
        assert!(!eval_str(
            "!setting(hookshotAnywhereOot, off) && !setting(ageChange, none)",
            &ctx
        )
        .unwrap());
    }

    #[test]
    fn test_eval_setting_mixed_one_and_two_args() {
        let ctx = MockContext::new()
            .with_setting("agelessBoots", true)
            .with_setting_value("openDungeonsOot", "DC");

        // Mix of 1-arg and 2-arg settings
        assert!(eval_str(
            "setting(agelessBoots) && setting(openDungeonsOot, DC)",
            &ctx
        )
        .unwrap());
        assert!(!eval_str(
            "setting(agelessBoots) && setting(openDungeonsOot, Shadow)",
            &ctx
        )
        .unwrap());
    }

    #[test]
    fn test_eval_setting_three_args_error() {
        let ctx = MockContext::new();
        let result = eval_str("setting(a, b, c)", &ctx);
        assert!(result.is_err());
    }
}
