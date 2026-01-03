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
    // Masks start at 0x32
    pub const MASK_POSTMAN: u8 = 0x32;
    pub const MASK_ALL_NIGHT: u8 = 0x33;
    pub const MASK_BLAST: u8 = 0x34;
    pub const MASK_STONE: u8 = 0x35;
    pub const MASK_GREAT_FAIRY: u8 = 0x36;
    pub const MASK_DEKU: u8 = 0x37;
    pub const MASK_KEATON: u8 = 0x38;
    pub const MASK_BREMEN: u8 = 0x39;
    pub const MASK_BUNNY: u8 = 0x3A;
    pub const MASK_DON_GERO: u8 = 0x3B;
    pub const MASK_SCENTS: u8 = 0x3C;
    pub const MASK_GORON: u8 = 0x3D;
    pub const MASK_ROMANI: u8 = 0x3E;
    pub const MASK_CIRCUS_LEADER: u8 = 0x3F;
    pub const MASK_KAFEI: u8 = 0x40;
    pub const MASK_COUPLES: u8 = 0x41;
    pub const MASK_TRUTH: u8 = 0x42;
    pub const MASK_ZORA: u8 = 0x43;
    pub const MASK_KAMARO: u8 = 0x44;
    pub const MASK_GIBDO: u8 = 0x45;
    pub const MASK_GARO: u8 = 0x46;
    pub const MASK_CAPTAIN: u8 = 0x47;
    pub const MASK_GIANT: u8 = 0x48;
    pub const MASK_FIERCE_DEITY: u8 = 0x49;
    pub const NONE: u8 = 0xFF;
}

// ============================================================================
// MM Save Structure Offsets (relative to SaveContext start)
// ============================================================================

/// Memory offsets within MM SaveContext
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
    pub const PERM_SCENE_FLAGS: usize = 0x00F0;
    /// Cycle scene flags start (after perm flags)
    pub const CYCLE_SCENE_FLAGS: usize = 0x0DF0;
    /// Current day (u32)
    pub const DAY: usize = 0x0048;
    /// Current time (u16)
    pub const TIME: usize = 0x004C;
    /// Is night flag (derived from time, but sometimes stored)
    pub const IS_NIGHT: usize = 0x0050;

    // Size constants for arrays
    pub const MASK_SLOTS: usize = 24;
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
}

// ============================================================================
// MmSave Mask Accessor Methods
// ============================================================================

impl MmSave {
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
    // Collectible Mask Accessors (MmMasksLow)
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

    /// Returns true if the player has the Don Gero's Mask
    pub fn has_don_gero_mask(&self) -> bool {
        self.masks.masks_low.contains(MmMasksLow::DON_GERO)
    }

    /// Returns true if the player has the Mask of Scents
    pub fn has_mask_of_scents(&self) -> bool {
        self.masks.masks_low.contains(MmMasksLow::SCENTS)
    }

    /// Returns true if the player has the Romani's Mask
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

    // ========================================================================
    // Collectible Mask Accessors (MmMasksHigh)
    // ========================================================================

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
        let save = MmSave::from_save_data(data)?;
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
        self.save = MmSave::from_save_data(data)?;
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
        let mut original = MmSave::default();
        original.player_form = PlayerForm::Goron;
        original.health_capacity = 0x0140;
        original.health = 0x0100;
        original.magic = MmMagicCapacity::Double;
        original.double_defense = true;
        original.rupees = 500;
        original.sword = MmSword::GildedSword;
        original.shield = MmShield::MirrorShield;
        original.quest_items = MmQuestItems::REMAINS_ODOLWA | MmQuestItems::SONG_TIME;
        original.upgrades = MmUpgrades::ADULTS_WALLET;
        original.dungeon_items.woodfall = MmDungeonItems::MAP | MmDungeonItems::COMPASS;
        original.small_keys.woodfall = 2;
        original.stray_fairies.woodfall = 15;
        original.skull_tokens_swamp = 20;
        original.skull_tokens_ocean = 10;
        original.day = 2;
        original.time = 0x8000;
        original.is_night = true;

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

