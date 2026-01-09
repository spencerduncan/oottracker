//! Config file parser for OoTMM randomizer settings.
//!
//! This module provides functionality to parse YAML configuration files
//! into [`RandomizerSettings`]. The YAML format uses camelCase keys that
//! are mapped to snake_case Rust fields.
//!
//! # Example
//!
//! ```
//! use ootmm::config_parser::OotmmConfigFile;
//! use ootmm::settings::RandomizerSettings;
//!
//! let yaml = r#"
//! erMoon: true
//! openDungeonsOot: [DC, BotW]
//! ganonBossKey: custom
//! "#;
//!
//! let config = OotmmConfigFile::from_yaml_str(yaml).unwrap();
//! let settings: RandomizerSettings = config.into();
//! assert!(settings.er_moon);
//! ```

use std::fs;
use std::io;
use std::path::Path;

use serde::{Deserialize, Serialize};

use crate::settings::RandomizerSettings;

/// Errors that can occur when parsing config files.
#[derive(Debug)]
pub enum ConfigError {
    /// Error reading the config file from disk.
    Io(io::Error),
    /// Error parsing YAML content.
    Yaml(serde_yaml::Error),
}

impl std::fmt::Display for ConfigError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(err) => write!(f, "failed to read config file: {err}"),
            Self::Yaml(err) => write!(f, "failed to parse YAML: {err}"),
        }
    }
}

impl std::error::Error for ConfigError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Io(err) => Some(err),
            Self::Yaml(err) => Some(err),
        }
    }
}

impl From<io::Error> for ConfigError {
    fn from(err: io::Error) -> Self {
        Self::Io(err)
    }
}

impl From<serde_yaml::Error> for ConfigError {
    fn from(err: serde_yaml::Error) -> Self {
        Self::Yaml(err)
    }
}

/// Wrapper around the raw YAML configuration structure.
///
/// This struct represents the parsed YAML configuration file before
/// it is converted into [`RandomizerSettings`]. It provides a layer
/// of abstraction that can be used for validation or transformation.
///
/// The YAML format uses camelCase keys (e.g., `erMoon`, `openDungeonsOot`)
/// which are automatically mapped to the snake_case Rust fields in
/// [`RandomizerSettings`].
///
/// # Example
///
/// ```
/// use ootmm::config_parser::OotmmConfigFile;
/// use ootmm::settings::RandomizerSettings;
///
/// let yaml = r#"
/// skipZelda: true
/// logicMode: glitchless
/// bottleCount: 3
/// "#;
///
/// let config = OotmmConfigFile::from_yaml_str(yaml).unwrap();
/// let settings: RandomizerSettings = config.into();
/// assert!(settings.skip_zelda);
/// assert_eq!(settings.bottle_count, 3);
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct OotmmConfigFile {
    /// The underlying randomizer settings parsed from YAML.
    settings: RandomizerSettings,
}

impl OotmmConfigFile {
    /// Creates a new config file wrapper from existing settings.
    #[must_use]
    pub fn new(settings: RandomizerSettings) -> Self {
        Self { settings }
    }

    /// Parses a YAML string into an `OotmmConfigFile`.
    ///
    /// # Errors
    ///
    /// Returns a [`ConfigError::Yaml`] if the YAML is malformed or contains
    /// invalid values for the settings fields.
    ///
    /// # Example
    ///
    /// ```
    /// use ootmm::config_parser::OotmmConfigFile;
    ///
    /// let yaml = r#"
    /// erMoon: true
    /// openMaskShop: false
    /// "#;
    ///
    /// let config = OotmmConfigFile::from_yaml_str(yaml).unwrap();
    /// ```
    pub fn from_yaml_str(yaml: &str) -> Result<Self, ConfigError> {
        let settings: RandomizerSettings = serde_yaml::from_str(yaml)?;
        Ok(Self { settings })
    }

    /// Parses a YAML file into an `OotmmConfigFile`.
    ///
    /// # Errors
    ///
    /// Returns a [`ConfigError::Io`] if the file cannot be read, or
    /// [`ConfigError::Yaml`] if the content is malformed.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use ootmm::config_parser::OotmmConfigFile;
    ///
    /// let config = OotmmConfigFile::from_yaml_file("settings.yaml").unwrap();
    /// ```
    pub fn from_yaml_file<P: AsRef<Path>>(path: P) -> Result<Self, ConfigError> {
        let content = fs::read_to_string(path)?;
        Self::from_yaml_str(&content)
    }

