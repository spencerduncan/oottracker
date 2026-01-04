//! OoT and MM event handling for randomizer logic.
//!
//! This module provides event definitions and memory flag mappings for tracking
//! game events used in randomizer logic expressions.
//!
//! Events are categorized as:
//! - **Persistent events**: Stored in save data (EventChkInf, InfTable, scene flags)
//! - **Volatile events**: Computed at runtime (dungeon switches, water levels, etc.)

pub mod mm;
pub mod mm_flags;
pub mod oot;

pub use mm::{
    offsets as mm_event_offsets, MmEvent, MmEventCategory, MmEventFlag, MmEventReadError,
    MmEventReader, MmEventWriter,
};
pub use mm_flags::{
    mm_all_mappings, mm_chest_mappings, mm_collectible_mappings, mm_special_mappings,
    offsets as mm_flag_offsets, MmFlagMapping, MmFlagType, MmSceneFlag,
};
pub use oot::{
    offsets, EventReadError, OotEvent, OotEventCategory, OotEventFlag, OotEventReader,
    OotEventWriter,
};
