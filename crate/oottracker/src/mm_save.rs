//! Majora's Mask save data structures and trait definitions.
//!
//! This module provides the trait interface, real memory parsing, and stub implementation
//! for MM save data.
//!
//! Reference: OoTMM source - packages/core/include/combo/mm/save.h
//! MM SaveContext address: 0x801ef670 (size 0x48d0 = 18,640 bytes)

use {
    bitflags::bitflags,
    byteorder::{BigEndian, ByteOrder as _},
    derivative::Derivative,
    derive_more::From,
    std::num::TryFromIntError,
};

/// MM SaveContext base address in N64 memory
pub const MM_ADDR: u32 = 0x801ef670;
/// MM SaveContext size in bytes
pub const MM_SIZE: usize = 0x48d0;
/// Number of permanent scene flag slots in MM
pub const MM_PERM_SCENE_COUNT: usize = 120;
/// Size of each permanent scene flag entry in bytes
pub const MM_PERM_SCENE_SIZE: usize = 0x1c;

// ============================================================================
// MM Item IDs (different from OoT)
// ============================================================================

/// MM inventory item IDs
pub mod mm_item_ids {
    pub const OCARINA: u8 = 0x00;
    pub const BOW: u8 = 0x01;
    pub const FIRE_ARROW: u8 = 0x02;
    pub const ICE_ARROW: u8 = 0x03;
    pub const LIGHT_ARROW: u8 = 0x04;
    pub const QUEST_1: u8 = 0x05; // unused slot
    pub const BOMB: u8 = 0x06;
    pub const BOMBCHU: u8 = 0x07;
    pub const DEKU_STICK: u8 = 0x08;
    pub const DEKU_NUT: u8 = 0x09;
    pub const MAGIC_BEAN: u8 = 0x0A;
    pub const QUEST_2: u8 = 0x0B; // unused slot
    pub const POWDER_KEG: u8 = 0x0C;
    pub const PICTOGRAPH_BOX: u8 = 0x0D;
    pub const LENS: u8 = 0x0E;
    pub const HOOKSHOT: u8 = 0x0F;
    pub const GREAT_FAIRY_SWORD: u8 = 0x10;
    pub const QUEST_3: u8 = 0x11; // unused slot
                                  // Bottles start at 0x12
    pub const BOTTLE_EMPTY: u8 = 0x12;
    pub const BOTTLE_RED_POTION: u8 = 0x13;
    pub const BOTTLE_GREEN_POTION: u8 = 0x14;
    pub const BOTTLE_BLUE_POTION: u8 = 0x15;
    pub const BOTTLE_FAIRY: u8 = 0x16;
    pub const BOTTLE_DEKU_PRINCESS: u8 = 0x17;
    pub const BOTTLE_MILK: u8 = 0x18;
    pub const BOTTLE_MILK_HALF: u8 = 0x19;
    pub const BOTTLE_FISH: u8 = 0x1A;
    pub const BOTTLE_BUG: u8 = 0x1B;
    pub const BOTTLE_BLUE_FIRE: u8 = 0x1C;
    pub const BOTTLE_POE: u8 = 0x1D;
    pub const BOTTLE_BIG_POE: u8 = 0x1E;
    pub const BOTTLE_WATER: u8 = 0x1F;
    pub const BOTTLE_HOT_SPRING_WATER: u8 = 0x20;
    pub const BOTTLE_ZORA_EGG: u8 = 0x21;
    pub const BOTTLE_GOLD_DUST: u8 = 0x22;
    pub const BOTTLE_MUSHROOM: u8 = 0x23;
    pub const BOTTLE_SEAHORSE: u8 = 0x24;
    pub const BOTTLE_CHATEAU_ROMANI: u8 = 0x25;
    pub const BOTTLE_MYSTERY_MILK: u8 = 0x26;
    pub const BOTTLE_MYSTERY_MILK_SPOILED: u8 = 0x27;
    // Masks start at 0x32 - IDs match zeldaret/mm decomp project
    // https://github.com/zeldaret/mm/blob/main/include/z64item.h
    pub const MASK_DEKU: u8 = 0x32;
    pub const MASK_GORON: u8 = 0x33;
    pub const MASK_ZORA: u8 = 0x34;
    pub const MASK_FIERCE_DEITY: u8 = 0x35;
    pub const MASK_TRUTH: u8 = 0x36;
    pub const MASK_KAFEI: u8 = 0x37;
    pub const MASK_ALL_NIGHT: u8 = 0x38;
    pub const MASK_BUNNY: u8 = 0x39;
    pub const MASK_KEATON: u8 = 0x3A;
    pub const MASK_GARO: u8 = 0x3B;
    pub const MASK_ROMANI: u8 = 0x3C;
    pub const MASK_CIRCUS_LEADER: u8 = 0x3D;
    pub const MASK_POSTMAN: u8 = 0x3E;
    pub const MASK_COUPLES: u8 = 0x3F;
    pub const MASK_GREAT_FAIRY: u8 = 0x40;
    pub const MASK_GIBDO: u8 = 0x41;
    pub const MASK_DON_GERO: u8 = 0x42;
    pub const MASK_KAMARO: u8 = 0x43;
    pub const MASK_CAPTAIN: u8 = 0x44;
    pub const MASK_STONE: u8 = 0x45;
    pub const MASK_BREMEN: u8 = 0x46;
    pub const MASK_BLAST: u8 = 0x47;
    pub const MASK_SCENTS: u8 = 0x48;
    pub const MASK_GIANT: u8 = 0x49;
    pub const NONE: u8 = 0xFF;
}

// ============================================================================
// MM Save Structure Offsets (relative to SaveContext start)
// ============================================================================

/// ROM type detection for MM save parsing
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum MmRomType {
    /// Standard vanilla Majora's Mask ROM
    #[default]
    Vanilla,
    /// OoTMM combo randomizer ROM
    OoTMM,
}

impl MmRomType {
    /// Get the ROM type from the `OOTTRACKER_MM_ROM_TYPE` environment variable.
    ///
    /// Set to "ootmm" or "OoTMM" for OoTMM combo ROM support.
    /// Defaults to `Vanilla` if not set or unrecognized.
    pub fn from_env() -> Self {
        match std::env::var("OOTTRACKER_MM_ROM_TYPE")
            .unwrap_or_default()
            .to_lowercase()
            .as_str()
        {
            "ootmm" | "combo" => MmRomType::OoTMM,
            _ => MmRomType::Vanilla,
        }
    }

    /// Determine if this is a combo ROM type.
    ///
    /// When `is_combo` is true, returns `OoTMM`, otherwise returns `Vanilla`.
    /// This is used when the game type is detected automatically rather than
    /// relying on environment variables.
    pub fn from_combo_flag(is_combo: bool) -> Self {
        if is_combo {
            MmRomType::OoTMM
        } else {
            MmRomType::Vanilla
        }
    }
}

/// Memory offsets within MM SaveContext - vanilla MM structure
mod offsets {
    /// Player form (u8)
    pub const PLAYER_FORM: usize = 0x0020;
    /// Health capacity (u16, in 16ths of a heart)
    pub const HEALTH_CAPACITY: usize = 0x002C;
    /// Current health (u16)
    pub const HEALTH: usize = 0x002E;
    /// Magic capacity level (u8: 0=none, 1=single, 2=double)
    pub const MAGIC_LEVEL: usize = 0x0032;
    /// Rupees (u16)
    pub const RUPEES: usize = 0x0034;
    /// Sword and shield equipment bits (u8)
    pub const SWORD_SHIELD: usize = 0x0044;
    /// Double defense (u8: non-zero = has it)
    pub const DOUBLE_DEFENSE: usize = 0x003B;
    /// Inventory items array start (24 slots, each u8)
    pub const INVENTORY: usize = 0x0070;
    /// Mask inventory start (24 slots, each u8)
    pub const MASKS: usize = 0x0088;
    /// Quest items flags (u32)
    pub const QUEST_ITEMS: usize = 0x00A4;
    /// Dungeon items (4 dungeons × 1 byte each)
    pub const DUNGEON_ITEMS: usize = 0x00A8;
    /// Small keys for each dungeon (array starts here)
    pub const SMALL_KEYS: usize = 0x00BC;
    /// Upgrades (u32)
    pub const UPGRADES: usize = 0x00A0;
    /// Stray fairy counts (5 areas × 1 byte each): Clock Town, Woodfall, Snowhead, Great Bay, Stone Tower
    pub const STRAY_FAIRIES: usize = 0x00D0;
    /// Swamp skulltula count (u16)
    pub const SKULL_SWAMP: usize = 0x00D8;
    /// Ocean skulltula count (u16)
    pub const SKULL_OCEAN: usize = 0x00DA;
    /// Permanent scene flags start
    /// Decomp: SaveInfo at 0x24 + permanentSceneFlags at 0xD4 = 0xF8
    /// Reference: https://github.com/zeldaret/mm/blob/main/include/z64save.h
    pub const PERM_SCENE_FLAGS: usize = 0x00F8;
    /// Cycle scene flags start (after perm flags)
    /// Decomp: Cycle flags follow perm flags after additional save data
    pub const CYCLE_SCENE_FLAGS: usize = 0x0DF8;
    /// Current day (u32)
    pub const DAY: usize = 0x0048;
    /// Current time (u16)
    pub const TIME: usize = 0x004C;
    /// Is night flag (derived from time, but sometimes stored)
    pub const IS_NIGHT: usize = 0x0050;

    // Size constants for arrays
    pub const MASK_SLOTS: usize = 24;
    #[allow(dead_code)] // Documented for reference
    pub const INVENTORY_SLOTS: usize = 24;
}

/// Memory offsets within MM SaveContext - OoTMM combo ROM structure
///
/// OoTMM reorganizes the save structure significantly:
/// - Items and masks are combined into a single 48-slot array
/// - SaveInfo starts at offset 0x24 within MmSave
/// - All inventory-related offsets are shifted
///
/// Reference: https://github.com/OoTMM/OoTMM/blob/master/packages/core/include/combo/mm/save.h
mod ootmm_offsets {
    /// Player form (u8) - same as vanilla
    pub const PLAYER_FORM: usize = 0x0020;
    /// Health capacity (u16) - SaveInfo(0x24) + playerData(0x00) + healthCapacity(0x10)
    pub const HEALTH_CAPACITY: usize = 0x0034;
    /// Current health (u16)
    pub const HEALTH: usize = 0x0036;
    /// Magic capacity level (u8)
    pub const MAGIC_LEVEL: usize = 0x0038;
    /// Current magic (u8)
    #[allow(dead_code)] // Documented for reference
    pub const MAGIC: usize = 0x0039;
    /// Rupees (u16)
    pub const RUPEES: usize = 0x003A;
    /// Double defense (u8)
    pub const DOUBLE_DEFENSE: usize = 0x0042;
    /// Sword and shield equipment bits - in itemEquips at 0x4c + 0x20
    pub const SWORD_SHIELD: usize = 0x006C;
    /// Combined inventory array (48 slots: items 0-23, masks 24-47)
    /// SaveInfo(0x24) + inventory(0x4a) + items(0x00)
    pub const INVENTORY: usize = 0x006E;
    /// Masks are in the same array as items, starting at index 32 (0x20)
    /// OoTMM masks are at items[32-55], so INVENTORY + 32 = 0x8E
    /// Note: Not index 24 - the mask slots start at 32 in OoTMM
    pub const MASKS: usize = 0x008E;
    /// Ammo array (24 slots)
    #[allow(dead_code)] // Documented for reference
    pub const AMMO: usize = 0x009E;
    /// Upgrades (u32)
    pub const UPGRADES: usize = 0x00B6;
    /// Quest items flags (u32)
    pub const QUEST_ITEMS: usize = 0x00BA;
    /// Dungeon items (10 dungeons × 1 byte each)
    pub const DUNGEON_ITEMS: usize = 0x00BE;
    /// Small keys for each dungeon (9 dungeons)
    pub const SMALL_KEYS: usize = 0x00C8;
    /// Defense hearts (u8)
    #[allow(dead_code)] // Documented for reference
    pub const DEFENSE_HEARTS: usize = 0x00D1;
    /// Stray fairy counts (10 areas × 1 byte each)
    pub const STRAY_FAIRIES: usize = 0x00D2;
    /// Swamp skulltula count - need to verify this offset
    pub const SKULL_SWAMP: usize = 0x00DC;
    /// Ocean skulltula count - need to verify this offset
    pub const SKULL_OCEAN: usize = 0x00DE;
    /// Permanent scene flags start
    /// OoTMM: SaveInfo(0x24) + after inventory = 0xE4 from info start
    /// Calculation: MmSavePlayerData(0x28) + MmItemEquips(0x14) + MmInventory(0xA8) = 0xE4
    /// Absolute: 0x24 + 0xE4 = 0x108
    /// Reference: https://github.com/OoTMM/OoTMM/blob/master/packages/core/include/combo/mm/save.h
    pub const PERM_SCENE_FLAGS: usize = 0x0108;
    /// Cycle scene flags start
    /// OoTMM places cycle flags at 0x3F68 within MmSaveContext
    /// Reference: https://github.com/OoTMM/OoTMM/blob/master/packages/core/include/combo/mm/save.h
    pub const CYCLE_SCENE_FLAGS: usize = 0x3F68;
    /// Current day (u32)
    pub const DAY: usize = 0x0018;
    /// Days elapsed (u32)
    #[allow(dead_code)] // Documented for reference
    pub const DAYS_ELAPSED: usize = 0x001C;
    /// Current time (u16)
    pub const TIME: usize = 0x000C;
    /// Is night flag (s32)
    pub const IS_NIGHT: usize = 0x0010;

    // Size constants for arrays - OoTMM uses combined array
    pub const MASK_SLOTS: usize = 24;
    #[allow(dead_code)] // Documented for reference
    pub const INVENTORY_SLOTS: usize = 24;
    #[allow(dead_code)] // Documented for reference
    pub const COMBINED_INVENTORY_SLOTS: usize = 48;
}

// ============================================================================
// Decode Error Type
// ============================================================================

/// Errors that can occur when decoding MM save data from raw bytes
#[derive(Debug, From, Clone)]
pub enum MmDecodeError {
    /// A single byte assertion failed
    AssertEq {
        offset: u16,
        expected: u8,
        found: u8,
    },
    /// A range assertion failed
    AssertEqRange {
        start: u16,
        end: u16,
        expected: Vec<u8>,
        found: Vec<u8>,
    },
    /// Index out of bounds
    Index(u16),
    /// Range out of bounds
    IndexRange { start: u16, end: u16 },
    /// Save data is wrong size
    Size(usize),
    /// Unexpected value at offset
    UnexpectedValue {
        offset: u16,
        field: &'static str,
        value: u8,
    },
    /// Unexpected value in range
    UnexpectedValueRange {
        start: u16,
        end: u16,
        field: &'static str,
        value: Vec<u8>,
    },
    /// Integer conversion error
    #[from]
    TryFromInt(TryFromIntError),
}

// ============================================================================
// Core Enums and Types
// ============================================================================

/// Player transformation forms in MM
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum PlayerForm {
    FierceDeity = 0,
    Goron = 1,
    Zora = 2,
    Deku = 3,
    #[default]
    Human = 4,
}

impl TryFrom<u8> for PlayerForm {
    type Error = u8;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(PlayerForm::FierceDeity),
            1 => Ok(PlayerForm::Goron),
            2 => Ok(PlayerForm::Zora),
            3 => Ok(PlayerForm::Deku),
            4 => Ok(PlayerForm::Human),
            _ => Err(value),
        }
    }
}

/// Magic capacity levels
#[derive(Derivative, Debug, Clone, Copy, PartialEq, Eq)]
#[derivative(Default)]
#[repr(u8)]
pub enum MmMagicCapacity {
    #[derivative(Default)]
    None = 0,
    Single = 1,
    Double = 2,
}

impl TryFrom<u8> for MmMagicCapacity {
    type Error = u8;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(MmMagicCapacity::None),
            1 => Ok(MmMagicCapacity::Single),
            2 => Ok(MmMagicCapacity::Double),
            _ => Err(value),
        }
    }
}

/// MM sword levels
#[derive(Derivative, Debug, Clone, Copy, PartialEq, Eq)]
#[derivative(Default)]
#[repr(u8)]
pub enum MmSword {
    #[derivative(Default)]
    None = 0,
    KokiriSword = 1,
    RazorSword = 2,
    GildedSword = 3,
}

impl TryFrom<u8> for MmSword {
    type Error = u8;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(MmSword::None),
            1 => Ok(MmSword::KokiriSword),
            2 => Ok(MmSword::RazorSword),
            3 => Ok(MmSword::GildedSword),
            _ => Err(value),
        }
    }
}

/// MM shield types
#[derive(Derivative, Debug, Clone, Copy, PartialEq, Eq)]
#[derivative(Default)]
#[repr(u8)]
pub enum MmShield {
    #[derivative(Default)]
    None = 0,
    HeroShield = 1,
    MirrorShield = 2,
}

impl TryFrom<u8> for MmShield {
    type Error = u8;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(MmShield::None),
            1 => Ok(MmShield::HeroShield),
            2 => Ok(MmShield::MirrorShield),
            _ => Err(value),
        }
    }
}

// ============================================================================
// Quest Items (Songs, Remains, etc.)
// ============================================================================

bitflags! {
    /// Quest items including boss remains and songs
    #[derive(Default)]
    pub struct MmQuestItems: u32 {
        // Boss Remains (bits 0-3)
        const REMAINS_ODOLWA = 1 << 0;
        const REMAINS_GOHT = 1 << 1;
        const REMAINS_GYORG = 1 << 2;
        const REMAINS_TWINMOLD = 1 << 3;

        // Songs (bits 6-17)
        const SONG_AWAKENING = 1 << 6;    // Sonata of Awakening
        const SONG_GORON = 1 << 7;        // Goron Lullaby
        const SONG_ZORA = 1 << 8;         // New Wave Bossa Nova
        const SONG_EMPTINESS = 1 << 9;    // Elegy of Emptiness
        const SONG_ORDER = 1 << 10;       // Oath to Order
        const SONG_SARIA = 1 << 11;       // Saria's Song (unused in MM)
        const SONG_TIME = 1 << 12;        // Song of Time
        const SONG_HEALING = 1 << 13;     // Song of Healing
        const SONG_EPONA = 1 << 14;       // Epona's Song
        const SONG_SOARING = 1 << 15;     // Song of Soaring
        const SONG_STORMS = 1 << 16;      // Song of Storms
        const SONG_SUN = 1 << 17;         // Sun's Song (unused in MM)

        // Notebook (bit 18)
        const NOTEBOOK = 1 << 18;

        // Lullaby intro (bit 24)
        const SONG_LULLABY_INTRO = 1 << 24;

        // Heart pieces counter (bits 28-31)
        const HEART_PIECE_1 = 1 << 28;
        const HEART_PIECE_2 = 1 << 29;
        const HEART_PIECE_3 = 1 << 30;
        const HEART_PIECE_4 = 1 << 31;
    }
}

impl MmQuestItems {
    /// Returns the number of boss remains collected
    pub fn num_remains(&self) -> u8 {
        let mut count = 0;
        if self.contains(Self::REMAINS_ODOLWA) {
            count += 1;
        }
        if self.contains(Self::REMAINS_GOHT) {
            count += 1;
        }
        if self.contains(Self::REMAINS_GYORG) {
            count += 1;
        }
        if self.contains(Self::REMAINS_TWINMOLD) {
            count += 1;
        }
        count
    }

    /// Returns the heart piece count (0-3)
    pub fn heart_pieces(&self) -> u8 {
        ((self.bits() >> 28) & 0xF) as u8
    }
}

// ============================================================================
// Upgrades
// ============================================================================

