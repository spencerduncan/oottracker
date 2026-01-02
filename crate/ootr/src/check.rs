use {
    crate::{
        model::{Dungeon, Medallion},
        region::Mq,
        Rando,
    },
    derivative::Derivative,
    quote_value::QuoteValue,
    std::fmt,
};

#[derive(Derivative, QuoteValue)]
#[derivative(
    Debug(bound = ""),
    Clone(bound = ""),
    PartialEq(bound = ""),
    Eq(bound = ""),
    Hash(bound = "")
)]
#[quote_value(where(R::RegionName: QuoteValue))]
pub enum Check<R: Rando> {
    /// Constructed using `at` or `here`.
    AnonymousEvent(Box<Check<R>>, usize),
    Event(String),
    /// What's behind an entrance.
    Exit {
        from: R::RegionName,
        from_mq: Option<Mq>,
        to: R::RegionName,
    },
    /// These are the things the randomizer itself considers checks.
    Location(String),
    /// Used as the context for anonymous events in logic helpers.
    LogicHelper(String),
    /// Check whether the given dungeon is MQ or vanilla.
    Mq(Dungeon),
    Setting(String), //TODO include the partitions that can be checked
    TrialActive(Medallion),
    Trick(String),
}

impl<R: Rando> fmt::Display for Check<R> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Check::AnonymousEvent(at_check, id) => write!(f, "requirement {} for {}", id, at_check),
            Check::Event(event) => write!(f, "event: {}", event),
            Check::Exit { from, from_mq, to } => write!(
                f,
                "{} ({}) → {}",
                from,
                from_mq.map_or_else(|| "overworld".to_owned(), |mq| mq.to_string()),
                to
            ),
            Check::Location(loc) => loc.fmt(f),
            Check::LogicHelper(fn_name) => write!(f, "logic helper {:?}", fn_name),
            Check::Mq(dungeon) => write!(f, "is {} MQ or vanilla", dungeon),
            Check::Setting(setting) => write!(f, "setting: {}", setting), //TODO show setting's display name
            Check::TrialActive(med) => write!(f, "{} trial active", med.element()),
            Check::Trick(trick) => write!(f, "trick: {}", trick), //TODO show trick's display name
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::{MainDungeon, Medallion};
    use crate::region::Mq;
    use crate::{RandoErr, Regions};
    use std::{
        collections::{HashMap, HashSet},
        sync::Arc,
    };

    /// Mock error type for testing.
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

    /// Mock Rando implementation for testing Check Display.
    struct MockRando;

    impl Rando for MockRando {
        type Err = MockErr;
        type RegionName = String;

        fn escaped_items(&self) -> Result<Arc<HashMap<String, crate::item::Item>>, Self::Err> {
            Ok(Arc::new(HashMap::new()))
        }

        fn item_table(&self) -> Result<Arc<HashMap<String, crate::item::Item>>, Self::Err> {
            Ok(Arc::new(HashMap::new()))
        }

        fn logic_tricks(&self) -> Result<Arc<HashSet<String>>, Self::Err> {
            Ok(Arc::new(HashSet::new()))
        }

        fn regions(&self) -> Result<Regions<Self>, Self::Err> {
            Ok(Arc::new(Vec::new()))
        }

        fn root() -> Self::RegionName {
            "Root".to_string()
        }

        fn setting_infos(&self) -> Result<Arc<HashSet<String>>, Self::Err> {
            Ok(Arc::new(HashSet::new()))
        }
    }

    #[test]
    fn display_event() {
        let check: Check<MockRando> = Check::Event("Deku Tree Clear".to_string());
        assert_eq!(format!("{}", check), "event: Deku Tree Clear");
    }

    #[test]
    fn display_location() {
        let check: Check<MockRando> = Check::Location("KF Midos Top Left Chest".to_string());
        assert_eq!(format!("{}", check), "KF Midos Top Left Chest");
    }

    #[test]
    fn display_exit_with_vanilla() {
        let check: Check<MockRando> = Check::Exit {
            from: "Kokiri Forest".to_string(),
            from_mq: Some(Mq::Vanilla),
            to: "Lost Woods".to_string(),
        };
        assert_eq!(format!("{}", check), "Kokiri Forest (vanilla) → Lost Woods");
    }

    #[test]
    fn display_exit_with_mq() {
        let check: Check<MockRando> = Check::Exit {
            from: "Deku Tree Lobby".to_string(),
            from_mq: Some(Mq::Mq),
            to: "Deku Tree Basement".to_string(),
        };
        assert_eq!(
            format!("{}", check),
            "Deku Tree Lobby (MQ) → Deku Tree Basement"
        );
    }

    #[test]
    fn display_exit_overworld() {
        let check: Check<MockRando> = Check::Exit {
            from: "Hyrule Field".to_string(),
            from_mq: None,
            to: "Lon Lon Ranch".to_string(),
        };
        assert_eq!(
            format!("{}", check),
            "Hyrule Field (overworld) → Lon Lon Ranch"
        );
    }

    #[test]
    fn display_logic_helper() {
        let check: Check<MockRando> = Check::LogicHelper("can_use_hookshot".to_string());
        assert_eq!(format!("{}", check), "logic helper \"can_use_hookshot\"");
    }

    #[test]
    fn display_mq_dungeon() {
        let check: Check<MockRando> = Check::Mq(Dungeon::Main(MainDungeon::ForestTemple));
        assert_eq!(format!("{}", check), "is Forest Temple MQ or vanilla");
    }

    #[test]
    fn display_setting() {
        let check: Check<MockRando> = Check::Setting("shuffle_ganon_bosskey".to_string());
        assert_eq!(format!("{}", check), "setting: shuffle_ganon_bosskey");
    }

    #[test]
    fn display_trial_active() {
        let check: Check<MockRando> = Check::TrialActive(Medallion::Light);
        assert_eq!(format!("{}", check), "Light trial active");
    }

    #[test]
    fn display_trick() {
        let check: Check<MockRando> = Check::Trick("logic_grottos_without_agony".to_string());
        assert_eq!(format!("{}", check), "trick: logic_grottos_without_agony");
    }

    #[test]
    fn display_anonymous_event() {
        let inner_check = Check::Location("Test Location".to_string());
        let check: Check<MockRando> = Check::AnonymousEvent(Box::new(inner_check), 1);
        assert_eq!(format!("{}", check), "requirement 1 for Test Location");
    }

    #[test]
    fn display_anonymous_event_nested() {
        // Test deeply nested anonymous event
        let inner = Check::Event("Deku Tree Clear".to_string());
        let middle: Check<MockRando> = Check::AnonymousEvent(Box::new(inner), 2);
        let outer: Check<MockRando> = Check::AnonymousEvent(Box::new(middle), 1);
        assert_eq!(
            format!("{}", outer),
            "requirement 1 for requirement 2 for event: Deku Tree Clear"
        );
    }

    #[test]
    fn display_mq_all_dungeons() {
        // Test MQ display for main dungeons
        let main_dungeons = [
            (MainDungeon::DekuTree, "is Deku Tree MQ or vanilla"),
            (MainDungeon::DodongosCavern, "is Dodongo's Cavern MQ or vanilla"),
            (MainDungeon::JabuJabu, "is Jabu-Jabu MQ or vanilla"),
            (MainDungeon::ForestTemple, "is Forest Temple MQ or vanilla"),
            (MainDungeon::FireTemple, "is Fire Temple MQ or vanilla"),
            (MainDungeon::WaterTemple, "is Water Temple MQ or vanilla"),
            (MainDungeon::ShadowTemple, "is Shadow Temple MQ or vanilla"),
            (MainDungeon::SpiritTemple, "is Spirit Temple MQ or vanilla"),
        ];

        for (dungeon, expected) in &main_dungeons {
            let check: Check<MockRando> = Check::Mq(Dungeon::Main(*dungeon));
            assert_eq!(format!("{}", check), *expected);
        }
    }

    #[test]
    fn display_mq_mini_dungeons() {
        let mini_dungeons = [
            (Dungeon::IceCavern, "is Ice Cavern MQ or vanilla"),
            (Dungeon::BottomOfTheWell, "is Bottom of the Well MQ or vanilla"),
            (Dungeon::GerudoTrainingGround, "is Gerudo Training Ground MQ or vanilla"),
            (Dungeon::GanonsCastle, "is Ganon's Castle MQ or vanilla"),
        ];

        for (dungeon, expected) in &mini_dungeons {
            let check: Check<MockRando> = Check::Mq(*dungeon);
            assert_eq!(format!("{}", check), *expected);
        }
    }

    #[test]
    fn display_trial_active_all_medallions() {
        let medallions = [
            (Medallion::Light, "Light trial active"),
            (Medallion::Forest, "Forest trial active"),
            (Medallion::Fire, "Fire trial active"),
            (Medallion::Water, "Water trial active"),
            (Medallion::Shadow, "Shadow trial active"),
            (Medallion::Spirit, "Spirit trial active"),
        ];

        for (medallion, expected) in &medallions {
            let check: Check<MockRando> = Check::TrialActive(*medallion);
            assert_eq!(format!("{}", check), *expected);
        }
    }

    #[test]
    fn check_equality() {
        let check1: Check<MockRando> = Check::Event("test".to_string());
        let check2: Check<MockRando> = Check::Event("test".to_string());
        let check3: Check<MockRando> = Check::Event("different".to_string());

        assert_eq!(check1, check2);
        assert_ne!(check1, check3);
    }

    #[test]
    fn check_hash_consistency() {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};

        fn compute_hash<T: Hash>(value: &T) -> u64 {
            let mut hasher = DefaultHasher::new();
            value.hash(&mut hasher);
            hasher.finish()
        }

        let check1: Check<MockRando> = Check::Event("test".to_string());
        let check2: Check<MockRando> = Check::Event("test".to_string());

        assert_eq!(compute_hash(&check1), compute_hash(&check2));
    }

    #[test]
    fn check_clone() {
        let original: Check<MockRando> = Check::Exit {
            from: "Kokiri Forest".to_string(),
            from_mq: Some(Mq::Vanilla),
            to: "Lost Woods".to_string(),
        };
        let cloned = original.clone();

        assert_eq!(original, cloned);
    }

    #[test]
    fn display_exit_with_empty_region_names() {
        let check: Check<MockRando> = Check::Exit {
            from: "".to_string(),
            from_mq: None,
            to: "".to_string(),
        };
        assert_eq!(format!("{}", check), " (overworld) → ");
    }

    #[test]
    fn display_location_with_special_characters() {
        let check: Check<MockRando> =
            Check::Location("KF Mido's Top Left Chest".to_string());
        assert_eq!(format!("{}", check), "KF Mido's Top Left Chest");
    }

    #[test]
    fn display_event_with_spaces() {
        let check: Check<MockRando> =
            Check::Event("Forest Temple Boss Key Door Opened".to_string());
        assert_eq!(
            format!("{}", check),
            "event: Forest Temple Boss Key Door Opened"
        );
    }

    #[test]
    fn display_setting_with_underscores() {
        let check: Check<MockRando> =
            Check::Setting("shuffle_interior_entrances".to_string());
        assert_eq!(
            format!("{}", check),
            "setting: shuffle_interior_entrances"
        );
    }

    #[test]
    fn display_trick_with_underscores() {
        let check: Check<MockRando> =
            Check::Trick("logic_dc_jump".to_string());
        assert_eq!(format!("{}", check), "trick: logic_dc_jump");
    }

    #[test]
    fn display_logic_helper_with_underscores() {
        let check: Check<MockRando> =
            Check::LogicHelper("can_blast_or_smash".to_string());
        assert_eq!(
            format!("{}", check),
            "logic helper \"can_blast_or_smash\""
        );
    }

    #[test]
    fn check_debug_format() {
        let check: Check<MockRando> = Check::Event("test".to_string());
        let debug_str = format!("{:?}", check);
        assert!(debug_str.contains("Event"));
        assert!(debug_str.contains("test"));
    }
}
