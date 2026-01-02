//! Majora's Mask save data structures and trait definitions.
//!
//! This module provides the trait interface and stub implementation for MM save data.
//! The stub enables parallel UI development while actual memory reading is implemented.
//!
//! Reference: OoTMM source - packages/core/include/combo/mm/save.h
//! MM SaveContext address: 0x801ef670 (size 0x48d0 = 18,640 bytes)

use {
    bitflags::bitflags,
    derivative::Derivative,
};

/// MM SaveContext base address in N64 memory
pub const MM_ADDR: u32 = 0x801ef670;
/// MM SaveContext size in bytes
pub const MM_SIZE: usize = 0x48d0;

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
        if self.contains(Self::REMAINS_ODOLWA) { count += 1; }
        if self.contains(Self::REMAINS_GOHT) { count += 1; }
        if self.contains(Self::REMAINS_GYORG) { count += 1; }
        if self.contains(Self::REMAINS_TWINMOLD) { count += 1; }
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
        save.masks.masks_low = MmMasksLow::BUNNY
            | MmMasksLow::STONE
            | MmMasksLow::GREAT_FAIRY
            | MmMasksLow::BREMEN;

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
        save.health = 0x100;          // 4 hearts
        save.double_defense = true;

        // Sample dungeon progress
        save.dungeon_items.woodfall = MmDungeonItems::MAP | MmDungeonItems::COMPASS | MmDungeonItems::BOSS_KEY;
        save.dungeon_items.snowhead = MmDungeonItems::MAP | MmDungeonItems::COMPASS | MmDungeonItems::BOSS_KEY;
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
        assert_eq!(stub.get_save().masks.transformation, MmTransformationMasks::empty());
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
        assert!(save.masks.transformation.contains(MmTransformationMasks::DEKU));
        assert!(save.masks.transformation.contains(MmTransformationMasks::GORON));
        assert!(save.masks.transformation.contains(MmTransformationMasks::ZORA));
        assert!(!save.masks.transformation.contains(MmTransformationMasks::FIERCE_DEITY));

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
}
