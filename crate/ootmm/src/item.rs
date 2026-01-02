//! Item types for OoT and MM.

pub mod mm;
pub mod oot;

pub use mm::MmItem;
pub use oot::OotItem;

use serde::{Deserialize, Serialize};

/// The game an item originates from.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Game {
    /// Ocarina of Time
    OcarinaOfTime,
    /// Majora's Mask
    MajorasMask,
}

/// Category of an item for classification purposes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ItemCategory {
    /// Swords (Kokiri Sword, Master Sword, etc.)
    Sword,
    /// Shields (Deku Shield, Hylian Shield, etc.)
    Shield,
    /// Tunics (OoT only - Kokiri, Goron, Zora Tunics)
    Tunic,
    /// Boots (Kokiri, Iron, Hover Boots)
    Boots,
    /// Regular masks (non-transformation)
    Mask,
    /// Transformation masks (MM - Deku, Goron, Zora, Fierce Deity)
    TransformationMask,
    /// Equipment items (Bow, Hookshot, etc.)
    Equipment,
    /// Magic spells (Din's Fire, etc.)
    Magic,
    /// Songs
    Song,
    /// Ocarinas
    Ocarina,
    /// Generic dungeon items (Map, Compass)
    DungeonItem,
    /// Small keys (both generic and dungeon-specific)
    SmallKey,
    /// Boss keys (both generic and dungeon-specific)
    BossKey,
    /// Capacity/ability upgrades
    Upgrade,
    /// Consumable items (rupees, hearts, ammo)
    Consumable,
    /// Quest items (medallions, stones, remains, etc.)
    QuestItem,
    /// Collectible tokens (Gold Skulltulas, Stray Fairies)
    Token,
    /// Bottles and bottle contents
    Bottle,
    /// Trade sequence items
    Trade,
    /// Special/unique items
    Special,
}

/// Combined item enum for both games.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Item {
    /// An item from Ocarina of Time.
    Oot(OotItem),
    /// An item from Majora's Mask.
    Mm(MmItem),
}

impl Item {
    /// Look up an Item by its string name.
    ///
    /// Tries OoT items first, then MM items. Supports both PascalCase
    /// variant names (e.g., "MasterSword") and snake_case names (e.g., "master_sword").
    ///
    /// Note: Items with identical names in both games (e.g., "Hookshot") will
    /// return the OoT variant. Use `OotItem::by_name` or `MmItem::by_name`
    /// directly if you need a specific game's item.
    #[must_use]
    pub fn by_name(name: &str) -> Option<Item> {
        OotItem::by_name(name)
            .map(Item::Oot)
            .or_else(|| MmItem::by_name(name).map(Item::Mm))
    }

    /// Returns the game this item originates from.
    #[must_use]
    pub const fn game(&self) -> Game {
        match self {
            Item::Oot(_) => Game::OcarinaOfTime,
            Item::Mm(_) => Game::MajorasMask,
        }
    }

    /// Returns the category of this item.
    #[must_use]
    pub const fn category(&self) -> ItemCategory {
        match self {
            Item::Oot(item) => item.category(),
            Item::Mm(item) => item.category(),
        }
    }

    /// Returns true if this is a progressive item that can be collected multiple times
    /// and contributes to item progression (e.g., heart pieces, skulltulas).
    #[must_use]
    pub const fn is_progressive(&self) -> bool {
        match self {
            Item::Oot(item) => item.is_progressive(),
            Item::Mm(item) => item.is_progressive(),
        }
    }

    /// Returns true if this item can stack (be collected multiple times).
    #[must_use]
    pub const fn is_stackable(&self) -> bool {
        match self {
            Item::Oot(item) => item.is_stackable(),
            Item::Mm(item) => item.is_stackable(),
        }
    }

    /// Returns the maximum count for this item.
    /// Returns 1 for non-stackable items.
    #[must_use]
    pub const fn max_count(&self) -> u32 {
        match self {
            Item::Oot(item) => item.max_count(),
            Item::Mm(item) => item.max_count(),
        }
    }
}

impl From<OotItem> for Item {
    fn from(item: OotItem) -> Self {
        Item::Oot(item)
    }
}

