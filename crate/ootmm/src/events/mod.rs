//! OoT and MM event handling for randomizer logic.
//!
//! This module provides event definitions and memory flag mappings for tracking
//! game events used in randomizer logic expressions.
//!
//! Events are categorized as:
//! - **Persistent events**: Stored in save data (EventChkInf, InfTable, scene flags)
//! - **Volatile events**: Computed at runtime (dungeon switches, water levels, etc.)

pub mod oot;

pub use oot::{
    offsets, EventReadError, OotEvent, OotEventCategory, OotEventFlag, OotEventReader,
    OotEventWriter,
};
