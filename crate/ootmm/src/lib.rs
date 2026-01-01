//! OoTMM (Ocarina of Time + Majora's Mask) Randomizer Support
//!
//! This crate provides data structures and logic for the OoTMM combined randomizer.

#![deny(rust_2018_idioms, unused, unused_crate_dependencies, unused_import_braces, unused_lifetimes, unused_qualifications, warnings)]
#![forbid(unsafe_code)]

pub mod error;
pub mod expr;
pub mod item;
pub mod region;
