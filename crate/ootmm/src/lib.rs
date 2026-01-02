//! OoTMM (Ocarina of Time + Majora's Mask) Randomizer Support
//!
//! This crate provides data structures and logic for the OoTMM combined randomizer.
//!
//! # Embedded World Data
//!
//! World data (regions, locations, exits, events) is embedded at compile time
//! from YAML files. Use the [`embedded_data`] module to access this data:
//!
//! ```
//! use ootmm::embedded_data;
//!
//! // Load all embedded world data
//! let db = embedded_data::load_all_world_data().unwrap();
//! println!("Loaded {} regions", db.region_count());
//! ```

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
pub mod region;
pub mod world_database;

// Re-export item types for convenience
pub use item::{Game, Item, ItemCategory, MmItem, OotItem};
pub use world_database::WorldDatabase;
