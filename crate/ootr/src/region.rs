use {
    crate::{model::Dungeon, Rando},
    async_proto::Protocol,
    quote_value::QuoteValue,
    serde::{Deserialize, Serialize},
    std::{
        collections::HashSet,
        fmt,
        hash::{Hash, Hasher},
    },
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Protocol, Deserialize, Serialize, QuoteValue)]
#[serde(rename_all = "snake_case")]
pub enum Mq {
    Vanilla,
    Mq,
}

impl fmt::Display for Mq {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Mq::Vanilla => write!(f, "vanilla"),
            Mq::Mq => write!(f, "MQ"),
        }
    }
}

#[derive(Debug, Clone, QuoteValue)]
#[quote_value(where(R::RegionName: QuoteValue))]
pub struct Region<R: Rando> {
    pub name: R::RegionName,
    pub dungeon: Option<(Dungeon, Mq)>,
    pub scene: Option<String>, //TODO use Scene type from oottracker?
    pub hint: Option<String>,
    pub time_passes: bool,
    pub events: HashSet<String>,
    pub locations: HashSet<String>,
    pub exits: HashSet<R::RegionName>,
}

impl<R: Rando> PartialEq for Region<R> {
    fn eq(&self, other: &Region<R>) -> bool {
        self.dungeon == other.dungeon && self.name == other.name
    }
}

impl<R: Rando> Eq for Region<R> {}