bitflags! {
    /// Inventory upgrades (wallet, bags, etc.)
    #[derive(Default)]
    pub struct MmUpgrades: u32 {
        // Quiver (bits 0-2)
        const QUIVER_30 = 0x1;
        const QUIVER_40 = 0x2;
        const QUIVER_50 = 0x3;
        const QUIVER_MASK = 0x7;

        // Bomb bag (bits 3-5)
        const BOMB_BAG_20 = 0x8;
        const BOMB_BAG_30 = 0x10;
        const BOMB_BAG_40 = 0x18;
        const BOMB_BAG_MASK = 0x38;

        // Strength (bits 6-8) - unused in MM
        const STRENGTH_MASK = 0x1C0;

        // Scale (bits 9-11) - unused in MM
        const SCALE_MASK = 0xE00;

        // Wallet (bits 12-13)
        const ADULTS_WALLET = 0x1000;
        const GIANTS_WALLET = 0x2000;
        const WALLET_MASK = 0x3000;

        // Deku stick capacity (bits 17-19)
        const DEKU_STICK_10 = 0x20000;
        const DEKU_STICK_20 = 0x40000;
        const DEKU_STICK_30 = 0x60000;
        const DEKU_STICK_MASK = 0xE0000;

        // Deku nut capacity (bits 20-22)
        const DEKU_NUT_20 = 0x100000;
        const DEKU_NUT_30 = 0x200000;
        const DEKU_NUT_40 = 0x300000;
        const DEKU_NUT_MASK = 0x700000;
    }
}

impl MmUpgrades {
    pub fn wallet(&self) -> MmUpgrades {
        *self & MmUpgrades::WALLET_MASK
    }

    pub fn set_wallet(&mut self, wallet: MmUpgrades) {
        self.remove(MmUpgrades::WALLET_MASK);
        self.insert(wallet & MmUpgrades::WALLET_MASK);
    }

    pub fn bomb_bag(&self) -> MmUpgrades {
        *self & MmUpgrades::BOMB_BAG_MASK
    }

    pub fn quiver(&self) -> MmUpgrades {
        *self & MmUpgrades::QUIVER_MASK
    }
}

// ============================================================================
// Dungeon Items
// ============================================================================

bitflags! {
    /// Dungeon items for a single dungeon
    #[derive(Default)]
    pub struct MmDungeonItems: u8 {
        const BOSS_KEY = 0x01;
        const COMPASS = 0x02;
        const MAP = 0x04;
        // Note: maxKeys is stored in bits 3-7 but we handle that separately
    }
}

/// All dungeon items for MM's dungeons
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct MmAllDungeonItems {
    pub woodfall: MmDungeonItems,
    pub snowhead: MmDungeonItems,
    pub great_bay: MmDungeonItems,
    pub stone_tower: MmDungeonItems,
}

impl MmAllDungeonItems {
    /// Get dungeon items by dungeon index (0=Woodfall, 1=Snowhead, 2=Great Bay, 3=Stone Tower)
    pub fn get(&self, dungeon_idx: usize) -> MmDungeonItems {
        match dungeon_idx {
            0 => self.woodfall,
            1 => self.snowhead,
            2 => self.great_bay,
            3 => self.stone_tower,
            _ => MmDungeonItems::default(),
        }
    }
}

// ============================================================================
// Small Keys
// ============================================================================

/// Small key counts for each dungeon
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct MmSmallKeys {
    pub woodfall: u8,
    pub snowhead: u8,
    pub great_bay: u8,
    pub stone_tower: u8,
}

impl MmSmallKeys {
    /// Get small key count for Snowhead Temple
    pub fn snowhead(&self) -> u8 {
        self.snowhead
    }

    /// Get small key count for Great Bay Temple
    pub fn great_bay(&self) -> u8 {
        self.great_bay
    }

    /// Get small key count for Stone Tower Temple
    pub fn stone_tower(&self) -> u8 {
        self.stone_tower
    }
}

// ============================================================================
// Stray Fairies
// ============================================================================

/// Stray fairy counts for each dungeon (plus Clock Town)
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct MmStrayFairies {
    pub clock_town: u8,
    pub woodfall: u8,
    pub snowhead: u8,
    pub great_bay: u8,
    pub stone_tower: u8,
}

impl MmStrayFairies {
    /// Total stray fairies across all dungeons (excluding Clock Town)
    pub fn dungeon_total(&self) -> u8 {
        self.woodfall + self.snowhead + self.great_bay + self.stone_tower
    }
}

// ============================================================================
// Masks
// ============================================================================

bitflags! {
    /// Transformation masks
    #[derive(Default)]
    pub struct MmTransformationMasks: u8 {
        const DEKU = 0x01;
        const GORON = 0x02;
        const ZORA = 0x04;
        const FIERCE_DEITY = 0x08;
    }
}

bitflags! {
    /// Regular collectible masks (first 16)
    #[derive(Default)]
    pub struct MmMasksLow: u16 {
        const POSTMAN = 1 << 0;
        const ALL_NIGHT = 1 << 1;
        const BLAST = 1 << 2;
        const STONE = 1 << 3;
        const GREAT_FAIRY = 1 << 4;
        const KEATON = 1 << 5;
        const BREMEN = 1 << 6;
        const BUNNY = 1 << 7;
        const DON_GERO = 1 << 8;
        const SCENTS = 1 << 9;
        const ROMANI = 1 << 10;
        const CIRCUS_LEADER = 1 << 11;  // Troupe Leader's Mask
        const KAFEI = 1 << 12;
        const COUPLES = 1 << 13;
        const TRUTH = 1 << 14;
        const KAMARO = 1 << 15;
    }
}

bitflags! {
    /// Regular collectible masks (last 8)
    #[derive(Default)]
    pub struct MmMasksHigh: u8 {
        const GIBDO = 1 << 0;
        const GARO = 1 << 1;
        const CAPTAIN = 1 << 2;
        const GIANT = 1 << 3;
    }
}

/// All mask ownership state
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct MmMasks {
    pub transformation: MmTransformationMasks,
    pub masks_low: MmMasksLow,
    pub masks_high: MmMasksHigh,
}

impl MmMasks {
    /// Total number of masks collected (excluding transformation masks)
    pub fn regular_mask_count(&self) -> u8 {
        self.masks_low.bits().count_ones() as u8 + self.masks_high.bits().count_ones() as u8
    }

    /// Total masks including transformation masks
    pub fn total_mask_count(&self) -> u8 {
        self.regular_mask_count() + self.transformation.bits().count_ones() as u8
    }
}

// ============================================================================
// Inventory
// ============================================================================

/// MM inventory items (non-mask C-button items)
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct MmInventory {
    // Usable items
    pub ocarina: bool,
    pub bow: bool,
    pub fire_arrows: bool,
    pub ice_arrows: bool,
    pub light_arrows: bool,
    pub bombs: bool,
    pub bombchus: bool,
    pub deku_sticks: bool,
    pub deku_nuts: bool,
    pub magic_beans: bool,
    pub powder_keg: bool,
    pub pictograph_box: bool,
    pub lens: bool,
    pub hookshot: bool,
    pub great_fairy_sword: bool,

    // Bottles
    pub bottles: [MmBottle; 6],
}

/// Bottle contents
#[derive(Derivative, Debug, Clone, Copy, PartialEq, Eq)]
#[derivative(Default)]
pub enum MmBottle {
    #[derivative(Default)]
    None,
    Empty,
    RedPotion,
    GreenPotion,
    BluePotion,
    Fairy,
    DekuPrincess,
    Milk,
    MilkHalf,
    Fish,
    Bug,
    BlueFire,
    Poe,
    BigPoe,
    Water,
    HotSpringWater,
    ZoraEgg,
    GoldDust,
    MagicalMushroom,
    SeaHorse,
    ChateauRomani,
    MysteryMilk,
    MysteryMilkSpoiled,
}

impl TryFrom<u8> for MmBottle {
    type Error = u8;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        use mm_item_ids::*;
        match value {
            NONE => Ok(MmBottle::None),
            BOTTLE_EMPTY => Ok(MmBottle::Empty),
            BOTTLE_RED_POTION => Ok(MmBottle::RedPotion),
            BOTTLE_GREEN_POTION => Ok(MmBottle::GreenPotion),
            BOTTLE_BLUE_POTION => Ok(MmBottle::BluePotion),
            BOTTLE_FAIRY => Ok(MmBottle::Fairy),
            BOTTLE_DEKU_PRINCESS => Ok(MmBottle::DekuPrincess),
            BOTTLE_MILK => Ok(MmBottle::Milk),
            BOTTLE_MILK_HALF => Ok(MmBottle::MilkHalf),
            BOTTLE_FISH => Ok(MmBottle::Fish),
            BOTTLE_BUG => Ok(MmBottle::Bug),
            BOTTLE_BLUE_FIRE => Ok(MmBottle::BlueFire),
            BOTTLE_POE => Ok(MmBottle::Poe),
            BOTTLE_BIG_POE => Ok(MmBottle::BigPoe),
            BOTTLE_WATER => Ok(MmBottle::Water),
            BOTTLE_HOT_SPRING_WATER => Ok(MmBottle::HotSpringWater),
            BOTTLE_ZORA_EGG => Ok(MmBottle::ZoraEgg),
            BOTTLE_GOLD_DUST => Ok(MmBottle::GoldDust),
            BOTTLE_MUSHROOM => Ok(MmBottle::MagicalMushroom),
            BOTTLE_SEAHORSE => Ok(MmBottle::SeaHorse),
            BOTTLE_CHATEAU_ROMANI => Ok(MmBottle::ChateauRomani),
            BOTTLE_MYSTERY_MILK => Ok(MmBottle::MysteryMilk),
            BOTTLE_MYSTERY_MILK_SPOILED => Ok(MmBottle::MysteryMilkSpoiled),
            _ => Err(value),
        }
    }
}

impl From<MmBottle> for u8 {
    fn from(bottle: MmBottle) -> u8 {
        use mm_item_ids::*;
        match bottle {
            MmBottle::None => NONE,
            MmBottle::Empty => BOTTLE_EMPTY,
            MmBottle::RedPotion => BOTTLE_RED_POTION,
            MmBottle::GreenPotion => BOTTLE_GREEN_POTION,
            MmBottle::BluePotion => BOTTLE_BLUE_POTION,
            MmBottle::Fairy => BOTTLE_FAIRY,
            MmBottle::DekuPrincess => BOTTLE_DEKU_PRINCESS,
            MmBottle::Milk => BOTTLE_MILK,
            MmBottle::MilkHalf => BOTTLE_MILK_HALF,
            MmBottle::Fish => BOTTLE_FISH,
            MmBottle::Bug => BOTTLE_BUG,
            MmBottle::BlueFire => BOTTLE_BLUE_FIRE,
            MmBottle::Poe => BOTTLE_POE,
            MmBottle::BigPoe => BOTTLE_BIG_POE,
            MmBottle::Water => BOTTLE_WATER,
            MmBottle::HotSpringWater => BOTTLE_HOT_SPRING_WATER,
            MmBottle::ZoraEgg => BOTTLE_ZORA_EGG,
            MmBottle::GoldDust => BOTTLE_GOLD_DUST,
            MmBottle::MagicalMushroom => BOTTLE_MUSHROOM,
            MmBottle::SeaHorse => BOTTLE_SEAHORSE,
            MmBottle::ChateauRomani => BOTTLE_CHATEAU_ROMANI,
            MmBottle::MysteryMilk => BOTTLE_MYSTERY_MILK,
            MmBottle::MysteryMilkSpoiled => BOTTLE_MYSTERY_MILK_SPOILED,
        }
    }
}

// ============================================================================
// Scene Flags
// ============================================================================

/// Permanent scene flags for a single scene
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct MmPermanentSceneFlags {
    pub chest: u32,
    pub switch0: u32,
    pub switch1: u32,
    pub cleared_room: u32,
    pub collectible: u32,
    pub cleared_floors: u32,
    pub rooms: u32,
}

/// Cycle-based scene flags (reset on Song of Time)
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct MmCycleSceneFlags {
    pub chest: u32,
    pub switch0: u32,
    pub switch1: u32,
    pub cleared_room: u32,
    pub collectible: u32,
}

// ============================================================================
// Main Save Structure
// ============================================================================

/// Complete MM save state
#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct MmSave {
    // Player state
    pub player_form: PlayerForm,
    pub health_capacity: u16,
    pub health: u16,
    pub magic: MmMagicCapacity,
    pub double_defense: bool,
    pub rupees: u16,

    // Equipment
    pub sword: MmSword,
    pub shield: MmShield,

    // Items
    pub inventory: MmInventory,
    pub masks: MmMasks,
    pub upgrades: MmUpgrades,
    pub quest_items: MmQuestItems,

    // Dungeon progress
    pub dungeon_items: MmAllDungeonItems,
    pub small_keys: MmSmallKeys,
    pub stray_fairies: MmStrayFairies,

    // Skulltula tokens
    pub skull_tokens_swamp: u16,
    pub skull_tokens_ocean: u16,

    // Scene flags
    pub permanent_scene_flags: Vec<MmPermanentSceneFlags>,
    pub cycle_scene_flags: Vec<MmCycleSceneFlags>,

    // Time state
    pub day: u32,
    pub time: u16,
    pub is_night: bool,
}

impl MmSave {
    /// Converts Majora's Mask save data into an `MmSave`.
    ///
    /// # Arguments
    /// * `save_data` - Raw save data bytes (must be exactly MM_SIZE bytes)
    /// * `rom_type` - The type of ROM (vanilla or OoTMM) to determine offset layout
    ///
    /// # Errors
    /// Returns `MmDecodeError` if the data is invalid or wrong size.
    pub fn from_save_data_with_type(
        save_data: &[u8],
        rom_type: MmRomType,
    ) -> Result<MmSave, MmDecodeError> {
        match rom_type {
            MmRomType::Vanilla => Self::from_save_data(save_data),
            MmRomType::OoTMM => Self::from_save_data_ootmm(save_data),
        }
    }

    /// Converts Majora's Mask save data into an `MmSave` (vanilla offsets).
    ///
    /// # Arguments
    /// * `save_data` - Raw save data bytes (must be exactly MM_SIZE bytes)
    ///
    /// # Errors
    /// Returns `MmDecodeError` if the data is invalid or wrong size.
    pub fn from_save_data(save_data: &[u8]) -> Result<MmSave, MmDecodeError> {
        use offsets::*;

        // Helper macro to get a single byte at offset
        macro_rules! get_u8 {
            ($offset:expr) => {{
                *save_data
                    .get($offset)
                    .ok_or(MmDecodeError::Index($offset as u16))?
            }};
        }

        // Helper macro to get a u16 at offset (big endian)
        macro_rules! get_u16 {
            ($offset:expr) => {{
                let slice =
                    save_data
                        .get($offset..$offset + 2)
                        .ok_or(MmDecodeError::IndexRange {
                            start: $offset as u16,
                            end: ($offset + 2) as u16,
                        })?;
                BigEndian::read_u16(slice)
            }};
        }

        // Helper macro to get a u32 at offset (big endian)
        macro_rules! get_u32 {
            ($offset:expr) => {{
                let slice =
                    save_data
                        .get($offset..$offset + 4)
                        .ok_or(MmDecodeError::IndexRange {
                            start: $offset as u16,
                            end: ($offset + 4) as u16,
                        })?;
                BigEndian::read_u32(slice)
            }};
        }

        // Validate size
        if save_data.len() < MM_SIZE {
            return Err(MmDecodeError::Size(save_data.len()));
        }

        // Parse player form
        let player_form = PlayerForm::try_from(get_u8!(PLAYER_FORM)).unwrap_or(PlayerForm::Human);

        // Parse health
        let health_capacity = get_u16!(HEALTH_CAPACITY);
        let health = get_u16!(HEALTH);

        // Parse magic
        let magic =
            MmMagicCapacity::try_from(get_u8!(MAGIC_LEVEL)).unwrap_or(MmMagicCapacity::None);

        // Parse double defense
        let double_defense = get_u8!(DOUBLE_DEFENSE) != 0;

        // Parse rupees
        let rupees = get_u16!(RUPEES);

        // Parse sword and shield from equipment byte
        let sword_shield_byte = get_u8!(SWORD_SHIELD);
        let sword = MmSword::try_from(sword_shield_byte & 0x0F).unwrap_or(MmSword::None);
        let shield = MmShield::try_from((sword_shield_byte >> 4) & 0x0F).unwrap_or(MmShield::None);

        // Parse inventory
        let inventory = Self::parse_inventory(save_data)?;

        // Parse masks from mask inventory slots
        let masks = Self::parse_masks(save_data)?;

        // Parse quest items
        let quest_items = MmQuestItems::from_bits_truncate(get_u32!(QUEST_ITEMS));

        // Parse upgrades
        let upgrades = MmUpgrades::from_bits_truncate(get_u32!(UPGRADES));

        // Parse dungeon items
        let dungeon_items = MmAllDungeonItems {
            woodfall: MmDungeonItems::from_bits_truncate(get_u8!(DUNGEON_ITEMS)),
            snowhead: MmDungeonItems::from_bits_truncate(get_u8!(DUNGEON_ITEMS + 1)),
            great_bay: MmDungeonItems::from_bits_truncate(get_u8!(DUNGEON_ITEMS + 2)),
            stone_tower: MmDungeonItems::from_bits_truncate(get_u8!(DUNGEON_ITEMS + 3)),
        };

        // Parse small keys (handle 0xFF as 0)
        let parse_key = |offset: usize| -> u8 {
            let val = save_data.get(offset).copied().unwrap_or(0xFF);
            if val == 0xFF {
                0
            } else {
                val
            }
        };
        let small_keys = MmSmallKeys {
            woodfall: parse_key(SMALL_KEYS),
            snowhead: parse_key(SMALL_KEYS + 1),
            great_bay: parse_key(SMALL_KEYS + 2),
            stone_tower: parse_key(SMALL_KEYS + 3),
        };

        // Parse stray fairies
        let stray_fairies = MmStrayFairies {
            clock_town: get_u8!(STRAY_FAIRIES),
            woodfall: get_u8!(STRAY_FAIRIES + 1),
            snowhead: get_u8!(STRAY_FAIRIES + 2),
            great_bay: get_u8!(STRAY_FAIRIES + 3),
            stone_tower: get_u8!(STRAY_FAIRIES + 4),
        };

        // Parse skulltula tokens
        let skull_tokens_swamp = get_u16!(SKULL_SWAMP);
        let skull_tokens_ocean = get_u16!(SKULL_OCEAN);

        // Parse permanent scene flags (120 slots)
        let permanent_scene_flags = Self::parse_permanent_scene_flags(save_data)?;

        // Parse cycle scene flags (simplified - uses same structure but different offset)
        let cycle_scene_flags = Self::parse_cycle_scene_flags(save_data)?;

        // Parse time state
        let day = get_u32!(DAY);
        let time = get_u16!(TIME);
        let is_night = get_u8!(IS_NIGHT) != 0;

        Ok(MmSave {
            player_form,
            health_capacity,
            health,
            magic,
            double_defense,
            rupees,
            sword,
            shield,
            inventory,
            masks,
            upgrades,
            quest_items,
            dungeon_items,
            small_keys,
            stray_fairies,
            skull_tokens_swamp,
            skull_tokens_ocean,
            permanent_scene_flags,
            cycle_scene_flags,
            day,
            time,
            is_night,
        })
    }