    /// Returns a reference to the underlying settings.
    #[must_use]
    pub fn settings(&self) -> &RandomizerSettings {
        &self.settings
    }

    /// Consumes the config file and returns the underlying settings.
    #[must_use]
    pub fn into_settings(self) -> RandomizerSettings {
        self.settings
    }

    /// Serializes the config to a YAML string.
    ///
    /// # Errors
    ///
    /// Returns a [`ConfigError::Yaml`] if serialization fails.
    pub fn to_yaml_string(&self) -> Result<String, ConfigError> {
        Ok(serde_yaml::to_string(&self.settings)?)
    }
}

impl From<OotmmConfigFile> for RandomizerSettings {
    /// Converts the config file wrapper into `RandomizerSettings`.
    ///
    /// This conversion is infallible since the YAML has already been
    /// validated during parsing.
    fn from(config: OotmmConfigFile) -> Self {
        config.settings
    }
}

impl From<RandomizerSettings> for OotmmConfigFile {
    /// Creates a config file wrapper from existing settings.
    fn from(settings: RandomizerSettings) -> Self {
        Self::new(settings)
    }
}

impl Default for OotmmConfigFile {
    /// Creates a config file with default settings.
    fn default() -> Self {
        Self::new(RandomizerSettings::default())
    }
}

// Extension methods for RandomizerSettings
impl RandomizerSettings {
    /// Parses randomizer settings from a YAML string.
    ///
    /// This is a convenience method that wraps [`OotmmConfigFile::from_yaml_str`].
    ///
    /// # Errors
    ///
    /// Returns a [`ConfigError::Yaml`] if the YAML is malformed or contains
    /// invalid values for the settings fields.
    ///
    /// # Example
    ///
    /// ```
    /// use ootmm::settings::RandomizerSettings;
    ///
    /// let yaml = r#"
    /// erMoon: true
    /// openDungeonsOot: [DC, BotW]
    /// logicMode: glitchless
    /// "#;
    ///
    /// let settings = RandomizerSettings::from_yaml_str(yaml).unwrap();
    /// assert!(settings.er_moon);
    /// ```
    pub fn from_yaml_str(yaml: &str) -> Result<Self, ConfigError> {
        OotmmConfigFile::from_yaml_str(yaml).map(|c| c.into())
    }

