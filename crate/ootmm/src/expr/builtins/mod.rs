//! Built-in functions for expression evaluation.
//!
//! This module provides implementations for built-in functions used in OoTMM logic expressions:
//!
//! - **items**: `has()`, `can_use()` - inventory and item usability checks
//! - **logic**: `event()`, `setting()`, `trick()` - game state and setting checks
//! - **time**: `between()`, `at()` - MM time-based conditions (TODO)

pub mod items;
pub mod logic;
pub mod time;

// Re-export commonly used functions
pub use items::{eval_can_use, eval_has};
pub use logic::{eval_event, eval_setting};
