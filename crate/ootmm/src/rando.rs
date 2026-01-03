//! Implementation of the `ootr::Rando` trait for OoTMM data.
//!
//! This module provides `OotmmRando`, which wraps a `WorldDatabase` and implements
//! the `ootr::Rando` trait, allowing OoTMM world data to be used with the common
//! randomizer infrastructure.

use std::{
    collections::{HashMap, HashSet},
    fmt,
    sync::Arc,
};

use ootr::{item::Item as OotrItem, region::Region as OotrRegion, Rando, RandoErr, Regions};

use crate::{embedded_data, error::Error, world_database::WorldDatabase};

/// Error type for OotmmRando operations.
#[derive(Debug, Clone)]
pub struct OotmmRandoError {
    message: String,
}

impl OotmmRandoError {
    /// Creates a new error with the given message.
    pub fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
        }
    }
}

impl fmt::Display for OotmmRandoError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl RandoErr for OotmmRandoError {
    const ITEM_NOT_FOUND: Self = OotmmRandoError {
        message: String::new(),
    };
}

impl From<Error> for OotmmRandoError {
    fn from(err: Error) -> Self {
        Self::new(err.to_string())
    }
}

/// Region name type for OoTMM.
///
/// This type wraps a String and implements all the traits required by `ootr::Rando::RegionName`.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct OotmmRegionName(String);

impl OotmmRegionName {
    /// Creates a new region name from a string.
    pub fn new(name: impl Into<String>) -> Self {
        Self(name.into())
    }
}

impl From<&'static str> for OotmmRegionName {
    fn from(s: &'static str) -> Self {
        Self(s.to_string())
    }
}

impl AsRef<str> for OotmmRegionName {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl PartialEq<&str> for OotmmRegionName {
    fn eq(&self, other: &&str) -> bool {
        self.0.as_str() == *other
    }
}

impl fmt::Display for OotmmRegionName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// OoTMM Rando implementation.
///
/// This struct wraps a `WorldDatabase` and provides access to OoTMM randomizer data
/// through the `ootr::Rando` trait.
///
/// # Example
///
/// ```
/// use ootmm::rando::OotmmRando;
/// use ootr::Rando;
///
/// let rando = OotmmRando::new().expect("Failed to create OotmmRando");
/// let items = rando.item_table().expect("Failed to get item table");
/// println!("Loaded {} items", items.len());
/// ```
pub struct OotmmRando {
    /// The world database containing all region/location data.
    world_db: WorldDatabase,
    /// Cached item table (maps item names to ootr Items).
    item_table: Arc<HashMap<String, OotrItem>>,
    /// Cached escaped items (subset of items that are "escapeable").
    escaped_items: Arc<HashMap<String, OotrItem>>,
    /// Cached regions converted to ootr format.
    regions: Regions<Self>,
    /// Logic tricks currently enabled.
    logic_tricks: Arc<HashSet<String>>,
    /// Setting names/infos.
    setting_infos: Arc<HashSet<String>>,
}

impl OotmmRando {
    /// Creates a new `OotmmRando` with embedded world data.
    ///
    /// This loads all embedded world data from the ootmm crate and initializes
    /// the item tables.
    ///
    /// # Errors
    ///
    /// Returns an error if the embedded world data fails to load.
    pub fn new() -> Result<Self, OotmmRandoError> {
        let world_db = embedded_data::create_world_database()?;
        Self::from_world_database(world_db)
    }

    /// Creates a new `OotmmRando` from an existing `WorldDatabase`.
    ///
    /// # Errors
    ///
    /// Returns an error if the world database cannot be processed.
    pub fn from_world_database(world_db: WorldDatabase) -> Result<Self, OotmmRandoError> {
        let item_table = Arc::new(Self::build_item_table());
        let escaped_items = Arc::new(Self::build_escaped_items());
        let regions = Arc::new(Self::build_regions(&world_db));
        let logic_tricks = Arc::new(HashSet::new());
        let setting_infos = Arc::new(Self::build_setting_infos());

        Ok(Self {
            world_db,
            item_table,
            escaped_items,
            regions,
            logic_tricks,
            setting_infos,
        })
    }

    /// Returns a reference to the underlying `WorldDatabase`.
    pub fn world_database(&self) -> &WorldDatabase {
        &self.world_db
    }

    /// Builds the complete item table mapping item names to ootr Items.
    fn build_item_table() -> HashMap<String, OotrItem> {
        let mut table = HashMap::new();

        // Add all OoT items
        Self::add_oot_items_to_table(&mut table);

        // Add all MM items
        Self::add_mm_items_to_table(&mut table);

        table
    }

