//! Built-in functions for expression evaluation.
//!
//! This module provides implementations for built-in functions used in OoTMM logic expressions:
//!
//! - **items**: `has()`, `can_use()` - inventory and item usability checks
//! - **logic**: `event()`, `setting()`, `trick()` - game state and setting checks
//! - **time**: `is_day()`, `is_night()`, `between()`, `at()`, `mm_time()` - MM time-based conditions

pub mod items;
pub mod logic;
pub mod time;

// Re-export commonly used functions
pub use items::{eval_can_use, eval_has};
pub use logic::{eval_event, eval_setting};
pub use time::{
    eval_at, eval_between, eval_is_day, eval_is_night, eval_mm_time, get_day, get_time_in_day,
};
