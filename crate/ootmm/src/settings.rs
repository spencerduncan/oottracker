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

use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};

/// OoT dungeon identifiers for `openDungeonsOot` setting.
///
/// These correspond to the values used in logic expressions like
/// `setting(openDungeonsOot, DC)`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum OotDungeon {
    /// Dodongo's Cavern (DC)
    #[serde(rename = "DC")]
    DodongosCavern,
    /// Bottom of the Well (BotW)
    #[serde(rename = "BotW")]
    BottomOfTheWell,
    /// Jabu-Jabu's Belly (JJ)
    #[serde(rename = "JJ")]
    JabuJabu,
    /// Shadow Temple
    Shadow,
    /// Water Temple
    Water,
    /// Fire Temple accessible as child
    #[serde(rename = "fireChild")]
    FireChild,
    /// Well accessible as adult
    #[serde(rename = "wellAdult")]
    WellAdult,
}

impl OotDungeon {
    /// Returns the string identifier used in logic expressions.
    #[must_use]
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::DodongosCavern => "DC",
            Self::BottomOfTheWell => "BotW",
            Self::JabuJabu => "JJ",
            Self::Shadow => "Shadow",
            Self::Water => "Water",
            Self::FireChild => "fireChild",
            Self::WellAdult => "wellAdult",
        }
    }

    /// Parses a logic string identifier into an OotDungeon.
    #[must_use]
    pub fn parse(s: &str) -> Option<Self> {
        match s {
            "DC" => Some(Self::DodongosCavern),
            "BotW" => Some(Self::BottomOfTheWell),
            "JJ" => Some(Self::JabuJabu),
            "Shadow" => Some(Self::Shadow),
            "Water" => Some(Self::Water),
            "fireChild" => Some(Self::FireChild),
            "wellAdult" => Some(Self::WellAdult),
            _ => None,
        }
    }
}

/// MM dungeon identifiers for `openDungeonsMm` setting.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum MmDungeon {
    /// Stone Tower Temple (ST)
    #[serde(rename = "ST")]
    StoneTower,
    /// Woodfall Temple (WF)
    #[serde(rename = "WF")]
    Woodfall,
}

/// OoT dungeons that can be set to Master Quest.
///
/// These correspond to all dungeons in OoT that have Master Quest variants
/// with different layouts, puzzles, and check locations.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MqDungeon {
    /// Deku Tree
    DekuTree,
    /// Dodongo's Cavern
    DodongosCavern,
    /// Jabu-Jabu's Belly
    JabuJabu,
    /// Forest Temple
    ForestTemple,
    /// Fire Temple
    FireTemple,
    /// Water Temple
    WaterTemple,
    /// Spirit Temple
    SpiritTemple,
    /// Shadow Temple
    ShadowTemple,
    /// Bottom of the Well
    BottomOfTheWell,
    /// Ice Cavern
    IceCavern,
    /// Gerudo Training Ground
    GerudoTrainingGround,
    /// Ganon's Castle
    GanonsCastle,
}

impl MmDungeon {
    /// Returns the string identifier used in logic expressions.
    #[must_use]
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::StoneTower => "ST",
            Self::Woodfall => "WF",
        }
    }

    /// Parses a logic string identifier into an MmDungeon.
    #[must_use]
    pub fn parse(s: &str) -> Option<Self> {
        match s {
            "ST" => Some(Self::StoneTower),
            "WF" => Some(Self::Woodfall),
            _ => None,
        }
    }
}

impl MqDungeon {
    /// Returns the string identifier used in logic expressions and settings.
    #[must_use]
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::DekuTree => "deku_tree",
            Self::DodongosCavern => "dodongos_cavern",
            Self::JabuJabu => "jabu_jabu",
            Self::ForestTemple => "forest_temple",
            Self::FireTemple => "fire_temple",
            Self::WaterTemple => "water_temple",
            Self::SpiritTemple => "spirit_temple",
            Self::ShadowTemple => "shadow_temple",
            Self::BottomOfTheWell => "bottom_of_the_well",
            Self::IceCavern => "ice_cavern",
            Self::GerudoTrainingGround => "gerudo_training_ground",
            Self::GanonsCastle => "ganons_castle",
        }
    }

    /// Returns the location ID prefix for this dungeon in its vanilla variant.
    ///
    /// Vanilla locations use the `oot_<dungeon>_` prefix.
    #[must_use]
    pub const fn vanilla_location_prefix(&self) -> &'static str {
        match self {
            Self::DekuTree => "oot_deku_tree_",
            Self::DodongosCavern => "oot_dodongo_cavern_",
            Self::JabuJabu => "oot_jabu_jabu_",
            Self::ForestTemple => "oot_forest_temple_",
            Self::FireTemple => "oot_fire_temple_",
            Self::WaterTemple => "oot_water_temple_",
            Self::SpiritTemple => "oot_spirit_temple_",
            Self::ShadowTemple => "oot_shadow_temple_",
            Self::BottomOfTheWell => "oot_bottom_of_the_well_",
            Self::IceCavern => "oot_ice_cavern_",
            Self::GerudoTrainingGround => "oot_gerudo_training_",
            Self::GanonsCastle => "oot_ganon_castle_",
        }
    }

    /// Returns the location ID prefix for this dungeon in its MQ variant.
    ///
    /// MQ locations use the `mq_oot_mq_<dungeon>_` prefix for checks,
    /// or `mq_oot_<dungeon>_` for regions.
    #[must_use]
    pub const fn mq_location_prefix(&self) -> &'static str {
        match self {
            Self::DekuTree => "mq_oot_mq_deku_tree_",
            Self::DodongosCavern => "mq_oot_mq_dodongo_cavern_",
            Self::JabuJabu => "mq_oot_mq_jabu_jabu_",
            Self::ForestTemple => "mq_oot_mq_forest_temple_",
            Self::FireTemple => "mq_oot_mq_fire_temple_",
            Self::WaterTemple => "mq_oot_mq_water_temple_",
            Self::SpiritTemple => "mq_oot_mq_spirit_temple_",
            Self::ShadowTemple => "mq_oot_mq_shadow_temple_",
            Self::BottomOfTheWell => "mq_oot_mq_bottom_of_the_well_",
            Self::IceCavern => "mq_oot_mq_ice_cavern_",
            Self::GerudoTrainingGround => "mq_oot_mq_gerudo_training_",
            Self::GanonsCastle => "mq_oot_mq_ganon_castle_",
        }
    }

    /// Parses a string identifier into an MqDungeon.
    #[must_use]
    pub fn parse(s: &str) -> Option<Self> {
        match s {
            "deku_tree" | "DekuTree" => Some(Self::DekuTree),
            "dodongos_cavern" | "DodongosCavern" => Some(Self::DodongosCavern),
            "jabu_jabu" | "JabuJabu" => Some(Self::JabuJabu),
            "forest_temple" | "ForestTemple" => Some(Self::ForestTemple),
            "fire_temple" | "FireTemple" => Some(Self::FireTemple),
            "water_temple" | "WaterTemple" => Some(Self::WaterTemple),
            "spirit_temple" | "SpiritTemple" => Some(Self::SpiritTemple),
            "shadow_temple" | "ShadowTemple" => Some(Self::ShadowTemple),
            "bottom_of_the_well" | "BottomOfTheWell" => Some(Self::BottomOfTheWell),
            "ice_cavern" | "IceCavern" => Some(Self::IceCavern),
            "gerudo_training_ground" | "GerudoTrainingGround" => Some(Self::GerudoTrainingGround),
            "ganons_castle" | "GanonsCastle" => Some(Self::GanonsCastle),
            _ => None,
        }
    }

    /// Returns all MQ dungeon variants.
    #[must_use]
    pub const fn all() -> &'static [MqDungeon] {
        &[
            Self::DekuTree,
            Self::DodongosCavern,
            Self::JabuJabu,
            Self::ForestTemple,
            Self::FireTemple,
            Self::WaterTemple,
            Self::SpiritTemple,
            Self::ShadowTemple,
            Self::BottomOfTheWell,
            Self::IceCavern,
            Self::GerudoTrainingGround,
            Self::GanonsCastle,
        ]
    }

    /// Attempts to determine which dungeon a location ID belongs to.
    ///
    /// Returns `None` if the location is not in a dungeon that has MQ variants.
    #[must_use]
    pub fn from_location_id(location_id: &str) -> Option<Self> {
        // Check MQ locations first (they have the mq_ prefix)
        if location_id.starts_with("mq_oot_") {
            // MQ dungeon locations
            if location_id.contains("deku_tree") {
                return Some(Self::DekuTree);
            }
            if location_id.contains("dodongo") {
                return Some(Self::DodongosCavern);
            }
            if location_id.contains("jabu") {
                return Some(Self::JabuJabu);
            }
            if location_id.contains("forest_temple") {
                return Some(Self::ForestTemple);
            }
            if location_id.contains("fire_temple") {
                return Some(Self::FireTemple);
            }
            if location_id.contains("water_temple") {
                return Some(Self::WaterTemple);
            }
            if location_id.contains("spirit_temple") {
                return Some(Self::SpiritTemple);
            }
            if location_id.contains("shadow_temple") {
                return Some(Self::ShadowTemple);
            }
            if location_id.contains("bottom_of_the_well") {
                return Some(Self::BottomOfTheWell);
            }
            if location_id.contains("ice_cavern") {
                return Some(Self::IceCavern);
            }
            if location_id.contains("gerudo_training") {
                return Some(Self::GerudoTrainingGround);
            }
            if location_id.contains("ganon_castle") {
                return Some(Self::GanonsCastle);
            }
            return None;
        }

        // Check vanilla OoT dungeon locations
        if location_id.starts_with("oot_deku_tree_") {
            return Some(Self::DekuTree);
        }
        if location_id.starts_with("oot_dodongo_cavern_") || location_id.starts_with("oot_dodongo_")
        {
            return Some(Self::DodongosCavern);
        }
        if location_id.starts_with("oot_jabu_jabu_") {
            return Some(Self::JabuJabu);
        }
        if location_id.starts_with("oot_forest_temple_") {
            return Some(Self::ForestTemple);
        }
        if location_id.starts_with("oot_fire_temple_") {
            return Some(Self::FireTemple);
        }
        if location_id.starts_with("oot_water_temple_") {
            return Some(Self::WaterTemple);
        }
        if location_id.starts_with("oot_spirit_temple_") {
            return Some(Self::SpiritTemple);
        }
        if location_id.starts_with("oot_shadow_temple_") {
            return Some(Self::ShadowTemple);
        }
        if location_id.starts_with("oot_bottom_of_the_well_") {
            return Some(Self::BottomOfTheWell);
        }
        if location_id.starts_with("oot_ice_cavern_") {
            return Some(Self::IceCavern);
        }
        if location_id.starts_with("oot_gerudo_training_") {
            return Some(Self::GerudoTrainingGround);
        }
        if location_id.starts_with("oot_ganon_castle_") {
            return Some(Self::GanonsCastle);
        }

        None
    }
}

/// Deku Tree entrance state.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub enum DekuTreeState {
    /// Deku Tree is closed (requires Mido moved event)
    #[default]
    Closed,
    /// Deku Tree is open without requirements
    Open,
    /// Vanilla behavior (child can enter after meeting requirements)
    Vanilla,
}

impl DekuTreeState {
    /// Returns the string identifier used in logic expressions.
    #[must_use]
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::Closed => "closed",
            Self::Open => "open",
            Self::Vanilla => "vanilla",
        }
    }

    /// Parses a logic string identifier into a DekuTreeState.
    #[must_use]
    pub fn parse(s: &str) -> Option<Self> {
        match s {
            "closed" => Some(Self::Closed),
            "open" => Some(Self::Open),
            "vanilla" => Some(Self::Vanilla),
            _ => None,
        }
    }
}

/// Door of Time state.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub enum DoorOfTimeState {
    /// Door of Time is closed (requires Song of Time)
    #[default]
    Closed,
    /// Door of Time is open
    Open,
}

impl DoorOfTimeState {
    /// Returns the string identifier used in logic expressions.
    #[must_use]
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::Closed => "closed",
            Self::Open => "open",
        }
    }

    /// Parses a logic string identifier into a DoorOfTimeState.
    #[must_use]
    pub fn parse(s: &str) -> Option<Self> {
        match s {
            "closed" => Some(Self::Closed),
            "open" => Some(Self::Open),
            _ => None,
        }
    }
}

/// Kakariko Village gate state.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub enum KakarikoGateState {
    /// Gate is closed (vanilla)
    #[default]
    Closed,
    /// Gate is open
    Open,
}

impl KakarikoGateState {
    /// Returns the string identifier used in logic expressions.
    #[must_use]
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::Closed => "closed",
            Self::Open => "open",
        }
    }

    /// Parses a logic string identifier into a KakarikoGateState.
    #[must_use]
    pub fn parse(s: &str) -> Option<Self> {
        match s {
            "closed" => Some(Self::Closed),
            "open" => Some(Self::Open),
            _ => None,
        }
    }
}

/// Ganon's Castle Boss Key mode.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub enum GanonBossKeyMode {
    /// Vanilla behavior
    #[default]
    Vanilla,
    /// Boss Key is removed (not required)
    Removed,
    /// Custom location (set by special condition)
    Custom,
}

impl GanonBossKeyMode {
    /// Returns the string identifier used in logic expressions.
    #[must_use]
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::Vanilla => "vanilla",
            Self::Removed => "removed",
            Self::Custom => "custom",
        }
    }

    /// Parses a logic string identifier into a GanonBossKeyMode.
    #[must_use]
    pub fn parse(s: &str) -> Option<Self> {
        match s {
            "vanilla" => Some(Self::Vanilla),
            "removed" => Some(Self::Removed),
            "custom" => Some(Self::Custom),
            _ => None,
        }
    }
}

/// Light Arrow Cutscene (LACS) mode.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub enum LacsMode {
    /// Vanilla behavior
    #[default]
    Vanilla,
    /// Custom requirements
    Custom,
}

impl LacsMode {
    /// Returns the string identifier used in logic expressions.
    #[must_use]
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::Vanilla => "vanilla",
            Self::Custom => "custom",
        }
    }

    /// Parses a logic string identifier into a LacsMode.
    #[must_use]
    pub fn parse(s: &str) -> Option<Self> {
        match s {
            "vanilla" => Some(Self::Vanilla),
            "custom" => Some(Self::Custom),
            _ => None,
        }
    }
}

/// Majora's Mask child mode.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub enum MajoraChildMode {
    /// Vanilla behavior
    #[default]
    Vanilla,
    /// No child requirements
    None,
    /// Custom requirements
    Custom,
}

impl MajoraChildMode {
    /// Returns the string identifier used in logic expressions.
    #[must_use]
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::Vanilla => "vanilla",
            Self::None => "none",
            Self::Custom => "custom",
        }
    }

    /// Parses a logic string identifier into a MajoraChildMode.
    #[must_use]
    pub fn parse(s: &str) -> Option<Self> {
        match s {
            "vanilla" => Some(Self::Vanilla),
            "none" => Some(Self::None),
            "custom" => Some(Self::Custom),
            _ => None,
        }
    }
}

/// Moon crash behavior mode.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub enum MoonCrashMode {
    /// Vanilla behavior (moon crash causes game over)
    #[default]
    Vanilla,
    /// Cycle resets on moon crash
    Cycle,
}

impl MoonCrashMode {
    /// Returns the string identifier used in logic expressions.
    #[must_use]
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::Vanilla => "vanilla",
            Self::Cycle => "cycle",
        }
    }

    /// Parses a logic string identifier into a MoonCrashMode.
    #[must_use]
    pub fn parse(s: &str) -> Option<Self> {
        match s {
            "vanilla" => Some(Self::Vanilla),
            "cycle" => Some(Self::Cycle),
            _ => None,
        }
    }
}

/// Age change mode.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub enum AgeChangeMode {
    /// Age change requires Temple of Time
    #[default]
    TempleOfTime,
    /// Age change is disabled
    None,
    /// OoT-style age change
    Oot,
}

impl AgeChangeMode {
    /// Returns the string identifier used in logic expressions.
    #[must_use]
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::TempleOfTime => "templeOfTime",
            Self::None => "none",
            Self::Oot => "oot",
        }
    }

    /// Parses a logic string identifier into an AgeChangeMode.
    #[must_use]
    pub fn parse(s: &str) -> Option<Self> {
        match s {
            "templeOfTime" => Some(Self::TempleOfTime),
            "none" => Some(Self::None),
            "oot" => Some(Self::Oot),
            _ => None,
        }
    }
}

/// Climb Most Surfaces state (OoT specific glitch).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub enum ClimbMostSurfacesState {
    /// Climb glitch is enabled
    #[default]
    On,
    /// Climb glitch is disabled
    Off,
}

impl ClimbMostSurfacesState {
    /// Returns the string identifier used in logic expressions.
    #[must_use]
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::On => "on",
            Self::Off => "off",
        }
    }

    /// Parses a logic string identifier into a ClimbMostSurfacesState.
    #[must_use]
    pub fn parse(s: &str) -> Option<Self> {
        match s {
            "on" => Some(Self::On),
            "off" => Some(Self::Off),
            _ => None,
        }
    }
}

/// Hookshot Anywhere state (OoT specific glitch).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub enum HookshotAnywhereState {
    /// Hookshot anywhere is enabled
    #[default]
    On,
    /// Hookshot anywhere is disabled
    Off,
}

impl HookshotAnywhereState {
    /// Returns the string identifier used in logic expressions.
    #[must_use]
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::On => "on",
            Self::Off => "off",
        }
    }

    /// Parses a logic string identifier into a HookshotAnywhereState.
    #[must_use]
    pub fn parse(s: &str) -> Option<Self> {
        match s {
            "on" => Some(Self::On),
            "off" => Some(Self::Off),
            _ => None,
        }
    }
}

/// Beneath the Well state.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub enum BeneathWellState {
    /// Vanilla behavior (requires items to pass Gibdo)
    #[default]
    Vanilla,
    /// Well is open without requirements
    Open,
}

impl BeneathWellState {
    /// Returns the string identifier used in logic expressions.
    #[must_use]
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::Vanilla => "vanilla",
            Self::Open => "open",
        }
    }

    /// Parses a logic string identifier into a BeneathWellState.
    #[must_use]
    pub fn parse(s: &str) -> Option<Self> {
        match s {
            "vanilla" => Some(Self::Vanilla),
            "open" => Some(Self::Open),
            _ => None,
        }
    }
}

/// Entrance Randomizer overworld state.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub enum ErOverworldState {
    /// No overworld entrance randomization
    #[default]
    None,
    /// Full overworld entrance randomization
    Full,
}

impl ErOverworldState {
    /// Returns the string identifier used in logic expressions.
    #[must_use]
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::None => "none",
            Self::Full => "full",
        }
    }

    /// Parses a logic string identifier into an ErOverworldState.
    #[must_use]
    pub fn parse(s: &str) -> Option<Self> {
        match s {
            "none" => Some(Self::None),
            "full" => Some(Self::Full),
            _ => None,
        }
    }
}

/// Entrance Randomizer grottos state.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub enum ErGrottosState {
    /// No grotto entrance randomization
    #[default]
    None,
    /// Full grotto entrance randomization
    Full,
}

impl ErGrottosState {
    /// Returns the string identifier used in logic expressions.
    #[must_use]
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::None => "none",
            Self::Full => "full",
        }
    }

    /// Parses a logic string identifier into an ErGrottosState.
    #[must_use]
    pub fn parse(s: &str) -> Option<Self> {
        match s {
            "none" => Some(Self::None),
            "full" => Some(Self::Full),
            _ => None,
        }
    }
}

/// Boss Warp Pads mode.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub enum BossWarpPadsMode {
    /// Vanilla behavior
    #[default]
    Vanilla,
    /// Boss warp pads require boss remains
    Remains,
}

impl BossWarpPadsMode {
    /// Returns the string identifier used in logic expressions.
    #[must_use]
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::Vanilla => "vanilla",
            Self::Remains => "remains",
        }
    }

    /// Parses a logic string identifier into a BossWarpPadsMode.
    #[must_use]
    pub fn parse(s: &str) -> Option<Self> {
        match s {
            "vanilla" => Some(Self::Vanilla),
            "remains" => Some(Self::Remains),
            _ => None,
        }
    }
}

/// Clear state for MM dungeons.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum ClearStateDungeonsMm {
    /// Woodfall is cleared
    #[serde(rename = "WF")]
    Woodfall,
    /// Both dungeons are cleared
    Both,
}

impl ClearStateDungeonsMm {
    /// Returns the string identifier used in logic expressions.
    #[must_use]
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::Woodfall => "WF",
            Self::Both => "both",
        }
    }

    /// Parses a logic string identifier into a ClearStateDungeonsMm.
    #[must_use]
    pub fn parse(s: &str) -> Option<Self> {
        match s {
            "WF" => Some(Self::Woodfall),
            "both" => Some(Self::Both),
            _ => None,
        }
    }
}

/// Japan-specific layout locations.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum JpLayout {
    /// Great Bay Coast layout
    GreatBayCoast,
    /// Stone Tower entrance (ST)
    #[serde(rename = "ST")]
    StoneTowerEntrance,
    /// Stone Tower full area
    StoneTower,
}

impl JpLayout {
    /// Returns the string identifier used in logic expressions.
    #[must_use]
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::GreatBayCoast => "GreatBayCoast",
            Self::StoneTowerEntrance => "ST",
            Self::StoneTower => "StoneTower",
        }
    }

    /// Parses a logic string identifier into a JpLayout.
    #[must_use]
    pub fn parse(s: &str) -> Option<Self> {
        match s {
            "GreatBayCoast" => Some(Self::GreatBayCoast),
            "ST" => Some(Self::StoneTowerEntrance),
            "StoneTower" => Some(Self::StoneTower),
            _ => None,
        }
    }
}

/// Small key shuffle mode for OoT.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub enum SmallKeyShuffleOot {
    /// Vanilla (keys in their original locations)
    #[default]
    Vanilla,
    /// Keys within their dungeon
    Dungeon,
    /// Keys anywhere in the world
    Anywhere,
}