    /// Adds OoT items to the item table.
    fn add_oot_items_to_table(table: &mut HashMap<String, OotrItem>) {
        use crate::item::OotItem;

        // List of all OoT item names (using the enum variant names)
        let oot_items = [
            // Swords
            "KokiriSword",
            "MasterSword",
            "BiggoronSword",
            "GiantKnife",
            // Shields
            "DekuShield",
            "HylianShield",
            "MirrorShield",
            // Tunics
            "KokiriTunic",
            "GoronTunic",
            "ZoraTunic",
            // Boots
            "KokiriBoots",
            "IronBoots",
            "HoverBoots",
            // Equipment
            "DekuStick",
            "DekuNut",
            "Bomb",
            "Bow",
            "FireArrow",
            "IceArrow",
            "LightArrow",
            "DinsFire",
            "FaroresWind",
            "NayrusLove",
            "Slingshot",
            "Boomerang",
            "Hookshot",
            "Longshot",
            "LensOfTruth",
            "MegatonHammer",
            "OcarinaOfTime",
            // Bottles
            "Bottle",
            "BottleRedPotion",
            "BottleGreenPotion",
            "BottleBluePotion",
            "BottleFairy",
            "BottleFish",
            "BottleBlueFire",
            "BottleBugs",
            "BottlePoe",
            "BottleBigPoe",
            "BottleMilk",
            "BottleHalfMilk",
            "BottleRutosLetter",
            // Adult Trade
            "PocketEgg",
            "PocketCucco",
            "Cojiro",
            "OddMushroom",
            "OddPotion",
            "PoachersSaw",
            "BrokenSword",
            "Prescription",
            "EyeballFrog",
            "Eyedrops",
            "ClaimCheck",
            // Child Trade
            "WeirdEgg",
            "Chicken",
            "ZeldasLetter",
            "SkullMask",
            "SpookyMask",
            "KeatonMask",
            "BunnyHood",
            "GoronMask",
            "ZoraMask",
            "GerudoMask",
            "MaskOfTruth",
            // Songs
            "ZeldasLullaby",
            "EponasSong",
            "SariasSong",
            "SunsSong",
            "SongOfTime",
            "SongOfStorms",
            "MinuetOfForest",
            "BoleroOfFire",
            "SerenadeOfWater",
            "NocturneOfShadow",
            "RequiemOfSpirit",
            "PreludeOfLight",
            "ScarecrowSong",
            // Upgrades
            "GoronBracelet",
            "SilverGauntlets",
            "GoldenGauntlets",
            "SilverScale",
            "GoldenScale",
            "ChildWallet",
            "AdultWallet",
            "GiantWallet",
            "DekuStickCapacity20",
            "DekuStickCapacity30",
            "DekuNutCapacity30",
            "DekuNutCapacity40",
            "BulletBag30",
            "BulletBag40",
            "BulletBag50",
            "Quiver30",
            "Quiver40",
            "Quiver50",
            "BombBag20",
            "BombBag30",
            "BombBag40",
            "MagicMeter",
            "DoubleMagic",
            "DoubleDefense",
            // Quest Items
            "KokiriEmerald",
            "GoronRuby",
            "ZoraSapphire",
            "ForestMedallion",
            "FireMedallion",
            "WaterMedallion",
            "ShadowMedallion",
            "SpiritMedallion",
            "LightMedallion",
            "StoneOfAgony",
            "GerudoCard",
            // Dungeon Items
            "SmallKey",
            "BossKey",
            "Map",
            "Compass",
            // Dungeon-Specific Keys
            "SmallKeyForestTemple",
            "SmallKeyFireTemple",
            "SmallKeyWaterTemple",
            "SmallKeyShadowTemple",
            "SmallKeySpiritTemple",
            "SmallKeyBottomOfTheWell",
            "SmallKeyGerudoFortress",
            "SmallKeyGerudoTrainingGround",
            "SmallKeyGanonsCastle",
            "BossKeyForestTemple",
            "BossKeyFireTemple",
            "BossKeyWaterTemple",
            "BossKeyShadowTemple",
            "BossKeySpiritTemple",
            "BossKeyGanonsCastle",
            // Collectibles
            "HeartContainer",
            "PieceOfHeart",
            "GoldSkulltula",
            "SmallMagicJar",
            "LargeMagicJar",
            "RecoveryHeart",
            "GreenRupee",
            "BlueRupee",
            "RedRupee",
            "PurpleRupee",
            "GoldRupee",
            // Special
            "Triforce",
            "TriforceOfCourage",
            "GanonBossKey",
        ];

        for name in oot_items {
            if OotItem::by_name(name).is_some() {
                table.insert(name.to_string(), OotrItem(name.to_string()));
            }
        }
    }

