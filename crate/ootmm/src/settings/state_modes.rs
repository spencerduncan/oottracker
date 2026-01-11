//! State and mode enums for randomizer settings.
//!
//! This module contains all the enumeration types used to represent
//! various game states and configuration modes in the randomizer.

use serde::{Deserialize, Serialize};

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
