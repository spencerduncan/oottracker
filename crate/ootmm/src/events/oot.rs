//! OoT event definitions and memory flag mappings.
//!
//! This module defines all events used in OoT randomizer logic expressions and maps
//! persistent events to their memory flag locations in save data.
//!
//! # Event Categories
//!
//! Events are organized into the following categories:
//! - **Boss**: Boss defeat events (BOSS_GOHMA, BOSS_KING_DODONGO, etc.)
//! - **Story**: Main story progression events (DOOR_OF_TIME_OPEN, ZELDA_FLED, etc.)
//! - **Dungeon**: Dungeon-specific events (switches, room clears, etc.)
//! - **Overworld**: Overworld state changes (BOULDER_DEATH_MOUNTAIN, LAKE_HYLIA_WATER, etc.)
//! - **Volatile**: Runtime-computed events not stored in save data
//!
//! # Memory Locations
//!
//! Persistent events are stored in various parts of OoT save data:
//! - `event_chk_inf` (0x0ED4-0x0EF0): 14 u16 flags for main events
//! - `inf_table` (0x0EF8-0x0F34): 60 bytes for NPC/item interactions
//! - Scene flags (0x00D4-0x0E9C): Per-scene switches, chests, room clears

use byteorder::{BigEndian, ByteOrder};
use std::collections::HashMap;

/// Categories for OoT events.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum OotEventCategory {
    /// Boss defeat events
    Boss,
    /// Main story progression events
    Story,
    /// Dungeon-specific events (standard dungeons)
    Dungeon,
    /// Master Quest dungeon events
    DungeonMq,
    /// Overworld state changes
    Overworld,
    /// Volatile events computed at runtime
    Volatile,
}

/// Memory flag location for persistent events.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OotEventFlag {
    /// EventChkInf flag: (word_index, bit_mask)
    EventChkInf(u8, u16),
    /// InfTable flag: (byte_index, bit_mask)
    InfTable(u8, u8),
    /// Scene flag: (scene_id, flag_type, bit_mask)
    /// Flag types: 0=chests, 1=switches, 2=room_clear, 3=collectible
    SceneFlag(u8, u8, u32),
    /// Not stored in save data (volatile)
    Volatile,
}

/// All OoT events used in randomizer logic.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub enum OotEvent {
    // ============================================
    // Boss Events (8 total)
    // ============================================
    BOSS_GOHMA,
    BOSS_KING_DODONGO,
    BOSS_BARINADE,
    BOSS_PHANTOM_GANON,
    BOSS_VOLVAGIA,
    BOSS_MORPHA,
    BOSS_BONGO_BONGO,
    BOSS_TWINROVA,

    // ============================================
    // Story/Persistent Events
    // ============================================
    BRIDGE_OPEN,
    BOULDER_DEATH_MOUNTAIN,
    DARUNIA_TORCH,
    DOOR_OF_TIME_OPEN,
    EPONA,
    GORON_CITY_SHORTCUT,
    KAKARIKO_GATE_OPEN,
    MALON,
    MALON_COW,
    MIDO_MOVED,
    OPEN_FORTRESS_GATE,
    RED_BOULDER_BROKEN,
    RICHARD,
    SCARECROW_CHILD,
    TALON_AWAKE,
    TIME_TRAVEL,
    WELL_DRAIN,
    WINDMILL_TOP,
    WISP_CLEAR_STATE_LAKE,

    // ============================================
    // Deku Tree Events
    // ============================================
    DEKU_BURN_WEB,
    DEKU_MUD_WALL,
    DEKU_BLOCK,

    // ============================================
    // Deku Tree MQ Events
    // ============================================
    DEKU_MQ_BASEMENT_EYE_SWITCH,
    DEKU_MQ_BEFORE_COMPASS_EYE_SWITCH,
    DEKU_MQ_GRAVE_ROOM_WEBS,
    DEKU_MQ_MAIN_TORCH,
    DEKU_MQ_ROOM_AFTER_WATER_CLEAR,
    DEKU_MQ_ROOM_BEFORE_WATER_CLEAR,
    DEKU_MQ_SLINGSHOT_ENEMIES,
    DEKU_MQ_SLINGSHOT_TORCH,
    DEKU_MQ_WATER_PATH_TORCH1,
    DEKU_TREE_MQ_PRE_BOSS_SCRUB_PUZZLE,
    MQ_DEKU_WATER_TORCHES,

    // ============================================
    // Dodongo's Cavern Events
    // ============================================
    DC_BOMB_EYES,
    DC_MAIN_SWITCH,
    DC_SHORTCUT,

    // ============================================
    // Dodongo's Cavern MQ Events
    // ============================================
    DC_MQ_DEFEAT_DONDONGOS_AFTER_STAIRCASE,
    DC_MQ_SHORTCUT,
    DC_MQ_STAIRCASE,
    MQ_DC_BOSS_LOOP_FIRE_WALL_CRYSTAL_AFTER_FIRE,
    MQ_DC_BOSS_LOOP_FIRE_WALL_CRYSTAL_BEFORE_FIRE,
    MQ_DC_BOSS_SWITCH,
    MQ_DC_CLEAR_LARVAE_ROOM,
    MQ_DC_LOWER_LIZALFOS_CLEAR_FROM_LOWER,
    MQ_DC_LOWER_LIZALFOS_CLEAR_FROM_UPPER,
    MQ_DC_LOWER_TUNNEL_EYE_SWITCH,
    MQ_DC_OPEN_SKULL,
    MQ_DC_PILLAR_RAISE,
    MQ_DC_POE_ROOM_BOMB_SWITCHES,
    MQ_DC_ROOM_BEFORE_UPPER_LIZALFOS_GOLD_TORCH,
    MQ_DC_STAIRCASE_SWITCH,
    MQ_DC_UPPER_LIZALFOS_CLEAR_FROM_LOWER,
    MQ_DC_UPPER_LIZALFOS_CLEAR_FROM_UPPER,
    OPEN_MQ_DC_LARVAE_ROOM,

    // ============================================
    // Jabu-Jabu Events
    // ============================================
    BIG_OCTO,
    BLUE_PARASITE,
    GREEN_PARASITE,
    PARASITE,
    RED_PARASITE,

    // ============================================
    // Jabu-Jabu MQ Events
    // ============================================
    JABU_BIG_OCTO,
    JABU_MQ_BACK_CENTER_SWITCH,
    JABU_MQ_BACK_FIRE_WEB,
    JABU_MQ_BACK_UNLOCK,
    JABU_MQ_BASEMENT_SIDE_BUTTON,
    JABU_MQ_BASEMENT_SIDE_PLATFORM,
    JABU_MQ_END,
    JABU_MQ_LIKE_LIKE_ROOM_CLEAR,
    JABU_MQ_LOWER_BIG_OCTO_PLATFORM,
    JABU_MQ_MAIN_ELEVATOR_COMPASS_CHEST_COW_SWITCH,
    JABU_MQ_START,
    JABU_MQ_UNDERWATER_ALCOVE_SWITCH,
    JABU_MQ_WATER_SPOUTS,
    JABU_TENTACLE_BLUE,
    JABU_TENTACLE_GREEN,
    JABU_TENTACLE_RED,
    MQ_JABU_AFTER_ABOVE_BIG_OCTO_SPAWN_COW_AND_CRATES,

    // ============================================
    // Forest Temple Events
    // ============================================
    FOREST_LEDGE_REACHED,
    FOREST_POE_1,
    FOREST_POE_2,
    FOREST_POE_3,
    FOREST_POE_4,
    FOREST_TWISTED_HALL,
    FOREST_WELL,

    // ============================================
    // Forest Temple MQ Events
    // ============================================
    FOREST_TWIST_SWITCH,

    // ============================================
    // Fire Temple Events
    // ============================================
    FIRE_TEMPLE_PILLAR_HAMMER,

    // ============================================
    // Fire Temple MQ Events
    // ============================================
    FIRE_MQ_1ST_GORON_LIKELIKE,
    FIRE_MQ_3F_LAVA_ROOM_BLUE_SWITCH,
    FIRE_MQ_3F_LAVA_ROOM_TORCHES,
    FIRE_MQ_EAST_TOWER_TOP_HOOKSHOT_TARGETS,
    FIRE_MQ_FIRE_WALLS_MIDDLE_ROOM_RUSTY_SWITCH,
    FIRE_MQ_FLARE_DANCER_AFTER_FIRE_WALLS,
    FIRE_MQ_HAMMER_LOOP_FIRST_CLEAR,
    FIRE_MQ_HAMMER_LOOP_FLARE_DANCER_CLEAR,
    FIRE_MQ_HAMMER_LOOP_KNUCKLE_CLEAR,
    FIRE_MQ_HIGH_LEDGE_AFTER_STAIRCASE_PILLAR,
    FIRE_MQ_LAVA_BRIDGE_ROOM_HOOKSHOT_PLATFORMS,
    FIRE_MQ_LAVA_BRIDGE_ROOM_TORCHES,
    FIRE_MQ_MAP_CHEST_HAMMER_SWITCH,
    FIRE_MQ_MAZE_ROOM_BLUE_SWITCH,
    FIRE_MQ_MAZE_ROOM_LOWER_CAGE_SWITCH,
    FIRE_MQ_MAZE_ROOM_LOWER_RUSTY_SWITCH,
    FIRE_MQ_MAZE_SHORTCUT_SWITCH,
    FIRE_MQ_PRE_BOSS_PILLAR,
    FIRE_MQ_PRE_BOSS_TORCHES,
    FIRE_MQ_ROOM_BEFORE_MAZE_HOOKSHOT_TARGET,
    FIRE_MQ_STAIRCASE_LOWERED,
    FIRE_MQ_TOWER_AFTER_FLARE_DANCER_CRYSTAL_SWITCH_FROM_BELOW,
    FIRE_MQ_TOWER_TOP_HAMMER_SWITCH_BEFORE_STAIRCASE,

    // ============================================
    // Water Temple Events
    // ============================================
    LONGSHOT_TIME_BLOCK,
    WATER_LEVEL_LOW,
    WATER_LEVEL_MIDDLE,
    WATER_LEVEL_RESET,

    // ============================================
    // Water Temple MQ Events
    // ============================================
    MOVE_WATER_TIME_BLOCK,
    RUTO_COLUMN_HOOKSHOTS,
    WATER_CENTRAL_GATE,
    WATER_GATES,
    WATER_LEVEL_HIGH,
    WATER_LEVEL_MID,
    WATER_MQ_BRONZE_SCALE_SOFTLOCK_PREVENTION,
    WATER_MQ_CARRY_SMALL_CRATE,
    WATER_MQ_THREE_TORCH_ROOM_GATE,

    // ============================================
    // Spirit Temple Events
    // ============================================
    SPIRIT_ADULT_DOOR,
    SPIRIT_CHEST_CHILD,
    SPIRIT_CHILD_DOOR,
    SPIRIT_LIGHT_STATUE,

    // ============================================
    // Spirit Temple MQ Events
    // ============================================
    SPIRIT_LOBBY_BOULDERS,
    SPIRIT_PARADOX,
    SPIRIT_STATUE_FIRE,
    SPIRIT_TEMPLE_LIGHT,

    // ============================================
    // Shadow Temple Events
    // ============================================
    SHADOW_INVISIBLE_SCYTHE_GATE,
    SHADOW_PILLAR,
    SHADOW_SHORTCUT,

    // ============================================
    // Shadow Temple MQ Events
    // ============================================
    SHADOW_MQ_ACTIVATE_BOAT_RIDE,
    SHADOW_MQ_AFTER_BOAT_AFTER_BRIDGE_EYE_SWITCH,
    SHADOW_MQ_AFTER_BOAT_BRIDGE_FALL,
    SHADOW_MQ_AFTER_WIND_TUNNEL_GIBDOS_CLEAR,
    SHADOW_MQ_AREA_AFTER_BOAT_AFTER_BRIDGE_HIGH_LEDGE_SWITCH,
    SHADOW_MQ_CRUSHING_WALLS_BURNT,
    SHADOW_MQ_DUAL_STAIRCASE_STALFOS_CLEAR,
    SHADOW_MQ_FIRST_BEAMOS_FORK_GIBDOS_CLEAR,
    SHADOW_MQ_INVISIBLE_SPIKE_FLOORS_ICE_PLATFORMS,
    SHADOW_MQ_INVISIBLE_SPIKE_FLOORS_REDEADS_CLEAR,
    SHADOW_MQ_INVISIBLE_WALL_MAZE_DEAD_HAND_CLEAR,
    SHADOW_MQ_LOWER_HUGE_PIT_FALLING_SPIKES_GATE_SWITCH,
    SHADOW_MQ_SCYTHE_ROOM_SKULTULLAS_CLEAR,
    SHADOW_MQ_SHORTCUT_BLOCK_PUSHED,
    SHADOW_MQ_SPINNER_ROOM_ICE_PLATFORM,
    SHADOW_MQ_TEXTURED_MAZE_DEAD_HAND_ROOM_CLEAR,
    SHADOW_MQ_TEXTURED_MAZE_LOWER_ICE_BLOCK,
    SHADOW_MQ_TEXTURED_MAZE_SIDE_ROOM_CLEAR,
    SHADOW_MQ_TRUTH_SPINNER,
    SHADOW_MQ_UPPER_HUGE_PIT_FROZEN_EYE,
    SHADOW_MQ_WIND_TUNNEL_HINT_ROOM_REDEADS_CLEAR,

    // ============================================
    // Bottom of the Well Events
    // ============================================
    BOTW_WATER_DRAINED,

    // ============================================
    // Bottom of the Well MQ Events
    // ============================================
    BOTW_CENTER_EAST_TORCH,
    BOTW_CENTER_WEST_TORCH,
    BOTW_MQ_CENTER_GATES_LOWERED,
    BOTW_MQ_DRAIN_WATER,
    BOTW_MQ_EYE_GATE,
    BOTW_MQ_GATE_FROM_SWITCH,
    BOTW_MQ_REDEAD_CHEST,

    // ============================================
    // Ice Cavern MQ Events
    // ============================================
    ICE_MQ_FINAL_ROOM_CLEAR,
    ICE_MQ_FIRST_CRYSTAL,
    ICE_MQ_MAIN_ENEMIES_CLEAR,
    ICE_MQ_MAP_SWITCH,
    ICE_MQ_SECOND_CRYSTAL,

    // ============================================
    // Gerudo Training Ground Events
    // ============================================
    GTG_ICE_ARROWS_SWITCH,
    GTG_IRON_KNUCKLE,
    GTG_LAVA_HOOK_TARGETS,
    GTG_LEFT_SIDE,
    GTG_LIKE_LIKE_ROOM,
    GTG_RIGHT_SIDE,

    // ============================================
    // Gerudo Training Ground MQ Events
    // ============================================
    GTG_MQ_KNUCKLE_AND_SLUGS_CLEAR,
    GTG_MQ_LAVA_HAMMER_SWITCH,
    GTG_MQ_LAVA_TORCH_COMMON_PLATFORMS_ENTRANCE,
    GTG_MQ_LAVA_TORCH_ENTRANCE_SIDE,
    GTG_MQ_LAVA_TORCH_FAR,
    GTG_MQ_LAVA_TORCH_WATER_ROOM_SIDE,
    GTG_MQ_RIGHT_FIRST_CLEAR,
    GTG_MQ_SILVER_BLOCK_PUSH,
    GTG_MQ_SLOPES_STALAGMITES,
    GTG_MQ_SPINNING_STATUE_CRYSTAL,
    GTG_MQ_SPINNING_STATUE_EYES,
    GTG_MQ_STALFOS_CLEAR,
    GTG_MQ_STALFOS_SIDE_CLEAR,

    // ============================================
    // Ganon's Castle Events
    // ============================================
    GANON_START,
    GANON_TOWER_IRON_KNUCKLE_CLEAR,
    GANON_TOWER_LIZALFOS_CLEAR,
    GANON_TOWER_STALFOS_CLEAR,

    // ============================================
    // Ganon's Castle MQ Events
    // ============================================
    GANON_CASTLE_MQ_FOREST_WIND_FROZEN_EYE,
    GANON_CASTLE_MQ_FOREST_WIND_OPEN_EYE,
    GANON_MQ_FIRE_MONOLITH,
    GANON_MQ_FOREST_ENEMIES,
    GANON_MQ_FOREST_SWITCH,
    GANON_MQ_LIGHT_ENEMIES,
    GANON_MQ_SHADOW_BOMBFLOWER_FROM_ENTRANCE,
    GANON_MQ_SHADOW_EYE_CHEST,
    GANON_MQ_SHADOW_PROGRESSION_FROM_BEAMOS_1,
    GANON_MQ_SHADOW_PROGRESSION_FROM_ENTRANCE,
    GANON_MQ_SHADOW_TORCH_ICE_BLOCK,
    GANON_MQ_SPIRIT_CRYSTAL,
    GANON_MQ_SPIRIT_ENDING_SUNS,
    GANON_MQ_SPIRIT_HAMMER_SWITCH,
    GANON_MQ_SPIRIT_ZOMBIES,
    MQ_GANON_OPEN_MAIN,
    MQ_GANON_WATER_BF_SWITCH,
}