impl<R: Rando> Hash for Region<R> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.name.hash(state);
        self.dungeon.hash(state);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::hash_map::DefaultHasher;

    // Helper to compute hash for any hashable value
    fn compute_hash<T: Hash>(value: &T) -> u64 {
        let mut hasher = DefaultHasher::new();
        value.hash(&mut hasher);
        hasher.finish()
    }

    #[test]
    fn test_mq_display() {
        assert_eq!(format!("{}", Mq::Vanilla), "vanilla");
        assert_eq!(format!("{}", Mq::Mq), "MQ");
    }

    #[test]
    fn test_mq_equality() {
        assert_eq!(Mq::Vanilla, Mq::Vanilla);
        assert_eq!(Mq::Mq, Mq::Mq);
        assert_ne!(Mq::Vanilla, Mq::Mq);
    }

    #[test]
    fn test_mq_hash_consistency() {
        // Same values should produce the same hash
        let hash1 = compute_hash(&Mq::Vanilla);
        let hash2 = compute_hash(&Mq::Vanilla);
        assert_eq!(hash1, hash2);

        // Different values should produce different hashes
        let hash_vanilla = compute_hash(&Mq::Vanilla);
        let hash_mq = compute_hash(&Mq::Mq);
        assert_ne!(hash_vanilla, hash_mq);
    }

    // Mock Rando implementation for testing Region
    mod mock {
        use crate::{item::Item, Rando, RandoErr, Regions};
        use std::{
            collections::{HashMap, HashSet},
            fmt,
            sync::Arc,
        };

        #[derive(Debug, Clone)]
        pub struct MockErr;

        impl fmt::Display for MockErr {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                write!(f, "mock error")
            }
        }

        impl RandoErr for MockErr {
            const ITEM_NOT_FOUND: Self = MockErr;
        }

        #[derive(Debug, Clone, PartialEq, Eq, Hash)]
        pub struct MockRegionName(pub String);

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

        #[derive(Debug)]
        pub struct MockRando;

        impl Rando for MockRando {
            type Err = MockErr;
            type RegionName = MockRegionName;

            fn escaped_items(&self) -> Result<Arc<HashMap<String, Item>>, Self::Err> {
                Ok(Arc::new(HashMap::new()))
            }

            fn item_table(&self) -> Result<Arc<HashMap<String, Item>>, Self::Err> {
                Ok(Arc::new(HashMap::new()))
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
    }

    #[test]
    fn test_region_equality_ignores_non_key_fields() {
        use mock::{MockRando, MockRegionName};

        // Two regions with same name and dungeon but different other fields
        let region1: Region<MockRando> = Region {
            name: MockRegionName::from("Kokiri Forest"),
            dungeon: None,
            scene: Some("kokiri".to_string()),
            hint: Some("hint1".to_string()),
            time_passes: true,
            events: HashSet::from(["event1".to_string()]),
            locations: HashSet::from(["loc1".to_string()]),
            exits: HashSet::from([MockRegionName::from("Lost Woods")]),
        };

        let region2: Region<MockRando> = Region {
            name: MockRegionName::from("Kokiri Forest"),
            dungeon: None,
            scene: Some("different_scene".to_string()),
            hint: Some("hint2".to_string()),
            time_passes: false,
            events: HashSet::from(["event2".to_string()]),
            locations: HashSet::from(["loc2".to_string()]),
            exits: HashSet::new(),
        };

        // Should be equal since name and dungeon match
        assert_eq!(region1, region2);
    }

    #[test]
    fn test_region_inequality_on_dungeon_difference() {
        use crate::model::MainDungeon;
        use mock::{MockRando, MockRegionName};

        let region1: Region<MockRando> = Region {
            name: MockRegionName::from("Temple"),
            dungeon: Some((Dungeon::Main(MainDungeon::ForestTemple), Mq::Vanilla)),
            scene: None,
            hint: None,
            time_passes: false,
            events: HashSet::new(),
            locations: HashSet::new(),
            exits: HashSet::new(),
        };

        let region2: Region<MockRando> = Region {
            name: MockRegionName::from("Temple"),
            dungeon: Some((Dungeon::Main(MainDungeon::ForestTemple), Mq::Mq)),
            scene: None,
            hint: None,
            time_passes: false,
            events: HashSet::new(),
            locations: HashSet::new(),
            exits: HashSet::new(),
        };

        // Should not be equal since dungeon MQ status differs
        assert_ne!(region1, region2);
    }

    #[test]
    fn test_region_inequality_on_name_difference() {
        use mock::{MockRando, MockRegionName};

        let region1: Region<MockRando> = Region {
            name: MockRegionName::from("Kokiri Forest"),
            dungeon: None,
            scene: None,
            hint: None,
            time_passes: false,
            events: HashSet::new(),
            locations: HashSet::new(),
            exits: HashSet::new(),
        };

        let region2: Region<MockRando> = Region {
            name: MockRegionName::from("Lost Woods"),
            dungeon: None,
            scene: None,
            hint: None,
            time_passes: false,
            events: HashSet::new(),
            locations: HashSet::new(),
            exits: HashSet::new(),
        };

        // Should not be equal since names differ
        assert_ne!(region1, region2);
    }

    #[test]
    fn test_region_hash_consistency_with_equality() {
        use mock::{MockRando, MockRegionName};

        // Two equal regions should have the same hash
        let region1: Region<MockRando> = Region {
            name: MockRegionName::from("Hyrule Field"),
            dungeon: None,
            scene: Some("hyrule_field".to_string()),
            hint: Some("hint1".to_string()),
            time_passes: true,
            events: HashSet::from(["event1".to_string()]),
            locations: HashSet::from(["loc1".to_string()]),
            exits: HashSet::from([MockRegionName::from("Kakariko")]),
        };

        let region2: Region<MockRando> = Region {
            name: MockRegionName::from("Hyrule Field"),
            dungeon: None,
            scene: Some("different_scene".to_string()),
            hint: Some("hint2".to_string()),
            time_passes: false,
            events: HashSet::from(["event2".to_string()]),
            locations: HashSet::from(["loc2".to_string()]),
            exits: HashSet::new(),
        };

        // Should be equal (ignoring non-key fields)
        assert_eq!(region1, region2);

        // Equal regions must have equal hashes (Rust contract)
        let hash1 = compute_hash(&region1);
        let hash2 = compute_hash(&region2);
        assert_eq!(
            hash1, hash2,
            "Equal regions must have equal hashes"
        );
    }

    #[test]
    fn test_region_hash_differs_on_name() {
        use mock::{MockRando, MockRegionName};

        let region1: Region<MockRando> = Region {
            name: MockRegionName::from("Kokiri Forest"),
            dungeon: None,
            scene: None,
            hint: None,
            time_passes: false,
            events: HashSet::new(),
            locations: HashSet::new(),
            exits: HashSet::new(),
        };

        let region2: Region<MockRando> = Region {
            name: MockRegionName::from("Lost Woods"),
            dungeon: None,
            scene: None,
            hint: None,
            time_passes: false,
            events: HashSet::new(),
            locations: HashSet::new(),
            exits: HashSet::new(),
        };

        // Different names should produce different hashes (with high probability)
        let hash1 = compute_hash(&region1);
        let hash2 = compute_hash(&region2);
        assert_ne!(
            hash1, hash2,
            "Different region names should produce different hashes"
        );
    }

    #[test]
    fn test_region_hash_differs_on_dungeon() {
        use crate::model::MainDungeon;
        use mock::{MockRando, MockRegionName};

        let region1: Region<MockRando> = Region {
            name: MockRegionName::from("Temple"),
            dungeon: Some((Dungeon::Main(MainDungeon::ForestTemple), Mq::Vanilla)),
            scene: None,
            hint: None,
            time_passes: false,
            events: HashSet::new(),
            locations: HashSet::new(),
            exits: HashSet::new(),
        };

        let region2: Region<MockRando> = Region {
            name: MockRegionName::from("Temple"),
            dungeon: Some((Dungeon::Main(MainDungeon::FireTemple), Mq::Vanilla)),
            scene: None,
            hint: None,
            time_passes: false,
            events: HashSet::new(),
            locations: HashSet::new(),
            exits: HashSet::new(),
        };

        // Different dungeons should produce different hashes
        let hash1 = compute_hash(&region1);
        let hash2 = compute_hash(&region2);
        assert_ne!(
            hash1, hash2,
            "Different dungeons should produce different hashes"
        );
    }

    #[test]
    fn test_mq_copy_clone() {
        let mq1 = Mq::Vanilla;
        let mq2 = mq1; // Copy
        let mq3 = mq1.clone(); // Clone

        assert_eq!(mq1, mq2);
        assert_eq!(mq1, mq3);
    }

    #[test]
    fn test_mq_debug() {
        assert_eq!(format!("{:?}", Mq::Vanilla), "Vanilla");
        assert_eq!(format!("{:?}", Mq::Mq), "Mq");
    }

    #[test]
    fn test_region_with_dungeon_context() {
        use crate::model::MainDungeon;
        use mock::{MockRando, MockRegionName};

        // Test region with full dungeon context
        let region: Region<MockRando> = Region {
            name: MockRegionName::from("Deku Tree Lobby"),
            dungeon: Some((Dungeon::Main(MainDungeon::DekuTree), Mq::Vanilla)),
            scene: Some("deku_tree".to_string()),
            hint: Some("Inside the Deku Tree".to_string()),
            time_passes: false,
            events: HashSet::from(["Deku Tree Clear".to_string()]),
            locations: HashSet::from(["Deku Tree Compass Chest".to_string()]),
            exits: HashSet::from([MockRegionName::from("Kokiri Forest")]),
        };

        // Verify all fields are accessible
        assert_eq!(region.name, "Deku Tree Lobby");
        assert_eq!(
            region.dungeon,
            Some((Dungeon::Main(MainDungeon::DekuTree), Mq::Vanilla))
        );
        assert_eq!(region.scene, Some("deku_tree".to_string()));
        assert_eq!(region.hint, Some("Inside the Deku Tree".to_string()));
        assert!(!region.time_passes);
        assert!(region.events.contains("Deku Tree Clear"));
        assert!(region.locations.contains("Deku Tree Compass Chest"));
        assert!(region.exits.contains(&MockRegionName::from("Kokiri Forest")));
    }

    #[test]
    fn test_region_with_mini_dungeon() {
        use mock::{MockRando, MockRegionName};

        let region: Region<MockRando> = Region {
            name: MockRegionName::from("Ice Cavern"),
            dungeon: Some((Dungeon::IceCavern, Mq::Vanilla)),
            scene: None,
            hint: None,
            time_passes: false,
            events: HashSet::new(),
            locations: HashSet::new(),
            exits: HashSet::new(),
        };

        assert_eq!(region.dungeon, Some((Dungeon::IceCavern, Mq::Vanilla)));
    }

    #[test]
    fn test_region_overworld_no_dungeon() {
        use mock::{MockRando, MockRegionName};

        let region: Region<MockRando> = Region {
            name: MockRegionName::from("Hyrule Field"),
            dungeon: None,
            scene: Some("hyrule_field".to_string()),
            hint: None,
            time_passes: true,
            events: HashSet::new(),
            locations: HashSet::new(),
            exits: HashSet::new(),
        };

        assert!(region.dungeon.is_none());
        assert!(region.time_passes);
    }
}