impl From<MmItem> for Item {
    fn from(item: MmItem) -> Self {
        Item::Mm(item)
    }
}

#[cfg(test)]
mod tests {
    use super::{Game, Item, ItemCategory, MmItem, OotItem};
    use serde_yaml;

    #[test]
    fn test_from_oot_item() {
        let item: Item = OotItem::MasterSword.into();
        assert_eq!(item, Item::Oot(OotItem::MasterSword));
    }

    #[test]
    fn test_from_mm_item() {
        let item: Item = MmItem::DekuMask.into();
        assert_eq!(item, Item::Mm(MmItem::DekuMask));
    }

    #[test]
    fn test_by_name_oot_specific() {
        // Items unique to OoT
        assert_eq!(
            Item::by_name("MasterSword"),
            Some(Item::Oot(OotItem::MasterSword))
        );
        assert_eq!(
            Item::by_name("master_sword"),
            Some(Item::Oot(OotItem::MasterSword))
        );
        assert_eq!(
            Item::by_name("Boomerang"),
            Some(Item::Oot(OotItem::Boomerang))
        );
    }

    #[test]
    fn test_by_name_mm_specific() {
        // Items unique to MM
        assert_eq!(Item::by_name("DekuMask"), Some(Item::Mm(MmItem::DekuMask)));
        assert_eq!(Item::by_name("deku_mask"), Some(Item::Mm(MmItem::DekuMask)));
        assert_eq!(
            Item::by_name("OdolwaRemains"),
            Some(Item::Mm(MmItem::OdolwaRemains))
        );
    }

    #[test]
    fn test_by_name_shared_returns_oot() {
        // Items that exist in both games should return OoT variant first
        assert_eq!(
            Item::by_name("Hookshot"),
            Some(Item::Oot(OotItem::Hookshot))
        );
        assert_eq!(Item::by_name("Bomb"), Some(Item::Oot(OotItem::Bomb)));
    }

    #[test]
    fn test_by_name_not_found() {
        assert_eq!(Item::by_name("NotAnItem"), None);
        assert_eq!(Item::by_name(""), None);
    }

    #[test]
    fn test_game() {
        assert_eq!(Item::Oot(OotItem::MasterSword).game(), Game::OcarinaOfTime);
        assert_eq!(Item::Mm(MmItem::DekuMask).game(), Game::MajorasMask);
    }

    #[test]
    fn test_category_swords() {
        assert_eq!(
            Item::Oot(OotItem::MasterSword).category(),
            ItemCategory::Sword
        );
        assert_eq!(
            Item::Mm(MmItem::GildedSword).category(),
            ItemCategory::Sword
        );
    }

    #[test]
    fn test_category_masks() {
        // OoT masks are regular masks (trade-related)
        assert_eq!(Item::Oot(OotItem::BunnyHood).category(), ItemCategory::Mask);
        // MM transformation masks
        assert_eq!(
            Item::Mm(MmItem::DekuMask).category(),
            ItemCategory::TransformationMask
        );
        // MM regular masks
        assert_eq!(Item::Mm(MmItem::BunnyHood).category(), ItemCategory::Mask);
    }

    #[test]
    fn test_category_songs() {
        assert_eq!(
            Item::Oot(OotItem::ZeldasLullaby).category(),
            ItemCategory::Song
        );
        assert_eq!(
            Item::Mm(MmItem::SongOfHealing).category(),
            ItemCategory::Song
        );
    }

    #[test]
    fn test_category_keys() {
        assert_eq!(
            Item::Oot(OotItem::SmallKeyFireTemple).category(),
            ItemCategory::SmallKey
        );
        assert_eq!(
            Item::Oot(OotItem::BossKeyFireTemple).category(),
            ItemCategory::BossKey
        );
        assert_eq!(
            Item::Mm(MmItem::SmallKeyWoodfallTemple).category(),
            ItemCategory::SmallKey
        );
    }

