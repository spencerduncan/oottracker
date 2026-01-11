//! Dungeon-related tests.

use crate::settings::*;

// === OotDungeon Tests ===

#[test]
fn test_oot_dungeon_from_str() {
    assert_eq!(OotDungeon::parse("DC"), Some(OotDungeon::DodongosCavern));
    assert_eq!(OotDungeon::parse("BotW"), Some(OotDungeon::BottomOfTheWell));
    assert_eq!(OotDungeon::parse("JJ"), Some(OotDungeon::JabuJabu));
    assert_eq!(OotDungeon::parse("Shadow"), Some(OotDungeon::Shadow));
    assert_eq!(OotDungeon::parse("Water"), Some(OotDungeon::Water));
    assert_eq!(OotDungeon::parse("fireChild"), Some(OotDungeon::FireChild));
    assert_eq!(OotDungeon::parse("wellAdult"), Some(OotDungeon::WellAdult));
    assert_eq!(OotDungeon::parse("invalid"), None);
}

#[test]
fn test_oot_dungeon_as_str_all_variants() {
    assert_eq!(OotDungeon::DodongosCavern.as_str(), "DC");
    assert_eq!(OotDungeon::BottomOfTheWell.as_str(), "BotW");
    assert_eq!(OotDungeon::JabuJabu.as_str(), "JJ");
    assert_eq!(OotDungeon::Shadow.as_str(), "Shadow");
    assert_eq!(OotDungeon::Water.as_str(), "Water");
    assert_eq!(OotDungeon::FireChild.as_str(), "fireChild");
    assert_eq!(OotDungeon::WellAdult.as_str(), "wellAdult");
}

#[test]
fn test_oot_dungeon_roundtrip_all_variants() {
    for dungeon in [
        OotDungeon::DodongosCavern,
        OotDungeon::BottomOfTheWell,
        OotDungeon::JabuJabu,
        OotDungeon::Shadow,
        OotDungeon::Water,
        OotDungeon::FireChild,
        OotDungeon::WellAdult,
    ] {
        let s = dungeon.as_str();
        assert_eq!(OotDungeon::parse(s), Some(dungeon));
    }
}

#[test]
fn test_oot_dungeon_serde_roundtrip() {
    for dungeon in [
        OotDungeon::DodongosCavern,
        OotDungeon::BottomOfTheWell,
        OotDungeon::JabuJabu,
        OotDungeon::Shadow,
        OotDungeon::Water,
        OotDungeon::FireChild,
        OotDungeon::WellAdult,
    ] {
        let json = serde_json::to_string(&dungeon).unwrap();
        let parsed: OotDungeon = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed, dungeon);
    }
}

// === MmDungeon Tests ===

#[test]
fn test_mm_dungeon_from_str() {
    assert_eq!(MmDungeon::parse("ST"), Some(MmDungeon::StoneTower));
    assert_eq!(MmDungeon::parse("WF"), Some(MmDungeon::Woodfall));
    assert_eq!(MmDungeon::parse("invalid"), None);
}

#[test]
fn test_mm_dungeon_as_str_all_variants() {
    assert_eq!(MmDungeon::StoneTower.as_str(), "ST");
    assert_eq!(MmDungeon::Woodfall.as_str(), "WF");
}

#[test]
fn test_mm_dungeon_roundtrip_all_variants() {
    for dungeon in [MmDungeon::StoneTower, MmDungeon::Woodfall] {
        let s = dungeon.as_str();
        assert_eq!(MmDungeon::parse(s), Some(dungeon));
    }
}

#[test]
fn test_mm_dungeon_serde_roundtrip() {
    for dungeon in [MmDungeon::StoneTower, MmDungeon::Woodfall] {
        let json = serde_json::to_string(&dungeon).unwrap();
        let parsed: MmDungeon = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed, dungeon);
    }
}

// === Master Quest Dungeon Tests ===