    /// Parse inventory items from save data
    fn parse_inventory(save_data: &[u8]) -> Result<MmInventory, MmDecodeError> {
        use mm_item_ids::*;
        use offsets::*;

        let get_item =
            |offset: usize| -> u8 { save_data.get(INVENTORY + offset).copied().unwrap_or(NONE) };

        // Check presence of items by their inventory slot contents
        let ocarina = get_item(0) == OCARINA;
        let bow = get_item(1) == BOW;
        let fire_arrows = get_item(2) == FIRE_ARROW;
        let ice_arrows = get_item(3) == ICE_ARROW;
        let light_arrows = get_item(4) == LIGHT_ARROW;
        let bombs = get_item(6) == BOMB;
        let bombchus = get_item(7) == BOMBCHU;
        let deku_sticks = get_item(8) == DEKU_STICK;
        let deku_nuts = get_item(9) == DEKU_NUT;
        let magic_beans = get_item(10) == MAGIC_BEAN;
        let powder_keg = get_item(12) == POWDER_KEG;
        let pictograph_box = get_item(13) == PICTOGRAPH_BOX;
        let lens = get_item(14) == LENS;
        let hookshot = get_item(15) == HOOKSHOT;
        let great_fairy_sword = get_item(16) == GREAT_FAIRY_SWORD;

        // Parse bottles (slots 18-23 in inventory)
        let parse_bottle = |slot: usize| -> MmBottle {
            let val = get_item(slot);
            MmBottle::try_from(val).unwrap_or(MmBottle::None)
        };

        let bottles = [
            parse_bottle(18),
            parse_bottle(19),
            parse_bottle(20),
            parse_bottle(21),
            parse_bottle(22),
            parse_bottle(23),
        ];

        Ok(MmInventory {
            ocarina,
            bow,
            fire_arrows,
            ice_arrows,
            light_arrows,
            bombs,
            bombchus,
            deku_sticks,
            deku_nuts,
            magic_beans,
            powder_keg,
            pictograph_box,
            lens,
            hookshot,
            great_fairy_sword,
            bottles,
        })
    }

    /// Parse masks from the mask inventory slots
    fn parse_masks(save_data: &[u8]) -> Result<MmMasks, MmDecodeError> {
        use mm_item_ids::*;
        use offsets::*;

        let mut transformation = MmTransformationMasks::empty();
        let mut masks_low = MmMasksLow::empty();
        let mut masks_high = MmMasksHigh::empty();

        // Scan all mask slots and set flags based on what masks are present
        for i in 0..MASK_SLOTS {
            let mask_id = save_data.get(MASKS + i).copied().unwrap_or(NONE);
            match mask_id {
                MASK_DEKU => transformation.insert(MmTransformationMasks::DEKU),
                MASK_GORON => transformation.insert(MmTransformationMasks::GORON),
                MASK_ZORA => transformation.insert(MmTransformationMasks::ZORA),
                MASK_FIERCE_DEITY => transformation.insert(MmTransformationMasks::FIERCE_DEITY),
                MASK_POSTMAN => masks_low.insert(MmMasksLow::POSTMAN),
                MASK_ALL_NIGHT => masks_low.insert(MmMasksLow::ALL_NIGHT),
                MASK_BLAST => masks_low.insert(MmMasksLow::BLAST),
                MASK_STONE => masks_low.insert(MmMasksLow::STONE),
                MASK_GREAT_FAIRY => masks_low.insert(MmMasksLow::GREAT_FAIRY),
                MASK_KEATON => masks_low.insert(MmMasksLow::KEATON),
                MASK_BREMEN => masks_low.insert(MmMasksLow::BREMEN),
                MASK_BUNNY => masks_low.insert(MmMasksLow::BUNNY),
                MASK_DON_GERO => masks_low.insert(MmMasksLow::DON_GERO),
                MASK_SCENTS => masks_low.insert(MmMasksLow::SCENTS),
                MASK_ROMANI => masks_low.insert(MmMasksLow::ROMANI),
                MASK_CIRCUS_LEADER => masks_low.insert(MmMasksLow::CIRCUS_LEADER),
                MASK_KAFEI => masks_low.insert(MmMasksLow::KAFEI),
                MASK_COUPLES => masks_low.insert(MmMasksLow::COUPLES),
                MASK_TRUTH => masks_low.insert(MmMasksLow::TRUTH),
                MASK_KAMARO => masks_low.insert(MmMasksLow::KAMARO),
                MASK_GIBDO => masks_high.insert(MmMasksHigh::GIBDO),
                MASK_GARO => masks_high.insert(MmMasksHigh::GARO),
                MASK_CAPTAIN => masks_high.insert(MmMasksHigh::CAPTAIN),
                MASK_GIANT => masks_high.insert(MmMasksHigh::GIANT),
                _ => {}
            }
        }

        Ok(MmMasks {
            transformation,
            masks_low,
            masks_high,
        })
    }

    // ========================================================================
    // OoTMM-specific parsing functions
    // ========================================================================

    /// Converts OoTMM combo ROM save data into an `MmSave`.
    ///
    /// OoTMM has a different save structure layout than vanilla MM.
    /// Reference: https://github.com/OoTMM/OoTMM/blob/master/packages/core/include/combo/mm/save.h
    ///
    /// # Arguments
    /// * `save_data` - Raw save data bytes (must be exactly MM_SIZE bytes)
    ///
    /// # Errors
    /// Returns `MmDecodeError` if the data is invalid or wrong size.
    pub fn from_save_data_ootmm(save_data: &[u8]) -> Result<MmSave, MmDecodeError> {
        use ootmm_offsets::*;

        // Helper macro to get a single byte at offset
        macro_rules! get_u8 {
            ($offset:expr) => {{
                *save_data
                    .get($offset)
                    .ok_or(MmDecodeError::Index($offset as u16))?
            }};
        }

        // Helper macro to get a u16 at offset (big endian)
        macro_rules! get_u16 {
            ($offset:expr) => {{
                let slice =
                    save_data
                        .get($offset..$offset + 2)
                        .ok_or(MmDecodeError::IndexRange {
                            start: $offset as u16,
                            end: ($offset + 2) as u16,
                        })?;
                BigEndian::read_u16(slice)
            }};
        }

        // Helper macro to get a u32 at offset (big endian)
        macro_rules! get_u32 {
            ($offset:expr) => {{
                let slice =
                    save_data
                        .get($offset..$offset + 4)
                        .ok_or(MmDecodeError::IndexRange {
                            start: $offset as u16,
                            end: ($offset + 4) as u16,
                        })?;
                BigEndian::read_u32(slice)
            }};
        }

        // Validate size
        if save_data.len() < MM_SIZE {
            return Err(MmDecodeError::Size(save_data.len()));
        }

        // Parse player form (same offset as vanilla)
        let player_form = PlayerForm::try_from(get_u8!(PLAYER_FORM)).unwrap_or(PlayerForm::Human);

        // Parse health (different offset in OoTMM)
        let health_capacity = get_u16!(HEALTH_CAPACITY);
        let health = get_u16!(HEALTH);

        // Parse magic
        let magic =
            MmMagicCapacity::try_from(get_u8!(MAGIC_LEVEL)).unwrap_or(MmMagicCapacity::None);

        // Parse double defense
        let double_defense = get_u8!(DOUBLE_DEFENSE) != 0;

        // Parse rupees
        let rupees = get_u16!(RUPEES);

        // Parse sword and shield from equipment byte (different offset in OoTMM)
        let sword_shield_byte = get_u8!(SWORD_SHIELD);
        let sword = MmSword::try_from(sword_shield_byte & 0x0F).unwrap_or(MmSword::None);
        let shield = MmShield::try_from((sword_shield_byte >> 4) & 0x0F).unwrap_or(MmShield::None);

        // Parse inventory (OoTMM uses different offset)
        let inventory = Self::parse_inventory_ootmm(save_data)?;

        // Parse masks (OoTMM has masks in same array as items, starting at index 24)
        let masks = Self::parse_masks_ootmm(save_data)?;

        // Parse quest items (different offset in OoTMM)
        let quest_items = MmQuestItems::from_bits_truncate(get_u32!(QUEST_ITEMS));

        // Parse upgrades (different offset in OoTMM)
        let upgrades = MmUpgrades::from_bits_truncate(get_u32!(UPGRADES));

        // Parse dungeon items (OoTMM has 10 dungeons, we use first 4)
        let dungeon_items = MmAllDungeonItems {
            woodfall: MmDungeonItems::from_bits_truncate(get_u8!(DUNGEON_ITEMS)),
            snowhead: MmDungeonItems::from_bits_truncate(get_u8!(DUNGEON_ITEMS + 1)),
            great_bay: MmDungeonItems::from_bits_truncate(get_u8!(DUNGEON_ITEMS + 2)),
            stone_tower: MmDungeonItems::from_bits_truncate(get_u8!(DUNGEON_ITEMS + 3)),
        };

        // Parse small keys (OoTMM has 9 dungeons, handle 0xFF as 0)
        let parse_key = |offset: usize| -> u8 {
            let val = save_data.get(offset).copied().unwrap_or(0xFF);
            if val == 0xFF {
                0
            } else {
                val
            }
        };
        let small_keys = MmSmallKeys {
            woodfall: parse_key(SMALL_KEYS),
            snowhead: parse_key(SMALL_KEYS + 1),
            great_bay: parse_key(SMALL_KEYS + 2),
            stone_tower: parse_key(SMALL_KEYS + 3),
        };

        // Parse stray fairies (OoTMM has 10 areas, we use first 5)
        let stray_fairies = MmStrayFairies {
            clock_town: get_u8!(STRAY_FAIRIES),
            woodfall: get_u8!(STRAY_FAIRIES + 1),
            snowhead: get_u8!(STRAY_FAIRIES + 2),
            great_bay: get_u8!(STRAY_FAIRIES + 3),
            stone_tower: get_u8!(STRAY_FAIRIES + 4),
        };

        // Parse skulltula tokens
        let skull_tokens_swamp = get_u16!(SKULL_SWAMP);
        let skull_tokens_ocean = get_u16!(SKULL_OCEAN);

        // Parse permanent scene flags (same structure, potentially same offset)
        let permanent_scene_flags = Self::parse_permanent_scene_flags_ootmm(save_data)?;

        // Parse cycle scene flags
        let cycle_scene_flags = Self::parse_cycle_scene_flags_ootmm(save_data)?;

        // Parse time state (different offsets in OoTMM)
        let day = get_u32!(DAY);
        let time = get_u16!(TIME);
        let is_night = get_u32!(IS_NIGHT) != 0; // Note: IS_NIGHT is s32 in OoTMM

        Ok(MmSave {
            player_form,
            health_capacity,
            health,
            magic,
            double_defense,
            rupees,
            sword,
            shield,
            inventory,
            masks,
            upgrades,
            quest_items,
            dungeon_items,
            small_keys,
            stray_fairies,
            skull_tokens_swamp,
            skull_tokens_ocean,
            permanent_scene_flags,
            cycle_scene_flags,
            day,
            time,
            is_night,
        })
    }

    /// Parse inventory items from OoTMM save data
    fn parse_inventory_ootmm(save_data: &[u8]) -> Result<MmInventory, MmDecodeError> {
        use mm_item_ids::*;
        use ootmm_offsets::*;

        let get_item =
            |offset: usize| -> u8 { save_data.get(INVENTORY + offset).copied().unwrap_or(NONE) };

        // Check presence of items by their inventory slot contents
        // OoTMM uses same item IDs, just at different base offset
        let ocarina = get_item(0) == OCARINA;
        let bow = get_item(1) == BOW;
        let fire_arrows = get_item(2) == FIRE_ARROW;
        let ice_arrows = get_item(3) == ICE_ARROW;
        let light_arrows = get_item(4) == LIGHT_ARROW;
        let bombs = get_item(6) == BOMB;
        let bombchus = get_item(7) == BOMBCHU;
        let deku_sticks = get_item(8) == DEKU_STICK;
        let deku_nuts = get_item(9) == DEKU_NUT;
        let magic_beans = get_item(10) == MAGIC_BEAN;
        let powder_keg = get_item(12) == POWDER_KEG;
        let pictograph_box = get_item(13) == PICTOGRAPH_BOX;
        let lens = get_item(14) == LENS;
        let hookshot = get_item(15) == HOOKSHOT;
        let great_fairy_sword = get_item(16) == GREAT_FAIRY_SWORD;

        // Parse bottles (slots 18-23 in inventory)
        let parse_bottle = |slot: usize| -> MmBottle {
            let val = get_item(slot);
            MmBottle::try_from(val).unwrap_or(MmBottle::None)
        };

        let bottles = [
            parse_bottle(18),
            parse_bottle(19),
            parse_bottle(20),
            parse_bottle(21),
            parse_bottle(22),
            parse_bottle(23),
        ];

        Ok(MmInventory {
            ocarina,
            bow,
            fire_arrows,
            ice_arrows,
            light_arrows,
            bombs,
            bombchus,
            deku_sticks,
            deku_nuts,
            magic_beans,
            powder_keg,
            pictograph_box,
            lens,
            hookshot,
            great_fairy_sword,
            bottles,
        })
    }

    /// Parse masks from OoTMM save data
    ///
    /// In OoTMM, masks are stored in the same array as items, starting at index 24
    /// (INVENTORY + 24 = MASKS offset)
    fn parse_masks_ootmm(save_data: &[u8]) -> Result<MmMasks, MmDecodeError> {
        use mm_item_ids::*;
        use ootmm_offsets::*;

        let mut transformation = MmTransformationMasks::empty();
        let mut masks_low = MmMasksLow::empty();
        let mut masks_high = MmMasksHigh::empty();

        // In OoTMM, masks start at INVENTORY + 24 (= MASKS offset 0x86)
        // Scan all 24 mask slots
        for i in 0..MASK_SLOTS {
            let mask_id = save_data.get(MASKS + i).copied().unwrap_or(NONE);
            match mask_id {
                MASK_DEKU => transformation.insert(MmTransformationMasks::DEKU),
                MASK_GORON => transformation.insert(MmTransformationMasks::GORON),
                MASK_ZORA => transformation.insert(MmTransformationMasks::ZORA),
                MASK_FIERCE_DEITY => transformation.insert(MmTransformationMasks::FIERCE_DEITY),
                MASK_POSTMAN => masks_low.insert(MmMasksLow::POSTMAN),
                MASK_ALL_NIGHT => masks_low.insert(MmMasksLow::ALL_NIGHT),
                MASK_BLAST => masks_low.insert(MmMasksLow::BLAST),
                MASK_STONE => masks_low.insert(MmMasksLow::STONE),
                MASK_GREAT_FAIRY => masks_low.insert(MmMasksLow::GREAT_FAIRY),
                MASK_KEATON => masks_low.insert(MmMasksLow::KEATON),
                MASK_BREMEN => masks_low.insert(MmMasksLow::BREMEN),
                MASK_BUNNY => masks_low.insert(MmMasksLow::BUNNY),
                MASK_DON_GERO => masks_low.insert(MmMasksLow::DON_GERO),
                MASK_SCENTS => masks_low.insert(MmMasksLow::SCENTS),
                MASK_ROMANI => masks_low.insert(MmMasksLow::ROMANI),
                MASK_CIRCUS_LEADER => masks_low.insert(MmMasksLow::CIRCUS_LEADER),
                MASK_KAFEI => masks_low.insert(MmMasksLow::KAFEI),
                MASK_COUPLES => masks_low.insert(MmMasksLow::COUPLES),
                MASK_TRUTH => masks_low.insert(MmMasksLow::TRUTH),
                MASK_KAMARO => masks_low.insert(MmMasksLow::KAMARO),
                MASK_GIBDO => masks_high.insert(MmMasksHigh::GIBDO),
                MASK_GARO => masks_high.insert(MmMasksHigh::GARO),
                MASK_CAPTAIN => masks_high.insert(MmMasksHigh::CAPTAIN),
                MASK_GIANT => masks_high.insert(MmMasksHigh::GIANT),
                _ => {}
            }
        }

        Ok(MmMasks {
            transformation,
            masks_low,
            masks_high,
        })
    }

    /// Parse permanent scene flags for OoTMM (120 slots)
    fn parse_permanent_scene_flags_ootmm(
        save_data: &[u8],
    ) -> Result<Vec<MmPermanentSceneFlags>, MmDecodeError> {
        use ootmm_offsets::PERM_SCENE_FLAGS;

        let mut flags = Vec::with_capacity(MM_PERM_SCENE_COUNT);

        for i in 0..MM_PERM_SCENE_COUNT {
            let base = PERM_SCENE_FLAGS + (i * MM_PERM_SCENE_SIZE);

            let get_u32_at = |offset: usize| -> u32 {
                save_data
                    .get(base + offset..base + offset + 4)
                    .map(BigEndian::read_u32)
                    .unwrap_or(0)
            };

            flags.push(MmPermanentSceneFlags {
                chest: get_u32_at(0x00),
                switch0: get_u32_at(0x04),
                switch1: get_u32_at(0x08),
                cleared_room: get_u32_at(0x0c),
                collectible: get_u32_at(0x10),
                cleared_floors: get_u32_at(0x14),
                rooms: get_u32_at(0x18),
            });
        }

        Ok(flags)
    }

    /// Parse cycle scene flags for OoTMM (reset on Song of Time)
    fn parse_cycle_scene_flags_ootmm(
        save_data: &[u8],
    ) -> Result<Vec<MmCycleSceneFlags>, MmDecodeError> {
        use ootmm_offsets::CYCLE_SCENE_FLAGS;

        const CYCLE_SCENE_SIZE: usize = 0x14;
        const CYCLE_SCENE_COUNT: usize = 120;

        let mut flags = Vec::with_capacity(CYCLE_SCENE_COUNT);

        for i in 0..CYCLE_SCENE_COUNT {
            let base = CYCLE_SCENE_FLAGS + (i * CYCLE_SCENE_SIZE);

            let get_u32_at = |offset: usize| -> u32 {
                save_data
                    .get(base + offset..base + offset + 4)
                    .map(BigEndian::read_u32)
                    .unwrap_or(0)
            };

            flags.push(MmCycleSceneFlags {
                chest: get_u32_at(0x00),
                switch0: get_u32_at(0x04),
                switch1: get_u32_at(0x08),
                cleared_room: get_u32_at(0x0c),
                collectible: get_u32_at(0x10),
            });
        }

        Ok(flags)
    }

    /// Parse permanent scene flags (120 slots)
    fn parse_permanent_scene_flags(
        save_data: &[u8],
    ) -> Result<Vec<MmPermanentSceneFlags>, MmDecodeError> {
        use offsets::PERM_SCENE_FLAGS;

        let mut flags = Vec::with_capacity(MM_PERM_SCENE_COUNT);

        for i in 0..MM_PERM_SCENE_COUNT {
            let base = PERM_SCENE_FLAGS + (i * MM_PERM_SCENE_SIZE);

            // Each scene flag entry is 0x1c bytes (7 u32s)
            let get_u32_at = |offset: usize| -> u32 {
                save_data
                    .get(base + offset..base + offset + 4)
                    .map(BigEndian::read_u32)
                    .unwrap_or(0)
            };

            flags.push(MmPermanentSceneFlags {
                chest: get_u32_at(0x00),
                switch0: get_u32_at(0x04),
                switch1: get_u32_at(0x08),
                cleared_room: get_u32_at(0x0c),
                collectible: get_u32_at(0x10),
                cleared_floors: get_u32_at(0x14),
                rooms: get_u32_at(0x18),
            });
        }

        Ok(flags)
    }

