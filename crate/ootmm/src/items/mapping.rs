//! Item name mapping implementation.
//!
//! This module provides the core mapping functionality between OoTMM's string-based
//! item identifiers and the internal enum representations.

use crate::item::{Item, MmItem, OotItem};
use serde::{Deserialize, Serialize};
use std::fmt;

/// Error type for item mapping operations.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MappingError {
    /// The item name that failed to map.
    pub name: String,
    /// Description of the error.
    pub message: String,
}

impl MappingError {
    /// Create a new mapping error for an unknown item name.
    pub fn unknown_item(name: impl Into<String>) -> Self {
        let name = name.into();
        Self {
            message: format!("unknown item name: '{}'", name),
            name,
        }
    }

    /// Create a new mapping error for an invalid game context.
    pub fn invalid_game(name: impl Into<String>, expected_game: &str) -> Self {
        let name = name.into();
        Self {
            message: format!("item '{}' is not from {}", name, expected_game),
            name,
        }
    }
}

impl fmt::Display for MappingError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for MappingError {}

/// A wrapper type for OoTMM item names.
///
/// This type represents an item name as used in OoTMM's logic files and data.
/// It supports both PascalCase (e.g., "MasterSword") and snake_case (e.g., "master_sword")
/// naming conventions.
///
/// # Examples
///
/// ```
/// use ootmm::items::ItemName;
/// use ootmm::OotItem;
///
/// // Create from string
/// let name = ItemName::new("MasterSword");
/// assert_eq!(name.as_str(), "MasterSword");
///
/// // Convert to OotItem
/// let item: OotItem = name.try_into().unwrap();
/// assert_eq!(item, OotItem::MasterSword);
///
/// // Convert from OotItem
/// let name2 = ItemName::from(OotItem::MasterSword);
/// assert_eq!(name2.as_str(), "MasterSword");
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ItemName(String);

impl ItemName {
    /// Create a new ItemName from a string.
    pub fn new(name: impl Into<String>) -> Self {
        Self(name.into())
    }

    /// Get the string representation of the item name.
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Convert this item name to an OotItem, if it represents an OoT item.
    pub fn to_oot_item(&self) -> Result<OotItem, MappingError> {
        OotItem::by_name(&self.0).ok_or_else(|| MappingError::unknown_item(&self.0))
    }

    /// Convert this item name to an MmItem, if it represents an MM item.
    pub fn to_mm_item(&self) -> Result<MmItem, MappingError> {
        MmItem::by_name(&self.0).ok_or_else(|| MappingError::unknown_item(&self.0))
    }

    /// Convert this item name to an Item (either OoT or MM).
    ///
    /// Note: Items that exist in both games will return the OoT variant.
    /// Use `to_oot_item` or `to_mm_item` directly if you need a specific game's item.
    pub fn to_item(&self) -> Result<Item, MappingError> {
        Item::by_name(&self.0).ok_or_else(|| MappingError::unknown_item(&self.0))
    }

    /// Check if this item name is valid (maps to any known item).
    pub fn is_valid(&self) -> bool {
        Item::by_name(&self.0).is_some()
    }

    /// Check if this item name is a valid OoT item.
    pub fn is_oot_item(&self) -> bool {
        OotItem::by_name(&self.0).is_some()
    }

    /// Check if this item name is a valid MM item.
    pub fn is_mm_item(&self) -> bool {
        MmItem::by_name(&self.0).is_some()
    }
}

impl fmt::Display for ItemName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<&str> for ItemName {
    fn from(s: &str) -> Self {
        Self::new(s)
    }
}

impl From<String> for ItemName {
    fn from(s: String) -> Self {
        Self(s)
    }
}

impl AsRef<str> for ItemName {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

// OotItem conversions

impl From<OotItem> for ItemName {
    fn from(item: OotItem) -> Self {
        Self::new(format!("{:?}", item))
    }
}

impl TryFrom<ItemName> for OotItem {
    type Error = MappingError;

    fn try_from(name: ItemName) -> Result<Self, Self::Error> {
        name.to_oot_item()
    }
}

impl TryFrom<&ItemName> for OotItem {
    type Error = MappingError;

