//! Owl Statue location mappings.
//!
//! Owl statues are global flags stored in the quest status bitfield.
//! Each owl statue has a unique bit position (bits 20-29).

use std::collections::HashMap;

use ootmm::events::mm_flags::owl_bits;

use crate::mm_flag_mapping::{MmFlagMapping, MmFlagType};

/// Registers owl statue mappings into the provided map.
pub fn register_owl_statues(map: &mut HashMap<&'static str, MmFlagMapping>) {
    add_global(
        map,
        "mm_clock_town_owl_statue",
        MmFlagType::OwlStatue,
        1 << owl_bits::OWL_CLOCK_TOWN,
    );
    add_global(
        map,
        "mm_milk_road_owl_statue",
        MmFlagType::OwlStatue,
        1 << owl_bits::OWL_MILK_ROAD,
    );
    add_global(
        map,
        "mm_southern_swamp_owl_statue",
        MmFlagType::OwlStatue,
        1 << owl_bits::OWL_SOUTHERN_SWAMP,
    );
    add_global(
        map,
        "mm_woodfall_owl_statue",
        MmFlagType::OwlStatue,
        1 << owl_bits::OWL_WOODFALL,
    );
    add_global(
        map,
        "mm_mountain_village_owl_statue",
        MmFlagType::OwlStatue,
        1 << owl_bits::OWL_MOUNTAIN_VILLAGE,
    );
    add_global(
        map,
        "mm_snowhead_owl_statue",
        MmFlagType::OwlStatue,
        1 << owl_bits::OWL_SNOWHEAD,
    );
    add_global(
        map,
        "mm_zora_cape_owl_statue",
        MmFlagType::OwlStatue,
        1 << owl_bits::OWL_ZORA_CAPE,
    );
    add_global(
        map,
        "mm_great_bay_coast_owl_statue",
        MmFlagType::OwlStatue,
        1 << owl_bits::OWL_GREAT_BAY,
    );
    add_global(
        map,
        "mm_ikana_canyon_owl_statue",
        MmFlagType::OwlStatue,
        1 << owl_bits::OWL_IKANA_CANYON,
    );
    add_global(
        map,
        "mm_stone_tower_owl_statue",
        MmFlagType::OwlStatue,
        1 << owl_bits::OWL_STONE_TOWER,
    );
}

fn add_global(
    map: &mut HashMap<&'static str, MmFlagMapping>,
    location_id: &'static str,
    flag_type: MmFlagType,
    flag_bit: u32,
) {
    map.insert(
        location_id,
        MmFlagMapping::global(location_id, flag_type, flag_bit),
    );
}
