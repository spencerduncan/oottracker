//! E2E testing utilities for oottracker.
//!
//! This crate provides tools for running automated end-to-end tests,
//! including a Wine+PJ64-EM process launcher for testing against
//! real emulator instances.
//!
//! # Modules
//!
//! - [`launcher`] - Wine+PJ64-EM process launcher
//! - [`harness`] - Test harness for coordinating E2E tests
//! - [`fixtures`] - Save state fixtures for various game states
//! - [`scenarios`] - Test scenarios for event detection
//! - [`config`] - ROM configuration helpers
//!
//! # Example
//!
//! ```ignore
//! use oottracker_e2e::{
//!     HarnessBuilder,
//!     fixtures::{deku_tree_complete, GameStateFixture},
//!     scenarios::{deku_tree_completion, TestScenario},
//! };
//!
//! // Create a test harness
//! let harness = HarnessBuilder::new()
//!     .wine_prefix("/home/user/.wine")
//!     .pj64_exe("/home/user/pj64/Project64.exe")
//!     .rom("/home/user/roms/oot.z64")
//!     .build();
//!
//! // Load a fixture
//! let fixture = deku_tree_complete();
//! let save_context = fixture.to_save_context();
//!
//! // Run a test scenario
//! let scenario = deku_tree_completion();
//! for step in &scenario.steps {
//!     // Execute step and verify expected events
//! }
//! ```

pub mod config;
pub mod fixtures;
pub mod harness;
pub mod launcher;
pub mod scenarios;

// Re-export commonly used types
pub use config::{
    ConfigError, MmVersion, OotVersion, Pj64EmConfig, RomInfo, RomSettings, RomType,
    SaveStateManager, TestEnvironment,
};
pub use fixtures::{
    all_fixtures, deku_tree_complete, ganon_ready, new_game, BossDefeats, Equipment,
    GameStateFixture, ItemId, ItemSlot, QuestStatus,
};
pub use harness::{
    event_channel, EventReceiver, HarnessBuilder, HarnessConfig, HarnessError, PacketType, RamData,
    TestHarness, DEFAULT_TRACKER_PORT, PROTOCOL_VERSION,
};
pub use launcher::{LauncherError, Pj64EmLauncher, Result};
pub use scenarios::{
    all_scenarios, regression_scenarios, smoke_test_scenarios, ScenarioStep, TestScenario,
    TrackerEvent,
};
