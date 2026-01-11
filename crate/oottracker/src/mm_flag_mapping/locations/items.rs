//! NPC item rewards, minigames, stray fairies, boss rewards, and other item locations.
//!
//! This module contains mappings for various item locations that don't fit
//! into dungeons, overworld chests, or heart pieces.

use std::collections::HashMap;

use crate::mm_flag_mapping::{scenes, MmFlagMapping, MmFlagType};

/// Registers all item-related mappings into the provided map.
pub fn register_items(map: &mut HashMap<&'static str, MmFlagMapping>) {
    register_npc_item_rewards(map);
    register_stray_fairies(map);
    register_boss_rewards(map);
    register_cows(map);
    register_shops(map);
    register_scrubs(map);
    register_minigames(map);
    register_grottos(map);
}

/// Registers NPC item reward mappings.
fn register_npc_item_rewards(map: &mut HashMap<&'static str, MmFlagMapping>) {
    // Bomber's Notebook - from Jim after Hide and Seek
    add_global(
        map,
        "mm_clock_town_bomber_notebook",
        MmFlagType::EventInf,
        0x0200,
    );

    // Pendant of Memories - from Kafei in his hideout
    add_global(
        map,
        "mm_kafei_hideout_pendant_of_memories",
        MmFlagType::WeekEventReg,
        0x0100,
    );

    // Bottle from Madame Aroma - at Milk Bar
    add_global(
        map,
        "mm_milk_bar_madame_aroma_bottle",
        MmFlagType::WeekEventReg,
        0x0200,
    );

    // Post Box reward - delivering mail
    add_global(
        map,
        "mm_clock_town_post_box",
        MmFlagType::WeekEventReg,
        0x0400,
    );

    // Moon's Tear - from Astral Observatory
    add_global(
        map,
        "mm_astral_observatory_moon_tear",
        MmFlagType::MoonsTear,
        0x01,
    );

    // Pictobox - from Tourist Information
    add_global(
        map,
        "mm_tourist_information_pictobox",
        MmFlagType::EventInf,
        0x0400,
    );

    // Tingle Picture reward
    add_global(
        map,
        "mm_tourist_information_tingle_picture",
        MmFlagType::EventInf,
        0x0800,
    );
}

/// Registers stray fairy and Great Fairy reward mappings.
fn register_stray_fairies(map: &mut HashMap<&'static str, MmFlagMapping>) {
    // Clock Town Great Fairy rewards (counter index 0)
    add_global(
        map,
        "mm_clock_town_great_fairy",
        MmFlagType::StrayFairy,
        0, // Clock Town counter
    );
    add_global(
        map,
        "mm_clock_town_great_fairy_alt",
        MmFlagType::StrayFairy,
        0, // Clock Town counter (alternate form reward)
    );

    // Woodfall Great Fairy reward (counter index 1)
    add_global(
        map,
        "mm_woodfall_great_fairy",
        MmFlagType::StrayFairy,
        1, // Woodfall counter
    );

    // Snowhead Great Fairy reward (counter index 2)
    add_global(
        map,
        "mm_snowhead_great_fairy",
        MmFlagType::StrayFairy,
        2, // Snowhead counter
    );

    // Great Bay Great Fairy reward (counter index 3)
    add_global(
        map,
        "mm_great_bay_great_fairy",
        MmFlagType::StrayFairy,
        3, // Great Bay counter
    );

    // Ikana (Stone Tower) Great Fairy reward (counter index 4)
    add_global(
        map,
        "mm_ikana_great_fairy",
        MmFlagType::StrayFairy,
        4, // Stone Tower counter
    );

    // Clock Town stray fairy (collectible in Laundry Pool)
    add_scene(
        map,
        "mm_clock_town_stray_fairy",
        scenes::LAUNDRY_POOL,
        MmFlagType::Collectible,
        0x01,
    );

    // Beneath the Well Fairy Fountain fairies (8 healing fairies)
    for i in 1..=8 {
        let location_id = match i {
            1 => "mm_beneath_the_well_fairy_fountain_fairy_1",
            2 => "mm_beneath_the_well_fairy_fountain_fairy_2",
            3 => "mm_beneath_the_well_fairy_fountain_fairy_3",
            4 => "mm_beneath_the_well_fairy_fountain_fairy_4",
            5 => "mm_beneath_the_well_fairy_fountain_fairy_5",
            6 => "mm_beneath_the_well_fairy_fountain_fairy_6",
            7 => "mm_beneath_the_well_fairy_fountain_fairy_7",
            8 => "mm_beneath_the_well_fairy_fountain_fairy_8",
            _ => unreachable!(),
        };
        add_scene(
            map,
            location_id,
            scenes::BENEATH_THE_WELL,
            MmFlagType::Collectible,
            1 << (i - 1),
        );
    }
}