impl SmallKeyShuffleOot {
    /// Returns the string identifier used in logic expressions.
    #[must_use]
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::Vanilla => "vanilla",
            Self::Dungeon => "dungeon",
            Self::Anywhere => "anywhere",
        }
    }

    /// Parses a logic string identifier into a SmallKeyShuffleOot.
    #[must_use]
    pub fn parse(s: &str) -> Option<Self> {
        match s {
            "vanilla" => Some(Self::Vanilla),
            "dungeon" => Some(Self::Dungeon),
            "anywhere" => Some(Self::Anywhere),
            _ => None,
        }
    }
}

/// Shuffle pots mode for MM.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub enum ShufflePotsMm {
    /// No pot shuffle
    #[default]
    None,
    /// All pots shuffled
    All,
}

impl ShufflePotsMm {
    /// Returns the string identifier used in logic expressions.
    #[must_use]
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::None => "none",
            Self::All => "all",
        }
    }

    /// Parses a logic string identifier into a ShufflePotsMm.
    #[must_use]
    pub fn parse(s: &str) -> Option<Self> {
        match s {
            "none" => Some(Self::None),
            "all" => Some(Self::All),
            _ => None,
        }
    }
}

/// Logic rules mode.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub enum LogicMode {
    /// Standard glitchless logic
    #[default]
    Glitchless,
    /// Logic that allows glitches
    Glitched,
    /// No logic (all locations accessible)
    NoLogic,
}

impl LogicMode {
    /// Returns the string identifier used in logic expressions.
    #[must_use]
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::Glitchless => "glitchless",
            Self::Glitched => "glitched",
            Self::NoLogic => "noLogic",
        }
    }

    /// Parses a logic string identifier into a LogicMode.
    #[must_use]
    pub fn parse(s: &str) -> Option<Self> {
        match s {
            "glitchless" => Some(Self::Glitchless),
            "glitched" => Some(Self::Glitched),
            "noLogic" | "no_logic" => Some(Self::NoLogic),
            _ => None,
        }
    }
}

/// Rainbow Bridge access requirements mode.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub enum RainbowBridgeMode {
    /// Vanilla behavior (requires all medallions and stones)
    #[default]
    Vanilla,
    /// Bridge is always open
    Open,
    /// Requires medallions only
    Medallions,
    /// Requires spiritual stones only
    Stones,
    /// Requires dungeon rewards (medallions + stones)
    DungeonRewards,
    /// Requires Gold Skulltula tokens
    Skulltulas,
    /// Requires boss remains (MM)
    Remains,
    /// Custom requirements
    Custom,
}

impl RainbowBridgeMode {
    /// Returns the string identifier used in logic expressions.
    #[must_use]
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::Vanilla => "vanilla",
            Self::Open => "open",
            Self::Medallions => "medallions",
            Self::Stones => "stones",
            Self::DungeonRewards => "dungeonRewards",
            Self::Skulltulas => "skulltulas",
            Self::Remains => "remains",
            Self::Custom => "custom",
        }
    }

    /// Parses a logic string identifier into a RainbowBridgeMode.
    #[must_use]
    pub fn parse(s: &str) -> Option<Self> {
        match s {
            "vanilla" => Some(Self::Vanilla),
            "open" => Some(Self::Open),
            "medallions" => Some(Self::Medallions),
            "stones" => Some(Self::Stones),
            "dungeonRewards" => Some(Self::DungeonRewards),
            "skulltulas" => Some(Self::Skulltulas),
            "remains" => Some(Self::Remains),
            "custom" => Some(Self::Custom),
            _ => None,
        }
    }
}

/// Song shuffle mode.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub enum SongsMode {
    /// Songs only shuffle with other songs
    #[default]
    SongsOnly,
    /// Songs can be anywhere
    Anywhere,
    /// Songs on dungeon rewards
    DungeonRewards,
}

impl SongsMode {
    /// Returns the string identifier used in logic expressions.
    #[must_use]
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::SongsOnly => "songsOnly",
            Self::Anywhere => "anywhere",
            Self::DungeonRewards => "dungeonRewards",
        }
    }

    /// Parses a logic string identifier into a SongsMode.
    #[must_use]
    pub fn parse(s: &str) -> Option<Self> {
        match s {
            "songsOnly" => Some(Self::SongsOnly),
            "anywhere" => Some(Self::Anywhere),
            "dungeonRewards" => Some(Self::DungeonRewards),
            _ => None,
        }
    }
}

/// Dungeon reward shuffle mode.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub enum DungeonRewardShuffle {
    /// Vanilla (rewards in their original dungeons)
    #[default]
    Vanilla,
    /// Rewards on dungeon blue warps
    DungeonBlueWarps,
    /// Rewards can be anywhere
    Anywhere,
}

impl DungeonRewardShuffle {
    /// Returns the string identifier used in logic expressions.
    #[must_use]
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::Vanilla => "vanilla",
            Self::DungeonBlueWarps => "dungeonBlueWarps",
            Self::Anywhere => "anywhere",
        }
    }

    /// Parses a logic string identifier into a DungeonRewardShuffle.
    #[must_use]
    pub fn parse(s: &str) -> Option<Self> {
        match s {
            "vanilla" => Some(Self::Vanilla),
            "dungeonBlueWarps" => Some(Self::DungeonBlueWarps),
            "anywhere" => Some(Self::Anywhere),
            _ => None,
        }
    }
}

/// Shuffle mode for entrances or locations.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub enum ShuffleMode {
    /// No shuffling
    #[default]
    None,
    /// Overworld only
    Overworld,
    /// Dungeons only
    Dungeon,
    /// All locations shuffled
    All,
}

impl ShuffleMode {
    /// Returns the string identifier used in logic expressions.
    #[must_use]
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::None => "none",
            Self::Overworld => "overworld",
            Self::Dungeon => "dungeon",
            Self::All => "all",
        }
    }

    /// Parses a logic string identifier into a ShuffleMode.
    #[must_use]
    pub fn parse(s: &str) -> Option<Self> {
        match s {
            "none" => Some(Self::None),
            "overworld" => Some(Self::Overworld),
            "dungeon" => Some(Self::Dungeon),
            "all" => Some(Self::All),
            _ => None,
        }
    }

    /// Returns true if any shuffling is enabled.
    #[must_use]
    pub fn is_shuffled(&self) -> bool {
        !matches!(self, Self::None)
    }
}

/// Tingle map shuffle mode.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub enum TingleShuffle {
    /// Vanilla locations
    #[default]
    Vanilla,
    /// Start with Tingle maps
    Starting,
    /// Tingle maps removed
    Removed,
    /// Tingle maps can be anywhere
    Anywhere,
    /// Tingle maps in their own region
    OwnRegion,
}

impl TingleShuffle {
    /// Returns the string identifier used in logic expressions.
    #[must_use]
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::Vanilla => "vanilla",
            Self::Starting => "starting",
            Self::Removed => "removed",
            Self::Anywhere => "anywhere",
            Self::OwnRegion => "ownRegion",
        }
    }

    /// Parses a logic string identifier into a TingleShuffle.
    #[must_use]
    pub fn parse(s: &str) -> Option<Self> {
        match s {
            "vanilla" => Some(Self::Vanilla),
            "starting" => Some(Self::Starting),
            "removed" => Some(Self::Removed),
            "anywhere" => Some(Self::Anywhere),
            "ownRegion" => Some(Self::OwnRegion),
            _ => None,
        }
    }
}

/// Owl statue shuffle mode.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub enum OwlShuffle {
    /// No owl shuffling
    #[default]
    None,
    /// Owl statues can be anywhere
    Anywhere,
}

impl OwlShuffle {
    /// Returns the string identifier used in logic expressions.
    #[must_use]
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::None => "none",
            Self::Anywhere => "anywhere",
        }
    }

    /// Parses a logic string identifier into an OwlShuffle.
    #[must_use]
    pub fn parse(s: &str) -> Option<Self> {
        match s {
            "none" => Some(Self::None),
            "anywhere" => Some(Self::Anywhere),
            _ => None,
        }
    }
}

/// Skulltula token shuffle mode.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub enum SkulltulaTokenShuffle {
    /// No token shuffling
    #[default]
    None,
    /// Dungeon tokens only
    Dungeons,
    /// Overworld tokens only
    Overworld,
    /// All tokens shuffled
    All,
}

impl SkulltulaTokenShuffle {
    /// Returns the string identifier used in logic expressions.
    #[must_use]
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::None => "none",
            Self::Dungeons => "dungeons",
            Self::Overworld => "overworld",
            Self::All => "all",
        }
    }

    /// Parses a logic string identifier into a SkulltulaTokenShuffle.
    #[must_use]
    pub fn parse(s: &str) -> Option<Self> {
        match s {
            "none" => Some(Self::None),
            "dungeons" => Some(Self::Dungeons),
            "overworld" => Some(Self::Overworld),
            "all" => Some(Self::All),
            _ => None,
        }
    }
}

/// Key shuffle mode.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub enum KeyShuffle {
    /// Vanilla key locations
    #[default]
    Vanilla,
    /// Keys within their own dungeon
    OwnDungeon,
    /// Keys can be anywhere
    Anywhere,
    /// Keys removed (not required)
    Removed,
}

impl KeyShuffle {
    /// Returns the string identifier used in logic expressions.
    #[must_use]
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::Vanilla => "vanilla",
            Self::OwnDungeon => "ownDungeon",
            Self::Anywhere => "anywhere",
            Self::Removed => "removed",
        }
    }

    /// Parses a logic string identifier into a KeyShuffle.
    #[must_use]
    pub fn parse(s: &str) -> Option<Self> {
        match s {
            "vanilla" => Some(Self::Vanilla),
            "ownDungeon" => Some(Self::OwnDungeon),
            "anywhere" => Some(Self::Anywhere),
            "removed" => Some(Self::Removed),
            _ => None,
        }
    }

    /// Returns true if keys are shuffled from vanilla locations.
    ///
    /// OwnDungeon and Anywhere are considered shuffled since keys are moved
    /// from their vanilla locations. Vanilla and Removed are not shuffled.
    #[must_use]
    pub const fn is_shuffled(&self) -> bool {
        !matches!(self, Self::Vanilla | Self::Removed)
    }
}

/// Map and compass shuffle mode.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub enum MapCompassShuffle {
    /// Vanilla locations
    #[default]
    Vanilla,
    /// Start with maps/compasses
    Starting,
    /// Maps/compasses within their own dungeon
    OwnDungeon,
    /// Maps/compasses can be anywhere
    Anywhere,
    /// Maps/compasses removed
    Removed,
}

impl MapCompassShuffle {
    /// Returns the string identifier used in logic expressions.
    #[must_use]
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::Vanilla => "vanilla",
            Self::Starting => "starting",
            Self::OwnDungeon => "ownDungeon",
            Self::Anywhere => "anywhere",
            Self::Removed => "removed",
        }
    }

    /// Parses a logic string identifier into a MapCompassShuffle.
    #[must_use]
    pub fn parse(s: &str) -> Option<Self> {
        match s {
            "vanilla" => Some(Self::Vanilla),
            "starting" => Some(Self::Starting),
            "ownDungeon" => Some(Self::OwnDungeon),
            "anywhere" => Some(Self::Anywhere),
            "removed" => Some(Self::Removed),
            _ => None,
        }
    }

    /// Returns true if maps/compasses are shuffled from vanilla locations.
    ///
    /// OwnDungeon and Anywhere are considered shuffled since items are moved
    /// from their vanilla locations. Vanilla, Starting, and Removed are not shuffled.
    #[must_use]
    pub const fn is_shuffled(&self) -> bool {
        !matches!(self, Self::Vanilla | Self::Starting | Self::Removed)
    }
}

/// Shop shuffle mode.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub enum ShopShuffleMode {
    /// No shop shuffling
    #[default]
    None,
    /// Shuffle within same shop
    OwnShop,
    /// Shuffle across all shops
    All,
}

impl ShopShuffleMode {
    /// Returns the string identifier used in logic expressions.
    #[must_use]
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::None => "none",
            Self::OwnShop => "ownShop",
            Self::All => "all",
        }
    }

    /// Parses a logic string identifier into a ShopShuffleMode.
    #[must_use]
    pub fn parse(s: &str) -> Option<Self> {
        match s {
            "none" => Some(Self::None),
            "ownShop" => Some(Self::OwnShop),
            "all" => Some(Self::All),
            _ => None,
        }
    }
}

/// Price mode for shops and scrubs.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub enum PriceMode {
    /// Vanilla prices
    #[default]
    Vanilla,
    /// Weighted random prices (favor affordable)
    Weighted,
    /// Fully random prices
    Random,
    /// Set all prices to a fixed value
    Fixed,
}

impl PriceMode {
    /// Returns the string identifier used in logic expressions.
    #[must_use]
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::Vanilla => "vanilla",
            Self::Weighted => "weighted",
            Self::Random => "random",
            Self::Fixed => "fixed",
        }
    }

    /// Parses a logic string identifier into a PriceMode.
    #[must_use]
    pub fn parse(s: &str) -> Option<Self> {
        match s {
            "vanilla" => Some(Self::Vanilla),
            "weighted" => Some(Self::Weighted),
            "random" => Some(Self::Random),
            "fixed" => Some(Self::Fixed),
            _ => None,
        }
    }
}

/// Town fairy shuffle mode for Clock Town stray fairies.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub enum TownFairyShuffle {
    /// Vanilla locations
    #[default]
    Vanilla,
    /// Fairies shuffled within Clock Town
    Anywhere,
}

impl TownFairyShuffle {
    /// Returns the string identifier used in logic expressions.
    #[must_use]
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::Vanilla => "vanilla",
            Self::Anywhere => "anywhere",
        }
    }

    /// Parses a logic string identifier into a TownFairyShuffle.
    #[must_use]
    pub fn parse(s: &str) -> Option<Self> {
        match s {
            "vanilla" => Some(Self::Vanilla),
            "anywhere" => Some(Self::Anywhere),
            _ => None,
        }
    }
}

/// Stray fairy shuffle mode for dungeon stray fairies.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub enum StrayFairyShuffle {
    /// Vanilla locations (no shuffle)
    #[default]
    Vanilla,
    /// Start with fairies
    Starting,
    /// Fairies removed
    Removed,
    /// Fairies shuffled within their dungeon
    OwnDungeon,
    /// Fairies can be anywhere
    Anywhere,
}

impl StrayFairyShuffle {
    /// Returns the string identifier used in logic expressions.
    #[must_use]
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::Vanilla => "vanilla",
            Self::Starting => "starting",
            Self::Removed => "removed",
            Self::OwnDungeon => "ownDungeon",
            Self::Anywhere => "anywhere",
        }
    }

    /// Parses a logic string identifier into a StrayFairyShuffle.
    #[must_use]
    pub fn parse(s: &str) -> Option<Self> {
        match s {
            "vanilla" => Some(Self::Vanilla),
            "starting" => Some(Self::Starting),
            "removed" => Some(Self::Removed),
            "ownDungeon" => Some(Self::OwnDungeon),
            "anywhere" => Some(Self::Anywhere),
            _ => None,
        }
    }
}

/// Cross-warp mode (warping between games).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub enum CrossWarpMode {
    /// No cross-warping
    #[default]
    None,
    /// Child cross-warp only
    ChildOnly,
    /// Adult cross-warp only
    AdultOnly,
    /// Full cross-warp enabled
    Full,
}

impl CrossWarpMode {
    /// Returns the string identifier used in logic expressions.
    #[must_use]
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::None => "none",
            Self::ChildOnly => "childOnly",
            Self::AdultOnly => "adultOnly",
            Self::Full => "full",
        }
    }

    /// Parses a logic string identifier into a CrossWarpMode.
    #[must_use]
    pub fn parse(s: &str) -> Option<Self> {
        match s {
            "none" => Some(Self::None),
            "childOnly" => Some(Self::ChildOnly),
            "adultOnly" => Some(Self::AdultOnly),
            "full" => Some(Self::Full),
            _ => None,
        }
    }
}

/// Chest Size Matches Contents mode.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub enum CsmcMode {
    /// CSMC disabled
    #[default]
    Never,
    /// CSMC always enabled
    Always,
    /// CSMC for agony hints
    Agony,
}

impl CsmcMode {
    /// Returns the string identifier used in logic expressions.
    #[must_use]
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::Never => "never",
            Self::Always => "always",
            Self::Agony => "agony",
        }
    }

    /// Parses a logic string identifier into a CsmcMode.
    #[must_use]
    pub fn parse(s: &str) -> Option<Self> {
        match s {
            "never" => Some(Self::Never),
            "always" => Some(Self::Always),
            "agony" => Some(Self::Agony),
            _ => None,
        }
    }
}

/// Bombchu behavior mode.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub enum BombchuBehavior {
    /// Bombchus are just items
    #[default]
    Normal,
    /// Bombchus are considered logic bombs
    BombsOrLogic,
    /// Bombchus can always be used as bombs
    AsBombs,
}

impl BombchuBehavior {
    /// Returns the string identifier used in logic expressions.
    #[must_use]
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::Normal => "normal",
            Self::BombsOrLogic => "bombsOrLogic",
            Self::AsBombs => "asBombs",
        }
    }

    /// Parses a logic string identifier into a BombchuBehavior.
    #[must_use]
    pub fn parse(s: &str) -> Option<Self> {
        match s {
            "normal" => Some(Self::Normal),
            "bombsOrLogic" => Some(Self::BombsOrLogic),
            "asBombs" => Some(Self::AsBombs),
            _ => None,
        }
    }
}

/// Auto-invert camera mode.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub enum AutoInvertMode {
    /// No auto-invert
    #[default]
    Off,
    /// First-person auto-invert
    FirstPerson,
    /// Always auto-invert
    Always,
}

impl AutoInvertMode {
    /// Returns the string identifier used in logic expressions.
    #[must_use]
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::Off => "off",
            Self::FirstPerson => "firstPerson",
            Self::Always => "always",
        }
    }

    /// Parses a logic string identifier into an AutoInvertMode.
    #[must_use]
    pub fn parse(s: &str) -> Option<Self> {
        match s {
            "off" => Some(Self::Off),
            "firstPerson" => Some(Self::FirstPerson),
            "always" => Some(Self::Always),
            _ => None,
        }
    }
}

/// Starting age for the player.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub enum StartingAge {
    /// Start as child
    #[default]
    Child,
    /// Start as adult
    Adult,
    /// Random starting age
    Random,
}

impl StartingAge {
    /// Returns the string identifier used in logic expressions.
    #[must_use]
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::Child => "child",
            Self::Adult => "adult",
            Self::Random => "random",
        }
    }

    /// Parses a logic string identifier into a StartingAge.
    #[must_use]
    pub fn parse(s: &str) -> Option<Self> {
        match s {
            "child" => Some(Self::Child),
            "adult" => Some(Self::Adult),
            "random" => Some(Self::Random),
            _ => None,
        }
    }
}

/// Damage multiplier for enemies.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub enum DamageMultiplier {
    /// Half damage
    Half,
    /// Normal damage
    #[default]
    Normal,
    /// Double damage
    Double,
    /// Quadruple damage
    Quadruple,
    /// One-hit KO
    Ohko,
}

impl DamageMultiplier {
    /// Returns the string identifier used in logic expressions.
    #[must_use]
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::Half => "half",
            Self::Normal => "normal",
            Self::Double => "double",
            Self::Quadruple => "quadruple",
            Self::Ohko => "ohko",
        }
    }

    /// Parses a logic string identifier into a DamageMultiplier.
    #[must_use]
    pub fn parse(s: &str) -> Option<Self> {
        match s {
            "half" => Some(Self::Half),
            "normal" => Some(Self::Normal),
            "double" => Some(Self::Double),
            "quadruple" => Some(Self::Quadruple),
            "ohko" => Some(Self::Ohko),
            _ => None,
        }
    }
}

/// Item pool size.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub enum ItemPool {
    /// Plentiful item pool (more progression items)
    Plentiful,
    /// Normal item pool
    #[default]
    Normal,
    /// Scarce item pool (fewer items)
    Scarce,
    /// Minimal item pool (bare minimum)
    Minimal,
}

impl ItemPool {
    /// Returns the string identifier used in logic expressions.
    #[must_use]
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::Plentiful => "plentiful",
            Self::Normal => "normal",
            Self::Scarce => "scarce",
            Self::Minimal => "minimal",
        }
    }

    /// Parses a logic string identifier into an ItemPool.
    #[must_use]
    pub fn parse(s: &str) -> Option<Self> {
        match s {
            "plentiful" => Some(Self::Plentiful),
            "normal" => Some(Self::Normal),
            "scarce" => Some(Self::Scarce),
            "minimal" => Some(Self::Minimal),
            _ => None,
        }
    }
}

/// Traps quantity in the item pool.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub enum TrapsQuantity {
    /// No traps
    #[default]
    None,
    /// Few traps
    Few,
    /// Normal amount of traps
    Normal,
    /// Many traps
    Many,
    /// Maximum traps (most junk replaced)
    Onslaught,
}

impl TrapsQuantity {
    /// Returns the string identifier used in logic expressions.
    #[must_use]
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::None => "none",
            Self::Few => "few",
            Self::Normal => "normal",
            Self::Many => "many",
            Self::Onslaught => "onslaught",
        }
    }

    /// Parses a logic string identifier into a TrapsQuantity.
    #[must_use]
    pub fn parse(s: &str) -> Option<Self> {
        match s {
            "none" => Some(Self::None),
            "few" => Some(Self::Few),
            "normal" => Some(Self::Normal),
            "many" => Some(Self::Many),
            "onslaught" => Some(Self::Onslaught),
            _ => None,
        }
    }
}

/// Special condition for custom requirements (bridge, LACS, etc.).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SpecialCondition {
    /// Number of stones required
    #[serde(default)]
    pub stones: u8,
    /// Number of medallions required
    #[serde(default)]
    pub medallions: u8,
    /// Number of dungeon rewards required
    #[serde(default)]
    pub dungeon_rewards: u8,
    /// Number of Gold Skulltula tokens required
    #[serde(default)]
    pub skulltulas: u8,
    /// Number of boss remains required
    #[serde(default)]
    pub remains: u8,
}

impl SpecialCondition {
    /// Creates an empty condition with no requirements.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Creates a condition requiring specific medallions.
    #[must_use]
    pub fn with_medallions(count: u8) -> Self {
        Self {
            medallions: count,
            ..Default::default()
        }
    }

    /// Creates a condition requiring specific stones.
    #[must_use]
    pub fn with_stones(count: u8) -> Self {
        Self {
            stones: count,
            ..Default::default()
        }
    }