        // All masks should be false by default
        assert!(!save.has_deku_mask());
        assert!(!save.has_goron_mask());
        assert!(!save.has_zora_mask());
        assert!(!save.has_fierce_deity_mask());
    }

    #[test]
    fn test_transformation_mask_accessors_with_masks() {
        let mut save = MmSave::default();

        // Set Deku mask
        save.masks
            .transformation
            .insert(MmTransformationMasks::DEKU);
        assert!(save.has_deku_mask());
        assert!(!save.has_goron_mask());
        assert!(!save.has_zora_mask());
        assert!(!save.has_fierce_deity_mask());

        // Add Goron mask
        save.masks
            .transformation
            .insert(MmTransformationMasks::GORON);
        assert!(save.has_deku_mask());
        assert!(save.has_goron_mask());
        assert!(!save.has_zora_mask());
        assert!(!save.has_fierce_deity_mask());

        // Add Zora mask
        save.masks
            .transformation
            .insert(MmTransformationMasks::ZORA);
        assert!(save.has_deku_mask());
        assert!(save.has_goron_mask());
        assert!(save.has_zora_mask());
        assert!(!save.has_fierce_deity_mask());

        // Add Fierce Deity mask
        save.masks
            .transformation
            .insert(MmTransformationMasks::FIERCE_DEITY);
        assert!(save.has_deku_mask());
        assert!(save.has_goron_mask());
        assert!(save.has_zora_mask());
        assert!(save.has_fierce_deity_mask());
    }

    #[test]
    fn test_collectible_mask_accessors_low_default() {
        let save = MmSave::default();

        // All masks should be false by default
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
    }

    #[test]
    fn test_collectible_mask_accessors_low_with_masks() {
        let mut save = MmSave::default();

        // Set individual masks and verify
        save.masks.masks_low.insert(MmMasksLow::POSTMAN);
        assert!(save.has_postman_hat());

        save.masks.masks_low.insert(MmMasksLow::ALL_NIGHT);
        assert!(save.has_all_night_mask());

        save.masks.masks_low.insert(MmMasksLow::BLAST);
        assert!(save.has_blast_mask());

        save.masks.masks_low.insert(MmMasksLow::STONE);
        assert!(save.has_stone_mask());

        save.masks.masks_low.insert(MmMasksLow::GREAT_FAIRY);
        assert!(save.has_great_fairy_mask());

        save.masks.masks_low.insert(MmMasksLow::KEATON);
        assert!(save.has_keaton_mask());

        save.masks.masks_low.insert(MmMasksLow::BREMEN);
        assert!(save.has_bremen_mask());

        save.masks.masks_low.insert(MmMasksLow::BUNNY);
        assert!(save.has_bunny_hood());

        save.masks.masks_low.insert(MmMasksLow::DON_GERO);
        assert!(save.has_don_gero_mask());

        save.masks.masks_low.insert(MmMasksLow::SCENTS);
        assert!(save.has_mask_of_scents());

        save.masks.masks_low.insert(MmMasksLow::ROMANI);
        assert!(save.has_romani_mask());

        save.masks.masks_low.insert(MmMasksLow::CIRCUS_LEADER);
        assert!(save.has_circus_leader_mask());

        save.masks.masks_low.insert(MmMasksLow::KAFEI);
        assert!(save.has_kafei_mask());

        save.masks.masks_low.insert(MmMasksLow::COUPLES);
        assert!(save.has_couples_mask());

        save.masks.masks_low.insert(MmMasksLow::TRUTH);
        assert!(save.has_mask_of_truth());

        save.masks.masks_low.insert(MmMasksLow::KAMARO);
        assert!(save.has_kamaro_mask());
    }

    #[test]
    fn test_collectible_mask_accessors_high_default() {
        let save = MmSave::default();

        // All masks should be false by default
        assert!(!save.has_gibdo_mask());
        assert!(!save.has_garo_mask());
        assert!(!save.has_captain_hat());
        assert!(!save.has_giant_mask());
    }

    #[test]
    fn test_collectible_mask_accessors_high_with_masks() {
        let mut save = MmSave::default();

        // Set individual masks and verify
        save.masks.masks_high.insert(MmMasksHigh::GIBDO);
        assert!(save.has_gibdo_mask());
        assert!(!save.has_garo_mask());
        assert!(!save.has_captain_hat());
        assert!(!save.has_giant_mask());

        save.masks.masks_high.insert(MmMasksHigh::GARO);
        assert!(save.has_gibdo_mask());
        assert!(save.has_garo_mask());
        assert!(!save.has_captain_hat());
        assert!(!save.has_giant_mask());

        save.masks.masks_high.insert(MmMasksHigh::CAPTAIN);
        assert!(save.has_gibdo_mask());
        assert!(save.has_garo_mask());
        assert!(save.has_captain_hat());
        assert!(!save.has_giant_mask());

        save.masks.masks_high.insert(MmMasksHigh::GIANT);
        assert!(save.has_gibdo_mask());
        assert!(save.has_garo_mask());
        assert!(save.has_captain_hat());
        assert!(save.has_giant_mask());
    }

    #[test]
    fn test_mask_accessors_with_sample_data() {
        let stub = MmSaveStub::with_sample_data();
        let save = stub.get_save();

        // Sample data has Deku, Goron, Zora but not Fierce Deity
        assert!(save.has_deku_mask());
        assert!(save.has_goron_mask());
        assert!(save.has_zora_mask());
        assert!(!save.has_fierce_deity_mask());

        // Sample data has Bunny, Stone, Great Fairy, Bremen
        assert!(save.has_bunny_hood());
        assert!(save.has_stone_mask());
        assert!(save.has_great_fairy_mask());
        assert!(save.has_bremen_mask());

        // Sample data should NOT have these
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

    #[test]
    fn test_all_masks_combined() {
        let mut save = MmSave::default();

        // Set all transformation masks
        save.masks.transformation = MmTransformationMasks::DEKU
            | MmTransformationMasks::GORON
            | MmTransformationMasks::ZORA
            | MmTransformationMasks::FIERCE_DEITY;

        // Set all low masks
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

        // Set all high masks
        save.masks.masks_high =
            MmMasksHigh::GIBDO | MmMasksHigh::GARO | MmMasksHigh::CAPTAIN | MmMasksHigh::GIANT;

        // Verify all transformation masks
        assert!(save.has_deku_mask());
        assert!(save.has_goron_mask());
        assert!(save.has_zora_mask());
        assert!(save.has_fierce_deity_mask());

        // Verify all low masks
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

        // Verify all high masks
        assert!(save.has_gibdo_mask());
        assert!(save.has_garo_mask());
        assert!(save.has_captain_hat());
        assert!(save.has_giant_mask());

        // Verify total count: 4 transformation + 16 low + 4 high = 24 masks
        assert_eq!(save.masks.total_mask_count(), 24);
    }

    #[test]
    fn test_mask_accessors_from_parsed_save_data() {
        use offsets::*;

        let mut data = vec![0u8; MM_SIZE];

        // Set mask slot data - place various masks in slots
        // Slot layout follows MM's mask inventory
        data[MASKS] = mm_item_ids::MASK_POSTMAN;
        data[MASKS + 1] = mm_item_ids::MASK_ALL_NIGHT;
        data[MASKS + 2] = mm_item_ids::MASK_BLAST;
        data[MASKS + 3] = mm_item_ids::MASK_STONE;
        data[MASKS + 4] = mm_item_ids::MASK_GREAT_FAIRY;
        data[MASKS + 5] = mm_item_ids::MASK_DEKU;
        data[MASKS + 6] = mm_item_ids::MASK_GORON;
        data[MASKS + 7] = mm_item_ids::MASK_ZORA;
        data[MASKS + 8] = mm_item_ids::MASK_FIERCE_DEITY;
        data[MASKS + 9] = mm_item_ids::MASK_GIBDO;
        data[MASKS + 10] = mm_item_ids::MASK_GARO;
        data[MASKS + 11] = mm_item_ids::MASK_CAPTAIN;
        data[MASKS + 12] = mm_item_ids::MASK_GIANT;

        let save = MmSave::from_save_data(&data).unwrap();

        // Verify transformation masks
        assert!(save.has_deku_mask());
        assert!(save.has_goron_mask());
        assert!(save.has_zora_mask());
        assert!(save.has_fierce_deity_mask());

        // Verify collectible masks (low)
        assert!(save.has_postman_hat());
        assert!(save.has_all_night_mask());
        assert!(save.has_blast_mask());
        assert!(save.has_stone_mask());
        assert!(save.has_great_fairy_mask());

        // Verify collectible masks (high)
        assert!(save.has_gibdo_mask());
        assert!(save.has_garo_mask());
        assert!(save.has_captain_hat());
        assert!(save.has_giant_mask());

        // Verify masks NOT in the save data
        assert!(!save.has_keaton_mask());
        assert!(!save.has_bremen_mask());
        assert!(!save.has_bunny_hood());
        assert!(!save.has_romani_mask());
    }
}
