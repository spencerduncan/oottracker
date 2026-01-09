# OoTMM Settings Parser Implementation Plan

This document describes a phased implementation plan for adding comprehensive OoTMM randomizer config file parsing support to the oottracker project.

## Overview

The OoTMM randomizer uses a YAML-based configuration format with the following sections:
- **Settings**: ~200+ key-value pairs (booleans, enums, numbers)
- **Special Conditions**: Nested structures for BRIDGE, MOON, LACS, GANON_BK, MAJORA
- **Starting Items**: Item name to quantity mapping
- **Junk Locations**: List of location names excluded from logic
- **World Flags**: World-level configuration (trials, key rings, etc.)

### Current State

The existing `RandomizerSettings` struct in `/crate/ootmm/src/settings.rs` has:
- ~15 boolean settings
- ~10 enum settings
- ~5 set-based settings
- 1 bottle_count setting

### Target State

Full support for parsing OoTMM config files with all sections and ~200+ settings.

### Estimated Size

| Component | Estimated Lines |
|-----------|----------------|
| New enum types (~30 new enums) | ~600 |
| Expanded `RandomizerSettings` fields | ~400 |
| `SpecialCondition` struct + parsing | ~150 |
| Starting items / junk locations types | ~100 |
| World flags struct | ~150 |
| Config file parser (YAML wrapper) | ~200 |
| Tests | ~400+ |
| **Total** | **~2000+ lines** |

---

## Dependency Graph

```
Phase 1: Core Types Foundation
├─ [1a] SpecialCondition struct
├─ [1b] StartingItems type
├─ [1c] JunkLocations type
└─ [1d] WorldFlags struct
         │
         ▼
Phase 2: Missing Enum Types (parallel)
├─ [2a] RainbowBridgeMode
├─ [2b] SongsMode
├─ [2c] DungeonRewardShuffle
├─ [2d] ShopShuffleMode
├─ [2e] Fairy shuffle enums
├─ [2f] PriceMode
├─ [2g] CrossWarpMode
├─ [2h] CsmcMode
├─ [2i] ProgressiveMode
├─ [2j] Misc enums (BombchuBehavior, AutoInvert, etc.)
└─ [2k] ShuffleMode and related enums
         │
         ▼
Phase 3: Expand RandomizerSettings
├─ [3a] Add missing boolean settings (~100 fields)
├─ [3b] Add missing enum settings (~50 fields)
├─ [3c] Add special_conditions field
├─ [3d] Add starting_items field
├─ [3e] Add junk_locations field
└─ [3f] Add world_flags field
         │
         ▼
Phase 4: Config File Parser (sequential)
├─ [4a] Create OotmmConfigFile wrapper struct
├─ [4b] Implement From<OotmmConfigFile> for RandomizerSettings
├─ [4c] Add from_yaml_file() method
└─ [4d] Add from_yaml_str() method
         │
         ▼
Phase 5: Integration & Getters (parallel)
├─ [5a] Update get_bool_setting()
├─ [5b] Update check_setting_value()
├─ [5c] Add special condition methods
└─ [5d] Add starting items / junk location methods
         │
         ▼
Phase 6: Tests (parallel)
├─ [6a] Unit tests for new enum types
├─ [6b] Unit tests for SpecialCondition, WorldFlags
├─ [6c] Integration tests for config parsing
└─ [6d] Round-trip serialization tests
```

---

## Phase 1: Core Types Foundation

### Step 1a: Add SpecialCondition Struct

**File to modify:** `/home/user/oottracker/crate/ootmm/src/settings.rs`

**Description:** Create a `SpecialCondition` struct that represents the requirements for game conditions like BRIDGE, MOON, LACS, GANON_BK, and MAJORA. Each condition has a count and multiple boolean flags for different collectible types.

**Implementation:**

Add the following struct after the existing enum definitions (around line 940):

```rust
/// Special condition requirements for game progression gates.
///
/// Used for BRIDGE, MOON, LACS, GANON_BK, and MAJORA conditions.
/// Each condition specifies a count and which collectible types are required.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SpecialCondition {
    /// Number of items required
    #[serde(default)]
    pub count: u32,

    /// Require spiritual stones
    #[serde(default)]
    pub stones: bool,

    /// Require medallions
    #[serde(default)]
    pub medallions: bool,

    /// Require boss remains
    #[serde(default)]
    pub remains: bool,

    /// Require gold skulltula tokens
    #[serde(default, rename = "skullsGold")]
    pub skulls_gold: bool,

    /// Require Swamp Spider House skulltulas
    #[serde(default, rename = "skullsSwamp")]
    pub skulls_swamp: bool,

    /// Require Ocean Spider House skulltulas
    #[serde(default, rename = "skullsOcean")]
    pub skulls_ocean: bool,

    /// Require Woodfall stray fairies
    #[serde(default, rename = "fairiesWF")]
    pub fairies_wf: bool,

    /// Require Snowhead stray fairies
    #[serde(default, rename = "fairiesSH")]
    pub fairies_sh: bool,

    /// Require Great Bay stray fairies
    #[serde(default, rename = "fairiesGB")]
    pub fairies_gb: bool,

    /// Require Stone Tower stray fairies
    #[serde(default, rename = "fairiesST")]
    pub fairies_st: bool,

    /// Require Clock Town stray fairy
    #[serde(default, rename = "fairyTown")]
    pub fairy_town: bool,

    /// Require regular masks
    #[serde(default, rename = "masksRegular")]
    pub masks_regular: bool,

    /// Require transformation masks
    #[serde(default, rename = "masksTransform")]
    pub masks_transform: bool,

    /// Require OoT masks
    #[serde(default, rename = "masksOot")]
    pub masks_oot: bool,

    /// Require triforce pieces
    #[serde(default)]
    pub triforce: bool,

    /// Require red coins
    #[serde(default, rename = "coinsRed")]
    pub coins_red: bool,

    /// Require green coins
    #[serde(default, rename = "coinsGreen")]
    pub coins_green: bool,

    /// Require blue coins
    #[serde(default, rename = "coinsBlue")]
    pub coins_blue: bool,

    /// Require yellow coins
    #[serde(default, rename = "coinsYellow")]
    pub coins_yellow: bool,
}
```

