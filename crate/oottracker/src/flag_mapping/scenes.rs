//! OoT Scene ID constants.
//!
//! Scene IDs correspond to the index in the scene flag array.
//! Reference: <https://wiki.cloudmodding.com/oot/Scene_Table/NTSC_1.0>

// Dungeons
pub const DEKU_TREE: u8 = 0x00;
pub const DODONGOS_CAVERN: u8 = 0x01;
pub const JABU_JABU: u8 = 0x02;
pub const FOREST_TEMPLE: u8 = 0x03;
pub const FIRE_TEMPLE: u8 = 0x04;
pub const WATER_TEMPLE: u8 = 0x05;
pub const SPIRIT_TEMPLE: u8 = 0x06;
pub const SHADOW_TEMPLE: u8 = 0x07;
pub const BOTTOM_OF_THE_WELL: u8 = 0x08;
pub const ICE_CAVERN: u8 = 0x09;
pub const GANONS_TOWER: u8 = 0x0A;
pub const GERUDO_TRAINING_GROUND: u8 = 0x0B;
pub const THIEVES_HIDEOUT: u8 = 0x0C;
pub const GANONS_CASTLE: u8 = 0x0D;
pub const GANONS_TOWER_COLLAPSING: u8 = 0x0E;
pub const GANONS_CASTLE_COLLAPSING: u8 = 0x0F;
pub const TREASURE_CHEST_GAME: u8 = 0x10;

// Boss Rooms
pub const DEKU_TREE_BOSS: u8 = 0x11;
pub const DODONGOS_CAVERN_BOSS: u8 = 0x12;
pub const JABU_JABU_BOSS: u8 = 0x13;
pub const FOREST_TEMPLE_BOSS: u8 = 0x14;
pub const FIRE_TEMPLE_BOSS: u8 = 0x15;
pub const WATER_TEMPLE_BOSS: u8 = 0x16;
pub const SPIRIT_TEMPLE_BOSS: u8 = 0x17;
pub const SHADOW_TEMPLE_BOSS: u8 = 0x18;
pub const GANONDORF_BOSS: u8 = 0x19;
pub const GANON_BOSS: u8 = 0x1A;
pub const TOWER_COLLAPSE_EXTERIOR: u8 = 0x1B;

// Overworld
pub const MARKET_ENTRANCE_DAY: u8 = 0x1C;
pub const MARKET_ENTRANCE_NIGHT: u8 = 0x1D;
pub const MARKET_ENTRANCE_RUINS: u8 = 0x1E;
pub const BACK_ALLEY_DAY: u8 = 0x1F;
pub const BACK_ALLEY_NIGHT: u8 = 0x20;
pub const MARKET_DAY: u8 = 0x21;
pub const MARKET_NIGHT: u8 = 0x22;
pub const MARKET_RUINS: u8 = 0x23;
pub const TEMPLE_OF_TIME_EXTERIOR_DAY: u8 = 0x24;
pub const TEMPLE_OF_TIME_EXTERIOR_NIGHT: u8 = 0x25;
pub const TEMPLE_OF_TIME_EXTERIOR_RUINS: u8 = 0x26;
pub const KNOW_IT_ALL_BROTHERS_HOUSE: u8 = 0x27;
pub const MIDOS_HOUSE: u8 = 0x28;
pub const SARIAS_HOUSE: u8 = 0x29;
pub const TWINS_HOUSE: u8 = 0x2A;
pub const LINKS_HOUSE: u8 = 0x2B;
pub const KAKARIKO_HOUSE_1: u8 = 0x2C;
pub const BACK_ALLEY_HOUSE: u8 = 0x2D;
pub const BAZAAR: u8 = 0x2E;
pub const KOKIRI_SHOP: u8 = 0x2F;
pub const GORON_SHOP: u8 = 0x30;
pub const ZORA_SHOP: u8 = 0x31;
pub const KAKARIKO_POTION_SHOP: u8 = 0x32;
pub const MARKET_POTION_SHOP: u8 = 0x33;
pub const BOMBCHU_SHOP: u8 = 0x34;
pub const HAPPY_MASK_SHOP: u8 = 0x35;
pub const GERUDO_VALLEY_TENT: u8 = 0x36;
pub const IMPAS_HOUSE: u8 = 0x37;
pub const LAKESIDE_LABORATORY: u8 = 0x38;
pub const CARPENTERS_TENT: u8 = 0x39;
pub const GRAVEKEEPERS_HUT: u8 = 0x3A;
pub const GREAT_FAIRY_FOUNTAIN_UPGRADES: u8 = 0x3B;
pub const FAIRY_FOUNTAIN: u8 = 0x3C;
pub const GREAT_FAIRY_FOUNTAIN_SPELLS: u8 = 0x3D;
pub const GROTTOS: u8 = 0x3E;
pub const GRAVE_HEART_PIECE: u8 = 0x3F;
pub const GRAVE_SHIELD: u8 = 0x40;
pub const ROYAL_FAMILYS_TOMB: u8 = 0x41;
pub const SHOOTING_GALLERY: u8 = 0x42;
pub const TEMPLE_OF_TIME: u8 = 0x43;
pub const CHAMBER_OF_SAGES: u8 = 0x44;
pub const CASTLE_HEDGE_MAZE_DAY: u8 = 0x45;
pub const CASTLE_HEDGE_MAZE_NIGHT: u8 = 0x46;
pub const CUTSCENE_MAP: u8 = 0x47;
pub const WINDMILL_AND_DAMPES_GRAVE: u8 = 0x48;
pub const FISHING_POND: u8 = 0x49;
pub const CASTLE_COURTYARD: u8 = 0x4A;
pub const BOMBCHU_BOWLING: u8 = 0x4B;
pub const LON_LON_RANCH_TOWER: u8 = 0x4C;
pub const LON_LON_RANCH_HOUSE: u8 = 0x4D;
pub const GUARD_HOUSE: u8 = 0x4E;
pub const KAKARIKO_HOUSE_2: u8 = 0x4F;
pub const KAKARIKO_HOUSE_3: u8 = 0x50;

// Main Overworld Areas
pub const HYRULE_FIELD: u8 = 0x51;
pub const KAKARIKO_VILLAGE: u8 = 0x52;
pub const GRAVEYARD: u8 = 0x53;
pub const ZORA_RIVER: u8 = 0x54;
pub const KOKIRI_FOREST: u8 = 0x55;
pub const SACRED_FOREST_MEADOW: u8 = 0x56;
pub const LAKE_HYLIA: u8 = 0x57;
pub const ZORAS_DOMAIN: u8 = 0x58;
pub const ZORAS_FOUNTAIN: u8 = 0x59;
pub const GERUDO_VALLEY: u8 = 0x5A;
pub const LOST_WOODS: u8 = 0x5B;
pub const DESERT_COLOSSUS: u8 = 0x5C;
pub const GERUDO_FORTRESS: u8 = 0x5D;
pub const HAUNTED_WASTELAND: u8 = 0x5E;
pub const HYRULE_CASTLE: u8 = 0x5F;
pub const DEATH_MOUNTAIN_TRAIL: u8 = 0x60;
pub const DEATH_MOUNTAIN_CRATER: u8 = 0x61;
pub const GORON_CITY: u8 = 0x62;
pub const LON_LON_RANCH: u8 = 0x63;
pub const OUTSIDE_GANONS_CASTLE: u8 = 0x64;

/// Maximum scene ID for OoT.
pub const MAX_SCENE_ID: u8 = 0x64;

/// Number of scenes in OoT.
pub const SCENE_COUNT: usize = 101;
