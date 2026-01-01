//! Item types for OoT and MM.

pub mod mm;
pub mod oot;

pub use mm::MmItem;
pub use oot::OotItem;

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
    use super::{Item, MmItem, OotItem};

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
        assert_eq!(
            Item::by_name("DekuMask"),
            Some(Item::Mm(MmItem::DekuMask))
        );
        assert_eq!(
            Item::by_name("deku_mask"),
            Some(Item::Mm(MmItem::DekuMask))
        );
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
}