**Tests to add:**

```rust
#[test]
fn test_special_condition_default() {
    let cond = SpecialCondition::default();
    assert_eq!(cond.count, 0);
    assert!(!cond.stones);
    assert!(!cond.medallions);
    assert!(!cond.remains);
}

#[test]
fn test_special_condition_serde_roundtrip() {
    let cond = SpecialCondition {
        count: 4,
        remains: true,
        medallions: true,
        ..Default::default()
    };
    let json = serde_json::to_string(&cond).unwrap();
    let parsed: SpecialCondition = serde_json::from_str(&json).unwrap();
    assert_eq!(parsed.count, 4);
    assert!(parsed.remains);
    assert!(parsed.medallions);
    assert!(!parsed.stones);
}
```

**Acceptance criteria:**
- [ ] SpecialCondition struct compiles with all fields
- [ ] Default implementation sets count to 0 and all booleans to false
- [ ] Serde serialization uses camelCase field names
- [ ] Tests pass

---

### Step 1b: Add StartingItems Type

**File to modify:** `/home/user/oottracker/crate/ootmm/src/settings.rs`

**Description:** Create a type alias and helper methods for starting items configuration. Starting items map item names (strings) to quantities.

**Implementation:**

Add after SpecialCondition struct:

```rust
use std::collections::HashMap;

/// Starting items configuration mapping item names to quantities.
///
/// Example: `{"Deku Stick": 10, "10 Deku Nuts": 1}`
pub type StartingItems = HashMap<String, u32>;

/// Helper functions for StartingItems
pub trait StartingItemsExt {
    /// Returns the quantity of a starting item, or 0 if not present.
    fn get_quantity(&self, item: &str) -> u32;

    /// Checks if an item is in the starting inventory.
    fn has_item(&self, item: &str) -> bool;
}

impl StartingItemsExt for StartingItems {
    fn get_quantity(&self, item: &str) -> u32 {
        self.get(item).copied().unwrap_or(0)
    }

    fn has_item(&self, item: &str) -> bool {
        self.get(item).map(|&q| q > 0).unwrap_or(false)
    }
}
```

**Tests to add:**

```rust
#[test]
fn test_starting_items_get_quantity() {
    let mut items: StartingItems = HashMap::new();
    items.insert("Deku Stick".to_string(), 10);
    items.insert("10 Deku Nuts".to_string(), 1);

    assert_eq!(items.get_quantity("Deku Stick"), 10);
    assert_eq!(items.get_quantity("10 Deku Nuts"), 1);
    assert_eq!(items.get_quantity("Hookshot"), 0);
}

#[test]
fn test_starting_items_has_item() {
    let mut items: StartingItems = HashMap::new();
    items.insert("Deku Stick".to_string(), 10);

    assert!(items.has_item("Deku Stick"));
    assert!(!items.has_item("Hookshot"));
}
```

**Acceptance criteria:**
- [ ] StartingItems type alias works with HashMap<String, u32>
- [ ] Extension trait provides get_quantity and has_item methods
- [ ] Tests pass

---

### Step 1c: Add JunkLocations Type

**File to modify:** `/home/user/oottracker/crate/ootmm/src/settings.rs`

**Description:** Create a type alias for junk locations - locations that are excluded from logic or marked as containing only junk items.

**Implementation:**

Add after StartingItems:

```rust
/// Junk locations that are excluded from logic consideration.
///
/// These locations are guaranteed to contain non-progression items.
/// Example: `["MM Laboratory Zora Song", "OOT Skulltula House 50 Tokens"]`
pub type JunkLocations = HashSet<String>;

/// Helper functions for JunkLocations
pub trait JunkLocationsExt {
    /// Checks if a location is marked as junk.
    fn is_junk(&self, location: &str) -> bool;
}

impl JunkLocationsExt for JunkLocations {
    fn is_junk(&self, location: &str) -> bool {
        self.contains(location)
    }
}
```

**Tests to add:**