    #[test]
    fn test_is_progressive() {
        assert!(Item::Oot(OotItem::PieceOfHeart).is_progressive());
        assert!(Item::Oot(OotItem::GoldSkulltula).is_progressive());
        assert!(Item::Mm(MmItem::StrayFairy).is_progressive());
        assert!(!Item::Oot(OotItem::MasterSword).is_progressive());
        assert!(!Item::Mm(MmItem::DekuMask).is_progressive());
    }

    #[test]
    fn test_is_stackable() {
        assert!(Item::Oot(OotItem::SmallKeyFireTemple).is_stackable());
        assert!(Item::Oot(OotItem::GoldSkulltula).is_stackable());
        assert!(Item::Mm(MmItem::StrayFairyWoodfall).is_stackable());
        assert!(!Item::Oot(OotItem::MasterSword).is_stackable());
        assert!(!Item::Mm(MmItem::Hookshot).is_stackable());
    }

    #[test]
    fn test_max_count() {
        // OoT
        assert_eq!(Item::Oot(OotItem::SmallKeyFireTemple).max_count(), 8);
        assert_eq!(Item::Oot(OotItem::GoldSkulltula).max_count(), 100);
        assert_eq!(Item::Oot(OotItem::PieceOfHeart).max_count(), 36);
        assert_eq!(Item::Oot(OotItem::Bottle).max_count(), 4);
        assert_eq!(Item::Oot(OotItem::MasterSword).max_count(), 1);
        // MM
        assert_eq!(Item::Mm(MmItem::SmallKeySnowheadTemple).max_count(), 3);
        assert_eq!(Item::Mm(MmItem::StrayFairyWoodfall).max_count(), 15);
        assert_eq!(Item::Mm(MmItem::PieceOfHeart).max_count(), 52);
        assert_eq!(Item::Mm(MmItem::Bottle).max_count(), 6);
        assert_eq!(Item::Mm(MmItem::Hookshot).max_count(), 1);
    }

    // ===== SERDE ROUNDTRIP TESTS =====

    #[test]
    fn test_serde_roundtrip_game_oot() {
        let game = Game::OcarinaOfTime;
        let serialized = serde_yaml::to_string(&game).unwrap();
        let deserialized: Game = serde_yaml::from_str(&serialized).unwrap();
        assert_eq!(game, deserialized);
    }

    #[test]
    fn test_serde_roundtrip_game_mm() {
        let game = Game::MajorasMask;
        let serialized = serde_yaml::to_string(&game).unwrap();
        let deserialized: Game = serde_yaml::from_str(&serialized).unwrap();
        assert_eq!(game, deserialized);
    }

    #[test]
    fn test_serde_roundtrip_oot_item_sword() {
        let item = OotItem::MasterSword;
        let serialized = serde_yaml::to_string(&item).unwrap();
        let deserialized: OotItem = serde_yaml::from_str(&serialized).unwrap();
        assert_eq!(item, deserialized);
    }

    #[test]
    fn test_serde_roundtrip_oot_item_equipment() {
        let items = vec![
            OotItem::Hookshot,
            OotItem::Longshot,
            OotItem::Boomerang,
            OotItem::Bow,
            OotItem::MegatonHammer,
        ];
        for item in items {
            let serialized = serde_yaml::to_string(&item).unwrap();
            let deserialized: OotItem = serde_yaml::from_str(&serialized).unwrap();
            assert_eq!(item, deserialized);
        }
    }

    #[test]
    fn test_serde_roundtrip_oot_item_songs() {
        let items = vec![
            OotItem::ZeldasLullaby,
            OotItem::EponasSong,
            OotItem::BoleroOfFire,
            OotItem::PreludeOfLight,
        ];
        for item in items {
            let serialized = serde_yaml::to_string(&item).unwrap();
            let deserialized: OotItem = serde_yaml::from_str(&serialized).unwrap();
            assert_eq!(item, deserialized);
        }
    }

    #[test]
    fn test_serde_roundtrip_oot_item_dungeon_keys() {
        let items = vec![
            OotItem::SmallKeyFireTemple,
            OotItem::BossKeyWaterTemple,
            OotItem::Map,
            OotItem::Compass,
        ];
        for item in items {
            let serialized = serde_yaml::to_string(&item).unwrap();
            let deserialized: OotItem = serde_yaml::from_str(&serialized).unwrap();
            assert_eq!(item, deserialized);
        }
    }

