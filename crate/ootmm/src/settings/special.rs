//! Special condition and world flag types for randomizer settings.
//!
//! This module contains complex types used for custom requirements
//! and world configuration.

use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};

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