    /// Returns true if this condition has any requirements.
    #[must_use]
    pub fn has_requirements(&self) -> bool {
        self.stones > 0
            || self.medallions > 0
            || self.dungeon_rewards > 0
            || self.skulltulas > 0
            || self.remains > 0
    }
}

/// Type alias for starting items collection.
pub type StartingItems = HashMap<String, u32>;

/// Type alias for junk locations collection.
pub type JunkLocations = HashSet<String>;

/// World flags that affect gameplay logic.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct WorldFlags {
    /// Whether OoT world is enabled
    #[serde(default = "default_true")]
    pub oot_enabled: bool,
    /// Whether MM world is enabled
    #[serde(default = "default_true")]
    pub mm_enabled: bool,
    /// Whether shared items are enabled
    #[serde(default)]
    pub shared_items: bool,
    /// Whether shared masks are enabled
    #[serde(default)]
    pub shared_masks: bool,
}

fn default_true() -> bool {
    true
}

impl WorldFlags {
    /// Creates default world flags (both games enabled).
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Returns true if OoT world is accessible.
    #[must_use]
    pub fn is_oot_enabled(&self) -> bool {
        self.oot_enabled
    }

    /// Returns true if MM world is accessible.
    #[must_use]
    pub fn is_mm_enabled(&self) -> bool {
        self.mm_enabled
    }
}

/// Complete randomizer settings configuration.
///
/// This struct contains all settings that can affect logic evaluation
/// in an OoTMM randomizer seed.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RandomizerSettings {
    // === Boolean Settings ===
    // These are evaluated with `setting(name)` returning true/false
    /// Allow boots to be used without age constraints.
    #[serde(default)]
    pub ageless_boots: bool,

    /// Allow hookshot to be used without age constraints.
    #[serde(default)]
    pub ageless_hookshot: bool,

    /// Allow strength upgrades to be used without age constraints.
    #[serde(default)]
    pub ageless_strength: bool,

    /// Modify Lost Woods exits.
    #[serde(default)]
    pub alter_lost_woods_exits: bool,

    /// Enable entrance randomization for extra indoor locations.
    #[serde(default)]
    pub er_indoors_extra: bool,

    /// Enable entrance randomization for game link indoor locations.
    #[serde(default)]
    pub er_indoors_game_links: bool,

    /// Enable entrance randomization for major indoor locations.
    #[serde(default)]
    pub er_indoors_major: bool,

    /// Enable entrance randomization for the Moon.
    #[serde(default)]
    pub er_moon: bool,

    /// Open the Mask Shop without requirements.
    #[serde(default)]
    pub open_mask_shop: bool,

    /// Open Moon access without requirements.
    #[serde(default)]
    pub open_moon: bool,

    /// Open Zora's Domain shortcut.
    #[serde(default)]
    pub open_zd_shortcut: bool,

    /// Enable fishing pond fish shuffling.
    #[serde(default)]
    pub pond_fish_shuffle: bool,

    /// Restore broken actors in dungeons.
    #[serde(default)]
    pub restore_broken_actors: bool,

    /// Skip Child Zelda meeting requirement.
    #[serde(default)]
    pub skip_zelda: bool,

    /// Require Master Sword for time travel.
    #[serde(default)]
    pub time_travel_sword: bool,

    // === Shuffle Settings ===
    /// Scrub shuffle (OoT).
    #[serde(default)]
    pub scrub_shuffle_oot: bool,

    /// Scrub shuffle (MM).
    #[serde(default)]
    pub scrub_shuffle_mm: bool,

    /// Cow shuffle (OoT).
    #[serde(default)]
    pub cow_shuffle_oot: bool,

    /// Cow shuffle (MM).
    #[serde(default)]
    pub cow_shuffle_mm: bool,

    /// Beehive shuffle (OoT).
    #[serde(default)]
    pub shuffle_hives_oot: bool,

    /// Beehive shuffle (MM).
    #[serde(default)]
    pub shuffle_hives_mm: bool,

    /// Pot shuffle (OoT).
    #[serde(default)]
    pub shuffle_pots_oot: bool,

    /// Grass shuffle (OoT).
    #[serde(default)]
    pub shuffle_grass_oot: bool,

    /// Grass shuffle (MM).
    #[serde(default)]
    pub shuffle_grass_mm: bool,

    /// Freestanding items shuffle (OoT).
    #[serde(default)]
    pub shuffle_freestanding_oot: bool,

    /// Freestanding items shuffle (MM).
    #[serde(default)]
    pub shuffle_freestanding_mm: bool,

    /// Wonder items shuffle (OoT).
    #[serde(default)]
    pub shuffle_wonderitems_oot: bool,

    /// Wonder items shuffle (MM).
    #[serde(default)]
    pub shuffle_wonderitems_mm: bool,

    /// Snowball shuffle (MM).
    #[serde(default)]
    pub shuffle_snowballs_mm: bool,

    // === Souls Settings ===
    /// Enemy souls (OoT).
    #[serde(default)]
    pub souls_enemy_oot: bool,

    /// Enemy souls (MM).
    #[serde(default)]
    pub souls_enemy_mm: bool,

    /// Boss souls (OoT).
    #[serde(default)]
    pub souls_boss_oot: bool,

    /// Boss souls (MM).
    #[serde(default)]
    pub souls_boss_mm: bool,

    /// NPC souls (OoT).
    #[serde(default)]
    pub souls_npc_oot: bool,

    /// NPC souls (MM).
    #[serde(default)]
    pub souls_npc_mm: bool,

    // === Shared Item Settings ===
    /// Shared spin attack upgrade between games.
    #[serde(default)]
    pub shared_spin_upgrade: bool,

    /// Shared bows between games.
    #[serde(default)]
    pub shared_bows: bool,

    /// Shared bomb bags between games.
    #[serde(default)]
    pub shared_bomb_bags: bool,

    /// Shared magic upgrade between games.
    #[serde(default)]
    pub shared_magic_upgrade: bool,

    /// Shared wallets between games.
    #[serde(default)]
    pub shared_wallets: bool,

    /// Shared health between games.
    #[serde(default)]
    pub shared_health: bool,

    /// Shared shields between games.
    #[serde(default)]
    pub shared_shields: bool,

    /// Shared nuts and sticks between games.
    #[serde(default)]
    pub shared_nuts_sticks: bool,

    /// Shared hookshot between games.
    #[serde(default)]
    pub shared_hookshot: bool,

    /// Shared Lens of Truth between games.
    #[serde(default)]
    pub shared_lens: bool,

    /// Shared ocarina between games.
    #[serde(default)]
    pub shared_ocarina: bool,

    /// Shared masks between games.
    #[serde(default)]
    pub shared_masks: bool,

    /// Shared ocarina songs between games.
    #[serde(default)]
    pub shared_ocarinas_songs: bool,

    /// Shared Song of Time between games.
    #[serde(default)]
    pub shared_song_time: bool,

    /// Shared Epona's Song between games.
    #[serde(default)]
    pub shared_song_epona: bool,

    /// Shared Song of Storms between games.
    #[serde(default)]
    pub shared_song_storms: bool,

    /// Shared Sun's Song between games.
    #[serde(default)]
    pub shared_song_sun: bool,

    /// Shared Saria's Song between games.
    #[serde(default)]
    pub shared_song_saria: bool,

    /// Shared Zelda's Lullaby between games.
    #[serde(default)]
    pub shared_song_zelda: bool,

    /// Shared Song of Healing between games.
    #[serde(default)]
    pub shared_song_healing: bool,

    /// Shared Song of Soaring between games.
    #[serde(default)]
    pub shared_song_soaring: bool,

    // === Ageless Settings ===
    /// Ageless swords.
    #[serde(default)]
    pub ageless_swords: bool,

    /// Ageless shields.
    #[serde(default)]
    pub ageless_shields: bool,

    /// Ageless tunics.
    #[serde(default)]
    pub ageless_tunics: bool,

    /// Ageless sticks.
    #[serde(default)]
    pub ageless_sticks: bool,

    /// Ageless bombs.
    #[serde(default)]
    pub ageless_bombs: bool,

    /// Ageless boomerang.
    #[serde(default)]
    pub ageless_boomerang: bool,

    /// Ageless hammer.
    #[serde(default)]
    pub ageless_hammer: bool,

    /// Ageless child trade items.
    #[serde(default)]
    pub ageless_child_trade: bool,

    /// Ageless adult trade items.
    #[serde(default)]
    pub ageless_adult_trade: bool,

    // === Cross-Game Settings ===
    /// Cross-age play enabled.
    #[serde(default)]
    pub cross_age: bool,

    /// Cross-game Farore's Wind enabled.
    #[serde(default)]
    pub cross_game_fw: bool,

    // === MM-Specific Settings ===
    /// Fire spell available in MM.
    #[serde(default)]
    pub spell_fire_mm: bool,

    /// Iron Boots available in MM.
    #[serde(default)]
    pub boots_iron_mm: bool,

    /// Goron Tunic available in MM.
    #[serde(default)]
    pub tunic_goron_mm: bool,

    /// Zora Tunic available in MM.
    #[serde(default)]
    pub tunic_zora_mm: bool,

    /// Golden Scale available in MM.
    #[serde(default)]
    pub scale_gold_mm: bool,

    // === QOL/Features Settings ===
    /// Swordless adult allowed.
    #[serde(default)]
    pub swordless_adult: bool,

    /// Free scarecrow song in OoT.
    #[serde(default)]
    pub free_scarecrow_oot: bool,

    /// Blue Fire Arrows enabled.
    #[serde(default)]
    pub blue_fire_arrows: bool,

    /// Sunlight Arrows enabled.
    #[serde(default)]
    pub sunlight_arrows: bool,

    /// Fairy Ocarina available in MM.
    #[serde(default)]
    pub fairy_ocarina_mm: bool,

    // === Hints Settings ===
    /// Generate spoiler log.
    #[serde(default)]
    pub generate_spoiler_log: bool,

    /// Probabilistic foolish hints.
    #[serde(default)]
    pub probabilistic_foolish: bool,

    /// Hint importance enabled.
    #[serde(default)]
    pub hint_importance: bool,

    // === Traps Settings ===
    /// Ice traps enabled.
    #[serde(default)]
    pub trap_ice: bool,

    /// Fire traps enabled.
    #[serde(default)]
    pub trap_fire: bool,

    /// Shock traps enabled.
    #[serde(default)]
    pub trap_shock: bool,

    /// Cloak traps (disguised traps).
    #[serde(default)]
    pub cloak_traps: bool,

    // === Misc Settings ===
    /// Clocks shuffled.
    #[serde(default)]
    pub clocks: bool,

    /// Menu notebook enabled.
    #[serde(default)]
    pub menu_notebook: bool,

    /// Coins enabled.
    #[serde(default)]
    pub coins: bool,

    /// Void warp in MM enabled.
    #[serde(default)]
    pub void_warp_mm: bool,

    // === Enumerated Settings ===
    // These are evaluated with `setting(name, value)`
    /// Set of OoT dungeons that are open without requirements.
    #[serde(default)]
    pub open_dungeons_oot: HashSet<OotDungeon>,

    /// Set of MM dungeons that are open without requirements.
    #[serde(default)]
    pub open_dungeons_mm: HashSet<MmDungeon>,

    /// Set of OoT dungeons that use Master Quest layouts.
    ///
    /// When a dungeon is in this set, its checks use MQ flag mappings
    /// instead of vanilla mappings. This affects which locations are
    /// tracked and their memory flag addresses.
    #[serde(default)]
    pub mq_dungeons: HashSet<MqDungeon>,

    /// Deku Tree entrance state.
    #[serde(default)]
    pub deku_tree: DekuTreeState,

    /// Door of Time state.
    #[serde(default)]
    pub door_of_time: DoorOfTimeState,

    /// Kakariko Village gate state.
    #[serde(default)]
    pub kakariko_gate: KakarikoGateState,

    /// Ganon's Castle Boss Key mode.
    #[serde(default)]
    pub ganon_boss_key: GanonBossKeyMode,

    /// Light Arrow Cutscene mode.
    #[serde(default)]
    pub lacs: LacsMode,

    /// Majora child mode.
    #[serde(default)]
    pub majora_child: MajoraChildMode,

    /// Moon crash behavior.
    #[serde(default)]
    pub moon_crash: MoonCrashMode,

    /// Age change mode.
    #[serde(default)]
    pub age_change: AgeChangeMode,

    /// Climb Most Surfaces glitch state (OoT).
    #[serde(default)]
    pub climb_most_surfaces_oot: ClimbMostSurfacesState,

    /// Hookshot Anywhere glitch state (OoT).
    #[serde(default)]
    pub hookshot_anywhere_oot: HookshotAnywhereState,

    /// Beneath the Well state.
    #[serde(default)]
    pub beneath_well: BeneathWellState,

    /// Entrance Randomizer overworld state.
    #[serde(default)]
    pub er_overworld: ErOverworldState,

    /// Entrance Randomizer grottos state.
    #[serde(default)]
    pub er_grottos: ErGrottosState,

    /// Boss Warp Pads mode.
    #[serde(default)]
    pub boss_warp_pads: BossWarpPadsMode,

    /// Clear state for MM dungeons.
    #[serde(default)]
    pub clear_state_dungeons_mm: HashSet<ClearStateDungeonsMm>,

    /// Japan-specific layouts enabled.
    #[serde(default)]
    pub jp_layouts: HashSet<JpLayout>,

    /// Small key shuffle mode for OoT.
    #[serde(default)]
    pub small_key_shuffle_oot: SmallKeyShuffleOot,

    /// Shuffle pots mode for MM.
    #[serde(default)]
    pub shuffle_pots_mm: ShufflePotsMm,

    /// Logic rules mode.
    #[serde(default)]
    pub logic_mode: LogicMode,

    /// Set of enabled logic tricks.
    #[serde(default)]
    pub logic_tricks: HashSet<String>,

    /// Maximum number of bottles (for shared bottle randomizer settings).
    ///
    /// In OoTMM with shared bottles, players may have fewer than 4 bottles.
    /// Valid range is 1-4, defaults to 4.
    #[serde(default = "default_bottle_count")]
    pub bottle_count: u8,

    // === Game Mode Settings ===
    /// Rainbow Bridge access requirements mode.
    #[serde(default)]
    pub rainbow_bridge: RainbowBridgeMode,

    /// Song shuffle mode.
    #[serde(default)]
    pub songs: SongsMode,

    /// Dungeon reward shuffle mode.
    #[serde(default)]
    pub dungeon_reward_shuffle: DungeonRewardShuffle,

    // === Shop/Price Settings ===
    /// Shop shuffle mode for OoT.
    #[serde(default)]
    pub shop_shuffle_oot: ShopShuffleMode,

    /// Shop shuffle mode for MM.
    #[serde(default)]
    pub shop_shuffle_mm: ShopShuffleMode,

    /// Price mode for OoT shops.
    #[serde(default)]
    pub price_oot_shops: PriceMode,

    /// Price mode for OoT scrubs.
    #[serde(default)]
    pub price_oot_scrubs: PriceMode,

    /// Price mode for MM shops.
    #[serde(default)]
    pub price_mm_shops: PriceMode,

    /// Price mode for Tingle maps.
    #[serde(default)]
    pub tingle_prices: PriceMode,

    // === Fairy Shuffle Settings ===
    /// Town fairy shuffle mode (Clock Town stray fairies).
    #[serde(default)]
    pub town_fairy_shuffle: TownFairyShuffle,

    /// Stray fairy shuffle mode for chest fairies.
    #[serde(default)]
    pub stray_fairy_chest_shuffle: StrayFairyShuffle,

    /// Stray fairy shuffle mode for other fairies.
    #[serde(default)]
    pub stray_fairy_other_shuffle: StrayFairyShuffle,

    // === Cross-Warp Settings ===
    /// Cross-warp mode for OoT.
    #[serde(default)]
    pub cross_warp_oot: CrossWarpMode,

    /// Cross-warp mode for MM.
    #[serde(default)]
    pub cross_warp_mm: CrossWarpMode,

    // === Miscellaneous Enum Settings ===
    /// Chest Size Matches Contents mode.
    #[serde(default)]
    pub csmc: CsmcMode,

    /// Bombchu behavior mode.
    #[serde(default)]
    pub bombchu_behavior: BombchuBehavior,

    /// Auto-invert camera mode.
    #[serde(default)]
    pub auto_invert: AutoInvertMode,

    /// Starting age for the player.
    #[serde(default)]
    pub starting_age: StartingAge,

    /// Damage multiplier.
    #[serde(default)]
    pub damage_multiplier: DamageMultiplier,

    /// Item pool size.
    #[serde(default)]
    pub item_pool: ItemPool,

    /// Traps quantity in the item pool.
    #[serde(default)]
    pub traps_quantity: TrapsQuantity,

    // === Collection Fields ===
    /// Special conditions for custom requirements.
    #[serde(default)]
    pub special_conditions: HashMap<String, SpecialCondition>,

    /// Starting items and their quantities.
    #[serde(default)]
    pub starting_items: StartingItems,

    /// Locations designated as junk.
    #[serde(default)]
    pub junk_locations: JunkLocations,

    /// World flags affecting gameplay.
    #[serde(default)]
    pub world_flags: WorldFlags,
}

/// Returns the default bottle count (4).
fn default_bottle_count() -> u8 {
    4
}

impl Default for RandomizerSettings {
    /// Creates default "casual" settings configuration.
    ///
    /// Default settings represent a standard playthrough without
    /// any open dungeons, entrance randomization, or glitch logic.
    fn default() -> Self {
        Self {
            // Boolean settings default to false
            ageless_boots: false,
            ageless_hookshot: false,
            ageless_strength: false,
            alter_lost_woods_exits: false,
            er_indoors_extra: false,
            er_indoors_game_links: false,
            er_indoors_major: false,
            er_moon: false,
            open_mask_shop: false,
            open_moon: false,
            open_zd_shortcut: false,
            pond_fish_shuffle: false,
            restore_broken_actors: false,
            skip_zelda: false,
            time_travel_sword: false,

            // Shuffle settings
            scrub_shuffle_oot: false,
            scrub_shuffle_mm: false,
            cow_shuffle_oot: false,
            cow_shuffle_mm: false,
            shuffle_hives_oot: false,
            shuffle_hives_mm: false,
            shuffle_pots_oot: false,
            shuffle_grass_oot: false,
            shuffle_grass_mm: false,
            shuffle_freestanding_oot: false,
            shuffle_freestanding_mm: false,
            shuffle_wonderitems_oot: false,
            shuffle_wonderitems_mm: false,
            shuffle_snowballs_mm: false,

            // Souls settings
            souls_enemy_oot: false,
            souls_enemy_mm: false,
            souls_boss_oot: false,
            souls_boss_mm: false,
            souls_npc_oot: false,
            souls_npc_mm: false,

            // Shared item settings
            shared_spin_upgrade: false,
            shared_bows: false,
            shared_bomb_bags: false,
            shared_magic_upgrade: false,
            shared_wallets: false,
            shared_health: false,
            shared_shields: false,
            shared_nuts_sticks: false,
            shared_hookshot: false,
            shared_lens: false,
            shared_ocarina: false,
            shared_masks: false,
            shared_ocarinas_songs: false,
            shared_song_time: false,
            shared_song_epona: false,
            shared_song_storms: false,
            shared_song_sun: false,
            shared_song_saria: false,
            shared_song_zelda: false,
            shared_song_healing: false,
            shared_song_soaring: false,

            // Ageless settings
            ageless_swords: false,
            ageless_shields: false,
            ageless_tunics: false,
            ageless_sticks: false,
            ageless_bombs: false,
            ageless_boomerang: false,
            ageless_hammer: false,
            ageless_child_trade: false,
            ageless_adult_trade: false,

            // Cross-game settings
            cross_age: false,
            cross_game_fw: false,

            // MM-specific settings
            spell_fire_mm: false,
            boots_iron_mm: false,
            tunic_goron_mm: false,
            tunic_zora_mm: false,
            scale_gold_mm: false,

            // QOL/Features settings
            swordless_adult: false,
            free_scarecrow_oot: false,
            blue_fire_arrows: false,
            sunlight_arrows: false,
            fairy_ocarina_mm: false,

            // Hints settings
            generate_spoiler_log: false,
            probabilistic_foolish: false,
            hint_importance: false,

            // Traps settings
            trap_ice: false,
            trap_fire: false,
            trap_shock: false,
            cloak_traps: false,

            // Misc settings
            clocks: false,
            menu_notebook: false,
            coins: false,
            void_warp_mm: false,

            // Set settings default to empty
            open_dungeons_oot: HashSet::new(),
            open_dungeons_mm: HashSet::new(),
            mq_dungeons: HashSet::new(),
            clear_state_dungeons_mm: HashSet::new(),
            jp_layouts: HashSet::new(),
            logic_tricks: HashSet::new(),

            // Enum settings default to their Default variants
            deku_tree: DekuTreeState::default(),
            door_of_time: DoorOfTimeState::default(),
            kakariko_gate: KakarikoGateState::default(),
            ganon_boss_key: GanonBossKeyMode::default(),
            lacs: LacsMode::default(),
            majora_child: MajoraChildMode::default(),
            moon_crash: MoonCrashMode::default(),
            age_change: AgeChangeMode::default(),
            climb_most_surfaces_oot: ClimbMostSurfacesState::default(),
            hookshot_anywhere_oot: HookshotAnywhereState::default(),
            beneath_well: BeneathWellState::default(),
            er_overworld: ErOverworldState::default(),
            er_grottos: ErGrottosState::default(),
            boss_warp_pads: BossWarpPadsMode::default(),
            small_key_shuffle_oot: SmallKeyShuffleOot::default(),
            shuffle_pots_mm: ShufflePotsMm::default(),
            logic_mode: LogicMode::default(),
            bottle_count: 4,

            // Game mode settings
            rainbow_bridge: RainbowBridgeMode::default(),
            songs: SongsMode::default(),
            dungeon_reward_shuffle: DungeonRewardShuffle::default(),

            // Shop/price settings
            shop_shuffle_oot: ShopShuffleMode::default(),
            shop_shuffle_mm: ShopShuffleMode::default(),
            price_oot_shops: PriceMode::default(),
            price_oot_scrubs: PriceMode::default(),
            price_mm_shops: PriceMode::default(),
            tingle_prices: PriceMode::default(),

            // Fairy shuffle settings
            town_fairy_shuffle: TownFairyShuffle::default(),
            stray_fairy_chest_shuffle: StrayFairyShuffle::default(),
            stray_fairy_other_shuffle: StrayFairyShuffle::default(),

            // Cross-warp settings
            cross_warp_oot: CrossWarpMode::default(),
            cross_warp_mm: CrossWarpMode::default(),

            // Miscellaneous enum settings
            csmc: CsmcMode::default(),
            bombchu_behavior: BombchuBehavior::default(),
            auto_invert: AutoInvertMode::default(),
            starting_age: StartingAge::default(),
            damage_multiplier: DamageMultiplier::default(),
            item_pool: ItemPool::default(),
            traps_quantity: TrapsQuantity::default(),

            // Collection fields
            special_conditions: HashMap::new(),
            starting_items: StartingItems::new(),
            junk_locations: JunkLocations::new(),
            world_flags: WorldFlags::default(),
        }
    }
}

