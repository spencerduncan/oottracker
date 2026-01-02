//! Majora's Mask save data structures and trait definitions.
//!
//! This module provides the trait interface and stub implementation for MM save data.
//! The stub enables parallel UI development while actual memory reading is implemented.
//!
//! Reference: OoTMM source - packages/core/include/combo/mm/save.h
//! MM SaveContext address: 0x801ef670 (size 0x48d0 = 18,640 bytes)

use {bitflags::bitflags, derivative::Derivative};

/// MM SaveContext base address in N64 memory
pub const MM_ADDR: u32 = 0x801ef670;
/// MM SaveContext size in bytes
pub const MM_SIZE: usize = 0x48d0;

// ============================================================================
// Memory Layout Offsets (used by TryFrom<&[u8]> for MmSave)
// ============================================================================
/// Time of day (u16)
const OFFSET_TIME: usize = 0x000c;
/// Is it night? (s32)
const OFFSET_IS_NIGHT: usize = 0x0010;
/// Current day (u32)
const OFFSET_DAY: usize = 0x0018;
/// Player form (u8)
const OFFSET_PLAYER_FORM: usize = 0x0020;
/// Start of MmSaveInfo structure
const OFFSET_INFO: usize = 0x0024;
/// Health capacity within info (s16)
const INFO_HEALTH_CAPACITY: usize = 0x10;
/// Current health within info (s16)
const INFO_HEALTH: usize = 0x12;
/// Magic level within info (s8)
const INFO_MAGIC_LEVEL: usize = 0x14;
/// Rupees within info (s16)
const INFO_RUPEES: usize = 0x16;
/// Double defense within info (u8)
const INFO_DOUBLE_DEFENSE: usize = 0x1e;
/// Equipment bitfield within info (u16)
const INFO_EQUIPMENT: usize = 0x48;
/// Items array within info (48 bytes)
const INFO_ITEMS: usize = 0x4c;
/// Upgrades within info (u32)
const INFO_UPGRADES: usize = 0x94;
/// Quest items within info (u32)
const INFO_QUEST: usize = 0x98;
/// Dungeon items within info (10 bytes)
const INFO_DUNGEON_ITEMS: usize = 0x9c;
/// Dungeon keys within info (9 bytes)
const INFO_DUNGEON_KEYS: usize = 0xa6;
/// Stray fairies within info (10 bytes)
const INFO_STRAY_FAIRIES: usize = 0xb0;
/// Swamp skulltula count within info (u16)
const INFO_SKULL_SWAMP: usize = 0xec0;
/// Ocean skulltula count within info (u16)
const INFO_SKULL_OCEAN: usize = 0xec2;
/// Number of bottle slots
const NUM_BOTTLES: usize = 6;
/// First bottle slot index
const SLOT_BOTTLE_START: usize = 18;

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
        // MM bottle item IDs
        const NONE: u8 = 0xFF;
        const EMPTY: u8 = 0x12;
        const RED_POTION: u8 = 0x13;
        const GREEN_POTION: u8 = 0x14;
        const BLUE_POTION: u8 = 0x15;
        const FAIRY: u8 = 0x16;
        const DEKU_PRINCESS: u8 = 0x17;
        const MILK: u8 = 0x18;
        const MILK_HALF: u8 = 0x19;
        const FISH: u8 = 0x1A;
        const BUG: u8 = 0x1B;
        const BLUE_FIRE: u8 = 0x1C;
        const POE: u8 = 0x1D;
        const BIG_POE: u8 = 0x1E;
        const WATER: u8 = 0x1F;
        const HOT_SPRING_WATER: u8 = 0x20;
        const ZORA_EGG: u8 = 0x21;
        const GOLD_DUST: u8 = 0x22;
        const MAGICAL_MUSHROOM: u8 = 0x23;
        const SEA_HORSE: u8 = 0x24;
        const CHATEAU_ROMANI: u8 = 0x25;
        const MYSTERY_MILK: u8 = 0x26;
        const MYSTERY_MILK_SPOILED: u8 = 0x27;

        match value {
            NONE => Ok(MmBottle::None),
            EMPTY => Ok(MmBottle::Empty),
            RED_POTION => Ok(MmBottle::RedPotion),
            GREEN_POTION => Ok(MmBottle::GreenPotion),
            BLUE_POTION => Ok(MmBottle::BluePotion),
            FAIRY => Ok(MmBottle::Fairy),
            DEKU_PRINCESS => Ok(MmBottle::DekuPrincess),
            MILK => Ok(MmBottle::Milk),
            MILK_HALF => Ok(MmBottle::MilkHalf),
            FISH => Ok(MmBottle::Fish),
            BUG => Ok(MmBottle::Bug),
            BLUE_FIRE => Ok(MmBottle::BlueFire),
            POE => Ok(MmBottle::Poe),
            BIG_POE => Ok(MmBottle::BigPoe),
            WATER => Ok(MmBottle::Water),
            HOT_SPRING_WATER => Ok(MmBottle::HotSpringWater),
            ZORA_EGG => Ok(MmBottle::ZoraEgg),
            GOLD_DUST => Ok(MmBottle::GoldDust),
            MAGICAL_MUSHROOM => Ok(MmBottle::MagicalMushroom),
            SEA_HORSE => Ok(MmBottle::SeaHorse),
            CHATEAU_ROMANI => Ok(MmBottle::ChateauRomani),
            MYSTERY_MILK => Ok(MmBottle::MysteryMilk),
            MYSTERY_MILK_SPOILED => Ok(MmBottle::MysteryMilkSpoiled),
            _ => Err(value),
        }
    }
}

