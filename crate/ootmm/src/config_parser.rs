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
}