#[test]
fn test_mq_dungeon_default() {
    let settings = RandomizerSettings::default();
    assert!(settings.mq_dungeons.is_empty());
    assert_eq!(settings.mq_dungeon_count(), 0);
}

#[test]
fn test_mq_dungeon_set_and_check() {
    let mut settings = RandomizerSettings::new();

    assert!(!settings.is_dungeon_mq(MqDungeon::DekuTree));
    settings.set_dungeon_mq(MqDungeon::DekuTree);
    assert!(settings.is_dungeon_mq(MqDungeon::DekuTree));
    assert!(!settings.is_dungeon_mq(MqDungeon::DodongosCavern));

    settings.set_dungeon_vanilla(MqDungeon::DekuTree);
    assert!(!settings.is_dungeon_mq(MqDungeon::DekuTree));
}

#[test]
fn test_mq_dungeon_check_by_name() {
    let mut settings = RandomizerSettings::new();
    settings.set_dungeon_mq(MqDungeon::ForestTemple);

    assert!(settings.is_dungeon_mq_by_name("forest_temple"));
    assert!(settings.is_dungeon_mq_by_name("ForestTemple"));
    assert!(!settings.is_dungeon_mq_by_name("fire_temple"));
    assert!(!settings.is_dungeon_mq_by_name("invalid"));
}

#[test]
fn test_mq_dungeon_set_all() {
    let mut settings = RandomizerSettings::new();

    settings.set_all_dungeons_mq();
    assert_eq!(settings.mq_dungeon_count(), 12);
    assert!(settings.is_dungeon_mq(MqDungeon::DekuTree));
    assert!(settings.is_dungeon_mq(MqDungeon::GanonsCastle));

    settings.set_all_dungeons_vanilla();
    assert_eq!(settings.mq_dungeon_count(), 0);
    assert!(!settings.is_dungeon_mq(MqDungeon::DekuTree));
}

#[test]
fn test_mq_dungeon_location_prefix() {
    let mut settings = RandomizerSettings::new();

    // Vanilla prefix by default
    assert_eq!(
        settings.get_dungeon_location_prefix(MqDungeon::DekuTree),
        "oot_deku_tree_"
    );

    // MQ prefix when set
    settings.set_dungeon_mq(MqDungeon::DekuTree);
    assert_eq!(
        settings.get_dungeon_location_prefix(MqDungeon::DekuTree),
        "mq_oot_mq_deku_tree_"
    );
}

#[test]
fn test_mq_dungeon_location_active() {
    let mut settings = RandomizerSettings::new();

    // By default, vanilla locations are active
    assert!(settings.is_location_active("oot_deku_tree_compass_chest"));
    assert!(!settings.is_location_active("mq_oot_mq_deku_tree_compass_chest"));

    // When dungeon is MQ, MQ locations are active
    settings.set_dungeon_mq(MqDungeon::DekuTree);
    assert!(!settings.is_location_active("oot_deku_tree_compass_chest"));
    assert!(settings.is_location_active("mq_oot_mq_deku_tree_compass_chest"));

    // Non-dungeon locations are always active
    assert!(settings.is_location_active("oot_kokiri_forest_sword"));
}

#[test]
fn test_mq_dungeon_check_setting_value() {
    let mut settings = RandomizerSettings::new();
    settings.set_dungeon_mq(MqDungeon::WaterTemple);
    settings.set_dungeon_mq(MqDungeon::ShadowTemple);

    assert!(settings.check_setting_value("mqDungeons", "water_temple"));
    assert!(settings.check_setting_value("mqDungeons", "shadow_temple"));
    assert!(!settings.check_setting_value("mqDungeons", "fire_temple"));
    assert!(!settings.check_setting_value("mqDungeons", "invalid"));
}