impl OotEvent {
    /// Parse an event name string to an OotEvent.
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
            Self::BOSS_GOHMA => "BOSS_GOHMA",
            Self::BOSS_KING_DODONGO => "BOSS_KING_DODONGO",
            Self::BOSS_BARINADE => "BOSS_BARINADE",
            Self::BOSS_PHANTOM_GANON => "BOSS_PHANTOM_GANON",
            Self::BOSS_VOLVAGIA => "BOSS_VOLVAGIA",
            Self::BOSS_MORPHA => "BOSS_MORPHA",
            Self::BOSS_BONGO_BONGO => "BOSS_BONGO_BONGO",
            Self::BOSS_TWINROVA => "BOSS_TWINROVA",
            // Story events
            Self::BRIDGE_OPEN => "BRIDGE_OPEN",
            Self::BOULDER_DEATH_MOUNTAIN => "BOULDER_DEATH_MOUNTAIN",
            Self::DARUNIA_TORCH => "DARUNIA_TORCH",
            Self::DOOR_OF_TIME_OPEN => "DOOR_OF_TIME_OPEN",
            Self::EPONA => "EPONA",
            Self::GORON_CITY_SHORTCUT => "GORON_CITY_SHORTCUT",
            Self::KAKARIKO_GATE_OPEN => "KAKARIKO_GATE_OPEN",
            Self::MALON => "MALON",
            Self::MALON_COW => "MALON_COW",
            Self::MIDO_MOVED => "MIDO_MOVED",
            Self::OPEN_FORTRESS_GATE => "OPEN_FORTRESS_GATE",
            Self::RED_BOULDER_BROKEN => "RED_BOULDER_BROKEN",
            Self::RICHARD => "RICHARD",
            Self::SCARECROW_CHILD => "SCARECROW_CHILD",
            Self::TALON_AWAKE => "TALON_AWAKE",
            Self::TIME_TRAVEL => "TIME_TRAVEL",
            Self::WELL_DRAIN => "WELL_DRAIN",
            Self::WINDMILL_TOP => "WINDMILL_TOP",
            Self::WISP_CLEAR_STATE_LAKE => "WISP_CLEAR_STATE_LAKE",
            // Deku Tree
            Self::DEKU_BURN_WEB => "DEKU_BURN_WEB",
            Self::DEKU_MUD_WALL => "DEKU_MUD_WALL",
            Self::DEKU_BLOCK => "DEKU_BLOCK",
            // Deku Tree MQ
            Self::DEKU_MQ_BASEMENT_EYE_SWITCH => "DEKU_MQ_BASEMENT_EYE_SWITCH",
            Self::DEKU_MQ_BEFORE_COMPASS_EYE_SWITCH => "DEKU_MQ_BEFORE_COMPASS_EYE_SWITCH",
            Self::DEKU_MQ_GRAVE_ROOM_WEBS => "DEKU_MQ_GRAVE_ROOM_WEBS",
            Self::DEKU_MQ_MAIN_TORCH => "DEKU_MQ_MAIN_TORCH",
            Self::DEKU_MQ_ROOM_AFTER_WATER_CLEAR => "DEKU_MQ_ROOM_AFTER_WATER_CLEAR",
            Self::DEKU_MQ_ROOM_BEFORE_WATER_CLEAR => "DEKU_MQ_ROOM_BEFORE_WATER_CLEAR",
            Self::DEKU_MQ_SLINGSHOT_ENEMIES => "DEKU_MQ_SLINGSHOT_ENEMIES",
            Self::DEKU_MQ_SLINGSHOT_TORCH => "DEKU_MQ_SLINGSHOT_TORCH",
            Self::DEKU_MQ_WATER_PATH_TORCH1 => "DEKU_MQ_WATER_PATH_TORCH1",
            Self::DEKU_TREE_MQ_PRE_BOSS_SCRUB_PUZZLE => "DEKU_TREE_MQ_PRE_BOSS_SCRUB_PUZZLE",
            Self::MQ_DEKU_WATER_TORCHES => "MQ_DEKU_WATER_TORCHES",
            // Dodongo's Cavern
            Self::DC_BOMB_EYES => "DC_BOMB_EYES",
            Self::DC_MAIN_SWITCH => "DC_MAIN_SWITCH",
            Self::DC_SHORTCUT => "DC_SHORTCUT",
            // Dodongo's Cavern MQ
            Self::DC_MQ_DEFEAT_DONDONGOS_AFTER_STAIRCASE => {
                "DC_MQ_DEFEAT_DONDONGOS_AFTER_STAIRCASE"
            }
            Self::DC_MQ_SHORTCUT => "DC_MQ_SHORTCUT",
            Self::DC_MQ_STAIRCASE => "DC_MQ_STAIRCASE",
            Self::MQ_DC_BOSS_LOOP_FIRE_WALL_CRYSTAL_AFTER_FIRE => {
                "MQ_DC_BOSS_LOOP_FIRE_WALL_CRYSTAL_AFTER_FIRE"
            }
            Self::MQ_DC_BOSS_LOOP_FIRE_WALL_CRYSTAL_BEFORE_FIRE => {
                "MQ_DC_BOSS_LOOP_FIRE_WALL_CRYSTAL_BEFORE_FIRE"
            }
            Self::MQ_DC_BOSS_SWITCH => "MQ_DC_BOSS_SWITCH",
            Self::MQ_DC_CLEAR_LARVAE_ROOM => "MQ_DC_CLEAR_LARVAE_ROOM",
            Self::MQ_DC_LOWER_LIZALFOS_CLEAR_FROM_LOWER => "MQ_DC_LOWER_LIZALFOS_CLEAR_FROM_LOWER",
            Self::MQ_DC_LOWER_LIZALFOS_CLEAR_FROM_UPPER => "MQ_DC_LOWER_LIZALFOS_CLEAR_FROM_UPPER",
            Self::MQ_DC_LOWER_TUNNEL_EYE_SWITCH => "MQ_DC_LOWER_TUNNEL_EYE_SWITCH",
            Self::MQ_DC_OPEN_SKULL => "MQ_DC_OPEN_SKULL",
            Self::MQ_DC_PILLAR_RAISE => "MQ_DC_PILLAR_RAISE",
            Self::MQ_DC_POE_ROOM_BOMB_SWITCHES => "MQ_DC_POE_ROOM_BOMB_SWITCHES",
            Self::MQ_DC_ROOM_BEFORE_UPPER_LIZALFOS_GOLD_TORCH => {
                "MQ_DC_ROOM_BEFORE_UPPER_LIZALFOS_GOLD_TORCH"
            }
            Self::MQ_DC_STAIRCASE_SWITCH => "MQ_DC_STAIRCASE_SWITCH",
            Self::MQ_DC_UPPER_LIZALFOS_CLEAR_FROM_LOWER => "MQ_DC_UPPER_LIZALFOS_CLEAR_FROM_LOWER",
            Self::MQ_DC_UPPER_LIZALFOS_CLEAR_FROM_UPPER => "MQ_DC_UPPER_LIZALFOS_CLEAR_FROM_UPPER",
            Self::OPEN_MQ_DC_LARVAE_ROOM => "OPEN_MQ_DC_LARVAE_ROOM",
            // Jabu-Jabu
            Self::BIG_OCTO => "BIG_OCTO",
            Self::BLUE_PARASITE => "BLUE_PARASITE",
            Self::GREEN_PARASITE => "GREEN_PARASITE",
            Self::PARASITE => "PARASITE",
            Self::RED_PARASITE => "RED_PARASITE",
            // Jabu-Jabu MQ
            Self::JABU_BIG_OCTO => "JABU_BIG_OCTO",
            Self::JABU_MQ_BACK_CENTER_SWITCH => "JABU_MQ_BACK_CENTER_SWITCH",
            Self::JABU_MQ_BACK_FIRE_WEB => "JABU_MQ_BACK_FIRE_WEB",
            Self::JABU_MQ_BACK_UNLOCK => "JABU_MQ_BACK_UNLOCK",
            Self::JABU_MQ_BASEMENT_SIDE_BUTTON => "JABU_MQ_BASEMENT_SIDE_BUTTON",
            Self::JABU_MQ_BASEMENT_SIDE_PLATFORM => "JABU_MQ_BASEMENT_SIDE_PLATFORM",
            Self::JABU_MQ_END => "JABU_MQ_END",
            Self::JABU_MQ_LIKE_LIKE_ROOM_CLEAR => "JABU_MQ_LIKE_LIKE_ROOM_CLEAR",
            Self::JABU_MQ_LOWER_BIG_OCTO_PLATFORM => "JABU_MQ_LOWER_BIG_OCTO_PLATFORM",
            Self::JABU_MQ_MAIN_ELEVATOR_COMPASS_CHEST_COW_SWITCH => {
                "JABU_MQ_MAIN_ELEVATOR_COMPASS_CHEST_COW_SWITCH"
            }
            Self::JABU_MQ_START => "JABU_MQ_START",
            Self::JABU_MQ_UNDERWATER_ALCOVE_SWITCH => "JABU_MQ_UNDERWATER_ALCOVE_SWITCH",
            Self::JABU_MQ_WATER_SPOUTS => "JABU_MQ_WATER_SPOUTS",
            Self::JABU_TENTACLE_BLUE => "JABU_TENTACLE_BLUE",
            Self::JABU_TENTACLE_GREEN => "JABU_TENTACLE_GREEN",
            Self::JABU_TENTACLE_RED => "JABU_TENTACLE_RED",
            Self::MQ_JABU_AFTER_ABOVE_BIG_OCTO_SPAWN_COW_AND_CRATES => {
                "MQ_JABU_AFTER_ABOVE_BIG_OCTO_SPAWN_COW_AND_CRATES"
            }
            // Forest Temple
            Self::FOREST_LEDGE_REACHED => "FOREST_LEDGE_REACHED",
            Self::FOREST_POE_1 => "FOREST_POE_1",
            Self::FOREST_POE_2 => "FOREST_POE_2",
            Self::FOREST_POE_3 => "FOREST_POE_3",
            Self::FOREST_POE_4 => "FOREST_POE_4",
            Self::FOREST_TWISTED_HALL => "FOREST_TWISTED_HALL",
            Self::FOREST_WELL => "FOREST_WELL",
            // Forest Temple MQ
            Self::FOREST_TWIST_SWITCH => "FOREST_TWIST_SWITCH",
            // Fire Temple
            Self::FIRE_TEMPLE_PILLAR_HAMMER => "FIRE_TEMPLE_PILLAR_HAMMER",
            // Fire Temple MQ
            Self::FIRE_MQ_1ST_GORON_LIKELIKE => "FIRE_MQ_1ST_GORON_LIKELIKE",
            Self::FIRE_MQ_3F_LAVA_ROOM_BLUE_SWITCH => "FIRE_MQ_3F_LAVA_ROOM_BLUE_SWITCH",
            Self::FIRE_MQ_3F_LAVA_ROOM_TORCHES => "FIRE_MQ_3F_LAVA_ROOM_TORCHES",
            Self::FIRE_MQ_EAST_TOWER_TOP_HOOKSHOT_TARGETS => {
                "FIRE_MQ_EAST_TOWER_TOP_HOOKSHOT_TARGETS"
            }
            Self::FIRE_MQ_FIRE_WALLS_MIDDLE_ROOM_RUSTY_SWITCH => {
                "FIRE_MQ_FIRE_WALLS_MIDDLE_ROOM_RUSTY_SWITCH"
            }
            Self::FIRE_MQ_FLARE_DANCER_AFTER_FIRE_WALLS => "FIRE_MQ_FLARE_DANCER_AFTER_FIRE_WALLS",
            Self::FIRE_MQ_HAMMER_LOOP_FIRST_CLEAR => "FIRE_MQ_HAMMER_LOOP_FIRST_CLEAR",
            Self::FIRE_MQ_HAMMER_LOOP_FLARE_DANCER_CLEAR => {
                "FIRE_MQ_HAMMER_LOOP_FLARE_DANCER_CLEAR"
            }
            Self::FIRE_MQ_HAMMER_LOOP_KNUCKLE_CLEAR => "FIRE_MQ_HAMMER_LOOP_KNUCKLE_CLEAR",
            Self::FIRE_MQ_HIGH_LEDGE_AFTER_STAIRCASE_PILLAR => {
                "FIRE_MQ_HIGH_LEDGE_AFTER_STAIRCASE_PILLAR"
            }
            Self::FIRE_MQ_LAVA_BRIDGE_ROOM_HOOKSHOT_PLATFORMS => {
                "FIRE_MQ_LAVA_BRIDGE_ROOM_HOOKSHOT_PLATFORMS"
            }
            Self::FIRE_MQ_LAVA_BRIDGE_ROOM_TORCHES => "FIRE_MQ_LAVA_BRIDGE_ROOM_TORCHES",
            Self::FIRE_MQ_MAP_CHEST_HAMMER_SWITCH => "FIRE_MQ_MAP_CHEST_HAMMER_SWITCH",
            Self::FIRE_MQ_MAZE_ROOM_BLUE_SWITCH => "FIRE_MQ_MAZE_ROOM_BLUE_SWITCH",
            Self::FIRE_MQ_MAZE_ROOM_LOWER_CAGE_SWITCH => "FIRE_MQ_MAZE_ROOM_LOWER_CAGE_SWITCH",
            Self::FIRE_MQ_MAZE_ROOM_LOWER_RUSTY_SWITCH => "FIRE_MQ_MAZE_ROOM_LOWER_RUSTY_SWITCH",
            Self::FIRE_MQ_MAZE_SHORTCUT_SWITCH => "FIRE_MQ_MAZE_SHORTCUT_SWITCH",
            Self::FIRE_MQ_PRE_BOSS_PILLAR => "FIRE_MQ_PRE_BOSS_PILLAR",
            Self::FIRE_MQ_PRE_BOSS_TORCHES => "FIRE_MQ_PRE_BOSS_TORCHES",
            Self::FIRE_MQ_ROOM_BEFORE_MAZE_HOOKSHOT_TARGET => {
                "FIRE_MQ_ROOM_BEFORE_MAZE_HOOKSHOT_TARGET"
            }
            Self::FIRE_MQ_STAIRCASE_LOWERED => "FIRE_MQ_STAIRCASE_LOWERED",
            Self::FIRE_MQ_TOWER_AFTER_FLARE_DANCER_CRYSTAL_SWITCH_FROM_BELOW => {
                "FIRE_MQ_TOWER_AFTER_FLARE_DANCER_CRYSTAL_SWITCH_FROM_BELOW"
            }
            Self::FIRE_MQ_TOWER_TOP_HAMMER_SWITCH_BEFORE_STAIRCASE => {
                "FIRE_MQ_TOWER_TOP_HAMMER_SWITCH_BEFORE_STAIRCASE"
            }
            // Water Temple
            Self::LONGSHOT_TIME_BLOCK => "LONGSHOT_TIME_BLOCK",
            Self::WATER_LEVEL_LOW => "WATER_LEVEL_LOW",
            Self::WATER_LEVEL_MIDDLE => "WATER_LEVEL_MIDDLE",
            Self::WATER_LEVEL_RESET => "WATER_LEVEL_RESET",
            // Water Temple MQ
            Self::MOVE_WATER_TIME_BLOCK => "MOVE_WATER_TIME_BLOCK",
            Self::RUTO_COLUMN_HOOKSHOTS => "RUTO_COLUMN_HOOKSHOTS",
            Self::WATER_CENTRAL_GATE => "WATER_CENTRAL_GATE",
            Self::WATER_GATES => "WATER_GATES",
            Self::WATER_LEVEL_HIGH => "WATER_LEVEL_HIGH",
            Self::WATER_LEVEL_MID => "WATER_LEVEL_MID",
            Self::WATER_MQ_BRONZE_SCALE_SOFTLOCK_PREVENTION => {
                "WATER_MQ_BRONZE_SCALE_SOFTLOCK_PREVENTION"
            }
            Self::WATER_MQ_CARRY_SMALL_CRATE => "WATER_MQ_CARRY_SMALL_CRATE",
            Self::WATER_MQ_THREE_TORCH_ROOM_GATE => "WATER_MQ_THREE_TORCH_ROOM_GATE",
            // Spirit Temple
            Self::SPIRIT_ADULT_DOOR => "SPIRIT_ADULT_DOOR",
            Self::SPIRIT_CHEST_CHILD => "SPIRIT_CHEST_CHILD",
            Self::SPIRIT_CHILD_DOOR => "SPIRIT_CHILD_DOOR",
            Self::SPIRIT_LIGHT_STATUE => "SPIRIT_LIGHT_STATUE",
            // Spirit Temple MQ
            Self::SPIRIT_LOBBY_BOULDERS => "SPIRIT_LOBBY_BOULDERS",
            Self::SPIRIT_PARADOX => "SPIRIT_PARADOX",
            Self::SPIRIT_STATUE_FIRE => "SPIRIT_STATUE_FIRE",
            Self::SPIRIT_TEMPLE_LIGHT => "SPIRIT_TEMPLE_LIGHT",
            // Shadow Temple
            Self::SHADOW_INVISIBLE_SCYTHE_GATE => "SHADOW_INVISIBLE_SCYTHE_GATE",
            Self::SHADOW_PILLAR => "SHADOW_PILLAR",
            Self::SHADOW_SHORTCUT => "SHADOW_SHORTCUT",
            // Shadow Temple MQ
            Self::SHADOW_MQ_ACTIVATE_BOAT_RIDE => "SHADOW_MQ_ACTIVATE_BOAT_RIDE",
            Self::SHADOW_MQ_AFTER_BOAT_AFTER_BRIDGE_EYE_SWITCH => {
                "SHADOW_MQ_AFTER_BOAT_AFTER_BRIDGE_EYE_SWITCH"
            }
            Self::SHADOW_MQ_AFTER_BOAT_BRIDGE_FALL => "SHADOW_MQ_AFTER_BOAT_BRIDGE_FALL",
            Self::SHADOW_MQ_AFTER_WIND_TUNNEL_GIBDOS_CLEAR => {
                "SHADOW_MQ_AFTER_WIND_TUNNEL_GIBDOS_CLEAR"
            }
            Self::SHADOW_MQ_AREA_AFTER_BOAT_AFTER_BRIDGE_HIGH_LEDGE_SWITCH => {
                "SHADOW_MQ_AREA_AFTER_BOAT_AFTER_BRIDGE_HIGH_LEDGE_SWITCH"
            }
            Self::SHADOW_MQ_CRUSHING_WALLS_BURNT => "SHADOW_MQ_CRUSHING_WALLS_BURNT",
            Self::SHADOW_MQ_DUAL_STAIRCASE_STALFOS_CLEAR => {
                "SHADOW_MQ_DUAL_STAIRCASE_STALFOS_CLEAR"
            }
            Self::SHADOW_MQ_FIRST_BEAMOS_FORK_GIBDOS_CLEAR => {
                "SHADOW_MQ_FIRST_BEAMOS_FORK_GIBDOS_CLEAR"
            }
            Self::SHADOW_MQ_INVISIBLE_SPIKE_FLOORS_ICE_PLATFORMS => {
                "SHADOW_MQ_INVISIBLE_SPIKE_FLOORS_ICE_PLATFORMS"
            }
            Self::SHADOW_MQ_INVISIBLE_SPIKE_FLOORS_REDEADS_CLEAR => {
                "SHADOW_MQ_INVISIBLE_SPIKE_FLOORS_REDEADS_CLEAR"
            }
            Self::SHADOW_MQ_INVISIBLE_WALL_MAZE_DEAD_HAND_CLEAR => {
                "SHADOW_MQ_INVISIBLE_WALL_MAZE_DEAD_HAND_CLEAR"
            }
            Self::SHADOW_MQ_LOWER_HUGE_PIT_FALLING_SPIKES_GATE_SWITCH => {
                "SHADOW_MQ_LOWER_HUGE_PIT_FALLING_SPIKES_GATE_SWITCH"
            }
            Self::SHADOW_MQ_SCYTHE_ROOM_SKULTULLAS_CLEAR => {
                "SHADOW_MQ_SCYTHE_ROOM_SKULTULLAS_CLEAR"
            }
            Self::SHADOW_MQ_SHORTCUT_BLOCK_PUSHED => "SHADOW_MQ_SHORTCUT_BLOCK_PUSHED",
            Self::SHADOW_MQ_SPINNER_ROOM_ICE_PLATFORM => "SHADOW_MQ_SPINNER_ROOM_ICE_PLATFORM",
            Self::SHADOW_MQ_TEXTURED_MAZE_DEAD_HAND_ROOM_CLEAR => {
                "SHADOW_MQ_TEXTURED_MAZE_DEAD_HAND_ROOM_CLEAR"
            }
            Self::SHADOW_MQ_TEXTURED_MAZE_LOWER_ICE_BLOCK => {
                "SHADOW_MQ_TEXTURED_MAZE_LOWER_ICE_BLOCK"
            }
            Self::SHADOW_MQ_TEXTURED_MAZE_SIDE_ROOM_CLEAR => {
                "SHADOW_MQ_TEXTURED_MAZE_SIDE_ROOM_CLEAR"
            }
            Self::SHADOW_MQ_TRUTH_SPINNER => "SHADOW_MQ_TRUTH_SPINNER",
            Self::SHADOW_MQ_UPPER_HUGE_PIT_FROZEN_EYE => "SHADOW_MQ_UPPER_HUGE_PIT_FROZEN_EYE",
            Self::SHADOW_MQ_WIND_TUNNEL_HINT_ROOM_REDEADS_CLEAR => {
                "SHADOW_MQ_WIND_TUNNEL_HINT_ROOM_REDEADS_CLEAR"
            }
            // Bottom of the Well
            Self::BOTW_WATER_DRAINED => "BOTW_WATER_DRAINED",
            // Bottom of the Well MQ
            Self::BOTW_CENTER_EAST_TORCH => "BOTW_CENTER_EAST_TORCH",
            Self::BOTW_CENTER_WEST_TORCH => "BOTW_CENTER_WEST_TORCH",
            Self::BOTW_MQ_CENTER_GATES_LOWERED => "BOTW_MQ_CENTER_GATES_LOWERED",
            Self::BOTW_MQ_DRAIN_WATER => "BOTW_MQ_DRAIN_WATER",
            Self::BOTW_MQ_EYE_GATE => "BOTW_MQ_EYE_GATE",
            Self::BOTW_MQ_GATE_FROM_SWITCH => "BOTW_MQ_GATE_FROM_SWITCH",
            Self::BOTW_MQ_REDEAD_CHEST => "BOTW_MQ_REDEAD_CHEST",
            // Ice Cavern MQ
            Self::ICE_MQ_FINAL_ROOM_CLEAR => "ICE_MQ_FINAL_ROOM_CLEAR",
            Self::ICE_MQ_FIRST_CRYSTAL => "ICE_MQ_FIRST_CRYSTAL",
            Self::ICE_MQ_MAIN_ENEMIES_CLEAR => "ICE_MQ_MAIN_ENEMIES_CLEAR",
            Self::ICE_MQ_MAP_SWITCH => "ICE_MQ_MAP_SWITCH",
            Self::ICE_MQ_SECOND_CRYSTAL => "ICE_MQ_SECOND_CRYSTAL",
            // Gerudo Training Ground
            Self::GTG_ICE_ARROWS_SWITCH => "GTG_ICE_ARROWS_SWITCH",
            Self::GTG_IRON_KNUCKLE => "GTG_IRON_KNUCKLE",
            Self::GTG_LAVA_HOOK_TARGETS => "GTG_LAVA_HOOK_TARGETS",
            Self::GTG_LEFT_SIDE => "GTG_LEFT_SIDE",
            Self::GTG_LIKE_LIKE_ROOM => "GTG_LIKE_LIKE_ROOM",
            Self::GTG_RIGHT_SIDE => "GTG_RIGHT_SIDE",
            // Gerudo Training Ground MQ
            Self::GTG_MQ_KNUCKLE_AND_SLUGS_CLEAR => "GTG_MQ_KNUCKLE_AND_SLUGS_CLEAR",
            Self::GTG_MQ_LAVA_HAMMER_SWITCH => "GTG_MQ_LAVA_HAMMER_SWITCH",
            Self::GTG_MQ_LAVA_TORCH_COMMON_PLATFORMS_ENTRANCE => {
                "GTG_MQ_LAVA_TORCH_COMMON_PLATFORMS_ENTRANCE"
            }
            Self::GTG_MQ_LAVA_TORCH_ENTRANCE_SIDE => "GTG_MQ_LAVA_TORCH_ENTRANCE_SIDE",
            Self::GTG_MQ_LAVA_TORCH_FAR => "GTG_MQ_LAVA_TORCH_FAR",
            Self::GTG_MQ_LAVA_TORCH_WATER_ROOM_SIDE => "GTG_MQ_LAVA_TORCH_WATER_ROOM_SIDE",
            Self::GTG_MQ_RIGHT_FIRST_CLEAR => "GTG_MQ_RIGHT_FIRST_CLEAR",
            Self::GTG_MQ_SILVER_BLOCK_PUSH => "GTG_MQ_SILVER_BLOCK_PUSH",
            Self::GTG_MQ_SLOPES_STALAGMITES => "GTG_MQ_SLOPES_STALAGMITES",
            Self::GTG_MQ_SPINNING_STATUE_CRYSTAL => "GTG_MQ_SPINNING_STATUE_CRYSTAL",
            Self::GTG_MQ_SPINNING_STATUE_EYES => "GTG_MQ_SPINNING_STATUE_EYES",
            Self::GTG_MQ_STALFOS_CLEAR => "GTG_MQ_STALFOS_CLEAR",
            Self::GTG_MQ_STALFOS_SIDE_CLEAR => "GTG_MQ_STALFOS_SIDE_CLEAR",
            // Ganon's Castle
            Self::GANON_START => "GANON_START",
            Self::GANON_TOWER_IRON_KNUCKLE_CLEAR => "GANON_TOWER_IRON_KNUCKLE_CLEAR",
            Self::GANON_TOWER_LIZALFOS_CLEAR => "GANON_TOWER_LIZALFOS_CLEAR",
            Self::GANON_TOWER_STALFOS_CLEAR => "GANON_TOWER_STALFOS_CLEAR",
            // Ganon's Castle MQ
            Self::GANON_CASTLE_MQ_FOREST_WIND_FROZEN_EYE => {
                "GANON_CASTLE_MQ_FOREST_WIND_FROZEN_EYE"
            }
            Self::GANON_CASTLE_MQ_FOREST_WIND_OPEN_EYE => "GANON_CASTLE_MQ_FOREST_WIND_OPEN_EYE",
            Self::GANON_MQ_FIRE_MONOLITH => "GANON_MQ_FIRE_MONOLITH",
            Self::GANON_MQ_FOREST_ENEMIES => "GANON_MQ_FOREST_ENEMIES",
            Self::GANON_MQ_FOREST_SWITCH => "GANON_MQ_FOREST_SWITCH",
            Self::GANON_MQ_LIGHT_ENEMIES => "GANON_MQ_LIGHT_ENEMIES",
            Self::GANON_MQ_SHADOW_BOMBFLOWER_FROM_ENTRANCE => {
                "GANON_MQ_SHADOW_BOMBFLOWER_FROM_ENTRANCE"
            }
            Self::GANON_MQ_SHADOW_EYE_CHEST => "GANON_MQ_SHADOW_EYE_CHEST",
            Self::GANON_MQ_SHADOW_PROGRESSION_FROM_BEAMOS_1 => {
                "GANON_MQ_SHADOW_PROGRESSION_FROM_BEAMOS_1"
            }
            Self::GANON_MQ_SHADOW_PROGRESSION_FROM_ENTRANCE => {
                "GANON_MQ_SHADOW_PROGRESSION_FROM_ENTRANCE"
            }
            Self::GANON_MQ_SHADOW_TORCH_ICE_BLOCK => "GANON_MQ_SHADOW_TORCH_ICE_BLOCK",
            Self::GANON_MQ_SPIRIT_CRYSTAL => "GANON_MQ_SPIRIT_CRYSTAL",
            Self::GANON_MQ_SPIRIT_ENDING_SUNS => "GANON_MQ_SPIRIT_ENDING_SUNS",
            Self::GANON_MQ_SPIRIT_HAMMER_SWITCH => "GANON_MQ_SPIRIT_HAMMER_SWITCH",
            Self::GANON_MQ_SPIRIT_ZOMBIES => "GANON_MQ_SPIRIT_ZOMBIES",
            Self::MQ_GANON_OPEN_MAIN => "MQ_GANON_OPEN_MAIN",
            Self::MQ_GANON_WATER_BF_SWITCH => "MQ_GANON_WATER_BF_SWITCH",
        }
    }

    /// Get the category of this event.
    #[must_use]
    pub fn category(&self) -> OotEventCategory {
        match self {
            // Boss events
            Self::BOSS_GOHMA
            | Self::BOSS_KING_DODONGO
            | Self::BOSS_BARINADE
            | Self::BOSS_PHANTOM_GANON
            | Self::BOSS_VOLVAGIA
            | Self::BOSS_MORPHA
            | Self::BOSS_BONGO_BONGO
            | Self::BOSS_TWINROVA => OotEventCategory::Boss,

            // Story/overworld events
            Self::BRIDGE_OPEN
            | Self::BOULDER_DEATH_MOUNTAIN
            | Self::DOOR_OF_TIME_OPEN
            | Self::EPONA
            | Self::GORON_CITY_SHORTCUT
            | Self::KAKARIKO_GATE_OPEN
            | Self::MALON
            | Self::MALON_COW
            | Self::MIDO_MOVED
            | Self::OPEN_FORTRESS_GATE
            | Self::RED_BOULDER_BROKEN
            | Self::RICHARD
            | Self::SCARECROW_CHILD
            | Self::TALON_AWAKE
            | Self::TIME_TRAVEL
            | Self::WELL_DRAIN
            | Self::WINDMILL_TOP
            | Self::WISP_CLEAR_STATE_LAKE => OotEventCategory::Story,

            // Dungeon MQ events
            Self::DEKU_MQ_BASEMENT_EYE_SWITCH
            | Self::DEKU_MQ_BEFORE_COMPASS_EYE_SWITCH
            | Self::DEKU_MQ_GRAVE_ROOM_WEBS
            | Self::DEKU_MQ_MAIN_TORCH
            | Self::DEKU_MQ_ROOM_AFTER_WATER_CLEAR
            | Self::DEKU_MQ_ROOM_BEFORE_WATER_CLEAR
            | Self::DEKU_MQ_SLINGSHOT_ENEMIES
            | Self::DEKU_MQ_SLINGSHOT_TORCH
            | Self::DEKU_MQ_WATER_PATH_TORCH1
            | Self::DEKU_TREE_MQ_PRE_BOSS_SCRUB_PUZZLE
            | Self::MQ_DEKU_WATER_TORCHES
            | Self::DC_MQ_DEFEAT_DONDONGOS_AFTER_STAIRCASE
            | Self::DC_MQ_SHORTCUT
            | Self::DC_MQ_STAIRCASE
            | Self::MQ_DC_BOSS_LOOP_FIRE_WALL_CRYSTAL_AFTER_FIRE
            | Self::MQ_DC_BOSS_LOOP_FIRE_WALL_CRYSTAL_BEFORE_FIRE
            | Self::MQ_DC_BOSS_SWITCH
            | Self::MQ_DC_CLEAR_LARVAE_ROOM
            | Self::MQ_DC_LOWER_LIZALFOS_CLEAR_FROM_LOWER
            | Self::MQ_DC_LOWER_LIZALFOS_CLEAR_FROM_UPPER
            | Self::MQ_DC_LOWER_TUNNEL_EYE_SWITCH
            | Self::MQ_DC_OPEN_SKULL
            | Self::MQ_DC_PILLAR_RAISE
            | Self::MQ_DC_POE_ROOM_BOMB_SWITCHES
            | Self::MQ_DC_ROOM_BEFORE_UPPER_LIZALFOS_GOLD_TORCH
            | Self::MQ_DC_STAIRCASE_SWITCH
            | Self::MQ_DC_UPPER_LIZALFOS_CLEAR_FROM_LOWER
            | Self::MQ_DC_UPPER_LIZALFOS_CLEAR_FROM_UPPER
            | Self::OPEN_MQ_DC_LARVAE_ROOM
            | Self::JABU_MQ_BACK_CENTER_SWITCH
            | Self::JABU_MQ_BACK_FIRE_WEB
            | Self::JABU_MQ_BACK_UNLOCK
            | Self::JABU_MQ_BASEMENT_SIDE_BUTTON
            | Self::JABU_MQ_BASEMENT_SIDE_PLATFORM
            | Self::JABU_MQ_END
            | Self::JABU_MQ_LIKE_LIKE_ROOM_CLEAR
            | Self::JABU_MQ_LOWER_BIG_OCTO_PLATFORM
            | Self::JABU_MQ_MAIN_ELEVATOR_COMPASS_CHEST_COW_SWITCH
            | Self::JABU_MQ_START
            | Self::JABU_MQ_UNDERWATER_ALCOVE_SWITCH
            | Self::JABU_MQ_WATER_SPOUTS
            | Self::MQ_JABU_AFTER_ABOVE_BIG_OCTO_SPAWN_COW_AND_CRATES
            | Self::FOREST_TWIST_SWITCH
            | Self::FIRE_MQ_1ST_GORON_LIKELIKE
            | Self::FIRE_MQ_3F_LAVA_ROOM_BLUE_SWITCH
            | Self::FIRE_MQ_3F_LAVA_ROOM_TORCHES
            | Self::FIRE_MQ_EAST_TOWER_TOP_HOOKSHOT_TARGETS
            | Self::FIRE_MQ_FIRE_WALLS_MIDDLE_ROOM_RUSTY_SWITCH
            | Self::FIRE_MQ_FLARE_DANCER_AFTER_FIRE_WALLS
            | Self::FIRE_MQ_HAMMER_LOOP_FIRST_CLEAR
            | Self::FIRE_MQ_HAMMER_LOOP_FLARE_DANCER_CLEAR
            | Self::FIRE_MQ_HAMMER_LOOP_KNUCKLE_CLEAR
            | Self::FIRE_MQ_HIGH_LEDGE_AFTER_STAIRCASE_PILLAR
            | Self::FIRE_MQ_LAVA_BRIDGE_ROOM_HOOKSHOT_PLATFORMS
            | Self::FIRE_MQ_LAVA_BRIDGE_ROOM_TORCHES
            | Self::FIRE_MQ_MAP_CHEST_HAMMER_SWITCH
            | Self::FIRE_MQ_MAZE_ROOM_BLUE_SWITCH
            | Self::FIRE_MQ_MAZE_ROOM_LOWER_CAGE_SWITCH
            | Self::FIRE_MQ_MAZE_ROOM_LOWER_RUSTY_SWITCH
            | Self::FIRE_MQ_MAZE_SHORTCUT_SWITCH
            | Self::FIRE_MQ_PRE_BOSS_PILLAR
            | Self::FIRE_MQ_PRE_BOSS_TORCHES
            | Self::FIRE_MQ_ROOM_BEFORE_MAZE_HOOKSHOT_TARGET
            | Self::FIRE_MQ_STAIRCASE_LOWERED
            | Self::FIRE_MQ_TOWER_AFTER_FLARE_DANCER_CRYSTAL_SWITCH_FROM_BELOW
            | Self::FIRE_MQ_TOWER_TOP_HAMMER_SWITCH_BEFORE_STAIRCASE
            | Self::MOVE_WATER_TIME_BLOCK
            | Self::RUTO_COLUMN_HOOKSHOTS
            | Self::WATER_CENTRAL_GATE
            | Self::WATER_GATES
            | Self::WATER_LEVEL_HIGH
            | Self::WATER_LEVEL_MID
            | Self::WATER_MQ_BRONZE_SCALE_SOFTLOCK_PREVENTION
            | Self::WATER_MQ_CARRY_SMALL_CRATE
            | Self::WATER_MQ_THREE_TORCH_ROOM_GATE
            | Self::SPIRIT_LOBBY_BOULDERS
            | Self::SPIRIT_PARADOX
            | Self::SPIRIT_STATUE_FIRE
            | Self::SPIRIT_TEMPLE_LIGHT
            | Self::SHADOW_MQ_ACTIVATE_BOAT_RIDE
            | Self::SHADOW_MQ_AFTER_BOAT_AFTER_BRIDGE_EYE_SWITCH
            | Self::SHADOW_MQ_AFTER_BOAT_BRIDGE_FALL
            | Self::SHADOW_MQ_AFTER_WIND_TUNNEL_GIBDOS_CLEAR
            | Self::SHADOW_MQ_AREA_AFTER_BOAT_AFTER_BRIDGE_HIGH_LEDGE_SWITCH
            | Self::SHADOW_MQ_CRUSHING_WALLS_BURNT
            | Self::SHADOW_MQ_DUAL_STAIRCASE_STALFOS_CLEAR
            | Self::SHADOW_MQ_FIRST_BEAMOS_FORK_GIBDOS_CLEAR
            | Self::SHADOW_MQ_INVISIBLE_SPIKE_FLOORS_ICE_PLATFORMS
            | Self::SHADOW_MQ_INVISIBLE_SPIKE_FLOORS_REDEADS_CLEAR
            | Self::SHADOW_MQ_INVISIBLE_WALL_MAZE_DEAD_HAND_CLEAR
            | Self::SHADOW_MQ_LOWER_HUGE_PIT_FALLING_SPIKES_GATE_SWITCH
            | Self::SHADOW_MQ_SCYTHE_ROOM_SKULTULLAS_CLEAR
            | Self::SHADOW_MQ_SHORTCUT_BLOCK_PUSHED
            | Self::SHADOW_MQ_SPINNER_ROOM_ICE_PLATFORM
            | Self::SHADOW_MQ_TEXTURED_MAZE_DEAD_HAND_ROOM_CLEAR
            | Self::SHADOW_MQ_TEXTURED_MAZE_LOWER_ICE_BLOCK
            | Self::SHADOW_MQ_TEXTURED_MAZE_SIDE_ROOM_CLEAR
            | Self::SHADOW_MQ_TRUTH_SPINNER
            | Self::SHADOW_MQ_UPPER_HUGE_PIT_FROZEN_EYE
            | Self::SHADOW_MQ_WIND_TUNNEL_HINT_ROOM_REDEADS_CLEAR
            | Self::BOTW_CENTER_EAST_TORCH
            | Self::BOTW_CENTER_WEST_TORCH
            | Self::BOTW_MQ_CENTER_GATES_LOWERED
            | Self::BOTW_MQ_DRAIN_WATER
            | Self::BOTW_MQ_EYE_GATE
            | Self::BOTW_MQ_GATE_FROM_SWITCH
            | Self::BOTW_MQ_REDEAD_CHEST
            | Self::ICE_MQ_FINAL_ROOM_CLEAR
            | Self::ICE_MQ_FIRST_CRYSTAL
            | Self::ICE_MQ_MAIN_ENEMIES_CLEAR
            | Self::ICE_MQ_MAP_SWITCH
            | Self::ICE_MQ_SECOND_CRYSTAL
            | Self::GTG_MQ_KNUCKLE_AND_SLUGS_CLEAR
            | Self::GTG_MQ_LAVA_HAMMER_SWITCH
            | Self::GTG_MQ_LAVA_TORCH_COMMON_PLATFORMS_ENTRANCE
            | Self::GTG_MQ_LAVA_TORCH_ENTRANCE_SIDE
            | Self::GTG_MQ_LAVA_TORCH_FAR
            | Self::GTG_MQ_LAVA_TORCH_WATER_ROOM_SIDE
            | Self::GTG_MQ_RIGHT_FIRST_CLEAR
            | Self::GTG_MQ_SILVER_BLOCK_PUSH
            | Self::GTG_MQ_SLOPES_STALAGMITES
            | Self::GTG_MQ_SPINNING_STATUE_CRYSTAL
            | Self::GTG_MQ_SPINNING_STATUE_EYES
            | Self::GTG_MQ_STALFOS_CLEAR
            | Self::GTG_MQ_STALFOS_SIDE_CLEAR
            | Self::GANON_CASTLE_MQ_FOREST_WIND_FROZEN_EYE
            | Self::GANON_CASTLE_MQ_FOREST_WIND_OPEN_EYE
            | Self::GANON_MQ_FIRE_MONOLITH
            | Self::GANON_MQ_FOREST_ENEMIES
            | Self::GANON_MQ_FOREST_SWITCH
            | Self::GANON_MQ_LIGHT_ENEMIES
            | Self::GANON_MQ_SHADOW_BOMBFLOWER_FROM_ENTRANCE
            | Self::GANON_MQ_SHADOW_EYE_CHEST
            | Self::GANON_MQ_SHADOW_PROGRESSION_FROM_BEAMOS_1
            | Self::GANON_MQ_SHADOW_PROGRESSION_FROM_ENTRANCE
            | Self::GANON_MQ_SHADOW_TORCH_ICE_BLOCK
            | Self::GANON_MQ_SPIRIT_CRYSTAL
            | Self::GANON_MQ_SPIRIT_ENDING_SUNS
            | Self::GANON_MQ_SPIRIT_HAMMER_SWITCH
            | Self::GANON_MQ_SPIRIT_ZOMBIES
            | Self::MQ_GANON_OPEN_MAIN
            | Self::MQ_GANON_WATER_BF_SWITCH => OotEventCategory::DungeonMq,

            // All other events are dungeon or volatile events
            _ => OotEventCategory::Dungeon,
        }
    }

    /// Get the memory flag location for this event.
    ///
    /// Returns the flag location if the event is persistent (stored in save data),
    /// or `OotEventFlag::Volatile` if the event is computed at runtime.
    #[must_use]
    pub fn flag(&self) -> OotEventFlag {
        match self {
            // ============================================
            // Boss Events - EventChkInf mappings
            // ============================================
            // Boss defeats are stored in EventChkInf
            Self::BOSS_GOHMA => OotEventFlag::EventChkInf(0, 0x0080), // "Deku Tree Clear"
            Self::BOSS_KING_DODONGO => OotEventFlag::EventChkInf(2, 0x0020),
            Self::BOSS_BARINADE => OotEventFlag::EventChkInf(3, 0x0080),
            Self::BOSS_PHANTOM_GANON => OotEventFlag::EventChkInf(4, 0x0100),
            Self::BOSS_VOLVAGIA => OotEventFlag::EventChkInf(4, 0x0200),
            Self::BOSS_MORPHA => OotEventFlag::EventChkInf(4, 0x0400),
            Self::BOSS_BONGO_BONGO => OotEventFlag::EventChkInf(10, 0x0001),
            Self::BOSS_TWINROVA => OotEventFlag::EventChkInf(10, 0x0002),

            // ============================================
            // Story Events - EventChkInf/InfTable mappings
            // ============================================
            Self::MIDO_MOVED => OotEventFlag::EventChkInf(0, 0x0010), // "Showed Mido Sword & Shield"
            Self::WELL_DRAIN => OotEventFlag::EventChkInf(6, 0x0080), // "Drain Well"
            Self::KAKARIKO_GATE_OPEN => OotEventFlag::InfTable(15, 0x40),

            // ============================================
            // Water Temple water level - Scene flags
            // ============================================
            // Water level is stored in Water Temple scene (0x05) switches
            // Note: These may need adjustment based on actual memory layout
            Self::WATER_LEVEL_LOW => OotEventFlag::SceneFlag(0x05, 1, 0x1000_0000),
            Self::WATER_LEVEL_MIDDLE => OotEventFlag::SceneFlag(0x05, 1, 0x2000_0000),
            Self::WATER_LEVEL_RESET => OotEventFlag::SceneFlag(0x05, 1, 0x4000_0000),

            // ============================================
            // Volatile Events - Not stored in save data
            // ============================================
            // Most dungeon events are volatile (computed at runtime)
            _ => OotEventFlag::Volatile,
        }
    }

    /// Check if this event is persistent (stored in save data).
    #[must_use]
    pub fn is_persistent(&self) -> bool {
        !matches!(self.flag(), OotEventFlag::Volatile)
    }

    /// Get all OoT events.
    #[must_use]
    pub fn all() -> &'static [OotEvent] {
        ALL_EVENTS
    }

    /// Get all persistent events (stored in save data).
    pub fn all_persistent() -> impl Iterator<Item = OotEvent> {
        ALL_EVENTS.iter().copied().filter(|e| e.is_persistent())
    }

    /// Get all volatile events (computed at runtime).
    pub fn all_volatile() -> impl Iterator<Item = OotEvent> {
        ALL_EVENTS.iter().copied().filter(|e| !e.is_persistent())
    }

    /// Get all events in a specific category.
    pub fn by_category(category: OotEventCategory) -> impl Iterator<Item = OotEvent> {
        ALL_EVENTS
            .iter()
            .copied()
            .filter(move |e| e.category() == category)
    }
}

