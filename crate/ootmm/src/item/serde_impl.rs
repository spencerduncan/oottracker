//! Serde implementations for item types.
//!
//! Items are serialized as SCREAMING_SNAKE_CASE strings.

use serde::{de, Deserialize, Deserializer, Serialize, Serializer};

use super::{Item, ItemName, MmItem, OotItem};

impl Serialize for OotItem {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(self.to_str())
    }
}

impl<'de> Deserialize<'de> for OotItem {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        OotItem::from_str(&s).ok_or_else(|| de::Error::unknown_variant(&s, &["valid OotItem name"]))
    }
}

impl Serialize for MmItem {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(self.to_str())
    }
}

impl<'de> Deserialize<'de> for MmItem {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        MmItem::from_str(&s).ok_or_else(|| de::Error::unknown_variant(&s, &["valid MmItem name"]))
    }
}

impl Serialize for Item {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(self.to_str())
    }
}

impl<'de> Deserialize<'de> for Item {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Item::from_str(&s).ok_or_else(|| de::Error::unknown_variant(&s, &["valid Item name"]))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_oot_item_serialize() {
        let item = OotItem::MasterSword;
        let json = serde_yaml::to_string(&item).unwrap();
        assert_eq!(json.trim(), "MASTER_SWORD");
    }

    #[test]
    fn test_oot_item_deserialize() {
        let item: OotItem = serde_yaml::from_str("MASTER_SWORD").unwrap();
        assert_eq!(item, OotItem::MasterSword);
    }

    #[test]
    fn test_oot_item_roundtrip() {
        let items = [
            OotItem::MasterSword,
            OotItem::Hookshot,
            OotItem::ZeldasLullaby,
            OotItem::SmallKeyFireTemple,
            OotItem::BossKeyForestTemple,
        ];

        for item in items {
            let yaml = serde_yaml::to_string(&item).unwrap();
            let parsed: OotItem = serde_yaml::from_str(&yaml).unwrap();
            assert_eq!(item, parsed);
        }
    }

    #[test]
    fn test_mm_item_serialize() {
        let item = MmItem::DekuMask;
        let json = serde_yaml::to_string(&item).unwrap();
        assert_eq!(json.trim(), "DEKU_MASK");
    }

    #[test]
    fn test_mm_item_deserialize() {
        let item: MmItem = serde_yaml::from_str("DEKU_MASK").unwrap();
        assert_eq!(item, MmItem::DekuMask);
    }

    #[test]
    fn test_mm_item_roundtrip() {
        let items = [
            MmItem::DekuMask,
            MmItem::Hookshot,
            MmItem::OdolwaRemains,
            MmItem::SmallKeyWoodfallTemple,
            MmItem::StrayFairySnowhead,
        ];

        for item in items {
            let yaml = serde_yaml::to_string(&item).unwrap();
            let parsed: MmItem = serde_yaml::from_str(&yaml).unwrap();
            assert_eq!(item, parsed);
        }
    }

    #[test]
    fn test_item_serialize() {
        let oot_item = Item::Oot(OotItem::MasterSword);
        let yaml = serde_yaml::to_string(&oot_item).unwrap();
        assert_eq!(yaml.trim(), "OOT_MASTER_SWORD");

        let mm_item = Item::Mm(MmItem::DekuMask);
        let yaml = serde_yaml::to_string(&mm_item).unwrap();
        assert_eq!(yaml.trim(), "MM_DEKU_MASK");
    }

    #[test]
    fn test_item_deserialize() {
        let item: Item = serde_yaml::from_str("OOT_MASTER_SWORD").unwrap();
        assert_eq!(item, Item::Oot(OotItem::MasterSword));

        let item: Item = serde_yaml::from_str("MM_DEKU_MASK").unwrap();
        assert_eq!(item, Item::Mm(MmItem::DekuMask));
    }

    #[test]
    fn test_item_roundtrip() {
        let items = [
            Item::Oot(OotItem::MasterSword),
            Item::Oot(OotItem::Hookshot),
            Item::Mm(MmItem::DekuMask),
            Item::Mm(MmItem::OdolwaRemains),
        ];

        for item in items {
            let yaml = serde_yaml::to_string(&item).unwrap();
            let parsed: Item = serde_yaml::from_str(&yaml).unwrap();
            assert_eq!(item, parsed);
        }
    }

    #[test]
    fn test_deserialize_invalid_item() {
        let result: Result<OotItem, _> = serde_yaml::from_str("INVALID_ITEM");
        assert!(result.is_err());

        let result: Result<MmItem, _> = serde_yaml::from_str("INVALID_ITEM");
        assert!(result.is_err());

        let result: Result<Item, _> = serde_yaml::from_str("INVALID_ITEM");
        assert!(result.is_err());
    }
}
