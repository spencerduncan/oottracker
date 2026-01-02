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

pub mod embedded_data;
pub mod error;
pub mod expr;
pub mod item;
pub mod items;
pub mod rando;
pub mod region;
pub mod world_database;

// Re-export item types for convenience
pub use item::{Game, Item, ItemCategory, MmItem, OotItem};
pub use items::{ItemMapping, ItemName, MappingError};
pub use world_database::WorldDatabase;

// Re-export embedded data convenience functions
pub use embedded_data::{create_world_database, create_world_database_from};

// Re-export rando types
pub use rando::{OotmmRando, OotmmRandoError, OotmmRegionName};
