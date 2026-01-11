//! Dungeon location mappings.
//!
//! This module contains chest mappings for main temples and mini dungeons.

mod main_temples;
mod mini_dungeons;

use std::collections::HashMap;

use crate::mm_flag_mapping::MmFlagMapping;

/// Registers all dungeon mappings into the provided map.
pub fn register_dungeons(map: &mut HashMap<&'static str, MmFlagMapping>) {
    main_temples::register_main_temples(map);
    mini_dungeons::register_mini_dungeons(map);
}