```rust
#[test]
fn test_junk_locations() {
    let mut junk: JunkLocations = HashSet::new();
    junk.insert("MM Laboratory Zora Song".to_string());
    junk.insert("OOT Skulltula House 50 Tokens".to_string());

    assert!(junk.is_junk("MM Laboratory Zora Song"));
    assert!(junk.is_junk("OOT Skulltula House 50 Tokens"));
    assert!(!junk.is_junk("MM Clock Town Chest"));
}
```

**Acceptance criteria:**
- [ ] JunkLocations type alias works with HashSet<String>
- [ ] Extension trait provides is_junk method
- [ ] Tests pass

---

### Step 1d: Add WorldFlags Struct

**File to modify:** `/home/user/oottracker/crate/ootmm/src/settings.rs`

**Description:** Create a WorldFlags struct that holds world-level configuration flags like Ganon trials, small key rings, silver rupee pouches, etc.

**Implementation:**

Add after JunkLocations:

```rust
/// World-level configuration flags.
///
/// These flags control global world state like which trials are active,
/// key ring settings, and dungeon configurations.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WorldFlags {
    /// Ganon's Castle trials setting ("all", "none", or specific count)
    #[serde(default, rename = "Ganon Trials")]
    pub ganon_trials: String,

    /// Small key ring setting for OoT ("all", "none", or specific dungeons)
    #[serde(default, rename = "Small Key Ring (OoT)")]
    pub small_key_ring_oot: String,

    /// Small key ring setting for MM
    #[serde(default, rename = "Small Key Ring (MM)")]
    pub small_key_ring_mm: String,

    /// Silver rupee pouches setting
    #[serde(default, rename = "Silver Rupee Pouches")]
    pub silver_rupee_pouches: String,

    /// Open dungeons for MM
    #[serde(default, rename = "Open Dungeons (MM)")]
    pub open_dungeons_mm: String,

    /// Open dungeons for OoT
    #[serde(default, rename = "Open Dungeons (OoT)")]
    pub open_dungeons_oot: String,

    /// Pre-activated owl statues
    #[serde(default, rename = "Pre-Activated Owl Statues")]
    pub pre_activated_owl_statues: String,

    /// Master Quest dungeons setting
    #[serde(default, rename = "Master Quest Dungeons")]
    pub master_quest_dungeons: String,

    /// Majora's Mask JP layouts
    #[serde(default, rename = "Majora's Mask JP Layouts")]
    pub mm_jp_layouts: String,
}

impl WorldFlags {
    /// Checks if all Ganon trials are enabled.
    pub fn all_ganon_trials(&self) -> bool {
        self.ganon_trials.eq_ignore_ascii_case("all")
    }

    /// Checks if no Ganon trials are enabled.
    pub fn no_ganon_trials(&self) -> bool {
        self.ganon_trials.eq_ignore_ascii_case("none")
    }

    /// Checks if all small key rings are enabled for OoT.
    pub fn all_key_rings_oot(&self) -> bool {
        self.small_key_ring_oot.eq_ignore_ascii_case("all")
    }

    /// Checks if all small key rings are enabled for MM.
    pub fn all_key_rings_mm(&self) -> bool {
        self.small_key_ring_mm.eq_ignore_ascii_case("all")
    }
}
```

**Tests to add:**

```rust
#[test]
fn test_world_flags_default() {
    let flags = WorldFlags::default();
    assert!(flags.ganon_trials.is_empty());
    assert!(!flags.all_ganon_trials());
}

#[test]
fn test_world_flags_ganon_trials() {
    let mut flags = WorldFlags::default();
    flags.ganon_trials = "all".to_string();
    assert!(flags.all_ganon_trials());
    assert!(!flags.no_ganon_trials());

    flags.ganon_trials = "none".to_string();
    assert!(!flags.all_ganon_trials());
    assert!(flags.no_ganon_trials());
}

#[test]
fn test_world_flags_key_rings() {
    let mut flags = WorldFlags::default();
    flags.small_key_ring_oot = "all".to_string();
    flags.small_key_ring_mm = "none".to_string();

    assert!(flags.all_key_rings_oot());
    assert!(!flags.all_key_rings_mm());
}
```

**Acceptance criteria:**
- [ ] WorldFlags struct compiles with all fields
- [ ] Field names serialize to match config format (with spaces)
- [ ] Helper methods work correctly
- [ ] Tests pass

---

## Phase 2: Missing Enum Types

### Step 2a: Add RainbowBridgeMode Enum

**File to modify:** `/home/user/oottracker/crate/ootmm/src/settings.rs`

**Description:** Create an enum for the Rainbow Bridge requirement mode. This controls what is needed to access the bridge to Ganon's Castle.

**Implementation:**

Add after existing enum definitions:

