//! MM event definitions and memory flag mappings.
//!
//! This module defines all events used in MM randomizer logic expressions and maps
//! persistent events to their memory flag locations in save data.
//!
//! # Event Categories
//!
//! Events are organized into the following categories:
//! - **Boss**: Boss defeat events (BOSS_ODOLWA, BOSS_GOHT, etc.)
//! - **Story**: Main story progression events (BOMBER_CODE, CLOCK_TOWN_SCRUB, etc.)
//! - **Dungeon**: Dungeon-specific events (temple switches, puzzles, etc.)
//! - **Overworld**: Overworld state changes (ALIENS, CLEAR_STATE_*, etc.)
//! - **Volatile**: Runtime-computed events not stored in save data
//!
//! # Memory Locations
//!
//! MM uses different flag structures than OoT:
//! - `event_inf` (4 u16 values): Main event flags
//! - `week_event_reg` (100 bytes): Week event flags that reset each 3-day cycle
//! - Scene flags: Per-scene switches, chests, room clears
//!
//! The MM save base is 0x1EF670.

use byteorder::{BigEndian, ByteOrder};
use std::collections::HashMap;

/// Categories for MM events.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MmEventCategory {
    /// Boss defeat events
    Boss,
    /// Main story progression events
    Story,
    /// Dungeon-specific events
    Dungeon,
    /// Overworld state changes
    Overworld,
    /// Volatile events computed at runtime
    Volatile,
}

/// Memory flag location for persistent events.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MmEventFlag {
    /// EventInf flag: (word_index, bit_mask)
    EventInf(u8, u16),
    /// WeekEventReg flag: (byte_index, bit_mask)
    /// Note: These flags reset each 3-day cycle
    WeekEventReg(u8, u8),
    /// Scene flag: (scene_id, flag_type, bit_mask)
    /// Flag types: 0=chests, 1=switches, 2=room_clear, 3=collectible
    SceneFlag(u8, u8, u32),
    /// Not stored in save data (volatile)
    Volatile,
}

/// All MM events used in randomizer logic.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub enum MmEvent {
    // ============================================
    // Boss Events (4 total)
    // ============================================
    BOSS_ODOLWA,
    BOSS_GOHT,
    BOSS_GYORG,
    BOSS_TWINMOLD,

    // ============================================
    // Story/Quest Events - Clock Town
    // ============================================
    BOMBER_CODE,
    CLOCK_TOWN_SCRUB,
    GUESS_BOMBER,
    HIDE_SEEK1,
    HIDE_SEEK2,
    HIDE_SEEK3,
    BOMBERS_EAST1,
    BOMBERS_EAST2,
    BOMBERS_EAST3,
    BOMBERS_NORTH1,
    BOMBERS_NORTH2,
    BOMBERS_NORTH3,
    BOMBERS_WEST1,
    BOMBERS_WEST2,
    BOMBERS_WEST3,
    MAIL_LETTER,
    MEET_ANJU,
    MEET_KAFEI,
    DELIVER_PENDANT,
    POSTMAN_FREEDOM,
    SAKON_BOMB_BAG,
    SAKON_BOOM,
    SETUP_MEET,
    SPI_ROOF_FARORE,
    TOILET_HAND,
    TOILET_RUTO_LETTER,

    // ============================================
    // Story/Quest Events - General
    // ============================================
    ARROWS,
    BUGS,
    BUY_KEG,
    FAIRY,
    FISH,
    FROG_1,
    FROG_2,
    FROG_3,
    FROG_4,
    GORON_FOOD,
    HD_REWARD_1,
    HD_REWARD_2,
    HD_REWARD_3,
    DEKU_REWARD_1,
    DEKU_REWARD_2,
    DEKU_REWARD_3,
    MAGIC,
    MUSHROOM,
    NUTS,
    PICTURE_DEKU_KING,
    PICTURE_TINGLE,
    PLAY_LOTTERY,
    RUPEES,
    SCARECROW,
    SCRUB_TELESCOPE,
    STICKS,
    SUN_MASK,
    TEAR_TELESCOPE,
    WATER,
    BOMBS_OR_BOMBCHU,

    // ============================================
    // Regional Clear State Events
    // ============================================
    CLEAR_STATE_GREAT_BAY,
    CLEAR_STATE_IKANA,
    CLEAR_STATE_SNOWHEAD,
    CLEAR_STATE_WOODFALL,

    // ============================================
    // Swamp Events
    // ============================================
    DEKU_PRINCESS,
    KOUME,
    MAGIC_BEANS_PALACE,
    MEET_KOUME,
    MEET_KOUME_DAY1,
    OPEN_WOODFALL_TEMPLE,
    PICTURE_BIG_OCTO,
    PICTURE_SWAMP,
    RETURN_PRINCESS,
    SOUTHERN_SWAMP_HIVE,
    SWAMP_KOTAKE,
    SWAMP_OCTO_LEFT,
    SWAMP_OCTO_RIGHT,
    SWAMP_SONG,

    // ============================================
    // Mountain Events
    // ============================================
    BLACKSMITH_ENABLED,
    GOLD_DUST_USED,
    GORON_GRAVE_FARORE,
    HOT_WATER_NORTH_SPRING,
    HOT_WATER_NORTH_WINTER,
    HOT_WATER_WELL_SPRING,
    HOT_WATER_WELL_WINTER,
    MOUNTAIN_SONG,
    OPEN_SNOWHEAD_TEMPLE,
    POWDER_KEG_TRIAL,

    // ============================================
    // Ocean Events
    // ============================================
    ALIENS,
    DAMPE_BIG_POE,
    OCEAN_SONG,
    PHOTO_GERUDO,
    SEAHORSE,
    ZORA_EGGS_BARREL_MAZE,
    ZORA_EGGS_HOOKSHOT_ROOM,
    ZORA_EGGS_LONE_GUARD,
    ZORA_EGGS_PINNACLE_ROCK,
    ZORA_EGGS_TREASURE_ROOM,
    FORTRESS_BEEHIVE,

    // ============================================
    // Canyon Events
    // ============================================
    CANYON_SONG,
    IKANA_CASTLE_LIGHT,
    IKANA_CASTLE_LIGHT2,
    IKANA_CASTLE_LIGHT_ENTRANCE,
    IKANA_CURSE_LIFTED,
    WELL_BIG_POE,

    // ============================================
    // Great Bay Temple Events
    // ============================================
    GB_PIPE_GREEN,
    GB_PIPE_GREEN2,
    GB_PIPE_RED,
    GB_PIPE_RED2,
    GB_WATER_WHEEL,

    // ============================================
    // Snowhead Temple Events
    // ============================================
    SHT_STICK_RUN,
    SNOWHEAD_PUSH_BLOCK,
    SNOWHEAD_RAISE_PILLAR,

    // ============================================
    // Stone Tower Temple Events
    // ============================================
    POE,
    STONE_TOWER_BRIDGE_CHEST_SWITCH,
    STONE_TOWER_EAST_ENTRY_BLOCK,
    STONE_TOWER_ENTRANCE_CHEST_SWITCH,
    STONE_TOWER_WATER_CHEST_SUN,
    STONE_TOWER_WATER_CHEST_SWITCH,
    STONE_TOWER_WEST_GARDEN_LIGHT,
    ST_WATER_CRYSTAL,

    // ============================================
    // Woodfall Temple Events
    // ============================================
    WOODFALL_TEMPLE_MAIN_FLOWER,
    WOODFALL_TEMPLE_MAIN_LADDER,

    // ============================================
    // Secret Shrine Events
    // ============================================
    SECRET_SHRINE_DINOLFOS,
    SECRET_SHRINE_GARO,
    SECRET_SHRINE_WART,
    SECRET_SHRINE_WIZZROBE,

    // ============================================
    // Moon Events
    // ============================================
    MAJORA,
    MAJORA_PHASE_1,
    MAJORA_PRE_BOSS,
    MM_MOON_OPEN,
    MOON_TRIAL_DEKU,
    MOON_TRIAL_GORON,
    MOON_TRIAL_LINK,
    MOON_TRIAL_ZORA,
    TRIAL_BOULDER,
}