    #[test]
    fn test_serde_roundtrip_mm_item_masks() {
        let items = vec![
            MmItem::DekuMask,
            MmItem::GoronMask,
            MmItem::ZoraMask,
            MmItem::FierceDeityMask,
            MmItem::BunnyHood,
        ];
        for item in items {
            let serialized = serde_yaml::to_string(&item).unwrap();
            let deserialized: MmItem = serde_yaml::from_str(&serialized).unwrap();
            assert_eq!(item, deserialized);
        }
    }

    #[test]
    fn test_serde_roundtrip_mm_item_songs() {
        let items = vec![
            MmItem::SongOfTime,
            MmItem::SongOfHealing,
            MmItem::OathToOrder,
            MmItem::ElegyOfEmptiness,
        ];
        for item in items {
            let serialized = serde_yaml::to_string(&item).unwrap();
            let deserialized: MmItem = serde_yaml::from_str(&serialized).unwrap();
            assert_eq!(item, deserialized);
        }
    }

    #[test]
    fn test_serde_roundtrip_mm_item_boss_remains() {
        let items = vec![
            MmItem::OdolwaRemains,
            MmItem::GohtRemains,
            MmItem::GyorgRemains,
            MmItem::TwinmoldRemains,
        ];
        for item in items {
            let serialized = serde_yaml::to_string(&item).unwrap();
            let deserialized: MmItem = serde_yaml::from_str(&serialized).unwrap();
            assert_eq!(item, deserialized);
        }
    }

    #[test]
    fn test_serde_roundtrip_combined_item_oot() {
        let item = Item::Oot(OotItem::MasterSword);
        let serialized = serde_yaml::to_string(&item).unwrap();
        let deserialized: Item = serde_yaml::from_str(&serialized).unwrap();
        assert_eq!(item, deserialized);
    }

    #[test]
    fn test_serde_roundtrip_combined_item_mm() {
        let item = Item::Mm(MmItem::DekuMask);
        let serialized = serde_yaml::to_string(&item).unwrap();
        let deserialized: Item = serde_yaml::from_str(&serialized).unwrap();
        assert_eq!(item, deserialized);
    }

    #[test]
    fn test_serde_roundtrip_item_category_all() {
        let categories = vec![
            ItemCategory::Sword,
            ItemCategory::Shield,
            ItemCategory::Tunic,
            ItemCategory::Boots,
            ItemCategory::Mask,
            ItemCategory::TransformationMask,
            ItemCategory::Equipment,
            ItemCategory::Magic,
            ItemCategory::Song,
            ItemCategory::Ocarina,
            ItemCategory::DungeonItem,
            ItemCategory::SmallKey,
            ItemCategory::BossKey,
            ItemCategory::Upgrade,
            ItemCategory::Consumable,
            ItemCategory::QuestItem,
            ItemCategory::Token,
            ItemCategory::Bottle,
            ItemCategory::Trade,
            ItemCategory::Special,
        ];
        for category in categories {
            let serialized = serde_yaml::to_string(&category).unwrap();
            let deserialized: ItemCategory = serde_yaml::from_str(&serialized).unwrap();
            assert_eq!(category, deserialized);
        }
    }

    #[test]
    fn test_serde_deserialize_oot_item_from_string() {
        let yaml = "MasterSword";
        let item: OotItem = serde_yaml::from_str(yaml).unwrap();
        assert_eq!(item, OotItem::MasterSword);
    }

    #[test]
    fn test_serde_deserialize_mm_item_from_string() {
        let yaml = "DekuMask";
        let item: MmItem = serde_yaml::from_str(yaml).unwrap();
        assert_eq!(item, MmItem::DekuMask);
    }

    // ===== COMPREHENSIVE CATEGORY TESTS =====

    #[test]
    fn test_category_all_oot_swords() {
        assert_eq!(OotItem::KokiriSword.category(), ItemCategory::Sword);
        assert_eq!(OotItem::MasterSword.category(), ItemCategory::Sword);
        assert_eq!(OotItem::BiggoronSword.category(), ItemCategory::Sword);
        assert_eq!(OotItem::GiantKnife.category(), ItemCategory::Sword);
    }