impl std::fmt::Display for OotEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name())
    }
}

impl std::str::FromStr for OotEvent {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        OotEvent::from_name(s).ok_or_else(|| format!("Unknown OoT event: {}", s))
    }
}

// Static list of all events for iteration
static ALL_EVENTS: &[OotEvent] = &[
    // Boss events
    OotEvent::BOSS_GOHMA,
    OotEvent::BOSS_KING_DODONGO,
    OotEvent::BOSS_BARINADE,
    OotEvent::BOSS_PHANTOM_GANON,
    OotEvent::BOSS_VOLVAGIA,
    OotEvent::BOSS_MORPHA,
    OotEvent::BOSS_BONGO_BONGO,
    OotEvent::BOSS_TWINROVA,
    // Story events
    OotEvent::BRIDGE_OPEN,
    OotEvent::BOULDER_DEATH_MOUNTAIN,
    OotEvent::DARUNIA_TORCH,
    OotEvent::DOOR_OF_TIME_OPEN,
    OotEvent::EPONA,
    OotEvent::GORON_CITY_SHORTCUT,
    OotEvent::KAKARIKO_GATE_OPEN,
    OotEvent::MALON,
    OotEvent::MALON_COW,
    OotEvent::MIDO_MOVED,
    OotEvent::OPEN_FORTRESS_GATE,
    OotEvent::RED_BOULDER_BROKEN,
    OotEvent::RICHARD,
    OotEvent::SCARECROW_CHILD,
    OotEvent::TALON_AWAKE,
    OotEvent::TIME_TRAVEL,
    OotEvent::WELL_DRAIN,
    OotEvent::WINDMILL_TOP,
    OotEvent::WISP_CLEAR_STATE_LAKE,
    // Deku Tree
    OotEvent::DEKU_BURN_WEB,
    OotEvent::DEKU_MUD_WALL,
    OotEvent::DEKU_BLOCK,
    // Deku Tree MQ
    OotEvent::DEKU_MQ_BASEMENT_EYE_SWITCH,
    OotEvent::DEKU_MQ_BEFORE_COMPASS_EYE_SWITCH,
    OotEvent::DEKU_MQ_GRAVE_ROOM_WEBS,
    OotEvent::DEKU_MQ_MAIN_TORCH,
    OotEvent::DEKU_MQ_ROOM_AFTER_WATER_CLEAR,
    OotEvent::DEKU_MQ_ROOM_BEFORE_WATER_CLEAR,
    OotEvent::DEKU_MQ_SLINGSHOT_ENEMIES,
    OotEvent::DEKU_MQ_SLINGSHOT_TORCH,
    OotEvent::DEKU_MQ_WATER_PATH_TORCH1,
    OotEvent::DEKU_TREE_MQ_PRE_BOSS_SCRUB_PUZZLE,
    OotEvent::MQ_DEKU_WATER_TORCHES,
    // Dodongo's Cavern
    OotEvent::DC_BOMB_EYES,
    OotEvent::DC_MAIN_SWITCH,
    OotEvent::DC_SHORTCUT,
    // Dodongo's Cavern MQ
    OotEvent::DC_MQ_DEFEAT_DONDONGOS_AFTER_STAIRCASE,
    OotEvent::DC_MQ_SHORTCUT,
    OotEvent::DC_MQ_STAIRCASE,
    OotEvent::MQ_DC_BOSS_LOOP_FIRE_WALL_CRYSTAL_AFTER_FIRE,
    OotEvent::MQ_DC_BOSS_LOOP_FIRE_WALL_CRYSTAL_BEFORE_FIRE,
    OotEvent::MQ_DC_BOSS_SWITCH,
    OotEvent::MQ_DC_CLEAR_LARVAE_ROOM,
    OotEvent::MQ_DC_LOWER_LIZALFOS_CLEAR_FROM_LOWER,
    OotEvent::MQ_DC_LOWER_LIZALFOS_CLEAR_FROM_UPPER,
    OotEvent::MQ_DC_LOWER_TUNNEL_EYE_SWITCH,
    OotEvent::MQ_DC_OPEN_SKULL,
    OotEvent::MQ_DC_PILLAR_RAISE,
    OotEvent::MQ_DC_POE_ROOM_BOMB_SWITCHES,
    OotEvent::MQ_DC_ROOM_BEFORE_UPPER_LIZALFOS_GOLD_TORCH,
    OotEvent::MQ_DC_STAIRCASE_SWITCH,
    OotEvent::MQ_DC_UPPER_LIZALFOS_CLEAR_FROM_LOWER,
    OotEvent::MQ_DC_UPPER_LIZALFOS_CLEAR_FROM_UPPER,
    OotEvent::OPEN_MQ_DC_LARVAE_ROOM,
    // Jabu-Jabu
    OotEvent::BIG_OCTO,
    OotEvent::BLUE_PARASITE,
    OotEvent::GREEN_PARASITE,
    OotEvent::PARASITE,
    OotEvent::RED_PARASITE,
    // Jabu-Jabu MQ
    OotEvent::JABU_BIG_OCTO,
    OotEvent::JABU_MQ_BACK_CENTER_SWITCH,
    OotEvent::JABU_MQ_BACK_FIRE_WEB,
    OotEvent::JABU_MQ_BACK_UNLOCK,
    OotEvent::JABU_MQ_BASEMENT_SIDE_BUTTON,
    OotEvent::JABU_MQ_BASEMENT_SIDE_PLATFORM,
    OotEvent::JABU_MQ_END,
    OotEvent::JABU_MQ_LIKE_LIKE_ROOM_CLEAR,
    OotEvent::JABU_MQ_LOWER_BIG_OCTO_PLATFORM,
    OotEvent::JABU_MQ_MAIN_ELEVATOR_COMPASS_CHEST_COW_SWITCH,
    OotEvent::JABU_MQ_START,
    OotEvent::JABU_MQ_UNDERWATER_ALCOVE_SWITCH,
    OotEvent::JABU_MQ_WATER_SPOUTS,
    OotEvent::JABU_TENTACLE_BLUE,
    OotEvent::JABU_TENTACLE_GREEN,
    OotEvent::JABU_TENTACLE_RED,
    OotEvent::MQ_JABU_AFTER_ABOVE_BIG_OCTO_SPAWN_COW_AND_CRATES,
    // Forest Temple
    OotEvent::FOREST_LEDGE_REACHED,
    OotEvent::FOREST_POE_1,
    OotEvent::FOREST_POE_2,
    OotEvent::FOREST_POE_3,
    OotEvent::FOREST_POE_4,
    OotEvent::FOREST_TWISTED_HALL,
    OotEvent::FOREST_WELL,
    // Forest Temple MQ
    OotEvent::FOREST_TWIST_SWITCH,
    // Fire Temple
    OotEvent::FIRE_TEMPLE_PILLAR_HAMMER,
    // Fire Temple MQ
    OotEvent::FIRE_MQ_1ST_GORON_LIKELIKE,
    OotEvent::FIRE_MQ_3F_LAVA_ROOM_BLUE_SWITCH,
    OotEvent::FIRE_MQ_3F_LAVA_ROOM_TORCHES,
    OotEvent::FIRE_MQ_EAST_TOWER_TOP_HOOKSHOT_TARGETS,
    OotEvent::FIRE_MQ_FIRE_WALLS_MIDDLE_ROOM_RUSTY_SWITCH,
    OotEvent::FIRE_MQ_FLARE_DANCER_AFTER_FIRE_WALLS,
    OotEvent::FIRE_MQ_HAMMER_LOOP_FIRST_CLEAR,
    OotEvent::FIRE_MQ_HAMMER_LOOP_FLARE_DANCER_CLEAR,
    OotEvent::FIRE_MQ_HAMMER_LOOP_KNUCKLE_CLEAR,
    OotEvent::FIRE_MQ_HIGH_LEDGE_AFTER_STAIRCASE_PILLAR,
    OotEvent::FIRE_MQ_LAVA_BRIDGE_ROOM_HOOKSHOT_PLATFORMS,
    OotEvent::FIRE_MQ_LAVA_BRIDGE_ROOM_TORCHES,
    OotEvent::FIRE_MQ_MAP_CHEST_HAMMER_SWITCH,
    OotEvent::FIRE_MQ_MAZE_ROOM_BLUE_SWITCH,
    OotEvent::FIRE_MQ_MAZE_ROOM_LOWER_CAGE_SWITCH,
    OotEvent::FIRE_MQ_MAZE_ROOM_LOWER_RUSTY_SWITCH,
    OotEvent::FIRE_MQ_MAZE_SHORTCUT_SWITCH,
    OotEvent::FIRE_MQ_PRE_BOSS_PILLAR,
    OotEvent::FIRE_MQ_PRE_BOSS_TORCHES,
    OotEvent::FIRE_MQ_ROOM_BEFORE_MAZE_HOOKSHOT_TARGET,
    OotEvent::FIRE_MQ_STAIRCASE_LOWERED,
    OotEvent::FIRE_MQ_TOWER_AFTER_FLARE_DANCER_CRYSTAL_SWITCH_FROM_BELOW,
    OotEvent::FIRE_MQ_TOWER_TOP_HAMMER_SWITCH_BEFORE_STAIRCASE,
    // Water Temple
    OotEvent::LONGSHOT_TIME_BLOCK,
    OotEvent::WATER_LEVEL_LOW,
    OotEvent::WATER_LEVEL_MIDDLE,
    OotEvent::WATER_LEVEL_RESET,
    // Water Temple MQ
    OotEvent::MOVE_WATER_TIME_BLOCK,
    OotEvent::RUTO_COLUMN_HOOKSHOTS,
    OotEvent::WATER_CENTRAL_GATE,
    OotEvent::WATER_GATES,
    OotEvent::WATER_LEVEL_HIGH,
    OotEvent::WATER_LEVEL_MID,
    OotEvent::WATER_MQ_BRONZE_SCALE_SOFTLOCK_PREVENTION,
    OotEvent::WATER_MQ_CARRY_SMALL_CRATE,
    OotEvent::WATER_MQ_THREE_TORCH_ROOM_GATE,
    // Spirit Temple
    OotEvent::SPIRIT_ADULT_DOOR,
    OotEvent::SPIRIT_CHEST_CHILD,
    OotEvent::SPIRIT_CHILD_DOOR,
    OotEvent::SPIRIT_LIGHT_STATUE,
    // Spirit Temple MQ
    OotEvent::SPIRIT_LOBBY_BOULDERS,
    OotEvent::SPIRIT_PARADOX,
    OotEvent::SPIRIT_STATUE_FIRE,
    OotEvent::SPIRIT_TEMPLE_LIGHT,
    // Shadow Temple
    OotEvent::SHADOW_INVISIBLE_SCYTHE_GATE,
    OotEvent::SHADOW_PILLAR,
    OotEvent::SHADOW_SHORTCUT,
    // Shadow Temple MQ
    OotEvent::SHADOW_MQ_ACTIVATE_BOAT_RIDE,
    OotEvent::SHADOW_MQ_AFTER_BOAT_AFTER_BRIDGE_EYE_SWITCH,
    OotEvent::SHADOW_MQ_AFTER_BOAT_BRIDGE_FALL,
    OotEvent::SHADOW_MQ_AFTER_WIND_TUNNEL_GIBDOS_CLEAR,
    OotEvent::SHADOW_MQ_AREA_AFTER_BOAT_AFTER_BRIDGE_HIGH_LEDGE_SWITCH,
    OotEvent::SHADOW_MQ_CRUSHING_WALLS_BURNT,
    OotEvent::SHADOW_MQ_DUAL_STAIRCASE_STALFOS_CLEAR,
    OotEvent::SHADOW_MQ_FIRST_BEAMOS_FORK_GIBDOS_CLEAR,
    OotEvent::SHADOW_MQ_INVISIBLE_SPIKE_FLOORS_ICE_PLATFORMS,
    OotEvent::SHADOW_MQ_INVISIBLE_SPIKE_FLOORS_REDEADS_CLEAR,
    OotEvent::SHADOW_MQ_INVISIBLE_WALL_MAZE_DEAD_HAND_CLEAR,
    OotEvent::SHADOW_MQ_LOWER_HUGE_PIT_FALLING_SPIKES_GATE_SWITCH,
    OotEvent::SHADOW_MQ_SCYTHE_ROOM_SKULTULLAS_CLEAR,
    OotEvent::SHADOW_MQ_SHORTCUT_BLOCK_PUSHED,
    OotEvent::SHADOW_MQ_SPINNER_ROOM_ICE_PLATFORM,
    OotEvent::SHADOW_MQ_TEXTURED_MAZE_DEAD_HAND_ROOM_CLEAR,
    OotEvent::SHADOW_MQ_TEXTURED_MAZE_LOWER_ICE_BLOCK,
    OotEvent::SHADOW_MQ_TEXTURED_MAZE_SIDE_ROOM_CLEAR,
    OotEvent::SHADOW_MQ_TRUTH_SPINNER,
    OotEvent::SHADOW_MQ_UPPER_HUGE_PIT_FROZEN_EYE,
    OotEvent::SHADOW_MQ_WIND_TUNNEL_HINT_ROOM_REDEADS_CLEAR,
    // Bottom of the Well
    OotEvent::BOTW_WATER_DRAINED,
    // Bottom of the Well MQ
    OotEvent::BOTW_CENTER_EAST_TORCH,
    OotEvent::BOTW_CENTER_WEST_TORCH,
    OotEvent::BOTW_MQ_CENTER_GATES_LOWERED,
    OotEvent::BOTW_MQ_DRAIN_WATER,
    OotEvent::BOTW_MQ_EYE_GATE,
    OotEvent::BOTW_MQ_GATE_FROM_SWITCH,
    OotEvent::BOTW_MQ_REDEAD_CHEST,
    // Ice Cavern MQ
    OotEvent::ICE_MQ_FINAL_ROOM_CLEAR,
    OotEvent::ICE_MQ_FIRST_CRYSTAL,
    OotEvent::ICE_MQ_MAIN_ENEMIES_CLEAR,
    OotEvent::ICE_MQ_MAP_SWITCH,
    OotEvent::ICE_MQ_SECOND_CRYSTAL,
    // Gerudo Training Ground
    OotEvent::GTG_ICE_ARROWS_SWITCH,
    OotEvent::GTG_IRON_KNUCKLE,
    OotEvent::GTG_LAVA_HOOK_TARGETS,
    OotEvent::GTG_LEFT_SIDE,
    OotEvent::GTG_LIKE_LIKE_ROOM,
    OotEvent::GTG_RIGHT_SIDE,
    // Gerudo Training Ground MQ
    OotEvent::GTG_MQ_KNUCKLE_AND_SLUGS_CLEAR,
    OotEvent::GTG_MQ_LAVA_HAMMER_SWITCH,
    OotEvent::GTG_MQ_LAVA_TORCH_COMMON_PLATFORMS_ENTRANCE,
    OotEvent::GTG_MQ_LAVA_TORCH_ENTRANCE_SIDE,
    OotEvent::GTG_MQ_LAVA_TORCH_FAR,
    OotEvent::GTG_MQ_LAVA_TORCH_WATER_ROOM_SIDE,
    OotEvent::GTG_MQ_RIGHT_FIRST_CLEAR,
    OotEvent::GTG_MQ_SILVER_BLOCK_PUSH,
    OotEvent::GTG_MQ_SLOPES_STALAGMITES,
    OotEvent::GTG_MQ_SPINNING_STATUE_CRYSTAL,
    OotEvent::GTG_MQ_SPINNING_STATUE_EYES,
    OotEvent::GTG_MQ_STALFOS_CLEAR,
    OotEvent::GTG_MQ_STALFOS_SIDE_CLEAR,
    // Ganon's Castle
    OotEvent::GANON_START,
    OotEvent::GANON_TOWER_IRON_KNUCKLE_CLEAR,
    OotEvent::GANON_TOWER_LIZALFOS_CLEAR,
    OotEvent::GANON_TOWER_STALFOS_CLEAR,
    // Ganon's Castle MQ
    OotEvent::GANON_CASTLE_MQ_FOREST_WIND_FROZEN_EYE,
    OotEvent::GANON_CASTLE_MQ_FOREST_WIND_OPEN_EYE,
    OotEvent::GANON_MQ_FIRE_MONOLITH,
    OotEvent::GANON_MQ_FOREST_ENEMIES,
    OotEvent::GANON_MQ_FOREST_SWITCH,
    OotEvent::GANON_MQ_LIGHT_ENEMIES,
    OotEvent::GANON_MQ_SHADOW_BOMBFLOWER_FROM_ENTRANCE,
    OotEvent::GANON_MQ_SHADOW_EYE_CHEST,
    OotEvent::GANON_MQ_SHADOW_PROGRESSION_FROM_BEAMOS_1,
    OotEvent::GANON_MQ_SHADOW_PROGRESSION_FROM_ENTRANCE,
    OotEvent::GANON_MQ_SHADOW_TORCH_ICE_BLOCK,
    OotEvent::GANON_MQ_SPIRIT_CRYSTAL,
    OotEvent::GANON_MQ_SPIRIT_ENDING_SUNS,
    OotEvent::GANON_MQ_SPIRIT_HAMMER_SWITCH,
    OotEvent::GANON_MQ_SPIRIT_ZOMBIES,
    OotEvent::MQ_GANON_OPEN_MAIN,
    OotEvent::MQ_GANON_WATER_BF_SWITCH,
];