impl MmEvent {
    /// Parse an event name string to an MmEvent.
    ///
    /// Event names are case-insensitive and can use either snake_case or SCREAMING_SNAKE_CASE.
    #[must_use]
    pub fn from_name(name: &str) -> Option<Self> {
        EVENT_NAME_MAP.get(name.to_uppercase().as_str()).copied()
    }

    /// Get the string name of this event.
    #[must_use]
    pub fn name(&self) -> &'static str {
        match self {
            // Boss events
            Self::BOSS_ODOLWA => "BOSS_ODOLWA",
            Self::BOSS_GOHT => "BOSS_GOHT",
            Self::BOSS_GYORG => "BOSS_GYORG",
            Self::BOSS_TWINMOLD => "BOSS_TWINMOLD",
            // Clock Town events
            Self::BOMBER_CODE => "BOMBER_CODE",
            Self::CLOCK_TOWN_SCRUB => "CLOCK_TOWN_SCRUB",
            Self::GUESS_BOMBER => "GUESS_BOMBER",
            Self::HIDE_SEEK1 => "HIDE_SEEK1",
            Self::HIDE_SEEK2 => "HIDE_SEEK2",
            Self::HIDE_SEEK3 => "HIDE_SEEK3",
            Self::BOMBERS_EAST1 => "BOMBERS_EAST1",
            Self::BOMBERS_EAST2 => "BOMBERS_EAST2",
            Self::BOMBERS_EAST3 => "BOMBERS_EAST3",
            Self::BOMBERS_NORTH1 => "BOMBERS_NORTH1",
            Self::BOMBERS_NORTH2 => "BOMBERS_NORTH2",
            Self::BOMBERS_NORTH3 => "BOMBERS_NORTH3",
            Self::BOMBERS_WEST1 => "BOMBERS_WEST1",
            Self::BOMBERS_WEST2 => "BOMBERS_WEST2",
            Self::BOMBERS_WEST3 => "BOMBERS_WEST3",
            Self::MAIL_LETTER => "MAIL_LETTER",
            Self::MEET_ANJU => "MEET_ANJU",
            Self::MEET_KAFEI => "MEET_KAFEI",
            Self::DELIVER_PENDANT => "DELIVER_PENDANT",
            Self::POSTMAN_FREEDOM => "POSTMAN_FREEDOM",
            Self::SAKON_BOMB_BAG => "SAKON_BOMB_BAG",
            Self::SAKON_BOOM => "SAKON_BOOM",
            Self::SETUP_MEET => "SETUP_MEET",
            Self::SPI_ROOF_FARORE => "SPI_ROOF_FARORE",
            Self::TOILET_HAND => "TOILET_HAND",
            Self::TOILET_RUTO_LETTER => "TOILET_RUTO_LETTER",
            // General quest events
            Self::ARROWS => "ARROWS",
            Self::BUGS => "BUGS",
            Self::BUY_KEG => "BUY_KEG",
            Self::FAIRY => "FAIRY",
            Self::FISH => "FISH",
            Self::FROG_1 => "FROG_1",
            Self::FROG_2 => "FROG_2",
            Self::FROG_3 => "FROG_3",
            Self::FROG_4 => "FROG_4",
            Self::GORON_FOOD => "GORON_FOOD",
            Self::HD_REWARD_1 => "HD_REWARD_1",
            Self::HD_REWARD_2 => "HD_REWARD_2",
            Self::HD_REWARD_3 => "HD_REWARD_3",
            Self::DEKU_REWARD_1 => "DEKU_REWARD_1",
            Self::DEKU_REWARD_2 => "DEKU_REWARD_2",
            Self::DEKU_REWARD_3 => "DEKU_REWARD_3",
            Self::MAGIC => "MAGIC",
            Self::MUSHROOM => "MUSHROOM",
            Self::NUTS => "NUTS",
            Self::PICTURE_DEKU_KING => "PICTURE_DEKU_KING",
            Self::PICTURE_TINGLE => "PICTURE_TINGLE",
            Self::PLAY_LOTTERY => "PLAY_LOTTERY",
            Self::RUPEES => "RUPEES",
            Self::SCARECROW => "SCARECROW",
            Self::SCRUB_TELESCOPE => "SCRUB_TELESCOPE",
            Self::STICKS => "STICKS",
            Self::SUN_MASK => "SUN_MASK",
            Self::TEAR_TELESCOPE => "TEAR_TELESCOPE",
            Self::WATER => "WATER",
            Self::BOMBS_OR_BOMBCHU => "BOMBS_OR_BOMBCHU",
            // Clear state events
            Self::CLEAR_STATE_GREAT_BAY => "CLEAR_STATE_GREAT_BAY",
            Self::CLEAR_STATE_IKANA => "CLEAR_STATE_IKANA",
            Self::CLEAR_STATE_SNOWHEAD => "CLEAR_STATE_SNOWHEAD",
            Self::CLEAR_STATE_WOODFALL => "CLEAR_STATE_WOODFALL",
            // Swamp events
            Self::DEKU_PRINCESS => "DEKU_PRINCESS",
            Self::KOUME => "KOUME",
            Self::MAGIC_BEANS_PALACE => "MAGIC_BEANS_PALACE",
            Self::MEET_KOUME => "MEET_KOUME",
            Self::MEET_KOUME_DAY1 => "MEET_KOUME_DAY1",
            Self::OPEN_WOODFALL_TEMPLE => "OPEN_WOODFALL_TEMPLE",
            Self::PICTURE_BIG_OCTO => "PICTURE_BIG_OCTO",
            Self::PICTURE_SWAMP => "PICTURE_SWAMP",
            Self::RETURN_PRINCESS => "RETURN_PRINCESS",
            Self::SOUTHERN_SWAMP_HIVE => "SOUTHERN_SWAMP_HIVE",
            Self::SWAMP_KOTAKE => "SWAMP_KOTAKE",
            Self::SWAMP_OCTO_LEFT => "SWAMP_OCTO_LEFT",
            Self::SWAMP_OCTO_RIGHT => "SWAMP_OCTO_RIGHT",
            Self::SWAMP_SONG => "SWAMP_SONG",
            // Mountain events
            Self::BLACKSMITH_ENABLED => "BLACKSMITH_ENABLED",
            Self::GOLD_DUST_USED => "GOLD_DUST_USED",
            Self::GORON_GRAVE_FARORE => "GORON_GRAVE_FARORE",
            Self::HOT_WATER_NORTH_SPRING => "HOT_WATER_NORTH_SPRING",
            Self::HOT_WATER_NORTH_WINTER => "HOT_WATER_NORTH_WINTER",
            Self::HOT_WATER_WELL_SPRING => "HOT_WATER_WELL_SPRING",
            Self::HOT_WATER_WELL_WINTER => "HOT_WATER_WELL_WINTER",
            Self::MOUNTAIN_SONG => "MOUNTAIN_SONG",
            Self::OPEN_SNOWHEAD_TEMPLE => "OPEN_SNOWHEAD_TEMPLE",
            Self::POWDER_KEG_TRIAL => "POWDER_KEG_TRIAL",
            // Ocean events
            Self::ALIENS => "ALIENS",
            Self::DAMPE_BIG_POE => "DAMPE_BIG_POE",
            Self::OCEAN_SONG => "OCEAN_SONG",
            Self::PHOTO_GERUDO => "PHOTO_GERUDO",
            Self::SEAHORSE => "SEAHORSE",
            Self::ZORA_EGGS_BARREL_MAZE => "ZORA_EGGS_BARREL_MAZE",
            Self::ZORA_EGGS_HOOKSHOT_ROOM => "ZORA_EGGS_HOOKSHOT_ROOM",
            Self::ZORA_EGGS_LONE_GUARD => "ZORA_EGGS_LONE_GUARD",
            Self::ZORA_EGGS_PINNACLE_ROCK => "ZORA_EGGS_PINNACLE_ROCK",
            Self::ZORA_EGGS_TREASURE_ROOM => "ZORA_EGGS_TREASURE_ROOM",
            Self::FORTRESS_BEEHIVE => "FORTRESS_BEEHIVE",
            // Canyon events
            Self::CANYON_SONG => "CANYON_SONG",
            Self::IKANA_CASTLE_LIGHT => "IKANA_CASTLE_LIGHT",
            Self::IKANA_CASTLE_LIGHT2 => "IKANA_CASTLE_LIGHT2",
            Self::IKANA_CASTLE_LIGHT_ENTRANCE => "IKANA_CASTLE_LIGHT_ENTRANCE",
            Self::IKANA_CURSE_LIFTED => "IKANA_CURSE_LIFTED",
            Self::WELL_BIG_POE => "WELL_BIG_POE",
            // Great Bay Temple events
            Self::GB_PIPE_GREEN => "GB_PIPE_GREEN",
            Self::GB_PIPE_GREEN2 => "GB_PIPE_GREEN2",
            Self::GB_PIPE_RED => "GB_PIPE_RED",
            Self::GB_PIPE_RED2 => "GB_PIPE_RED2",
            Self::GB_WATER_WHEEL => "GB_WATER_WHEEL",
            // Snowhead Temple events
            Self::SHT_STICK_RUN => "SHT_STICK_RUN",
            Self::SNOWHEAD_PUSH_BLOCK => "SNOWHEAD_PUSH_BLOCK",
            Self::SNOWHEAD_RAISE_PILLAR => "SNOWHEAD_RAISE_PILLAR",
            // Stone Tower events
            Self::POE => "POE",
            Self::STONE_TOWER_BRIDGE_CHEST_SWITCH => "STONE_TOWER_BRIDGE_CHEST_SWITCH",
            Self::STONE_TOWER_EAST_ENTRY_BLOCK => "STONE_TOWER_EAST_ENTRY_BLOCK",
            Self::STONE_TOWER_ENTRANCE_CHEST_SWITCH => "STONE_TOWER_ENTRANCE_CHEST_SWITCH",
            Self::STONE_TOWER_WATER_CHEST_SUN => "STONE_TOWER_WATER_CHEST_SUN",
            Self::STONE_TOWER_WATER_CHEST_SWITCH => "STONE_TOWER_WATER_CHEST_SWITCH",
            Self::STONE_TOWER_WEST_GARDEN_LIGHT => "STONE_TOWER_WEST_GARDEN_LIGHT",
            Self::ST_WATER_CRYSTAL => "ST_WATER_CRYSTAL",
            // Woodfall Temple events
            Self::WOODFALL_TEMPLE_MAIN_FLOWER => "WOODFALL_TEMPLE_MAIN_FLOWER",
            Self::WOODFALL_TEMPLE_MAIN_LADDER => "WOODFALL_TEMPLE_MAIN_LADDER",
            // Secret Shrine events
            Self::SECRET_SHRINE_DINOLFOS => "SECRET_SHRINE_DINOLFOS",
            Self::SECRET_SHRINE_GARO => "SECRET_SHRINE_GARO",
            Self::SECRET_SHRINE_WART => "SECRET_SHRINE_WART",
            Self::SECRET_SHRINE_WIZZROBE => "SECRET_SHRINE_WIZZROBE",
            // Moon events
            Self::MAJORA => "MAJORA",
            Self::MAJORA_PHASE_1 => "MAJORA_PHASE_1",
            Self::MAJORA_PRE_BOSS => "MAJORA_PRE_BOSS",
            Self::MM_MOON_OPEN => "MM_MOON_OPEN",
            Self::MOON_TRIAL_DEKU => "MOON_TRIAL_DEKU",
            Self::MOON_TRIAL_GORON => "MOON_TRIAL_GORON",
            Self::MOON_TRIAL_LINK => "MOON_TRIAL_LINK",
            Self::MOON_TRIAL_ZORA => "MOON_TRIAL_ZORA",
            Self::TRIAL_BOULDER => "TRIAL_BOULDER",
        }
    }

    /// Get the category of this event.
    #[must_use]
    pub fn category(&self) -> MmEventCategory {
        match self {
            // Boss events
            Self::BOSS_ODOLWA | Self::BOSS_GOHT | Self::BOSS_GYORG | Self::BOSS_TWINMOLD => {
                MmEventCategory::Boss
            }

            // Story events
            Self::BOMBER_CODE
            | Self::CLOCK_TOWN_SCRUB
            | Self::GUESS_BOMBER
            | Self::HIDE_SEEK1
            | Self::HIDE_SEEK2
            | Self::HIDE_SEEK3
            | Self::BOMBERS_EAST1
            | Self::BOMBERS_EAST2
            | Self::BOMBERS_EAST3
            | Self::BOMBERS_NORTH1
            | Self::BOMBERS_NORTH2
            | Self::BOMBERS_NORTH3
            | Self::BOMBERS_WEST1
            | Self::BOMBERS_WEST2
            | Self::BOMBERS_WEST3
            | Self::MAIL_LETTER
            | Self::MEET_ANJU
            | Self::MEET_KAFEI
            | Self::DELIVER_PENDANT
            | Self::POSTMAN_FREEDOM
            | Self::SAKON_BOMB_BAG
            | Self::SAKON_BOOM
            | Self::SETUP_MEET
            | Self::SPI_ROOF_FARORE
            | Self::TOILET_HAND
            | Self::TOILET_RUTO_LETTER
            | Self::ARROWS
            | Self::BUGS
            | Self::BUY_KEG
            | Self::FAIRY
            | Self::FISH
            | Self::FROG_1
            | Self::FROG_2
            | Self::FROG_3
            | Self::FROG_4
            | Self::GORON_FOOD
            | Self::HD_REWARD_1
            | Self::HD_REWARD_2
            | Self::HD_REWARD_3
            | Self::DEKU_REWARD_1
            | Self::DEKU_REWARD_2
            | Self::DEKU_REWARD_3
            | Self::MAGIC
            | Self::MUSHROOM
            | Self::NUTS
            | Self::PICTURE_DEKU_KING
            | Self::PICTURE_TINGLE
            | Self::PLAY_LOTTERY
            | Self::RUPEES
            | Self::SCARECROW
            | Self::SCRUB_TELESCOPE
            | Self::STICKS
            | Self::SUN_MASK
            | Self::TEAR_TELESCOPE
            | Self::WATER
            | Self::BOMBS_OR_BOMBCHU => MmEventCategory::Story,

            // Overworld events
            Self::CLEAR_STATE_GREAT_BAY
            | Self::CLEAR_STATE_IKANA
            | Self::CLEAR_STATE_SNOWHEAD
            | Self::CLEAR_STATE_WOODFALL
            | Self::DEKU_PRINCESS
            | Self::KOUME
            | Self::MAGIC_BEANS_PALACE
            | Self::MEET_KOUME
            | Self::MEET_KOUME_DAY1
            | Self::OPEN_WOODFALL_TEMPLE
            | Self::PICTURE_BIG_OCTO
            | Self::PICTURE_SWAMP
            | Self::RETURN_PRINCESS
            | Self::SOUTHERN_SWAMP_HIVE
            | Self::SWAMP_KOTAKE
            | Self::SWAMP_OCTO_LEFT
            | Self::SWAMP_OCTO_RIGHT
            | Self::SWAMP_SONG
            | Self::BLACKSMITH_ENABLED
            | Self::GOLD_DUST_USED
            | Self::GORON_GRAVE_FARORE
            | Self::HOT_WATER_NORTH_SPRING
            | Self::HOT_WATER_NORTH_WINTER
            | Self::HOT_WATER_WELL_SPRING
            | Self::HOT_WATER_WELL_WINTER
            | Self::MOUNTAIN_SONG
            | Self::OPEN_SNOWHEAD_TEMPLE
            | Self::POWDER_KEG_TRIAL
            | Self::ALIENS
            | Self::DAMPE_BIG_POE
            | Self::OCEAN_SONG
            | Self::PHOTO_GERUDO
            | Self::SEAHORSE
            | Self::ZORA_EGGS_BARREL_MAZE
            | Self::ZORA_EGGS_HOOKSHOT_ROOM
            | Self::ZORA_EGGS_LONE_GUARD
            | Self::ZORA_EGGS_PINNACLE_ROCK
            | Self::ZORA_EGGS_TREASURE_ROOM
            | Self::FORTRESS_BEEHIVE
            | Self::CANYON_SONG
            | Self::IKANA_CASTLE_LIGHT
            | Self::IKANA_CASTLE_LIGHT2
            | Self::IKANA_CASTLE_LIGHT_ENTRANCE
            | Self::IKANA_CURSE_LIFTED
            | Self::WELL_BIG_POE => MmEventCategory::Overworld,

            // Dungeon events
            Self::GB_PIPE_GREEN
            | Self::GB_PIPE_GREEN2
            | Self::GB_PIPE_RED
            | Self::GB_PIPE_RED2
            | Self::GB_WATER_WHEEL
            | Self::SHT_STICK_RUN
            | Self::SNOWHEAD_PUSH_BLOCK
            | Self::SNOWHEAD_RAISE_PILLAR
            | Self::POE
            | Self::STONE_TOWER_BRIDGE_CHEST_SWITCH
            | Self::STONE_TOWER_EAST_ENTRY_BLOCK
            | Self::STONE_TOWER_ENTRANCE_CHEST_SWITCH
            | Self::STONE_TOWER_WATER_CHEST_SUN
            | Self::STONE_TOWER_WATER_CHEST_SWITCH
            | Self::STONE_TOWER_WEST_GARDEN_LIGHT
            | Self::ST_WATER_CRYSTAL
            | Self::WOODFALL_TEMPLE_MAIN_FLOWER
            | Self::WOODFALL_TEMPLE_MAIN_LADDER
            | Self::SECRET_SHRINE_DINOLFOS
            | Self::SECRET_SHRINE_GARO
            | Self::SECRET_SHRINE_WART
            | Self::SECRET_SHRINE_WIZZROBE
            | Self::MAJORA
            | Self::MAJORA_PHASE_1
            | Self::MAJORA_PRE_BOSS
            | Self::MM_MOON_OPEN
            | Self::MOON_TRIAL_DEKU
            | Self::MOON_TRIAL_GORON
            | Self::MOON_TRIAL_LINK
            | Self::MOON_TRIAL_ZORA
            | Self::TRIAL_BOULDER => MmEventCategory::Dungeon,
        }
    }

    /// Get the memory flag location for this event.
    ///
    /// Returns the flag location if the event is persistent (stored in save data),
    /// or `MmEventFlag::Volatile` if the event is computed at runtime.
    ///
    /// Note: MM flag mappings are complex due to the 3-day cycle system.
    /// Many events are volatile and computed at runtime based on game state.
    #[must_use]
    pub fn flag(&self) -> MmEventFlag {
        match self {
            // ============================================
            // Boss Events - WeekEventReg mappings
            // ============================================
            // Boss defeats are stored in WeekEventReg (reset each cycle)
            // These need to be re-obtained each cycle unless using owl saves
            Self::BOSS_ODOLWA => MmEventFlag::WeekEventReg(25, 0x01),
            Self::BOSS_GOHT => MmEventFlag::WeekEventReg(25, 0x02),
            Self::BOSS_GYORG => MmEventFlag::WeekEventReg(25, 0x04),
            Self::BOSS_TWINMOLD => MmEventFlag::WeekEventReg(25, 0x08),

            // ============================================
            // Permanent Events - EventInf mappings
            // ============================================
            // Some events persist across cycles
            Self::BOMBER_CODE => MmEventFlag::EventInf(0, 0x0001),

            // ============================================
            // Week Events - WeekEventReg mappings
            // ============================================
            // Most story progress resets each cycle

            // ============================================
            // Volatile Events - Not stored in save data
            // ============================================
            // Most dungeon events are volatile (computed at runtime)
            _ => MmEventFlag::Volatile,
        }
    }

    /// Check if this event is persistent (stored in save data).
    #[must_use]
    pub fn is_persistent(&self) -> bool {
        !matches!(self.flag(), MmEventFlag::Volatile)
    }

    /// Check if this event resets each 3-day cycle.
    #[must_use]
    pub fn is_week_event(&self) -> bool {
        matches!(self.flag(), MmEventFlag::WeekEventReg(_, _))
    }

    /// Get all MM events.
    #[must_use]
    pub fn all() -> &'static [MmEvent] {
        ALL_EVENTS
    }

    /// Get all persistent events (stored in save data).
    pub fn all_persistent() -> impl Iterator<Item = MmEvent> {
        ALL_EVENTS.iter().copied().filter(|e| e.is_persistent())
    }

    /// Get all volatile events (computed at runtime).
    pub fn all_volatile() -> impl Iterator<Item = MmEvent> {
        ALL_EVENTS.iter().copied().filter(|e| !e.is_persistent())
    }

    /// Get all events in a specific category.
    pub fn by_category(category: MmEventCategory) -> impl Iterator<Item = MmEvent> {
        ALL_EVENTS
            .iter()
            .copied()
            .filter(move |e| e.category() == category)
    }
}