```rust
/// Rainbow Bridge requirement mode.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub enum RainbowBridgeMode {
    /// Vanilla requirements
    #[default]
    Vanilla,
    /// Always open
    Open,
    /// Require medallions
    Medallions,
    /// Require spiritual stones
    Stones,
    /// Require dungeon rewards (medallions + stones)
    DungeonRewards,
    /// Require skulltula tokens
    Skulltulas,
    /// Require MM boss remains
    Remains,
    /// Custom requirements (uses special condition)
    Custom,
}

impl RainbowBridgeMode {
    /// Returns the string identifier used in config files.
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

    /// Parses a string identifier into a RainbowBridgeMode.
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
```

**Tests to add:**

```rust
#[test]
fn test_rainbow_bridge_mode() {
    assert_eq!(RainbowBridgeMode::default(), RainbowBridgeMode::Vanilla);
    assert_eq!(RainbowBridgeMode::Medallions.as_str(), "medallions");
    assert_eq!(RainbowBridgeMode::parse("medallions"), Some(RainbowBridgeMode::Medallions));
    assert_eq!(RainbowBridgeMode::parse("invalid"), None);
}
```

**Acceptance criteria:**
- [ ] Enum compiles with all variants
- [ ] Default is Vanilla
- [ ] as_str() and parse() are inverse operations
- [ ] Tests pass

---

### Step 2b: Add SongsMode Enum

**File to modify:** `/home/user/oottracker/crate/ootmm/src/settings.rs`

**Description:** Create an enum for song shuffle mode - where songs can be found.

**Implementation:**

```rust
/// Song shuffle mode.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub enum SongsMode {
    /// Songs only at song locations
    #[default]
    SongsOnly,
    /// Songs can be anywhere
    Anywhere,
    /// Songs at dungeon rewards
    DungeonRewards,
}

impl SongsMode {
    #[must_use]
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::SongsOnly => "songsOnly",
            Self::Anywhere => "anywhere",
            Self::DungeonRewards => "dungeonRewards",
        }
    }

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
```

**Tests to add:**

```rust
#[test]
fn test_songs_mode() {
    assert_eq!(SongsMode::default(), SongsMode::SongsOnly);
    assert_eq!(SongsMode::Anywhere.as_str(), "anywhere");
    assert_eq!(SongsMode::parse("anywhere"), Some(SongsMode::Anywhere));
}
```

**Acceptance criteria:**
- [ ] Enum compiles
- [ ] Serde works correctly
- [ ] Tests pass

---

### Step 2c: Add DungeonRewardShuffle Enum

**File to modify:** `/home/user/oottracker/crate/ootmm/src/settings.rs`

**Description:** Create an enum for dungeon reward shuffle mode.

**Implementation:**

```rust
/// Dungeon reward shuffle mode.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub enum DungeonRewardShuffle {
    /// Vanilla locations
    #[default]
    Vanilla,
    /// Rewards at dungeon blue warps
    DungeonBlueWarps,
    /// Rewards anywhere
    Anywhere,
}

impl DungeonRewardShuffle {
    #[must_use]
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::Vanilla => "vanilla",
            Self::DungeonBlueWarps => "dungeonBlueWarps",
            Self::Anywhere => "anywhere",
        }
    }

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
```

**Acceptance criteria:**
- [ ] Enum compiles with all variants
- [ ] Tests pass

---

### Step 2d: Add ShopShuffleMode Enum

**File to modify:** `/home/user/oottracker/crate/ootmm/src/settings.rs`

**Description:** Create an enum for shop shuffle mode.

**Implementation:**

```rust
/// Shop shuffle mode.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub enum ShopShuffleMode {
    /// No shop shuffle
    #[default]
    None,
    /// Full shop shuffle
    Full,
}

impl ShopShuffleMode {
    #[must_use]
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::None => "none",
            Self::Full => "full",
        }
    }

    #[must_use]
    pub fn parse(s: &str) -> Option<Self> {
        match s {
            "none" => Some(Self::None),
            "full" => Some(Self::Full),
            _ => None,
        }
    }
}
```

**Acceptance criteria:**
- [ ] Enum compiles
- [ ] Tests pass

---

### Step 2e: Add Fairy Shuffle Enums

**File to modify:** `/home/user/oottracker/crate/ootmm/src/settings.rs`

**Description:** Create enums for various fairy shuffle modes (town fairy, stray fairy chest, stray fairy other).

**Implementation:**

```rust
/// Town fairy shuffle mode.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub enum TownFairyShuffle {
    /// Vanilla
    #[default]
    Vanilla,
    /// Anywhere
    Anywhere,
}

impl TownFairyShuffle {
    #[must_use]
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::Vanilla => "vanilla",
            Self::Anywhere => "anywhere",
        }
    }

    #[must_use]
    pub fn parse(s: &str) -> Option<Self> {
        match s {
            "vanilla" => Some(Self::Vanilla),
            "anywhere" => Some(Self::Anywhere),
            _ => None,
        }
    }
}

/// Stray fairy shuffle mode.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub enum StrayFairyShuffle {
    /// Vanilla
    #[default]
    Vanilla,
    /// Starting
    Starting,
    /// Anywhere
    Anywhere,
    /// Own dungeon
    OwnDungeon,
}

impl StrayFairyShuffle {
    #[must_use]
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::Vanilla => "vanilla",
            Self::Starting => "starting",
            Self::Anywhere => "anywhere",
            Self::OwnDungeon => "ownDungeon",
        }
    }

    #[must_use]
    pub fn parse(s: &str) -> Option<Self> {
        match s {
            "vanilla" => Some(Self::Vanilla),
            "starting" => Some(Self::Starting),
            "anywhere" => Some(Self::Anywhere),
            "ownDungeon" => Some(Self::OwnDungeon),
            _ => None,
        }
    }
}
```