    /// Adds MM items to the item table.
    fn add_mm_items_to_table(table: &mut HashMap<String, OotrItem>) {
        use crate::item::MmItem;

        // List of all MM item names (using the enum variant names)
        // Only add items that don't already exist in the table (OoT takes precedence)
        let mm_items = [
            // Transformation Masks
            "DekuMask",
            "GoronMask",
            "ZoraMask",
            "FierceDeityMask",
            // Regular Masks
            "PostmanHat",
            "AllNightMask",
            "BlastMask",
            "StoneMask",
            "GreatFairyMask",
            "BremenMask",
            "DonGeroMask",
            "MaskOfScents",
            "RomaniMask",
            "CircusLeaderMask",
            "KafeiMask",
            "CouplesMask",
            "KamaroMask",
            "GibdoMask",
            "GaroMask",
            "CaptainHat",
            "GiantMask",
            // MM-specific swords
            "RazorSword",
            "GildedSword",
            "GreatFairySword",
            // MM-specific shields
            "HeroShield",
            // MM-specific equipment
            "HerosBow",
            "Bombchu",
            "MagicBean",
            "PowderKeg",
            "PictographBox",
            // MM-specific bottles
            "BottleDekuPrincess",
            "BottleHotSpringWater",
            "BottleZoraEgg",
            "BottleMushroom",
            "BottleGoldDust",
            "BottleChateau",
            "BottleSeaHorse",
            // MM-specific songs
            "SongOfHealing",
            "SongOfSoaring",
            "SonataOfAwakening",
            "GoronLullaby",
            "NewWaveBossaNova",
            "ElegyOfEmptiness",
            "OathToOrder",
            // MM-specific quest items
            "MoonsTear",
            "LandTitleDeed",
            "SwampTitleDeed",
            "MountainTitleDeed",
            "OceanTitleDeed",
            "RoomKey",
            "LetterToKafei",
            "PendantOfMemories",
            "LetterToMama",
            "SpecialDeliveryToMama",
            // Boss Remains
            "OdolwaRemains",
            "GohtRemains",
            "GyorgRemains",
            "TwinmoldRemains",
            // MM-specific dungeon items
            "StrayFairy",
            "SmallKeyWoodfallTemple",
            "SmallKeySnowheadTemple",
            "SmallKeyGreatBayTemple",
            "SmallKeyStoneTowerTemple",
            "BossKeyWoodfallTemple",
            "BossKeySnowheadTemple",
            "BossKeyGreatBayTemple",
            "BossKeyStoneTowerTemple",
            // Stray Fairies per dungeon
            "StrayFairyWoodfall",
            "StrayFairySnowhead",
            "StrayFairyGreatBay",
            "StrayFairyStoneTower",
            "StrayFairyClockTown",
            // MM-specific rupees
            "SilverRupee",
            // Special
            "BomberNotebook",
            "GiantsWallet",
            "OceanTitleDeedTraded",
        ];

        for name in mm_items {
            // Only add if not already present (OoT items take precedence for shared names)
            if !table.contains_key(name) && MmItem::by_name(name).is_some() {
                table.insert(name.to_string(), OotrItem(name.to_string()));
            }
        }
    }

    /// Builds the escaped items table.
    ///
    /// Escaped items are items that can "escape" from their original location,
    /// typically progression items. For now, we include all major progression items.
    fn build_escaped_items() -> HashMap<String, OotrItem> {
        let mut escaped = HashMap::new();

        // Major progression items from both games
        let progression_items = [
            // OoT progression
            "Hookshot",
            "Longshot",
            "Bow",
            "FireArrow",
            "IceArrow",
            "LightArrow",
            "Boomerang",
            "Slingshot",
            "Bomb",
            "Bombchu",
            "MegatonHammer",
            "DinsFire",
            "FaroresWind",
            "NayrusLove",
            "LensOfTruth",
            "OcarinaOfTime",
            "IronBoots",
            "HoverBoots",
            "GoronTunic",
            "ZoraTunic",
            "MirrorShield",
            "GoronBracelet",
            "SilverGauntlets",
            "GoldenGauntlets",
            "SilverScale",
            "GoldenScale",
            "MagicMeter",
            "DoubleMagic",
            // Songs
            "ZeldasLullaby",
            "EponasSong",
            "SariasSong",
            "SunsSong",
            "SongOfTime",
            "SongOfStorms",
            "MinuetOfForest",
            "BoleroOfFire",
            "SerenadeOfWater",
            "NocturneOfShadow",
            "RequiemOfSpirit",
            "PreludeOfLight",
            // Stones and Medallions
            "KokiriEmerald",
            "GoronRuby",
            "ZoraSapphire",
            "ForestMedallion",
            "FireMedallion",
            "WaterMedallion",
            "ShadowMedallion",
            "SpiritMedallion",
            "LightMedallion",
            // MM progression
            "DekuMask",
            "GoronMask",
            "ZoraMask",
            "FierceDeityMask",
            "HerosBow",
            "PowderKeg",
            "SongOfHealing",
            "SongOfSoaring",
            "SonataOfAwakening",
            "GoronLullaby",
            "NewWaveBossaNova",
            "ElegyOfEmptiness",
            "OathToOrder",
            "OdolwaRemains",
            "GohtRemains",
            "GyorgRemains",
            "TwinmoldRemains",
        ];

        for name in progression_items {
            escaped.insert(name.to_string(), OotrItem(name.to_string()));
        }

        escaped
    }

