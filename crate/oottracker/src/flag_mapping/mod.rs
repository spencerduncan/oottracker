//! OoT flag mapping for location check tracking.
//!
//! This module provides mappings from location IDs to their corresponding
//! flag addresses in the OoT save data, enabling automatic tracking of
//! which locations have been checked.
//!
//! # Module Structure
//!
//! - [`types`]: Core type definitions (FlagType, FlagMapping, CheckStatus, etc.)
//! - [`scenes`]: Scene ID constants
//! - [`mappings`]: Static mapping tables (internal)
//! - [`queries`]: Lookup functions for mappings
//! - [`mq`]: Master Quest integration
//! - [`eval`]: OoT logic evaluation context
//! - [`checking`]: Location status checking functions

pub mod checking;
pub mod eval;
mod mappings;
pub mod mq;
pub mod queries;
pub mod scenes;
pub mod types;

#[cfg(test)]
mod tests;

// Re-export commonly used types for convenience
pub use types::{
    Accessibility, CheckStatus, CheckedLocationsSummary, FlagMapping, FlagType, LocationCheckResult,
};

// Re-export scene constants
pub use scenes as scene;

// Re-export query functions
pub use queries::{
    get_all_oot_location_ids, get_all_oot_mappings, get_location_logic, get_mapped_locations,
    get_mapping, get_mappings_by_flag_type, get_mappings_for_scene, get_stub_locations,
    oot_location_count, oot_mapped_count, oot_stub_count,
};

// Re-export MQ functions
pub use mq::{
    active_location_count, active_mapped_count, dungeon_to_mq_dungeon, get_active_mapped_locations,
    get_active_mapping, get_active_mappings, get_dungeon_mappings, mq_settings_from_knowledge,
    MqDungeonType as MqDungeon,
};

// Re-export checking functions
pub use checking::{
    check_location_status, get_all_checked_locations, get_all_checked_locations_combo,
    get_all_checked_locations_filtered, get_checked_locations_map, get_checked_locations_summary,
    get_checked_locations_summary_filtered,
};

// Re-export eval context
pub use eval::OotEvalContext;