impl From<MmBottle> for u8 {
    fn from(bottle: MmBottle) -> u8 {
        match bottle {
            MmBottle::None => 0xFF,
            MmBottle::Empty => 0x12,
            MmBottle::RedPotion => 0x13,
            MmBottle::GreenPotion => 0x14,
            MmBottle::BluePotion => 0x15,
            MmBottle::Fairy => 0x16,
            MmBottle::DekuPrincess => 0x17,
            MmBottle::Milk => 0x18,
            MmBottle::MilkHalf => 0x19,
            MmBottle::Fish => 0x1A,
            MmBottle::Bug => 0x1B,
            MmBottle::BlueFire => 0x1C,
            MmBottle::Poe => 0x1D,
            MmBottle::BigPoe => 0x1E,
            MmBottle::Water => 0x1F,
            MmBottle::HotSpringWater => 0x20,
            MmBottle::ZoraEgg => 0x21,
            MmBottle::GoldDust => 0x22,
            MmBottle::MagicalMushroom => 0x23,
            MmBottle::SeaHorse => 0x24,
            MmBottle::ChateauRomani => 0x25,
            MmBottle::MysteryMilk => 0x26,
            MmBottle::MysteryMilkSpoiled => 0x27,
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
// Save Data Parsing
// ============================================================================

/// Error type for MM save data decoding
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MmDecodeError {
    /// Input data is too small
    DataTooSmall { expected: usize, actual: usize },
    /// Invalid player form value
    InvalidPlayerForm(u8),
    /// Invalid magic capacity value
    InvalidMagicCapacity(u8),
}

impl std::fmt::Display for MmDecodeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MmDecodeError::DataTooSmall { expected, actual } => {
                write!(
                    f,
                    "data too small: expected {} bytes, got {}",
                    expected, actual
                )
            }
            MmDecodeError::InvalidPlayerForm(v) => write!(f, "invalid player form: {}", v),
            MmDecodeError::InvalidMagicCapacity(v) => write!(f, "invalid magic capacity: {}", v),
        }
    }
}

impl std::error::Error for MmDecodeError {}

/// Helper to read a big-endian u16 from a byte slice
fn read_u16_be(data: &[u8], offset: usize) -> u16 {
    u16::from_be_bytes([data[offset], data[offset + 1]])
}

/// Helper to read a big-endian u32 from a byte slice
fn read_u32_be(data: &[u8], offset: usize) -> u32 {
    u32::from_be_bytes([
        data[offset],
        data[offset + 1],
        data[offset + 2],
        data[offset + 3],
    ])
}

/// Helper to read a big-endian i16 from a byte slice
fn read_i16_be(data: &[u8], offset: usize) -> i16 {
    i16::from_be_bytes([data[offset], data[offset + 1]])
}

