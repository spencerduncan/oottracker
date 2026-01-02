use {
    crate::{Rando, RandoErr as _},
    quote_value::QuoteValue,
    serde::{Deserialize, Serialize},
};

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize, QuoteValue)]
#[serde(transparent)]
pub struct Item(pub String);

impl Item {
    pub fn from_str<R: Rando>(rando: &R, s: &str) -> Result<Item, R::Err> {
        rando
            .item_table()?
            .get(s)
            .cloned()
            .ok_or(R::Err::ITEM_NOT_FOUND)
    }

    pub fn name(&self) -> &str {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Rando, RandoErr, Regions};
    use std::{
        collections::{HashMap, HashSet},
        fmt,
        sync::Arc,
    };

    // Mock Rando implementation for testing Item
    #[derive(Debug, Clone)]
    struct MockErr;

    impl fmt::Display for MockErr {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "mock error")
        }
    }

    impl RandoErr for MockErr {
        const ITEM_NOT_FOUND: Self = MockErr;
    }

    #[derive(Debug, Clone, PartialEq, Eq, Hash)]
    struct MockRegionName(String);

    impl From<&'static str> for MockRegionName {
        fn from(s: &'static str) -> Self {
            MockRegionName(s.to_string())
        }
    }

    impl AsRef<str> for MockRegionName {
        fn as_ref(&self) -> &str {
            &self.0
        }
    }

    impl PartialEq<&str> for MockRegionName {
        fn eq(&self, other: &&str) -> bool {
            self.0.as_str() == *other
        }
    }

    impl fmt::Display for MockRegionName {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "{}", self.0)
        }
    }

    struct MockRando {
        items: HashMap<String, Item>,
    }

    impl MockRando {
        fn new() -> Self {
            Self {
                items: HashMap::new(),
            }
        }

        fn with_items(items: HashMap<String, Item>) -> Self {
            Self { items }
        }
    }

    impl Rando for MockRando {
        type Err = MockErr;
        type RegionName = MockRegionName;

        fn escaped_items(&self) -> Result<Arc<HashMap<String, Item>>, Self::Err> {
            Ok(Arc::new(HashMap::new()))
        }

        fn item_table(&self) -> Result<Arc<HashMap<String, Item>>, Self::Err> {
            Ok(Arc::new(self.items.clone()))
        }

        fn logic_tricks(&self) -> Result<Arc<HashSet<String>>, Self::Err> {
            Ok(Arc::new(HashSet::new()))
        }

        fn regions(&self) -> Result<Regions<Self>, Self::Err> {
            Ok(Arc::new(Vec::new()))
        }

        fn root() -> Self::RegionName {
            MockRegionName("Root".to_string())
        }

        fn setting_infos(&self) -> Result<Arc<HashSet<String>>, Self::Err> {
            Ok(Arc::new(HashSet::new()))
        }
    }

    #[test]
    fn test_item_name() {
        let item = Item("Kokiri Sword".to_string());
        assert_eq!(item.name(), "Kokiri Sword");

        let item2 = Item("Deku Shield".to_string());
        assert_eq!(item2.name(), "Deku Shield");
    }

    #[test]
    fn test_item_equality() {
        let item1 = Item("Master Sword".to_string());
        let item2 = Item("Master Sword".to_string());
        let item3 = Item("Hylian Shield".to_string());

        assert_eq!(item1, item2);
        assert_ne!(item1, item3);
    }

    #[test]
    fn test_item_from_str_found() {
        let mut items = HashMap::new();
        items.insert("Bow".to_string(), Item("Bow".to_string()));
        items.insert("Hookshot".to_string(), Item("Hookshot".to_string()));

        let rando = MockRando::with_items(items);

        let result = Item::from_str(&rando, "Bow");
        assert!(result.is_ok());
        assert_eq!(result.unwrap().name(), "Bow");

        let result2 = Item::from_str(&rando, "Hookshot");
        assert!(result2.is_ok());
        assert_eq!(result2.unwrap().name(), "Hookshot");
    }

    #[test]
    fn test_item_from_str_not_found() {
        let rando = MockRando::new();

        let result = Item::from_str(&rando, "NonexistentItem");
        assert!(result.is_err());
    }
}
