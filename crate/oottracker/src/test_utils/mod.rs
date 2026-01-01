//! Test utilities for oottracker.
//!
//! This module provides testing infrastructure including mock traits
//! and implementations for external dependencies.

pub mod mocks;

pub use mocks::{MockError, MockRamReader, MockSaveReader, RamReader, SaveReader};