impl std::fmt::Display for MmEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name())
    }
}

impl std::str::FromStr for MmEvent {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        MmEvent::from_name(s).ok_or_else(|| format!("Unknown MM event: {}", s))
    }
}

// Static list of all events for iteration
static ALL_EVENTS: &[MmEvent] = &[
    // Boss events
    MmEvent::BOSS_ODOLWA,
    MmEvent::BOSS_GOHT,
    MmEvent::BOSS_GYORG,
    MmEvent::BOSS_TWINMOLD,
    // Clock Town events
    MmEvent::BOMBER_CODE,
    MmEvent::CLOCK_TOWN_SCRUB,
    MmEvent::GUESS_BOMBER,
    MmEvent::HIDE_SEEK1,
    MmEvent::HIDE_SEEK2,
    MmEvent::HIDE_SEEK3,
    MmEvent::BOMBERS_EAST1,
    MmEvent::BOMBERS_EAST2,
    MmEvent::BOMBERS_EAST3,
    MmEvent::BOMBERS_NORTH1,
    MmEvent::BOMBERS_NORTH2,
    MmEvent::BOMBERS_NORTH3,
    MmEvent::BOMBERS_WEST1,
    MmEvent::BOMBERS_WEST2,
    MmEvent::BOMBERS_WEST3,
    MmEvent::MAIL_LETTER,
    MmEvent::MEET_ANJU,
    MmEvent::MEET_KAFEI,
    MmEvent::DELIVER_PENDANT,
    MmEvent::POSTMAN_FREEDOM,
    MmEvent::SAKON_BOMB_BAG,
    MmEvent::SAKON_BOOM,
    MmEvent::SETUP_MEET,
    MmEvent::SPI_ROOF_FARORE,
    MmEvent::TOILET_HAND,
    MmEvent::TOILET_RUTO_LETTER,
    // General quest events
    MmEvent::ARROWS,
    MmEvent::BUGS,
    MmEvent::BUY_KEG,
    MmEvent::FAIRY,
    MmEvent::FISH,
    MmEvent::FROG_1,
    MmEvent::FROG_2,
    MmEvent::FROG_3,
    MmEvent::FROG_4,
    MmEvent::GORON_FOOD,
    MmEvent::HD_REWARD_1,
    MmEvent::HD_REWARD_2,
    MmEvent::HD_REWARD_3,
    MmEvent::DEKU_REWARD_1,
    MmEvent::DEKU_REWARD_2,
    MmEvent::DEKU_REWARD_3,
    MmEvent::MAGIC,
    MmEvent::MUSHROOM,
    MmEvent::NUTS,
    MmEvent::PICTURE_DEKU_KING,
    MmEvent::PICTURE_TINGLE,
    MmEvent::PLAY_LOTTERY,
    MmEvent::RUPEES,
    MmEvent::SCARECROW,
    MmEvent::SCRUB_TELESCOPE,
    MmEvent::STICKS,
    MmEvent::SUN_MASK,
    MmEvent::TEAR_TELESCOPE,
    MmEvent::WATER,
    MmEvent::BOMBS_OR_BOMBCHU,
    // Clear state events
    MmEvent::CLEAR_STATE_GREAT_BAY,
    MmEvent::CLEAR_STATE_IKANA,
    MmEvent::CLEAR_STATE_SNOWHEAD,
    MmEvent::CLEAR_STATE_WOODFALL,
    // Swamp events
    MmEvent::DEKU_PRINCESS,
    MmEvent::KOUME,
    MmEvent::MAGIC_BEANS_PALACE,
    MmEvent::MEET_KOUME,
    MmEvent::MEET_KOUME_DAY1,
    MmEvent::OPEN_WOODFALL_TEMPLE,
    MmEvent::PICTURE_BIG_OCTO,
    MmEvent::PICTURE_SWAMP,
    MmEvent::RETURN_PRINCESS,
    MmEvent::SOUTHERN_SWAMP_HIVE,
    MmEvent::SWAMP_KOTAKE,
    MmEvent::SWAMP_OCTO_LEFT,
    MmEvent::SWAMP_OCTO_RIGHT,
    MmEvent::SWAMP_SONG,
    // Mountain events
    MmEvent::BLACKSMITH_ENABLED,
    MmEvent::GOLD_DUST_USED,
    MmEvent::GORON_GRAVE_FARORE,
    MmEvent::HOT_WATER_NORTH_SPRING,
    MmEvent::HOT_WATER_NORTH_WINTER,
    MmEvent::HOT_WATER_WELL_SPRING,
    MmEvent::HOT_WATER_WELL_WINTER,
    MmEvent::MOUNTAIN_SONG,
    MmEvent::OPEN_SNOWHEAD_TEMPLE,
    MmEvent::POWDER_KEG_TRIAL,
    // Ocean events
    MmEvent::ALIENS,
    MmEvent::DAMPE_BIG_POE,
    MmEvent::OCEAN_SONG,
    MmEvent::PHOTO_GERUDO,
    MmEvent::SEAHORSE,
    MmEvent::ZORA_EGGS_BARREL_MAZE,
    MmEvent::ZORA_EGGS_HOOKSHOT_ROOM,
    MmEvent::ZORA_EGGS_LONE_GUARD,
    MmEvent::ZORA_EGGS_PINNACLE_ROCK,
    MmEvent::ZORA_EGGS_TREASURE_ROOM,
    MmEvent::FORTRESS_BEEHIVE,
    // Canyon events
    MmEvent::CANYON_SONG,
    MmEvent::IKANA_CASTLE_LIGHT,
    MmEvent::IKANA_CASTLE_LIGHT2,
    MmEvent::IKANA_CASTLE_LIGHT_ENTRANCE,
    MmEvent::IKANA_CURSE_LIFTED,
    MmEvent::WELL_BIG_POE,
    // Great Bay Temple events
    MmEvent::GB_PIPE_GREEN,
    MmEvent::GB_PIPE_GREEN2,
    MmEvent::GB_PIPE_RED,
    MmEvent::GB_PIPE_RED2,
    MmEvent::GB_WATER_WHEEL,
    // Snowhead Temple events
    MmEvent::SHT_STICK_RUN,
    MmEvent::SNOWHEAD_PUSH_BLOCK,
    MmEvent::SNOWHEAD_RAISE_PILLAR,
    // Stone Tower events
    MmEvent::POE,
    MmEvent::STONE_TOWER_BRIDGE_CHEST_SWITCH,
    MmEvent::STONE_TOWER_EAST_ENTRY_BLOCK,
    MmEvent::STONE_TOWER_ENTRANCE_CHEST_SWITCH,
    MmEvent::STONE_TOWER_WATER_CHEST_SUN,
    MmEvent::STONE_TOWER_WATER_CHEST_SWITCH,
    MmEvent::STONE_TOWER_WEST_GARDEN_LIGHT,
    MmEvent::ST_WATER_CRYSTAL,
    // Woodfall Temple events
    MmEvent::WOODFALL_TEMPLE_MAIN_FLOWER,
    MmEvent::WOODFALL_TEMPLE_MAIN_LADDER,
    // Secret Shrine events
    MmEvent::SECRET_SHRINE_DINOLFOS,
    MmEvent::SECRET_SHRINE_GARO,
    MmEvent::SECRET_SHRINE_WART,
    MmEvent::SECRET_SHRINE_WIZZROBE,
    // Moon events
    MmEvent::MAJORA,
    MmEvent::MAJORA_PHASE_1,
    MmEvent::MAJORA_PRE_BOSS,
    MmEvent::MM_MOON_OPEN,
    MmEvent::MOON_TRIAL_DEKU,
    MmEvent::MOON_TRIAL_GORON,
    MmEvent::MOON_TRIAL_LINK,
    MmEvent::MOON_TRIAL_ZORA,
    MmEvent::TRIAL_BOULDER,
];

