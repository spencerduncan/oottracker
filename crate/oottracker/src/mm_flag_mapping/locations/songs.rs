//! Song location mappings.
//!
//! Songs are stored in quest_items bitfield at specific bit positions.

use std::collections::HashMap;

use crate::mm_flag_mapping::{MmFlagMapping, MmFlagType};

/// Song bit positions in quest_items bitfield.
#[allow(dead_code)]
mod song_bits {
    pub const SONATA_OF_AWAKENING: u32 = 6;
    pub const GORON_LULLABY: u32 = 7;
    pub const NEW_WAVE_BOSSA_NOVA: u32 = 8;
    pub const ELEGY_OF_EMPTINESS: u32 = 9;
    pub const OATH_TO_ORDER: u32 = 10;
    pub const SONG_OF_TIME: u32 = 12;
    pub const SONG_OF_HEALING: u32 = 13;
    pub const EPONAS_SONG: u32 = 14;
    pub const SONG_OF_SOARING: u32 = 15;
    pub const SONG_OF_STORMS: u32 = 16;
    pub const LULLABY_INTRO: u32 = 24;
}

/// Registers song location mappings into the provided map.
pub fn register_songs(map: &mut HashMap<&'static str, MmFlagMapping>) {
    // Song of Healing - learned from Happy Mask Salesman in Clock Tower
    add_global(
        map,
        "mm_initial_song_of_healing",
        MmFlagType::Song,
        1 << song_bits::SONG_OF_HEALING,
    );

    // Sonata of Awakening - learned from Deku Butler's Son in Deku Palace
    add_global(
        map,
        "mm_deku_palace_sonata_of_awakening",
        MmFlagType::Song,
        1 << song_bits::SONATA_OF_AWAKENING,
    );

    // Goron Lullaby (Intro) - learned from crying Goron Baby
    add_global(
        map,
        "mm_goron_baby",
        MmFlagType::Song,
        1 << song_bits::LULLABY_INTRO,
    );

    // Goron Lullaby (Full) - learned from Goron Elder
    add_global(
        map,
        "mm_goron_elder",
        MmFlagType::Song,
        1 << song_bits::GORON_LULLABY,
    );

    // New Wave Bossa Nova - learned at Marine Research Lab
    add_global(
        map,
        "mm_laboratory_zora_song",
        MmFlagType::Song,
        1 << song_bits::NEW_WAVE_BOSSA_NOVA,
    );

    // Elegy of Emptiness - learned from Igos du Ikana in Ancient Castle
    add_global(
        map,
        "mm_ancient_castle_of_ikana_elegy",
        MmFlagType::Song,
        1 << song_bits::ELEGY_OF_EMPTINESS,
    );

    // Song of Storms - learned from Flat's ghost beneath the graveyard
    add_global(
        map,
        "mm_beneath_the_graveyard_song_of_storms",
        MmFlagType::Song,
        1 << song_bits::SONG_OF_STORMS,
    );

    // Epona's Song - learned from Romani at Romani Ranch
    add_global(
        map,
        "mm_romani_ranch_epona_song",
        MmFlagType::Song,
        1 << song_bits::EPONAS_SONG,
    );

    // Song of Soaring - learned from Owl in Southern Swamp
    add_global(
        map,
        "mm_southern_swamp_song_of_soaring",
        MmFlagType::Song,
        1 << song_bits::SONG_OF_SOARING,
    );

    // Oath to Order - learned after defeating all four bosses
    // This is typically obtained during the game's finale sequence
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
