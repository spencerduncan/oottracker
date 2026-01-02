//! Item name mappings between OoTMM's item system and oottracker's internal representation.
//!
//! This module provides bidirectional mappings between OoTMM's string-based item names
//! and the internal `OotItem`/`MmItem` enums, with full serde support.

mod mapping;

pub use mapping::{ItemMapping, ItemName, MappingError};