    #[test]
    fn test_category_all_oot_shields() {
        assert_eq!(OotItem::DekuShield.category(), ItemCategory::Shield);
        assert_eq!(OotItem::HylianShield.category(), ItemCategory::Shield);
        assert_eq!(OotItem::MirrorShield.category(), ItemCategory::Shield);
    }

    #[test]
    fn test_category_all_oot_tunics() {
        assert_eq!(OotItem::KokiriTunic.category(), ItemCategory::Tunic);
        assert_eq!(OotItem::GoronTunic.category(), ItemCategory::Tunic);
        assert_eq!(OotItem::ZoraTunic.category(), ItemCategory::Tunic);
    }

    #[test]
    fn test_category_all_oot_boots() {
        assert_eq!(OotItem::KokiriBoots.category(), ItemCategory::Boots);
        assert_eq!(OotItem::IronBoots.category(), ItemCategory::Boots);
        assert_eq!(OotItem::HoverBoots.category(), ItemCategory::Boots);
    }

    #[test]
    fn test_category_all_oot_magic() {
        assert_eq!(OotItem::DinsFire.category(), ItemCategory::Magic);
        assert_eq!(OotItem::FaroresWind.category(), ItemCategory::Magic);
        assert_eq!(OotItem::NayrusLove.category(), ItemCategory::Magic);
    }

    #[test]
    fn test_category_all_oot_quest_items() {
        assert_eq!(OotItem::KokiriEmerald.category(), ItemCategory::QuestItem);
        assert_eq!(OotItem::GoronRuby.category(), ItemCategory::QuestItem);
        assert_eq!(OotItem::ZoraSapphire.category(), ItemCategory::QuestItem);
        assert_eq!(OotItem::ForestMedallion.category(), ItemCategory::QuestItem);
        assert_eq!(OotItem::FireMedallion.category(), ItemCategory::QuestItem);
        assert_eq!(OotItem::WaterMedallion.category(), ItemCategory::QuestItem);
        assert_eq!(OotItem::ShadowMedallion.category(), ItemCategory::QuestItem);
        assert_eq!(OotItem::SpiritMedallion.category(), ItemCategory::QuestItem);
        assert_eq!(OotItem::LightMedallion.category(), ItemCategory::QuestItem);
    }

    #[test]
    fn test_category_all_oot_bottles() {
        assert_eq!(OotItem::Bottle.category(), ItemCategory::Bottle);
        assert_eq!(OotItem::BottleRedPotion.category(), ItemCategory::Bottle);
        assert_eq!(OotItem::BottleFairy.category(), ItemCategory::Bottle);
        assert_eq!(OotItem::BottleBlueFire.category(), ItemCategory::Bottle);
        assert_eq!(OotItem::BottleRutosLetter.category(), ItemCategory::Bottle);
    }

    #[test]
    fn test_category_all_mm_transformation_masks() {
        assert_eq!(
            MmItem::DekuMask.category(),
            ItemCategory::TransformationMask
        );
        assert_eq!(
            MmItem::GoronMask.category(),
            ItemCategory::TransformationMask
        );
        assert_eq!(
            MmItem::ZoraMask.category(),
            ItemCategory::TransformationMask
        );
        assert_eq!(
            MmItem::FierceDeityMask.category(),
            ItemCategory::TransformationMask
        );
    }

    #[test]
    fn test_category_all_mm_regular_masks() {
        assert_eq!(MmItem::PostmanHat.category(), ItemCategory::Mask);
        assert_eq!(MmItem::AllNightMask.category(), ItemCategory::Mask);
        assert_eq!(MmItem::BlastMask.category(), ItemCategory::Mask);
        assert_eq!(MmItem::StoneMask.category(), ItemCategory::Mask);
        assert_eq!(MmItem::BunnyHood.category(), ItemCategory::Mask);
        assert_eq!(MmItem::GiantMask.category(), ItemCategory::Mask);
    }

