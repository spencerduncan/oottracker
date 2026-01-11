//! Randomizer settings schema for OoTMM.
//!
//! This module defines all the settings that can be configured for an OoTMM randomizer seed.
//! Settings affect logic evaluation and determine what checks are accessible.
//!
//! # Setting Types
//!
//! Settings come in two forms:
//! - **Boolean settings**: Simple on/off flags, evaluated as `setting(name)`
//! - **Value settings**: Settings with specific values, evaluated as `setting(name, value)`
//!
//! # Example
//!
//! ```
//! use ootmm::settings::{RandomizerSettings, OotDungeon, MmDungeon};
//!
//! let mut settings = RandomizerSettings::default();
//! settings.open_dungeons_oot.insert(OotDungeon::DodongosCavern);
//! settings.open_dungeons_mm.insert(MmDungeon::StoneTower);
//! settings.ageless_boots = true;
//! ```

mod accessors;
mod core;
mod dungeons;
mod special;
mod state_modes;

#[cfg(test)]
mod tests;

// Re-export core types
pub use core::RandomizerSettings;

// Re-export dungeon types
pub use dungeons::{MmDungeon, MqDungeon, OotDungeon};

// Re-export special types
pub use special::{JunkLocations, SpecialCondition, StartingItems, WorldFlags};

// Re-export all state/mode enums
pub use state_modes::{
    AgeChangeMode, AutoInvertMode, BeneathWellState, BombchuBehavior, BossWarpPadsMode,
    ClearStateDungeonsMm, ClimbMostSurfacesState, CrossWarpMode, CsmcMode, DamageMultiplier,
    DekuTreeState, DoorOfTimeState, DungeonRewardShuffle, ErGrottosState, ErOverworldState,
    GanonBossKeyMode, HookshotAnywhereState, ItemPool, JpLayout, KakarikoGateState, KeyShuffle,
    LacsMode, LogicMode, MajoraChildMode, MapCompassShuffle, MoonCrashMode, OwlShuffle, PriceMode,
    RainbowBridgeMode, ShopShuffleMode, ShuffleMode, ShufflePotsMm, SkulltulaTokenShuffle,
    SmallKeyShuffleOot, SongsMode, StartingAge, StrayFairyShuffle, TingleShuffle, TownFairyShuffle,
    TrapsQuantity,
};