**Acceptance criteria:**
- [ ] Both enums compile
- [ ] Tests pass

---

### Step 2f: Add PriceMode Enum

**File to modify:** `/home/user/oottracker/crate/ootmm/src/settings.rs`

**Description:** Create an enum for price modes (shops, scrubs, merchants, tingle).

**Implementation:**

```rust
/// Price mode for shops, scrubs, and merchants.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub enum PriceMode {
    /// Vanilla prices
    #[default]
    Vanilla,
    /// Affordable prices
    Affordable,
    /// Random prices
    Random,
    /// Weighted random prices
    WeightedRandom,
}

impl PriceMode {
    #[must_use]
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::Vanilla => "vanilla",
            Self::Affordable => "affordable",
            Self::Random => "random",
            Self::WeightedRandom => "weightedRandom",
        }
    }

    #[must_use]
    pub fn parse(s: &str) -> Option<Self> {
        match s {
            "vanilla" => Some(Self::Vanilla),
            "affordable" => Some(Self::Affordable),
            "random" => Some(Self::Random),
            "weightedRandom" => Some(Self::WeightedRandom),
            _ => None,
        }
    }
}
```

**Acceptance criteria:**
- [ ] Enum compiles
- [ ] Tests pass

---

### Step 2g: Add CrossWarpMode Enum

**File to modify:** `/home/user/oottracker/crate/ootmm/src/settings.rs`

**Description:** Create an enum for cross-game warp modes.

**Implementation:**

```rust
/// Cross-game warp mode.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub enum CrossWarpMode {
    /// No cross-game warps
    #[default]
    None,
    /// Child dungeons only
    ChildDungeons,
    /// Full cross-game warps
    Full,
}

impl CrossWarpMode {
    #[must_use]
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::None => "none",
            Self::ChildDungeons => "childDungeons",
            Self::Full => "full",
        }
    }

    #[must_use]
    pub fn parse(s: &str) -> Option<Self> {
        match s {
            "none" => Some(Self::None),
            "childDungeons" => Some(Self::ChildDungeons),
            "full" => Some(Self::Full),
            _ => None,
        }
    }
}
```

**Acceptance criteria:**
- [ ] Enum compiles
- [ ] Tests pass

---

### Step 2h: Add CsmcMode Enum

**File to modify:** `/home/user/oottracker/crate/ootmm/src/settings.rs`

**Description:** Create an enum for Chest Size Matches Contents (CSMC) mode.

**Implementation:**

```rust
/// Chest Size Matches Contents mode.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub enum CsmcMode {
    /// CSMC disabled
    #[default]
    Never,
    /// CSMC always enabled
    Always,
    /// CSMC for major items only
    MajorOnly,
}

impl CsmcMode {
    #[must_use]
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::Never => "never",
            Self::Always => "always",
            Self::MajorOnly => "majorOnly",
        }
    }

    #[must_use]
    pub fn parse(s: &str) -> Option<Self> {
        match s {
            "never" => Some(Self::Never),
            "always" => Some(Self::Always),
            "majorOnly" => Some(Self::MajorOnly),
            _ => None,
        }
    }
}
```

**Acceptance criteria:**
- [ ] Enum compiles
- [ ] Tests pass

---

### Step 2i: Add ProgressiveMode Enum

**File to modify:** `/home/user/oottracker/crate/ootmm/src/settings.rs`

**Description:** Create an enum for progressive item modes (shields, swords, etc.).

**Implementation:**

```rust
/// Progressive item mode.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub enum ProgressiveMode {
    /// Items are separate
    #[default]
    Separate,
    /// Items are progressive
    Progressive,
    /// Items start at a specific tier (for swords: "goron")
    Goron,
}

impl ProgressiveMode {
    #[must_use]
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::Separate => "separate",
            Self::Progressive => "progressive",
            Self::Goron => "goron",
        }
    }

    #[must_use]
    pub fn parse(s: &str) -> Option<Self> {
        match s {
            "separate" => Some(Self::Separate),
            "progressive" => Some(Self::Progressive),
            "goron" => Some(Self::Goron),
            _ => None,
        }
    }
}
```

**Acceptance criteria:**
- [ ] Enum compiles
- [ ] Tests pass

---

### Step 2j: Add Miscellaneous Enums

**File to modify:** `/home/user/oottracker/crate/ootmm/src/settings.rs`

**Description:** Create enums for various miscellaneous settings.

**Implementation:**