// Build name lookup map at compile time using lazy_static pattern
use std::sync::LazyLock;

static EVENT_NAME_MAP: LazyLock<HashMap<&'static str, MmEvent>> = LazyLock::new(|| {
    let mut map = HashMap::new();
    for event in ALL_EVENTS {
        map.insert(event.name(), *event);
    }
    map
});

/// MM save data offsets for event flags.
///
/// The MM save base is 0x1EF670 in N64 RAM.
pub mod offsets {
    /// EventInf offset in save data (4 u16 values = 8 bytes).
    pub const EVENT_INF: usize = 0x0EF8;
    /// EventInf size in bytes.
    pub const EVENT_INF_SIZE: usize = 8;
    /// WeekEventReg offset in save data (100 bytes).
    /// These flags reset each 3-day cycle.
    pub const WEEK_EVENT_REG: usize = 0x0F18;
    /// WeekEventReg size in bytes.
    pub const WEEK_EVENT_REG_SIZE: usize = 100;
    /// Scene flags offset in save data.
    pub const SCENE_FLAGS: usize = 0x00D4;
    /// Size of each scene's flags (0x1C = 28 bytes).
    pub const SCENE_SIZE: usize = 0x1C;
    /// Number of scenes in MM.
    pub const NUM_SCENES: usize = 0x78;
}

