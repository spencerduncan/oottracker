//! Common test utilities for ootmm integration tests.
//!
//! This module provides test fixtures, helper functions, and mock implementations
//! for integration testing of the ootmm crate.

mod context;
mod fixtures;

pub use context::MockEvalContext;
pub use fixtures::load_expression_fixture;

// Re-export for future use in additional integration tests
#[allow(unused_imports)]
pub use fixtures::{fixture_path, load_fixture, ExpressionTestCase};