/// Helper to read a big-endian i32 from a byte slice
fn read_i32_be(data: &[u8], offset: usize) -> i32 {
    i32::from_be_bytes([
        data[offset],
        data[offset + 1],
        data[offset + 2],
        data[offset + 3],
    ])
}

impl TryFrom<&[u8]> for MmSave {
    type Error = MmDecodeError;

    fn try_from(data: &[u8]) -> Result<Self, Self::Error> {
        // Minimum size check - we need at least enough for basic fields
        let min_size = OFFSET_INFO + INFO_SKULL_OCEAN + 2;
        if data.len() < min_size {
            return Err(MmDecodeError::DataTooSmall {
                expected: min_size,
                actual: data.len(),
            });
        }

        // Parse basic fields
        let time = read_u16_be(data, OFFSET_TIME);
        let is_night = read_i32_be(data, OFFSET_IS_NIGHT) != 0;
        let day = read_u32_be(data, OFFSET_DAY);

        let player_form_raw = data[OFFSET_PLAYER_FORM];
        let player_form =
            PlayerForm::try_from(player_form_raw).map_err(MmDecodeError::InvalidPlayerForm)?;

        // Info section base offset
        let info = OFFSET_INFO;

        // Health and magic
        let health_capacity = read_i16_be(data, info + INFO_HEALTH_CAPACITY) as u16;
        let health = read_i16_be(data, info + INFO_HEALTH) as u16;
        let magic_raw = data[info + INFO_MAGIC_LEVEL];
        let magic =
            MmMagicCapacity::try_from(magic_raw).map_err(MmDecodeError::InvalidMagicCapacity)?;
        let rupees = read_i16_be(data, info + INFO_RUPEES) as u16;
        let double_defense = data[info + INFO_DOUBLE_DEFENSE] != 0;

        // Equipment (sword and shield from bitfield)
        let equipment = read_u16_be(data, info + INFO_EQUIPMENT);
        let sword_raw = ((equipment >> 0) & 0xF) as u8;
        let shield_raw = ((equipment >> 4) & 0xF) as u8;
        let sword = MmSword::try_from(sword_raw).unwrap_or(MmSword::None);
        let shield = MmShield::try_from(shield_raw).unwrap_or(MmShield::None);

        // Parse inventory items
        let items_base = info + INFO_ITEMS;
        let inventory = parse_inventory(data, items_base);

        // Upgrades and quest items
        let upgrades = MmUpgrades::from_bits_truncate(read_u32_be(data, info + INFO_UPGRADES));
        let quest_items = MmQuestItems::from_bits_truncate(read_u32_be(data, info + INFO_QUEST));

        // Dungeon items
        let dungeon_items_base = info + INFO_DUNGEON_ITEMS;
        let dungeon_items = MmAllDungeonItems {
            woodfall: MmDungeonItems::from_bits_truncate(data[dungeon_items_base] & 0x07),
            snowhead: MmDungeonItems::from_bits_truncate(data[dungeon_items_base + 1] & 0x07),
            great_bay: MmDungeonItems::from_bits_truncate(data[dungeon_items_base + 2] & 0x07),
            stone_tower: MmDungeonItems::from_bits_truncate(data[dungeon_items_base + 3] & 0x07),
        };

        // Dungeon keys
        let keys_base = info + INFO_DUNGEON_KEYS;
        let small_keys = MmSmallKeys {
            woodfall: data[keys_base] as u8,
            snowhead: data[keys_base + 1] as u8,
            great_bay: data[keys_base + 2] as u8,
            stone_tower: data[keys_base + 3] as u8,
        };

        // Stray fairies
        let fairies_base = info + INFO_STRAY_FAIRIES;
        let stray_fairies = MmStrayFairies {
            clock_town: data[fairies_base] as u8,
            woodfall: data[fairies_base + 1] as u8,
            snowhead: data[fairies_base + 2] as u8,
            great_bay: data[fairies_base + 3] as u8,
            stone_tower: data[fairies_base + 4] as u8,
        };

        // Skulltula tokens
        let skull_tokens_swamp = read_u16_be(data, info + INFO_SKULL_SWAMP);
        let skull_tokens_ocean = read_u16_be(data, info + INFO_SKULL_OCEAN);

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
            masks: MmMasks::default(), // Parsed from inventory above
            upgrades,
            quest_items,
            dungeon_items,
            small_keys,
            stray_fairies,
            skull_tokens_swamp,
            skull_tokens_ocean,
            permanent_scene_flags: Vec::new(), // Not parsing scene flags for now
            cycle_scene_flags: Vec::new(),
            day,
            time,
            is_night,
        })
    }
}