/// Registers boss reward mappings.
fn register_boss_rewards(map: &mut HashMap<&'static str, MmFlagMapping>) {
    add_global(
        map,
        "mm_woodfall_temple_boss_remains",
        MmFlagType::Boss,
        0x01,
    );
    add_global(
        map,
        "mm_snowhead_temple_boss_remains",
        MmFlagType::Boss,
        0x02,
    );
    add_global(
        map,
        "mm_great_bay_temple_boss_remains",
        MmFlagType::Boss,
        0x04,
    );
    add_global(
        map,
        "mm_stone_tower_temple_boss_remains",
        MmFlagType::Boss,
        0x08,
    );
}

/// Registers cow location mappings.
fn register_cows(map: &mut HashMap<&'static str, MmFlagMapping>) {
    add_global(map, "mm_romani_ranch_cow_1", MmFlagType::Cow, 0x01);
    add_global(map, "mm_romani_ranch_cow_2", MmFlagType::Cow, 0x02);
    add_global(map, "mm_romani_ranch_cow_3", MmFlagType::Cow, 0x04);
    add_global(map, "mm_great_bay_coast_cow", MmFlagType::Cow, 0x08);
    add_global(map, "mm_beneath_the_well_cow", MmFlagType::Cow, 0x10);
}

/// Registers shop item mappings.
fn register_shops(map: &mut HashMap<&'static str, MmFlagMapping>) {
    // Bomb Shop (0x00-0x03)
    add_global(map, "mm_bomb_shop_item_1", MmFlagType::Shop, 0x00);
    add_global(map, "mm_bomb_shop_item_2", MmFlagType::Shop, 0x01);

    // Trading Post (0x05-0x0C)
    add_global(map, "mm_trading_post_item_1", MmFlagType::Shop, 0x05);
    add_global(map, "mm_trading_post_item_2", MmFlagType::Shop, 0x06);
    add_global(map, "mm_trading_post_item_3", MmFlagType::Shop, 0x07);
    add_global(map, "mm_trading_post_item_4", MmFlagType::Shop, 0x08);
    add_global(map, "mm_trading_post_item_5", MmFlagType::Shop, 0x09);
    add_global(map, "mm_trading_post_item_6", MmFlagType::Shop, 0x0A);
    add_global(map, "mm_trading_post_item_7", MmFlagType::Shop, 0x0B);
    add_global(map, "mm_trading_post_item_8", MmFlagType::Shop, 0x0C);

    // Goron Shop (0x10-0x12)
    add_global(map, "mm_goron_shop_item_1", MmFlagType::Shop, 0x10);
    add_global(map, "mm_goron_shop_item_2", MmFlagType::Shop, 0x11);
    add_global(map, "mm_goron_shop_item_3", MmFlagType::Shop, 0x12);

    // Zora Shop (0x13-0x15)
    add_global(map, "mm_zora_shop_item_1", MmFlagType::Shop, 0x13);
    add_global(map, "mm_zora_shop_item_2", MmFlagType::Shop, 0x14);
    add_global(map, "mm_zora_shop_item_3", MmFlagType::Shop, 0x15);

    // Milk Bar Purchases
    add_global(map, "mm_milk_bar_purchase_milk", MmFlagType::Shop, 0x16);
    add_global(map, "mm_milk_bar_purchase_chateau", MmFlagType::Shop, 0x17);

    // Gorman Track Milk Purchase
    add_global(map, "mm_gorman_track_milk_purchase", MmFlagType::Shop, 0x18);
}