    fn try_from(name: &ItemName) -> Result<Self, Self::Error> {
        name.to_oot_item()
    }
}

// MmItem conversions

impl From<MmItem> for ItemName {
    fn from(item: MmItem) -> Self {
        Self::new(format!("{:?}", item))
    }
}

impl TryFrom<ItemName> for MmItem {
    type Error = MappingError;

    fn try_from(name: ItemName) -> Result<Self, Self::Error> {
        name.to_mm_item()
    }
}

impl TryFrom<&ItemName> for MmItem {
    type Error = MappingError;

    fn try_from(name: &ItemName) -> Result<Self, Self::Error> {
        name.to_mm_item()
    }
}

// Combined Item conversions

impl From<Item> for ItemName {
    fn from(item: Item) -> Self {
        match item {
            Item::Oot(oot) => ItemName::from(oot),
            Item::Mm(mm) => ItemName::from(mm),
        }
    }
}

impl TryFrom<ItemName> for Item {
    type Error = MappingError;

    fn try_from(name: ItemName) -> Result<Self, Self::Error> {
        name.to_item()
    }
}

impl TryFrom<&ItemName> for Item {
    type Error = MappingError;

    fn try_from(name: &ItemName) -> Result<Self, Self::Error> {
        name.to_item()
    }
}

/// Provides utilities for item name mapping operations.
///
/// This struct provides batch operations and lookup utilities for item mappings.
#[derive(Debug, Clone, Default)]
pub struct ItemMapping;

impl ItemMapping {
    /// Create a new ItemMapping instance.
    pub fn new() -> Self {
        Self
    }

    /// Look up an item by name, returning the combined Item type.
    ///
    /// Note: Items that exist in both games will return the OoT variant.
    pub fn lookup(&self, name: &str) -> Option<Item> {
        Item::by_name(name)
    }

    /// Look up an OoT item by name.
    pub fn lookup_oot(&self, name: &str) -> Option<OotItem> {
        OotItem::by_name(name)
    }

    /// Look up an MM item by name.
    pub fn lookup_mm(&self, name: &str) -> Option<MmItem> {
        MmItem::by_name(name)
    }

    /// Get the canonical name for an OoT item.
    pub fn name_of_oot(&self, item: OotItem) -> ItemName {
        ItemName::from(item)
    }

    /// Get the canonical name for an MM item.
    pub fn name_of_mm(&self, item: MmItem) -> ItemName {
        ItemName::from(item)
    }

    /// Get the canonical name for a combined Item.
    pub fn name_of(&self, item: Item) -> ItemName {
        ItemName::from(item)
    }

    /// Parse multiple item names into OoT items.
    ///
    /// Returns a tuple of (successful items, failed names).
    pub fn parse_oot_items<'a>(
        &self,
        names: impl IntoIterator<Item = &'a str>,
    ) -> (Vec<OotItem>, Vec<&'a str>) {
        let mut items = Vec::new();
        let mut failed = Vec::new();

        for name in names {
            match OotItem::by_name(name) {
                Some(item) => items.push(item),
                None => failed.push(name),
            }
        }

        (items, failed)
    }