    #[test]
    fn test_category_all_mm_boss_remains() {
        assert_eq!(MmItem::OdolwaRemains.category(), ItemCategory::QuestItem);
        assert_eq!(MmItem::GohtRemains.category(), ItemCategory::QuestItem);
        assert_eq!(MmItem::GyorgRemains.category(), ItemCategory::QuestItem);
        assert_eq!(MmItem::TwinmoldRemains.category(), ItemCategory::QuestItem);
    }

    #[test]
    fn test_category_all_mm_stray_fairies() {
        assert_eq!(MmItem::StrayFairy.category(), ItemCategory::Token);
        assert_eq!(MmItem::StrayFairyWoodfall.category(), ItemCategory::Token);
        assert_eq!(MmItem::StrayFairySnowhead.category(), ItemCategory::Token);
        assert_eq!(MmItem::StrayFairyGreatBay.category(), ItemCategory::Token);
        assert_eq!(MmItem::StrayFairyStoneTower.category(), ItemCategory::Token);
        assert_eq!(MmItem::StrayFairyClockTown.category(), ItemCategory::Token);
    }

    #[test]
    fn test_category_token_items() {
        // OoT tokens
        assert_eq!(OotItem::GoldSkulltula.category(), ItemCategory::Token);
        // MM tokens (stray fairies)
        assert_eq!(MmItem::StrayFairy.category(), ItemCategory::Token);
    }

    #[test]
    fn test_category_special_items() {
        assert_eq!(OotItem::Triforce.category(), ItemCategory::Special);
        assert_eq!(OotItem::TriforceOfCourage.category(), ItemCategory::Special);
        assert_eq!(MmItem::BomberNotebook.category(), ItemCategory::Special);
    }

    #[test]
    fn test_category_ocarina() {
        assert_eq!(OotItem::OcarinaOfTime.category(), ItemCategory::Ocarina);
        assert_eq!(MmItem::OcarinaOfTime.category(), ItemCategory::Ocarina);
    }

    // ===== GAME ENUM TESTS =====

    #[test]
    fn test_game_enum_debug() {
        assert_eq!(format!("{:?}", Game::OcarinaOfTime), "OcarinaOfTime");
        assert_eq!(format!("{:?}", Game::MajorasMask), "MajorasMask");
    }

    #[test]
    fn test_game_enum_equality() {
        assert_eq!(Game::OcarinaOfTime, Game::OcarinaOfTime);
        assert_eq!(Game::MajorasMask, Game::MajorasMask);
        assert_ne!(Game::OcarinaOfTime, Game::MajorasMask);
    }

    #[test]
    fn test_game_enum_clone_copy() {
        let game = Game::OcarinaOfTime;
        let copied = game;
        #[allow(clippy::clone_on_copy)]
        let cloned = game.clone();
        assert_eq!(game, copied);
        assert_eq!(game, cloned);
    }

    // ===== ITEM CATEGORY TESTS =====

    #[test]
    fn test_item_category_debug() {
        assert_eq!(format!("{:?}", ItemCategory::Sword), "Sword");
        assert_eq!(
            format!("{:?}", ItemCategory::TransformationMask),
            "TransformationMask"
        );
        assert_eq!(format!("{:?}", ItemCategory::Equipment), "Equipment");
    }

    #[test]
    fn test_item_category_equality() {
        assert_eq!(ItemCategory::Sword, ItemCategory::Sword);
        assert_ne!(ItemCategory::Sword, ItemCategory::Shield);
    }

    #[test]
    fn test_item_category_clone_copy() {
        let cat = ItemCategory::Song;
        let copied = cat;
        #[allow(clippy::clone_on_copy)]
        let cloned = cat.clone();
        assert_eq!(cat, copied);
        assert_eq!(cat, cloned);
    }

    #[test]
    fn test_item_category_hash() {
        use std::collections::HashSet;
        let mut set = HashSet::new();
        set.insert(ItemCategory::Sword);
        set.insert(ItemCategory::Shield);
        set.insert(ItemCategory::Sword); // duplicate
        assert_eq!(set.len(), 2);
    }
}
