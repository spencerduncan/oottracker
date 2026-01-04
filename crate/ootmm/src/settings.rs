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
}