    /// Parse cycle scene flags (reset on Song of Time)
    fn parse_cycle_scene_flags(save_data: &[u8]) -> Result<Vec<MmCycleSceneFlags>, MmDecodeError> {
        use offsets::CYCLE_SCENE_FLAGS;

        // Cycle flags have fewer entries and smaller structure (5 u32s per entry)
        const CYCLE_SCENE_SIZE: usize = 0x14;
        const CYCLE_SCENE_COUNT: usize = 120;

        let mut flags = Vec::with_capacity(CYCLE_SCENE_COUNT);

        for i in 0..CYCLE_SCENE_COUNT {
            let base = CYCLE_SCENE_FLAGS + (i * CYCLE_SCENE_SIZE);

            let get_u32_at = |offset: usize| -> u32 {
                save_data
                    .get(base + offset..base + offset + 4)
                    .map(BigEndian::read_u32)
                    .unwrap_or(0)
            };

            flags.push(MmCycleSceneFlags {
                chest: get_u32_at(0x00),
                switch0: get_u32_at(0x04),
                switch1: get_u32_at(0x08),
                cleared_room: get_u32_at(0x0c),
                collectible: get_u32_at(0x10),
            });
        }

        Ok(flags)
    }

    /// Convert the save state back to raw bytes
    pub fn to_save_data(&self) -> Vec<u8> {
        use offsets::*;

        let mut buf = vec![0u8; MM_SIZE];

        // Write player form
        buf[PLAYER_FORM] = self.player_form as u8;

        // Write health
        buf[HEALTH_CAPACITY..HEALTH_CAPACITY + 2]
            .copy_from_slice(&self.health_capacity.to_be_bytes());
        buf[HEALTH..HEALTH + 2].copy_from_slice(&self.health.to_be_bytes());

        // Write magic
        buf[MAGIC_LEVEL] = self.magic as u8;

        // Write double defense
        buf[DOUBLE_DEFENSE] = if self.double_defense { 1 } else { 0 };

        // Write rupees
        buf[RUPEES..RUPEES + 2].copy_from_slice(&self.rupees.to_be_bytes());

        // Write sword and shield
        buf[SWORD_SHIELD] = (self.sword as u8) | ((self.shield as u8) << 4);

        // Write quest items
        buf[QUEST_ITEMS..QUEST_ITEMS + 4].copy_from_slice(&self.quest_items.bits().to_be_bytes());

        // Write upgrades
        buf[UPGRADES..UPGRADES + 4].copy_from_slice(&self.upgrades.bits().to_be_bytes());

        // Write dungeon items
        buf[DUNGEON_ITEMS] = self.dungeon_items.woodfall.bits();
        buf[DUNGEON_ITEMS + 1] = self.dungeon_items.snowhead.bits();
        buf[DUNGEON_ITEMS + 2] = self.dungeon_items.great_bay.bits();
        buf[DUNGEON_ITEMS + 3] = self.dungeon_items.stone_tower.bits();

        // Write small keys
        buf[SMALL_KEYS] = self.small_keys.woodfall;
        buf[SMALL_KEYS + 1] = self.small_keys.snowhead;
        buf[SMALL_KEYS + 2] = self.small_keys.great_bay;
        buf[SMALL_KEYS + 3] = self.small_keys.stone_tower;

        // Write stray fairies
        buf[STRAY_FAIRIES] = self.stray_fairies.clock_town;
        buf[STRAY_FAIRIES + 1] = self.stray_fairies.woodfall;
        buf[STRAY_FAIRIES + 2] = self.stray_fairies.snowhead;
        buf[STRAY_FAIRIES + 3] = self.stray_fairies.great_bay;
        buf[STRAY_FAIRIES + 4] = self.stray_fairies.stone_tower;

        // Write skulltula tokens
        buf[SKULL_SWAMP..SKULL_SWAMP + 2].copy_from_slice(&self.skull_tokens_swamp.to_be_bytes());
        buf[SKULL_OCEAN..SKULL_OCEAN + 2].copy_from_slice(&self.skull_tokens_ocean.to_be_bytes());

        // Write time state
        buf[DAY..DAY + 4].copy_from_slice(&self.day.to_be_bytes());
        buf[TIME..TIME + 2].copy_from_slice(&self.time.to_be_bytes());
        buf[IS_NIGHT] = if self.is_night { 1 } else { 0 };

        // Write permanent scene flags
        for (i, scene) in self.permanent_scene_flags.iter().enumerate() {
            let base = PERM_SCENE_FLAGS + (i * MM_PERM_SCENE_SIZE);
            buf[base..base + 4].copy_from_slice(&scene.chest.to_be_bytes());
            buf[base + 4..base + 8].copy_from_slice(&scene.switch0.to_be_bytes());
            buf[base + 8..base + 12].copy_from_slice(&scene.switch1.to_be_bytes());
            buf[base + 12..base + 16].copy_from_slice(&scene.cleared_room.to_be_bytes());
            buf[base + 16..base + 20].copy_from_slice(&scene.collectible.to_be_bytes());
            buf[base + 20..base + 24].copy_from_slice(&scene.cleared_floors.to_be_bytes());
            buf[base + 24..base + 28].copy_from_slice(&scene.rooms.to_be_bytes());
        }

        buf
    }

    // ========================================================================
    // Heart Accessors
    // ========================================================================

    /// Returns the number of heart containers (full hearts).
    /// MM starts with 3 hearts, max is 20.
    pub fn heart_containers(&self) -> u8 {
        // health_capacity is in 16ths of a heart (0x10 per heart)
        (self.health_capacity / 0x10) as u8
    }

    // ========================================================================
    // Transformation Mask Accessors
    // ========================================================================

    /// Returns true if the player has the Deku Mask
    pub fn has_deku_mask(&self) -> bool {
        self.masks
            .transformation
            .contains(MmTransformationMasks::DEKU)
    }

    /// Returns true if the player has the Goron Mask
    pub fn has_goron_mask(&self) -> bool {
        self.masks
            .transformation
            .contains(MmTransformationMasks::GORON)
    }

    /// Returns true if the player has the Zora Mask
    pub fn has_zora_mask(&self) -> bool {
        self.masks
            .transformation
            .contains(MmTransformationMasks::ZORA)
    }

    /// Returns true if the player has the Fierce Deity Mask
    pub fn has_fierce_deity_mask(&self) -> bool {
        self.masks
            .transformation
            .contains(MmTransformationMasks::FIERCE_DEITY)
    }

    // ========================================================================
    // Collectible Mask Accessors
    // ========================================================================

    /// Returns true if the player has the Postman's Hat
    pub fn has_postman_hat(&self) -> bool {
        self.masks.masks_low.contains(MmMasksLow::POSTMAN)
    }

    /// Returns true if the player has the All-Night Mask
    pub fn has_all_night_mask(&self) -> bool {
        self.masks.masks_low.contains(MmMasksLow::ALL_NIGHT)
    }

    /// Returns true if the player has the Blast Mask
    pub fn has_blast_mask(&self) -> bool {
        self.masks.masks_low.contains(MmMasksLow::BLAST)
    }

    /// Returns true if the player has the Stone Mask
    pub fn has_stone_mask(&self) -> bool {
        self.masks.masks_low.contains(MmMasksLow::STONE)
    }

    /// Returns true if the player has the Great Fairy Mask
    pub fn has_great_fairy_mask(&self) -> bool {
        self.masks.masks_low.contains(MmMasksLow::GREAT_FAIRY)
    }

    /// Returns true if the player has the Keaton Mask
    pub fn has_keaton_mask(&self) -> bool {
        self.masks.masks_low.contains(MmMasksLow::KEATON)
    }

    /// Returns true if the player has the Bremen Mask
    pub fn has_bremen_mask(&self) -> bool {
        self.masks.masks_low.contains(MmMasksLow::BREMEN)
    }

    /// Returns true if the player has the Bunny Hood
    pub fn has_bunny_hood(&self) -> bool {
        self.masks.masks_low.contains(MmMasksLow::BUNNY)
    }

    /// Returns true if the player has Don Gero's Mask
    pub fn has_don_gero_mask(&self) -> bool {
        self.masks.masks_low.contains(MmMasksLow::DON_GERO)
    }

    /// Returns true if the player has the Mask of Scents
    pub fn has_mask_of_scents(&self) -> bool {
        self.masks.masks_low.contains(MmMasksLow::SCENTS)
    }

    /// Returns true if the player has Romani's Mask
    pub fn has_romani_mask(&self) -> bool {
        self.masks.masks_low.contains(MmMasksLow::ROMANI)
    }

    /// Returns true if the player has the Circus Leader's Mask (Troupe Leader's Mask)
    pub fn has_circus_leader_mask(&self) -> bool {
        self.masks.masks_low.contains(MmMasksLow::CIRCUS_LEADER)
    }

    /// Returns true if the player has Kafei's Mask
    pub fn has_kafei_mask(&self) -> bool {
        self.masks.masks_low.contains(MmMasksLow::KAFEI)
    }

    /// Returns true if the player has the Couple's Mask
    pub fn has_couples_mask(&self) -> bool {
        self.masks.masks_low.contains(MmMasksLow::COUPLES)
    }

    /// Returns true if the player has the Mask of Truth
    pub fn has_mask_of_truth(&self) -> bool {
        self.masks.masks_low.contains(MmMasksLow::TRUTH)
    }

    /// Returns true if the player has Kamaro's Mask
    pub fn has_kamaro_mask(&self) -> bool {
        self.masks.masks_low.contains(MmMasksLow::KAMARO)
    }

    /// Returns true if the player has the Gibdo Mask
    pub fn has_gibdo_mask(&self) -> bool {
        self.masks.masks_high.contains(MmMasksHigh::GIBDO)
    }

    /// Returns true if the player has the Garo's Mask
    pub fn has_garo_mask(&self) -> bool {
        self.masks.masks_high.contains(MmMasksHigh::GARO)
    }

    /// Returns true if the player has the Captain's Hat
    pub fn has_captain_hat(&self) -> bool {
        self.masks.masks_high.contains(MmMasksHigh::CAPTAIN)
    }

    /// Returns true if the player has the Giant's Mask
    pub fn has_giant_mask(&self) -> bool {
        self.masks.masks_high.contains(MmMasksHigh::GIANT)
    }

    // ========================================================================
    // Equipment Accessor Methods
    // ========================================================================

    /// Check if player has the Ocarina of Time
    pub fn has_ocarina(&self) -> bool {
        self.inventory.ocarina
    }

    /// Check if player has the Hero's Bow
    pub fn has_heros_bow(&self) -> bool {
        self.inventory.bow
    }

    /// Check if player has Fire Arrows
    pub fn has_fire_arrow(&self) -> bool {
        self.inventory.fire_arrows
    }

    /// Check if player has Ice Arrows
    pub fn has_ice_arrow(&self) -> bool {
        self.inventory.ice_arrows
    }

    /// Check if player has Light Arrows
    pub fn has_light_arrow(&self) -> bool {
        self.inventory.light_arrows
    }

    /// Check if player has the Hookshot
    pub fn has_hookshot(&self) -> bool {
        self.inventory.hookshot
    }

    /// Check if player has Bombs
    pub fn has_bombs(&self) -> bool {
        self.inventory.bombs
    }

    /// Check if player has Bombchus
    pub fn has_bombchu(&self) -> bool {
        self.inventory.bombchus
    }

    /// Check if player has Powder Kegs
    pub fn has_powder_keg(&self) -> bool {
        self.inventory.powder_keg
    }

    /// Check if player has the Lens of Truth
    pub fn has_lens_of_truth(&self) -> bool {
        self.inventory.lens
    }

    /// Check if player has the Pictograph Box
    pub fn has_pictograph_box(&self) -> bool {
        self.inventory.pictograph_box
    }

    /// Check if player has the Great Fairy's Sword
    pub fn has_great_fairy_sword(&self) -> bool {
        self.inventory.great_fairy_sword
    }

    /// Check if player has Magic Beans
    pub fn has_magic_bean(&self) -> bool {
        self.inventory.magic_beans
    }

    /// Check if player has magic (single or double)
    pub fn has_magic(&self) -> bool {
        self.magic != MmMagicCapacity::None
    }

    // ========================================================================
    // Song Accessor Methods
    // ========================================================================

    /// Check if player has Song of Time
    pub fn has_song_of_time(&self) -> bool {
        self.quest_items.contains(MmQuestItems::SONG_TIME)
    }

    /// Check if player has Song of Healing
    pub fn has_song_of_healing(&self) -> bool {
        self.quest_items.contains(MmQuestItems::SONG_HEALING)
    }

    /// Check if player has Epona's Song
    pub fn has_eponas_song(&self) -> bool {
        self.quest_items.contains(MmQuestItems::SONG_EPONA)
    }

    /// Check if player has Song of Soaring
    pub fn has_song_of_soaring(&self) -> bool {
        self.quest_items.contains(MmQuestItems::SONG_SOARING)
    }

    /// Check if player has Song of Storms
    pub fn has_song_of_storms(&self) -> bool {
        self.quest_items.contains(MmQuestItems::SONG_STORMS)
    }

    /// Check if player has Sonata of Awakening
    pub fn has_sonata_of_awakening(&self) -> bool {
        self.quest_items.contains(MmQuestItems::SONG_AWAKENING)
    }

    /// Check if player has Goron Lullaby
    pub fn has_goron_lullaby(&self) -> bool {
        self.quest_items.contains(MmQuestItems::SONG_GORON)
    }

    /// Check if player has New Wave Bossa Nova
    pub fn has_new_wave_bossa_nova(&self) -> bool {
        self.quest_items.contains(MmQuestItems::SONG_ZORA)
    }

    /// Check if player has Elegy of Emptiness
    pub fn has_elegy_of_emptiness(&self) -> bool {
        self.quest_items.contains(MmQuestItems::SONG_EMPTINESS)
    }

    /// Check if player has Oath to Order
    pub fn has_oath_to_order(&self) -> bool {
        self.quest_items.contains(MmQuestItems::SONG_ORDER)
    }

    // ========================================================================
    // Boss Remains Accessor Methods
    // ========================================================================

    /// Check if player has Odolwa's Remains
    pub fn has_odolwa_remains(&self) -> bool {
        self.quest_items.contains(MmQuestItems::REMAINS_ODOLWA)
    }

    /// Check if player has Goht's Remains
    pub fn has_goht_remains(&self) -> bool {
        self.quest_items.contains(MmQuestItems::REMAINS_GOHT)
    }

    /// Check if player has Gyorg's Remains
    pub fn has_gyorg_remains(&self) -> bool {
        self.quest_items.contains(MmQuestItems::REMAINS_GYORG)
    }

    /// Check if player has Twinmold's Remains
    pub fn has_twinmold_remains(&self) -> bool {
        self.quest_items.contains(MmQuestItems::REMAINS_TWINMOLD)
    }

    // ========================================================================
    // Bomber's Notebook Accessor Methods
    // ========================================================================

    /// Check if player has the Bomber's Notebook
    pub fn has_bombers_notebook(&self) -> bool {
        self.quest_items.contains(MmQuestItems::NOTEBOOK)
    }
}

// ============================================================================
// Real Memory Reader Implementation
// ============================================================================

/// Real implementation that reads from actual N64 memory
#[derive(Debug, Clone)]
pub struct MmSaveReader {
    save: MmSave,
    game_mode: MmGameMode,
}

impl MmSaveReader {
    /// Create a new reader from raw save data
    pub fn from_bytes(data: &[u8]) -> Result<Self, MmDecodeError> {
        let rom_type = MmRomType::from_env();
        let save = MmSave::from_save_data_with_type(data, rom_type)?;
        Ok(Self {
            save,
            game_mode: MmGameMode::Gameplay,
        })
    }

    /// Update the game mode
    pub fn set_game_mode(&mut self, mode: MmGameMode) {
        self.game_mode = mode;
    }

    /// Update save from new data
    pub fn update(&mut self, data: &[u8]) -> Result<(), MmDecodeError> {
        let rom_type = MmRomType::from_env();
        self.save = MmSave::from_save_data_with_type(data, rom_type)?;
        Ok(())
    }
}

impl MmSaveData for MmSaveReader {
    fn get_save(&self) -> &MmSave {
        &self.save
    }

    fn game_mode(&self) -> MmGameMode {
        self.game_mode
    }
}

// ============================================================================
// Game Mode
// ============================================================================

/// MM game mode states
#[derive(Derivative, Debug, Clone, Copy, PartialEq, Eq)]
#[derivative(Default)]
pub enum MmGameMode {
    #[derivative(Default)]
    Gameplay = 0,
    TitleScreen = 1,
    FileSelect = 2,
    EndCredits = 3,
    OwlSave = 4,
}

impl TryFrom<u32> for MmGameMode {
    type Error = u32;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(MmGameMode::Gameplay),
            1 => Ok(MmGameMode::TitleScreen),
            2 => Ok(MmGameMode::FileSelect),
            3 => Ok(MmGameMode::EndCredits),
            4 => Ok(MmGameMode::OwlSave),
            _ => Err(value),
        }
    }
}

// ============================================================================
// Trait Definition
// ============================================================================

/// Trait for accessing MM save data
///
/// This trait allows for both real memory reading and stub implementations,
/// enabling parallel development of UI and memory reading code.
pub trait MmSaveData {
    /// Get the current MM save state
    fn get_save(&self) -> &MmSave;

    /// Get the current game mode
    fn game_mode(&self) -> MmGameMode;

    /// Check if player has a specific transformation mask
    fn has_transformation_mask(&self, mask: MmTransformationMasks) -> bool {
        self.get_save().masks.transformation.contains(mask)
    }

    /// Check if player has a specific boss remains
    fn has_remains(&self, remains: MmQuestItems) -> bool {
        self.get_save().quest_items.contains(remains)
    }

    /// Get total stray fairies for a dungeon
    fn stray_fairy_count(&self, dungeon_idx: usize) -> u8 {
        let fairies = &self.get_save().stray_fairies;
        match dungeon_idx {
            0 => fairies.woodfall,
            1 => fairies.snowhead,
            2 => fairies.great_bay,
            3 => fairies.stone_tower,
            _ => 0,
        }
    }

    /// Check if a dungeon has all 15 stray fairies
    fn dungeon_fairies_complete(&self, dungeon_idx: usize) -> bool {
        self.stray_fairy_count(dungeon_idx) >= 15
    }
}

// ============================================================================
// Stub Implementation
// ============================================================================

/// Stub implementation with mock data for UI development
#[derive(Debug, Clone)]
pub struct MmSaveStub {
    save: MmSave,
    game_mode: MmGameMode,
}

impl Default for MmSaveStub {
    fn default() -> Self {
        Self::new()
    }
}

impl MmSaveStub {
    /// Create a new stub with default (empty) state
    pub fn new() -> Self {
        Self {
            save: MmSave::default(),
            game_mode: MmGameMode::Gameplay,
        }
    }

    /// Create a stub with sample data for testing UI
    pub fn with_sample_data() -> Self {
        let mut save = MmSave::default();

        // Sample inventory
        save.inventory.ocarina = true;
        save.inventory.bow = true;
        save.inventory.hookshot = true;
        save.inventory.bombs = true;
        save.inventory.lens = true;
        save.inventory.bottles = [
            MmBottle::Empty,
            MmBottle::RedPotion,
            MmBottle::ChateauRomani,
            MmBottle::None,
            MmBottle::None,
            MmBottle::None,
        ];

        // Sample masks
        save.masks.transformation = MmTransformationMasks::DEKU
            | MmTransformationMasks::GORON
            | MmTransformationMasks::ZORA;
        save.masks.masks_low =
            MmMasksLow::BUNNY | MmMasksLow::STONE | MmMasksLow::GREAT_FAIRY | MmMasksLow::BREMEN;

        // Sample quest items
        save.quest_items = MmQuestItems::REMAINS_ODOLWA
            | MmQuestItems::REMAINS_GOHT
            | MmQuestItems::SONG_HEALING
            | MmQuestItems::SONG_TIME
            | MmQuestItems::SONG_SOARING
            | MmQuestItems::SONG_EPONA
            | MmQuestItems::SONG_AWAKENING
            | MmQuestItems::NOTEBOOK;

        // Sample upgrades
        save.upgrades = MmUpgrades::ADULTS_WALLET | MmUpgrades::BOMB_BAG_30 | MmUpgrades::QUIVER_40;

        // Sample sword/shield
        save.sword = MmSword::GildedSword;
        save.shield = MmShield::MirrorShield;

        // Sample magic/health
        save.magic = MmMagicCapacity::Double;
        save.health_capacity = 0x140; // 5 hearts
        save.health = 0x100; // 4 hearts
        save.double_defense = true;

        // Sample dungeon progress
        save.dungeon_items.woodfall =
            MmDungeonItems::MAP | MmDungeonItems::COMPASS | MmDungeonItems::BOSS_KEY;
        save.dungeon_items.snowhead =
            MmDungeonItems::MAP | MmDungeonItems::COMPASS | MmDungeonItems::BOSS_KEY;
        save.small_keys.woodfall = 1;
        save.small_keys.snowhead = 3;

        // Sample stray fairies
        save.stray_fairies.clock_town = 1;
        save.stray_fairies.woodfall = 15;
        save.stray_fairies.snowhead = 8;
        save.stray_fairies.great_bay = 3;

        // Sample skulltulas
        save.skull_tokens_swamp = 18;
        save.skull_tokens_ocean = 7;

        // Time state
        save.day = 2;
        save.time = 0x8000; // Noon-ish
        save.is_night = false;

        Self {
            save,
            game_mode: MmGameMode::Gameplay,
        }
    }