/// Error type for event reading operations.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MmEventReadError {
    /// Save data buffer is too small.
    BufferTooSmall { expected: usize, actual: usize },
    /// Event is volatile and not stored in save data.
    VolatileEvent(MmEvent),
    /// Invalid scene index.
    InvalidScene(u8),
}

impl std::fmt::Display for MmEventReadError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::BufferTooSmall { expected, actual } => {
                write!(
                    f,
                    "Buffer too small: expected {} bytes, got {}",
                    expected, actual
                )
            }
            Self::VolatileEvent(event) => {
                write!(f, "Event {} is volatile and not stored in save data", event)
            }
            Self::InvalidScene(id) => {
                write!(f, "Invalid scene ID: {}", id)
            }
        }
    }
}

impl std::error::Error for MmEventReadError {}

/// Reader for MM event flags from save data.
///
/// This struct provides methods to check event states from raw save data bytes.
///
/// # Example
///
/// ```
/// use ootmm::events::mm::{MmEvent, MmEventReader};
///
/// // Create a mock save buffer (in real use, this would be actual save data)
/// let save_data = vec![0u8; 0x2000];
/// let reader = MmEventReader::new(&save_data);
///
/// // Check if an event is set (will return false for empty data)
/// if let Ok(is_set) = reader.is_event_set(MmEvent::BOSS_ODOLWA) {
///     println!("Odolwa defeated: {}", is_set);
/// }
/// ```
pub struct MmEventReader<'a> {
    data: &'a [u8],
}

