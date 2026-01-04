//! E2E testing utilities for oottracker.
//!
//! This crate provides tools for running automated end-to-end tests,
//! including a Wine+PJ64-EM process launcher for testing against
//! real emulator instances.
//!
//! ## Components
//!
//! - [`launcher`]: Wine+PJ64-EM process management
//! - [`harness`]: TCP client for controlling the emulator via Lua test harness

pub mod harness;
pub mod launcher;

pub use harness::{ControllerInput, HarnessClient, HarnessError, E2E_HARNESS_PORT};
pub use launcher::{LauncherError, Pj64EmLauncher};