impl TryFrom<Vec<u8>> for MmSave {
    type Error = MmDecodeError;

    fn try_from(data: Vec<u8>) -> Result<Self, Self::Error> {
        Self::try_from(data.as_slice())
    }
}

/// Parse inventory from raw bytes
fn parse_inventory(data: &[u8], base: usize) -> MmInventory {
    // Item slot constants
    const SLOT_OCARINA: usize = 0;
    const SLOT_BOW: usize = 1;
    const SLOT_FIRE_ARROWS: usize = 2;
    const SLOT_ICE_ARROWS: usize = 3;
    const SLOT_LIGHT_ARROWS: usize = 4;
    const SLOT_BOMB: usize = 6;
    const SLOT_BOMBCHU: usize = 7;
    const SLOT_DEKU_STICK: usize = 8;
    const SLOT_DEKU_NUT: usize = 9;
    const SLOT_MAGIC_BEAN: usize = 10;
    const SLOT_POWDER_KEG: usize = 12;
    const SLOT_PICTOGRAPH_BOX: usize = 13;
    const SLOT_LENS: usize = 14;
    const SLOT_HOOKSHOT: usize = 15;
    const SLOT_GREAT_FAIRY_SWORD: usize = 16;

    const ITEM_NONE: u8 = 0xFF;

    let has_item = |slot: usize| data[base + slot] != ITEM_NONE;

    // Parse bottles
    let mut bottles = [MmBottle::None; 6];
    for i in 0..NUM_BOTTLES {
        let slot = SLOT_BOTTLE_START + i;
        bottles[i] = MmBottle::try_from(data[base + slot]).unwrap_or(MmBottle::None);
    }

    MmInventory {
        ocarina: has_item(SLOT_OCARINA),
        bow: has_item(SLOT_BOW),
        fire_arrows: has_item(SLOT_FIRE_ARROWS),
        ice_arrows: has_item(SLOT_ICE_ARROWS),
        light_arrows: has_item(SLOT_LIGHT_ARROWS),
        bombs: has_item(SLOT_BOMB),
        bombchus: has_item(SLOT_BOMBCHU),
        deku_sticks: has_item(SLOT_DEKU_STICK),
        deku_nuts: has_item(SLOT_DEKU_NUT),
        magic_beans: has_item(SLOT_MAGIC_BEAN),
        powder_keg: has_item(SLOT_POWDER_KEG),
        pictograph_box: has_item(SLOT_PICTOGRAPH_BOX),
        lens: has_item(SLOT_LENS),
        hookshot: has_item(SLOT_HOOKSHOT),
        great_fairy_sword: has_item(SLOT_GREAT_FAIRY_SWORD),
        bottles,
    }
}

