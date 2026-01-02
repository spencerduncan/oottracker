//! Item types for OoT and MM.

pub mod mm;
pub mod oot;

pub use mm::MmItem;
pub use oot::OotItem;

/// The game an item originates from.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Game {
    /// Ocarina of Time
    OcarinaOfTime,
    /// Majora's Mask
    MajorasMask,
}

/// Category of an item for classification purposes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
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
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
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
}
