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
}