impl RandomizerSettings {
    /// Creates a new settings instance with default values.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Checks if a boolean setting is enabled.
    ///
    /// This is used for `setting(name)` logic expressions.
    #[must_use]
    pub fn get_bool_setting(&self, name: &str) -> Option<bool> {
        match name {
            // Original boolean settings
            "agelessBoots" => Some(self.ageless_boots),
            "agelessHookshot" => Some(self.ageless_hookshot),
            "agelessStrength" => Some(self.ageless_strength),
            "alterLostWoodsExits" => Some(self.alter_lost_woods_exits),
            "erIndoorsExtra" => Some(self.er_indoors_extra),
            "erIndoorsGameLinks" => Some(self.er_indoors_game_links),
            "erIndoorsMajor" => Some(self.er_indoors_major),
            "erMoon" => Some(self.er_moon),
            "openMaskShop" => Some(self.open_mask_shop),
            "openMoon" => Some(self.open_moon),
            "openZdShortcut" => Some(self.open_zd_shortcut),
            "pondFishShuffle" => Some(self.pond_fish_shuffle),
            "restoreBrokenActors" => Some(self.restore_broken_actors),
            "skipZelda" => Some(self.skip_zelda),
            "timeTravelSword" => Some(self.time_travel_sword),

            // Shuffle settings
            "scrubShuffleOot" => Some(self.scrub_shuffle_oot),
            "scrubShuffleMm" => Some(self.scrub_shuffle_mm),
            "cowShuffleOot" => Some(self.cow_shuffle_oot),
            "cowShuffleMm" => Some(self.cow_shuffle_mm),
            "shuffleHivesOot" => Some(self.shuffle_hives_oot),
            "shuffleHivesMm" => Some(self.shuffle_hives_mm),
            "shufflePotsOot" => Some(self.shuffle_pots_oot),
            "shuffleGrassOot" => Some(self.shuffle_grass_oot),
            "shuffleGrassMm" => Some(self.shuffle_grass_mm),
            "shuffleFreestandingOot" => Some(self.shuffle_freestanding_oot),
            "shuffleFreestandingMm" => Some(self.shuffle_freestanding_mm),
            "shuffleWonderitemsOot" => Some(self.shuffle_wonderitems_oot),
            "shuffleWonderitemsMm" => Some(self.shuffle_wonderitems_mm),
            "shuffleSnowballsMm" => Some(self.shuffle_snowballs_mm),

            // Souls settings
            "soulsEnemyOot" => Some(self.souls_enemy_oot),
            "soulsEnemyMm" => Some(self.souls_enemy_mm),
            "soulsBossOot" => Some(self.souls_boss_oot),
            "soulsBossMm" => Some(self.souls_boss_mm),
            "soulsNpcOot" => Some(self.souls_npc_oot),
            "soulsNpcMm" => Some(self.souls_npc_mm),

            // Shared item settings
            "sharedSpinUpgrade" => Some(self.shared_spin_upgrade),
            "sharedBows" => Some(self.shared_bows),
            "sharedBombBags" => Some(self.shared_bomb_bags),
            "sharedMagicUpgrade" => Some(self.shared_magic_upgrade),
            "sharedWallets" => Some(self.shared_wallets),
            "sharedHealth" => Some(self.shared_health),
            "sharedShields" => Some(self.shared_shields),
            "sharedNutsSticks" => Some(self.shared_nuts_sticks),
            "sharedHookshot" => Some(self.shared_hookshot),
            "sharedLens" => Some(self.shared_lens),
            "sharedOcarina" => Some(self.shared_ocarina),
            "sharedMasks" => Some(self.shared_masks),
            "sharedOcarinasSongs" => Some(self.shared_ocarinas_songs),
            "sharedSongTime" => Some(self.shared_song_time),
            "sharedSongEpona" => Some(self.shared_song_epona),
            "sharedSongStorms" => Some(self.shared_song_storms),
            "sharedSongSun" => Some(self.shared_song_sun),
            "sharedSongSaria" => Some(self.shared_song_saria),
            "sharedSongZelda" => Some(self.shared_song_zelda),
            "sharedSongHealing" => Some(self.shared_song_healing),
            "sharedSongSoaring" => Some(self.shared_song_soaring),

            // Ageless settings
            "agelessSwords" => Some(self.ageless_swords),
            "agelessShields" => Some(self.ageless_shields),
            "agelessTunics" => Some(self.ageless_tunics),
            "agelessSticks" => Some(self.ageless_sticks),
            "agelessBombs" => Some(self.ageless_bombs),
            "agelessBoomerang" => Some(self.ageless_boomerang),
            "agelessHammer" => Some(self.ageless_hammer),
            "agelessChildTrade" => Some(self.ageless_child_trade),
            "agelessAdultTrade" => Some(self.ageless_adult_trade),

            // Cross-game settings
            "crossAge" => Some(self.cross_age),
            "crossGameFw" => Some(self.cross_game_fw),

            // MM-specific settings
            "spellFireMm" => Some(self.spell_fire_mm),
            "bootsIronMm" => Some(self.boots_iron_mm),
            "tunicGoronMm" => Some(self.tunic_goron_mm),
            "tunicZoraMm" => Some(self.tunic_zora_mm),
            "scaleGoldMm" => Some(self.scale_gold_mm),

            // QOL/Features settings
            "swordlessAdult" => Some(self.swordless_adult),
            "freeScarecrowOot" => Some(self.free_scarecrow_oot),
            "blueFireArrows" => Some(self.blue_fire_arrows),
            "sunlightArrows" => Some(self.sunlight_arrows),
            "fairyOcarinaMm" => Some(self.fairy_ocarina_mm),

            // Hints settings
            "generateSpoilerLog" => Some(self.generate_spoiler_log),
            "probabilisticFoolish" => Some(self.probabilistic_foolish),
            "hintImportance" => Some(self.hint_importance),

            // Traps settings
            "trapIce" => Some(self.trap_ice),
            "trapFire" => Some(self.trap_fire),
            "trapShock" => Some(self.trap_shock),
            "cloakTraps" => Some(self.cloak_traps),

            // Misc settings
            "clocks" => Some(self.clocks),
            "menuNotebook" => Some(self.menu_notebook),
            "coins" => Some(self.coins),
            "voidWarpMm" => Some(self.void_warp_mm),

            _ => None,
        }
    }

    /// Checks if a setting has a specific value.
    ///
    /// This is used for `setting(name, value)` logic expressions.
    #[must_use]
    pub fn check_setting_value(&self, name: &str, value: &str) -> bool {
        match name {
            "openDungeonsOot" => OotDungeon::parse(value)
                .map(|d| self.open_dungeons_oot.contains(&d))
                .unwrap_or(false),
            "openDungeonsMm" => MmDungeon::parse(value)
                .map(|d| self.open_dungeons_mm.contains(&d))
                .unwrap_or(false),
            "mqDungeons" => MqDungeon::parse(value)
                .map(|d| self.mq_dungeons.contains(&d))
                .unwrap_or(false),
            "dekuTree" => self.deku_tree.as_str() == value,
            "doorOfTime" => self.door_of_time.as_str() == value,
            "kakarikoGate" => self.kakariko_gate.as_str() == value,
            "ganonBossKey" => self.ganon_boss_key.as_str() == value,
            "lacs" => self.lacs.as_str() == value,
            "majoraChild" => self.majora_child.as_str() == value,
            "moonCrash" => self.moon_crash.as_str() == value,
            "ageChange" => self.age_change.as_str() == value,
            "climbMostSurfacesOot" => self.climb_most_surfaces_oot.as_str() == value,
            "hookshotAnywhereOot" => self.hookshot_anywhere_oot.as_str() == value,
            "beneathWell" => self.beneath_well.as_str() == value,
            "erOverworld" => self.er_overworld.as_str() == value,
            "erGrottos" => self.er_grottos.as_str() == value,
            "bossWarpPads" => self.boss_warp_pads.as_str() == value,
            "clearStateDungeonsMm" => ClearStateDungeonsMm::parse(value)
                .map(|d| self.clear_state_dungeons_mm.contains(&d))
                .unwrap_or(false),
            "jpLayouts" => JpLayout::parse(value)
                .map(|l| self.jp_layouts.contains(&l))
                .unwrap_or(false),
            "smallKeyShuffleOot" => self.small_key_shuffle_oot.as_str() == value,
            "shufflePotsMm" => self.shuffle_pots_mm.as_str() == value,
            "logicMode" => self.logic_mode.as_str() == value,
            // Game mode settings
            "rainbowBridge" => self.rainbow_bridge.as_str() == value,
            "songs" => self.songs.as_str() == value,
            "dungeonRewardShuffle" => self.dungeon_reward_shuffle.as_str() == value,
            // Shop/price settings
            "shopShuffleOot" => self.shop_shuffle_oot.as_str() == value,
            "shopShuffleMm" => self.shop_shuffle_mm.as_str() == value,
            "priceOotShops" => self.price_oot_shops.as_str() == value,
            "priceOotScrubs" => self.price_oot_scrubs.as_str() == value,
            "priceMmShops" => self.price_mm_shops.as_str() == value,
            "tinglePrices" => self.tingle_prices.as_str() == value,
            // Fairy shuffle settings
            "townFairyShuffle" => self.town_fairy_shuffle.as_str() == value,
            "strayFairyChestShuffle" => self.stray_fairy_chest_shuffle.as_str() == value,
            "strayFairyOtherShuffle" => self.stray_fairy_other_shuffle.as_str() == value,
            // Cross-warp settings
            "crossWarpOot" => self.cross_warp_oot.as_str() == value,
            "crossWarpMm" => self.cross_warp_mm.as_str() == value,
            // Miscellaneous enum settings
            "csmc" => self.csmc.as_str() == value,
            "bombchuBehavior" => self.bombchu_behavior.as_str() == value,
            "autoInvert" => self.auto_invert.as_str() == value,
            "startingAge" => self.starting_age.as_str() == value,
            "damageMultiplier" => self.damage_multiplier.as_str() == value,
            "itemPool" => self.item_pool.as_str() == value,
            "trapsQuantity" => self.traps_quantity.as_str() == value,
            _ => false,
        }
    }

    /// Checks if a logic trick is enabled.
    #[must_use]
    pub fn has_trick(&self, trick: &str) -> bool {
        self.logic_tricks.contains(trick)
    }

    /// Enables a logic trick.
    pub fn enable_trick(&mut self, trick: impl Into<String>) {
        self.logic_tricks.insert(trick.into());
    }

    /// Disables a logic trick.
    pub fn disable_trick(&mut self, trick: &str) {
        self.logic_tricks.remove(trick);
    }

    // === Bottle Count Methods ===

    /// Returns the maximum bottle count for this seed.
    ///
    /// For shared bottle randomizer settings, this may be less than 4.
    #[must_use]
    pub fn get_bottle_count(&self) -> u8 {
        self.bottle_count.clamp(1, 4)
    }

    /// Sets the maximum bottle count.
    ///
    /// The value is clamped to the valid range of 1-4.
    pub fn set_bottle_count(&mut self, count: u8) {
        self.bottle_count = count.clamp(1, 4);
    }

    // === Master Quest Dungeon Methods ===

    /// Checks if a dungeon is set to Master Quest.
    #[must_use]
    pub fn is_dungeon_mq(&self, dungeon: MqDungeon) -> bool {
        self.mq_dungeons.contains(&dungeon)
    }

    /// Checks if a dungeon is set to Master Quest by its string identifier.
    #[must_use]
    pub fn is_dungeon_mq_by_name(&self, name: &str) -> bool {
        MqDungeon::parse(name)
            .map(|d| self.mq_dungeons.contains(&d))
            .unwrap_or(false)
    }

    /// Sets a dungeon to Master Quest mode.
    pub fn set_dungeon_mq(&mut self, dungeon: MqDungeon) {
        self.mq_dungeons.insert(dungeon);
    }

    /// Sets a dungeon to vanilla (non-MQ) mode.
    pub fn set_dungeon_vanilla(&mut self, dungeon: MqDungeon) {
        self.mq_dungeons.remove(&dungeon);
    }

    /// Sets all dungeons to Master Quest mode.
    pub fn set_all_dungeons_mq(&mut self) {
        for &dungeon in MqDungeon::all() {
            self.mq_dungeons.insert(dungeon);
        }
    }

    /// Sets all dungeons to vanilla (non-MQ) mode.
    pub fn set_all_dungeons_vanilla(&mut self) {
        self.mq_dungeons.clear();
    }