/// Registers business scrub mappings.
fn register_scrubs(map: &mut HashMap<&'static str, MmFlagMapping>) {
    // Clock Town Business Scrub - sells Moon's Tear trade item
    add_global(map, "mm_clock_town_business_scrub", MmFlagType::Scrub, 0x00);

    // Southern Swamp Scrubs
    add_global(map, "mm_southern_swamp_scrub_deed", MmFlagType::Scrub, 0x01);
    add_global(map, "mm_southern_swamp_scrub_shop", MmFlagType::Scrub, 0x02);

    // Goron Village Scrubs
    add_global(map, "mm_goron_village_scrub_deed", MmFlagType::Scrub, 0x03);
    add_global(
        map,
        "mm_goron_village_scrub_bomb_bag",
        MmFlagType::Scrub,
        0x04,
    );

    // Zora Hall Scrubs
    add_global(map, "mm_zora_hall_scrub_deed", MmFlagType::Scrub, 0x05);
    add_global(map, "mm_zora_hall_scrub_shop", MmFlagType::Scrub, 0x06);

    // Ikana Valley Scrub
    add_global(map, "mm_ikana_valley_scrub_shop", MmFlagType::Scrub, 0x07);

    // Termina Field Scrub Grotto
    add_global(map, "mm_termina_field_scrub", MmFlagType::Scrub, 0x08);
    add_global(map, "mm_termina_field_scrub_crate", MmFlagType::Scrub, 0x09);
}

/// Registers minigame reward mappings.
fn register_minigames(map: &mut HashMap<&'static str, MmFlagMapping>) {
    // Romani Ranch - Aliens defense reward
    add_global(
        map,
        "mm_romani_ranch_aliens",
        MmFlagType::WeekEventReg,
        0x0800,
    );

    // Beaver Race rewards
    add_global(
        map,
        "mm_waterfall_rapids_beaver_race_1",
        MmFlagType::EventInf,
        0x8000,
    );
    add_global(
        map,
        "mm_waterfall_rapids_beaver_race_2",
        MmFlagType::ItemGetInf,
        0x01,
    );

    // Zora Hall Scene Lights minigame
    add_global(
        map,
        "mm_zora_hall_scene_lights",
        MmFlagType::WeekEventReg,
        0x1000,
    );
}

/// Registers grotto location mappings.
fn register_grottos(map: &mut HashMap<&'static str, MmFlagMapping>) {
    add_scene(
        map,
        "mm_road_to_southern_swamp_grotto",
        scenes::ROAD_TO_SOUTHERN_SWAMP,
        MmFlagType::Collectible,
        0x01,
    );
    add_scene(
        map,
        "mm_southern_swamp_grotto",
        scenes::SOUTHERN_SWAMP,
        MmFlagType::Collectible,
        0x01,
    );
    add_scene(
        map,
        "mm_woods_of_mystery_grotto",
        0x59, // Woods of Mystery scene
        MmFlagType::Collectible,
        0x01,
    );
    add_scene(
        map,
        "mm_mountain_village_tunnel_grotto",
        scenes::MOUNTAIN_VILLAGE,
        MmFlagType::Collectible,
        0x01,
    );
    add_scene(
        map,
        "mm_path_to_snowhead_grotto",
        scenes::PATH_TO_SNOWHEAD,
        MmFlagType::Collectible,
        0x01,
    );
    add_scene(
        map,
        "mm_zora_cape_grotto",
        scenes::ZORA_CAPE,
        MmFlagType::Collectible,
        0x01,
    );
    add_scene(
        map,
        "mm_road_to_ikana_grotto",
        scenes::ROAD_TO_IKANA,
        MmFlagType::Collectible,
        0x01,
    );
    add_scene(
        map,
        "mm_ikana_graveyard_grotto",
        scenes::IKANA_GRAVEYARD,
        MmFlagType::Collectible,
        0x01,
    );
    add_scene(
        map,
        "mm_ikana_valley_grotto",
        scenes::IKANA_CANYON,
        MmFlagType::Collectible,
        0x02,
    );
    add_scene(
        map,
        "mm_termina_field_peahat_grotto",
        scenes::TERMINA_FIELD,
        MmFlagType::Collectible,
        0x01,
    );
    add_scene(
        map,
        "mm_termina_field_bio_baba_grotto",
        scenes::TERMINA_FIELD,
        MmFlagType::Collectible,
        0x02,
    );
    add_scene(
        map,
        "mm_termina_field_dodongo_grotto",
        scenes::TERMINA_FIELD,
        MmFlagType::Collectible,
        0x04,
    );
    add_scene(
        map,
        "mm_termina_field_pillar_grotto",
        scenes::TERMINA_FIELD,
        MmFlagType::Collectible,
        0x08,
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

fn add_scene(
    map: &mut HashMap<&'static str, MmFlagMapping>,
    location_id: &'static str,
    scene_id: u8,
    flag_type: MmFlagType,
    flag_bit: u32,
) {
    map.insert(
        location_id,
        MmFlagMapping::mapped(location_id, scene_id, flag_type, flag_bit),
    );
}