    /// Parses randomizer settings from a YAML file.
    ///
    /// This is a convenience method that wraps [`OotmmConfigFile::from_yaml_file`].
    ///
    /// # Errors
    ///
    /// Returns a [`ConfigError::Io`] if the file cannot be read, or
    /// [`ConfigError::Yaml`] if the content is malformed.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use ootmm::settings::RandomizerSettings;
    ///
    /// let settings = RandomizerSettings::from_yaml_file("settings.yaml").unwrap();
    /// ```
    pub fn from_yaml_file<P: AsRef<Path>>(path: P) -> Result<Self, ConfigError> {
        OotmmConfigFile::from_yaml_file(path).map(|c| c.into())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::settings::{
        GanonBossKeyMode, LogicMode, MmDungeon, MqDungeon, OotDungeon, StartingAge,
    };

    #[test]
    fn test_parse_empty_yaml() {
        let yaml = "";
        let config = OotmmConfigFile::from_yaml_str(yaml).unwrap();
        let settings: RandomizerSettings = config.into();
        // Should use all defaults
        assert!(!settings.er_moon);
        assert!(!settings.skip_zelda);
        assert_eq!(settings.bottle_count, 4);
    }

    #[test]
    fn test_parse_boolean_settings() {
        let yaml = r#"
erMoon: true
skipZelda: true
agelessBoots: true
openMaskShop: false
"#;
        let settings = RandomizerSettings::from_yaml_str(yaml).unwrap();
        assert!(settings.er_moon);
        assert!(settings.skip_zelda);
        assert!(settings.ageless_boots);
        assert!(!settings.open_mask_shop);
    }

    #[test]
    fn test_parse_enum_settings() {
        let yaml = r#"
ganonBossKey: custom
logicMode: glitchless
startingAge: adult
"#;
        let settings = RandomizerSettings::from_yaml_str(yaml).unwrap();
        assert_eq!(settings.ganon_boss_key, GanonBossKeyMode::Custom);
        assert_eq!(settings.logic_mode, LogicMode::Glitchless);
        assert_eq!(settings.starting_age, StartingAge::Adult);
    }

    #[test]
    fn test_parse_hashset_settings() {
        let yaml = r#"
openDungeonsOot: [DC, BotW, JJ]
openDungeonsMm: [ST, WF]
mqDungeons: [deku_tree, forest_temple]
"#;
        let settings = RandomizerSettings::from_yaml_str(yaml).unwrap();
        assert!(settings
            .open_dungeons_oot
            .contains(&OotDungeon::DodongosCavern));
        assert!(settings
            .open_dungeons_oot
            .contains(&OotDungeon::BottomOfTheWell));
        assert!(settings.open_dungeons_oot.contains(&OotDungeon::JabuJabu));
        assert!(settings.open_dungeons_mm.contains(&MmDungeon::StoneTower));
        assert!(settings.open_dungeons_mm.contains(&MmDungeon::Woodfall));
        assert!(settings.mq_dungeons.contains(&MqDungeon::DekuTree));
        assert!(settings.mq_dungeons.contains(&MqDungeon::ForestTemple));
    }

    #[test]
    fn test_parse_numeric_settings() {
        let yaml = r#"
bottleCount: 2
"#;
        let settings = RandomizerSettings::from_yaml_str(yaml).unwrap();
        assert_eq!(settings.bottle_count, 2);
    }

    #[test]
    fn test_parse_special_conditions() {
        let yaml = r#"
specialConditions:
  bridge:
    medallions: 6
    stones: 3
  lacs:
    dungeonRewards: 9
"#;
        let settings = RandomizerSettings::from_yaml_str(yaml).unwrap();
        let bridge = settings.special_conditions.get("bridge").unwrap();
        assert_eq!(bridge.medallions, 6);
        assert_eq!(bridge.stones, 3);
        let lacs = settings.special_conditions.get("lacs").unwrap();
        assert_eq!(lacs.dungeon_rewards, 9);
    }

    #[test]
    fn test_parse_starting_items() {
        let yaml = r#"
startingItems:
  Sword: 1
  Shield: 2
  Bow: 1
"#;
        let settings = RandomizerSettings::from_yaml_str(yaml).unwrap();
        assert_eq!(settings.starting_items.get("Sword"), Some(&1));
        assert_eq!(settings.starting_items.get("Shield"), Some(&2));
        assert_eq!(settings.starting_items.get("Bow"), Some(&1));
    }

    #[test]
    fn test_parse_junk_locations() {
        let yaml = r#"
junkLocations:
  - oot_kokiri_forest_chest
  - mm_clock_town_chest
"#;
        let settings = RandomizerSettings::from_yaml_str(yaml).unwrap();
        assert!(settings.junk_locations.contains("oot_kokiri_forest_chest"));
        assert!(settings.junk_locations.contains("mm_clock_town_chest"));
    }

    #[test]
    fn test_parse_world_flags() {
        let yaml = r#"
worldFlags:
  ootEnabled: true
  mmEnabled: false
  sharedItems: true
"#;
        let settings = RandomizerSettings::from_yaml_str(yaml).unwrap();
        assert!(settings.world_flags.oot_enabled);
        assert!(!settings.world_flags.mm_enabled);
        assert!(settings.world_flags.shared_items);
    }

    #[test]
    fn test_parse_logic_tricks() {
        let yaml = r#"
logicTricks:
  - OOT_LENS
  - OOT_HOOKSHOT_JUMP
  - MM_GORON_BOMB_JUMP
"#;
        let settings = RandomizerSettings::from_yaml_str(yaml).unwrap();
        assert!(settings.logic_tricks.contains("OOT_LENS"));
        assert!(settings.logic_tricks.contains("OOT_HOOKSHOT_JUMP"));
        assert!(settings.logic_tricks.contains("MM_GORON_BOMB_JUMP"));
    }

    #[test]
    fn test_invalid_yaml_syntax() {
        let yaml = r#"
erMoon: [invalid yaml
"#;
        let result = RandomizerSettings::from_yaml_str(yaml);
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), ConfigError::Yaml(_)));
    }

    #[test]
    fn test_invalid_enum_value() {
        let yaml = r#"
ganonBossKey: invalidValue
"#;
        let result = RandomizerSettings::from_yaml_str(yaml);
        assert!(result.is_err());
    }

    #[test]
    fn test_config_file_to_settings_conversion() {
        let yaml = r#"
erMoon: true
"#;
        let config = OotmmConfigFile::from_yaml_str(yaml).unwrap();
        assert!(config.settings().er_moon);

        let settings: RandomizerSettings = config.into();
        assert!(settings.er_moon);
    }

    #[test]
    fn test_settings_to_config_file_conversion() {
        let mut settings = RandomizerSettings::default();
        settings.er_moon = true;

        let config: OotmmConfigFile = settings.into();
        assert!(config.settings().er_moon);
    }

    #[test]
    fn test_roundtrip_serialization() {
        let yaml = r#"
erMoon: true
skipZelda: true
openDungeonsOot:
  - DC
  - BotW
ganonBossKey: custom
bottleCount: 3
"#;
        let settings = RandomizerSettings::from_yaml_str(yaml).unwrap();
        let config: OotmmConfigFile = settings.clone().into();

        let yaml_output = config.to_yaml_string().unwrap();
        let settings2 = RandomizerSettings::from_yaml_str(&yaml_output).unwrap();

        assert_eq!(settings.er_moon, settings2.er_moon);
        assert_eq!(settings.skip_zelda, settings2.skip_zelda);
        assert_eq!(settings.ganon_boss_key, settings2.ganon_boss_key);
        assert_eq!(settings.bottle_count, settings2.bottle_count);
        assert_eq!(settings.open_dungeons_oot, settings2.open_dungeons_oot);
    }

    #[test]
    fn test_default_config_file() {
        let config = OotmmConfigFile::default();
        let settings = config.into_settings();
        assert_eq!(settings.bottle_count, 4);
        assert!(!settings.er_moon);
    }

    #[test]
    fn test_config_error_display() {
        let io_err = ConfigError::Io(io::Error::new(io::ErrorKind::NotFound, "file not found"));
        assert!(io_err.to_string().contains("failed to read config file"));

        let yaml_err = ConfigError::from(
            serde_yaml::from_str::<RandomizerSettings>("invalid: [").unwrap_err(),
        );
        assert!(yaml_err.to_string().contains("failed to parse YAML"));
    }

    #[test]
    fn test_comprehensive_settings() {
        // Test a comprehensive set of settings to ensure all field types work
        let yaml = r#"
# Boolean settings
erMoon: true
erIndoorsMajor: true
openMaskShop: true
skipZelda: true
scrubShuffleOot: true
cowShuffleMm: true
soulsEnemyOot: true
sharedBows: true
sharedWallets: true
agelessSwords: true
crossAge: true
spellFireMm: true
swordlessAdult: true
blueFireArrows: true
trapIce: true
clocks: true

# Enum settings
dekuTree: open
doorOfTime: open
kakarikoGate: open
ganonBossKey: removed
lacs: custom
majoraChild: none
moonCrash: cycle
ageChange: oot
climbMostSurfacesOot: off
hookshotAnywhereOot: off
beneathWell: open
erOverworld: full
erGrottos: full
bossWarpPads: remains
smallKeyShuffleOot: anywhere
shufflePotsMm: all
logicMode: glitched
rainbowBridge: medallions
songs: anywhere
dungeonRewardShuffle: anywhere
shopShuffleOot: all
shopShuffleMm: ownShop
priceOotShops: random
townFairyShuffle: anywhere
strayFairyChestShuffle: anywhere
crossWarpOot: full
csmc: always
bombchuBehavior: asBombs
autoInvert: always
startingAge: random
damageMultiplier: double
itemPool: scarce
trapsQuantity: many

# Collection settings
openDungeonsOot: [DC, shadow, water]
openDungeonsMm: [ST]
mqDungeons: [deku_tree, fire_temple, water_temple]
clearStateDungeonsMm: [WF]
jpLayouts: [greatBayCoast, stoneTower]
logicTricks: [TRICK_1, TRICK_2]

# Numeric
bottleCount: 2

# Complex types
specialConditions:
  bridge:
    medallions: 6
    stones: 3
    remains: 4
startingItems:
  Kokiri_Sword: 1
  Deku_Shield: 1
junkLocations:
  - oot_chest_1
  - mm_chest_2
worldFlags:
  ootEnabled: true
  mmEnabled: true
  sharedItems: true
  sharedMasks: true
"#;
        let settings = RandomizerSettings::from_yaml_str(yaml).unwrap();

        // Verify boolean settings
        assert!(settings.er_moon);
        assert!(settings.er_indoors_major);
        assert!(settings.skip_zelda);
        assert!(settings.shared_bows);
        assert!(settings.blue_fire_arrows);

        // Verify enum settings
        assert_eq!(settings.ganon_boss_key, GanonBossKeyMode::Removed);
        assert_eq!(settings.logic_mode, LogicMode::Glitched);
        assert_eq!(settings.starting_age, StartingAge::Random);

        // Verify collections
        assert!(settings
            .open_dungeons_oot
            .contains(&OotDungeon::DodongosCavern));
        assert!(settings.open_dungeons_oot.contains(&OotDungeon::Shadow));
        assert!(settings.open_dungeons_oot.contains(&OotDungeon::Water));
        assert!(settings.mq_dungeons.contains(&MqDungeon::DekuTree));
        assert!(settings.logic_tricks.contains("TRICK_1"));

        // Verify numeric
        assert_eq!(settings.bottle_count, 2);

        // Verify complex types
        let bridge = settings.special_conditions.get("bridge").unwrap();
        assert_eq!(bridge.medallions, 6);
        assert!(settings.starting_items.contains_key("Kokiri_Sword"));
        assert!(settings.junk_locations.contains("oot_chest_1"));
        assert!(settings.world_flags.shared_items);
    }

    // === Additional Integration Tests ===

    #[test]
    fn test_parse_partial_config_with_defaults() {
        // Only set a few fields, rest should use defaults
        let yaml = r#"
erMoon: true
logicMode: glitched
"#;
        let settings = RandomizerSettings::from_yaml_str(yaml).unwrap();

        // Set values
        assert!(settings.er_moon);
        assert_eq!(settings.logic_mode, LogicMode::Glitched);

        // Default values
        assert!(!settings.skip_zelda);
        assert!(!settings.ageless_boots);
        assert_eq!(settings.bottle_count, 4);
        assert_eq!(settings.ganon_boss_key, GanonBossKeyMode::Vanilla);
        assert!(settings.open_dungeons_oot.is_empty());
        assert!(settings.starting_items.is_empty());
    }

    #[test]
    fn test_parse_empty_collections() {
        let yaml = r#"
openDungeonsOot: []
mqDungeons: []
logicTricks: []
startingItems: {}
junkLocations: []
"#;
        let settings = RandomizerSettings::from_yaml_str(yaml).unwrap();

        assert!(settings.open_dungeons_oot.is_empty());
        assert!(settings.mq_dungeons.is_empty());
        assert!(settings.logic_tricks.is_empty());
        assert!(settings.starting_items.is_empty());
        assert!(settings.junk_locations.is_empty());
    }

    #[test]
    fn test_parse_single_item_collections() {
        let yaml = r#"
openDungeonsOot:
  - DC
mqDungeons:
  - forest_temple
logicTricks:
  - ONE_TRICK
startingItems:
  SingleItem: 1
junkLocations:
  - single_location
"#;
        let settings = RandomizerSettings::from_yaml_str(yaml).unwrap();

        assert_eq!(settings.open_dungeons_oot.len(), 1);
        assert!(settings
            .open_dungeons_oot
            .contains(&OotDungeon::DodongosCavern));
        assert_eq!(settings.mq_dungeons.len(), 1);
        assert_eq!(settings.logic_tricks.len(), 1);
        assert_eq!(settings.starting_items.len(), 1);
        assert_eq!(settings.junk_locations.len(), 1);
    }

    #[test]
    fn test_parse_invalid_dungeon_value() {
        let yaml = r#"
openDungeonsOot:
  - InvalidDungeon
"#;
        // Invalid dungeon values should cause an error
        let result = RandomizerSettings::from_yaml_str(yaml);
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_wrong_type_for_boolean() {
        let yaml = r#"
erMoon: "not a boolean"
"#;
        let result = RandomizerSettings::from_yaml_str(yaml);
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_wrong_type_for_numeric() {
        let yaml = r#"
bottleCount: "three"
"#;
        let result = RandomizerSettings::from_yaml_str(yaml);
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_negative_numeric() {
        // YAML parser might accept negative values, but the field is u8
        // Let's see how it's handled
        let yaml = r#"
bottleCount: -1
"#;
        let result = RandomizerSettings::from_yaml_str(yaml);
        // This should fail because u8 cannot be negative
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_yaml_with_comments() {
        let yaml = r#"
# This is a comment
erMoon: true  # inline comment
# Another comment
skipZelda: true
"#;
        let settings = RandomizerSettings::from_yaml_str(yaml).unwrap();
        assert!(settings.er_moon);
        assert!(settings.skip_zelda);
    }

    #[test]
    fn test_parse_yaml_with_anchors_and_aliases() {
        let yaml = r#"
# Define an anchor
commonSettings: &common
  erMoon: true
  skipZelda: true

# Note: YAML anchors work at document level, so we just test basic parsing
erMoon: true
skipZelda: true
"#;
        let settings = RandomizerSettings::from_yaml_str(yaml).unwrap();
        assert!(settings.er_moon);
        assert!(settings.skip_zelda);
    }

    #[test]
    fn test_parse_multiline_yaml() {
        let yaml = r#"
openDungeonsOot:
  - DC
  - BotW
  - JJ
openDungeonsMm:
  - ST
  - WF
"#;
        let settings = RandomizerSettings::from_yaml_str(yaml).unwrap();
        assert_eq!(settings.open_dungeons_oot.len(), 3);
        assert_eq!(settings.open_dungeons_mm.len(), 2);
    }

    #[test]
    fn test_parse_flow_style_yaml() {
        let yaml = r#"
openDungeonsOot: [DC, BotW, JJ]
openDungeonsMm: [ST, WF]
startingItems: {Sword: 1, Shield: 2}
"#;
        let settings = RandomizerSettings::from_yaml_str(yaml).unwrap();
        assert_eq!(settings.open_dungeons_oot.len(), 3);
        assert_eq!(settings.open_dungeons_mm.len(), 2);
        assert_eq!(settings.starting_items.len(), 2);
    }

    #[test]
    fn test_parse_mixed_case_sensitivity() {
        // YAML keys are case-sensitive, so this should use defaults
        // for incorrectly cased keys
        let yaml = r#"
ErMoon: true
SKIPZELDA: true
"#;
        let result = RandomizerSettings::from_yaml_str(yaml);
        // Unknown fields should be ignored by serde
        // If they are not, this might error or use defaults
        if let Ok(settings) = result {
            // These won't be set because keys are wrong case
            assert!(!settings.er_moon);
            assert!(!settings.skip_zelda);
        }
    }

    #[test]
    fn test_parse_special_condition_partial() {
        let yaml = r#"
specialConditions:
  bridge:
    medallions: 6
  lacs:
    stones: 3
    skulltulas: 100
"#;
        let settings = RandomizerSettings::from_yaml_str(yaml).unwrap();

        let bridge = settings.special_conditions.get("bridge").unwrap();
        assert_eq!(bridge.medallions, 6);
        assert_eq!(bridge.stones, 0); // default
        assert_eq!(bridge.remains, 0); // default

        let lacs = settings.special_conditions.get("lacs").unwrap();
        assert_eq!(lacs.stones, 3);
        assert_eq!(lacs.skulltulas, 100);
        assert_eq!(lacs.medallions, 0); // default
    }

    #[test]
    fn test_parse_world_flags_partial() {
        let yaml = r#"
worldFlags:
  mmEnabled: false
"#;
        let settings = RandomizerSettings::from_yaml_str(yaml).unwrap();

        // mmEnabled was explicitly set
        assert!(!settings.world_flags.mm_enabled);
        // ootEnabled should default to true
        assert!(settings.world_flags.oot_enabled);
        // shared_items should default to false
        assert!(!settings.world_flags.shared_items);
    }

    #[test]
    fn test_config_file_new_and_settings_access() {
        let mut settings = RandomizerSettings::default();
        settings.er_moon = true;
        settings.bottle_count = 3;

        let config = OotmmConfigFile::new(settings);

        // Test settings() accessor
        assert!(config.settings().er_moon);
        assert_eq!(config.settings().bottle_count, 3);

        // Test into_settings()
        let retrieved = config.into_settings();
        assert!(retrieved.er_moon);
        assert_eq!(retrieved.bottle_count, 3);
    }

    #[test]
    fn test_yaml_to_yaml_roundtrip() {
        let yaml = r#"
erMoon: true
skipZelda: true
logicMode: glitched
bottleCount: 2
openDungeonsOot:
  - DC
  - BotW
"#;
        let settings = RandomizerSettings::from_yaml_str(yaml).unwrap();
        let config: OotmmConfigFile = settings.into();

        let yaml_output = config.to_yaml_string().unwrap();

        // Parse the output again
        let settings2 = RandomizerSettings::from_yaml_str(&yaml_output).unwrap();

        assert!(settings2.er_moon);
        assert!(settings2.skip_zelda);
        assert_eq!(settings2.logic_mode, LogicMode::Glitched);
        assert_eq!(settings2.bottle_count, 2);
        assert!(settings2
            .open_dungeons_oot
            .contains(&OotDungeon::DodongosCavern));
        assert!(settings2
            .open_dungeons_oot
            .contains(&OotDungeon::BottomOfTheWell));
    }

    #[test]
    fn test_config_error_source() {
        use std::error::Error;

        let io_err = ConfigError::Io(io::Error::new(io::ErrorKind::NotFound, "file not found"));
        assert!(io_err.source().is_some());

        let yaml_err = ConfigError::from(
            serde_yaml::from_str::<RandomizerSettings>("invalid: [").unwrap_err(),
        );
        assert!(yaml_err.source().is_some());
    }

    #[test]
    fn test_parse_all_dungeon_rewards_shuffle_modes() {
        use crate::settings::DungeonRewardShuffle;

        for (yaml_val, expected) in [
            ("vanilla", DungeonRewardShuffle::Vanilla),
            ("dungeonBlueWarps", DungeonRewardShuffle::DungeonBlueWarps),
            ("anywhere", DungeonRewardShuffle::Anywhere),
        ] {
            let yaml = format!("dungeonRewardShuffle: {}", yaml_val);
            let settings = RandomizerSettings::from_yaml_str(&yaml).unwrap();
            assert_eq!(settings.dungeon_reward_shuffle, expected);
        }
    }

    #[test]
    fn test_parse_all_rainbow_bridge_modes() {
        use crate::settings::RainbowBridgeMode;

        for (yaml_val, expected) in [
            ("vanilla", RainbowBridgeMode::Vanilla),
            ("open", RainbowBridgeMode::Open),
            ("medallions", RainbowBridgeMode::Medallions),
            ("stones", RainbowBridgeMode::Stones),
            ("dungeonRewards", RainbowBridgeMode::DungeonRewards),
            ("skulltulas", RainbowBridgeMode::Skulltulas),
            ("remains", RainbowBridgeMode::Remains),
            ("custom", RainbowBridgeMode::Custom),
        ] {
            let yaml = format!("rainbowBridge: {}", yaml_val);
            let settings = RandomizerSettings::from_yaml_str(&yaml).unwrap();
            assert_eq!(settings.rainbow_bridge, expected);
        }
    }

    #[test]
    fn test_parse_all_damage_multiplier_modes() {
        use crate::settings::DamageMultiplier;

        for (yaml_val, expected) in [
            ("half", DamageMultiplier::Half),
            ("normal", DamageMultiplier::Normal),
            ("double", DamageMultiplier::Double),
            ("quadruple", DamageMultiplier::Quadruple),
            ("ohko", DamageMultiplier::Ohko),
        ] {
            let yaml = format!("damageMultiplier: {}", yaml_val);
            let settings = RandomizerSettings::from_yaml_str(&yaml).unwrap();
            assert_eq!(settings.damage_multiplier, expected);
        }
    }

    #[test]
    fn test_parse_all_item_pool_modes() {
        use crate::settings::ItemPool;

        for (yaml_val, expected) in [
            ("plentiful", ItemPool::Plentiful),
            ("normal", ItemPool::Normal),
            ("scarce", ItemPool::Scarce),
            ("minimal", ItemPool::Minimal),
        ] {
            let yaml = format!("itemPool: {}", yaml_val);
            let settings = RandomizerSettings::from_yaml_str(&yaml).unwrap();
            assert_eq!(settings.item_pool, expected);
        }
    }

    #[test]
    fn test_parse_all_traps_quantity_modes() {
        use crate::settings::TrapsQuantity;

        for (yaml_val, expected) in [
            ("none", TrapsQuantity::None),
            ("few", TrapsQuantity::Few),
            ("normal", TrapsQuantity::Normal),
            ("many", TrapsQuantity::Many),
            ("onslaught", TrapsQuantity::Onslaught),
        ] {
            let yaml = format!("trapsQuantity: {}", yaml_val);
            let settings = RandomizerSettings::from_yaml_str(&yaml).unwrap();
            assert_eq!(settings.traps_quantity, expected);
        }
    }

    #[test]
    fn test_parse_all_mq_dungeons() {
        let yaml = r#"
mqDungeons:
  - deku_tree
  - dodongos_cavern
  - jabu_jabu
  - forest_temple
  - fire_temple
  - water_temple
  - spirit_temple
  - shadow_temple
  - bottom_of_the_well
  - ice_cavern
  - gerudo_training_ground
  - ganons_castle
"#;
        let settings = RandomizerSettings::from_yaml_str(yaml).unwrap();
        assert_eq!(settings.mq_dungeons.len(), 12);
        assert!(settings.mq_dungeons.contains(&MqDungeon::DekuTree));
        assert!(settings.mq_dungeons.contains(&MqDungeon::DodongosCavern));
        assert!(settings.mq_dungeons.contains(&MqDungeon::JabuJabu));
        assert!(settings.mq_dungeons.contains(&MqDungeon::ForestTemple));
        assert!(settings.mq_dungeons.contains(&MqDungeon::FireTemple));
        assert!(settings.mq_dungeons.contains(&MqDungeon::WaterTemple));
        assert!(settings.mq_dungeons.contains(&MqDungeon::SpiritTemple));
        assert!(settings.mq_dungeons.contains(&MqDungeon::ShadowTemple));
        assert!(settings.mq_dungeons.contains(&MqDungeon::BottomOfTheWell));
        assert!(settings.mq_dungeons.contains(&MqDungeon::IceCavern));
        assert!(settings
            .mq_dungeons
            .contains(&MqDungeon::GerudoTrainingGround));
        assert!(settings.mq_dungeons.contains(&MqDungeon::GanonsCastle));
    }

    #[test]
    fn test_parse_large_starting_items() {
        let yaml = r#"
startingItems:
  Bombs: 99
  Arrows: 50
  Rupees: 500
  DekuNuts: 40
  DekuSticks: 30
"#;
        let settings = RandomizerSettings::from_yaml_str(yaml).unwrap();
        assert_eq!(settings.starting_items.get("Bombs"), Some(&99));
        assert_eq!(settings.starting_items.get("Arrows"), Some(&50));
        assert_eq!(settings.starting_items.get("Rupees"), Some(&500));
    }

    #[test]
    fn test_parse_many_junk_locations() {
        let yaml = r#"
junkLocations:
  - location_1
  - location_2
  - location_3
  - location_4
  - location_5
  - location_6
  - location_7
  - location_8
  - location_9
  - location_10
"#;
        let settings = RandomizerSettings::from_yaml_str(yaml).unwrap();
        assert_eq!(settings.junk_locations.len(), 10);
    }

    #[test]
    fn test_parse_many_logic_tricks() {
        let yaml = r#"
logicTricks:
  - OOT_LENS
  - OOT_HOOKSHOT_JUMP
  - OOT_BOMB_HOVER
  - MM_GORON_ROLL
  - MM_ZORA_JUMP
  - OOT_ADULT_KOKIRI
  - MM_DEKU_SKIP
"#;
        let settings = RandomizerSettings::from_yaml_str(yaml).unwrap();
        assert_eq!(settings.logic_tricks.len(), 7);
        assert!(settings.logic_tricks.contains("OOT_LENS"));
        assert!(settings.logic_tricks.contains("MM_GORON_ROLL"));
    }

    #[test]
    fn test_json_and_yaml_consistency() {
        // Create settings via YAML
        let yaml = r#"
erMoon: true
bottleCount: 2
logicMode: glitched
openDungeonsOot:
  - DC
"#;
        let settings_yaml = RandomizerSettings::from_yaml_str(yaml).unwrap();

        // Serialize to JSON and back
        let json = serde_json::to_string(&settings_yaml).unwrap();
        let settings_json: RandomizerSettings = serde_json::from_str(&json).unwrap();

        // They should match
        assert_eq!(settings_yaml.er_moon, settings_json.er_moon);
        assert_eq!(settings_yaml.bottle_count, settings_json.bottle_count);
        assert_eq!(settings_yaml.logic_mode, settings_json.logic_mode);
        assert_eq!(
            settings_yaml.open_dungeons_oot,
            settings_json.open_dungeons_oot
        );
    }
}