    /// Get mutable access to the save for testing
    pub fn save_mut(&mut self) -> &mut MmSave {
        &mut self.save
    }

    /// Set the game mode
    pub fn set_game_mode(&mut self, mode: MmGameMode) {
        self.game_mode = mode;
    }
}

impl MmSaveData for MmSaveStub {
    fn get_save(&self) -> &MmSave {
        &self.save
    }

    fn game_mode(&self) -> MmGameMode {
        self.game_mode
    }
}

// ============================================================================
// Protocol Implementation for MmSave
// ============================================================================

impl async_proto::Protocol for MmSave {
    fn read<'a, R: tokio::io::AsyncRead + Unpin + Send + 'a>(
        stream: &'a mut R,
    ) -> std::pin::Pin<
        Box<dyn std::future::Future<Output = Result<Self, async_proto::ReadError>> + Send + 'a>,
    > {
        Box::pin(async move {
            use tokio::io::AsyncReadExt;
            let mut buf = vec![0u8; MM_SIZE];
            stream.read_exact(&mut buf).await?;
            let rom_type = MmRomType::from_env();
            MmSave::from_save_data_with_type(&buf, rom_type)
                .map_err(|e| async_proto::ReadError::Custom(format!("MM decode error: {:?}", e)))
        })
    }

    fn write<'a, W: tokio::io::AsyncWrite + Unpin + Send + 'a>(
        &'a self,
        sink: &'a mut W,
    ) -> std::pin::Pin<
        Box<dyn std::future::Future<Output = Result<(), async_proto::WriteError>> + Send + 'a>,
    > {
        Box::pin(async move {
            use tokio::io::AsyncWriteExt;
            let data = self.to_save_data();
            sink.write_all(&data).await?;
            Ok(())
        })
    }

    fn read_sync(stream: &mut impl std::io::Read) -> Result<Self, async_proto::ReadError> {
        let mut buf = vec![0u8; MM_SIZE];
        stream.read_exact(&mut buf)?;
        let rom_type = MmRomType::from_env();
        MmSave::from_save_data_with_type(&buf, rom_type)
            .map_err(|e| async_proto::ReadError::Custom(format!("MM decode error: {:?}", e)))
    }

