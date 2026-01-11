//! Dungeon progress tracking: dungeon items, small keys, and stray fairies.

use bitflags::bitflags;

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