    /// Builds regions from the world database.
    fn build_regions(world_db: &WorldDatabase) -> Vec<Arc<OotrRegion<OotmmRando>>> {
        world_db
            .regions()
            .map(|region| {
                let ootr_region = OotrRegion {
                    name: OotmmRegionName::new(&region.id),
                    dungeon: None, // Could be populated from region metadata if available
                    scene: Some(region.id.clone()),
                    hint: Some(region.name.clone()),
                    time_passes: false,
                    events: region.events.iter().map(|e| e.id.clone()).collect(),
                    locations: region.locations.iter().map(|l| l.id.clone()).collect(),
                    exits: region
                        .exits
                        .iter()
                        .map(|e| OotmmRegionName::new(&e.target))
                        .collect(),
                };
                Arc::new(ootr_region)
            })
            .collect()
    }

    /// Builds the setting infos set.
    fn build_setting_infos() -> HashSet<String> {
        // Common OoTMM settings that might be checked in logic
        let settings = [
            "open_door_of_time",
            "open_kakariko",
            "open_gerudo_fortress",
            "zora_fountain",
            "kokiri_forest",
            "bridge",
            "ganon_bosskey",
            "mq_dungeons",
            "shuffle_songs",
            "shuffle_scrubs",
            "shuffle_cows",
            "shuffle_beans",
            "shuffle_kokiri_sword",
            "shuffle_ocarinas",
            "shuffle_weird_egg",
            "shuffle_gerudo_card",
            "shuffle_mapcompass",
            "shuffle_smallkeys",
            "shuffle_bosskeys",
            "shuffle_ganon_bosskey",
            "enhance_map_compass",
            "damage_multiplier",
            "starting_age",
            "bombchus_in_logic",
            "logic_rules",
            "all_reachable",
            "starting_items",
        ];

        settings.iter().map(|s| (*s).to_string()).collect()
    }

    /// Sets the enabled logic tricks.
    pub fn set_logic_tricks(&mut self, tricks: HashSet<String>) {
        self.logic_tricks = Arc::new(tricks);
    }

    /// Adds a logic trick to the enabled set.
    pub fn enable_trick(&mut self, trick: impl Into<String>) {
        let mut tricks = (*self.logic_tricks).clone();
        tricks.insert(trick.into());
        self.logic_tricks = Arc::new(tricks);
    }
}

impl fmt::Debug for OotmmRando {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("OotmmRando")
            .field("region_count", &self.world_db.region_count())
            .field("item_count", &self.item_table.len())
            .finish()
    }
}

impl Rando for OotmmRando {
    type Err = OotmmRandoError;
    type RegionName = OotmmRegionName;

    fn escaped_items(&self) -> Result<Arc<HashMap<String, OotrItem>>, Self::Err> {
        Ok(Arc::clone(&self.escaped_items))
    }

    fn item_table(&self) -> Result<Arc<HashMap<String, OotrItem>>, Self::Err> {
        Ok(Arc::clone(&self.item_table))
    }

    fn logic_tricks(&self) -> Result<Arc<HashSet<String>>, Self::Err> {
        Ok(Arc::clone(&self.logic_tricks))
    }

    fn regions(&self) -> Result<Regions<Self>, Self::Err> {
        Ok(Arc::clone(&self.regions))
    }

    fn root() -> Self::RegionName {
        // In OoTMM, the root region is typically the starting area
        // This could be "Root" or a specific starting region
        OotmmRegionName::new("Root")
    }

