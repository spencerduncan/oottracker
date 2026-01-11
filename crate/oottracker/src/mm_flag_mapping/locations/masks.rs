//! NPC Mask reward location mappings.
//!
//! Masks obtained from NPCs are tracked by EventInf or WeekEventReg flags.
//! These flags are set when the player receives the mask.

use std::collections::HashMap;

use crate::mm_flag_mapping::{MmFlagMapping, MmFlagType};

/// Registers mask reward mappings into the provided map.
pub fn register_masks(map: &mut HashMap<&'static str, MmFlagMapping>) {
    // Blast Mask - from old lady saved from Sakon (Night 1)
    add_global(
        map,
        "mm_clock_town_blast_mask",
        MmFlagType::WeekEventReg,
        0x01,
    );

    // Bremen Mask - from Guru-Guru in Laundry Pool
    add_global(
        map,
        "mm_clock_town_guru_guru_mask_bremen",
        MmFlagType::WeekEventReg,
        0x02,
    );

    // Kafei's Mask - from Madame Aroma in Mayor's Office
    add_global(
        map,
        "mm_mayors_office_kafeis_mask",
        MmFlagType::EventInf,
        0x01,
    );

    // Postman's Hat - from Postman after delivering mail
    add_global(
        map,
        "mm_clock_town_postman_hat",
        MmFlagType::WeekEventReg,
        0x04,
    );

    // All-Night Mask - from Curiosity Shop (expensive purchase)
    add_global(
        map,
        "mm_curiosity_shop_all_night_mask",
        MmFlagType::WeekEventReg,
        0x08,
    );

    // Troupe Leader's Mask - from Gorman Brothers at Milk Bar
    add_global(
        map,
        "mm_milk_bar_troupe_leader_mask",
        MmFlagType::WeekEventReg,
        0x10,
    );

    // Bunny Hood - from Grog at Cucco Shack
    add_global(
        map,
        "mm_cucco_shack_bunny_mask",
        MmFlagType::WeekEventReg,
        0x20,
    );

    // Don Gero's Mask - from hungry Goron in Mountain Village
    add_global(
        map,
        "mm_mountain_village_don_gero_mask",
        MmFlagType::WeekEventReg,
        0x40,
    );

    // Mask of Scents - from Deku Butler in Deku Shrine
    add_global(
        map,
        "mm_deku_shrine_mask_of_scents",
        MmFlagType::EventInf,
        0x02,
    );

    // Romani's Mask - from Cremia after escort mission
    add_global(
        map,
        "mm_romani_ranch_cremia_escort",
        MmFlagType::WeekEventReg,
        0x80,
    );

    // Garo's Mask - from Gorman Brothers at Gorman Track
    add_global(map, "mm_gorman_track_garo_mask", MmFlagType::EventInf, 0x04);

    // Captain's Hat - from Skull Keeta in Ikana Graveyard
    add_global(
        map,
        "mm_ikana_graveyard_captain_mask",
        MmFlagType::EventInf,
        0x08,
    );

    // Gibdo Mask - from Pamela's Father in Music Box House
    add_global(
        map,
        "mm_music_box_house_gibdo_mask",
        MmFlagType::EventInf,
        0x10,
    );

    // Stone Mask - from invisible soldier on Road to Ikana
    add_global(
        map,
        "mm_road_to_ikana_stone_mask",
        MmFlagType::EventInf,
        0x20,
    );

    // Kamaro's Mask - from Kamaro ghost in Termina Field
    add_global(
        map,
        "mm_termina_field_kamaro_mask",
        MmFlagType::EventInf,
        0x40,
    );

    // Goron Mask - from Darmani's ghost in Goron Graveyard
    add_global(map, "mm_goron_graveyard_mask", MmFlagType::EventInf, 0x80);

    // Zora Mask - from Mikau on Great Bay Coast
    add_global(
        map,
        "mm_great_bay_coast_zora_mask",
        MmFlagType::EventInf,
        0x0100,
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