#[test]
fn test_mq_dungeon_parse() {
    assert_eq!(MqDungeon::parse("deku_tree"), Some(MqDungeon::DekuTree));
    assert_eq!(MqDungeon::parse("DekuTree"), Some(MqDungeon::DekuTree));
    assert_eq!(
        MqDungeon::parse("dodongos_cavern"),
        Some(MqDungeon::DodongosCavern)
    );
    assert_eq!(
        MqDungeon::parse("gerudo_training_ground"),
        Some(MqDungeon::GerudoTrainingGround)
    );
    assert_eq!(MqDungeon::parse("invalid"), None);
}

#[test]
fn test_mq_dungeon_from_location_id() {
    // Vanilla locations
    assert_eq!(
        MqDungeon::from_location_id("oot_deku_tree_compass_chest"),
        Some(MqDungeon::DekuTree)
    );
    assert_eq!(
        MqDungeon::from_location_id("oot_fire_temple_boss_key"),
        Some(MqDungeon::FireTemple)
    );
    assert_eq!(
        MqDungeon::from_location_id("oot_ganon_castle_light_trial"),
        Some(MqDungeon::GanonsCastle)
    );

    // MQ locations
    assert_eq!(
        MqDungeon::from_location_id("mq_oot_mq_deku_tree_compass_chest"),
        Some(MqDungeon::DekuTree)
    );
    assert_eq!(
        MqDungeon::from_location_id("mq_oot_dodongo_cavern_entrance"),
        Some(MqDungeon::DodongosCavern)
    );

    // Non-dungeon locations
    assert_eq!(MqDungeon::from_location_id("oot_kokiri_forest_sword"), None);
    assert_eq!(MqDungeon::from_location_id("mm_clock_town_chest"), None);
}

#[test]
fn test_mq_dungeon_serde_roundtrip() {
    let mut settings = RandomizerSettings::new();
    settings.set_dungeon_mq(MqDungeon::ForestTemple);
    settings.set_dungeon_mq(MqDungeon::SpiritTemple);

    let json = serde_json::to_string(&settings).unwrap();
    let parsed: RandomizerSettings = serde_json::from_str(&json).unwrap();

    assert!(parsed.is_dungeon_mq(MqDungeon::ForestTemple));
    assert!(parsed.is_dungeon_mq(MqDungeon::SpiritTemple));
    assert!(!parsed.is_dungeon_mq(MqDungeon::ShadowTemple));
    assert_eq!(parsed.mq_dungeon_count(), 2);
}

#[test]
fn test_mq_dungeon_all() {
    let all = MqDungeon::all();
    assert_eq!(all.len(), 12);
    assert!(all.contains(&MqDungeon::DekuTree));
    assert!(all.contains(&MqDungeon::GanonsCastle));
}

#[test]
fn test_mq_dungeon_as_str_all_variants() {
    assert_eq!(MqDungeon::DekuTree.as_str(), "deku_tree");
    assert_eq!(MqDungeon::DodongosCavern.as_str(), "dodongos_cavern");
    assert_eq!(MqDungeon::JabuJabu.as_str(), "jabu_jabu");
    assert_eq!(MqDungeon::ForestTemple.as_str(), "forest_temple");
    assert_eq!(MqDungeon::FireTemple.as_str(), "fire_temple");
    assert_eq!(MqDungeon::WaterTemple.as_str(), "water_temple");
    assert_eq!(MqDungeon::SpiritTemple.as_str(), "spirit_temple");
    assert_eq!(MqDungeon::ShadowTemple.as_str(), "shadow_temple");
    assert_eq!(MqDungeon::BottomOfTheWell.as_str(), "bottom_of_the_well");
    assert_eq!(MqDungeon::IceCavern.as_str(), "ice_cavern");
    assert_eq!(
        MqDungeon::GerudoTrainingGround.as_str(),
        "gerudo_training_ground"
    );
    assert_eq!(MqDungeon::GanonsCastle.as_str(), "ganons_castle");
}

#[test]
fn test_mq_dungeon_serde_roundtrip_all() {
    for dungeon in MqDungeon::all() {
        let json = serde_json::to_string(dungeon).unwrap();
        let parsed: MqDungeon = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed, *dungeon);
    }
}
