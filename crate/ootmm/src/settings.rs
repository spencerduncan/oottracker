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
use std::collections::HashSet;

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
}
