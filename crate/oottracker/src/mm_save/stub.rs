//! Stub implementation for testing and UI development.

use crate::mm_save::{
    dungeon_progress::MmDungeonItems,
    inventory::MmBottle,
    masks::{MmMasksLow, MmTransformationMasks},
    quest_items::MmQuestItems,
    save::MmSave,
    traits::{MmGameMode, MmSaveData},
    types::{MmMagicCapacity, MmShield, MmSword},
    upgrades::MmUpgrades,
};

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