    /// Parse multiple item names into MM items.
    ///
    /// Returns a tuple of (successful items, failed names).
    pub fn parse_mm_items<'a>(
        &self,
        names: impl IntoIterator<Item = &'a str>,
    ) -> (Vec<MmItem>, Vec<&'a str>) {
        let mut items = Vec::new();
        let mut failed = Vec::new();

        for name in names {
            match MmItem::by_name(name) {
                Some(item) => items.push(item),
                None => failed.push(name),
            }
        }

        (items, failed)
    }

    /// Parse multiple item names into combined Items.
    ///
    /// Returns a tuple of (successful items, failed names).
    pub fn parse_items<'a>(
        &self,
        names: impl IntoIterator<Item = &'a str>,
    ) -> (Vec<Item>, Vec<&'a str>) {
        let mut items = Vec::new();
        let mut failed = Vec::new();

        for name in names {
            match Item::by_name(name) {
                Some(item) => items.push(item),
                None => failed.push(name),
            }
        }

        (items, failed)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // ===== ItemName Basic Tests =====

    #[test]
    fn test_item_name_new() {
        let name = ItemName::new("MasterSword");
        assert_eq!(name.as_str(), "MasterSword");
    }

    #[test]
    fn test_item_name_from_str() {
        let name: ItemName = "Hookshot".into();
        assert_eq!(name.as_str(), "Hookshot");
    }

    #[test]
    fn test_item_name_from_string() {
        let name: ItemName = String::from("Boomerang").into();
        assert_eq!(name.as_str(), "Boomerang");
    }

    #[test]
    fn test_item_name_display() {
        let name = ItemName::new("LensOfTruth");
        assert_eq!(format!("{}", name), "LensOfTruth");
    }

    #[test]
    fn test_item_name_as_ref() {
        let name = ItemName::new("Bow");
        let s: &str = name.as_ref();
        assert_eq!(s, "Bow");
    }

    // ===== ItemName Validation Tests =====

    #[test]
    fn test_item_name_is_valid_oot() {
        assert!(ItemName::new("MasterSword").is_valid());
        assert!(ItemName::new("Hookshot").is_valid());
        assert!(ItemName::new("ZeldasLullaby").is_valid());
    }

    #[test]
    fn test_item_name_is_valid_mm() {
        assert!(ItemName::new("DekuMask").is_valid());
        assert!(ItemName::new("FierceDeityMask").is_valid());
        assert!(ItemName::new("OathToOrder").is_valid());
    }

    #[test]
    fn test_item_name_is_valid_snake_case() {
        assert!(ItemName::new("master_sword").is_valid());
        assert!(ItemName::new("deku_mask").is_valid());
        assert!(ItemName::new("song_of_time").is_valid());
    }

    #[test]
    fn test_item_name_is_invalid() {
        assert!(!ItemName::new("NotAnItem").is_valid());
        assert!(!ItemName::new("").is_valid());
        assert!(!ItemName::new("invalid_item_name").is_valid());
    }

    #[test]
    fn test_item_name_is_oot_item() {
        assert!(ItemName::new("MasterSword").is_oot_item());
        assert!(ItemName::new("Boomerang").is_oot_item());
        // Items in both games - OoT has them too
        assert!(ItemName::new("Hookshot").is_oot_item());
        // MM-only items
        assert!(!ItemName::new("DekuMask").is_oot_item());
        assert!(!ItemName::new("FierceDeityMask").is_oot_item());
    }

    #[test]
    fn test_item_name_is_mm_item() {
        assert!(ItemName::new("DekuMask").is_mm_item());
        assert!(ItemName::new("FierceDeityMask").is_mm_item());
        // Items in both games - MM has them too
        assert!(ItemName::new("Hookshot").is_mm_item());
        // OoT-only items
        assert!(!ItemName::new("Boomerang").is_mm_item());
        assert!(!ItemName::new("Longshot").is_mm_item());
    }

    // ===== OotItem Conversion Tests =====

    #[test]
    fn test_oot_item_to_name() {
        let name = ItemName::from(OotItem::MasterSword);
        assert_eq!(name.as_str(), "MasterSword");
    }

    #[test]
    fn test_oot_item_roundtrip() {
        let original = OotItem::Hookshot;
        let name = ItemName::from(original);
        let converted: OotItem = name.try_into().unwrap();
        assert_eq!(original, converted);
    }

    #[test]
    fn test_oot_item_from_pascal_case() {
        let name = ItemName::new("MasterSword");
        let item: OotItem = name.try_into().unwrap();
        assert_eq!(item, OotItem::MasterSword);
    }

    #[test]
    fn test_oot_item_from_snake_case() {
        let name = ItemName::new("master_sword");
        let item: OotItem = name.try_into().unwrap();
        assert_eq!(item, OotItem::MasterSword);
    }

    #[test]
    fn test_oot_item_conversion_error() {
        let name = ItemName::new("DekuMask");
        let result: Result<OotItem, _> = name.try_into();
        assert!(result.is_err());
    }

    #[test]
    fn test_oot_item_all_swords() {
        for item in [
            OotItem::KokiriSword,
            OotItem::MasterSword,
            OotItem::BiggoronSword,
            OotItem::GiantKnife,
        ] {
            let name = ItemName::from(item);
            let roundtrip: OotItem = name.try_into().unwrap();
            assert_eq!(item, roundtrip);
        }
    }

    #[test]
    fn test_oot_item_all_shields() {
        for item in [
            OotItem::DekuShield,
            OotItem::HylianShield,
            OotItem::MirrorShield,
        ] {
            let name = ItemName::from(item);
            let roundtrip: OotItem = name.try_into().unwrap();
            assert_eq!(item, roundtrip);
        }
    }

    #[test]
    fn test_oot_item_all_songs() {
        for item in [
            OotItem::ZeldasLullaby,
            OotItem::EponasSong,
            OotItem::SariasSong,
            OotItem::SunsSong,
            OotItem::SongOfTime,
            OotItem::SongOfStorms,
            OotItem::MinuetOfForest,
            OotItem::BoleroOfFire,
            OotItem::SerenadeOfWater,
            OotItem::NocturneOfShadow,
            OotItem::RequiemOfSpirit,
            OotItem::PreludeOfLight,
            OotItem::ScarecrowSong,
        ] {
            let name = ItemName::from(item);
            let roundtrip: OotItem = name.try_into().unwrap();
            assert_eq!(item, roundtrip);
        }
    }

    #[test]
    fn test_oot_item_all_medallions() {
        for item in [
            OotItem::ForestMedallion,
            OotItem::FireMedallion,
            OotItem::WaterMedallion,
            OotItem::ShadowMedallion,
            OotItem::SpiritMedallion,
            OotItem::LightMedallion,
        ] {
            let name = ItemName::from(item);
            let roundtrip: OotItem = name.try_into().unwrap();
            assert_eq!(item, roundtrip);
        }
    }

    #[test]
    fn test_oot_item_all_spiritual_stones() {
        for item in [
            OotItem::KokiriEmerald,
            OotItem::GoronRuby,
            OotItem::ZoraSapphire,
        ] {
            let name = ItemName::from(item);
            let roundtrip: OotItem = name.try_into().unwrap();
            assert_eq!(item, roundtrip);
        }
    }

    #[test]
    fn test_oot_item_equipment() {
        for item in [
            OotItem::DekuStick,
            OotItem::DekuNut,
            OotItem::Bomb,
            OotItem::Bow,
            OotItem::FireArrow,
            OotItem::IceArrow,
            OotItem::LightArrow,
            OotItem::Slingshot,
            OotItem::Boomerang,
            OotItem::Hookshot,
            OotItem::Longshot,
            OotItem::LensOfTruth,
            OotItem::MegatonHammer,
        ] {
            let name = ItemName::from(item);
            let roundtrip: OotItem = name.try_into().unwrap();
            assert_eq!(item, roundtrip);
        }
    }

    #[test]
    fn test_oot_item_magic_spells() {
        for item in [OotItem::DinsFire, OotItem::FaroresWind, OotItem::NayrusLove] {
            let name = ItemName::from(item);
            let roundtrip: OotItem = name.try_into().unwrap();
            assert_eq!(item, roundtrip);
        }
    }

    #[test]
    fn test_oot_item_dungeon_keys() {
        for item in [
            OotItem::SmallKey,
            OotItem::BossKey,
            OotItem::SmallKeyForestTemple,
            OotItem::SmallKeyFireTemple,
            OotItem::SmallKeyWaterTemple,
            OotItem::SmallKeyShadowTemple,
            OotItem::SmallKeySpiritTemple,
            OotItem::BossKeyForestTemple,
            OotItem::BossKeyFireTemple,
            OotItem::BossKeyWaterTemple,
            OotItem::BossKeyShadowTemple,
            OotItem::BossKeySpiritTemple,
            OotItem::BossKeyGanonsCastle,
        ] {
            let name = ItemName::from(item);
            let roundtrip: OotItem = name.try_into().unwrap();
            assert_eq!(item, roundtrip);
        }
    }

    // ===== MmItem Conversion Tests =====

    #[test]
    fn test_mm_item_to_name() {
        let name = ItemName::from(MmItem::DekuMask);
        assert_eq!(name.as_str(), "DekuMask");
    }

    #[test]
    fn test_mm_item_roundtrip() {
        let original = MmItem::FierceDeityMask;
        let name = ItemName::from(original);
        let converted: MmItem = name.try_into().unwrap();
        assert_eq!(original, converted);
    }

    #[test]
    fn test_mm_item_from_pascal_case() {
        let name = ItemName::new("DekuMask");
        let item: MmItem = name.try_into().unwrap();
        assert_eq!(item, MmItem::DekuMask);
    }

    #[test]
    fn test_mm_item_from_snake_case() {
        let name = ItemName::new("deku_mask");
        let item: MmItem = name.try_into().unwrap();
        assert_eq!(item, MmItem::DekuMask);
    }

    #[test]
    fn test_mm_item_conversion_error() {
        let name = ItemName::new("Boomerang"); // OoT-only item
        let result: Result<MmItem, _> = name.try_into();
        assert!(result.is_err());
    }

    #[test]
    fn test_mm_item_all_transformation_masks() {
        for item in [
            MmItem::DekuMask,
            MmItem::GoronMask,
            MmItem::ZoraMask,
            MmItem::FierceDeityMask,
        ] {
            let name = ItemName::from(item);
            let roundtrip: MmItem = name.try_into().unwrap();
            assert_eq!(item, roundtrip);
        }
    }

    #[test]
    fn test_mm_item_regular_masks() {
        for item in [
            MmItem::PostmanHat,
            MmItem::AllNightMask,
            MmItem::BlastMask,
            MmItem::StoneMask,
            MmItem::GreatFairyMask,
            MmItem::KeatonMask,
            MmItem::BremenMask,
            MmItem::BunnyHood,
            MmItem::DonGeroMask,
            MmItem::MaskOfScents,
            MmItem::RomaniMask,
            MmItem::CircusLeaderMask,
            MmItem::KafeiMask,
            MmItem::CouplesMask,
            MmItem::MaskOfTruth,
            MmItem::KamaroMask,
            MmItem::GibdoMask,
            MmItem::GaroMask,
            MmItem::CaptainHat,
            MmItem::GiantMask,
        ] {
            let name = ItemName::from(item);
            let roundtrip: MmItem = name.try_into().unwrap();
            assert_eq!(item, roundtrip);
        }
    }

    #[test]
    fn test_mm_item_all_songs() {
        for item in [
            MmItem::SongOfTime,
            MmItem::SongOfHealing,
            MmItem::EponasSong,
            MmItem::SongOfSoaring,
            MmItem::SongOfStorms,
            MmItem::SonataOfAwakening,
            MmItem::GoronLullaby,
            MmItem::NewWaveBossaNova,
            MmItem::ElegyOfEmptiness,
            MmItem::OathToOrder,
        ] {
            let name = ItemName::from(item);
            let roundtrip: MmItem = name.try_into().unwrap();
            assert_eq!(item, roundtrip);
        }
    }

    #[test]
    fn test_mm_item_boss_remains() {
        for item in [
            MmItem::OdolwaRemains,
            MmItem::GohtRemains,
            MmItem::GyorgRemains,
            MmItem::TwinmoldRemains,
        ] {
            let name = ItemName::from(item);
            let roundtrip: MmItem = name.try_into().unwrap();
            assert_eq!(item, roundtrip);
        }
    }

    #[test]
    fn test_mm_item_stray_fairies() {
        for item in [
            MmItem::StrayFairy,
            MmItem::StrayFairyWoodfall,
            MmItem::StrayFairySnowhead,
            MmItem::StrayFairyGreatBay,
            MmItem::StrayFairyStoneTower,
            MmItem::StrayFairyClockTown,
        ] {
            let name = ItemName::from(item);
            let roundtrip: MmItem = name.try_into().unwrap();
            assert_eq!(item, roundtrip);
        }
    }

    #[test]
    fn test_mm_item_dungeon_keys() {
        for item in [
            MmItem::SmallKey,
            MmItem::BossKey,
            MmItem::SmallKeyWoodfallTemple,
            MmItem::SmallKeySnowheadTemple,
            MmItem::SmallKeyGreatBayTemple,
            MmItem::SmallKeyStoneTowerTemple,
            MmItem::BossKeyWoodfallTemple,
            MmItem::BossKeySnowheadTemple,
            MmItem::BossKeyGreatBayTemple,
            MmItem::BossKeyStoneTowerTemple,
        ] {
            let name = ItemName::from(item);
            let roundtrip: MmItem = name.try_into().unwrap();
            assert_eq!(item, roundtrip);
        }
    }

    // ===== Combined Item Tests =====

    #[test]
    fn test_combined_item_from_oot() {
        let item = Item::Oot(OotItem::MasterSword);
        let name = ItemName::from(item);
        assert_eq!(name.as_str(), "MasterSword");
    }

    #[test]
    fn test_combined_item_from_mm() {
        let item = Item::Mm(MmItem::DekuMask);
        let name = ItemName::from(item);
        assert_eq!(name.as_str(), "DekuMask");
    }

    #[test]
    fn test_combined_item_roundtrip_oot() {
        let original = Item::Oot(OotItem::Boomerang);
        let name = ItemName::from(original);
        let converted: Item = name.try_into().unwrap();
        assert_eq!(original, converted);
    }

    #[test]
    fn test_combined_item_roundtrip_mm() {
        let original = Item::Mm(MmItem::OdolwaRemains);
        let name = ItemName::from(original);
        let converted: Item = name.try_into().unwrap();
        assert_eq!(original, converted);
    }

    #[test]
    fn test_shared_item_returns_oot() {
        // Items in both games should return OoT variant
        let name = ItemName::new("Hookshot");
        let item: Item = name.try_into().unwrap();
        assert_eq!(item, Item::Oot(OotItem::Hookshot));
    }

    // ===== Serde Tests =====

    #[test]
    fn test_item_name_serde_roundtrip() {
        let name = ItemName::new("MasterSword");
        let serialized = serde_yaml::to_string(&name).unwrap();
        let deserialized: ItemName = serde_yaml::from_str(&serialized).unwrap();
        assert_eq!(name, deserialized);
    }

    #[test]
    fn test_item_name_serde_transparent() {
        let name = ItemName::new("Hookshot");
        let serialized = serde_yaml::to_string(&name).unwrap();
        // Should serialize as just the string, not a struct
        assert_eq!(serialized.trim(), "Hookshot");
    }

    #[test]
    fn test_item_name_deserialize_from_string() {
        let yaml = "DekuMask";
        let name: ItemName = serde_yaml::from_str(yaml).unwrap();
        assert_eq!(name.as_str(), "DekuMask");
    }

    #[test]
    fn test_item_name_vec_serde() {
        let names = vec![
            ItemName::new("MasterSword"),
            ItemName::new("DekuMask"),
            ItemName::new("Hookshot"),
        ];
        let serialized = serde_yaml::to_string(&names).unwrap();
        let deserialized: Vec<ItemName> = serde_yaml::from_str(&serialized).unwrap();
        assert_eq!(names, deserialized);
    }

    // ===== MappingError Tests =====

    #[test]
    fn test_mapping_error_unknown_item() {
        let err = MappingError::unknown_item("FakeItem");
        assert_eq!(err.name, "FakeItem");
        assert!(err.message.contains("unknown"));
        assert!(err.message.contains("FakeItem"));
    }

    #[test]
    fn test_mapping_error_invalid_game() {
        let err = MappingError::invalid_game("DekuMask", "OoT");
        assert_eq!(err.name, "DekuMask");
        assert!(err.message.contains("DekuMask"));
        assert!(err.message.contains("OoT"));
    }

    #[test]
    fn test_mapping_error_display() {
        let err = MappingError::unknown_item("Test");
        let display = format!("{}", err);
        assert!(display.contains("unknown"));
    }

    // ===== ItemMapping Tests =====

    #[test]
    fn test_item_mapping_lookup() {
        let mapper = ItemMapping::new();
        assert_eq!(
            mapper.lookup("MasterSword"),
            Some(Item::Oot(OotItem::MasterSword))
        );
        assert_eq!(mapper.lookup("DekuMask"), Some(Item::Mm(MmItem::DekuMask)));
        assert_eq!(mapper.lookup("NotAnItem"), None);
    }

    #[test]
    fn test_item_mapping_lookup_oot() {
        let mapper = ItemMapping::new();
        assert_eq!(mapper.lookup_oot("MasterSword"), Some(OotItem::MasterSword));
        assert_eq!(mapper.lookup_oot("DekuMask"), None);
    }

    #[test]
    fn test_item_mapping_lookup_mm() {
        let mapper = ItemMapping::new();
        assert_eq!(mapper.lookup_mm("DekuMask"), Some(MmItem::DekuMask));
        assert_eq!(mapper.lookup_mm("Boomerang"), None);
    }

    #[test]
    fn test_item_mapping_name_of() {
        let mapper = ItemMapping::new();
        assert_eq!(
            mapper.name_of(Item::Oot(OotItem::Bow)).as_str(),
            "Bow"
        );
        assert_eq!(
            mapper.name_of(Item::Mm(MmItem::HerosBow)).as_str(),
            "HerosBow"
        );
    }

    #[test]
    fn test_item_mapping_parse_oot_items() {
        let mapper = ItemMapping::new();
        let (items, failed) =
            mapper.parse_oot_items(["MasterSword", "Hookshot", "DekuMask", "Invalid"]);
        assert_eq!(items, vec![OotItem::MasterSword, OotItem::Hookshot]);
        assert_eq!(failed, vec!["DekuMask", "Invalid"]);
    }

    #[test]
    fn test_item_mapping_parse_mm_items() {
        let mapper = ItemMapping::new();
        let (items, failed) =
            mapper.parse_mm_items(["DekuMask", "Hookshot", "Boomerang", "Invalid"]);
        assert_eq!(items, vec![MmItem::DekuMask, MmItem::Hookshot]);
        assert_eq!(failed, vec!["Boomerang", "Invalid"]);
    }

    #[test]
    fn test_item_mapping_parse_items() {
        let mapper = ItemMapping::new();
        let (items, failed) = mapper.parse_items(["MasterSword", "DekuMask", "Invalid"]);
        assert_eq!(
            items,
            vec![Item::Oot(OotItem::MasterSword), Item::Mm(MmItem::DekuMask)]
        );
        assert_eq!(failed, vec!["Invalid"]);
    }

    // ===== Edge Case Tests =====

    #[test]
    fn test_empty_item_name() {
        let name = ItemName::new("");
        assert!(!name.is_valid());
        assert!(name.to_item().is_err());
    }

    #[test]
    fn test_whitespace_item_name() {
        let name = ItemName::new("  MasterSword  ");
        assert!(!name.is_valid()); // Whitespace not trimmed
    }

    #[test]
    fn test_case_sensitive() {
        // Only PascalCase and snake_case are supported
        let name = ItemName::new("mastersword");
        assert!(!name.is_valid());

        let name = ItemName::new("MASTERSWORD");
        assert!(!name.is_valid());
    }

    #[test]
    fn test_reference_conversion() {
        let name = ItemName::new("MasterSword");
        let item: OotItem = (&name).try_into().unwrap();
        assert_eq!(item, OotItem::MasterSword);
        // name is still valid after reference conversion
        assert_eq!(name.as_str(), "MasterSword");
    }

    // ===== Hash and Equality Tests =====

    #[test]
    fn test_item_name_hash() {
        use std::collections::HashSet;
        let mut set = HashSet::new();
        set.insert(ItemName::new("MasterSword"));
        set.insert(ItemName::new("Hookshot"));
        set.insert(ItemName::new("MasterSword")); // Duplicate

        assert_eq!(set.len(), 2);
        assert!(set.contains(&ItemName::new("MasterSword")));
    }

    #[test]
    fn test_item_name_equality() {
        assert_eq!(ItemName::new("MasterSword"), ItemName::new("MasterSword"));
        assert_ne!(ItemName::new("MasterSword"), ItemName::new("master_sword"));
    }

    #[test]
    fn test_item_name_clone() {
        let name = ItemName::new("Hookshot");
        let cloned = name.clone();
        assert_eq!(name, cloned);
    }
}