    /// Returns the location ID prefix for a dungeon based on its MQ status.
    ///
    /// This is used to determine which set of flag mappings to use.
    #[must_use]
    pub fn get_dungeon_location_prefix(&self, dungeon: MqDungeon) -> &'static str {
        if self.is_dungeon_mq(dungeon) {
            dungeon.mq_location_prefix()
        } else {
            dungeon.vanilla_location_prefix()
        }
    }

    /// Determines if a location ID should be active based on MQ settings.
    ///
    /// Returns `true` if the location matches the current MQ/vanilla state
    /// of its dungeon, or if the location is not in an MQ-able dungeon.
    #[must_use]
    pub fn is_location_active(&self, location_id: &str) -> bool {
        // Check if this is an MQ dungeon location
        if let Some(dungeon) = MqDungeon::from_location_id(location_id) {
            let is_mq_location = location_id.starts_with("mq_oot_");
            let dungeon_is_mq = self.is_dungeon_mq(dungeon);

            // Location is active if its MQ status matches the dungeon's setting
            is_mq_location == dungeon_is_mq
        } else {
            // Non-dungeon locations are always active
            true
        }
    }

    /// Returns the count of MQ dungeons.
    #[must_use]
    pub fn mq_dungeon_count(&self) -> usize {
        self.mq_dungeons.len()
    }

    // === Special Condition Methods ===

    /// Gets a special condition by name.
    #[must_use]
    pub fn get_special_condition(&self, name: &str) -> Option<&SpecialCondition> {
        self.special_conditions.get(name)
    }

    /// Gets the bridge special condition.
    ///
    /// Returns the special condition for custom rainbow bridge requirements
    /// when `rainbow_bridge` is set to `Custom`.
    #[must_use]
    pub fn bridge_condition(&self) -> Option<&SpecialCondition> {
        self.special_conditions.get("bridge")
    }

    // === Junk Location Methods ===

    /// Checks if a location is designated as junk.
    #[must_use]
    pub fn is_junk_location(&self, location: &str) -> bool {
        self.junk_locations.contains(location)
    }

    // === Starting Items Methods ===

    /// Returns the quantity of a starting item.
    ///
    /// Returns 0 if the item is not in the starting items.
    #[must_use]
    pub fn starting_item_quantity(&self, item: &str) -> u32 {
        self.starting_items.get(item).copied().unwrap_or(0)
    }

    /// Checks if a starting item is present (quantity > 0).
    #[must_use]
    pub fn has_starting_item(&self, item: &str) -> bool {
        self.starting_item_quantity(item) > 0
    }

    /// Adds or updates a starting item quantity.
    pub fn set_starting_item(&mut self, item: impl Into<String>, quantity: u32) {
        if quantity > 0 {
            self.starting_items.insert(item.into(), quantity);
        } else {
            self.starting_items.remove(&item.into());
        }
    }

    /// Removes a starting item.
    pub fn remove_starting_item(&mut self, item: &str) {
        self.starting_items.remove(item);
    }

    /// Returns an iterator over starting items and their quantities.
    pub fn starting_items_iter(&self) -> impl Iterator<Item = (&String, &u32)> {
        self.starting_items.iter()
    }

    /// Returns the number of distinct starting items.
    #[must_use]
    pub fn starting_items_count(&self) -> usize {
        self.starting_items.len()
    }

    // === Additional Junk Location Methods ===

    /// Adds a location to the junk locations set.
    pub fn add_junk_location(&mut self, location: impl Into<String>) {
        self.junk_locations.insert(location.into());
    }

    /// Removes a location from the junk locations set.
    pub fn remove_junk_location(&mut self, location: &str) {
        self.junk_locations.remove(location);
    }

    /// Returns an iterator over junk locations.
    pub fn junk_locations_iter(&self) -> impl Iterator<Item = &String> {
        self.junk_locations.iter()
    }

    /// Returns the number of junk locations.
    #[must_use]
    pub fn junk_locations_count(&self) -> usize {
        self.junk_locations.len()
    }

    // === Additional Special Condition Methods ===

    /// Checks if a special condition exists.
    #[must_use]
    pub fn has_special_condition(&self, name: &str) -> bool {
        self.special_conditions.contains_key(name)
    }

    /// Sets a special condition.
    pub fn set_special_condition(&mut self, name: impl Into<String>, condition: SpecialCondition) {
        self.special_conditions.insert(name.into(), condition);
    }

    /// Removes a special condition.
    pub fn remove_special_condition(&mut self, name: &str) {
        self.special_conditions.remove(name);
    }

    /// Returns an iterator over special conditions.
    pub fn special_conditions_iter(&self) -> impl Iterator<Item = (&String, &SpecialCondition)> {
        self.special_conditions.iter()
    }

    /// Returns the number of special conditions.
    #[must_use]
    pub fn special_conditions_count(&self) -> usize {
        self.special_conditions.len()
    }

    // === World Flags Accessors ===

    /// Returns whether OoT world is enabled.
    #[must_use]
    pub fn is_oot_enabled(&self) -> bool {
        self.world_flags.oot_enabled
    }

    /// Returns whether MM world is enabled.
    #[must_use]
    pub fn is_mm_enabled(&self) -> bool {
        self.world_flags.mm_enabled
    }

    /// Returns whether shared items are enabled in world flags.
    #[must_use]
    pub fn world_shared_items(&self) -> bool {
        self.world_flags.shared_items
    }

    /// Returns whether shared masks are enabled in world flags.
    #[must_use]
    pub fn world_shared_masks(&self) -> bool {
        self.world_flags.shared_masks
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_settings() {
        let settings = RandomizerSettings::default();
        assert!(!settings.ageless_boots);
        assert!(!settings.er_moon);
        assert!(settings.open_dungeons_oot.is_empty());
        assert!(settings.open_dungeons_mm.is_empty());
        assert_eq!(settings.deku_tree, DekuTreeState::Closed);
        assert_eq!(settings.logic_mode, LogicMode::Glitchless);
    }

    #[test]
    fn test_bool_setting_lookup() {
        let mut settings = RandomizerSettings::new();
        settings.ageless_boots = true;
        settings.er_moon = true;

        assert_eq!(settings.get_bool_setting("agelessBoots"), Some(true));
        assert_eq!(settings.get_bool_setting("erMoon"), Some(true));
        assert_eq!(settings.get_bool_setting("skipZelda"), Some(false));
        assert_eq!(settings.get_bool_setting("unknownSetting"), None);
    }

    #[test]
    fn test_value_setting_lookup_dungeons() {
        let mut settings = RandomizerSettings::new();
        settings
            .open_dungeons_oot
            .insert(OotDungeon::DodongosCavern);
        settings
            .open_dungeons_oot
            .insert(OotDungeon::BottomOfTheWell);
        settings.open_dungeons_mm.insert(MmDungeon::StoneTower);

        assert!(settings.check_setting_value("openDungeonsOot", "DC"));
        assert!(settings.check_setting_value("openDungeonsOot", "BotW"));
        assert!(!settings.check_setting_value("openDungeonsOot", "Shadow"));
        assert!(settings.check_setting_value("openDungeonsMm", "ST"));
        assert!(!settings.check_setting_value("openDungeonsMm", "WF"));
    }

    #[test]
    fn test_value_setting_lookup_enums() {
        let mut settings = RandomizerSettings::new();
        settings.deku_tree = DekuTreeState::Open;
        settings.ganon_boss_key = GanonBossKeyMode::Removed;
        settings.age_change = AgeChangeMode::None;

        assert!(settings.check_setting_value("dekuTree", "open"));
        assert!(!settings.check_setting_value("dekuTree", "vanilla"));
        assert!(settings.check_setting_value("ganonBossKey", "removed"));
        assert!(settings.check_setting_value("ageChange", "none"));
    }

    #[test]
    fn test_jp_layouts() {
        let mut settings = RandomizerSettings::new();
        settings.jp_layouts.insert(JpLayout::StoneTower);

        assert!(settings.check_setting_value("jpLayouts", "StoneTower"));
        assert!(!settings.check_setting_value("jpLayouts", "ST"));
    }

    #[test]
    fn test_tricks() {
        let mut settings = RandomizerSettings::new();
        assert!(!settings.has_trick("hover_boost"));

        settings.enable_trick("hover_boost");
        assert!(settings.has_trick("hover_boost"));

        settings.disable_trick("hover_boost");
        assert!(!settings.has_trick("hover_boost"));
    }

    #[test]
    fn test_oot_dungeon_from_str() {
        assert_eq!(OotDungeon::parse("DC"), Some(OotDungeon::DodongosCavern));
        assert_eq!(OotDungeon::parse("BotW"), Some(OotDungeon::BottomOfTheWell));
        assert_eq!(OotDungeon::parse("JJ"), Some(OotDungeon::JabuJabu));
        assert_eq!(OotDungeon::parse("Shadow"), Some(OotDungeon::Shadow));
        assert_eq!(OotDungeon::parse("Water"), Some(OotDungeon::Water));
        assert_eq!(OotDungeon::parse("fireChild"), Some(OotDungeon::FireChild));
        assert_eq!(OotDungeon::parse("wellAdult"), Some(OotDungeon::WellAdult));
        assert_eq!(OotDungeon::parse("invalid"), None);
    }

    #[test]
    fn test_mm_dungeon_from_str() {
        assert_eq!(MmDungeon::parse("ST"), Some(MmDungeon::StoneTower));
        assert_eq!(MmDungeon::parse("WF"), Some(MmDungeon::Woodfall));
        assert_eq!(MmDungeon::parse("invalid"), None);
    }

    #[test]
    fn test_serde_roundtrip() {
        let mut settings = RandomizerSettings::new();
        settings.ageless_boots = true;
        settings
            .open_dungeons_oot
            .insert(OotDungeon::DodongosCavern);
        settings.deku_tree = DekuTreeState::Open;
        settings.enable_trick("hover_boost");

        let json = serde_json::to_string(&settings).unwrap();
        let parsed: RandomizerSettings = serde_json::from_str(&json).unwrap();

        assert_eq!(parsed.ageless_boots, true);
        assert!(parsed
            .open_dungeons_oot
            .contains(&OotDungeon::DodongosCavern));
        assert_eq!(parsed.deku_tree, DekuTreeState::Open);
        assert!(parsed.has_trick("hover_boost"));
    }

    #[test]
    fn test_serde_yaml_roundtrip() {
        let mut settings = RandomizerSettings::new();
        settings.er_moon = true;
        settings.open_dungeons_mm.insert(MmDungeon::StoneTower);
        settings.ganon_boss_key = GanonBossKeyMode::Custom;

        let yaml = serde_yaml::to_string(&settings).unwrap();
        let parsed: RandomizerSettings = serde_yaml::from_str(&yaml).unwrap();

        assert_eq!(parsed.er_moon, true);
        assert!(parsed.open_dungeons_mm.contains(&MmDungeon::StoneTower));
        assert_eq!(parsed.ganon_boss_key, GanonBossKeyMode::Custom);
    }

    #[test]
    fn test_climb_most_surfaces_off() {
        let mut settings = RandomizerSettings::new();
        settings.climb_most_surfaces_oot = ClimbMostSurfacesState::Off;

        assert!(settings.check_setting_value("climbMostSurfacesOot", "off"));
        assert!(!settings.check_setting_value("climbMostSurfacesOot", "on"));
    }

    #[test]
    fn test_hookshot_anywhere_off() {
        let mut settings = RandomizerSettings::new();
        settings.hookshot_anywhere_oot = HookshotAnywhereState::Off;

        assert!(settings.check_setting_value("hookshotAnywhereOot", "off"));
        assert!(!settings.check_setting_value("hookshotAnywhereOot", "on"));
    }

    #[test]
    fn test_beneath_well_open() {
        let mut settings = RandomizerSettings::new();
        settings.beneath_well = BeneathWellState::Open;

        assert!(settings.check_setting_value("beneathWell", "open"));
        assert!(!settings.check_setting_value("beneathWell", "vanilla"));
    }

    #[test]
    fn test_small_key_shuffle_anywhere() {
        let mut settings = RandomizerSettings::new();
        settings.small_key_shuffle_oot = SmallKeyShuffleOot::Anywhere;

        assert!(settings.check_setting_value("smallKeyShuffleOot", "anywhere"));
        assert!(!settings.check_setting_value("smallKeyShuffleOot", "vanilla"));
    }

    // === Master Quest Dungeon Tests ===

    #[test]
    fn test_mq_dungeon_default() {
        let settings = RandomizerSettings::default();
        assert!(settings.mq_dungeons.is_empty());
        assert_eq!(settings.mq_dungeon_count(), 0);
    }

    #[test]
    fn test_mq_dungeon_set_and_check() {
        let mut settings = RandomizerSettings::new();

        assert!(!settings.is_dungeon_mq(MqDungeon::DekuTree));
        settings.set_dungeon_mq(MqDungeon::DekuTree);
        assert!(settings.is_dungeon_mq(MqDungeon::DekuTree));
        assert!(!settings.is_dungeon_mq(MqDungeon::DodongosCavern));

        settings.set_dungeon_vanilla(MqDungeon::DekuTree);
        assert!(!settings.is_dungeon_mq(MqDungeon::DekuTree));
    }

    #[test]
    fn test_mq_dungeon_check_by_name() {
        let mut settings = RandomizerSettings::new();
        settings.set_dungeon_mq(MqDungeon::ForestTemple);

        assert!(settings.is_dungeon_mq_by_name("forest_temple"));
        assert!(settings.is_dungeon_mq_by_name("ForestTemple"));
        assert!(!settings.is_dungeon_mq_by_name("fire_temple"));
        assert!(!settings.is_dungeon_mq_by_name("invalid"));
    }

    #[test]
    fn test_mq_dungeon_set_all() {
        let mut settings = RandomizerSettings::new();

        settings.set_all_dungeons_mq();
        assert_eq!(settings.mq_dungeon_count(), 12);
        assert!(settings.is_dungeon_mq(MqDungeon::DekuTree));
        assert!(settings.is_dungeon_mq(MqDungeon::GanonsCastle));

        settings.set_all_dungeons_vanilla();
        assert_eq!(settings.mq_dungeon_count(), 0);
        assert!(!settings.is_dungeon_mq(MqDungeon::DekuTree));
    }

    #[test]
    fn test_mq_dungeon_location_prefix() {
        let mut settings = RandomizerSettings::new();

        // Vanilla prefix by default
        assert_eq!(
            settings.get_dungeon_location_prefix(MqDungeon::DekuTree),
            "oot_deku_tree_"
        );

        // MQ prefix when set
        settings.set_dungeon_mq(MqDungeon::DekuTree);
        assert_eq!(
            settings.get_dungeon_location_prefix(MqDungeon::DekuTree),
            "mq_oot_mq_deku_tree_"
        );
    }

    #[test]
    fn test_mq_dungeon_location_active() {
        let mut settings = RandomizerSettings::new();

        // By default, vanilla locations are active
        assert!(settings.is_location_active("oot_deku_tree_compass_chest"));
        assert!(!settings.is_location_active("mq_oot_mq_deku_tree_compass_chest"));

        // When dungeon is MQ, MQ locations are active
        settings.set_dungeon_mq(MqDungeon::DekuTree);
        assert!(!settings.is_location_active("oot_deku_tree_compass_chest"));
        assert!(settings.is_location_active("mq_oot_mq_deku_tree_compass_chest"));

        // Non-dungeon locations are always active
        assert!(settings.is_location_active("oot_kokiri_forest_sword"));
    }

    #[test]
    fn test_mq_dungeon_check_setting_value() {
        let mut settings = RandomizerSettings::new();
        settings.set_dungeon_mq(MqDungeon::WaterTemple);
        settings.set_dungeon_mq(MqDungeon::ShadowTemple);

        assert!(settings.check_setting_value("mqDungeons", "water_temple"));
        assert!(settings.check_setting_value("mqDungeons", "shadow_temple"));
        assert!(!settings.check_setting_value("mqDungeons", "fire_temple"));
        assert!(!settings.check_setting_value("mqDungeons", "invalid"));
    }

    #[test]
    fn test_mq_dungeon_parse() {
        assert_eq!(MqDungeon::parse("deku_tree"), Some(MqDungeon::DekuTree));
        assert_eq!(MqDungeon::parse("DekuTree"), Some(MqDungeon::DekuTree));
        assert_eq!(
            MqDungeon::parse("dodongos_cavern"),
            Some(MqDungeon::DodongosCavern)
        );
        assert_eq!(
            MqDungeon::parse("gerudo_training_ground"),
            Some(MqDungeon::GerudoTrainingGround)
        );
        assert_eq!(MqDungeon::parse("invalid"), None);
    }

    #[test]
    fn test_mq_dungeon_from_location_id() {
        // Vanilla locations
        assert_eq!(
            MqDungeon::from_location_id("oot_deku_tree_compass_chest"),
            Some(MqDungeon::DekuTree)
        );
        assert_eq!(
            MqDungeon::from_location_id("oot_fire_temple_boss_key"),
            Some(MqDungeon::FireTemple)
        );
        assert_eq!(
            MqDungeon::from_location_id("oot_ganon_castle_light_trial"),
            Some(MqDungeon::GanonsCastle)
        );

        // MQ locations
        assert_eq!(
            MqDungeon::from_location_id("mq_oot_mq_deku_tree_compass_chest"),
            Some(MqDungeon::DekuTree)
        );
        assert_eq!(
            MqDungeon::from_location_id("mq_oot_dodongo_cavern_entrance"),
            Some(MqDungeon::DodongosCavern)
        );

        // Non-dungeon locations
        assert_eq!(MqDungeon::from_location_id("oot_kokiri_forest_sword"), None);
        assert_eq!(MqDungeon::from_location_id("mm_clock_town_chest"), None);
    }

    #[test]
    fn test_mq_dungeon_serde_roundtrip() {
        let mut settings = RandomizerSettings::new();
        settings.set_dungeon_mq(MqDungeon::ForestTemple);
        settings.set_dungeon_mq(MqDungeon::SpiritTemple);

        let json = serde_json::to_string(&settings).unwrap();
        let parsed: RandomizerSettings = serde_json::from_str(&json).unwrap();

        assert!(parsed.is_dungeon_mq(MqDungeon::ForestTemple));
        assert!(parsed.is_dungeon_mq(MqDungeon::SpiritTemple));
        assert!(!parsed.is_dungeon_mq(MqDungeon::ShadowTemple));
        assert_eq!(parsed.mq_dungeon_count(), 2);
    }

    #[test]
    fn test_mq_dungeon_all() {
        let all = MqDungeon::all();
        assert_eq!(all.len(), 12);
        assert!(all.contains(&MqDungeon::DekuTree));
        assert!(all.contains(&MqDungeon::GanonsCastle));
    }

    // === Bottle Count Tests ===

    #[test]
    fn test_bottle_count_default() {
        let settings = RandomizerSettings::default();
        assert_eq!(settings.bottle_count, 4);
        assert_eq!(settings.get_bottle_count(), 4);
    }

    #[test]
    fn test_bottle_count_set_and_get() {
        let mut settings = RandomizerSettings::new();

        settings.set_bottle_count(3);
        assert_eq!(settings.get_bottle_count(), 3);

        settings.set_bottle_count(1);
        assert_eq!(settings.get_bottle_count(), 1);
    }

    #[test]
    fn test_bottle_count_clamping() {
        let mut settings = RandomizerSettings::new();

        // Test upper bound clamping
        settings.set_bottle_count(10);
        assert_eq!(settings.get_bottle_count(), 4);

        // Test lower bound clamping
        settings.set_bottle_count(0);
        assert_eq!(settings.get_bottle_count(), 1);
    }

    #[test]
    fn test_bottle_count_serde_roundtrip() {
        let mut settings = RandomizerSettings::new();
        settings.set_bottle_count(2);

        let json = serde_json::to_string(&settings).unwrap();
        let parsed: RandomizerSettings = serde_json::from_str(&json).unwrap();

        assert_eq!(parsed.get_bottle_count(), 2);
    }

    #[test]
    fn test_bottle_count_defaults_in_deserialization() {
        // Test that missing bottle_count in JSON defaults to 4
        let json = r#"{"agelessBoots": true}"#;
        let parsed: RandomizerSettings = serde_json::from_str(json).unwrap();

        assert_eq!(parsed.get_bottle_count(), 4);
    }

    // === Rainbow Bridge Mode Tests ===

    #[test]
    fn test_rainbow_bridge_mode_default() {
        let mode = RainbowBridgeMode::default();
        assert_eq!(mode, RainbowBridgeMode::Vanilla);
    }

    #[test]
    fn test_rainbow_bridge_mode_as_str() {
        assert_eq!(RainbowBridgeMode::Vanilla.as_str(), "vanilla");
        assert_eq!(RainbowBridgeMode::Open.as_str(), "open");
        assert_eq!(RainbowBridgeMode::Medallions.as_str(), "medallions");
        assert_eq!(RainbowBridgeMode::Stones.as_str(), "stones");
        assert_eq!(RainbowBridgeMode::DungeonRewards.as_str(), "dungeonRewards");
        assert_eq!(RainbowBridgeMode::Skulltulas.as_str(), "skulltulas");
        assert_eq!(RainbowBridgeMode::Remains.as_str(), "remains");
        assert_eq!(RainbowBridgeMode::Custom.as_str(), "custom");
    }

    #[test]
    fn test_rainbow_bridge_mode_parse() {
        assert_eq!(
            RainbowBridgeMode::parse("vanilla"),
            Some(RainbowBridgeMode::Vanilla)
        );
        assert_eq!(
            RainbowBridgeMode::parse("open"),
            Some(RainbowBridgeMode::Open)
        );
        assert_eq!(
            RainbowBridgeMode::parse("medallions"),
            Some(RainbowBridgeMode::Medallions)
        );
        assert_eq!(
            RainbowBridgeMode::parse("stones"),
            Some(RainbowBridgeMode::Stones)
        );
        assert_eq!(
            RainbowBridgeMode::parse("dungeonRewards"),
            Some(RainbowBridgeMode::DungeonRewards)
        );
        assert_eq!(
            RainbowBridgeMode::parse("skulltulas"),
            Some(RainbowBridgeMode::Skulltulas)
        );
        assert_eq!(
            RainbowBridgeMode::parse("remains"),
            Some(RainbowBridgeMode::Remains)
        );
        assert_eq!(
            RainbowBridgeMode::parse("custom"),
            Some(RainbowBridgeMode::Custom)
        );
        assert_eq!(RainbowBridgeMode::parse("invalid"), None);
    }

    #[test]
    fn test_rainbow_bridge_mode_roundtrip() {
        for mode in [
            RainbowBridgeMode::Vanilla,
            RainbowBridgeMode::Open,
            RainbowBridgeMode::Medallions,
            RainbowBridgeMode::Stones,
            RainbowBridgeMode::DungeonRewards,
            RainbowBridgeMode::Skulltulas,
            RainbowBridgeMode::Remains,
            RainbowBridgeMode::Custom,
        ] {
            let s = mode.as_str();
            let parsed = RainbowBridgeMode::parse(s);
            assert_eq!(parsed, Some(mode));
        }
    }

    // === Songs Mode Tests ===

    #[test]
    fn test_songs_mode_default() {
        let mode = SongsMode::default();
        assert_eq!(mode, SongsMode::SongsOnly);
    }

    #[test]
    fn test_songs_mode_as_str() {
        assert_eq!(SongsMode::SongsOnly.as_str(), "songsOnly");
        assert_eq!(SongsMode::Anywhere.as_str(), "anywhere");
        assert_eq!(SongsMode::DungeonRewards.as_str(), "dungeonRewards");
    }

    #[test]
    fn test_songs_mode_parse() {
        assert_eq!(SongsMode::parse("songsOnly"), Some(SongsMode::SongsOnly));
        assert_eq!(SongsMode::parse("anywhere"), Some(SongsMode::Anywhere));
        assert_eq!(
            SongsMode::parse("dungeonRewards"),
            Some(SongsMode::DungeonRewards)
        );
        assert_eq!(SongsMode::parse("invalid"), None);
    }

    #[test]
    fn test_songs_mode_roundtrip() {
        for mode in [
            SongsMode::SongsOnly,
            SongsMode::Anywhere,
            SongsMode::DungeonRewards,
        ] {
            let s = mode.as_str();
            let parsed = SongsMode::parse(s);
            assert_eq!(parsed, Some(mode));
        }
    }

    // === Dungeon Reward Shuffle Tests ===

    #[test]
    fn test_dungeon_reward_shuffle_default() {
        let mode = DungeonRewardShuffle::default();
        assert_eq!(mode, DungeonRewardShuffle::Vanilla);
    }

    #[test]
    fn test_dungeon_reward_shuffle_as_str() {
        assert_eq!(DungeonRewardShuffle::Vanilla.as_str(), "vanilla");
        assert_eq!(
            DungeonRewardShuffle::DungeonBlueWarps.as_str(),
            "dungeonBlueWarps"
        );
        assert_eq!(DungeonRewardShuffle::Anywhere.as_str(), "anywhere");
    }

    #[test]
    fn test_dungeon_reward_shuffle_parse() {
        assert_eq!(
            DungeonRewardShuffle::parse("vanilla"),
            Some(DungeonRewardShuffle::Vanilla)
        );
        assert_eq!(
            DungeonRewardShuffle::parse("dungeonBlueWarps"),
            Some(DungeonRewardShuffle::DungeonBlueWarps)
        );
        assert_eq!(
            DungeonRewardShuffle::parse("anywhere"),
            Some(DungeonRewardShuffle::Anywhere)
        );
        assert_eq!(DungeonRewardShuffle::parse("invalid"), None);
    }

    #[test]
    fn test_dungeon_reward_shuffle_roundtrip() {
        for mode in [
            DungeonRewardShuffle::Vanilla,
            DungeonRewardShuffle::DungeonBlueWarps,
            DungeonRewardShuffle::Anywhere,
        ] {
            let s = mode.as_str();
            let parsed = DungeonRewardShuffle::parse(s);
            assert_eq!(parsed, Some(mode));
        }
    }

    // === ShuffleMode Tests ===

    #[test]
    fn test_shuffle_mode_default() {
        let mode = ShuffleMode::default();
        assert_eq!(mode, ShuffleMode::None);
    }

    #[test]
    fn test_shuffle_mode_as_str() {
        assert_eq!(ShuffleMode::None.as_str(), "none");
        assert_eq!(ShuffleMode::Overworld.as_str(), "overworld");
        assert_eq!(ShuffleMode::Dungeon.as_str(), "dungeon");
        assert_eq!(ShuffleMode::All.as_str(), "all");
    }

    #[test]
    fn test_shuffle_mode_parse() {
        assert_eq!(ShuffleMode::parse("none"), Some(ShuffleMode::None));
        assert_eq!(
            ShuffleMode::parse("overworld"),
            Some(ShuffleMode::Overworld)
        );
        assert_eq!(ShuffleMode::parse("dungeon"), Some(ShuffleMode::Dungeon));
        assert_eq!(ShuffleMode::parse("all"), Some(ShuffleMode::All));
        assert_eq!(ShuffleMode::parse("invalid"), None);
    }

    #[test]
    fn test_shuffle_mode_is_shuffled() {
        assert!(!ShuffleMode::None.is_shuffled());
        assert!(ShuffleMode::Overworld.is_shuffled());
        assert!(ShuffleMode::Dungeon.is_shuffled());
        assert!(ShuffleMode::All.is_shuffled());
    }

    #[test]
    fn test_shuffle_mode_serde_roundtrip() {
        let mode = ShuffleMode::Overworld;
        let json = serde_json::to_string(&mode).unwrap();
        let parsed: ShuffleMode = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed, mode);
    }

    // === TingleShuffle Tests ===

    #[test]
    fn test_tingle_shuffle_default() {
        let mode = TingleShuffle::default();
        assert_eq!(mode, TingleShuffle::Vanilla);
    }

    #[test]
    fn test_tingle_shuffle_as_str() {
        assert_eq!(TingleShuffle::Vanilla.as_str(), "vanilla");
        assert_eq!(TingleShuffle::Starting.as_str(), "starting");
        assert_eq!(TingleShuffle::Removed.as_str(), "removed");
        assert_eq!(TingleShuffle::Anywhere.as_str(), "anywhere");
        assert_eq!(TingleShuffle::OwnRegion.as_str(), "ownRegion");
    }

    #[test]
    fn test_tingle_shuffle_parse() {
        assert_eq!(
            TingleShuffle::parse("vanilla"),
            Some(TingleShuffle::Vanilla)
        );
        assert_eq!(
            TingleShuffle::parse("starting"),
            Some(TingleShuffle::Starting)
        );
        assert_eq!(
            TingleShuffle::parse("removed"),
            Some(TingleShuffle::Removed)
        );
        assert_eq!(
            TingleShuffle::parse("anywhere"),
            Some(TingleShuffle::Anywhere)
        );
        assert_eq!(
            TingleShuffle::parse("ownRegion"),
            Some(TingleShuffle::OwnRegion)
        );
        assert_eq!(TingleShuffle::parse("invalid"), None);
    }

    #[test]
    fn test_tingle_shuffle_serde_roundtrip() {
        let mode = TingleShuffle::OwnRegion;
        let json = serde_json::to_string(&mode).unwrap();
        let parsed: TingleShuffle = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed, mode);
    }

    // === OwlShuffle Tests ===

    #[test]
    fn test_owl_shuffle_default() {
        let mode = OwlShuffle::default();
        assert_eq!(mode, OwlShuffle::None);
    }

    #[test]
    fn test_owl_shuffle_as_str() {
        assert_eq!(OwlShuffle::None.as_str(), "none");
        assert_eq!(OwlShuffle::Anywhere.as_str(), "anywhere");
    }

    #[test]
    fn test_owl_shuffle_parse() {
        assert_eq!(OwlShuffle::parse("none"), Some(OwlShuffle::None));
        assert_eq!(OwlShuffle::parse("anywhere"), Some(OwlShuffle::Anywhere));
        assert_eq!(OwlShuffle::parse("invalid"), None);
    }

    #[test]
    fn test_owl_shuffle_serde_roundtrip() {
        let mode = OwlShuffle::Anywhere;
        let json = serde_json::to_string(&mode).unwrap();
        let parsed: OwlShuffle = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed, mode);
    }

    // === SkulltulaTokenShuffle Tests ===

    #[test]
    fn test_skulltula_token_shuffle_default() {
        let mode = SkulltulaTokenShuffle::default();
        assert_eq!(mode, SkulltulaTokenShuffle::None);
    }

    #[test]
    fn test_skulltula_token_shuffle_as_str() {
        assert_eq!(SkulltulaTokenShuffle::None.as_str(), "none");
        assert_eq!(SkulltulaTokenShuffle::Dungeons.as_str(), "dungeons");
        assert_eq!(SkulltulaTokenShuffle::Overworld.as_str(), "overworld");
        assert_eq!(SkulltulaTokenShuffle::All.as_str(), "all");
    }

    #[test]
    fn test_skulltula_token_shuffle_parse() {
        assert_eq!(
            SkulltulaTokenShuffle::parse("none"),
            Some(SkulltulaTokenShuffle::None)
        );
        assert_eq!(
            SkulltulaTokenShuffle::parse("dungeons"),
            Some(SkulltulaTokenShuffle::Dungeons)
        );
        assert_eq!(
            SkulltulaTokenShuffle::parse("overworld"),
            Some(SkulltulaTokenShuffle::Overworld)
        );
        assert_eq!(
            SkulltulaTokenShuffle::parse("all"),
            Some(SkulltulaTokenShuffle::All)
        );
        assert_eq!(SkulltulaTokenShuffle::parse("invalid"), None);
    }

    #[test]
    fn test_skulltula_token_shuffle_serde_roundtrip() {
        let mode = SkulltulaTokenShuffle::Dungeons;
        let json = serde_json::to_string(&mode).unwrap();
        let parsed: SkulltulaTokenShuffle = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed, mode);
    }

    // === KeyShuffle Tests ===

    #[test]
    fn test_key_shuffle_default() {
        let mode = KeyShuffle::default();
        assert_eq!(mode, KeyShuffle::Vanilla);
    }

    #[test]
    fn test_key_shuffle_as_str() {
        assert_eq!(KeyShuffle::Vanilla.as_str(), "vanilla");
        assert_eq!(KeyShuffle::OwnDungeon.as_str(), "ownDungeon");
        assert_eq!(KeyShuffle::Anywhere.as_str(), "anywhere");
        assert_eq!(KeyShuffle::Removed.as_str(), "removed");
    }

    #[test]
    fn test_key_shuffle_parse() {
        assert_eq!(KeyShuffle::parse("vanilla"), Some(KeyShuffle::Vanilla));
        assert_eq!(
            KeyShuffle::parse("ownDungeon"),
            Some(KeyShuffle::OwnDungeon)
        );
        assert_eq!(KeyShuffle::parse("anywhere"), Some(KeyShuffle::Anywhere));
        assert_eq!(KeyShuffle::parse("removed"), Some(KeyShuffle::Removed));
        assert_eq!(KeyShuffle::parse("invalid"), None);
    }

    #[test]
    fn test_key_shuffle_serde_roundtrip() {
        let mode = KeyShuffle::OwnDungeon;
        let json = serde_json::to_string(&mode).unwrap();
        let parsed: KeyShuffle = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed, mode);
    }

    #[test]
    fn test_key_shuffle_is_shuffled() {
        assert!(!KeyShuffle::Vanilla.is_shuffled());
        assert!(KeyShuffle::OwnDungeon.is_shuffled());
        assert!(KeyShuffle::Anywhere.is_shuffled());
        assert!(!KeyShuffle::Removed.is_shuffled());
    }

    // === MapCompassShuffle Tests ===

    #[test]
    fn test_map_compass_shuffle_default() {
        let mode = MapCompassShuffle::default();
        assert_eq!(mode, MapCompassShuffle::Vanilla);
    }

    #[test]
    fn test_map_compass_shuffle_as_str() {
        assert_eq!(MapCompassShuffle::Vanilla.as_str(), "vanilla");
        assert_eq!(MapCompassShuffle::Starting.as_str(), "starting");
        assert_eq!(MapCompassShuffle::OwnDungeon.as_str(), "ownDungeon");
        assert_eq!(MapCompassShuffle::Anywhere.as_str(), "anywhere");
        assert_eq!(MapCompassShuffle::Removed.as_str(), "removed");
    }

    #[test]
    fn test_map_compass_shuffle_parse() {
        assert_eq!(
            MapCompassShuffle::parse("vanilla"),
            Some(MapCompassShuffle::Vanilla)
        );
        assert_eq!(
            MapCompassShuffle::parse("starting"),
            Some(MapCompassShuffle::Starting)
        );
        assert_eq!(
            MapCompassShuffle::parse("ownDungeon"),
            Some(MapCompassShuffle::OwnDungeon)
        );
        assert_eq!(
            MapCompassShuffle::parse("anywhere"),
            Some(MapCompassShuffle::Anywhere)
        );
        assert_eq!(
            MapCompassShuffle::parse("removed"),
            Some(MapCompassShuffle::Removed)
        );
        assert_eq!(MapCompassShuffle::parse("invalid"), None);
    }

    #[test]
    fn test_map_compass_shuffle_serde_roundtrip() {
        let mode = MapCompassShuffle::OwnDungeon;
        let json = serde_json::to_string(&mode).unwrap();
        let parsed: MapCompassShuffle = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed, mode);
    }

    #[test]
    fn test_map_compass_shuffle_is_shuffled() {
        assert!(!MapCompassShuffle::Vanilla.is_shuffled());
        assert!(!MapCompassShuffle::Starting.is_shuffled());
        assert!(MapCompassShuffle::OwnDungeon.is_shuffled());
        assert!(MapCompassShuffle::Anywhere.is_shuffled());
        assert!(!MapCompassShuffle::Removed.is_shuffled());
    }

    // === DekuTreeState Tests ===

    #[test]
    fn test_deku_tree_state_default() {
        let state = DekuTreeState::default();
        assert_eq!(state, DekuTreeState::Closed);
    }

    #[test]
    fn test_deku_tree_state_as_str() {
        assert_eq!(DekuTreeState::Closed.as_str(), "closed");
        assert_eq!(DekuTreeState::Open.as_str(), "open");
        assert_eq!(DekuTreeState::Vanilla.as_str(), "vanilla");
    }

    #[test]
    fn test_deku_tree_state_parse() {
        assert_eq!(DekuTreeState::parse("closed"), Some(DekuTreeState::Closed));
        assert_eq!(DekuTreeState::parse("open"), Some(DekuTreeState::Open));
        assert_eq!(
            DekuTreeState::parse("vanilla"),
            Some(DekuTreeState::Vanilla)
        );
        assert_eq!(DekuTreeState::parse("invalid"), None);
    }

    #[test]
    fn test_deku_tree_state_roundtrip() {
        for state in [
            DekuTreeState::Closed,
            DekuTreeState::Open,
            DekuTreeState::Vanilla,
        ] {
            let s = state.as_str();
            assert_eq!(DekuTreeState::parse(s), Some(state));
        }
    }

    #[test]
    fn test_deku_tree_state_serde_roundtrip() {
        let state = DekuTreeState::Open;
        let json = serde_json::to_string(&state).unwrap();
        let parsed: DekuTreeState = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed, state);
    }

    // === DoorOfTimeState Tests ===

    #[test]
    fn test_door_of_time_state_default() {
        let state = DoorOfTimeState::default();
        assert_eq!(state, DoorOfTimeState::Closed);
    }

    #[test]
    fn test_door_of_time_state_as_str() {
        assert_eq!(DoorOfTimeState::Closed.as_str(), "closed");
        assert_eq!(DoorOfTimeState::Open.as_str(), "open");
    }

    #[test]
    fn test_door_of_time_state_parse() {
        assert_eq!(
            DoorOfTimeState::parse("closed"),
            Some(DoorOfTimeState::Closed)
        );
        assert_eq!(DoorOfTimeState::parse("open"), Some(DoorOfTimeState::Open));
        assert_eq!(DoorOfTimeState::parse("invalid"), None);
    }

    #[test]
    fn test_door_of_time_state_serde_roundtrip() {
        let state = DoorOfTimeState::Open;
        let json = serde_json::to_string(&state).unwrap();
        let parsed: DoorOfTimeState = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed, state);
    }

    // === KakarikoGateState Tests ===

    #[test]
    fn test_kakariko_gate_state_default() {
        let state = KakarikoGateState::default();
        assert_eq!(state, KakarikoGateState::Closed);
    }

    #[test]
    fn test_kakariko_gate_state_as_str() {
        assert_eq!(KakarikoGateState::Closed.as_str(), "closed");
        assert_eq!(KakarikoGateState::Open.as_str(), "open");
    }

    #[test]
    fn test_kakariko_gate_state_parse() {
        assert_eq!(
            KakarikoGateState::parse("closed"),
            Some(KakarikoGateState::Closed)
        );
        assert_eq!(
            KakarikoGateState::parse("open"),
            Some(KakarikoGateState::Open)
        );
        assert_eq!(KakarikoGateState::parse("invalid"), None);
    }

    #[test]
    fn test_kakariko_gate_state_serde_roundtrip() {
        let state = KakarikoGateState::Open;
        let json = serde_json::to_string(&state).unwrap();
        let parsed: KakarikoGateState = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed, state);
    }

    // === GanonBossKeyMode Tests ===

    #[test]
    fn test_ganon_boss_key_mode_default() {
        let mode = GanonBossKeyMode::default();
        assert_eq!(mode, GanonBossKeyMode::Vanilla);
    }

    #[test]
    fn test_ganon_boss_key_mode_as_str() {
        assert_eq!(GanonBossKeyMode::Vanilla.as_str(), "vanilla");
        assert_eq!(GanonBossKeyMode::Removed.as_str(), "removed");
        assert_eq!(GanonBossKeyMode::Custom.as_str(), "custom");
    }

    #[test]
    fn test_ganon_boss_key_mode_parse() {
        assert_eq!(
            GanonBossKeyMode::parse("vanilla"),
            Some(GanonBossKeyMode::Vanilla)
        );
        assert_eq!(
            GanonBossKeyMode::parse("removed"),
            Some(GanonBossKeyMode::Removed)
        );
        assert_eq!(
            GanonBossKeyMode::parse("custom"),
            Some(GanonBossKeyMode::Custom)
        );
        assert_eq!(GanonBossKeyMode::parse("invalid"), None);
    }

    #[test]
    fn test_ganon_boss_key_mode_serde_roundtrip() {
        for mode in [
            GanonBossKeyMode::Vanilla,
            GanonBossKeyMode::Removed,
            GanonBossKeyMode::Custom,
        ] {
            let json = serde_json::to_string(&mode).unwrap();
            let parsed: GanonBossKeyMode = serde_json::from_str(&json).unwrap();
            assert_eq!(parsed, mode);
        }
    }

    // === LacsMode Tests ===

    #[test]
    fn test_lacs_mode_default() {
        let mode = LacsMode::default();
        assert_eq!(mode, LacsMode::Vanilla);
    }

    #[test]
    fn test_lacs_mode_as_str() {
        assert_eq!(LacsMode::Vanilla.as_str(), "vanilla");
        assert_eq!(LacsMode::Custom.as_str(), "custom");
    }

    #[test]
    fn test_lacs_mode_parse() {
        assert_eq!(LacsMode::parse("vanilla"), Some(LacsMode::Vanilla));
        assert_eq!(LacsMode::parse("custom"), Some(LacsMode::Custom));
        assert_eq!(LacsMode::parse("invalid"), None);
    }

    #[test]
    fn test_lacs_mode_serde_roundtrip() {
        for mode in [LacsMode::Vanilla, LacsMode::Custom] {
            let json = serde_json::to_string(&mode).unwrap();
            let parsed: LacsMode = serde_json::from_str(&json).unwrap();
            assert_eq!(parsed, mode);
        }
    }

    // === MajoraChildMode Tests ===

    #[test]
    fn test_majora_child_mode_default() {
        let mode = MajoraChildMode::default();
        assert_eq!(mode, MajoraChildMode::Vanilla);
    }

    #[test]
    fn test_majora_child_mode_as_str() {
        assert_eq!(MajoraChildMode::Vanilla.as_str(), "vanilla");
        assert_eq!(MajoraChildMode::None.as_str(), "none");
        assert_eq!(MajoraChildMode::Custom.as_str(), "custom");
    }

    #[test]
    fn test_majora_child_mode_parse() {
        assert_eq!(
            MajoraChildMode::parse("vanilla"),
            Some(MajoraChildMode::Vanilla)
        );
        assert_eq!(MajoraChildMode::parse("none"), Some(MajoraChildMode::None));
        assert_eq!(
            MajoraChildMode::parse("custom"),
            Some(MajoraChildMode::Custom)
        );
        assert_eq!(MajoraChildMode::parse("invalid"), None);
    }

    #[test]
    fn test_majora_child_mode_serde_roundtrip() {
        for mode in [
            MajoraChildMode::Vanilla,
            MajoraChildMode::None,
            MajoraChildMode::Custom,
        ] {
            let json = serde_json::to_string(&mode).unwrap();
            let parsed: MajoraChildMode = serde_json::from_str(&json).unwrap();
            assert_eq!(parsed, mode);
        }
    }

    // === MoonCrashMode Tests ===

    #[test]
    fn test_moon_crash_mode_default() {
        let mode = MoonCrashMode::default();
        assert_eq!(mode, MoonCrashMode::Vanilla);
    }

    #[test]
    fn test_moon_crash_mode_as_str() {
        assert_eq!(MoonCrashMode::Vanilla.as_str(), "vanilla");
        assert_eq!(MoonCrashMode::Cycle.as_str(), "cycle");
    }

    #[test]
    fn test_moon_crash_mode_parse() {
        assert_eq!(
            MoonCrashMode::parse("vanilla"),
            Some(MoonCrashMode::Vanilla)
        );
        assert_eq!(MoonCrashMode::parse("cycle"), Some(MoonCrashMode::Cycle));
        assert_eq!(MoonCrashMode::parse("invalid"), None);
    }

    #[test]
    fn test_moon_crash_mode_serde_roundtrip() {
        for mode in [MoonCrashMode::Vanilla, MoonCrashMode::Cycle] {
            let json = serde_json::to_string(&mode).unwrap();
            let parsed: MoonCrashMode = serde_json::from_str(&json).unwrap();
            assert_eq!(parsed, mode);
        }
    }

    // === AgeChangeMode Tests ===

    #[test]
    fn test_age_change_mode_default() {
        let mode = AgeChangeMode::default();
        assert_eq!(mode, AgeChangeMode::TempleOfTime);
    }

    #[test]
    fn test_age_change_mode_as_str() {
        assert_eq!(AgeChangeMode::TempleOfTime.as_str(), "templeOfTime");
        assert_eq!(AgeChangeMode::None.as_str(), "none");
        assert_eq!(AgeChangeMode::Oot.as_str(), "oot");
    }

    #[test]
    fn test_age_change_mode_parse() {
        assert_eq!(
            AgeChangeMode::parse("templeOfTime"),
            Some(AgeChangeMode::TempleOfTime)
        );
        assert_eq!(AgeChangeMode::parse("none"), Some(AgeChangeMode::None));
        assert_eq!(AgeChangeMode::parse("oot"), Some(AgeChangeMode::Oot));
        assert_eq!(AgeChangeMode::parse("invalid"), None);
    }

    #[test]
    fn test_age_change_mode_serde_roundtrip() {
        for mode in [
            AgeChangeMode::TempleOfTime,
            AgeChangeMode::None,
            AgeChangeMode::Oot,
        ] {
            let json = serde_json::to_string(&mode).unwrap();
            let parsed: AgeChangeMode = serde_json::from_str(&json).unwrap();
            assert_eq!(parsed, mode);
        }
    }

    // === ClimbMostSurfacesState Tests ===

    #[test]
    fn test_climb_most_surfaces_state_default() {
        let state = ClimbMostSurfacesState::default();
        assert_eq!(state, ClimbMostSurfacesState::On);
    }

    #[test]
    fn test_climb_most_surfaces_state_as_str() {
        assert_eq!(ClimbMostSurfacesState::On.as_str(), "on");
        assert_eq!(ClimbMostSurfacesState::Off.as_str(), "off");
    }

    #[test]
    fn test_climb_most_surfaces_state_parse() {
        assert_eq!(
            ClimbMostSurfacesState::parse("on"),
            Some(ClimbMostSurfacesState::On)
        );
        assert_eq!(
            ClimbMostSurfacesState::parse("off"),
            Some(ClimbMostSurfacesState::Off)
        );
        assert_eq!(ClimbMostSurfacesState::parse("invalid"), None);
    }

    #[test]
    fn test_climb_most_surfaces_state_serde_roundtrip() {
        for state in [ClimbMostSurfacesState::On, ClimbMostSurfacesState::Off] {
            let json = serde_json::to_string(&state).unwrap();
            let parsed: ClimbMostSurfacesState = serde_json::from_str(&json).unwrap();
            assert_eq!(parsed, state);
        }
    }

    // === HookshotAnywhereState Tests ===

    #[test]
    fn test_hookshot_anywhere_state_default() {
        let state = HookshotAnywhereState::default();
        assert_eq!(state, HookshotAnywhereState::On);
    }

    #[test]
    fn test_hookshot_anywhere_state_as_str() {
        assert_eq!(HookshotAnywhereState::On.as_str(), "on");
        assert_eq!(HookshotAnywhereState::Off.as_str(), "off");
    }

    #[test]
    fn test_hookshot_anywhere_state_parse() {
        assert_eq!(
            HookshotAnywhereState::parse("on"),
            Some(HookshotAnywhereState::On)
        );
        assert_eq!(
            HookshotAnywhereState::parse("off"),
            Some(HookshotAnywhereState::Off)
        );
        assert_eq!(HookshotAnywhereState::parse("invalid"), None);
    }

    #[test]
    fn test_hookshot_anywhere_state_serde_roundtrip() {
        for state in [HookshotAnywhereState::On, HookshotAnywhereState::Off] {
            let json = serde_json::to_string(&state).unwrap();
            let parsed: HookshotAnywhereState = serde_json::from_str(&json).unwrap();
            assert_eq!(parsed, state);
        }
    }

    // === BeneathWellState Tests ===

    #[test]
    fn test_beneath_well_state_default() {
        let state = BeneathWellState::default();
        assert_eq!(state, BeneathWellState::Vanilla);
    }

    #[test]
    fn test_beneath_well_state_as_str() {
        assert_eq!(BeneathWellState::Vanilla.as_str(), "vanilla");
        assert_eq!(BeneathWellState::Open.as_str(), "open");
    }

    #[test]
    fn test_beneath_well_state_parse() {
        assert_eq!(
            BeneathWellState::parse("vanilla"),
            Some(BeneathWellState::Vanilla)
        );
        assert_eq!(
            BeneathWellState::parse("open"),
            Some(BeneathWellState::Open)
        );
        assert_eq!(BeneathWellState::parse("invalid"), None);
    }

    #[test]
    fn test_beneath_well_state_serde_roundtrip() {
        for state in [BeneathWellState::Vanilla, BeneathWellState::Open] {
            let json = serde_json::to_string(&state).unwrap();
            let parsed: BeneathWellState = serde_json::from_str(&json).unwrap();
            assert_eq!(parsed, state);
        }
    }

    // === ErOverworldState Tests ===

    #[test]
    fn test_er_overworld_state_default() {
        let state = ErOverworldState::default();
        assert_eq!(state, ErOverworldState::None);
    }

    #[test]
    fn test_er_overworld_state_as_str() {
        assert_eq!(ErOverworldState::None.as_str(), "none");
        assert_eq!(ErOverworldState::Full.as_str(), "full");
    }

    #[test]
    fn test_er_overworld_state_parse() {
        assert_eq!(
            ErOverworldState::parse("none"),
            Some(ErOverworldState::None)
        );
        assert_eq!(
            ErOverworldState::parse("full"),
            Some(ErOverworldState::Full)
        );
        assert_eq!(ErOverworldState::parse("invalid"), None);
    }

    #[test]
    fn test_er_overworld_state_serde_roundtrip() {
        for state in [ErOverworldState::None, ErOverworldState::Full] {
            let json = serde_json::to_string(&state).unwrap();
            let parsed: ErOverworldState = serde_json::from_str(&json).unwrap();
            assert_eq!(parsed, state);
        }
    }

    // === ErGrottosState Tests ===

    #[test]
    fn test_er_grottos_state_default() {
        let state = ErGrottosState::default();
        assert_eq!(state, ErGrottosState::None);
    }

    #[test]
    fn test_er_grottos_state_as_str() {
        assert_eq!(ErGrottosState::None.as_str(), "none");
        assert_eq!(ErGrottosState::Full.as_str(), "full");
    }

    #[test]
    fn test_er_grottos_state_parse() {
        assert_eq!(ErGrottosState::parse("none"), Some(ErGrottosState::None));
        assert_eq!(ErGrottosState::parse("full"), Some(ErGrottosState::Full));
        assert_eq!(ErGrottosState::parse("invalid"), None);
    }

    #[test]
    fn test_er_grottos_state_serde_roundtrip() {
        for state in [ErGrottosState::None, ErGrottosState::Full] {
            let json = serde_json::to_string(&state).unwrap();
            let parsed: ErGrottosState = serde_json::from_str(&json).unwrap();
            assert_eq!(parsed, state);
        }
    }

    // === BossWarpPadsMode Tests ===

    #[test]
    fn test_boss_warp_pads_mode_default() {
        let mode = BossWarpPadsMode::default();
        assert_eq!(mode, BossWarpPadsMode::Vanilla);
    }

    #[test]
    fn test_boss_warp_pads_mode_as_str() {
        assert_eq!(BossWarpPadsMode::Vanilla.as_str(), "vanilla");
        assert_eq!(BossWarpPadsMode::Remains.as_str(), "remains");
    }

    #[test]
    fn test_boss_warp_pads_mode_parse() {
        assert_eq!(
            BossWarpPadsMode::parse("vanilla"),
            Some(BossWarpPadsMode::Vanilla)
        );
        assert_eq!(
            BossWarpPadsMode::parse("remains"),
            Some(BossWarpPadsMode::Remains)
        );
        assert_eq!(BossWarpPadsMode::parse("invalid"), None);
    }

    #[test]
    fn test_boss_warp_pads_mode_serde_roundtrip() {
        for mode in [BossWarpPadsMode::Vanilla, BossWarpPadsMode::Remains] {
            let json = serde_json::to_string(&mode).unwrap();
            let parsed: BossWarpPadsMode = serde_json::from_str(&json).unwrap();
            assert_eq!(parsed, mode);
        }
    }

    // === ClearStateDungeonsMm Tests ===

    #[test]
    fn test_clear_state_dungeons_mm_as_str() {
        assert_eq!(ClearStateDungeonsMm::Woodfall.as_str(), "WF");
        assert_eq!(ClearStateDungeonsMm::Both.as_str(), "both");
    }

    #[test]
    fn test_clear_state_dungeons_mm_parse() {
        assert_eq!(
            ClearStateDungeonsMm::parse("WF"),
            Some(ClearStateDungeonsMm::Woodfall)
        );
        assert_eq!(
            ClearStateDungeonsMm::parse("both"),
            Some(ClearStateDungeonsMm::Both)
        );
        assert_eq!(ClearStateDungeonsMm::parse("invalid"), None);
    }

    #[test]
    fn test_clear_state_dungeons_mm_serde_roundtrip() {
        for state in [ClearStateDungeonsMm::Woodfall, ClearStateDungeonsMm::Both] {
            let json = serde_json::to_string(&state).unwrap();
            let parsed: ClearStateDungeonsMm = serde_json::from_str(&json).unwrap();
            assert_eq!(parsed, state);
        }
    }

    // === JpLayout Tests ===

    #[test]
    fn test_jp_layout_as_str() {
        assert_eq!(JpLayout::GreatBayCoast.as_str(), "GreatBayCoast");
        assert_eq!(JpLayout::StoneTowerEntrance.as_str(), "ST");
        assert_eq!(JpLayout::StoneTower.as_str(), "StoneTower");
    }

    #[test]
    fn test_jp_layout_parse() {
        assert_eq!(
            JpLayout::parse("GreatBayCoast"),
            Some(JpLayout::GreatBayCoast)
        );
        assert_eq!(JpLayout::parse("ST"), Some(JpLayout::StoneTowerEntrance));
        assert_eq!(JpLayout::parse("StoneTower"), Some(JpLayout::StoneTower));
        assert_eq!(JpLayout::parse("invalid"), None);
    }

    #[test]
    fn test_jp_layout_serde_roundtrip() {
        for layout in [
            JpLayout::GreatBayCoast,
            JpLayout::StoneTowerEntrance,
            JpLayout::StoneTower,
        ] {
            let json = serde_json::to_string(&layout).unwrap();
            let parsed: JpLayout = serde_json::from_str(&json).unwrap();
            assert_eq!(parsed, layout);
        }
    }

    // === SmallKeyShuffleOot Tests ===

    #[test]
    fn test_small_key_shuffle_oot_default() {
        let mode = SmallKeyShuffleOot::default();
        assert_eq!(mode, SmallKeyShuffleOot::Vanilla);
    }

    #[test]
    fn test_small_key_shuffle_oot_as_str() {
        assert_eq!(SmallKeyShuffleOot::Vanilla.as_str(), "vanilla");
        assert_eq!(SmallKeyShuffleOot::Dungeon.as_str(), "dungeon");
        assert_eq!(SmallKeyShuffleOot::Anywhere.as_str(), "anywhere");
    }

    #[test]
    fn test_small_key_shuffle_oot_parse() {
        assert_eq!(
            SmallKeyShuffleOot::parse("vanilla"),
            Some(SmallKeyShuffleOot::Vanilla)
        );
        assert_eq!(
            SmallKeyShuffleOot::parse("dungeon"),
            Some(SmallKeyShuffleOot::Dungeon)
        );
        assert_eq!(
            SmallKeyShuffleOot::parse("anywhere"),
            Some(SmallKeyShuffleOot::Anywhere)
        );
        assert_eq!(SmallKeyShuffleOot::parse("invalid"), None);
    }

    #[test]
    fn test_small_key_shuffle_oot_serde_roundtrip() {
        for mode in [
            SmallKeyShuffleOot::Vanilla,
            SmallKeyShuffleOot::Dungeon,
            SmallKeyShuffleOot::Anywhere,
        ] {
            let json = serde_json::to_string(&mode).unwrap();
            let parsed: SmallKeyShuffleOot = serde_json::from_str(&json).unwrap();
            assert_eq!(parsed, mode);
        }
    }

    // === ShufflePotsMm Tests ===

    #[test]
    fn test_shuffle_pots_mm_default() {
        let mode = ShufflePotsMm::default();
        assert_eq!(mode, ShufflePotsMm::None);
    }

    #[test]
    fn test_shuffle_pots_mm_as_str() {
        assert_eq!(ShufflePotsMm::None.as_str(), "none");
        assert_eq!(ShufflePotsMm::All.as_str(), "all");
    }

    #[test]
    fn test_shuffle_pots_mm_parse() {
        assert_eq!(ShufflePotsMm::parse("none"), Some(ShufflePotsMm::None));
        assert_eq!(ShufflePotsMm::parse("all"), Some(ShufflePotsMm::All));
        assert_eq!(ShufflePotsMm::parse("invalid"), None);
    }

    #[test]
    fn test_shuffle_pots_mm_serde_roundtrip() {
        for mode in [ShufflePotsMm::None, ShufflePotsMm::All] {
            let json = serde_json::to_string(&mode).unwrap();
            let parsed: ShufflePotsMm = serde_json::from_str(&json).unwrap();
            assert_eq!(parsed, mode);
        }
    }

    // === LogicMode Tests ===

    #[test]
    fn test_logic_mode_default() {
        let mode = LogicMode::default();
        assert_eq!(mode, LogicMode::Glitchless);
    }

    #[test]
    fn test_logic_mode_as_str() {
        assert_eq!(LogicMode::Glitchless.as_str(), "glitchless");
        assert_eq!(LogicMode::Glitched.as_str(), "glitched");
        assert_eq!(LogicMode::NoLogic.as_str(), "noLogic");
    }

    #[test]
    fn test_logic_mode_parse() {
        assert_eq!(LogicMode::parse("glitchless"), Some(LogicMode::Glitchless));
        assert_eq!(LogicMode::parse("glitched"), Some(LogicMode::Glitched));
        assert_eq!(LogicMode::parse("noLogic"), Some(LogicMode::NoLogic));
        assert_eq!(LogicMode::parse("no_logic"), Some(LogicMode::NoLogic));
        assert_eq!(LogicMode::parse("invalid"), None);
    }

    #[test]
    fn test_logic_mode_serde_roundtrip() {
        for mode in [
            LogicMode::Glitchless,
            LogicMode::Glitched,
            LogicMode::NoLogic,
        ] {
            let json = serde_json::to_string(&mode).unwrap();
            let parsed: LogicMode = serde_json::from_str(&json).unwrap();
            assert_eq!(parsed, mode);
        }
    }

    // === ShopShuffleMode Tests ===

    #[test]
    fn test_shop_shuffle_mode_default() {
        let mode = ShopShuffleMode::default();
        assert_eq!(mode, ShopShuffleMode::None);
    }

    #[test]
    fn test_shop_shuffle_mode_as_str() {
        assert_eq!(ShopShuffleMode::None.as_str(), "none");
        assert_eq!(ShopShuffleMode::OwnShop.as_str(), "ownShop");
        assert_eq!(ShopShuffleMode::All.as_str(), "all");
    }

    #[test]
    fn test_shop_shuffle_mode_parse() {
        assert_eq!(ShopShuffleMode::parse("none"), Some(ShopShuffleMode::None));
        assert_eq!(
            ShopShuffleMode::parse("ownShop"),
            Some(ShopShuffleMode::OwnShop)
        );
        assert_eq!(ShopShuffleMode::parse("all"), Some(ShopShuffleMode::All));
        assert_eq!(ShopShuffleMode::parse("invalid"), None);
    }

    #[test]
    fn test_shop_shuffle_mode_serde_roundtrip() {
        for mode in [
            ShopShuffleMode::None,
            ShopShuffleMode::OwnShop,
            ShopShuffleMode::All,
        ] {
            let json = serde_json::to_string(&mode).unwrap();
            let parsed: ShopShuffleMode = serde_json::from_str(&json).unwrap();
            assert_eq!(parsed, mode);
        }
    }

    // === PriceMode Tests ===

    #[test]
    fn test_price_mode_default() {
        let mode = PriceMode::default();
        assert_eq!(mode, PriceMode::Vanilla);
    }

    #[test]
    fn test_price_mode_as_str() {
        assert_eq!(PriceMode::Vanilla.as_str(), "vanilla");
        assert_eq!(PriceMode::Weighted.as_str(), "weighted");
        assert_eq!(PriceMode::Random.as_str(), "random");
        assert_eq!(PriceMode::Fixed.as_str(), "fixed");
    }

    #[test]
    fn test_price_mode_parse() {
        assert_eq!(PriceMode::parse("vanilla"), Some(PriceMode::Vanilla));
        assert_eq!(PriceMode::parse("weighted"), Some(PriceMode::Weighted));
        assert_eq!(PriceMode::parse("random"), Some(PriceMode::Random));
        assert_eq!(PriceMode::parse("fixed"), Some(PriceMode::Fixed));
        assert_eq!(PriceMode::parse("invalid"), None);
    }

    #[test]
    fn test_price_mode_serde_roundtrip() {
        for mode in [
            PriceMode::Vanilla,
            PriceMode::Weighted,
            PriceMode::Random,
            PriceMode::Fixed,
        ] {
            let json = serde_json::to_string(&mode).unwrap();
            let parsed: PriceMode = serde_json::from_str(&json).unwrap();
            assert_eq!(parsed, mode);
        }
    }

    // === TownFairyShuffle Tests ===

    #[test]
    fn test_town_fairy_shuffle_default() {
        let mode = TownFairyShuffle::default();
        assert_eq!(mode, TownFairyShuffle::Vanilla);
    }

    #[test]
    fn test_town_fairy_shuffle_as_str() {
        assert_eq!(TownFairyShuffle::Vanilla.as_str(), "vanilla");
        assert_eq!(TownFairyShuffle::Anywhere.as_str(), "anywhere");
    }

    #[test]
    fn test_town_fairy_shuffle_parse() {
        assert_eq!(
            TownFairyShuffle::parse("vanilla"),
            Some(TownFairyShuffle::Vanilla)
        );
        assert_eq!(
            TownFairyShuffle::parse("anywhere"),
            Some(TownFairyShuffle::Anywhere)
        );
        assert_eq!(TownFairyShuffle::parse("invalid"), None);
    }

    #[test]
    fn test_town_fairy_shuffle_serde_roundtrip() {
        for mode in [TownFairyShuffle::Vanilla, TownFairyShuffle::Anywhere] {
            let json = serde_json::to_string(&mode).unwrap();
            let parsed: TownFairyShuffle = serde_json::from_str(&json).unwrap();
            assert_eq!(parsed, mode);
        }
    }

    // === StrayFairyShuffle Tests ===

    #[test]
    fn test_stray_fairy_shuffle_default() {
        let mode = StrayFairyShuffle::default();
        assert_eq!(mode, StrayFairyShuffle::Vanilla);
    }

    #[test]
    fn test_stray_fairy_shuffle_as_str() {
        assert_eq!(StrayFairyShuffle::Vanilla.as_str(), "vanilla");
        assert_eq!(StrayFairyShuffle::Starting.as_str(), "starting");
        assert_eq!(StrayFairyShuffle::Removed.as_str(), "removed");
        assert_eq!(StrayFairyShuffle::OwnDungeon.as_str(), "ownDungeon");
        assert_eq!(StrayFairyShuffle::Anywhere.as_str(), "anywhere");
    }

    #[test]
    fn test_stray_fairy_shuffle_parse() {
        assert_eq!(
            StrayFairyShuffle::parse("vanilla"),
            Some(StrayFairyShuffle::Vanilla)
        );
        assert_eq!(
            StrayFairyShuffle::parse("starting"),
            Some(StrayFairyShuffle::Starting)
        );
        assert_eq!(
            StrayFairyShuffle::parse("removed"),
            Some(StrayFairyShuffle::Removed)
        );
        assert_eq!(
            StrayFairyShuffle::parse("ownDungeon"),
            Some(StrayFairyShuffle::OwnDungeon)
        );
        assert_eq!(
            StrayFairyShuffle::parse("anywhere"),
            Some(StrayFairyShuffle::Anywhere)
        );
        assert_eq!(StrayFairyShuffle::parse("invalid"), None);
    }

    #[test]
    fn test_stray_fairy_shuffle_serde_roundtrip() {
        for mode in [
            StrayFairyShuffle::Vanilla,
            StrayFairyShuffle::Starting,
            StrayFairyShuffle::Removed,
            StrayFairyShuffle::OwnDungeon,
            StrayFairyShuffle::Anywhere,
        ] {
            let json = serde_json::to_string(&mode).unwrap();
            let parsed: StrayFairyShuffle = serde_json::from_str(&json).unwrap();
            assert_eq!(parsed, mode);
        }
    }

    // === CrossWarpMode Tests ===

    #[test]
    fn test_cross_warp_mode_default() {
        let mode = CrossWarpMode::default();
        assert_eq!(mode, CrossWarpMode::None);
    }

    #[test]
    fn test_cross_warp_mode_as_str() {
        assert_eq!(CrossWarpMode::None.as_str(), "none");
        assert_eq!(CrossWarpMode::ChildOnly.as_str(), "childOnly");
        assert_eq!(CrossWarpMode::AdultOnly.as_str(), "adultOnly");
        assert_eq!(CrossWarpMode::Full.as_str(), "full");
    }

    #[test]
    fn test_cross_warp_mode_parse() {
        assert_eq!(CrossWarpMode::parse("none"), Some(CrossWarpMode::None));
        assert_eq!(
            CrossWarpMode::parse("childOnly"),
            Some(CrossWarpMode::ChildOnly)
        );
        assert_eq!(
            CrossWarpMode::parse("adultOnly"),
            Some(CrossWarpMode::AdultOnly)
        );
        assert_eq!(CrossWarpMode::parse("full"), Some(CrossWarpMode::Full));
        assert_eq!(CrossWarpMode::parse("invalid"), None);
    }

    #[test]
    fn test_cross_warp_mode_serde_roundtrip() {
        for mode in [
            CrossWarpMode::None,
            CrossWarpMode::ChildOnly,
            CrossWarpMode::AdultOnly,
            CrossWarpMode::Full,
        ] {
            let json = serde_json::to_string(&mode).unwrap();
            let parsed: CrossWarpMode = serde_json::from_str(&json).unwrap();
            assert_eq!(parsed, mode);
        }
    }

    // === CsmcMode Tests ===

    #[test]
    fn test_csmc_mode_default() {
        let mode = CsmcMode::default();
        assert_eq!(mode, CsmcMode::Never);
    }

    #[test]
    fn test_csmc_mode_as_str() {
        assert_eq!(CsmcMode::Never.as_str(), "never");
        assert_eq!(CsmcMode::Always.as_str(), "always");
        assert_eq!(CsmcMode::Agony.as_str(), "agony");
    }

    #[test]
    fn test_csmc_mode_parse() {
        assert_eq!(CsmcMode::parse("never"), Some(CsmcMode::Never));
        assert_eq!(CsmcMode::parse("always"), Some(CsmcMode::Always));
        assert_eq!(CsmcMode::parse("agony"), Some(CsmcMode::Agony));
        assert_eq!(CsmcMode::parse("invalid"), None);
    }

    #[test]
    fn test_csmc_mode_serde_roundtrip() {
        for mode in [CsmcMode::Never, CsmcMode::Always, CsmcMode::Agony] {
            let json = serde_json::to_string(&mode).unwrap();
            let parsed: CsmcMode = serde_json::from_str(&json).unwrap();
            assert_eq!(parsed, mode);
        }
    }

    // === BombchuBehavior Tests ===

    #[test]
    fn test_bombchu_behavior_default() {
        let mode = BombchuBehavior::default();
        assert_eq!(mode, BombchuBehavior::Normal);
    }

    #[test]
    fn test_bombchu_behavior_as_str() {
        assert_eq!(BombchuBehavior::Normal.as_str(), "normal");
        assert_eq!(BombchuBehavior::BombsOrLogic.as_str(), "bombsOrLogic");
        assert_eq!(BombchuBehavior::AsBombs.as_str(), "asBombs");
    }

    #[test]
    fn test_bombchu_behavior_parse() {
        assert_eq!(
            BombchuBehavior::parse("normal"),
            Some(BombchuBehavior::Normal)
        );
        assert_eq!(
            BombchuBehavior::parse("bombsOrLogic"),
            Some(BombchuBehavior::BombsOrLogic)
        );
        assert_eq!(
            BombchuBehavior::parse("asBombs"),
            Some(BombchuBehavior::AsBombs)
        );
        assert_eq!(BombchuBehavior::parse("invalid"), None);
    }

    #[test]
    fn test_bombchu_behavior_serde_roundtrip() {
        for mode in [
            BombchuBehavior::Normal,
            BombchuBehavior::BombsOrLogic,
            BombchuBehavior::AsBombs,
        ] {
            let json = serde_json::to_string(&mode).unwrap();
            let parsed: BombchuBehavior = serde_json::from_str(&json).unwrap();
            assert_eq!(parsed, mode);
        }
    }

    // === AutoInvertMode Tests ===

    #[test]
    fn test_auto_invert_mode_default() {
        let mode = AutoInvertMode::default();
        assert_eq!(mode, AutoInvertMode::Off);
    }

    #[test]
    fn test_auto_invert_mode_as_str() {
        assert_eq!(AutoInvertMode::Off.as_str(), "off");
        assert_eq!(AutoInvertMode::FirstPerson.as_str(), "firstPerson");
        assert_eq!(AutoInvertMode::Always.as_str(), "always");
    }

    #[test]
    fn test_auto_invert_mode_parse() {
        assert_eq!(AutoInvertMode::parse("off"), Some(AutoInvertMode::Off));
        assert_eq!(
            AutoInvertMode::parse("firstPerson"),
            Some(AutoInvertMode::FirstPerson)
        );
        assert_eq!(
            AutoInvertMode::parse("always"),
            Some(AutoInvertMode::Always)
        );
        assert_eq!(AutoInvertMode::parse("invalid"), None);
    }

    #[test]
    fn test_auto_invert_mode_serde_roundtrip() {
        for mode in [
            AutoInvertMode::Off,
            AutoInvertMode::FirstPerson,
            AutoInvertMode::Always,
        ] {
            let json = serde_json::to_string(&mode).unwrap();
            let parsed: AutoInvertMode = serde_json::from_str(&json).unwrap();
            assert_eq!(parsed, mode);
        }
    }

    // === StartingAge Tests ===

    #[test]
    fn test_starting_age_default() {
        let age = StartingAge::default();
        assert_eq!(age, StartingAge::Child);
    }

    #[test]
    fn test_starting_age_as_str() {
        assert_eq!(StartingAge::Child.as_str(), "child");
        assert_eq!(StartingAge::Adult.as_str(), "adult");
        assert_eq!(StartingAge::Random.as_str(), "random");
    }

    #[test]
    fn test_starting_age_parse() {
        assert_eq!(StartingAge::parse("child"), Some(StartingAge::Child));
        assert_eq!(StartingAge::parse("adult"), Some(StartingAge::Adult));
        assert_eq!(StartingAge::parse("random"), Some(StartingAge::Random));
        assert_eq!(StartingAge::parse("invalid"), None);
    }

    #[test]
    fn test_starting_age_serde_roundtrip() {
        for age in [StartingAge::Child, StartingAge::Adult, StartingAge::Random] {
            let json = serde_json::to_string(&age).unwrap();
            let parsed: StartingAge = serde_json::from_str(&json).unwrap();
            assert_eq!(parsed, age);
        }
    }

    // === DamageMultiplier Tests ===

    #[test]
    fn test_damage_multiplier_default() {
        let mult = DamageMultiplier::default();
        assert_eq!(mult, DamageMultiplier::Normal);
    }

    #[test]
    fn test_damage_multiplier_as_str() {
        assert_eq!(DamageMultiplier::Half.as_str(), "half");
        assert_eq!(DamageMultiplier::Normal.as_str(), "normal");
        assert_eq!(DamageMultiplier::Double.as_str(), "double");
        assert_eq!(DamageMultiplier::Quadruple.as_str(), "quadruple");
        assert_eq!(DamageMultiplier::Ohko.as_str(), "ohko");
    }

    #[test]
    fn test_damage_multiplier_parse() {
        assert_eq!(
            DamageMultiplier::parse("half"),
            Some(DamageMultiplier::Half)
        );
        assert_eq!(
            DamageMultiplier::parse("normal"),
            Some(DamageMultiplier::Normal)
        );
        assert_eq!(
            DamageMultiplier::parse("double"),
            Some(DamageMultiplier::Double)
        );
        assert_eq!(
            DamageMultiplier::parse("quadruple"),
            Some(DamageMultiplier::Quadruple)
        );
        assert_eq!(
            DamageMultiplier::parse("ohko"),
            Some(DamageMultiplier::Ohko)
        );
        assert_eq!(DamageMultiplier::parse("invalid"), None);
    }

    #[test]
    fn test_damage_multiplier_serde_roundtrip() {
        for mult in [
            DamageMultiplier::Half,
            DamageMultiplier::Normal,
            DamageMultiplier::Double,
            DamageMultiplier::Quadruple,
            DamageMultiplier::Ohko,
        ] {
            let json = serde_json::to_string(&mult).unwrap();
            let parsed: DamageMultiplier = serde_json::from_str(&json).unwrap();
            assert_eq!(parsed, mult);
        }
    }

    // === ItemPool Tests ===

    #[test]
    fn test_item_pool_default() {
        let pool = ItemPool::default();
        assert_eq!(pool, ItemPool::Normal);
    }

    #[test]
    fn test_item_pool_as_str() {
        assert_eq!(ItemPool::Plentiful.as_str(), "plentiful");
        assert_eq!(ItemPool::Normal.as_str(), "normal");
        assert_eq!(ItemPool::Scarce.as_str(), "scarce");
        assert_eq!(ItemPool::Minimal.as_str(), "minimal");
    }

    #[test]
    fn test_item_pool_parse() {
        assert_eq!(ItemPool::parse("plentiful"), Some(ItemPool::Plentiful));
        assert_eq!(ItemPool::parse("normal"), Some(ItemPool::Normal));
        assert_eq!(ItemPool::parse("scarce"), Some(ItemPool::Scarce));
        assert_eq!(ItemPool::parse("minimal"), Some(ItemPool::Minimal));
        assert_eq!(ItemPool::parse("invalid"), None);
    }

    #[test]
    fn test_item_pool_serde_roundtrip() {
        for pool in [
            ItemPool::Plentiful,
            ItemPool::Normal,
            ItemPool::Scarce,
            ItemPool::Minimal,
        ] {
            let json = serde_json::to_string(&pool).unwrap();
            let parsed: ItemPool = serde_json::from_str(&json).unwrap();
            assert_eq!(parsed, pool);
        }
    }

    // === TrapsQuantity Tests ===

    #[test]
    fn test_traps_quantity_default() {
        let qty = TrapsQuantity::default();
        assert_eq!(qty, TrapsQuantity::None);
    }

    #[test]
    fn test_traps_quantity_as_str() {
        assert_eq!(TrapsQuantity::None.as_str(), "none");
        assert_eq!(TrapsQuantity::Few.as_str(), "few");
        assert_eq!(TrapsQuantity::Normal.as_str(), "normal");
        assert_eq!(TrapsQuantity::Many.as_str(), "many");
        assert_eq!(TrapsQuantity::Onslaught.as_str(), "onslaught");
    }

    #[test]
    fn test_traps_quantity_parse() {
        assert_eq!(TrapsQuantity::parse("none"), Some(TrapsQuantity::None));
        assert_eq!(TrapsQuantity::parse("few"), Some(TrapsQuantity::Few));
        assert_eq!(TrapsQuantity::parse("normal"), Some(TrapsQuantity::Normal));
        assert_eq!(TrapsQuantity::parse("many"), Some(TrapsQuantity::Many));
        assert_eq!(
            TrapsQuantity::parse("onslaught"),
            Some(TrapsQuantity::Onslaught)
        );
        assert_eq!(TrapsQuantity::parse("invalid"), None);
    }

    #[test]
    fn test_traps_quantity_serde_roundtrip() {
        for qty in [
            TrapsQuantity::None,
            TrapsQuantity::Few,
            TrapsQuantity::Normal,
            TrapsQuantity::Many,
            TrapsQuantity::Onslaught,
        ] {
            let json = serde_json::to_string(&qty).unwrap();
            let parsed: TrapsQuantity = serde_json::from_str(&json).unwrap();
            assert_eq!(parsed, qty);
        }
    }

    // === SpecialCondition Tests ===

    #[test]
    fn test_special_condition_default() {
        let cond = SpecialCondition::default();
        assert_eq!(cond.stones, 0);
        assert_eq!(cond.medallions, 0);
        assert_eq!(cond.dungeon_rewards, 0);
        assert_eq!(cond.skulltulas, 0);
        assert_eq!(cond.remains, 0);
        assert!(!cond.has_requirements());
    }

    #[test]
    fn test_special_condition_new() {
        let cond = SpecialCondition::new();
        assert!(!cond.has_requirements());
    }

    #[test]
    fn test_special_condition_with_medallions() {
        let cond = SpecialCondition::with_medallions(6);
        assert_eq!(cond.medallions, 6);
        assert_eq!(cond.stones, 0);
        assert!(cond.has_requirements());
    }

    #[test]
    fn test_special_condition_with_stones() {
        let cond = SpecialCondition::with_stones(3);
        assert_eq!(cond.stones, 3);
        assert_eq!(cond.medallions, 0);
        assert!(cond.has_requirements());
    }

    #[test]
    fn test_special_condition_has_requirements() {
        let mut cond = SpecialCondition::default();
        assert!(!cond.has_requirements());

        cond.stones = 1;
        assert!(cond.has_requirements());

        cond = SpecialCondition::default();
        cond.medallions = 1;
        assert!(cond.has_requirements());

        cond = SpecialCondition::default();
        cond.dungeon_rewards = 1;
        assert!(cond.has_requirements());

        cond = SpecialCondition::default();
        cond.skulltulas = 1;
        assert!(cond.has_requirements());

        cond = SpecialCondition::default();
        cond.remains = 1;
        assert!(cond.has_requirements());
    }

    #[test]
    fn test_special_condition_serde_json_roundtrip() {
        let cond = SpecialCondition {
            stones: 3,
            medallions: 6,
            dungeon_rewards: 9,
            skulltulas: 50,
            remains: 4,
        };
        let json = serde_json::to_string(&cond).unwrap();
        let parsed: SpecialCondition = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed.stones, 3);
        assert_eq!(parsed.medallions, 6);
        assert_eq!(parsed.dungeon_rewards, 9);
        assert_eq!(parsed.skulltulas, 50);
        assert_eq!(parsed.remains, 4);
    }

    #[test]
    fn test_special_condition_serde_yaml_roundtrip() {
        let cond = SpecialCondition {
            stones: 3,
            medallions: 6,
            dungeon_rewards: 0,
            skulltulas: 0,
            remains: 4,
        };
        let yaml = serde_yaml::to_string(&cond).unwrap();
        let parsed: SpecialCondition = serde_yaml::from_str(&yaml).unwrap();
        assert_eq!(parsed.stones, 3);
        assert_eq!(parsed.medallions, 6);
        assert_eq!(parsed.remains, 4);
    }

    #[test]
    fn test_special_condition_partial_deserialization() {
        // Test that missing fields default to 0
        let json = r#"{"medallions": 6}"#;
        let parsed: SpecialCondition = serde_json::from_str(json).unwrap();
        assert_eq!(parsed.medallions, 6);
        assert_eq!(parsed.stones, 0);
        assert_eq!(parsed.dungeon_rewards, 0);
    }

    // === WorldFlags Tests ===

    #[test]
    fn test_world_flags_default() {
        // Default trait derives default values (false for booleans)
        let flags = WorldFlags::default();
        assert!(!flags.oot_enabled);
        assert!(!flags.mm_enabled);
        assert!(!flags.shared_items);
        assert!(!flags.shared_masks);
    }

    #[test]
    fn test_world_flags_new() {
        // new() uses Default, so also defaults to false
        let flags = WorldFlags::new();
        assert!(!flags.is_oot_enabled());
        assert!(!flags.is_mm_enabled());
    }

    #[test]
    fn test_world_flags_accessors() {
        let mut flags = WorldFlags::default();
        // Start with default values (false)
        assert!(!flags.is_oot_enabled());
        assert!(!flags.is_mm_enabled());

        // Set to true and verify
        flags.oot_enabled = true;
        assert!(flags.is_oot_enabled());

        flags.mm_enabled = true;
        assert!(flags.is_mm_enabled());
    }

    #[test]
    fn test_world_flags_serde_json_roundtrip() {
        let flags = WorldFlags {
            oot_enabled: true,
            mm_enabled: false,
            shared_items: true,
            shared_masks: true,
        };
        let json = serde_json::to_string(&flags).unwrap();
        let parsed: WorldFlags = serde_json::from_str(&json).unwrap();
        assert!(parsed.oot_enabled);
        assert!(!parsed.mm_enabled);
        assert!(parsed.shared_items);
        assert!(parsed.shared_masks);
    }

    #[test]
    fn test_world_flags_serde_yaml_roundtrip() {
        let flags = WorldFlags {
            oot_enabled: false,
            mm_enabled: true,
            shared_items: false,
            shared_masks: true,
        };
        let yaml = serde_yaml::to_string(&flags).unwrap();
        let parsed: WorldFlags = serde_yaml::from_str(&yaml).unwrap();
        assert!(!parsed.oot_enabled);
        assert!(parsed.mm_enabled);
        assert!(!parsed.shared_items);
        assert!(parsed.shared_masks);
    }

    #[test]
    fn test_world_flags_partial_deserialization() {
        // Test that missing fields use default values
        let json = r#"{"sharedItems": true}"#;
        let parsed: WorldFlags = serde_json::from_str(json).unwrap();
        assert!(parsed.oot_enabled); // defaults to true
        assert!(parsed.mm_enabled); // defaults to true
        assert!(parsed.shared_items);
        assert!(!parsed.shared_masks); // defaults to false
    }

    // === Starting Items Tests ===

    #[test]
    fn test_starting_items_operations() {
        let mut settings = RandomizerSettings::new();
        assert_eq!(settings.starting_item_quantity("Sword"), 0);
        assert!(!settings.has_starting_item("Sword"));

        settings.set_starting_item("Sword", 1);
        assert_eq!(settings.starting_item_quantity("Sword"), 1);
        assert!(settings.has_starting_item("Sword"));

        settings.set_starting_item("Bow", 3);
        assert_eq!(settings.starting_item_quantity("Bow"), 3);
        assert_eq!(settings.starting_items_count(), 2);

        settings.remove_starting_item("Sword");
        assert!(!settings.has_starting_item("Sword"));
        assert_eq!(settings.starting_items_count(), 1);

        // Setting to 0 removes the item
        settings.set_starting_item("Bow", 0);
        assert!(!settings.has_starting_item("Bow"));
        assert_eq!(settings.starting_items_count(), 0);
    }

    #[test]
    fn test_starting_items_iterator() {
        let mut settings = RandomizerSettings::new();
        settings.set_starting_item("Sword", 1);
        settings.set_starting_item("Shield", 2);
        settings.set_starting_item("Bow", 3);

        let items: Vec<_> = settings.starting_items_iter().collect();
        assert_eq!(items.len(), 3);
    }

    #[test]
    fn test_starting_items_serde_roundtrip() {
        let mut settings = RandomizerSettings::new();
        settings.set_starting_item("Kokiri_Sword", 1);
        settings.set_starting_item("Deku_Shield", 1);
        settings.set_starting_item("Bombs", 20);

        let json = serde_json::to_string(&settings).unwrap();
        let parsed: RandomizerSettings = serde_json::from_str(&json).unwrap();

        assert_eq!(parsed.starting_item_quantity("Kokiri_Sword"), 1);
        assert_eq!(parsed.starting_item_quantity("Deku_Shield"), 1);
        assert_eq!(parsed.starting_item_quantity("Bombs"), 20);
    }

    // === Junk Locations Tests ===

    #[test]
    fn test_junk_locations_operations() {
        let mut settings = RandomizerSettings::new();
        assert!(!settings.is_junk_location("oot_chest_1"));
        assert_eq!(settings.junk_locations_count(), 0);

        settings.add_junk_location("oot_chest_1");
        assert!(settings.is_junk_location("oot_chest_1"));
        assert_eq!(settings.junk_locations_count(), 1);

        settings.add_junk_location("mm_chest_2");
        assert_eq!(settings.junk_locations_count(), 2);

        settings.remove_junk_location("oot_chest_1");
        assert!(!settings.is_junk_location("oot_chest_1"));
        assert_eq!(settings.junk_locations_count(), 1);
    }

    #[test]
    fn test_junk_locations_iterator() {
        let mut settings = RandomizerSettings::new();
        settings.add_junk_location("loc1");
        settings.add_junk_location("loc2");
        settings.add_junk_location("loc3");

        let locations: Vec<_> = settings.junk_locations_iter().collect();
        assert_eq!(locations.len(), 3);
    }

    #[test]
    fn test_junk_locations_serde_roundtrip() {
        let mut settings = RandomizerSettings::new();
        settings.add_junk_location("oot_kokiri_chest");
        settings.add_junk_location("mm_clock_tower_chest");

        let json = serde_json::to_string(&settings).unwrap();
        let parsed: RandomizerSettings = serde_json::from_str(&json).unwrap();

        assert!(parsed.is_junk_location("oot_kokiri_chest"));
        assert!(parsed.is_junk_location("mm_clock_tower_chest"));
    }

    // === Special Condition Settings Methods Tests ===

    #[test]
    fn test_special_conditions_operations() {
        let mut settings = RandomizerSettings::new();
        assert!(!settings.has_special_condition("bridge"));
        assert!(settings.get_special_condition("bridge").is_none());
        assert!(settings.bridge_condition().is_none());
        assert_eq!(settings.special_conditions_count(), 0);

        let bridge_cond = SpecialCondition::with_medallions(6);
        settings.set_special_condition("bridge", bridge_cond);

        assert!(settings.has_special_condition("bridge"));
        assert!(settings.get_special_condition("bridge").is_some());
        assert!(settings.bridge_condition().is_some());
        assert_eq!(settings.bridge_condition().unwrap().medallions, 6);
        assert_eq!(settings.special_conditions_count(), 1);

        settings.remove_special_condition("bridge");
        assert!(!settings.has_special_condition("bridge"));
        assert_eq!(settings.special_conditions_count(), 0);
    }

    #[test]
    fn test_special_conditions_iterator() {
        let mut settings = RandomizerSettings::new();
        settings.set_special_condition("bridge", SpecialCondition::with_medallions(6));
        settings.set_special_condition("lacs", SpecialCondition::with_stones(3));

        let conditions: Vec<_> = settings.special_conditions_iter().collect();
        assert_eq!(conditions.len(), 2);
    }

    #[test]
    fn test_special_conditions_serde_roundtrip() {
        let mut settings = RandomizerSettings::new();
        settings.set_special_condition("bridge", SpecialCondition::with_medallions(6));
        settings.set_special_condition("lacs", SpecialCondition::with_stones(3));

        let json = serde_json::to_string(&settings).unwrap();
        let parsed: RandomizerSettings = serde_json::from_str(&json).unwrap();

        assert_eq!(
            parsed.get_special_condition("bridge").unwrap().medallions,
            6
        );
        assert_eq!(parsed.get_special_condition("lacs").unwrap().stones, 3);
    }

    // === World Flags Settings Methods Tests ===

    #[test]
    fn test_world_flags_settings_accessors() {
        let mut settings = RandomizerSettings::new();

        // Default values (false for all booleans)
        assert!(!settings.is_oot_enabled());
        assert!(!settings.is_mm_enabled());
        assert!(!settings.world_shared_items());
        assert!(!settings.world_shared_masks());

        // Modify and check
        settings.world_flags.oot_enabled = true;
        assert!(settings.is_oot_enabled());

        settings.world_flags.mm_enabled = true;
        assert!(settings.is_mm_enabled());

        settings.world_flags.shared_items = true;
        assert!(settings.world_shared_items());

        settings.world_flags.shared_masks = true;
        assert!(settings.world_shared_masks());
    }

    // === Comprehensive JSON Roundtrip Test ===

    #[test]
    fn test_comprehensive_json_roundtrip() {
        let mut settings = RandomizerSettings::new();

        // Set various boolean settings
        settings.er_moon = true;
        settings.skip_zelda = true;
        settings.ageless_boots = true;
        settings.shared_bows = true;

        // Set enum settings
        settings.ganon_boss_key = GanonBossKeyMode::Custom;
        settings.logic_mode = LogicMode::Glitched;
        settings.rainbow_bridge = RainbowBridgeMode::Medallions;
        settings.starting_age = StartingAge::Adult;
        settings.damage_multiplier = DamageMultiplier::Double;

        // Set collection fields
        settings
            .open_dungeons_oot
            .insert(OotDungeon::DodongosCavern);
        settings.open_dungeons_mm.insert(MmDungeon::StoneTower);
        settings.set_dungeon_mq(MqDungeon::ForestTemple);
        settings.enable_trick("OOT_LENS");

        // Set complex types
        settings.set_starting_item("Sword", 1);
        settings.add_junk_location("oot_chest");
        settings.set_special_condition("bridge", SpecialCondition::with_medallions(6));

        // Set world flags
        settings.world_flags.shared_items = true;

        // Serialize and deserialize
        let json = serde_json::to_string(&settings).unwrap();
        let parsed: RandomizerSettings = serde_json::from_str(&json).unwrap();

        // Verify all fields
        assert!(parsed.er_moon);
        assert!(parsed.skip_zelda);
        assert!(parsed.ageless_boots);
        assert!(parsed.shared_bows);
        assert_eq!(parsed.ganon_boss_key, GanonBossKeyMode::Custom);
        assert_eq!(parsed.logic_mode, LogicMode::Glitched);
        assert_eq!(parsed.rainbow_bridge, RainbowBridgeMode::Medallions);
        assert_eq!(parsed.starting_age, StartingAge::Adult);
        assert_eq!(parsed.damage_multiplier, DamageMultiplier::Double);
        assert!(parsed
            .open_dungeons_oot
            .contains(&OotDungeon::DodongosCavern));
        assert!(parsed.open_dungeons_mm.contains(&MmDungeon::StoneTower));
        assert!(parsed.is_dungeon_mq(MqDungeon::ForestTemple));
        assert!(parsed.has_trick("OOT_LENS"));
        assert_eq!(parsed.starting_item_quantity("Sword"), 1);
        assert!(parsed.is_junk_location("oot_chest"));
        assert_eq!(parsed.bridge_condition().unwrap().medallions, 6);
        assert!(parsed.world_shared_items());
    }

    // === Comprehensive YAML Roundtrip Test ===

    #[test]
    fn test_comprehensive_yaml_roundtrip() {
        let mut settings = RandomizerSettings::new();

        // Set various settings
        settings.er_moon = true;
        settings.open_mask_shop = true;
        settings.deku_tree = DekuTreeState::Open;
        settings.door_of_time = DoorOfTimeState::Open;
        settings.kakariko_gate = KakarikoGateState::Open;
        settings.boss_warp_pads = BossWarpPadsMode::Remains;
        settings.csmc = CsmcMode::Always;
        settings.item_pool = ItemPool::Scarce;
        settings.traps_quantity = TrapsQuantity::Many;

        settings.open_dungeons_oot.insert(OotDungeon::Shadow);
        settings.open_dungeons_oot.insert(OotDungeon::Water);
        settings.jp_layouts.insert(JpLayout::StoneTower);

        settings.set_starting_item("Bow", 1);
        settings.set_starting_item("Bombs", 20);
        settings.world_flags.mm_enabled = false;

        // Serialize and deserialize via YAML
        let yaml = serde_yaml::to_string(&settings).unwrap();
        let parsed: RandomizerSettings = serde_yaml::from_str(&yaml).unwrap();

        // Verify fields
        assert!(parsed.er_moon);
        assert!(parsed.open_mask_shop);
        assert_eq!(parsed.deku_tree, DekuTreeState::Open);
        assert_eq!(parsed.door_of_time, DoorOfTimeState::Open);
        assert_eq!(parsed.kakariko_gate, KakarikoGateState::Open);
        assert_eq!(parsed.boss_warp_pads, BossWarpPadsMode::Remains);
        assert_eq!(parsed.csmc, CsmcMode::Always);
        assert_eq!(parsed.item_pool, ItemPool::Scarce);
        assert_eq!(parsed.traps_quantity, TrapsQuantity::Many);
        assert!(parsed.open_dungeons_oot.contains(&OotDungeon::Shadow));
        assert!(parsed.open_dungeons_oot.contains(&OotDungeon::Water));
        assert!(parsed.jp_layouts.contains(&JpLayout::StoneTower));
        assert_eq!(parsed.starting_item_quantity("Bow"), 1);
        assert_eq!(parsed.starting_item_quantity("Bombs"), 20);
        assert!(!parsed.is_mm_enabled());
    }

    // === OotDungeon additional tests ===

    #[test]
    fn test_oot_dungeon_as_str_all_variants() {
        assert_eq!(OotDungeon::DodongosCavern.as_str(), "DC");
        assert_eq!(OotDungeon::BottomOfTheWell.as_str(), "BotW");
        assert_eq!(OotDungeon::JabuJabu.as_str(), "JJ");
        assert_eq!(OotDungeon::Shadow.as_str(), "Shadow");
        assert_eq!(OotDungeon::Water.as_str(), "Water");
        assert_eq!(OotDungeon::FireChild.as_str(), "fireChild");
        assert_eq!(OotDungeon::WellAdult.as_str(), "wellAdult");
    }

    #[test]
    fn test_oot_dungeon_roundtrip_all_variants() {
        for dungeon in [
            OotDungeon::DodongosCavern,
            OotDungeon::BottomOfTheWell,
            OotDungeon::JabuJabu,
            OotDungeon::Shadow,
            OotDungeon::Water,
            OotDungeon::FireChild,
            OotDungeon::WellAdult,
        ] {
            let s = dungeon.as_str();
            assert_eq!(OotDungeon::parse(s), Some(dungeon));
        }
    }

    #[test]
    fn test_oot_dungeon_serde_roundtrip() {
        for dungeon in [
            OotDungeon::DodongosCavern,
            OotDungeon::BottomOfTheWell,
            OotDungeon::JabuJabu,
            OotDungeon::Shadow,
            OotDungeon::Water,
            OotDungeon::FireChild,
            OotDungeon::WellAdult,
        ] {
            let json = serde_json::to_string(&dungeon).unwrap();
            let parsed: OotDungeon = serde_json::from_str(&json).unwrap();
            assert_eq!(parsed, dungeon);
        }
    }

    // === MmDungeon additional tests ===

    #[test]
    fn test_mm_dungeon_as_str_all_variants() {
        assert_eq!(MmDungeon::StoneTower.as_str(), "ST");
        assert_eq!(MmDungeon::Woodfall.as_str(), "WF");
    }

    #[test]
    fn test_mm_dungeon_roundtrip_all_variants() {
        for dungeon in [MmDungeon::StoneTower, MmDungeon::Woodfall] {
            let s = dungeon.as_str();
            assert_eq!(MmDungeon::parse(s), Some(dungeon));
        }
    }

    #[test]
    fn test_mm_dungeon_serde_roundtrip() {
        for dungeon in [MmDungeon::StoneTower, MmDungeon::Woodfall] {
            let json = serde_json::to_string(&dungeon).unwrap();
            let parsed: MmDungeon = serde_json::from_str(&json).unwrap();
            assert_eq!(parsed, dungeon);
        }
    }

    // === MqDungeon additional tests ===

    #[test]
    fn test_mq_dungeon_as_str_all_variants() {
        assert_eq!(MqDungeon::DekuTree.as_str(), "deku_tree");
        assert_eq!(MqDungeon::DodongosCavern.as_str(), "dodongos_cavern");
        assert_eq!(MqDungeon::JabuJabu.as_str(), "jabu_jabu");
        assert_eq!(MqDungeon::ForestTemple.as_str(), "forest_temple");
        assert_eq!(MqDungeon::FireTemple.as_str(), "fire_temple");
        assert_eq!(MqDungeon::WaterTemple.as_str(), "water_temple");
        assert_eq!(MqDungeon::SpiritTemple.as_str(), "spirit_temple");
        assert_eq!(MqDungeon::ShadowTemple.as_str(), "shadow_temple");
        assert_eq!(MqDungeon::BottomOfTheWell.as_str(), "bottom_of_the_well");
        assert_eq!(MqDungeon::IceCavern.as_str(), "ice_cavern");
        assert_eq!(
            MqDungeon::GerudoTrainingGround.as_str(),
            "gerudo_training_ground"
        );
        assert_eq!(MqDungeon::GanonsCastle.as_str(), "ganons_castle");
    }

    #[test]
    fn test_mq_dungeon_serde_roundtrip_all() {
        for dungeon in MqDungeon::all() {
            let json = serde_json::to_string(dungeon).unwrap();
            let parsed: MqDungeon = serde_json::from_str(&json).unwrap();
            assert_eq!(parsed, *dungeon);
        }
    }
}
