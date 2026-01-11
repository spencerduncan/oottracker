//! Majora's Mask save data structures and trait definitions.
//!
//! This module provides the trait interface, real memory parsing, and stub implementation
//! for MM save data.
//!
//! Reference: OoTMM source - packages/core/include/combo/mm/save.h
//! MM SaveContext address: 0x801ef670 (size 0x48d0 = 18,640 bytes)

mod accessors;
pub mod constants;
pub mod dungeon_progress;
pub mod inventory;
pub mod masks;
pub mod offsets;
pub mod quest_items;
mod reader;
pub mod save;
pub mod scene_flags;
mod serialization;
mod stub;
#[cfg(test)]
mod tests;
pub mod traits;
pub mod types;
pub mod upgrades;

// Re-export all public types for convenience
pub use constants::{mm_item_ids, MM_ADDR, MM_PERM_SCENE_COUNT, MM_PERM_SCENE_SIZE, MM_SIZE};
pub use dungeon_progress::{MmAllDungeonItems, MmDungeonItems, MmSmallKeys, MmStrayFairies};
pub use inventory::{MmBottle, MmInventory};
pub use masks::{MmMasks, MmMasksHigh, MmMasksLow, MmTransformationMasks};
pub use offsets::{ootmm_offsets, vanilla_offsets, MmRomType};
pub use quest_items::MmQuestItems;
pub use reader::MmSaveReader;
pub use save::MmSave;
pub use scene_flags::{MmCycleSceneFlags, MmPermanentSceneFlags};
pub use stub::MmSaveStub;
pub use traits::{MmGameMode, MmSaveData};
pub use types::{MmDecodeError, MmMagicCapacity, MmShield, MmSword, PlayerForm};
pub use upgrades::MmUpgrades;