    fn write_sync(&self, sink: &mut impl std::io::Write) -> Result<(), async_proto::WriteError> {
        let data = self.to_save_data();
        sink.write_all(&data)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_player_form_conversion() {
        assert_eq!(PlayerForm::try_from(0), Ok(PlayerForm::FierceDeity));
        assert_eq!(PlayerForm::try_from(1), Ok(PlayerForm::Goron));
        assert_eq!(PlayerForm::try_from(2), Ok(PlayerForm::Zora));
        assert_eq!(PlayerForm::try_from(3), Ok(PlayerForm::Deku));
        assert_eq!(PlayerForm::try_from(4), Ok(PlayerForm::Human));
        assert_eq!(PlayerForm::try_from(5), Err(5));
    }

    #[test]
    fn test_quest_items_remains_count() {
        let mut quest = MmQuestItems::empty();
        assert_eq!(quest.num_remains(), 0);

        quest.insert(MmQuestItems::REMAINS_ODOLWA);
        assert_eq!(quest.num_remains(), 1);

        quest.insert(MmQuestItems::REMAINS_GOHT);
        quest.insert(MmQuestItems::REMAINS_GYORG);
        assert_eq!(quest.num_remains(), 3);

        quest.insert(MmQuestItems::REMAINS_TWINMOLD);
        assert_eq!(quest.num_remains(), 4);
    }

    #[test]
    fn test_mask_count() {
        let mut masks = MmMasks::default();
        assert_eq!(masks.total_mask_count(), 0);

        masks.transformation = MmTransformationMasks::DEKU | MmTransformationMasks::GORON;
        assert_eq!(masks.total_mask_count(), 2);

        masks.masks_low = MmMasksLow::BUNNY | MmMasksLow::STONE | MmMasksLow::KEATON;
        assert_eq!(masks.regular_mask_count(), 3);
        assert_eq!(masks.total_mask_count(), 5);
    }

    #[test]
    fn test_stray_fairy_dungeon_total() {
        let mut fairies = MmStrayFairies::default();
        assert_eq!(fairies.dungeon_total(), 0);

        fairies.woodfall = 15;
        fairies.snowhead = 10;
        fairies.great_bay = 5;
        fairies.stone_tower = 0;

        assert_eq!(fairies.dungeon_total(), 30);
    }

    #[test]
    fn test_stub_default() {
        let stub = MmSaveStub::new();
        assert_eq!(stub.game_mode(), MmGameMode::Gameplay);
        assert!(!stub.get_save().inventory.ocarina);
        assert_eq!(
            stub.get_save().masks.transformation,
            MmTransformationMasks::empty()
        );
    }

    #[test]
    fn test_stub_sample_data() {
        let stub = MmSaveStub::with_sample_data();
        let save = stub.get_save();

        // Check sample inventory
        assert!(save.inventory.ocarina);
        assert!(save.inventory.bow);
        assert!(save.inventory.hookshot);

        // Check sample masks
        assert!(save
            .masks
            .transformation
            .contains(MmTransformationMasks::DEKU));
        assert!(save
            .masks
            .transformation
            .contains(MmTransformationMasks::GORON));
        assert!(save
            .masks
            .transformation
            .contains(MmTransformationMasks::ZORA));
        assert!(!save
            .masks
            .transformation
            .contains(MmTransformationMasks::FIERCE_DEITY));

        // Check remains
        assert!(save.quest_items.contains(MmQuestItems::REMAINS_ODOLWA));
        assert!(save.quest_items.contains(MmQuestItems::REMAINS_GOHT));
        assert!(!save.quest_items.contains(MmQuestItems::REMAINS_GYORG));
        assert_eq!(save.quest_items.num_remains(), 2);

        // Check songs
        assert!(save.quest_items.contains(MmQuestItems::SONG_HEALING));
        assert!(save.quest_items.contains(MmQuestItems::SONG_TIME));
        assert!(save.quest_items.contains(MmQuestItems::SONG_SOARING));

        // Check stray fairies
        assert_eq!(save.stray_fairies.woodfall, 15);
        assert!(stub.dungeon_fairies_complete(0)); // Woodfall
        assert!(!stub.dungeon_fairies_complete(1)); // Snowhead (8/15)
    }

    #[test]
    fn test_trait_methods() {
        let stub = MmSaveStub::with_sample_data();

        assert!(stub.has_transformation_mask(MmTransformationMasks::GORON));
        assert!(!stub.has_transformation_mask(MmTransformationMasks::FIERCE_DEITY));

        assert!(stub.has_remains(MmQuestItems::REMAINS_ODOLWA));
        assert!(!stub.has_remains(MmQuestItems::REMAINS_TWINMOLD));

        assert_eq!(stub.stray_fairy_count(0), 15);
        assert_eq!(stub.stray_fairy_count(1), 8);
        assert_eq!(stub.stray_fairy_count(2), 3);
        assert_eq!(stub.stray_fairy_count(3), 0);
    }

    #[test]
    fn test_upgrades() {
        let upgrades = MmUpgrades::ADULTS_WALLET | MmUpgrades::BOMB_BAG_30;

        assert_eq!(upgrades.wallet(), MmUpgrades::ADULTS_WALLET);
        assert_eq!(upgrades.bomb_bag(), MmUpgrades::BOMB_BAG_30);
        assert_eq!(upgrades.quiver(), MmUpgrades::empty());
    }

    #[test]
    fn test_game_mode_conversion() {
        assert_eq!(MmGameMode::try_from(0u32), Ok(MmGameMode::Gameplay));
        assert_eq!(MmGameMode::try_from(1u32), Ok(MmGameMode::TitleScreen));
        assert_eq!(MmGameMode::try_from(2u32), Ok(MmGameMode::FileSelect));
        assert_eq!(MmGameMode::try_from(4u32), Ok(MmGameMode::OwlSave));
        assert_eq!(MmGameMode::try_from(99u32), Err(99));
    }

    // ========================================================================
    // Real Parsing Tests
    // ========================================================================

    #[test]
    fn test_bottle_conversion() {
        use mm_item_ids::*;

        // Test None -> None
        assert_eq!(MmBottle::try_from(NONE), Ok(MmBottle::None));

        // Test various bottles
        assert_eq!(MmBottle::try_from(BOTTLE_EMPTY), Ok(MmBottle::Empty));
        assert_eq!(
            MmBottle::try_from(BOTTLE_RED_POTION),
            Ok(MmBottle::RedPotion)
        );
        assert_eq!(MmBottle::try_from(BOTTLE_FAIRY), Ok(MmBottle::Fairy));
        assert_eq!(
            MmBottle::try_from(BOTTLE_DEKU_PRINCESS),
            Ok(MmBottle::DekuPrincess)
        );
        assert_eq!(
            MmBottle::try_from(BOTTLE_CHATEAU_ROMANI),
            Ok(MmBottle::ChateauRomani)
        );

        // Test invalid value
        assert!(MmBottle::try_from(0xAB).is_err());

        // Test round-trip conversion
        let bottle = MmBottle::ChateauRomani;
        let raw: u8 = bottle.into();
        assert_eq!(MmBottle::try_from(raw), Ok(bottle));
    }

    #[test]
    fn test_from_save_data_size_validation() {
        // Too small
        let small_data = vec![0u8; 100];
        assert!(matches!(
            MmSave::from_save_data(&small_data),
            Err(MmDecodeError::Size(100))
        ));

        // Correct size should work
        let correct_data = vec![0u8; MM_SIZE];
        assert!(MmSave::from_save_data(&correct_data).is_ok());
    }

    #[test]
    fn test_from_save_data_parses_health() {
        use offsets::*;

        let mut data = vec![0u8; MM_SIZE];

        // Set health capacity to 0x0140 (5 hearts = 80 in decimal)
        data[HEALTH_CAPACITY] = 0x01;
        data[HEALTH_CAPACITY + 1] = 0x40;

        // Set current health to 0x0100 (4 hearts)
        data[HEALTH] = 0x01;
        data[HEALTH + 1] = 0x00;

        let save = MmSave::from_save_data(&data).unwrap();
        assert_eq!(save.health_capacity, 0x0140);
        assert_eq!(save.health, 0x0100);
    }

    #[test]
    fn test_from_save_data_parses_rupees() {
        use offsets::*;

        let mut data = vec![0u8; MM_SIZE];

        // Set rupees to 500 (0x01F4)
        data[RUPEES] = 0x01;
        data[RUPEES + 1] = 0xF4;

        let save = MmSave::from_save_data(&data).unwrap();
        assert_eq!(save.rupees, 500);
    }

    #[test]
    fn test_from_save_data_parses_sword_shield() {
        use offsets::*;

        let mut data = vec![0u8; MM_SIZE];

        // Set Gilded Sword (3) and Mirror Shield (2)
        // Shield is in high nibble, sword in low nibble
        data[SWORD_SHIELD] = 0x03 | (0x02 << 4);

        let save = MmSave::from_save_data(&data).unwrap();
        assert_eq!(save.sword, MmSword::GildedSword);
        assert_eq!(save.shield, MmShield::MirrorShield);
    }

    #[test]
    fn test_from_save_data_parses_quest_items() {
        use offsets::*;

        let mut data = vec![0u8; MM_SIZE];

        // Set boss remains: Odolwa (bit 0) and Goht (bit 1)
        // And Song of Time (bit 12)
        let quest_bits: u32 = 0x00001003; // REMAINS_ODOLWA | REMAINS_GOHT | SONG_TIME
        data[QUEST_ITEMS..QUEST_ITEMS + 4].copy_from_slice(&quest_bits.to_be_bytes());

        let save = MmSave::from_save_data(&data).unwrap();
        assert!(save.quest_items.contains(MmQuestItems::REMAINS_ODOLWA));
        assert!(save.quest_items.contains(MmQuestItems::REMAINS_GOHT));
        assert!(!save.quest_items.contains(MmQuestItems::REMAINS_GYORG));
        assert!(save.quest_items.contains(MmQuestItems::SONG_TIME));
        assert_eq!(save.quest_items.num_remains(), 2);
    }

    #[test]
    fn test_from_save_data_parses_stray_fairies() {
        use offsets::*;

        let mut data = vec![0u8; MM_SIZE];

        data[STRAY_FAIRIES] = 1; // Clock Town
        data[STRAY_FAIRIES + 1] = 15; // Woodfall
        data[STRAY_FAIRIES + 2] = 10; // Snowhead
        data[STRAY_FAIRIES + 3] = 5; // Great Bay
        data[STRAY_FAIRIES + 4] = 0; // Stone Tower

        let save = MmSave::from_save_data(&data).unwrap();
        assert_eq!(save.stray_fairies.clock_town, 1);
        assert_eq!(save.stray_fairies.woodfall, 15);
        assert_eq!(save.stray_fairies.snowhead, 10);
        assert_eq!(save.stray_fairies.great_bay, 5);
        assert_eq!(save.stray_fairies.stone_tower, 0);
        assert_eq!(save.stray_fairies.dungeon_total(), 30);
    }

    #[test]
    fn test_from_save_data_parses_dungeon_items() {
        use offsets::*;

        let mut data = vec![0u8; MM_SIZE];

        // Woodfall: Map + Compass + Boss Key (0x07)
        data[DUNGEON_ITEMS] = 0x07;
        // Snowhead: Map only (0x04)
        data[DUNGEON_ITEMS + 1] = 0x04;
        // Great Bay: Compass only (0x02)
        data[DUNGEON_ITEMS + 2] = 0x02;
        // Stone Tower: Boss Key only (0x01)
        data[DUNGEON_ITEMS + 3] = 0x01;

        let save = MmSave::from_save_data(&data).unwrap();
        assert!(save.dungeon_items.woodfall.contains(MmDungeonItems::MAP));
        assert!(save
            .dungeon_items
            .woodfall
            .contains(MmDungeonItems::COMPASS));
        assert!(save
            .dungeon_items
            .woodfall
            .contains(MmDungeonItems::BOSS_KEY));
        assert!(save.dungeon_items.snowhead.contains(MmDungeonItems::MAP));
        assert!(!save
            .dungeon_items
            .snowhead
            .contains(MmDungeonItems::BOSS_KEY));
        assert!(save
            .dungeon_items
            .great_bay
            .contains(MmDungeonItems::COMPASS));
        assert!(save
            .dungeon_items
            .stone_tower
            .contains(MmDungeonItems::BOSS_KEY));
    }

    #[test]
    fn test_from_save_data_parses_skulltulas() {
        use offsets::*;

        let mut data = vec![0u8; MM_SIZE];

        // Set swamp skulltulas to 30 (0x001E)
        data[SKULL_SWAMP] = 0x00;
        data[SKULL_SWAMP + 1] = 0x1E;

        // Set ocean skulltulas to 25 (0x0019)
        data[SKULL_OCEAN] = 0x00;
        data[SKULL_OCEAN + 1] = 0x19;

        let save = MmSave::from_save_data(&data).unwrap();
        assert_eq!(save.skull_tokens_swamp, 30);
        assert_eq!(save.skull_tokens_ocean, 25);
    }

    #[test]
    fn test_from_save_data_parses_time() {
        use offsets::*;

        let mut data = vec![0u8; MM_SIZE];

        // Day 2
        data[DAY..DAY + 4].copy_from_slice(&2u32.to_be_bytes());

        // Time = 0x8000 (noon-ish)
        data[TIME..TIME + 2].copy_from_slice(&0x8000u16.to_be_bytes());

        // Night = 1
        data[IS_NIGHT] = 1;

        let save = MmSave::from_save_data(&data).unwrap();
        assert_eq!(save.day, 2);
        assert_eq!(save.time, 0x8000);
        assert!(save.is_night);
    }

    #[test]
    fn test_from_save_data_parses_permanent_scene_flags() {
        use offsets::*;

        let mut data = vec![0u8; MM_SIZE];

        // Set first scene's chest flags
        let chest_flags: u32 = 0x12345678;
        data[PERM_SCENE_FLAGS..PERM_SCENE_FLAGS + 4].copy_from_slice(&chest_flags.to_be_bytes());

        let save = MmSave::from_save_data(&data).unwrap();
        assert_eq!(save.permanent_scene_flags.len(), MM_PERM_SCENE_COUNT);
        assert_eq!(save.permanent_scene_flags[0].chest, 0x12345678);
    }

    #[test]
    fn test_roundtrip_save_data() {
        // Create a save with some data
        let original = MmSave {
            player_form: PlayerForm::Goron,
            health_capacity: 0x0140,
            health: 0x0100,
            magic: MmMagicCapacity::Double,
            double_defense: true,
            rupees: 500,
            sword: MmSword::GildedSword,
            shield: MmShield::MirrorShield,
            quest_items: MmQuestItems::REMAINS_ODOLWA | MmQuestItems::SONG_TIME,
            upgrades: MmUpgrades::ADULTS_WALLET,
            dungeon_items: MmAllDungeonItems {
                woodfall: MmDungeonItems::MAP | MmDungeonItems::COMPASS,
                ..Default::default()
            },
            small_keys: MmSmallKeys {
                woodfall: 2,
                ..Default::default()
            },
            stray_fairies: MmStrayFairies {
                woodfall: 15,
                ..Default::default()
            },
            skull_tokens_swamp: 20,
            skull_tokens_ocean: 10,
            day: 2,
            time: 0x8000,
            is_night: true,
            ..Default::default()
        };

        // Serialize and deserialize
        let bytes = original.to_save_data();
        let parsed = MmSave::from_save_data(&bytes).unwrap();

        // Check key fields survived the roundtrip
        assert_eq!(parsed.player_form, original.player_form);
        assert_eq!(parsed.health_capacity, original.health_capacity);
        assert_eq!(parsed.health, original.health);
        assert_eq!(parsed.magic, original.magic);
        assert_eq!(parsed.double_defense, original.double_defense);
        assert_eq!(parsed.rupees, original.rupees);
        assert_eq!(parsed.sword, original.sword);
        assert_eq!(parsed.shield, original.shield);
        assert_eq!(parsed.quest_items, original.quest_items);
        assert_eq!(parsed.upgrades, original.upgrades);
        assert_eq!(parsed.dungeon_items, original.dungeon_items);
        assert_eq!(parsed.small_keys, original.small_keys);
        assert_eq!(parsed.stray_fairies, original.stray_fairies);
        assert_eq!(parsed.skull_tokens_swamp, original.skull_tokens_swamp);
        assert_eq!(parsed.skull_tokens_ocean, original.skull_tokens_ocean);
        assert_eq!(parsed.day, original.day);
        assert_eq!(parsed.time, original.time);
        assert_eq!(parsed.is_night, original.is_night);
    }

    #[test]
    fn test_mm_save_reader() {
        let data = vec![0u8; MM_SIZE];
        let reader = MmSaveReader::from_bytes(&data).unwrap();

        assert_eq!(reader.game_mode(), MmGameMode::Gameplay);
        assert_eq!(reader.get_save().health, 0);
    }

    #[test]
    fn test_mm_save_reader_update() {
        use offsets::*;

        let data = vec![0u8; MM_SIZE];
        let mut reader = MmSaveReader::from_bytes(&data).unwrap();

        // Update with new data containing different rupees
        let mut new_data = vec![0u8; MM_SIZE];
        new_data[RUPEES..RUPEES + 2].copy_from_slice(&200u16.to_be_bytes());

        reader.update(&new_data).unwrap();
        assert_eq!(reader.get_save().rupees, 200);
    }

    #[test]
    fn test_decode_error_display() {
        let err = MmDecodeError::Size(100);
        assert!(matches!(err, MmDecodeError::Size(100)));

        let err = MmDecodeError::Index(42);
        assert!(matches!(err, MmDecodeError::Index(42)));
    }

    // ========================================================================
    // Mask Accessor Tests
    // ========================================================================

    #[test]
    fn test_transformation_mask_accessors_default() {
        let save = MmSave::default();

        // All transformation masks should be false by default
        assert!(!save.has_deku_mask());
        assert!(!save.has_goron_mask());
        assert!(!save.has_zora_mask());
        assert!(!save.has_fierce_deity_mask());
    }

    #[test]
    fn test_has_deku_mask() {
        let mut save = MmSave::default();
        assert!(!save.has_deku_mask());

        save.masks
            .transformation
            .insert(MmTransformationMasks::DEKU);
        assert!(save.has_deku_mask());

        // Other masks should remain unaffected
        assert!(!save.has_goron_mask());
        assert!(!save.has_zora_mask());
        assert!(!save.has_fierce_deity_mask());
    }

    #[test]
    fn test_has_goron_mask() {
        let mut save = MmSave::default();
        assert!(!save.has_goron_mask());

        save.masks
            .transformation
            .insert(MmTransformationMasks::GORON);
        assert!(save.has_goron_mask());

        // Other masks should remain unaffected
        assert!(!save.has_deku_mask());
        assert!(!save.has_zora_mask());
        assert!(!save.has_fierce_deity_mask());
    }

    #[test]
    fn test_has_zora_mask() {
        let mut save = MmSave::default();
        assert!(!save.has_zora_mask());

        save.masks
            .transformation
            .insert(MmTransformationMasks::ZORA);
        assert!(save.has_zora_mask());

        // Other masks should remain unaffected
        assert!(!save.has_deku_mask());
        assert!(!save.has_goron_mask());
        assert!(!save.has_fierce_deity_mask());
    }

    #[test]
    fn test_has_fierce_deity_mask() {
        let mut save = MmSave::default();
        assert!(!save.has_fierce_deity_mask());

        save.masks
            .transformation
            .insert(MmTransformationMasks::FIERCE_DEITY);
        assert!(save.has_fierce_deity_mask());

        // Other masks should remain unaffected
        assert!(!save.has_deku_mask());
        assert!(!save.has_goron_mask());
        assert!(!save.has_zora_mask());
    }

    #[test]
    fn test_all_transformation_masks() {
        let mut save = MmSave::default();

        // Set all transformation masks
        save.masks.transformation = MmTransformationMasks::DEKU
            | MmTransformationMasks::GORON
            | MmTransformationMasks::ZORA
            | MmTransformationMasks::FIERCE_DEITY;

        assert!(save.has_deku_mask());
        assert!(save.has_goron_mask());
        assert!(save.has_zora_mask());
        assert!(save.has_fierce_deity_mask());
    }

    #[test]
    fn test_collectible_mask_accessors_default() {
        let save = MmSave::default();

        // All collectible masks should be false by default
        assert!(!save.has_postman_hat());
        assert!(!save.has_all_night_mask());
        assert!(!save.has_blast_mask());
        assert!(!save.has_stone_mask());
        assert!(!save.has_great_fairy_mask());
        assert!(!save.has_keaton_mask());
        assert!(!save.has_bremen_mask());
        assert!(!save.has_bunny_hood());
        assert!(!save.has_don_gero_mask());
        assert!(!save.has_mask_of_scents());
        assert!(!save.has_romani_mask());
        assert!(!save.has_circus_leader_mask());
        assert!(!save.has_kafei_mask());
        assert!(!save.has_couples_mask());
        assert!(!save.has_mask_of_truth());
        assert!(!save.has_kamaro_mask());
        assert!(!save.has_gibdo_mask());
        assert!(!save.has_garo_mask());
        assert!(!save.has_captain_hat());
        assert!(!save.has_giant_mask());
    }

    #[test]
    fn test_has_postman_hat() {
        let mut save = MmSave::default();
        assert!(!save.has_postman_hat());

        save.masks.masks_low.insert(MmMasksLow::POSTMAN);
        assert!(save.has_postman_hat());
    }

    #[test]
    fn test_has_all_night_mask() {
        let mut save = MmSave::default();
        assert!(!save.has_all_night_mask());

        save.masks.masks_low.insert(MmMasksLow::ALL_NIGHT);
        assert!(save.has_all_night_mask());
    }

    #[test]
    fn test_has_blast_mask() {
        let mut save = MmSave::default();
        assert!(!save.has_blast_mask());

        save.masks.masks_low.insert(MmMasksLow::BLAST);
        assert!(save.has_blast_mask());
    }

    #[test]
    fn test_has_stone_mask() {
        let mut save = MmSave::default();
        assert!(!save.has_stone_mask());

        save.masks.masks_low.insert(MmMasksLow::STONE);
        assert!(save.has_stone_mask());
    }

    #[test]
    fn test_has_great_fairy_mask() {
        let mut save = MmSave::default();
        assert!(!save.has_great_fairy_mask());

        save.masks.masks_low.insert(MmMasksLow::GREAT_FAIRY);
        assert!(save.has_great_fairy_mask());
    }

    #[test]
    fn test_has_keaton_mask() {
        let mut save = MmSave::default();
        assert!(!save.has_keaton_mask());

        save.masks.masks_low.insert(MmMasksLow::KEATON);
        assert!(save.has_keaton_mask());
    }

    #[test]
    fn test_has_bremen_mask() {
        let mut save = MmSave::default();
        assert!(!save.has_bremen_mask());

        save.masks.masks_low.insert(MmMasksLow::BREMEN);
        assert!(save.has_bremen_mask());
    }

    #[test]
    fn test_has_bunny_hood() {
        let mut save = MmSave::default();
        assert!(!save.has_bunny_hood());

        save.masks.masks_low.insert(MmMasksLow::BUNNY);
        assert!(save.has_bunny_hood());
    }

    #[test]
    fn test_has_don_gero_mask() {
        let mut save = MmSave::default();
        assert!(!save.has_don_gero_mask());

        save.masks.masks_low.insert(MmMasksLow::DON_GERO);
        assert!(save.has_don_gero_mask());
    }

    #[test]
    fn test_has_mask_of_scents() {
        let mut save = MmSave::default();
        assert!(!save.has_mask_of_scents());

        save.masks.masks_low.insert(MmMasksLow::SCENTS);
        assert!(save.has_mask_of_scents());
    }

    #[test]
    fn test_has_romani_mask() {
        let mut save = MmSave::default();
        assert!(!save.has_romani_mask());

        save.masks.masks_low.insert(MmMasksLow::ROMANI);
        assert!(save.has_romani_mask());
    }

    #[test]
    fn test_has_circus_leader_mask() {
        let mut save = MmSave::default();
        assert!(!save.has_circus_leader_mask());

        save.masks.masks_low.insert(MmMasksLow::CIRCUS_LEADER);
        assert!(save.has_circus_leader_mask());
    }

    #[test]
    fn test_has_kafei_mask() {
        let mut save = MmSave::default();
        assert!(!save.has_kafei_mask());

        save.masks.masks_low.insert(MmMasksLow::KAFEI);
        assert!(save.has_kafei_mask());
    }

    #[test]
    fn test_has_couples_mask() {
        let mut save = MmSave::default();
        assert!(!save.has_couples_mask());

        save.masks.masks_low.insert(MmMasksLow::COUPLES);
        assert!(save.has_couples_mask());
    }

    #[test]
    fn test_has_mask_of_truth() {
        let mut save = MmSave::default();
        assert!(!save.has_mask_of_truth());

        save.masks.masks_low.insert(MmMasksLow::TRUTH);
        assert!(save.has_mask_of_truth());
    }

    #[test]
    fn test_has_kamaro_mask() {
        let mut save = MmSave::default();
        assert!(!save.has_kamaro_mask());

        save.masks.masks_low.insert(MmMasksLow::KAMARO);
        assert!(save.has_kamaro_mask());
    }

    #[test]
    fn test_has_gibdo_mask() {
        let mut save = MmSave::default();
        assert!(!save.has_gibdo_mask());

        save.masks.masks_high.insert(MmMasksHigh::GIBDO);
        assert!(save.has_gibdo_mask());
    }

    #[test]
    fn test_has_garo_mask() {
        let mut save = MmSave::default();
        assert!(!save.has_garo_mask());

        save.masks.masks_high.insert(MmMasksHigh::GARO);
        assert!(save.has_garo_mask());
    }

    #[test]
    fn test_has_captain_hat() {
        let mut save = MmSave::default();
        assert!(!save.has_captain_hat());

        save.masks.masks_high.insert(MmMasksHigh::CAPTAIN);
        assert!(save.has_captain_hat());
    }

    #[test]
    fn test_has_giant_mask() {
        let mut save = MmSave::default();
        assert!(!save.has_giant_mask());

        save.masks.masks_high.insert(MmMasksHigh::GIANT);
        assert!(save.has_giant_mask());
    }

    #[test]
    fn test_all_masks_low() {
        let mut save = MmSave::default();

        // Set all masks_low flags
        save.masks.masks_low = MmMasksLow::POSTMAN
            | MmMasksLow::ALL_NIGHT
            | MmMasksLow::BLAST
            | MmMasksLow::STONE
            | MmMasksLow::GREAT_FAIRY
            | MmMasksLow::KEATON
            | MmMasksLow::BREMEN
            | MmMasksLow::BUNNY
            | MmMasksLow::DON_GERO
            | MmMasksLow::SCENTS
            | MmMasksLow::ROMANI
            | MmMasksLow::CIRCUS_LEADER
            | MmMasksLow::KAFEI
            | MmMasksLow::COUPLES
            | MmMasksLow::TRUTH
            | MmMasksLow::KAMARO;

        assert!(save.has_postman_hat());
        assert!(save.has_all_night_mask());
        assert!(save.has_blast_mask());
        assert!(save.has_stone_mask());
        assert!(save.has_great_fairy_mask());
        assert!(save.has_keaton_mask());
        assert!(save.has_bremen_mask());
        assert!(save.has_bunny_hood());
        assert!(save.has_don_gero_mask());
        assert!(save.has_mask_of_scents());
        assert!(save.has_romani_mask());
        assert!(save.has_circus_leader_mask());
        assert!(save.has_kafei_mask());
        assert!(save.has_couples_mask());
        assert!(save.has_mask_of_truth());
        assert!(save.has_kamaro_mask());
    }

    #[test]
    fn test_all_masks_high() {
        let mut save = MmSave::default();

        // Set all masks_high flags
        save.masks.masks_high =
            MmMasksHigh::GIBDO | MmMasksHigh::GARO | MmMasksHigh::CAPTAIN | MmMasksHigh::GIANT;

        assert!(save.has_gibdo_mask());
        assert!(save.has_garo_mask());
        assert!(save.has_captain_hat());
        assert!(save.has_giant_mask());
    }

    #[test]
    fn test_mask_accessors_with_sample_data() {
        let stub = MmSaveStub::with_sample_data();
        let save = stub.get_save();

        // Sample data has: DEKU, GORON, ZORA (transformation)
        assert!(save.has_deku_mask());
        assert!(save.has_goron_mask());
        assert!(save.has_zora_mask());
        assert!(!save.has_fierce_deity_mask());

        // Sample data has: BUNNY, STONE, GREAT_FAIRY, BREMEN (collectible)
        assert!(save.has_bunny_hood());
        assert!(save.has_stone_mask());
        assert!(save.has_great_fairy_mask());
        assert!(save.has_bremen_mask());

        // These should not be set in sample data
        assert!(!save.has_postman_hat());
        assert!(!save.has_all_night_mask());
        assert!(!save.has_blast_mask());
        assert!(!save.has_keaton_mask());
        assert!(!save.has_don_gero_mask());
        assert!(!save.has_mask_of_scents());
        assert!(!save.has_romani_mask());
        assert!(!save.has_circus_leader_mask());
        assert!(!save.has_kafei_mask());
        assert!(!save.has_couples_mask());
        assert!(!save.has_mask_of_truth());
        assert!(!save.has_kamaro_mask());
        assert!(!save.has_gibdo_mask());
        assert!(!save.has_garo_mask());
        assert!(!save.has_captain_hat());
        assert!(!save.has_giant_mask());
    }

    // ========================================================================
    // Inventory Accessor Tests
    // ========================================================================

    #[test]
    fn test_equipment_accessors_default() {
        let save = MmSave::default();

        // All equipment should be false by default
        assert!(!save.has_ocarina());
        assert!(!save.has_heros_bow());
        assert!(!save.has_fire_arrow());
        assert!(!save.has_ice_arrow());
        assert!(!save.has_light_arrow());
        assert!(!save.has_hookshot());
        assert!(!save.has_bombs());
        assert!(!save.has_bombchu());
        assert!(!save.has_powder_keg());
        assert!(!save.has_lens_of_truth());
        assert!(!save.has_pictograph_box());
        assert!(!save.has_great_fairy_sword());
        assert!(!save.has_magic_bean());
        assert!(!save.has_magic());
    }

    #[test]
    fn test_equipment_accessors_with_items() {
        let mut save = MmSave::default();

        // Set some equipment
        save.inventory.ocarina = true;
        save.inventory.bow = true;
        save.inventory.fire_arrows = true;
        save.inventory.hookshot = true;
        save.inventory.bombs = true;
        save.inventory.lens = true;
        save.inventory.great_fairy_sword = true;
        save.magic = MmMagicCapacity::Double;

        // Check accessors return true for items we have
        assert!(save.has_ocarina());
        assert!(save.has_heros_bow());
        assert!(save.has_fire_arrow());
        assert!(save.has_hookshot());
        assert!(save.has_bombs());
        assert!(save.has_lens_of_truth());
        assert!(save.has_great_fairy_sword());
        assert!(save.has_magic());

        // Check accessors return false for items we don't have
        assert!(!save.has_ice_arrow());
        assert!(!save.has_light_arrow());
        assert!(!save.has_bombchu());
        assert!(!save.has_powder_keg());
        assert!(!save.has_pictograph_box());
        assert!(!save.has_magic_bean());
    }

    #[test]
    fn test_magic_accessor_single_magic() {
        let save = MmSave {
            magic: MmMagicCapacity::Single,
            ..Default::default()
        };
        assert!(save.has_magic());
    }

    #[test]
    fn test_magic_accessor_double_magic() {
        let save = MmSave {
            magic: MmMagicCapacity::Double,
            ..Default::default()
        };
        assert!(save.has_magic());
    }

    #[test]
    fn test_song_accessors_default() {
        let save = MmSave::default();

        // All songs should be false by default
        assert!(!save.has_song_of_time());
        assert!(!save.has_song_of_healing());
        assert!(!save.has_eponas_song());
        assert!(!save.has_song_of_soaring());
        assert!(!save.has_song_of_storms());
        assert!(!save.has_sonata_of_awakening());
        assert!(!save.has_goron_lullaby());
        assert!(!save.has_new_wave_bossa_nova());
        assert!(!save.has_elegy_of_emptiness());
        assert!(!save.has_oath_to_order());
    }

    #[test]
    fn test_song_accessors_with_songs() {
        // Set some songs
        let save = MmSave {
            quest_items: MmQuestItems::SONG_TIME
                | MmQuestItems::SONG_HEALING
                | MmQuestItems::SONG_EPONA
                | MmQuestItems::SONG_SOARING
                | MmQuestItems::SONG_AWAKENING
                | MmQuestItems::SONG_ORDER,
            ..Default::default()
        };

        // Check accessors return true for songs we have
        assert!(save.has_song_of_time());
        assert!(save.has_song_of_healing());
        assert!(save.has_eponas_song());
        assert!(save.has_song_of_soaring());
        assert!(save.has_sonata_of_awakening());
        assert!(save.has_oath_to_order());

        // Check accessors return false for songs we don't have
        assert!(!save.has_song_of_storms());
        assert!(!save.has_goron_lullaby());
        assert!(!save.has_new_wave_bossa_nova());
        assert!(!save.has_elegy_of_emptiness());
    }

    #[test]
    fn test_song_accessors_all_songs() {
        // Set all songs
        let save = MmSave {
            quest_items: MmQuestItems::SONG_TIME
                | MmQuestItems::SONG_HEALING
                | MmQuestItems::SONG_EPONA
                | MmQuestItems::SONG_SOARING
                | MmQuestItems::SONG_STORMS
                | MmQuestItems::SONG_AWAKENING
                | MmQuestItems::SONG_GORON
                | MmQuestItems::SONG_ZORA
                | MmQuestItems::SONG_EMPTINESS
                | MmQuestItems::SONG_ORDER,
            ..Default::default()
        };

        // All song accessors should return true
        assert!(save.has_song_of_time());
        assert!(save.has_song_of_healing());
        assert!(save.has_eponas_song());
        assert!(save.has_song_of_soaring());
        assert!(save.has_song_of_storms());
        assert!(save.has_sonata_of_awakening());
        assert!(save.has_goron_lullaby());
        assert!(save.has_new_wave_bossa_nova());
        assert!(save.has_elegy_of_emptiness());
        assert!(save.has_oath_to_order());
    }

    #[test]
    fn test_boss_remains_accessors_default() {
        let save = MmSave::default();

        // All remains should be false by default
        assert!(!save.has_odolwa_remains());
        assert!(!save.has_goht_remains());
        assert!(!save.has_gyorg_remains());
        assert!(!save.has_twinmold_remains());
    }

    #[test]
    fn test_boss_remains_accessors_with_remains() {
        // Set some boss remains
        let save = MmSave {
            quest_items: MmQuestItems::REMAINS_ODOLWA | MmQuestItems::REMAINS_GOHT,
            ..Default::default()
        };

        // Check accessors return true for remains we have
        assert!(save.has_odolwa_remains());
        assert!(save.has_goht_remains());

        // Check accessors return false for remains we don't have
        assert!(!save.has_gyorg_remains());
        assert!(!save.has_twinmold_remains());
    }

    #[test]
    fn test_boss_remains_accessors_all_remains() {
        // Set all boss remains
        let save = MmSave {
            quest_items: MmQuestItems::REMAINS_ODOLWA
                | MmQuestItems::REMAINS_GOHT
                | MmQuestItems::REMAINS_GYORG
                | MmQuestItems::REMAINS_TWINMOLD,
            ..Default::default()
        };

        // All remains accessors should return true
        assert!(save.has_odolwa_remains());
        assert!(save.has_goht_remains());
        assert!(save.has_gyorg_remains());
        assert!(save.has_twinmold_remains());
    }

    #[test]
    fn test_accessors_with_sample_data() {
        let stub = MmSaveStub::with_sample_data();
        let save = stub.get_save();

        // Check equipment from sample data
        assert!(save.has_ocarina());
        assert!(save.has_heros_bow());
        assert!(save.has_hookshot());
        assert!(save.has_bombs());
        assert!(save.has_lens_of_truth());
        assert!(save.has_magic());

        // Check songs from sample data
        assert!(save.has_song_of_time());
        assert!(save.has_song_of_healing());
        assert!(save.has_eponas_song());
        assert!(save.has_song_of_soaring());
        assert!(save.has_sonata_of_awakening());

        // Check boss remains from sample data
        assert!(save.has_odolwa_remains());
        assert!(save.has_goht_remains());
        assert!(!save.has_gyorg_remains());
        assert!(!save.has_twinmold_remains());
    }

    #[test]
    fn test_equipment_accessors_from_parsed_data() {
        use offsets::*;

        let mut data = vec![0u8; MM_SIZE];

        // Set inventory items at their slot positions
        // Ocarina is slot 0, Bow is slot 1, Hookshot is slot 15
        data[INVENTORY] = mm_item_ids::OCARINA;
        data[INVENTORY + 1] = mm_item_ids::BOW;
        data[INVENTORY + 2] = mm_item_ids::FIRE_ARROW;
        data[INVENTORY + 15] = mm_item_ids::HOOKSHOT;

        // Set magic level
        data[MAGIC_LEVEL] = 2; // Double magic

        let save = MmSave::from_save_data(&data).unwrap();

        assert!(save.has_ocarina());
        assert!(save.has_heros_bow());
        assert!(save.has_fire_arrow());
        assert!(save.has_hookshot());
        assert!(save.has_magic());

        // Items not set should be false
        assert!(!save.has_ice_arrow());
        assert!(!save.has_bombs());
    }

    #[test]
    fn test_song_accessors_from_parsed_data() {
        use offsets::*;

        let mut data = vec![0u8; MM_SIZE];

        // Set quest items with Song of Time (bit 12) and Song of Healing (bit 13)
        let quest_bits: u32 = MmQuestItems::SONG_TIME.bits() | MmQuestItems::SONG_HEALING.bits();
        data[QUEST_ITEMS..QUEST_ITEMS + 4].copy_from_slice(&quest_bits.to_be_bytes());

        let save = MmSave::from_save_data(&data).unwrap();

        assert!(save.has_song_of_time());
        assert!(save.has_song_of_healing());
        assert!(!save.has_eponas_song());
        assert!(!save.has_song_of_soaring());
    }

    #[test]
    fn test_boss_remains_accessors_from_parsed_data() {
        use offsets::*;

        let mut data = vec![0u8; MM_SIZE];

        // Set quest items with Odolwa and Gyorg remains
        let quest_bits: u32 =
            MmQuestItems::REMAINS_ODOLWA.bits() | MmQuestItems::REMAINS_GYORG.bits();
        data[QUEST_ITEMS..QUEST_ITEMS + 4].copy_from_slice(&quest_bits.to_be_bytes());

        let save = MmSave::from_save_data(&data).unwrap();

        assert!(save.has_odolwa_remains());
        assert!(!save.has_goht_remains());
        assert!(save.has_gyorg_remains());
        assert!(!save.has_twinmold_remains());
    }

    // ========================================================================
    // Mask Parsing from Raw Bytes Tests
    // ========================================================================

    #[test]
    fn test_parse_masks_transformation_masks_from_raw() {
        use offsets::*;

        let mut data = vec![0u8; MM_SIZE];

        // Place transformation masks in mask inventory slots
        data[MASKS] = mm_item_ids::MASK_DEKU;
        data[MASKS + 1] = mm_item_ids::MASK_GORON;
        data[MASKS + 2] = mm_item_ids::MASK_ZORA;
        data[MASKS + 3] = mm_item_ids::MASK_FIERCE_DEITY;

        let save = MmSave::from_save_data(&data).unwrap();

        assert!(save
            .masks
            .transformation
            .contains(MmTransformationMasks::DEKU));
        assert!(save
            .masks
            .transformation
            .contains(MmTransformationMasks::GORON));
        assert!(save
            .masks
            .transformation
            .contains(MmTransformationMasks::ZORA));
        assert!(save
            .masks
            .transformation
            .contains(MmTransformationMasks::FIERCE_DEITY));
    }

    #[test]
    fn test_parse_masks_collectible_masks_low_from_raw() {
        use offsets::*;

        let mut data = vec![0u8; MM_SIZE];

        // Place collectible masks (low bits) in mask inventory slots
        data[MASKS] = mm_item_ids::MASK_POSTMAN;
        data[MASKS + 1] = mm_item_ids::MASK_ALL_NIGHT;
        data[MASKS + 2] = mm_item_ids::MASK_BLAST;
        data[MASKS + 3] = mm_item_ids::MASK_STONE;
        data[MASKS + 4] = mm_item_ids::MASK_GREAT_FAIRY;
        data[MASKS + 5] = mm_item_ids::MASK_KEATON;
        data[MASKS + 6] = mm_item_ids::MASK_BREMEN;
        data[MASKS + 7] = mm_item_ids::MASK_BUNNY;
        data[MASKS + 8] = mm_item_ids::MASK_DON_GERO;
        data[MASKS + 9] = mm_item_ids::MASK_SCENTS;
        data[MASKS + 10] = mm_item_ids::MASK_ROMANI;
        data[MASKS + 11] = mm_item_ids::MASK_CIRCUS_LEADER;
        data[MASKS + 12] = mm_item_ids::MASK_KAFEI;
        data[MASKS + 13] = mm_item_ids::MASK_COUPLES;
        data[MASKS + 14] = mm_item_ids::MASK_TRUTH;
        data[MASKS + 15] = mm_item_ids::MASK_KAMARO;

        let save = MmSave::from_save_data(&data).unwrap();

        assert!(save.masks.masks_low.contains(MmMasksLow::POSTMAN));
        assert!(save.masks.masks_low.contains(MmMasksLow::ALL_NIGHT));
        assert!(save.masks.masks_low.contains(MmMasksLow::BLAST));
        assert!(save.masks.masks_low.contains(MmMasksLow::STONE));
        assert!(save.masks.masks_low.contains(MmMasksLow::GREAT_FAIRY));
        assert!(save.masks.masks_low.contains(MmMasksLow::KEATON));
        assert!(save.masks.masks_low.contains(MmMasksLow::BREMEN));
        assert!(save.masks.masks_low.contains(MmMasksLow::BUNNY));
        assert!(save.masks.masks_low.contains(MmMasksLow::DON_GERO));
        assert!(save.masks.masks_low.contains(MmMasksLow::SCENTS));
        assert!(save.masks.masks_low.contains(MmMasksLow::ROMANI));
        assert!(save.masks.masks_low.contains(MmMasksLow::CIRCUS_LEADER));
        assert!(save.masks.masks_low.contains(MmMasksLow::KAFEI));
        assert!(save.masks.masks_low.contains(MmMasksLow::COUPLES));
        assert!(save.masks.masks_low.contains(MmMasksLow::TRUTH));
        assert!(save.masks.masks_low.contains(MmMasksLow::KAMARO));
        assert_eq!(save.masks.masks_low.bits().count_ones(), 16);
    }

    #[test]
    fn test_parse_masks_collectible_masks_high_from_raw() {
        use offsets::*;

        let mut data = vec![0u8; MM_SIZE];

        // Place high-bit collectible masks
        data[MASKS] = mm_item_ids::MASK_GIBDO;
        data[MASKS + 1] = mm_item_ids::MASK_GARO;
        data[MASKS + 2] = mm_item_ids::MASK_CAPTAIN;
        data[MASKS + 3] = mm_item_ids::MASK_GIANT;

        let save = MmSave::from_save_data(&data).unwrap();

        assert!(save.masks.masks_high.contains(MmMasksHigh::GIBDO));
        assert!(save.masks.masks_high.contains(MmMasksHigh::GARO));
        assert!(save.masks.masks_high.contains(MmMasksHigh::CAPTAIN));
        assert!(save.masks.masks_high.contains(MmMasksHigh::GIANT));
        assert_eq!(save.masks.masks_high.bits().count_ones(), 4);
    }

    #[test]
    fn test_parse_masks_empty_slots() {
        use offsets::*;

        let mut data = vec![0u8; MM_SIZE];

        // Fill mask inventory with NONE (0xFF)
        for i in 0..24 {
            data[MASKS + i] = mm_item_ids::NONE;
        }

        let save = MmSave::from_save_data(&data).unwrap();

        assert!(save.masks.transformation.is_empty());
        assert!(save.masks.masks_low.is_empty());
        assert!(save.masks.masks_high.is_empty());
        assert_eq!(save.masks.total_mask_count(), 0);
    }

    #[test]
    fn test_parse_masks_invalid_ids_ignored() {
        use offsets::*;

        let mut data = vec![0u8; MM_SIZE];

        // Put some invalid mask IDs (regular item IDs, not masks)
        data[MASKS] = mm_item_ids::OCARINA; // Not a mask
        data[MASKS + 1] = mm_item_ids::BOW; // Not a mask
        data[MASKS + 2] = 0xAA; // Invalid ID
        data[MASKS + 3] = mm_item_ids::MASK_DEKU; // Valid mask

        let save = MmSave::from_save_data(&data).unwrap();

        // Only the Deku Mask should be recognized
        assert!(save
            .masks
            .transformation
            .contains(MmTransformationMasks::DEKU));
        assert_eq!(save.masks.total_mask_count(), 1);
    }

    #[test]
    fn test_parse_masks_all_24_slots() {
        use offsets::*;

        let mut data = vec![0u8; MM_SIZE];

        // Put all 24 masks in their slots
        // Transformation masks (4)
        data[MASKS] = mm_item_ids::MASK_DEKU;
        data[MASKS + 1] = mm_item_ids::MASK_GORON;
        data[MASKS + 2] = mm_item_ids::MASK_ZORA;
        data[MASKS + 3] = mm_item_ids::MASK_FIERCE_DEITY;

        // Low masks (16)
        data[MASKS + 4] = mm_item_ids::MASK_POSTMAN;
        data[MASKS + 5] = mm_item_ids::MASK_ALL_NIGHT;
        data[MASKS + 6] = mm_item_ids::MASK_BLAST;
        data[MASKS + 7] = mm_item_ids::MASK_STONE;
        data[MASKS + 8] = mm_item_ids::MASK_GREAT_FAIRY;
        data[MASKS + 9] = mm_item_ids::MASK_KEATON;
        data[MASKS + 10] = mm_item_ids::MASK_BREMEN;
        data[MASKS + 11] = mm_item_ids::MASK_BUNNY;
        data[MASKS + 12] = mm_item_ids::MASK_DON_GERO;
        data[MASKS + 13] = mm_item_ids::MASK_SCENTS;
        data[MASKS + 14] = mm_item_ids::MASK_ROMANI;
        data[MASKS + 15] = mm_item_ids::MASK_CIRCUS_LEADER;
        data[MASKS + 16] = mm_item_ids::MASK_KAFEI;
        data[MASKS + 17] = mm_item_ids::MASK_COUPLES;
        data[MASKS + 18] = mm_item_ids::MASK_TRUTH;
        data[MASKS + 19] = mm_item_ids::MASK_KAMARO;

        // High masks (4)
        data[MASKS + 20] = mm_item_ids::MASK_GIBDO;
        data[MASKS + 21] = mm_item_ids::MASK_GARO;
        data[MASKS + 22] = mm_item_ids::MASK_CAPTAIN;
        data[MASKS + 23] = mm_item_ids::MASK_GIANT;

        let save = MmSave::from_save_data(&data).unwrap();

        // Total should be 4 transformation + 16 low + 4 high = 24
        assert_eq!(save.masks.transformation.bits().count_ones(), 4);
        assert_eq!(save.masks.masks_low.bits().count_ones(), 16);
        assert_eq!(save.masks.masks_high.bits().count_ones(), 4);
        assert_eq!(save.masks.total_mask_count(), 24);
    }

    // ========================================================================
    // Inventory Parsing from Raw Bytes Tests
    // ========================================================================

    #[test]
    fn test_parse_inventory_all_items_from_raw() {
        use offsets::*;

        let mut data = vec![0u8; MM_SIZE];

        // Set all inventory items at their correct slot positions
        data[INVENTORY] = mm_item_ids::OCARINA; // slot 0
        data[INVENTORY + 1] = mm_item_ids::BOW; // slot 1
        data[INVENTORY + 2] = mm_item_ids::FIRE_ARROW; // slot 2
        data[INVENTORY + 3] = mm_item_ids::ICE_ARROW; // slot 3
        data[INVENTORY + 4] = mm_item_ids::LIGHT_ARROW; // slot 4
        data[INVENTORY + 6] = mm_item_ids::BOMB; // slot 6
        data[INVENTORY + 7] = mm_item_ids::BOMBCHU; // slot 7
        data[INVENTORY + 8] = mm_item_ids::DEKU_STICK; // slot 8
        data[INVENTORY + 9] = mm_item_ids::DEKU_NUT; // slot 9
        data[INVENTORY + 10] = mm_item_ids::MAGIC_BEAN; // slot 10
        data[INVENTORY + 12] = mm_item_ids::POWDER_KEG; // slot 12
        data[INVENTORY + 13] = mm_item_ids::PICTOGRAPH_BOX; // slot 13
        data[INVENTORY + 14] = mm_item_ids::LENS; // slot 14
        data[INVENTORY + 15] = mm_item_ids::HOOKSHOT; // slot 15
        data[INVENTORY + 16] = mm_item_ids::GREAT_FAIRY_SWORD; // slot 16

        let save = MmSave::from_save_data(&data).unwrap();

        assert!(save.inventory.ocarina);
        assert!(save.inventory.bow);
        assert!(save.inventory.fire_arrows);
        assert!(save.inventory.ice_arrows);
        assert!(save.inventory.light_arrows);
        assert!(save.inventory.bombs);
        assert!(save.inventory.bombchus);
        assert!(save.inventory.deku_sticks);
        assert!(save.inventory.deku_nuts);
        assert!(save.inventory.magic_beans);
        assert!(save.inventory.powder_keg);
        assert!(save.inventory.pictograph_box);
        assert!(save.inventory.lens);
        assert!(save.inventory.hookshot);
        assert!(save.inventory.great_fairy_sword);
    }

    #[test]
    fn test_parse_inventory_empty_from_raw() {
        let mut data = vec![0u8; MM_SIZE];

        // Fill inventory slots with NONE (0xFF)
        for i in 0..24 {
            data[offsets::INVENTORY + i] = mm_item_ids::NONE;
        }

        let save = MmSave::from_save_data(&data).unwrap();

        assert!(!save.inventory.ocarina);
        assert!(!save.inventory.bow);
        assert!(!save.inventory.fire_arrows);
        assert!(!save.inventory.ice_arrows);
        assert!(!save.inventory.light_arrows);
        assert!(!save.inventory.bombs);
        assert!(!save.inventory.bombchus);
        assert!(!save.inventory.deku_sticks);
        assert!(!save.inventory.deku_nuts);
        assert!(!save.inventory.magic_beans);
        assert!(!save.inventory.powder_keg);
        assert!(!save.inventory.pictograph_box);
        assert!(!save.inventory.lens);
        assert!(!save.inventory.hookshot);
        assert!(!save.inventory.great_fairy_sword);
    }

    #[test]
    fn test_parse_inventory_wrong_slot_item() {
        use offsets::*;

        let mut data = vec![0u8; MM_SIZE];

        // Put bow item ID in ocarina slot - should not detect ocarina
        data[INVENTORY] = mm_item_ids::BOW;

        let save = MmSave::from_save_data(&data).unwrap();

        // Ocarina should be false because the value doesn't match expected
        assert!(!save.inventory.ocarina);
        // Bow should also be false because it's not in the right slot
        assert!(!save.inventory.bow);
    }

    #[test]
    fn test_parse_inventory_bottles_all_types() {
        use offsets::*;

        let mut data = vec![0u8; MM_SIZE];

        // Set bottles in slots 18-23
        data[INVENTORY + 18] = mm_item_ids::BOTTLE_EMPTY;
        data[INVENTORY + 19] = mm_item_ids::BOTTLE_RED_POTION;
        data[INVENTORY + 20] = mm_item_ids::BOTTLE_FAIRY;
        data[INVENTORY + 21] = mm_item_ids::BOTTLE_DEKU_PRINCESS;
        data[INVENTORY + 22] = mm_item_ids::BOTTLE_CHATEAU_ROMANI;
        data[INVENTORY + 23] = mm_item_ids::BOTTLE_ZORA_EGG;

        let save = MmSave::from_save_data(&data).unwrap();

        assert_eq!(save.inventory.bottles[0], MmBottle::Empty);
        assert_eq!(save.inventory.bottles[1], MmBottle::RedPotion);
        assert_eq!(save.inventory.bottles[2], MmBottle::Fairy);
        assert_eq!(save.inventory.bottles[3], MmBottle::DekuPrincess);
        assert_eq!(save.inventory.bottles[4], MmBottle::ChateauRomani);
        assert_eq!(save.inventory.bottles[5], MmBottle::ZoraEgg);
    }

    #[test]
    fn test_parse_inventory_bottles_empty_slots() {
        let data = vec![0u8; MM_SIZE];

        let save = MmSave::from_save_data(&data).unwrap();

        // All bottles should be None when slots are zero
        for bottle in &save.inventory.bottles {
            assert_eq!(*bottle, MmBottle::None);
        }
    }

    #[test]
    fn test_parse_inventory_bottles_all_variants() {
        use offsets::*;

        let bottle_types = [
            (mm_item_ids::BOTTLE_EMPTY, MmBottle::Empty),
            (mm_item_ids::BOTTLE_GREEN_POTION, MmBottle::GreenPotion),
            (mm_item_ids::BOTTLE_BLUE_POTION, MmBottle::BluePotion),
            (mm_item_ids::BOTTLE_MILK, MmBottle::Milk),
            (mm_item_ids::BOTTLE_MILK_HALF, MmBottle::MilkHalf),
            (mm_item_ids::BOTTLE_FISH, MmBottle::Fish),
            (mm_item_ids::BOTTLE_BUG, MmBottle::Bug),
            (mm_item_ids::BOTTLE_BLUE_FIRE, MmBottle::BlueFire),
            (mm_item_ids::BOTTLE_POE, MmBottle::Poe),
            (mm_item_ids::BOTTLE_BIG_POE, MmBottle::BigPoe),
            (mm_item_ids::BOTTLE_WATER, MmBottle::Water),
            (
                mm_item_ids::BOTTLE_HOT_SPRING_WATER,
                MmBottle::HotSpringWater,
            ),
            (mm_item_ids::BOTTLE_GOLD_DUST, MmBottle::GoldDust),
            (mm_item_ids::BOTTLE_MUSHROOM, MmBottle::MagicalMushroom),
            (mm_item_ids::BOTTLE_SEAHORSE, MmBottle::SeaHorse),
            (mm_item_ids::BOTTLE_MYSTERY_MILK, MmBottle::MysteryMilk),
            (
                mm_item_ids::BOTTLE_MYSTERY_MILK_SPOILED,
                MmBottle::MysteryMilkSpoiled,
            ),
        ];

        for (raw_id, expected_bottle) in bottle_types {
            let mut data = vec![0u8; MM_SIZE];
            data[INVENTORY + 18] = raw_id;

            let save = MmSave::from_save_data(&data).unwrap();
            assert_eq!(
                save.inventory.bottles[0], expected_bottle,
                "Failed for bottle ID 0x{:02X}",
                raw_id
            );
        }
    }

    // ========================================================================
    // Small Keys Parsing Tests
    // ========================================================================

    #[test]
    fn test_parse_small_keys_from_raw() {
        use offsets::*;

        let mut data = vec![0u8; MM_SIZE];

        data[SMALL_KEYS] = 3; // Woodfall
        data[SMALL_KEYS + 1] = 2; // Snowhead
        data[SMALL_KEYS + 2] = 1; // Great Bay
        data[SMALL_KEYS + 3] = 4; // Stone Tower

        let save = MmSave::from_save_data(&data).unwrap();

        assert_eq!(save.small_keys.woodfall, 3);
        assert_eq!(save.small_keys.snowhead, 2);
        assert_eq!(save.small_keys.great_bay, 1);
        assert_eq!(save.small_keys.stone_tower, 4);
    }

    #[test]
    fn test_parse_small_keys_0xff_as_zero() {
        use offsets::*;

        let mut data = vec![0u8; MM_SIZE];

        // 0xFF means no keys collected yet (uninitialized)
        data[SMALL_KEYS] = 0xFF;
        data[SMALL_KEYS + 1] = 0xFF;
        data[SMALL_KEYS + 2] = 0xFF;
        data[SMALL_KEYS + 3] = 0xFF;

        let save = MmSave::from_save_data(&data).unwrap();

        // 0xFF should be treated as 0
        assert_eq!(save.small_keys.woodfall, 0);
        assert_eq!(save.small_keys.snowhead, 0);
        assert_eq!(save.small_keys.great_bay, 0);
        assert_eq!(save.small_keys.stone_tower, 0);
    }

    #[test]
    fn test_parse_small_keys_mixed_values() {
        use offsets::*;

        let mut data = vec![0u8; MM_SIZE];

        data[SMALL_KEYS] = 0xFF; // Should become 0
        data[SMALL_KEYS + 1] = 2;
        data[SMALL_KEYS + 2] = 0xFF; // Should become 0
        data[SMALL_KEYS + 3] = 3;

        let save = MmSave::from_save_data(&data).unwrap();

        assert_eq!(save.small_keys.woodfall, 0);
        assert_eq!(save.small_keys.snowhead, 2);
        assert_eq!(save.small_keys.great_bay, 0);
        assert_eq!(save.small_keys.stone_tower, 3);
    }

    #[test]
    fn test_small_keys_accessor_methods() {
        let keys = MmSmallKeys {
            woodfall: 1,
            snowhead: 2,
            great_bay: 3,
            stone_tower: 4,
        };

        assert_eq!(keys.snowhead(), 2);
        assert_eq!(keys.great_bay(), 3);
        assert_eq!(keys.stone_tower(), 4);
    }

    // ========================================================================
    // Song Parsing from Raw Bytes Tests
    // ========================================================================

    #[test]
    fn test_parse_all_songs_from_raw() {
        use offsets::*;

        let mut data = vec![0u8; MM_SIZE];

        // Set all song bits
        let quest_bits: u32 = MmQuestItems::SONG_AWAKENING.bits()
            | MmQuestItems::SONG_GORON.bits()
            | MmQuestItems::SONG_ZORA.bits()
            | MmQuestItems::SONG_EMPTINESS.bits()
            | MmQuestItems::SONG_ORDER.bits()
            | MmQuestItems::SONG_TIME.bits()
            | MmQuestItems::SONG_HEALING.bits()
            | MmQuestItems::SONG_EPONA.bits()
            | MmQuestItems::SONG_SOARING.bits()
            | MmQuestItems::SONG_STORMS.bits()
            | MmQuestItems::SONG_LULLABY_INTRO.bits();

        data[QUEST_ITEMS..QUEST_ITEMS + 4].copy_from_slice(&quest_bits.to_be_bytes());

        let save = MmSave::from_save_data(&data).unwrap();

        assert!(save.quest_items.contains(MmQuestItems::SONG_AWAKENING));
        assert!(save.quest_items.contains(MmQuestItems::SONG_GORON));
        assert!(save.quest_items.contains(MmQuestItems::SONG_ZORA));
        assert!(save.quest_items.contains(MmQuestItems::SONG_EMPTINESS));
        assert!(save.quest_items.contains(MmQuestItems::SONG_ORDER));
        assert!(save.quest_items.contains(MmQuestItems::SONG_TIME));
        assert!(save.quest_items.contains(MmQuestItems::SONG_HEALING));
        assert!(save.quest_items.contains(MmQuestItems::SONG_EPONA));
        assert!(save.quest_items.contains(MmQuestItems::SONG_SOARING));
        assert!(save.quest_items.contains(MmQuestItems::SONG_STORMS));
        assert!(save.quest_items.contains(MmQuestItems::SONG_LULLABY_INTRO));
    }

    #[test]
    fn test_parse_songs_individual_bits() {
        use offsets::*;

        // Test each song bit individually
        let songs = [
            (MmQuestItems::SONG_AWAKENING, "Sonata of Awakening"),
            (MmQuestItems::SONG_GORON, "Goron Lullaby"),
            (MmQuestItems::SONG_ZORA, "New Wave Bossa Nova"),
            (MmQuestItems::SONG_EMPTINESS, "Elegy of Emptiness"),
            (MmQuestItems::SONG_ORDER, "Oath to Order"),
            (MmQuestItems::SONG_TIME, "Song of Time"),
            (MmQuestItems::SONG_HEALING, "Song of Healing"),
            (MmQuestItems::SONG_EPONA, "Epona's Song"),
            (MmQuestItems::SONG_SOARING, "Song of Soaring"),
            (MmQuestItems::SONG_STORMS, "Song of Storms"),
        ];

        for (song_flag, name) in songs {
            let mut data = vec![0u8; MM_SIZE];
            data[QUEST_ITEMS..QUEST_ITEMS + 4].copy_from_slice(&song_flag.bits().to_be_bytes());

            let save = MmSave::from_save_data(&data).unwrap();
            assert!(
                save.quest_items.contains(song_flag),
                "Failed to parse {}",
                name
            );
        }
    }

    // ========================================================================
    // Edge Cases and Error Handling Tests
    // ========================================================================

    #[test]
    fn test_sword_try_from_all_variants() {
        assert_eq!(MmSword::try_from(0), Ok(MmSword::None));
        assert_eq!(MmSword::try_from(1), Ok(MmSword::KokiriSword));
        assert_eq!(MmSword::try_from(2), Ok(MmSword::RazorSword));
        assert_eq!(MmSword::try_from(3), Ok(MmSword::GildedSword));
        assert_eq!(MmSword::try_from(4), Err(4));
        assert_eq!(MmSword::try_from(255), Err(255));
    }

    #[test]
    fn test_shield_try_from_all_variants() {
        assert_eq!(MmShield::try_from(0), Ok(MmShield::None));
        assert_eq!(MmShield::try_from(1), Ok(MmShield::HeroShield));
        assert_eq!(MmShield::try_from(2), Ok(MmShield::MirrorShield));
        assert_eq!(MmShield::try_from(3), Err(3));
        assert_eq!(MmShield::try_from(255), Err(255));
    }

    #[test]
    fn test_magic_capacity_try_from_all_variants() {
        assert_eq!(MmMagicCapacity::try_from(0), Ok(MmMagicCapacity::None));
        assert_eq!(MmMagicCapacity::try_from(1), Ok(MmMagicCapacity::Single));
        assert_eq!(MmMagicCapacity::try_from(2), Ok(MmMagicCapacity::Double));
        assert_eq!(MmMagicCapacity::try_from(3), Err(3));
        assert_eq!(MmMagicCapacity::try_from(255), Err(255));
    }

    #[test]
    fn test_parse_invalid_player_form_defaults_to_human() {
        use offsets::*;

        let mut data = vec![0u8; MM_SIZE];
        data[PLAYER_FORM] = 99; // Invalid form

        let save = MmSave::from_save_data(&data).unwrap();
        assert_eq!(save.player_form, PlayerForm::Human);
    }

    #[test]
    fn test_parse_invalid_magic_defaults_to_none() {
        use offsets::*;

        let mut data = vec![0u8; MM_SIZE];
        data[MAGIC_LEVEL] = 99; // Invalid magic level

        let save = MmSave::from_save_data(&data).unwrap();
        assert_eq!(save.magic, MmMagicCapacity::None);
    }

    #[test]
    fn test_parse_invalid_sword_defaults_to_none() {
        use offsets::*;

        let mut data = vec![0u8; MM_SIZE];
        data[SWORD_SHIELD] = 0x0F; // Invalid sword value (15)

        let save = MmSave::from_save_data(&data).unwrap();
        assert_eq!(save.sword, MmSword::None);
    }

    #[test]
    fn test_parse_invalid_shield_defaults_to_none() {
        use offsets::*;

        let mut data = vec![0u8; MM_SIZE];
        data[SWORD_SHIELD] = 0xF0; // Invalid shield value (15 << 4)

        let save = MmSave::from_save_data(&data).unwrap();
        assert_eq!(save.shield, MmShield::None);
    }

    #[test]
    fn test_heart_pieces_parsing() {
        let mut quest = MmQuestItems::empty();
        assert_eq!(quest.heart_pieces(), 0);

        quest.insert(MmQuestItems::HEART_PIECE_1);
        assert_eq!(quest.heart_pieces(), 1);

        quest.insert(MmQuestItems::HEART_PIECE_2);
        assert_eq!(quest.heart_pieces(), 3);

        quest.insert(MmQuestItems::HEART_PIECE_3);
        assert_eq!(quest.heart_pieces(), 7);

        // Test maximum heart pieces
        quest = MmQuestItems::HEART_PIECE_1
            | MmQuestItems::HEART_PIECE_2
            | MmQuestItems::HEART_PIECE_3
            | MmQuestItems::HEART_PIECE_4;
        assert_eq!(quest.heart_pieces(), 15);
    }

    #[test]
    fn test_bombers_notebook_accessor() {
        let mut save = MmSave::default();
        assert!(!save.has_bombers_notebook());

        save.quest_items.insert(MmQuestItems::NOTEBOOK);
        assert!(save.has_bombers_notebook());
    }

    #[test]
    fn test_bombers_notebook_from_raw() {
        use offsets::*;

        let mut data = vec![0u8; MM_SIZE];
        let quest_bits: u32 = MmQuestItems::NOTEBOOK.bits();
        data[QUEST_ITEMS..QUEST_ITEMS + 4].copy_from_slice(&quest_bits.to_be_bytes());

        let save = MmSave::from_save_data(&data).unwrap();
        assert!(save.has_bombers_notebook());
    }

    #[test]
    fn test_dungeon_items_get_method() {
        let items = MmAllDungeonItems {
            woodfall: MmDungeonItems::MAP | MmDungeonItems::COMPASS,
            snowhead: MmDungeonItems::BOSS_KEY,
            great_bay: MmDungeonItems::MAP,
            stone_tower: MmDungeonItems::all(),
        };

        assert_eq!(items.get(0), MmDungeonItems::MAP | MmDungeonItems::COMPASS);
        assert_eq!(items.get(1), MmDungeonItems::BOSS_KEY);
        assert_eq!(items.get(2), MmDungeonItems::MAP);
        assert_eq!(items.get(3), MmDungeonItems::all());
        assert_eq!(items.get(4), MmDungeonItems::default()); // Out of bounds
        assert_eq!(items.get(99), MmDungeonItems::default()); // Way out of bounds
    }

    #[test]
    fn test_cycle_scene_flags_parsing() {
        use offsets::*;

        let mut data = vec![0u8; MM_SIZE];

        // Set first cycle scene's chest flags
        let chest_flags: u32 = 0xDEADBEEF;
        data[CYCLE_SCENE_FLAGS..CYCLE_SCENE_FLAGS + 4].copy_from_slice(&chest_flags.to_be_bytes());

        let save = MmSave::from_save_data(&data).unwrap();
        assert_eq!(save.cycle_scene_flags.len(), 120);
        assert_eq!(save.cycle_scene_flags[0].chest, 0xDEADBEEF);
    }

    #[test]
    fn test_decode_error_variants() {
        // Test various decode error types exist and can be created
        let _err = MmDecodeError::AssertEq {
            offset: 100,
            expected: 0,
            found: 1,
        };

        let _err = MmDecodeError::AssertEqRange {
            start: 0,
            end: 4,
            expected: vec![0, 0, 0, 0],
            found: vec![1, 2, 3, 4],
        };

        let _err = MmDecodeError::Index(42);

        let _err = MmDecodeError::IndexRange { start: 0, end: 100 };

        let _err = MmDecodeError::Size(50);

        let _err = MmDecodeError::UnexpectedValue {
            offset: 10,
            field: "test_field",
            value: 99,
        };

        let _err = MmDecodeError::UnexpectedValueRange {
            start: 0,
            end: 4,
            field: "test_field",
            value: vec![1, 2, 3, 4],
        };
    }

    #[test]
    fn test_from_save_data_exactly_mm_size() {
        // Test with exactly MM_SIZE bytes
        let data = vec![0u8; MM_SIZE];
        assert!(MmSave::from_save_data(&data).is_ok());
    }

    #[test]
    fn test_from_save_data_larger_than_mm_size() {
        // Data larger than MM_SIZE should still work (only reads MM_SIZE)
        let data = vec![0u8; MM_SIZE + 100];
        assert!(MmSave::from_save_data(&data).is_ok());
    }

    #[test]
    fn test_to_save_data_produces_correct_size() {
        let save = MmSave::default();
        let data = save.to_save_data();
        assert_eq!(data.len(), MM_SIZE);
    }

    #[test]
    fn test_parse_upgrades_from_raw() {
        use offsets::*;

        let mut data = vec![0u8; MM_SIZE];

        let upgrade_bits: u32 = MmUpgrades::ADULTS_WALLET.bits()
            | MmUpgrades::BOMB_BAG_30.bits()
            | MmUpgrades::QUIVER_40.bits();
        data[UPGRADES..UPGRADES + 4].copy_from_slice(&upgrade_bits.to_be_bytes());

        let save = MmSave::from_save_data(&data).unwrap();

        assert_eq!(save.upgrades.wallet(), MmUpgrades::ADULTS_WALLET);
        assert_eq!(save.upgrades.bomb_bag(), MmUpgrades::BOMB_BAG_30);
        assert_eq!(save.upgrades.quiver(), MmUpgrades::QUIVER_40);
    }

    #[test]
    fn test_upgrades_set_wallet() {
        let mut upgrades = MmUpgrades::empty();

        upgrades.set_wallet(MmUpgrades::ADULTS_WALLET);
        assert_eq!(upgrades.wallet(), MmUpgrades::ADULTS_WALLET);

        upgrades.set_wallet(MmUpgrades::GIANTS_WALLET);
        assert_eq!(upgrades.wallet(), MmUpgrades::GIANTS_WALLET);

        // Setting to empty should clear
        upgrades.set_wallet(MmUpgrades::empty());
        assert_eq!(upgrades.wallet(), MmUpgrades::empty());
    }

    #[test]
    fn test_double_defense_parsing() {
        use offsets::*;

        let mut data = vec![0u8; MM_SIZE];

        // No double defense
        data[DOUBLE_DEFENSE] = 0;
        let save = MmSave::from_save_data(&data).unwrap();
        assert!(!save.double_defense);

        // Has double defense (any non-zero value)
        data[DOUBLE_DEFENSE] = 1;
        let save = MmSave::from_save_data(&data).unwrap();
        assert!(save.double_defense);

        data[DOUBLE_DEFENSE] = 255;
        let save = MmSave::from_save_data(&data).unwrap();
        assert!(save.double_defense);
    }

    #[test]
    #[allow(clippy::field_reassign_with_default)]
    fn test_heart_containers_calculation() {
        let mut save = MmSave::default();

        // 3 hearts = 0x30 (48 in decimal, 48/16 = 3)
        save.health_capacity = 0x30;
        assert_eq!(save.heart_containers(), 3, "3 hearts should equal 0x30");

        // 20 hearts = 0x140 (320 in decimal, 320/16 = 20)
        save.health_capacity = 0x140;
        assert_eq!(save.heart_containers(), 20, "20 hearts should equal 0x140");

        // 10 hearts = 0xA0 (160 in decimal, 160/16 = 10)
        save.health_capacity = 0xA0;
        assert_eq!(save.heart_containers(), 10, "10 hearts should equal 0xA0");

        // Partial hearts should round down
        // 0x35 = 53, 53/16 = 3 (rounds down from 3.3125)
        save.health_capacity = 0x35;
        assert_eq!(
            save.heart_containers(),
            3,
            "Partial hearts should round down"
        );

        // 0x4F = 79, 79/16 = 4 (rounds down from 4.9375)
        save.health_capacity = 0x4F;
        assert_eq!(
            save.heart_containers(),
            4,
            "Partial hearts should round down to 4"
        );

        // Zero hearts edge case
        save.health_capacity = 0;
        assert_eq!(
            save.heart_containers(),
            0,
            "Zero health_capacity = 0 hearts"
        );
    }

    #[test]
    fn test_heart_containers_parsing() {
        use offsets::*;

        let mut data = vec![0u8; MM_SIZE];

        // Set health_capacity at offset HEALTH_CAPACITY (0x002C) - big-endian u16
        // 3 hearts = 0x0030
        data[HEALTH_CAPACITY] = 0x00;
        data[HEALTH_CAPACITY + 1] = 0x30;

        let save = MmSave::from_save_data(&data).unwrap();
        assert_eq!(save.health_capacity, 0x30, "Should parse 3 hearts (0x30)");
        assert_eq!(save.heart_containers(), 3);

        // Test 20 hearts = 0x0140
        data[HEALTH_CAPACITY] = 0x01;
        data[HEALTH_CAPACITY + 1] = 0x40;

        let save = MmSave::from_save_data(&data).unwrap();
        assert_eq!(
            save.health_capacity, 0x140,
            "Should parse 20 hearts (0x140)"
        );
        assert_eq!(save.heart_containers(), 20);

        // Test 12 hearts = 0x00C0
        data[HEALTH_CAPACITY] = 0x00;
        data[HEALTH_CAPACITY + 1] = 0xC0;

        let save = MmSave::from_save_data(&data).unwrap();
        assert_eq!(save.health_capacity, 0xC0, "Should parse 12 hearts (0xC0)");
        assert_eq!(save.heart_containers(), 12);
    }

    #[test]
    fn test_heart_pieces_method() {
        // Test the heart_pieces() method on MmQuestItems
        let mut quest_items = MmQuestItems::empty();

        // No heart pieces
        assert_eq!(quest_items.heart_pieces(), 0);

        // 1 heart piece
        quest_items = MmQuestItems::HEART_PIECE_1;
        assert_eq!(quest_items.heart_pieces(), 1);

        // 2 heart pieces
        quest_items = MmQuestItems::HEART_PIECE_1 | MmQuestItems::HEART_PIECE_2;
        assert_eq!(quest_items.heart_pieces(), 3); // bits 28+29 set = 0011 = 3

        // Actually test the bit counting correctly
        // HEART_PIECE_1 = 1 << 28, HEART_PIECE_2 = 1 << 29
        // The heart_pieces() method does (bits >> 28) & 0xF

        // Test individual piece bits
        quest_items = MmQuestItems::from_bits_truncate(1 << 28);
        assert_eq!(quest_items.heart_pieces(), 1, "Bit 28 should be 1 piece");

        quest_items = MmQuestItems::from_bits_truncate(2 << 28);
        assert_eq!(quest_items.heart_pieces(), 2, "Bits should be 2 pieces");

        quest_items = MmQuestItems::from_bits_truncate(3 << 28);
        assert_eq!(quest_items.heart_pieces(), 3, "Bits should be 3 pieces");

        // Test with other quest items set
        quest_items = MmQuestItems::REMAINS_ODOLWA
            | MmQuestItems::SONG_TIME
            | MmQuestItems::from_bits_truncate(2 << 28);
        assert_eq!(
            quest_items.heart_pieces(),
            2,
            "Should have 2 heart pieces with other items"
        );
    }
}
