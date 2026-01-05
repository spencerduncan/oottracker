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
//! - [`ram_validation`] - RAM validation for comparing emulator state against fixtures
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
//!
//! # RAM Validation Example
//!
//! ```ignore
//! use oottracker_e2e::{
//!     HarnessBuilder,
//!     fixtures::deku_tree_complete,
//!     ram_validation::{RamValidator, RamValidationExt},
//! };
//!
//! // Create validator from fixture
//! let fixture = deku_tree_complete();
//! let validator = RamValidator::from_fixture(&fixture);
//!
//! // Validate against emulator
//! let report = harness.validate_ram(&validator).await?;
//! println!("{}", report.summary());
//! ```

pub mod config;
pub mod fixtures;
pub mod harness;
pub mod launcher;
pub mod oot_flag_validation;
pub mod ram_validation;
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
pub use oot_flag_validation::{
    deku_tree_chests_validator, dodongos_cavern_chests_validator, global_flag_address,
    global_flag_offset, scene_flag_address, scene_flag_offset, skulltula_validator, GlobalFlagType,
    OotFlagValidator, SceneFlagType, EVENT_CHK_INF_OFFSET, EVENT_CHK_INF_SIZE, INF_TABLE_OFFSET,
    INF_TABLE_SIZE, ITEM_GET_INF_OFFSET, ITEM_GET_INF_SIZE, SCENE_FLAGS_OFFSET,
    SCENE_FLAG_ENTRY_SIZE, SKULLTULA_FLAGS_OFFSET, SKULLTULA_FLAGS_SIZE,
};
pub use ram_validation::{
    read_ram, read_ram_batch, BatchSummary, BatchValidator, CompareMode, ExpectedValue,
    FieldResult, RamReadRequest, RamReadResponse, RamValidationExt, RamValidator, ValidationReport,
    CMD_READ_RAM, DEFAULT_READ_TIMEOUT, E2E_TEST_PORT, MM_SAVE_ADDR, MM_SAVE_SIZE, OOT_SAVE_ADDR,
    OOT_SAVE_SIZE, RESP_ERROR, RESP_OK,
};
pub use scenarios::{
    all_scenarios, regression_scenarios, smoke_test_scenarios, ScenarioStep, TestScenario,
    TrackerEvent,
};
