//! MM time-related built-in functions (is_day, is_night, between, at, mm_time).
//!
//! These functions handle Majora's Mask time-based conditions for OoTMM logic expressions.
//!
//! # Time Representation
//!
//! Time is represented as minutes since the start of Day 1 at 6:00 AM.
//! Each in-game day spans 24 hours (1440 minutes):
//! - Day 1: 0-1439
//! - Day 2: 1440-2879
//! - Day 3: 2880-4319
//!
//! Within each day, time is relative to 6:00 AM:
//! - 6:00 AM = 0 (day starts)
//! - 12:00 PM = 360 (noon)
//! - 6:00 PM = 720 (night starts)
//! - 12:00 AM = 1080 (midnight)
//! - 6:00 AM (next day) = 1440
//!
//! # Time Constants
//!
//! Standard time constants follow the pattern: `DAYn_[AM|PM]_HH_MM`
//! Examples:
//! - `DAY1_AM_6_00` = 0
//! - `DAY1_PM_6_00` = 720
//! - `DAY2_AM_10_00` = 1680 (1440 + 240)
//! - `NIGHT1_PM_10_00` = 960 (720 + 240)

use crate::expr::{EvalContext, EvalError, Expr};

/// Evaluate the `is_day` built-in function.
///
/// Checks if the current time is during daytime (6:00 AM to 6:00 PM).
///
/// # Syntax
/// - `is_day()` - returns true if it's daytime
///
/// # Examples
/// - `is_day()` - true when time is between 6:00 AM and 6:00 PM
pub fn eval_is_day(args: &[Expr], ctx: &impl EvalContext) -> Result<bool, EvalError> {
    if !args.is_empty() {
        return Err(EvalError::Error(format!(
            "is_day() expects 0 arguments, got {}",
            args.len()
        )));
    }
    Ok(ctx.is_day())
}

/// Evaluate the `is_night` built-in function.
///
/// Checks if the current time is during nighttime (6:00 PM to 6:00 AM).
///
/// # Syntax
/// - `is_night()` - returns true if it's nighttime
///
/// # Examples
/// - `is_night()` - true when time is between 6:00 PM and 6:00 AM
pub fn eval_is_night(args: &[Expr], ctx: &impl EvalContext) -> Result<bool, EvalError> {
    if !args.is_empty() {
        return Err(EvalError::Error(format!(
            "is_night() expects 0 arguments, got {}",
            args.len()
        )));
    }
    Ok(ctx.is_night())
}

/// Evaluate the `mm_time` built-in function.
///
/// Returns the current MM time as a numeric value for comparison purposes.
/// This is primarily useful in expressions that need to compare against specific times.
///
/// # Syntax
/// - `mm_time()` - returns the current time value
///
/// # Returns
/// The current time as minutes since Day 1 at 6:00 AM.
pub fn eval_mm_time(args: &[Expr], ctx: &impl EvalContext) -> Result<u32, EvalError> {
    if !args.is_empty() {
        return Err(EvalError::Error(format!(
            "mm_time() expects 0 arguments, got {}",
            args.len()
        )));
    }
    Ok(ctx.mm_time())
}

/// Evaluate the `at` built-in function.
///
/// Checks if the current time matches a specific time constant or value.
/// Uses a small tolerance window (±15 minutes) for matching.
///
/// # Syntax
/// - `at(TIME)` - checks if current time is approximately at the given time
///
/// # Examples
/// - `at(DAY1_AM_6_00)` - true when time is around 6:00 AM on Day 1
/// - `at(720)` - true when time is around 6:00 PM on Day 1
pub fn eval_at(args: &[Expr], ctx: &impl EvalContext) -> Result<bool, EvalError> {
    if args.len() != 1 {
        return Err(EvalError::Error(format!(
            "at() expects 1 argument, got {}",
            args.len()
        )));
    }

    let target_time = extract_time(&args[0])?;
    let current_time = ctx.mm_time();

    // Use a tolerance of 15 minutes for "at" checks
    const TOLERANCE: u32 = 15;
    let diff = current_time.abs_diff(target_time);

    Ok(diff <= TOLERANCE)
}

