//! MM Scene ID constants.
//!
//! Scene IDs correspond to the index in the scene flag array.
//! MM has 120 permanent scene flag slots.

// Main Dungeons
pub const WOODFALL_TEMPLE: u8 = 0x1F;
pub const SNOWHEAD_TEMPLE: u8 = 0x22;
pub const GREAT_BAY_TEMPLE: u8 = 0x1E;
pub const STONE_TOWER_TEMPLE: u8 = 0x18;
pub const STONE_TOWER_TEMPLE_INVERTED: u8 = 0x19;

// Dungeon Boss Rooms
pub const WOODFALL_TEMPLE_BOSS: u8 = 0x1A;
pub const SNOWHEAD_TEMPLE_BOSS: u8 = 0x24;
pub const GREAT_BAY_TEMPLE_BOSS: u8 = 0x4F;
pub const STONE_TOWER_TEMPLE_BOSS: u8 = 0x36;

// Mini Dungeons
pub const BENEATH_THE_WELL: u8 = 0x1B;
pub const ANCIENT_CASTLE_OF_IKANA: u8 = 0x11;
pub const IKANA_CANYON_SECRET_SHRINE: u8 = 0x13;
pub const PIRATES_FORTRESS: u8 = 0x29;
pub const PIRATES_FORTRESS_INTERIOR: u8 = 0x2A;
pub const BENEATH_THE_GRAVEYARD: u8 = 0x07;

// Spider Houses
pub const SWAMP_SPIDER_HOUSE: u8 = 0x27;
pub const OCEANSIDE_SPIDER_HOUSE: u8 = 0x28;

// Clock Town Areas
pub const CLOCK_TOWN_SOUTH: u8 = 0x6C;
pub const CLOCK_TOWN_NORTH: u8 = 0x6D;
pub const CLOCK_TOWN_EAST: u8 = 0x6E;
pub const CLOCK_TOWN_WEST: u8 = 0x6F;
pub const LAUNDRY_POOL: u8 = 0x70;
pub const CLOCK_TOWER: u8 = 0x08;

// Clock Town Buildings
pub const STOCK_POT_INN: u8 = 0x4D;
pub const STOCK_POT_INN_RESERVATION: u8 = 0x4B;
pub const MILK_BAR: u8 = 0x51;
pub const MAYORS_OFFICE: u8 = 0x4E;
pub const POST_OFFICE: u8 = 0x30;
pub const LOTTERY_SHOP: u8 = 0x42;
pub const TRADING_POST: u8 = 0x4A;
pub const BOMB_SHOP: u8 = 0x32;
pub const CURIOSITY_SHOP: u8 = 0x33;
pub const HONEY_AND_DARLING: u8 = 0x44;
pub const TREASURE_CHEST_SHOP: u8 = 0x4C;
pub const ASTRAL_OBSERVATORY: u8 = 0x52;
pub const CLOCK_TOWN_GREAT_FAIRY: u8 = 0x26;

// Termina Field and Roads
pub const TERMINA_FIELD: u8 = 0x54;
pub const ROAD_TO_SOUTHERN_SWAMP: u8 = 0x0D;
pub const MILK_ROAD: u8 = 0x5E;
pub const PATH_TO_MOUNTAIN_VILLAGE: u8 = 0x64;
pub const ROAD_TO_IKANA: u8 = 0x47;
pub const GREAT_BAY_COAST: u8 = 0x37;

// Southern Swamp
pub const SOUTHERN_SWAMP: u8 = 0x55;
pub const SOUTHERN_SWAMP_CLEAR: u8 = 0x56;
pub const SWAMP_TOURIST_CENTER: u8 = 0x57;
pub const DEKU_PALACE: u8 = 0x14;
pub const DEKU_PALACE_GARDEN: u8 = 0x4F;
pub const WOODFALL: u8 = 0x20;
pub const WOODFALL_GREAT_FAIRY: u8 = 0x26;

// Snowhead Region
pub const MOUNTAIN_VILLAGE: u8 = 0x5A;
pub const MOUNTAIN_VILLAGE_SPRING: u8 = 0x5B;
pub const GORON_VILLAGE: u8 = 0x5C;
pub const GORON_VILLAGE_SPRING: u8 = 0x5D;
pub const PATH_TO_SNOWHEAD: u8 = 0x5F;
pub const SNOWHEAD: u8 = 0x23;
pub const GORON_SHRINE: u8 = 0x58;
pub const SNOWHEAD_GREAT_FAIRY: u8 = 0x26;

// Great Bay Region
pub const ZORA_CAPE: u8 = 0x38;
pub const ZORA_HALL: u8 = 0x60;
pub const PINNACLE_ROCK: u8 = 0x3F;
pub const PIRATES_FORTRESS_EXTERIOR: u8 = 0x3B;
pub const GREAT_BAY_GREAT_FAIRY: u8 = 0x26;

// Ikana Region
pub const IKANA_CANYON: u8 = 0x13;
pub const IKANA_GRAVEYARD: u8 = 0x09;
pub const STONE_TOWER: u8 = 0x17;
pub const STONE_TOWER_INVERTED: u8 = 0x17;
pub const IKANA_CASTLE: u8 = 0x11;
pub const IKANA_GREAT_FAIRY: u8 = 0x26;

// Ranch
pub const ROMANI_RANCH: u8 = 0x64;
pub const CUCCO_SHACK: u8 = 0x5F;
pub const DOGGY_RACETRACK: u8 = 0x61;

// The Moon
pub const MOON: u8 = 0x08;
pub const MOON_DEKU_TRIAL: u8 = 0x08;
pub const MOON_GORON_TRIAL: u8 = 0x08;
pub const MOON_ZORA_TRIAL: u8 = 0x08;
pub const MOON_LINK_TRIAL: u8 = 0x08;

/// Maximum scene ID for MM.
pub const MAX_SCENE_ID: u8 = 0x78;

/// Number of scenes in MM.
pub const SCENE_COUNT: usize = 120;