impl<'a> MmEventReader<'a> {
    /// Create a new event reader from save data.
    ///
    /// The data should be the full save data buffer, or at least contain
    /// the relevant flag sections.
    #[must_use]
    pub const fn new(data: &'a [u8]) -> Self {
        Self { data }
    }

    /// Check if an event flag is set.
    ///
    /// Returns `Ok(true)` if the event is set, `Ok(false)` if not set,
    /// or an error if the event is volatile or the buffer is too small.
    pub fn is_event_set(&self, event: MmEvent) -> Result<bool, MmEventReadError> {
        match event.flag() {
            MmEventFlag::EventInf(word_idx, mask) => self.read_event_inf(word_idx, mask),
            MmEventFlag::WeekEventReg(byte_idx, mask) => self.read_week_event_reg(byte_idx, mask),
            MmEventFlag::SceneFlag(scene_id, flag_type, mask) => {
                self.read_scene_flag(scene_id, flag_type, mask)
            }
            MmEventFlag::Volatile => Err(MmEventReadError::VolatileEvent(event)),
        }
    }

    /// Read a flag from EventInf.
    fn read_event_inf(&self, word_idx: u8, mask: u16) -> Result<bool, MmEventReadError> {
        let offset = offsets::EVENT_INF + (word_idx as usize) * 2;
        let end = offset + 2;

        if self.data.len() < end {
            return Err(MmEventReadError::BufferTooSmall {
                expected: end,
                actual: self.data.len(),
            });
        }

        let word = BigEndian::read_u16(&self.data[offset..end]);
        Ok((word & mask) != 0)
    }