/// Evaluate the `between` built-in function.
///
/// Checks if the current time falls within a specified range (inclusive).
///
/// # Syntax
/// - `between(START, END)` - checks if current time is between START and END
///
/// # Examples
/// - `between(DAY1_AM_6_00, DAY1_PM_6_00)` - true during Day 1 daytime
/// - `between(0, 720)` - true during first half of Day 1
/// - `between(720, 1440)` - true during Night 1
///
/// # Notes
/// - Both START and END are inclusive
/// - If END < START, the function checks if time wraps around (not yet supported)
pub fn eval_between(args: &[Expr], ctx: &impl EvalContext) -> Result<bool, EvalError> {
    if args.len() != 2 {
        return Err(EvalError::Error(format!(
            "between() expects 2 arguments, got {}",
            args.len()
        )));
    }

    let start_time = extract_time(&args[0])?;
    let end_time = extract_time(&args[1])?;
    let current_time = ctx.mm_time();

    // Simple range check (inclusive)
    Ok(current_time >= start_time && current_time <= end_time)
}

/// Well-known MM time constants.
///
/// Returns the time value in minutes since Day 1 at 6:00 AM for a given constant name.
fn get_time_constant(name: &str) -> Option<u32> {
    // Normalize to uppercase
    let name_upper = name.to_uppercase();

    // Parse time constants in format: DAYn_[AM|PM]_HH_MM or NIGHTn_PM_HH_MM
    // Day offsets: Day 1 = 0, Day 2 = 1440, Day 3 = 2880

    match name_upper.as_str() {
        // Day 1 times
        "DAY1_AM_6_00" => Some(0),
        "DAY1_AM_7_00" => Some(60),
        "DAY1_AM_8_00" => Some(120),
        "DAY1_AM_9_00" => Some(180),
        "DAY1_AM_10_00" => Some(240),
        "DAY1_AM_11_00" => Some(300),
        "DAY1_PM_12_00" | "DAY1_NOON" => Some(360),
        "DAY1_PM_1_00" => Some(420),
        "DAY1_PM_2_00" => Some(480),
        "DAY1_PM_3_00" => Some(540),
        "DAY1_PM_4_00" => Some(600),
        "DAY1_PM_5_00" => Some(660),
        "DAY1_PM_6_00" | "NIGHT1_PM_6_00" => Some(720),
        "NIGHT1_PM_7_00" => Some(780),
        "NIGHT1_PM_8_00" => Some(840),
        "NIGHT1_PM_9_00" => Some(900),
        "NIGHT1_PM_10_00" => Some(960),
        "NIGHT1_PM_11_00" => Some(1020),
        "NIGHT1_AM_12_00" | "NIGHT1_MIDNIGHT" => Some(1080),
        "NIGHT1_AM_1_00" => Some(1140),
        "NIGHT1_AM_2_00" => Some(1200),
        "NIGHT1_AM_3_00" => Some(1260),
        "NIGHT1_AM_4_00" => Some(1320),
        "NIGHT1_AM_5_00" => Some(1380),

        // Day 2 times (offset by 1440)
        "DAY2_AM_6_00" => Some(1440),
        "DAY2_AM_7_00" => Some(1500),
        "DAY2_AM_8_00" => Some(1560),
        "DAY2_AM_9_00" => Some(1620),
        "DAY2_AM_10_00" => Some(1680),
        "DAY2_AM_11_00" => Some(1740),
        "DAY2_PM_12_00" | "DAY2_NOON" => Some(1800),
        "DAY2_PM_1_00" => Some(1860),
        "DAY2_PM_2_00" => Some(1920),
        "DAY2_PM_3_00" => Some(1980),
        "DAY2_PM_4_00" => Some(2040),
        "DAY2_PM_5_00" => Some(2100),
        "DAY2_PM_6_00" | "NIGHT2_PM_6_00" => Some(2160),
        "NIGHT2_PM_7_00" => Some(2220),
        "NIGHT2_PM_8_00" => Some(2280),
        "NIGHT2_PM_9_00" => Some(2340),
        "NIGHT2_PM_10_00" => Some(2400),
        "NIGHT2_PM_11_00" => Some(2460),
        "NIGHT2_AM_12_00" | "NIGHT2_MIDNIGHT" => Some(2520),
        "NIGHT2_AM_1_00" => Some(2580),
        "NIGHT2_AM_2_00" => Some(2640),
        "NIGHT2_AM_3_00" => Some(2700),
        "NIGHT2_AM_4_00" => Some(2760),
        "NIGHT2_AM_5_00" => Some(2820),

        // Day 3 times (offset by 2880)
        "DAY3_AM_6_00" | "FINAL_DAY_AM_6_00" => Some(2880),
        "DAY3_AM_7_00" => Some(2940),
        "DAY3_AM_8_00" => Some(3000),
        "DAY3_AM_9_00" => Some(3060),
        "DAY3_AM_10_00" => Some(3120),
        "DAY3_AM_11_00" => Some(3180),
        "DAY3_PM_12_00" | "DAY3_NOON" => Some(3240),
        "DAY3_PM_1_00" => Some(3300),
        "DAY3_PM_2_00" => Some(3360),
        "DAY3_PM_3_00" => Some(3420),
        "DAY3_PM_4_00" => Some(3480),
        "DAY3_PM_5_00" => Some(3540),
        "DAY3_PM_6_00" | "NIGHT3_PM_6_00" | "FINAL_NIGHT_PM_6_00" => Some(3600),
        "NIGHT3_PM_7_00" => Some(3660),
        "NIGHT3_PM_8_00" => Some(3720),
        "NIGHT3_PM_9_00" => Some(3780),
        "NIGHT3_PM_10_00" => Some(3840),
        "NIGHT3_PM_11_00" => Some(3900),
        "NIGHT3_AM_12_00" | "NIGHT3_MIDNIGHT" => Some(3960),
        "NIGHT3_AM_1_00" => Some(4020),
        "NIGHT3_AM_2_00" => Some(4080),
        "NIGHT3_AM_3_00" => Some(4140),
        "NIGHT3_AM_4_00" => Some(4200),
        "NIGHT3_AM_5_00" => Some(4260),

        // Special convenience aliases
        "DAWN" => Some(0),        // 6:00 AM (relative, resets each day)
        "DUSK" => Some(720),      // 6:00 PM (relative)
        "MIDNIGHT" => Some(1080), // 12:00 AM (relative)
        "NOON" => Some(360),      // 12:00 PM (relative)

        _ => None,
    }
}

