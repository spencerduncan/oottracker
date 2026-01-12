//! Real memory reader implementation for MM save data.

use crate::mm_save::{
    save::MmSave,
    traits::{MmGameMode, MmSaveData},
    types::MmDecodeError,
};

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