    fn setting_infos(&self) -> Result<Arc<HashSet<String>>, Self::Err> {
        Ok(Arc::clone(&self.setting_infos))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ootmm_rando_error_display() {
        let err = OotmmRandoError::new("test error");
        assert_eq!(err.to_string(), "test error");
    }

    #[test]
    fn test_ootmm_region_name_from_static_str() {
        let name: OotmmRegionName = "Kokiri Forest".into();
        assert_eq!(name.as_ref(), "Kokiri Forest");
    }

    #[test]
    fn test_ootmm_region_name_display() {
        let name = OotmmRegionName::new("Lost Woods");
        assert_eq!(format!("{}", name), "Lost Woods");
    }

    #[test]
    fn test_ootmm_region_name_eq_str() {
        let name = OotmmRegionName::new("Hyrule Field");
        assert!(name == "Hyrule Field");
        assert!(!(name == "Other"));
    }

    #[test]
    fn test_ootmm_rando_new() {
        let rando = OotmmRando::new();
        assert!(
            rando.is_ok(),
            "Failed to create OotmmRando: {:?}",
            rando.err()
        );
    }

    #[test]
    fn test_ootmm_rando_item_table() {
        let rando = OotmmRando::new().unwrap();
        let items = rando.item_table().unwrap();

        // Should have items from both games with correct values
        assert_eq!(
            items.get("MasterSword").map(|i| i.name()),
            Some("MasterSword")
        );
        assert_eq!(items.get("Hookshot").map(|i| i.name()), Some("Hookshot"));
        assert_eq!(items.get("DekuMask").map(|i| i.name()), Some("DekuMask"));
        assert_eq!(
            items.get("OdolwaRemains").map(|i| i.name()),
            Some("OdolwaRemains")
        );
    }

    #[test]
    fn test_ootmm_rando_escaped_items() {
        let rando = OotmmRando::new().unwrap();
        let escaped = rando.escaped_items().unwrap();

        // Should contain major progression items with correct values
        assert_eq!(escaped.get("Hookshot").map(|i| i.name()), Some("Hookshot"));
        assert_eq!(escaped.get("DekuMask").map(|i| i.name()), Some("DekuMask"));
        assert_eq!(
            escaped.get("ForestMedallion").map(|i| i.name()),
            Some("ForestMedallion")
        );
    }

    #[test]
    fn test_ootmm_rando_regions() {
        let rando = OotmmRando::new().unwrap();
        let regions = rando.regions().unwrap();

        // Should have regions loaded from embedded data
        assert!(!regions.is_empty());
    }

    #[test]
    fn test_ootmm_rando_root() {
        let root = OotmmRando::root();
        assert_eq!(root.as_ref(), "Root");
    }

    #[test]
    fn test_ootmm_rando_setting_infos() {
        let rando = OotmmRando::new().unwrap();
        let settings = rando.setting_infos().unwrap();

        // Should have common settings
        assert!(settings.contains("open_door_of_time"));
        assert!(settings.contains("shuffle_songs"));
    }

    #[test]
    fn test_ootmm_rando_logic_tricks() {
        let rando = OotmmRando::new().unwrap();
        let tricks = rando.logic_tricks().unwrap();

        // By default, no tricks are enabled
        assert!(tricks.is_empty());
    }

    #[test]
    fn test_ootmm_rando_enable_trick() {
        let mut rando = OotmmRando::new().unwrap();
        rando.enable_trick("lens_of_truth_skip");

        let tricks = rando.logic_tricks().unwrap();
        assert!(tricks.contains("lens_of_truth_skip"));
    }

    #[test]
    fn test_ootmm_rando_set_logic_tricks() {
        let mut rando = OotmmRando::new().unwrap();
        let mut tricks = HashSet::new();
        tricks.insert("trick1".to_string());
        tricks.insert("trick2".to_string());

        rando.set_logic_tricks(tricks);

        let result = rando.logic_tricks().unwrap();
        assert!(result.contains("trick1"));
        assert!(result.contains("trick2"));
        assert_eq!(result.len(), 2);
    }

    #[test]
    fn test_ootmm_rando_world_database_access() {
        let rando = OotmmRando::new().unwrap();
        let db = rando.world_database();

        // Should have regions from embedded data
        assert!(db.region_count() > 0);
    }

    #[test]
    fn test_ootmm_rando_debug() {
        let rando = OotmmRando::new().unwrap();
        let debug_str = format!("{:?}", rando);
        assert!(debug_str.contains("OotmmRando"));
        assert!(debug_str.contains("region_count"));
        assert!(debug_str.contains("item_count"));
    }
}