```rust
/// Bombchu behavior mode.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub enum BombchuBehavior {
    /// Vanilla behavior
    #[default]
    Vanilla,
    /// Bag is separate item
    BagSeparate,
    /// Bag is shared
    BagShared,
}

/// Auto-invert mode for camera.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub enum AutoInvertMode {
    /// Never auto-invert
    #[default]
    Never,
    /// Always auto-invert
    Always,
    /// Auto-invert first person only
    FirstPerson,
}

/// Starting age.
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

/// Blast mask cooldown.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub enum BlastMaskCooldown {
    /// Default cooldown
    #[default]
    Default,
    /// Short cooldown
    Short,
    /// Very short cooldown
    VeryShort,
    /// Instant cooldown
    Instant,
}

/// Clock speed mode.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub enum ClockSpeed {
    /// Default clock speed
    #[default]
    Default,
    /// Slow clock
    Slow,
    /// Very slow clock
    VerySlow,
    /// Fast clock
    Fast,
    /// Very fast clock
    VeryFast,
    /// Super fast clock
    SuperFast,
}

/// Damage multiplier mode.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub enum DamageMultiplier {
    /// Normal damage
    #[default]
    Normal,
    /// Double damage
    Double,
    /// Quadruple damage
    Quadruple,
    /// OHKO (one-hit knockout)
    Ohko,
}

/// Zora King mode.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub enum ZoraKingMode {
    /// Vanilla
    #[default]
    Vanilla,
    /// King moved
    Open,
}

/// Gerudo Fortress mode.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub enum GerudoFortressMode {
    /// Vanilla
    #[default]
    Vanilla,
    /// Fast (one carpenter)
    Fast,
    /// Open (no carpenters)
    Open,
}

/// Item pool mode.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub enum ItemPool {
    /// Plentiful item pool
    #[default]
    Plentiful,
    /// Balanced item pool
    Balanced,
    /// Scarce item pool
    Scarce,
    /// Minimal item pool
    Minimal,
}

/// Traps quantity mode.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub enum TrapsQuantity {
    /// No traps
    #[default]
    None,
    /// Small amount of traps
    Small,
    /// Medium amount of traps
    Medium,
    /// Large amount of traps
    Large,
    /// Onslaught of traps
    Onslaught,
}
```

Add `as_str()` and `parse()` methods for each enum following the same pattern as previous enums.

**Acceptance criteria:**
- [ ] All enums compile
- [ ] Each has as_str() and parse() methods
- [ ] Tests pass

---

### Step 2k: Add ShuffleMode Enum

**File to modify:** `/home/user/oottracker/crate/ootmm/src/settings.rs`

**Description:** Create a generic shuffle mode enum used by many shuffle settings (pots, crates, grass, rocks, etc.).

**Implementation:**

```rust
/// Generic shuffle mode for various collectibles.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub enum ShuffleMode {
    /// No shuffle
    #[default]
    None,
    /// Shuffle overworld only
    Overworld,
    /// Shuffle dungeons only
    Dungeon,
    /// Shuffle all
    All,
}

impl ShuffleMode {
    #[must_use]
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::None => "none",
            Self::Overworld => "overworld",
            Self::Dungeon => "dungeon",
            Self::All => "all",
        }
    }

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

    /// Returns true if any shuffle is enabled.
    #[must_use]
    pub fn is_shuffled(&self) -> bool {
        !matches!(self, Self::None)
    }
}

/// Tingle shuffle mode.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub enum TingleShuffle {
    /// Vanilla (not shuffled)
    #[default]
    Vanilla,
    /// Starting (start with maps)
    Starting,
    /// Removed (no tingle maps)
    Removed,
    /// Anywhere (shuffled anywhere)
    Anywhere,
    /// Own region
    OwnRegion,
}

impl TingleShuffle {
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

/// Owl shuffle mode.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub enum OwlShuffle {
    /// No owl shuffle
    #[default]
    None,
    /// Owl items anywhere
    Anywhere,
}

impl OwlShuffle {
    #[must_use]
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::None => "none",
            Self::Anywhere => "anywhere",
        }
    }

    #[must_use]
    pub fn parse(s: &str) -> Option<Self> {
        match s {
            "none" => Some(Self::None),
            "anywhere" => Some(Self::Anywhere),
            _ => None,
        }
    }
}

/// Gold skulltula token shuffle mode.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub enum SkulltulaTokenShuffle {
    /// No shuffle
    #[default]
    None,
    /// Dungeons only
    Dungeons,
    /// Overworld only
    Overworld,
    /// All tokens
    All,
}

impl SkulltulaTokenShuffle {
    #[must_use]
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::None => "none",
            Self::Dungeons => "dungeons",
            Self::Overworld => "overworld",
            Self::All => "all",
        }
    }

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

/// Key shuffle modes (for small keys, boss keys).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub enum KeyShuffle {
    /// Vanilla
    #[default]
    Vanilla,
    /// Own dungeon
    OwnDungeon,
    /// Anywhere
    Anywhere,
    /// Removed
    Removed,
}

impl KeyShuffle {
    #[must_use]
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::Vanilla => "vanilla",
            Self::OwnDungeon => "ownDungeon",
            Self::Anywhere => "anywhere",
            Self::Removed => "removed",
        }
    }

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

/// Map/compass shuffle mode.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub enum MapCompassShuffle {
    /// Vanilla
    #[default]
    Vanilla,
    /// Starting inventory
    Starting,
    /// Own dungeon
    OwnDungeon,
    /// Anywhere
    Anywhere,
    /// Removed
    Removed,
}

impl MapCompassShuffle {
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
```