// ============================================================================
// Tests
// ============================================================================

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

    #[test]
    fn test_bottle_conversion() {
        // Test TryFrom<u8>
        assert_eq!(MmBottle::try_from(0xFF), Ok(MmBottle::None));
        assert_eq!(MmBottle::try_from(0x12), Ok(MmBottle::Empty));
        assert_eq!(MmBottle::try_from(0x25), Ok(MmBottle::ChateauRomani));
        assert_eq!(MmBottle::try_from(0x00), Err(0x00)); // Invalid

        // Test From<MmBottle>
        assert_eq!(u8::from(MmBottle::None), 0xFF);
        assert_eq!(u8::from(MmBottle::Empty), 0x12);
        assert_eq!(u8::from(MmBottle::ChateauRomani), 0x25);
    }

    #[test]
    fn test_parse_save_data_too_small() {
        let small_data = vec![0u8; 100];
        let result = MmSave::try_from(small_data);
        assert!(matches!(result, Err(MmDecodeError::DataTooSmall { .. })));
    }

    #[test]
    fn test_parse_save_basic_fields() {
        // Create minimal valid save data
        let mut data = vec![0u8; 4096];

        // Set time at offset 0x0c (big-endian u16)
        data[OFFSET_TIME] = 0x80;
        data[OFFSET_TIME + 1] = 0x00;

        // Set is_night at offset 0x10 (big-endian i32)
        data[OFFSET_IS_NIGHT + 3] = 1; // is_night = true

        // Set day at offset 0x18 (big-endian u32)
        data[OFFSET_DAY + 3] = 2; // day = 2

        // Set player form at offset 0x20
        data[OFFSET_PLAYER_FORM] = 4; // Human

        // Set health capacity at info + 0x10 (big-endian i16)
        let info = OFFSET_INFO;
        data[info + INFO_HEALTH_CAPACITY] = 0x01;
        data[info + INFO_HEALTH_CAPACITY + 1] = 0x40; // 0x140 = 5 hearts

        // Set current health at info + 0x12 (big-endian i16)
        data[info + INFO_HEALTH] = 0x01;
        data[info + INFO_HEALTH + 1] = 0x00; // 0x100 = 4 hearts

        // Set magic level at info + 0x14
        data[info + INFO_MAGIC_LEVEL] = 2; // Double magic

        // Set rupees at info + 0x16 (big-endian i16)
        data[info + INFO_RUPEES] = 0x00;
        data[info + INFO_RUPEES + 1] = 0x63; // 99 rupees

        // Set double defense at info + 0x1e
        data[info + INFO_DOUBLE_DEFENSE] = 1;

        // Set equipment at info + 0x48 (big-endian u16)
        // sword:4 bits (low), shield:4 bits, tunic:4 bits, boots:4 bits
        data[info + INFO_EQUIPMENT] = 0x00;
        data[info + INFO_EQUIPMENT + 1] = 0x23; // sword=3 (gilded), shield=2 (mirror)

        // Set quest items at info + 0x98 (big-endian u32)
        // Set REMAINS_ODOLWA (bit 0) and SONG_TIME (bit 12)
        data[info + INFO_QUEST + 2] = 0x10; // bit 12 = SONG_TIME
        data[info + INFO_QUEST + 3] = 0x01; // bit 0 = REMAINS_ODOLWA

        // Fill item slots with 0xFF (empty)
        for i in 0..48 {
            data[info + INFO_ITEMS + i] = 0xFF;
        }

        // Set ocarina at slot 0
        data[info + INFO_ITEMS] = 0x00; // Ocarina item ID

        // Set hookshot at slot 15
        data[info + INFO_ITEMS + 15] = 0x0F; // Hookshot item ID

        // Set stray fairies
        data[info + INFO_STRAY_FAIRIES] = 1; // Clock Town
        data[info + INFO_STRAY_FAIRIES + 1] = 15; // Woodfall
        data[info + INFO_STRAY_FAIRIES + 2] = 8; // Snowhead

        let result = MmSave::try_from(data);
        assert!(result.is_ok(), "Failed to parse: {:?}", result.err());

        let save = result.unwrap();
        assert_eq!(save.time, 0x8000);
        assert!(save.is_night);
        assert_eq!(save.day, 2);
        assert_eq!(save.player_form, PlayerForm::Human);
        assert_eq!(save.health_capacity, 0x140);
        assert_eq!(save.health, 0x100);
        assert_eq!(save.magic, MmMagicCapacity::Double);
        assert_eq!(save.rupees, 99);
        assert!(save.double_defense);
        assert_eq!(save.sword, MmSword::GildedSword);
        assert_eq!(save.shield, MmShield::MirrorShield);
        assert!(save.quest_items.contains(MmQuestItems::REMAINS_ODOLWA));
        assert!(save.quest_items.contains(MmQuestItems::SONG_TIME));
        assert!(save.inventory.ocarina);
        assert!(save.inventory.hookshot);
        assert!(!save.inventory.bow);
        assert_eq!(save.stray_fairies.clock_town, 1);
        assert_eq!(save.stray_fairies.woodfall, 15);
        assert_eq!(save.stray_fairies.snowhead, 8);
    }

    #[test]
    fn test_parse_save_invalid_player_form() {
        let mut data = vec![0u8; 4096];
        data[OFFSET_PLAYER_FORM] = 99; // Invalid player form

        let result = MmSave::try_from(data);
        assert!(matches!(result, Err(MmDecodeError::InvalidPlayerForm(99))));
    }
}