    /// Read a flag from WeekEventReg.
    fn read_week_event_reg(&self, byte_idx: u8, mask: u8) -> Result<bool, MmEventReadError> {
        let offset = offsets::WEEK_EVENT_REG + byte_idx as usize;

        if self.data.len() <= offset {
            return Err(MmEventReadError::BufferTooSmall {
                expected: offset + 1,
                actual: self.data.len(),
            });
        }

        Ok((self.data[offset] & mask) != 0)
    }

    /// Read a flag from scene flags.
    fn read_scene_flag(
        &self,
        scene_id: u8,
        flag_type: u8,
        mask: u32,
    ) -> Result<bool, MmEventReadError> {
        if scene_id as usize >= offsets::NUM_SCENES {
            return Err(MmEventReadError::InvalidScene(scene_id));
        }

        // Flag types: 0=chests, 1=switches, 2=room_clear, 3=collectible
        let flag_offset = match flag_type {
            0 => 0x00, // chests
            1 => 0x04, // switches
            2 => 0x08, // room_clear
            3 => 0x0C, // collectible
            _ => return Err(MmEventReadError::InvalidScene(scene_id)),
        };

        let scene_offset = offsets::SCENE_FLAGS + (scene_id as usize) * offsets::SCENE_SIZE;
        let offset = scene_offset + flag_offset;
        let end = offset + 4;

        if self.data.len() < end {
            return Err(MmEventReadError::BufferTooSmall {
                expected: end,
                actual: self.data.len(),
            });
        }

        let flags = BigEndian::read_u32(&self.data[offset..end]);
        Ok((flags & mask) != 0)
    }

    /// Get all set persistent events.
    ///
    /// Returns an iterator over all persistent events that are currently set
    /// in the save data.
    pub fn get_set_events(&self) -> impl Iterator<Item = MmEvent> + '_ {
        MmEvent::all_persistent().filter(|event| self.is_event_set(*event).unwrap_or(false))
    }

    /// Get all boss clear events that are set.
    pub fn get_boss_clears(&self) -> impl Iterator<Item = MmEvent> + '_ {
        MmEvent::by_category(MmEventCategory::Boss)
            .filter(|event| self.is_event_set(*event).unwrap_or(false))
    }

    /// Count how many bosses have been defeated.
    #[must_use]
    pub fn count_boss_clears(&self) -> usize {
        self.get_boss_clears().count()
    }
}

/// Writer for MM event flags to save data.
///
/// This struct provides methods to set event states in raw save data bytes.
pub struct MmEventWriter<'a> {
    data: &'a mut [u8],
}

impl<'a> MmEventWriter<'a> {
    /// Create a new event writer from mutable save data.
    #[must_use]
    pub fn new(data: &'a mut [u8]) -> Self {
        Self { data }
    }

    /// Set an event flag.
    ///
    /// Returns an error if the event is volatile or the buffer is too small.
    pub fn set_event(&mut self, event: MmEvent, value: bool) -> Result<(), MmEventReadError> {
        match event.flag() {
            MmEventFlag::EventInf(word_idx, mask) => self.write_event_inf(word_idx, mask, value),
            MmEventFlag::WeekEventReg(byte_idx, mask) => {
                self.write_week_event_reg(byte_idx, mask, value)
            }
            MmEventFlag::SceneFlag(scene_id, flag_type, mask) => {
                self.write_scene_flag(scene_id, flag_type, mask, value)
            }
            MmEventFlag::Volatile => Err(MmEventReadError::VolatileEvent(event)),
        }
    }

