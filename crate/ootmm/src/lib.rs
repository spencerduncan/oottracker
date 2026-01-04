//! OoTMM (Ocarina of Time + Majora's Mask) Randomizer Support
//!
//! This crate provides data structures and logic for the OoTMM combined randomizer.

#![deny(
    rust_2018_idioms,
    unused,
    unused_crate_dependencies,
    unused_import_braces,
    unused_lifetimes,
    unused_qualifications,
    warnings
)]
#![forbid(unsafe_code)]

pub mod checks;
pub mod embedded_data;
pub mod error;
pub mod events;
pub mod expr;
pub mod item;
pub mod items;
pub mod rando;
pub mod region;
pub mod settings;
pub mod world_database;

// Re-export item types for convenience
pub use item::{Game, Item, ItemCategory, MmItem, OotItem};
pub use items::{ItemMapping, ItemName, MappingError};
pub use world_database::WorldDatabase;

// Re-export check tracking types
pub use checks::{CheckError, CheckStatus, CheckTracker, GameContext};

// Re-export embedded data convenience functions
pub use embedded_data::{create_world_database, create_world_database_from};

// Re-export rando types
pub use rando::{OotmmRando, OotmmRandoError, OotmmRegionName};

// Re-export settings types
pub use settings::{
    AgeChangeMode, BeneathWellState, BossWarpPadsMode, ClearStateDungeonsMm,
    ClimbMostSurfacesState, DekuTreeState, DoorOfTimeState, ErGrottosState, ErOverworldState,
    GanonBossKeyMode, HookshotAnywhereState, JpLayout, KakarikoGateState, LacsMode, LogicMode,
    MajoraChildMode, MmDungeon, MoonCrashMode, OotDungeon, RandomizerSettings, ShufflePotsMm,
    SmallKeyShuffleOot,
};

// Re-export OoT event types
pub use events::{
    offsets as event_offsets, EventReadError, OotEvent, OotEventCategory, OotEventFlag,
    OotEventReader, OotEventWriter,
};

// Re-export MM event types
pub use events::{
    mm_event_offsets, MmEvent, MmEventCategory, MmEventFlag, MmEventReadError, MmEventReader,
    MmEventWriter,
};

// Re-export MM flag mapping types
pub use events::{
    mm_all_mappings, mm_chest_mappings, mm_collectible_mappings, mm_flag_offsets,
    mm_special_mappings, MmFlagMapping, MmFlagType, MmSceneFlag,
};
