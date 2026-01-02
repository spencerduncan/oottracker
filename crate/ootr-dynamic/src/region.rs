use {
    crate::RandoErr,
    ootr::{model::Dungeon, region::Mq},
    serde::Deserialize,
    std::collections::BTreeMap,
};

#[derive(Deserialize)]
pub(crate) struct RawRegion {
    pub region_name: String,
    #[allow(unused)] // taken from filename
    dungeon: Option<String>,
    pub scene: Option<String>,
    pub hint: Option<String>,
    #[serde(default)]
    pub time_passes: bool,
    #[serde(default)]
    pub events: BTreeMap<String, String>,
    #[serde(default)]
    pub locations: BTreeMap<String, String>,
    #[serde(default)]
    pub exits: BTreeMap<String, String>,
}

pub(crate) fn parse_dungeon_info(mut s: &str) -> Result<Option<(Dungeon, Mq)>, RandoErr> {
    Ok(if let "Overworld" | "Bosses" = s {
        None
    } else {
        let mq = if let Some(prefix) = s.strip_suffix(" MQ") {
            s = prefix;
            Mq::Mq
        } else {
            Mq::Vanilla
        };
        Some((
            s.parse()
                .map_err(|()| RandoErr::UnknownRegionFilename(s.to_owned()))?,
            mq,
        ))
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use ootr::model::MainDungeon;

    #[test]
    fn test_overworld_returns_none() {
        let result = parse_dungeon_info("Overworld").unwrap();
        assert_eq!(result, None);
    }

    #[test]
    fn test_bosses_returns_none() {
        let result = parse_dungeon_info("Bosses").unwrap();
        assert_eq!(result, None);
    }

    #[test]
    fn test_deku_tree_vanilla() {
        let result = parse_dungeon_info("Deku Tree").unwrap();
        assert_eq!(
            result,
            Some((Dungeon::Main(MainDungeon::DekuTree), Mq::Vanilla))
        );
    }

    #[test]
    fn test_deku_tree_mq() {
        let result = parse_dungeon_info("Deku Tree MQ").unwrap();
        assert_eq!(result, Some((Dungeon::Main(MainDungeon::DekuTree), Mq::Mq)));
    }

    #[test]
    fn test_all_main_dungeons_vanilla() {
        let cases = [
            ("Deku Tree", MainDungeon::DekuTree),
            ("Dodongos Cavern", MainDungeon::DodongosCavern),
            ("Jabu Jabus Belly", MainDungeon::JabuJabu),
            ("Forest Temple", MainDungeon::ForestTemple),
            ("Fire Temple", MainDungeon::FireTemple),
            ("Water Temple", MainDungeon::WaterTemple),
            ("Shadow Temple", MainDungeon::ShadowTemple),
            ("Spirit Temple", MainDungeon::SpiritTemple),
        ];

        for (input, expected_dungeon) in cases {
            let result = parse_dungeon_info(input).unwrap();
            assert_eq!(
                result,
                Some((Dungeon::Main(expected_dungeon), Mq::Vanilla)),
                "Failed for input: {}",
                input
            );
        }
    }

    #[test]
    fn test_minor_dungeons_vanilla() {
        let cases = [
            ("Ice Cavern", Dungeon::IceCavern),
            ("Bottom of the Well", Dungeon::BottomOfTheWell),
            ("Gerudo Training Ground", Dungeon::GerudoTrainingGround),
            ("Ganons Castle", Dungeon::GanonsCastle),
        ];

        for (input, expected_dungeon) in cases {
            let result = parse_dungeon_info(input).unwrap();
            assert_eq!(
                result,
                Some((expected_dungeon, Mq::Vanilla)),
                "Failed for input: {}",
                input
            );
        }
    }

    #[test]
    fn test_minor_dungeons_mq() {
        let cases = [
            ("Ice Cavern MQ", Dungeon::IceCavern),
            ("Bottom of the Well MQ", Dungeon::BottomOfTheWell),
            ("Gerudo Training Ground MQ", Dungeon::GerudoTrainingGround),
            ("Ganons Castle MQ", Dungeon::GanonsCastle),
        ];

        for (input, expected_dungeon) in cases {
            let result = parse_dungeon_info(input).unwrap();
            assert_eq!(
                result,
                Some((expected_dungeon, Mq::Mq)),
                "Failed for input: {}",
                input
            );
        }
    }

    #[test]
    fn test_invalid_dungeon_name_returns_error() {
        let result = parse_dungeon_info("Invalid Dungeon");
        assert!(result.is_err());
        match result {
            Err(RandoErr::UnknownRegionFilename(name)) => {
                assert_eq!(name, "Invalid Dungeon");
            }
            _ => panic!("Expected UnknownRegionFilename error"),
        }
    }

    #[test]
    fn test_empty_string_returns_error() {
        let result = parse_dungeon_info("");
        assert!(result.is_err());
        match result {
            Err(RandoErr::UnknownRegionFilename(name)) => {
                assert_eq!(name, "");
            }
            _ => panic!("Expected UnknownRegionFilename error"),
        }
    }

    #[test]
    fn test_case_sensitivity() {
        // Dungeon names are case sensitive
        let result = parse_dungeon_info("deku tree");
        assert!(result.is_err());

        let result = parse_dungeon_info("DEKU TREE");
        assert!(result.is_err());
    }

    #[test]
    fn test_mq_suffix_only_returns_error() {
        // " MQ" alone without dungeon name should fail
        let result = parse_dungeon_info(" MQ");
        assert!(result.is_err());
    }

    #[test]
    fn test_all_main_dungeons_mq() {
        let cases = [
            ("Deku Tree MQ", MainDungeon::DekuTree),
            ("Dodongos Cavern MQ", MainDungeon::DodongosCavern),
            ("Jabu Jabus Belly MQ", MainDungeon::JabuJabu),
            ("Forest Temple MQ", MainDungeon::ForestTemple),
            ("Fire Temple MQ", MainDungeon::FireTemple),
            ("Water Temple MQ", MainDungeon::WaterTemple),
            ("Shadow Temple MQ", MainDungeon::ShadowTemple),
            ("Spirit Temple MQ", MainDungeon::SpiritTemple),
        ];

        for (input, expected_dungeon) in cases {
            let result = parse_dungeon_info(input).unwrap();
            assert_eq!(
                result,
                Some((Dungeon::Main(expected_dungeon), Mq::Mq)),
                "Failed for input: {}",
                input
            );
        }
    }

    #[test]
    fn test_lowercase_mq_returns_error() {
        // "mq" in lowercase should not be recognized
        let result = parse_dungeon_info("Deku Tree mq");
        assert!(result.is_err());
    }

    #[test]
    fn test_mq_without_leading_space_returns_error() {
        // "MQ" without space should not be recognized
        let result = parse_dungeon_info("Deku TreeMQ");
        assert!(result.is_err());
    }

    #[test]
    fn test_overworld_mq_returns_error() {
        // "Overworld MQ" is not special-cased - the Overworld/Bosses check
        // happens before MQ suffix stripping, so this returns an error
        let result = parse_dungeon_info("Overworld MQ");
        assert!(result.is_err());
        match result {
            Err(RandoErr::UnknownRegionFilename(name)) => {
                assert_eq!(name, "Overworld");
            }
            _ => panic!("Expected UnknownRegionFilename error"),
        }
    }

    #[test]
    fn test_bosses_mq_returns_error() {
        // "Bosses MQ" is not special-cased - the Overworld/Bosses check
        // happens before MQ suffix stripping, so this returns an error
        let result = parse_dungeon_info("Bosses MQ");
        assert!(result.is_err());
        match result {
            Err(RandoErr::UnknownRegionFilename(name)) => {
                assert_eq!(name, "Bosses");
            }
            _ => panic!("Expected UnknownRegionFilename error"),
        }
    }

    #[test]
    fn test_double_space_before_mq_returns_error() {
        // Double space before MQ should not match
        let result = parse_dungeon_info("Deku Tree  MQ");
        assert!(result.is_err());
    }

    #[test]
    fn test_just_mq_returns_error() {
        let result = parse_dungeon_info("MQ");
        assert!(result.is_err());
    }

    #[test]
    fn test_similar_but_invalid_names() {
        // Names that are close but not exact matches
        let invalid_names = [
            "Deku",            // Partial name
            "Tree",            // Partial name
            "Forest",          // Partial name
            "Temple",          // Partial name
            "Fire Temple ",    // Trailing space
            " Fire Temple",    // Leading space
            "FireTemple",      // Missing space
            "Jabu Jabu Belly", // Wrong spelling (extra space)
        ];

        for name in invalid_names {
            let result = parse_dungeon_info(name);
            assert!(
                result.is_err(),
                "Expected error for input '{}', got {:?}",
                name,
                result
            );
        }
    }

    #[test]
    fn test_mq_suffix_with_extra_text_returns_error() {
        // Extra text after MQ should make it not match the suffix
        let result = parse_dungeon_info("Deku Tree MQ Extra");
        assert!(result.is_err());
    }

    #[test]
    fn test_dungeon_name_with_apostrophe_variation() {
        // Test that the expected format without apostrophe works
        // "Jabu Jabus Belly" is the expected format (no apostrophe)
        let result = parse_dungeon_info("Jabu Jabus Belly").unwrap();
        assert_eq!(
            result,
            Some((Dungeon::Main(MainDungeon::JabuJabu), Mq::Vanilla))
        );

        let result = parse_dungeon_info("Jabu Jabus Belly MQ").unwrap();
        assert_eq!(result, Some((Dungeon::Main(MainDungeon::JabuJabu), Mq::Mq)));
    }

    #[test]
    fn test_ganons_castle_variations() {
        // Test that Ganons Castle works (no apostrophe)
        let result = parse_dungeon_info("Ganons Castle").unwrap();
        assert_eq!(result, Some((Dungeon::GanonsCastle, Mq::Vanilla)));

        let result = parse_dungeon_info("Ganons Castle MQ").unwrap();
        assert_eq!(result, Some((Dungeon::GanonsCastle, Mq::Mq)));
    }
}