    /// Write a flag to EventInf.
    fn write_event_inf(
        &mut self,
        word_idx: u8,
        mask: u16,
        value: bool,
    ) -> Result<(), MmEventReadError> {
        let offset = offsets::EVENT_INF + (word_idx as usize) * 2;
        let end = offset + 2;

        if self.data.len() < end {
            return Err(MmEventReadError::BufferTooSmall {
                expected: end,
                actual: self.data.len(),
            });
        }

        let mut word = BigEndian::read_u16(&self.data[offset..end]);
        if value {
            word |= mask;
        } else {
            word &= !mask;
        }
        BigEndian::write_u16(&mut self.data[offset..end], word);
        Ok(())
    }

    /// Write a flag to WeekEventReg.
    fn write_week_event_reg(
        &mut self,
        byte_idx: u8,
        mask: u8,
        value: bool,
    ) -> Result<(), MmEventReadError> {
        let offset = offsets::WEEK_EVENT_REG + byte_idx as usize;

        if self.data.len() <= offset {
            return Err(MmEventReadError::BufferTooSmall {
                expected: offset + 1,
                actual: self.data.len(),
            });
        }

        if value {
            self.data[offset] |= mask;
        } else {
            self.data[offset] &= !mask;
        }
        Ok(())
    }

    /// Write a flag to scene flags.
    fn write_scene_flag(
        &mut self,
        scene_id: u8,
        flag_type: u8,
        mask: u32,
        value: bool,
    ) -> Result<(), MmEventReadError> {
        if scene_id as usize >= offsets::NUM_SCENES {
            return Err(MmEventReadError::InvalidScene(scene_id));
        }

        let flag_offset = match flag_type {
            0 => 0x00,
            1 => 0x04,
            2 => 0x08,
            3 => 0x0C,
            _ => return Err(MmEventReadError::InvalidScene(scene_id)),
        };

        let scene_offset = offsets::SCENE_FLAGS + (scene_id as usize) * offsets::SCENE_SIZE;
        let offset = scene_offset + flag_offset;
        let end = offset + 4;

        if self.data.len() < end {
            return Err(MmEventReadError::BufferTooSmall {
                expected: end,
                actual: self.data.len(),
            });
        }

        let mut flags = BigEndian::read_u32(&self.data[offset..end]);
        if value {
            flags |= mask;
        } else {
            flags &= !mask;
        }
        BigEndian::write_u32(&mut self.data[offset..end], flags);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_all_events_have_names() {
        for event in MmEvent::all() {
            assert!(!event.name().is_empty(), "Event {:?} has empty name", event);
        }
    }

    #[test]
    fn test_event_from_name() {
        assert_eq!(
            MmEvent::from_name("BOSS_ODOLWA"),
            Some(MmEvent::BOSS_ODOLWA)
        );
        assert_eq!(
            MmEvent::from_name("boss_odolwa"),
            Some(MmEvent::BOSS_ODOLWA)
        );
        assert_eq!(
            MmEvent::from_name("BOMBER_CODE"),
            Some(MmEvent::BOMBER_CODE)
        );
        assert_eq!(MmEvent::from_name("NOT_A_REAL_EVENT"), None);
    }

    #[test]
    fn test_boss_events_are_persistent() {
        for event in MmEvent::by_category(MmEventCategory::Boss) {
            assert!(
                event.is_persistent(),
                "Boss event {:?} should be persistent",
                event
            );
        }
    }

    #[test]
    fn test_boss_events_are_week_events() {
        // MM boss flags reset each cycle
        for event in MmEvent::by_category(MmEventCategory::Boss) {
            assert!(
                event.is_week_event(),
                "Boss event {:?} should be a week event",
                event
            );
        }
    }

    #[test]
    fn test_boss_flag_mappings() {
        // Verify boss events map to WeekEventReg
        assert!(matches!(
            MmEvent::BOSS_ODOLWA.flag(),
            MmEventFlag::WeekEventReg(25, 0x01)
        ));
        assert!(matches!(
            MmEvent::BOSS_GOHT.flag(),
            MmEventFlag::WeekEventReg(25, 0x02)
        ));
        assert!(matches!(
            MmEvent::BOSS_GYORG.flag(),
            MmEventFlag::WeekEventReg(25, 0x04)
        ));
        assert!(matches!(
            MmEvent::BOSS_TWINMOLD.flag(),
            MmEventFlag::WeekEventReg(25, 0x08)
        ));
    }

    #[test]
    fn test_event_reader_empty_data() {
        let data = vec![0u8; 0x2000];
        let reader = MmEventReader::new(&data);

        // All persistent events should return false for empty data
        for event in MmEvent::all_persistent() {
            assert_eq!(reader.is_event_set(event).unwrap(), false);
        }
    }

    #[test]
    fn test_event_reader_volatile() {
        let data = vec![0u8; 0x2000];
        let reader = MmEventReader::new(&data);

        // Volatile events should return an error
        let result = reader.is_event_set(MmEvent::CLOCK_TOWN_SCRUB);
        assert!(matches!(result, Err(MmEventReadError::VolatileEvent(_))));
    }

    #[test]
    fn test_event_writer() {
        let mut data = vec![0u8; 0x2000];

        {
            let mut writer = MmEventWriter::new(&mut data);
            writer.set_event(MmEvent::BOSS_ODOLWA, true).unwrap();
        }

        let reader = MmEventReader::new(&data);
        assert!(reader.is_event_set(MmEvent::BOSS_ODOLWA).unwrap());
        assert!(!reader.is_event_set(MmEvent::BOSS_GOHT).unwrap());
    }

    #[test]
    fn test_count_boss_clears() {
        let mut data = vec![0u8; 0x2000];

        {
            let mut writer = MmEventWriter::new(&mut data);
            writer.set_event(MmEvent::BOSS_ODOLWA, true).unwrap();
            writer.set_event(MmEvent::BOSS_GYORG, true).unwrap();
        }

        let reader = MmEventReader::new(&data);
        assert_eq!(reader.count_boss_clears(), 2);
    }

    #[test]
    fn test_category_classification() {
        assert_eq!(MmEvent::BOSS_ODOLWA.category(), MmEventCategory::Boss);
        assert_eq!(MmEvent::BOMBER_CODE.category(), MmEventCategory::Story);
        assert_eq!(
            MmEvent::CLEAR_STATE_WOODFALL.category(),
            MmEventCategory::Overworld
        );
        assert_eq!(MmEvent::GB_WATER_WHEEL.category(), MmEventCategory::Dungeon);
    }
}
