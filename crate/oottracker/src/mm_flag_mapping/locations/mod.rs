//! Location mappings organized by category.
//!
//! This module aggregates all location mapping registrations from submodules.

mod dungeons;
mod gold_skulltulas;
mod heart_pieces;
mod items;
mod masks;
mod overworld;
mod owl_statues;
mod songs;

use std::collections::HashMap;

use crate::mm_flag_mapping::MmFlagMapping;

/// Registers all location mappings into the provided map.
///
/// This function calls each submodule's registration function to populate
/// the mapping table with all known MM locations.
pub fn register_all_locations(map: &mut HashMap<&'static str, MmFlagMapping>) {
    owl_statues::register_owl_statues(map);
    songs::register_songs(map);
    masks::register_masks(map);
    items::register_items(map);
    dungeons::register_dungeons(map);
    overworld::register_overworld(map);
    heart_pieces::register_heart_pieces(map);
    gold_skulltulas::register_gold_skulltulas(map);
}