// Build name lookup map at compile time using lazy_static pattern
use std::sync::LazyLock;

static EVENT_NAME_MAP: LazyLock<HashMap<&'static str, OotEvent>> = LazyLock::new(|| {
    let mut map = HashMap::new();
    for event in ALL_EVENTS {
        map.insert(event.name(), *event);
    }
    map
});

/// OoT save data offsets for event flags.
pub mod offsets {
    /// EventChkInf offset in save data (14 u16 values = 28 bytes).
    pub const EVENT_CHK_INF: usize = 0x0ED4;
    /// EventChkInf size in bytes.
    pub const EVENT_CHK_INF_SIZE: usize = 28;
    /// InfTable offset in save data (60 bytes).
    pub const INF_TABLE: usize = 0x0EF8;
    /// InfTable size in bytes.
    pub const INF_TABLE_SIZE: usize = 60;
    /// Scene flags offset in save data.
    pub const SCENE_FLAGS: usize = 0x00D4;
    /// Size of each scene's flags (0x1C = 28 bytes).
    pub const SCENE_SIZE: usize = 0x1C;
    /// Number of scenes.
    pub const NUM_SCENES: usize = 0x65;
}

/// Error type for event reading operations.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EventReadError {
    /// Save data buffer is too small.
    BufferTooSmall { expected: usize, actual: usize },
    /// Event is volatile and not stored in save data.
    VolatileEvent(OotEvent),
    /// Invalid scene index.
    InvalidScene(u8),
}

