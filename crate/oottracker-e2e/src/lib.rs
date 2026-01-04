//! E2E testing utilities for oottracker.
//!
//! This crate provides tools for running automated end-to-end tests,
//! including a Wine+PJ64-EM process launcher for testing against
//! real emulator instances.

pub mod launcher;

pub use launcher::{LauncherError, Pj64EmLauncher, Result};