/// Extract a time value from an expression.
///
/// Accepts either:
/// - A numeric literal (direct time value in minutes)
/// - An identifier representing a time constant (e.g., DAY1_AM_6_00)
fn extract_time(expr: &Expr) -> Result<u32, EvalError> {
    match expr {
        Expr::Number(n) => {
            if *n < 0 {
                Err(EvalError::Error(format!(
                    "time value must be non-negative, got {}",
                    n
                )))
            } else {
                Ok(*n as u32)
            }
        }
        Expr::Ident(name) => get_time_constant(name)
            .ok_or_else(|| EvalError::Error(format!("unknown time constant: {}", name))),
        _ => Err(EvalError::Error(format!(
            "expected time value (number or constant), got {:?}",
            expr
        ))),
    }
}

/// Get the current day number (1, 2, or 3) from a time value.
pub fn get_day(time: u32) -> u8 {
    match time {
        0..=1439 => 1,
        1440..=2879 => 2,
        _ => 3,
    }
}

/// Get the time within the current day (0-1439) from an absolute time value.
pub fn get_time_in_day(time: u32) -> u32 {
    time % 1440
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Mock context for testing time functions.
    struct MockTimeContext {
        time: u32,
    }

    impl MockTimeContext {
        fn new(time: u32) -> Self {
            Self { time }
        }
    }

    impl EvalContext for MockTimeContext {
        fn has_item(&self, _item: &str, _count: u32) -> bool {
            false
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
            true
        }

        fn is_child(&self) -> bool {
            false
        }

        fn mm_time(&self) -> u32 {
            self.time
        }
    }

    // --- is_day() tests ---

    #[test]
    fn test_is_day_at_dawn() {
        let ctx = MockTimeContext::new(0); // 6:00 AM Day 1
        assert!(eval_is_day(&[], &ctx).unwrap());
    }

    #[test]
    fn test_is_day_at_noon() {
        let ctx = MockTimeContext::new(360); // 12:00 PM Day 1
        assert!(eval_is_day(&[], &ctx).unwrap());
    }

    #[test]
    fn test_is_day_at_dusk() {
        let ctx = MockTimeContext::new(720); // 6:00 PM Day 1 - night starts
        assert!(!eval_is_day(&[], &ctx).unwrap());
    }

    #[test]
    fn test_is_day_at_midnight() {
        let ctx = MockTimeContext::new(1080); // 12:00 AM
        assert!(!eval_is_day(&[], &ctx).unwrap());
    }

    #[test]
    fn test_is_day_wrong_args() {
        let ctx = MockTimeContext::new(0);
        let result = eval_is_day(&[Expr::Number(42)], &ctx);
        assert!(result.is_err());
    }

    // --- is_night() tests ---

    #[test]
    fn test_is_night_at_dawn() {
        let ctx = MockTimeContext::new(0);
        assert!(!eval_is_night(&[], &ctx).unwrap());
    }

    #[test]
    fn test_is_night_at_dusk() {
        let ctx = MockTimeContext::new(720);
        assert!(eval_is_night(&[], &ctx).unwrap());
    }

    #[test]
    fn test_is_night_at_midnight() {
        let ctx = MockTimeContext::new(1080);
        assert!(eval_is_night(&[], &ctx).unwrap());
    }

    #[test]
    fn test_is_night_day2() {
        // Day 2 at 3:00 PM (1440 + 540 = 1980)
        let ctx = MockTimeContext::new(1980);
        assert!(!eval_is_night(&[], &ctx).unwrap());

        // Night 2 at 10:00 PM (1440 + 720 + 240 = 2400)
        let ctx = MockTimeContext::new(2400);
        assert!(eval_is_night(&[], &ctx).unwrap());
    }

    // --- mm_time() tests ---

    #[test]
    fn test_mm_time_returns_current_time() {
        let ctx = MockTimeContext::new(1234);
        assert_eq!(eval_mm_time(&[], &ctx).unwrap(), 1234);
    }

    #[test]
    fn test_mm_time_wrong_args() {
        let ctx = MockTimeContext::new(0);
        let result = eval_mm_time(&[Expr::Number(42)], &ctx);
        assert!(result.is_err());
    }

    // --- at() tests ---

    #[test]
    fn test_at_exact_match() {
        let ctx = MockTimeContext::new(720);
        let args = vec![Expr::Number(720)];
        assert!(eval_at(&args, &ctx).unwrap());
    }

    #[test]
    fn test_at_within_tolerance() {
        let ctx = MockTimeContext::new(730); // 10 minutes past
        let args = vec![Expr::Number(720)];
        assert!(eval_at(&args, &ctx).unwrap());
    }

    #[test]
    fn test_at_outside_tolerance() {
        let ctx = MockTimeContext::new(800); // 80 minutes past
        let args = vec![Expr::Number(720)];
        assert!(!eval_at(&args, &ctx).unwrap());
    }

    #[test]
    fn test_at_with_constant() {
        let ctx = MockTimeContext::new(720);
        let args = vec![Expr::Ident("DAY1_PM_6_00".into())];
        assert!(eval_at(&args, &ctx).unwrap());
    }

    #[test]
    fn test_at_wrong_args() {
        let ctx = MockTimeContext::new(0);

        // No arguments
        assert!(eval_at(&[], &ctx).is_err());

        // Too many arguments
        let args = vec![Expr::Number(0), Expr::Number(720)];
        assert!(eval_at(&args, &ctx).is_err());
    }

    // --- between() tests ---

    #[test]
    fn test_between_in_range() {
        let ctx = MockTimeContext::new(360); // Noon
        let args = vec![Expr::Number(0), Expr::Number(720)];
        assert!(eval_between(&args, &ctx).unwrap());
    }

    #[test]
    fn test_between_at_start() {
        let ctx = MockTimeContext::new(0);
        let args = vec![Expr::Number(0), Expr::Number(720)];
        assert!(eval_between(&args, &ctx).unwrap());
    }

    #[test]
    fn test_between_at_end() {
        let ctx = MockTimeContext::new(720);
        let args = vec![Expr::Number(0), Expr::Number(720)];
        assert!(eval_between(&args, &ctx).unwrap());
    }

    #[test]
    fn test_between_outside_range() {
        let ctx = MockTimeContext::new(800);
        let args = vec![Expr::Number(0), Expr::Number(720)];
        assert!(!eval_between(&args, &ctx).unwrap());
    }

    #[test]
    fn test_between_with_constants() {
        let ctx = MockTimeContext::new(360);
        let args = vec![
            Expr::Ident("DAY1_AM_6_00".into()),
            Expr::Ident("DAY1_PM_6_00".into()),
        ];
        assert!(eval_between(&args, &ctx).unwrap());
    }

    #[test]
    fn test_between_wrong_args() {
        let ctx = MockTimeContext::new(0);

        // No arguments
        assert!(eval_between(&[], &ctx).is_err());

        // One argument
        let args = vec![Expr::Number(0)];
        assert!(eval_between(&args, &ctx).is_err());

        // Three arguments
        let args = vec![Expr::Number(0), Expr::Number(720), Expr::Number(1440)];
        assert!(eval_between(&args, &ctx).is_err());
    }

    // --- Time constant tests ---

    #[test]
    fn test_time_constants_day1() {
        assert_eq!(get_time_constant("DAY1_AM_6_00"), Some(0));
        assert_eq!(get_time_constant("DAY1_PM_12_00"), Some(360));
        assert_eq!(get_time_constant("DAY1_NOON"), Some(360));
        assert_eq!(get_time_constant("DAY1_PM_6_00"), Some(720));
    }

    #[test]
    fn test_time_constants_night1() {
        assert_eq!(get_time_constant("NIGHT1_PM_6_00"), Some(720));
        assert_eq!(get_time_constant("NIGHT1_MIDNIGHT"), Some(1080));
        assert_eq!(get_time_constant("NIGHT1_AM_5_00"), Some(1380));
    }

    #[test]
    fn test_time_constants_day2() {
        assert_eq!(get_time_constant("DAY2_AM_6_00"), Some(1440));
        assert_eq!(get_time_constant("DAY2_AM_10_00"), Some(1680));
    }

    #[test]
    fn test_time_constants_day3() {
        assert_eq!(get_time_constant("DAY3_AM_6_00"), Some(2880));
        assert_eq!(get_time_constant("FINAL_DAY_AM_6_00"), Some(2880));
        assert_eq!(get_time_constant("FINAL_NIGHT_PM_6_00"), Some(3600));
    }

    #[test]
    fn test_time_constants_case_insensitive() {
        assert_eq!(get_time_constant("day1_am_6_00"), Some(0));
        assert_eq!(get_time_constant("Day1_AM_6_00"), Some(0));
    }

    #[test]
    fn test_time_constants_aliases() {
        assert_eq!(get_time_constant("DAWN"), Some(0));
        assert_eq!(get_time_constant("NOON"), Some(360));
        assert_eq!(get_time_constant("DUSK"), Some(720));
        assert_eq!(get_time_constant("MIDNIGHT"), Some(1080));
    }

    #[test]
    fn test_unknown_time_constant() {
        assert_eq!(get_time_constant("UNKNOWN"), None);
        assert_eq!(get_time_constant("DAY4_AM_6_00"), None);
    }

    // --- Helper function tests ---

    #[test]
    fn test_get_day() {
        assert_eq!(get_day(0), 1);
        assert_eq!(get_day(720), 1);
        assert_eq!(get_day(1439), 1);
        assert_eq!(get_day(1440), 2);
        assert_eq!(get_day(2879), 2);
        assert_eq!(get_day(2880), 3);
        assert_eq!(get_day(4319), 3);
    }

    #[test]
    fn test_get_time_in_day() {
        assert_eq!(get_time_in_day(0), 0);
        assert_eq!(get_time_in_day(720), 720);
        assert_eq!(get_time_in_day(1440), 0);
        assert_eq!(get_time_in_day(2160), 720);
        assert_eq!(get_time_in_day(2880), 0);
    }

    // --- extract_time() tests ---

    #[test]
    fn test_extract_time_number() {
        let result = extract_time(&Expr::Number(720));
        assert_eq!(result.unwrap(), 720);
    }

    #[test]
    fn test_extract_time_negative_number() {
        let result = extract_time(&Expr::Number(-100));
        assert!(result.is_err());
    }

    #[test]
    fn test_extract_time_constant() {
        let result = extract_time(&Expr::Ident("DAY1_PM_6_00".into()));
        assert_eq!(result.unwrap(), 720);
    }

    #[test]
    fn test_extract_time_unknown_constant() {
        let result = extract_time(&Expr::Ident("INVALID_TIME".into()));
        assert!(result.is_err());
    }

    #[test]
    fn test_extract_time_invalid_expr() {
        let result = extract_time(&Expr::String("test".into()));
        assert!(result.is_err());
    }
}