impl std::fmt::Display for EventReadError {
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

impl std::error::Error for EventReadError {}

/// Reader for OoT event flags from save data.
///
/// This struct provides methods to check event states from raw save data bytes.
///
/// # Example
///
/// ```
/// use ootmm::events::oot::{OotEvent, OotEventReader};
///
/// // Create a mock save buffer (in real use, this would be actual save data)
/// let save_data = vec![0u8; 0x1000];
/// let reader = OotEventReader::new(&save_data);
///
/// // Check if an event is set (will return false for empty data)
/// if let Ok(is_set) = reader.is_event_set(OotEvent::BOSS_GOHMA) {
///     println!("Gohma defeated: {}", is_set);
/// }
/// ```
pub struct OotEventReader<'a> {
    data: &'a [u8],
}

impl<'a> OotEventReader<'a> {
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
    pub fn is_event_set(&self, event: OotEvent) -> Result<bool, EventReadError> {
        match event.flag() {
            OotEventFlag::EventChkInf(word_idx, mask) => self.read_event_chk_inf(word_idx, mask),
            OotEventFlag::InfTable(byte_idx, mask) => self.read_inf_table(byte_idx, mask),
            OotEventFlag::SceneFlag(scene_id, flag_type, mask) => {
                self.read_scene_flag(scene_id, flag_type, mask)
            }
            OotEventFlag::Volatile => Err(EventReadError::VolatileEvent(event)),
        }
    }

