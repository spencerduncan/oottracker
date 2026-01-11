//! Trait definitions for MM save data.

use crate::mm_save::{masks::MmTransformationMasks, quest_items::MmQuestItems, save::MmSave};

/// MM game mode states
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum MmGameMode {
    #[default]
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