**Acceptance criteria:**
- [ ] All shuffle enums compile
- [ ] as_str() and parse() methods work
- [ ] Tests pass

---

## Phase 3: Expand RandomizerSettings

### Step 3a: Add Missing Boolean Settings

**File to modify:** `/home/user/oottracker/crate/ootmm/src/settings.rs`

**Description:** Add all missing boolean settings from the config file to RandomizerSettings. These are settings that evaluate to true/false.

**Implementation:**

Add these fields to the RandomizerSettings struct (grouped by category). See the full list of ~100 boolean settings in the detailed prompt below.

**Categories of boolean settings to add:**
- Shuffle Boolean Settings (scrubShuffleOot, cowShuffleOot, shuffleHivesOot, etc.)
- Souls Settings (soulsEnemyOot, soulsBossOot, soulsNpcOot, etc.)
- Shared Item Settings (sharedSpinUpgrade, sharedBows, sharedBombBags, etc.)
- Ageless Settings (agelessSwords, agelessShields, agelessTunics, etc.)
- Cross-game Settings (crossAge, crossGameFw)
- MM-specific Item Settings (spellFireMm, bootsIronMm, tunicGoronMm, etc.)
- QoL and Feature Settings (swordlessAdult, freeScarecrowOot, blueFireArrows, etc.)
- Hint Settings (generateSpoilerLog, probabilisticFoolish, hintImportance, etc.)
- Trap Settings (trapIce, trapFire, trapShock, cloakTraps, etc.)
- Misc Settings (clocks, menuNotebook, coins, voidWarpMm, etc.)

Also update:
1. The `Default` implementation to initialize all new fields to `false`
2. The `get_bool_setting()` method to return values for all new settings

**Acceptance criteria:**
- [ ] All new boolean fields added to struct
- [ ] Default impl updated
- [ ] get_bool_setting() updated
- [ ] Compiles and tests pass

---

### Step 3b: Add Missing Enum Settings

**File to modify:** `/home/user/oottracker/crate/ootmm/src/settings.rs`

**Dependencies:** Phase 2 enums must be complete.

**Description:** Add all missing enum-valued settings using the enums from Phase 2.

**Categories of enum settings to add:**
- Game Mode Settings (rainbow_bridge, songs, dungeon_reward_shuffle)
- Shop/Price Settings (shop_shuffle_oot/mm, price_oot_shops/scrubs/merchants, etc.)
- Fairy Shuffle Settings (town_fairy_shuffle, stray_fairy_chest_shuffle, etc.)
- Cross-warp Settings (cross_warp_oot, cross_warp_mm)
- CSMC Settings (csmc)
- Progressive Settings (progressive_shields_oot, progressive_swords_oot, etc.)
- Misc Enum Settings (bombchu_behavior, auto_invert, starting_age, etc.)
- Shuffle Mode Settings (gold_skulltula_tokens, tingle_shuffle, owl_shuffle, etc.)
- Entrance Randomizer Settings (er_boss, er_dungeons, er_indoors, etc.)
- Region/Boss Settings (region_state, stray_fairy_reward_count)

Also update `check_setting_value()` to handle all new enum settings.

**Acceptance criteria:**
- [ ] All enum settings added
- [ ] Default impl updated
- [ ] check_setting_value() handles all new enums
- [ ] Tests pass

---

### Steps 3c-3f: Add Special Conditions, Starting Items, Junk Locations, World Flags

**File to modify:** `/home/user/oottracker/crate/ootmm/src/settings.rs`

**Dependencies:** Steps 1a-1d must be complete.

**Description:** Add the special conditions, starting items, junk locations, and world flags sections to RandomizerSettings.

**Implementation:**

Add these fields to RandomizerSettings:

```rust
/// Special conditions for game progression gates.
/// Keys are: "BRIDGE", "MOON", "LACS", "GANON_BK", "MAJORA"
#[serde(default, rename = "specialConditions")]
pub special_conditions: HashMap<String, SpecialCondition>,

/// Starting inventory items.
#[serde(default, rename = "startingItems")]
pub starting_items: StartingItems,

/// Locations marked as junk (excluded from logic).
#[serde(default, rename = "junkLocations")]
pub junk_locations: JunkLocations,

/// World-level configuration flags.
#[serde(default, rename = "worldFlags")]
pub world_flags: WorldFlags,
```

Add helper methods:

```rust
impl RandomizerSettings {
    /// Gets a special condition by name.
    pub fn get_special_condition(&self, name: &str) -> Option<&SpecialCondition> {
        self.special_conditions.get(name)
    }

    /// Gets the BRIDGE special condition.
    pub fn bridge_condition(&self) -> Option<&SpecialCondition> {
        self.get_special_condition("BRIDGE")
    }

    /// Gets the MOON special condition.
    pub fn moon_condition(&self) -> Option<&SpecialCondition> {
        self.get_special_condition("MOON")
    }

    /// Gets the LACS special condition.
    pub fn lacs_condition(&self) -> Option<&SpecialCondition> {
        self.get_special_condition("LACS")
    }

    /// Gets the GANON_BK special condition.
    pub fn ganon_bk_condition(&self) -> Option<&SpecialCondition> {
        self.get_special_condition("GANON_BK")
    }

    /// Gets the MAJORA special condition.
    pub fn majora_condition(&self) -> Option<&SpecialCondition> {
        self.get_special_condition("MAJORA")
    }

    /// Checks if a location is marked as junk.
    pub fn is_junk_location(&self, location: &str) -> bool {
        self.junk_locations.is_junk(location)
    }

    /// Gets the starting quantity of an item.
    pub fn starting_item_quantity(&self, item: &str) -> u32 {
        self.starting_items.get_quantity(item)
    }

    /// Checks if player starts with an item.
    pub fn has_starting_item(&self, item: &str) -> bool {
        self.starting_items.has_item(item)
    }
}
```

**Acceptance criteria:**
- [ ] All new section fields added
- [ ] Helper methods work correctly
- [ ] Default impl updated
- [ ] Tests pass

---

## Phase 4: Config File Parser

### Steps 4a-4d: Create Config File Parser

**File to create:** `/home/user/oottracker/crate/ootmm/src/config_parser.rs`

**File to modify:** `/home/user/oottracker/crate/ootmm/src/lib.rs` (add module)

**Dependencies:** Phase 3 must be complete.

**Description:** Create a config file parser that can read the OoTMM randomizer YAML format and convert it into RandomizerSettings.

**Implementation:**

Create `config_parser.rs` with:

1. `OotmmConfigFile` struct matching the YAML structure
2. `RawSettings` struct for flexible parsing
3. `ConfigError` enum for error handling
4. `From<OotmmConfigFile> for RandomizerSettings` conversion
5. `RandomizerSettings::from_yaml_file()` method
6. `RandomizerSettings::from_yaml_str()` method

Update `lib.rs` to export the new module.

**Acceptance criteria:**
- [ ] Config file structure parses correctly
- [ ] Special conditions converted properly
- [ ] Starting items converted properly
- [ ] Junk locations converted properly
- [ ] World flags converted properly
- [ ] Main settings mapped to RandomizerSettings fields
- [ ] from_yaml_file() and from_yaml_str() work
- [ ] Sample config test passes
- [ ] Error handling for invalid configs

---

## Phase 5: Integration & Getters

### Steps 5a-5d: Update Getter Methods

**File to modify:** `/home/user/oottracker/crate/ootmm/src/settings.rs`

**Dependencies:** Phases 3 and 4 must be complete.

**Description:** Update get_bool_setting() and check_setting_value() to handle all new settings added in Phase 3.

**Implementation:**

Update `get_bool_setting()` to handle all ~100 new boolean settings.

Update `check_setting_value()` to handle all ~50 new enum settings.

**Acceptance criteria:**
- [ ] All new boolean settings accessible via get_bool_setting()
- [ ] All new enum settings accessible via check_setting_value()
- [ ] Tests pass

---

## Phase 6: Tests

### Steps 6a-6d: Comprehensive Tests

**Files to modify:**
- `/home/user/oottracker/crate/ootmm/src/settings.rs`
- `/home/user/oottracker/crate/ootmm/src/config_parser.rs`

**Dependencies:** Phase 5 must be complete.

**Description:** Add comprehensive unit tests and integration tests for all new types, settings, and parsing functionality.

**Test categories:**

1. **6a: Unit tests for new enum types** - Test default values, as_str(), parse(), and serde roundtrip for each enum
2. **6b: Unit tests for SpecialCondition, WorldFlags** - Test all fields and helper methods
3. **6c: Integration tests for config file parsing** - Test full config parsing, partial configs, and error cases
4. **6d: Round-trip serialization tests** - Test JSON and YAML serialization/deserialization

**Acceptance criteria:**
- [ ] All enum types have comprehensive tests
- [ ] SpecialCondition and WorldFlags fully tested
- [ ] Config parsing handles various input formats
- [ ] Round-trip serialization works for JSON and YAML
- [ ] Error cases are tested
- [ ] All tests pass

---

## Summary Table

| Phase | Steps | Est. Lines | Can Parallelize |
|-------|-------|------------|-----------------|
| 1 | 1a, 1b, 1c, 1d | ~300 | Yes (all 4) |
| 2 | 2a-2k | ~600 | Yes (all 11) |
| 3 | 3a, 3b, 3c-3f | ~400 | Partial |
| 4 | 4a-4d | ~200 | Sequential |
| 5 | 5a-5d | ~150 | Yes (all 4) |
| 6 | 6a-6d | ~400 | Yes (all 4) |

**Total: ~2000+ lines across 18 discrete steps**

Each step is self-contained with clear acceptance criteria and can be implemented/reviewed independently.