    /// Read a flag from EventChkInf.
    fn read_event_chk_inf(&self, word_idx: u8, mask: u16) -> Result<bool, EventReadError> {
        let offset = offsets::EVENT_CHK_INF + (word_idx as usize) * 2;
        let end = offset + 2;

        if self.data.len() < end {
            return Err(EventReadError::BufferTooSmall {
                expected: end,
                actual: self.data.len(),
            });
        }

        let word = BigEndian::read_u16(&self.data[offset..end]);
        Ok((word & mask) != 0)
    }

    /// Read a flag from InfTable.
    fn read_inf_table(&self, byte_idx: u8, mask: u8) -> Result<bool, EventReadError> {
        let offset = offsets::INF_TABLE + byte_idx as usize;

        if self.data.len() <= offset {
            return Err(EventReadError::BufferTooSmall {
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
    ) -> Result<bool, EventReadError> {
        if scene_id as usize >= offsets::NUM_SCENES {
            return Err(EventReadError::InvalidScene(scene_id));
        }

        // Flag types: 0=chests, 1=switches, 2=room_clear, 3=collectible
        let flag_offset = match flag_type {
            0 => 0x00, // chests
            1 => 0x04, // switches
            2 => 0x08, // room_clear
            3 => 0x0C, // collectible
            _ => return Err(EventReadError::InvalidScene(scene_id)),
        };

        let scene_offset = offsets::SCENE_FLAGS + (scene_id as usize) * offsets::SCENE_SIZE;
        let offset = scene_offset + flag_offset;
        let end = offset + 4;

        if self.data.len() < end {
            return Err(EventReadError::BufferTooSmall {
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
    pub fn get_set_events(&self) -> impl Iterator<Item = OotEvent> + '_ {
        OotEvent::all_persistent().filter(|event| self.is_event_set(*event).unwrap_or(false))
    }

    /// Get all boss clear events that are set.
    pub fn get_boss_clears(&self) -> impl Iterator<Item = OotEvent> + '_ {
        OotEvent::by_category(OotEventCategory::Boss)
            .filter(|event| self.is_event_set(*event).unwrap_or(false))
    }

    /// Count how many bosses have been defeated.
    #[must_use]
    pub fn count_boss_clears(&self) -> usize {
        self.get_boss_clears().count()
    }
}

/// Writer for OoT event flags to save data.
///
/// This struct provides methods to set event states in raw save data bytes.
pub struct OotEventWriter<'a> {
    data: &'a mut [u8],
}

impl<'a> OotEventWriter<'a> {
    /// Create a new event writer from mutable save data.
    #[must_use]
    pub fn new(data: &'a mut [u8]) -> Self {
        Self { data }
    }

    /// Set an event flag.
    ///
    /// Returns an error if the event is volatile or the buffer is too small.
    pub fn set_event(&mut self, event: OotEvent, value: bool) -> Result<(), EventReadError> {
        match event.flag() {
            OotEventFlag::EventChkInf(word_idx, mask) => {
                self.write_event_chk_inf(word_idx, mask, value)
            }
            OotEventFlag::InfTable(byte_idx, mask) => self.write_inf_table(byte_idx, mask, value),
            OotEventFlag::SceneFlag(scene_id, flag_type, mask) => {
                self.write_scene_flag(scene_id, flag_type, mask, value)
            }
            OotEventFlag::Volatile => Err(EventReadError::VolatileEvent(event)),
        }
    }

    /// Write a flag to EventChkInf.
    fn write_event_chk_inf(
        &mut self,
        word_idx: u8,
        mask: u16,
        value: bool,
    ) -> Result<(), EventReadError> {
        let offset = offsets::EVENT_CHK_INF + (word_idx as usize) * 2;
        let end = offset + 2;

        if self.data.len() < end {
            return Err(EventReadError::BufferTooSmall {
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

    /// Write a flag to InfTable.
    fn write_inf_table(
        &mut self,
        byte_idx: u8,
        mask: u8,
        value: bool,
    ) -> Result<(), EventReadError> {
        let offset = offsets::INF_TABLE + byte_idx as usize;

        if self.data.len() <= offset {
            return Err(EventReadError::BufferTooSmall {
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
    ) -> Result<(), EventReadError> {
        if scene_id as usize >= offsets::NUM_SCENES {
            return Err(EventReadError::InvalidScene(scene_id));
        }

        let flag_offset = match flag_type {
            0 => 0x00,
            1 => 0x04,
            2 => 0x08,
            3 => 0x0C,
            _ => return Err(EventReadError::InvalidScene(scene_id)),
        };

        let scene_offset = offsets::SCENE_FLAGS + (scene_id as usize) * offsets::SCENE_SIZE;
        let offset = scene_offset + flag_offset;
        let end = offset + 4;

        if self.data.len() < end {
            return Err(EventReadError::BufferTooSmall {
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
        for event in OotEvent::all() {
            assert!(!event.name().is_empty(), "Event {:?} has empty name", event);
        }
    }

    #[test]
    fn test_event_from_name() {
        assert_eq!(
            OotEvent::from_name("BOSS_GOHMA"),
            Some(OotEvent::BOSS_GOHMA)
        );
        assert_eq!(
            OotEvent::from_name("boss_gohma"),
            Some(OotEvent::BOSS_GOHMA)
        );
        assert_eq!(
            OotEvent::from_name("MIDO_MOVED"),
            Some(OotEvent::MIDO_MOVED)
        );
        assert_eq!(OotEvent::from_name("NOT_A_REAL_EVENT"), None);
    }

    #[test]
    fn test_boss_events_are_persistent() {
        for event in OotEvent::by_category(OotEventCategory::Boss) {
            assert!(
                event.is_persistent(),
                "Boss event {:?} should be persistent",
                event
            );
        }
    }

    #[test]
    fn test_boss_flag_mappings() {
        // Verify boss events map to EventChkInf
        assert!(matches!(
            OotEvent::BOSS_GOHMA.flag(),
            OotEventFlag::EventChkInf(0, 0x0080)
        ));
        assert!(matches!(
            OotEvent::BOSS_KING_DODONGO.flag(),
            OotEventFlag::EventChkInf(2, 0x0020)
        ));
        assert!(matches!(
            OotEvent::BOSS_BARINADE.flag(),
            OotEventFlag::EventChkInf(3, 0x0080)
        ));
    }

    #[test]
    fn test_story_events() {
        assert!(matches!(
            OotEvent::MIDO_MOVED.flag(),
            OotEventFlag::EventChkInf(0, 0x0010)
        ));
        assert!(matches!(
            OotEvent::WELL_DRAIN.flag(),
            OotEventFlag::EventChkInf(6, 0x0080)
        ));
        assert!(matches!(
            OotEvent::KAKARIKO_GATE_OPEN.flag(),
            OotEventFlag::InfTable(15, 0x40)
        ));
    }

    #[test]
    fn test_event_count() {
        // We have 214 events total (224 from YAML minus numeric events 0-9)
        assert!(OotEvent::all().len() >= 200);
    }

    #[test]
    fn test_category_classification() {
        assert_eq!(OotEvent::BOSS_GOHMA.category(), OotEventCategory::Boss);
        assert_eq!(OotEvent::MIDO_MOVED.category(), OotEventCategory::Story);
        assert_eq!(OotEvent::DC_BOMB_EYES.category(), OotEventCategory::Dungeon);
        assert_eq!(
            OotEvent::DC_MQ_STAIRCASE.category(),
            OotEventCategory::DungeonMq
        );
    }

    #[test]
    fn test_display_and_parse() {
        let event = OotEvent::BOSS_GOHMA;
        let name = event.to_string();
        let parsed: OotEvent = name.parse().unwrap();
        assert_eq!(event, parsed);
    }

    // ============================================
    // Event Reader/Writer Tests
    // ============================================

    fn create_test_save_data() -> Vec<u8> {
        // Create a buffer large enough for all save data sections
        vec![0u8; 0x1500]
    }

    #[test]
    fn test_event_reader_empty_data() {
        let data = create_test_save_data();
        let reader = OotEventReader::new(&data);

        // All events should be unset in empty data
        assert!(!reader.is_event_set(OotEvent::BOSS_GOHMA).unwrap());
        assert!(!reader.is_event_set(OotEvent::MIDO_MOVED).unwrap());
        assert!(!reader.is_event_set(OotEvent::KAKARIKO_GATE_OPEN).unwrap());
    }

    #[test]
    fn test_event_reader_volatile_event() {
        let data = create_test_save_data();
        let reader = OotEventReader::new(&data);

        // Volatile events should return an error
        let result = reader.is_event_set(OotEvent::DC_BOMB_EYES);
        assert!(matches!(result, Err(EventReadError::VolatileEvent(_))));
    }

    #[test]
    fn test_event_reader_buffer_too_small() {
        let small_data = vec![0u8; 10]; // Too small
        let reader = OotEventReader::new(&small_data);

        let result = reader.is_event_set(OotEvent::BOSS_GOHMA);
        assert!(matches!(result, Err(EventReadError::BufferTooSmall { .. })));
    }

    #[test]
    fn test_event_writer_set_event_chk_inf() {
        let mut data = create_test_save_data();

        // Set BOSS_GOHMA (EventChkInf word 0, bit 0x0080)
        {
            let mut writer = OotEventWriter::new(&mut data);
            writer.set_event(OotEvent::BOSS_GOHMA, true).unwrap();
        }

        // Verify it's set
        let reader = OotEventReader::new(&data);
        assert!(reader.is_event_set(OotEvent::BOSS_GOHMA).unwrap());

        // Clear it
        {
            let mut writer = OotEventWriter::new(&mut data);
            writer.set_event(OotEvent::BOSS_GOHMA, false).unwrap();
        }

        // Verify it's cleared
        let reader = OotEventReader::new(&data);
        assert!(!reader.is_event_set(OotEvent::BOSS_GOHMA).unwrap());
    }

    #[test]
    fn test_event_writer_set_inf_table() {
        let mut data = create_test_save_data();

        // Set KAKARIKO_GATE_OPEN (InfTable byte 15, bit 0x40)
        {
            let mut writer = OotEventWriter::new(&mut data);
            writer
                .set_event(OotEvent::KAKARIKO_GATE_OPEN, true)
                .unwrap();
        }

        // Verify it's set
        let reader = OotEventReader::new(&data);
        assert!(reader.is_event_set(OotEvent::KAKARIKO_GATE_OPEN).unwrap());
    }

    #[test]
    fn test_event_writer_volatile_event() {
        let mut data = create_test_save_data();
        let mut writer = OotEventWriter::new(&mut data);

        // Volatile events should return an error
        let result = writer.set_event(OotEvent::DC_BOMB_EYES, true);
        assert!(matches!(result, Err(EventReadError::VolatileEvent(_))));
    }

    #[test]
    fn test_event_reader_get_set_events() {
        let mut data = create_test_save_data();

        // Set a few events
        {
            let mut writer = OotEventWriter::new(&mut data);
            writer.set_event(OotEvent::BOSS_GOHMA, true).unwrap();
            writer.set_event(OotEvent::BOSS_KING_DODONGO, true).unwrap();
            writer.set_event(OotEvent::MIDO_MOVED, true).unwrap();
        }

        // Get all set events
        let reader = OotEventReader::new(&data);
        let set_events: Vec<_> = reader.get_set_events().collect();

        assert!(set_events.contains(&OotEvent::BOSS_GOHMA));
        assert!(set_events.contains(&OotEvent::BOSS_KING_DODONGO));
        assert!(set_events.contains(&OotEvent::MIDO_MOVED));
    }

    #[test]
    fn test_event_reader_get_boss_clears() {
        let mut data = create_test_save_data();

        // Set some boss clears
        {
            let mut writer = OotEventWriter::new(&mut data);
            writer.set_event(OotEvent::BOSS_GOHMA, true).unwrap();
            writer.set_event(OotEvent::BOSS_VOLVAGIA, true).unwrap();
        }

        let reader = OotEventReader::new(&data);
        let boss_clears: Vec<_> = reader.get_boss_clears().collect();

        assert_eq!(boss_clears.len(), 2);
        assert!(boss_clears.contains(&OotEvent::BOSS_GOHMA));
        assert!(boss_clears.contains(&OotEvent::BOSS_VOLVAGIA));
    }

    #[test]
    fn test_event_reader_count_boss_clears() {
        let mut data = create_test_save_data();

        let reader = OotEventReader::new(&data);
        assert_eq!(reader.count_boss_clears(), 0);

        // Set some boss clears
        {
            let mut writer = OotEventWriter::new(&mut data);
            writer.set_event(OotEvent::BOSS_GOHMA, true).unwrap();
            writer.set_event(OotEvent::BOSS_KING_DODONGO, true).unwrap();
            writer.set_event(OotEvent::BOSS_BARINADE, true).unwrap();
        }

        let reader = OotEventReader::new(&data);
        assert_eq!(reader.count_boss_clears(), 3);
    }

    #[test]
    fn test_multiple_events_same_word() {
        let mut data = create_test_save_data();

        // BOSS_GOHMA and MIDO_MOVED are both in EventChkInf word 0
        {
            let mut writer = OotEventWriter::new(&mut data);
            writer.set_event(OotEvent::BOSS_GOHMA, true).unwrap();
            writer.set_event(OotEvent::MIDO_MOVED, true).unwrap();
        }

        let reader = OotEventReader::new(&data);
        assert!(reader.is_event_set(OotEvent::BOSS_GOHMA).unwrap());
        assert!(reader.is_event_set(OotEvent::MIDO_MOVED).unwrap());

        // Clear just one
        {
            let mut writer = OotEventWriter::new(&mut data);
            writer.set_event(OotEvent::BOSS_GOHMA, false).unwrap();
        }

        let reader = OotEventReader::new(&data);
        assert!(!reader.is_event_set(OotEvent::BOSS_GOHMA).unwrap());
        assert!(reader.is_event_set(OotEvent::MIDO_MOVED).unwrap()); // Should still be set
    }
}
