//! Tests for flag mapping module.

use std::collections::HashMap;

use ootmm::embedded_data;
use ootmm::region::Game;
use ootr::{
    model::{Dungeon, MainDungeon},
    region::Mq,
};

use super::checking::{get_all_checked_locations_combo, get_checked_locations_summary};
use super::mq::{
    active_location_count, dungeon_to_mq_dungeon, get_active_mapping, get_active_mappings,
    get_dungeon_mappings, mq_settings_from_knowledge, MqDungeonType as MqDungeon,
};
use super::queries::{
    get_all_oot_location_ids, get_mapping, get_mappings_by_flag_type, get_mappings_for_scene,
    oot_location_count, oot_mapped_count, oot_stub_count,
};
use super::scenes as scene;
use super::types::{FlagMapping, FlagType};
use crate::ModelState;

#[test]
fn test_flag_type_scene_offset() {
    assert_eq!(FlagType::Chest.scene_offset(), Some(0x00));
    assert_eq!(FlagType::Switch.scene_offset(), Some(0x04));
    assert_eq!(FlagType::RoomClear.scene_offset(), Some(0x08));
    assert_eq!(FlagType::Collectible.scene_offset(), Some(0x0C));
    assert_eq!(FlagType::GoldSkulltula.scene_offset(), None);
    assert_eq!(FlagType::EventChkInf.scene_offset(), None);
}

#[test]
fn test_flag_type_is_scene_based() {
    assert!(FlagType::Chest.is_scene_based());
    assert!(FlagType::Switch.is_scene_based());
    assert!(!FlagType::GoldSkulltula.is_scene_based());
    assert!(!FlagType::EventChkInf.is_scene_based());
}

#[test]
fn test_flag_mapping_stub() {
    let stub = FlagMapping::stub("test_location");
    assert!(stub.is_stub());
    assert!(!stub.is_mapped());
    assert_eq!(stub.location_id, "test_location");
    assert!(stub.scene_id.is_none());
    assert!(stub.flag_type.is_none());
    assert!(stub.flag_bit.is_none());
}

#[test]
fn test_flag_mapping_mapped() {
    let mapped = FlagMapping::mapped("test_chest", scene::DEKU_TREE, FlagType::Chest, 0x01);
    assert!(!mapped.is_stub());
    assert!(mapped.is_mapped());
    assert_eq!(mapped.scene_id, Some(scene::DEKU_TREE));
    assert_eq!(mapped.flag_type, Some(FlagType::Chest));
    assert_eq!(mapped.flag_bit, Some(0x01));
}

#[test]
fn test_flag_mapping_global() {
    let global = FlagMapping::global("test_skulltula", FlagType::GoldSkulltula, 0x04);
    assert!(!global.is_stub());
    assert!(global.is_mapped());
    assert!(global.scene_id.is_none());
    assert_eq!(global.flag_type, Some(FlagType::GoldSkulltula));
}

#[test]
fn test_get_mapping_exists() {
    // This should exist from the world data
    let mappings_count = oot_location_count();
    assert!(
        mappings_count > 0,
        "Should have at least some OoT locations"
    );
}

#[test]
fn test_get_mapping_not_found() {
    let result = get_mapping("nonexistent_location_xyz");
    assert!(result.is_none());
}

#[test]
fn test_location_counts() {
    let total = oot_location_count();
    let mapped = oot_mapped_count();
    let stubs = oot_stub_count();

    assert_eq!(total, mapped + stubs);
    // We should have mostly stubs since only some are mapped
    assert!(stubs > 0, "Should have some stub locations");
}

#[test]
fn test_get_mappings_for_scene() {
    let deku_tree_mappings: Vec<_> = get_mappings_for_scene(scene::DEKU_TREE).collect();
    // We added several Deku Tree chest mappings
    assert!(
        !deku_tree_mappings.is_empty(),
        "Should have Deku Tree mappings"
    );
    for mapping in &deku_tree_mappings {
        assert_eq!(mapping.scene_id, Some(scene::DEKU_TREE));
    }
}

#[test]
fn test_get_mappings_by_flag_type() {
    let chest_mappings: Vec<_> = get_mappings_by_flag_type(FlagType::Chest).collect();
    assert!(!chest_mappings.is_empty(), "Should have chest mappings");
    for mapping in &chest_mappings {
        assert_eq!(mapping.flag_type, Some(FlagType::Chest));
    }
}

#[test]
fn test_scene_constants() {
    assert_eq!(scene::DEKU_TREE, 0x00);
    assert_eq!(scene::DODONGOS_CAVERN, 0x01);
    assert_eq!(scene::KOKIRI_FOREST, 0x55);
    assert_eq!(scene::MAX_SCENE_ID, 0x64);
    assert_eq!(scene::SCENE_COUNT, 101);
}

#[test]
fn test_all_location_ids_loaded() {
    let ids: Vec<_> = get_all_oot_location_ids().collect();
    assert!(!ids.is_empty(), "Should have loaded OoT location IDs");
    // All IDs should start with "oot_" or "mq_oot_" (for Master Quest variants)
    for id in &ids {
        assert!(
            id.starts_with("oot_") || id.starts_with("mq_oot_"),
            "OoT location ID should start with 'oot_' or 'mq_oot_': {}",
            id
        );
    }
}

// === Master Quest Integration Tests ===

#[test]
fn test_mq_active_mappings_default() {
    use ootmm::settings::RandomizerSettings;

    let settings = RandomizerSettings::default();

    // With default settings (no MQ), vanilla locations should be active
    let active: Vec<_> = get_active_mappings(&settings).collect();
    assert!(!active.is_empty());

    // All active locations should be vanilla (not MQ)
    for mapping in &active {
        assert!(
            !mapping.location_id.starts_with("mq_oot_"),
            "Default settings should not include MQ locations: {}",
            mapping.location_id
        );
    }
}

#[test]
fn test_mq_active_mapping_filters() {
    use ootmm::settings::RandomizerSettings;

    let mut settings = RandomizerSettings::default();

    // Vanilla Deku Tree location should be active by default
    assert!(get_active_mapping("oot_deku_tree_compass_chest", &settings).is_some());

    // MQ Deku Tree location should NOT be active by default
    assert!(get_active_mapping("mq_oot_mq_deku_tree_compass_chest", &settings).is_none());

    // Set Deku Tree to MQ
    settings.set_dungeon_mq(MqDungeon::DekuTree);

    // Now vanilla should be inactive and MQ should be active
    assert!(get_active_mapping("oot_deku_tree_compass_chest", &settings).is_none());
    assert!(get_active_mapping("mq_oot_mq_deku_tree_compass_chest", &settings).is_some());
}

#[test]
fn test_mq_dungeon_mappings() {
    use ootmm::settings::RandomizerSettings;

    let mut settings = RandomizerSettings::default();

    // Get vanilla Deku Tree mappings
    let vanilla_mappings: Vec<_> = get_dungeon_mappings(MqDungeon::DekuTree, &settings).collect();
    assert!(!vanilla_mappings.is_empty());
    for mapping in &vanilla_mappings {
        assert!(
            mapping.location_id.starts_with("oot_deku_tree_"),
            "Expected vanilla Deku Tree location: {}",
            mapping.location_id
        );
    }

    // Set to MQ and get MQ mappings
    settings.set_dungeon_mq(MqDungeon::DekuTree);
    let mq_mappings: Vec<_> = get_dungeon_mappings(MqDungeon::DekuTree, &settings).collect();
    assert!(!mq_mappings.is_empty());
    for mapping in &mq_mappings {
        assert!(
            mapping.location_id.starts_with("mq_oot_mq_deku_tree_"),
            "Expected MQ Deku Tree location: {}",
            mapping.location_id
        );
    }
}

#[test]
fn test_active_counts_change_with_mq() {
    use ootmm::settings::RandomizerSettings;

    let mut settings = RandomizerSettings::default();

    let vanilla_count = active_location_count(&settings);
    assert!(vanilla_count > 0);

    // Set all dungeons to MQ
    settings.set_all_dungeons_mq();
    let mq_count = active_location_count(&settings);
    assert!(mq_count > 0);

    // Counts might differ since MQ dungeons have different check counts
    // The important thing is that we get different locations
    let vanilla_settings = RandomizerSettings::default();
    let vanilla_locs: std::collections::HashSet<_> = get_active_mappings(&vanilla_settings)
        .map(|m| m.location_id)
        .collect();
    let mq_locs: std::collections::HashSet<_> = get_active_mappings(&settings)
        .map(|m| m.location_id)
        .collect();

    // The sets should be different (vanilla vs MQ dungeon locations)
    assert_ne!(vanilla_locs, mq_locs);
}

// ========================================================================
// get_all_checked_locations_combo integration tests
// ========================================================================

#[test]
fn test_get_all_checked_locations_combo_oot_only() {
    // Test that without MM save data, only OoT locations are returned
    let model = ModelState::default();

    // Ensure mm_save is None (it should be by default)
    assert!(
        model.ram.mm_save.is_none(),
        "Default model should have no MM save data"
    );

    let locations = get_all_checked_locations_combo(&model);

    // Should have OoT locations
    assert!(!locations.is_empty(), "Should have some locations");

    // All locations should be OoT (start with "oot_" or "mq_oot_" for Master Quest)
    // No MM locations should be present
    for loc in &locations {
        let is_oot = loc.location_id.starts_with("oot_") || loc.location_id.starts_with("mq_oot_");
        assert!(
            is_oot,
            "Without MM save, all locations should be OoT: {}",
            loc.location_id
        );
        assert!(
            !loc.location_id.starts_with("mm_"),
            "Without MM save, no MM locations should be present: {}",
            loc.location_id
        );
    }
}

#[test]
fn test_get_all_checked_locations_combo_includes_mm_when_present() {
    use crate::mm_save::MmSave;

    // Create model with MM save data
    let mut model = ModelState::default();
    model.ram.mm_save = Some(MmSave::default());

    let locations = get_all_checked_locations_combo(&model);

    // Should have locations
    assert!(!locations.is_empty(), "Should have some locations");

    // Should have both OoT and MM locations
    let has_oot = locations.iter().any(|l| l.location_id.starts_with("oot_"));
    let has_mm = locations.iter().any(|l| l.location_id.starts_with("mm_"));

    assert!(has_oot, "Should have OoT locations");
    assert!(has_mm, "Should have MM locations when mm_save is present");
}

#[test]
fn test_get_all_checked_locations_combo_more_locations_with_mm() {
    use crate::mm_save::MmSave;

    // Get OoT-only count
    let model_oot_only = ModelState::default();
    let oot_only_count = get_all_checked_locations_combo(&model_oot_only).len();

    // Get OoT+MM count
    let mut model_with_mm = ModelState::default();
    model_with_mm.ram.mm_save = Some(MmSave::default());
    let combo_count = get_all_checked_locations_combo(&model_with_mm).len();

    // Combo should have more locations than OoT-only
    assert!(
        combo_count > oot_only_count,
        "Combo locations ({}) should be more than OoT-only ({})",
        combo_count,
        oot_only_count
    );
}

#[test]
fn test_get_checked_locations_summary_includes_mm() {
    use crate::mm_save::MmSave;

    // Create model with MM save data
    let mut model = ModelState::default();
    model.ram.mm_save = Some(MmSave::default());

    let summary = get_checked_locations_summary(&model);

    // Summary should include both OoT and MM locations
    let has_oot = summary
        .locations
        .iter()
        .any(|l| l.location_id.starts_with("oot_"));
    let has_mm = summary
        .locations
        .iter()
        .any(|l| l.location_id.starts_with("mm_"));

    assert!(has_oot, "Summary should include OoT locations");
    assert!(
        has_mm,
        "Summary should include MM locations when mm_save is present"
    );

    // Total should match location count
    assert_eq!(
        summary.total_mapped,
        summary.locations.len(),
        "total_mapped should match locations count"
    );

    // Counts should add up
    assert_eq!(
        summary.checked_count + summary.unchecked_count + summary.unknown_count,
        summary.total_mapped,
        "Status counts should sum to total_mapped"
    );
}

// === dungeon_to_mq_dungeon Tests ===

#[test]
fn test_dungeon_to_mq_dungeon_main_dungeons() {
    // Test all main dungeon conversions
    assert_eq!(
        dungeon_to_mq_dungeon(&Dungeon::Main(MainDungeon::DekuTree)),
        MqDungeon::DekuTree
    );
    assert_eq!(
        dungeon_to_mq_dungeon(&Dungeon::Main(MainDungeon::DodongosCavern)),
        MqDungeon::DodongosCavern
    );
    assert_eq!(
        dungeon_to_mq_dungeon(&Dungeon::Main(MainDungeon::JabuJabu)),
        MqDungeon::JabuJabu
    );
    assert_eq!(
        dungeon_to_mq_dungeon(&Dungeon::Main(MainDungeon::ForestTemple)),
        MqDungeon::ForestTemple
    );
    assert_eq!(
        dungeon_to_mq_dungeon(&Dungeon::Main(MainDungeon::FireTemple)),
        MqDungeon::FireTemple
    );
    assert_eq!(
        dungeon_to_mq_dungeon(&Dungeon::Main(MainDungeon::WaterTemple)),
        MqDungeon::WaterTemple
    );
    assert_eq!(
        dungeon_to_mq_dungeon(&Dungeon::Main(MainDungeon::ShadowTemple)),
        MqDungeon::ShadowTemple
    );
    assert_eq!(
        dungeon_to_mq_dungeon(&Dungeon::Main(MainDungeon::SpiritTemple)),
        MqDungeon::SpiritTemple
    );
}

#[test]
fn test_dungeon_to_mq_dungeon_mini_dungeons() {
    // Test mini dungeon conversions
    assert_eq!(
        dungeon_to_mq_dungeon(&Dungeon::IceCavern),
        MqDungeon::IceCavern
    );
    assert_eq!(
        dungeon_to_mq_dungeon(&Dungeon::BottomOfTheWell),
        MqDungeon::BottomOfTheWell
    );
    assert_eq!(
        dungeon_to_mq_dungeon(&Dungeon::GerudoTrainingGround),
        MqDungeon::GerudoTrainingGround
    );
    assert_eq!(
        dungeon_to_mq_dungeon(&Dungeon::GanonsCastle),
        MqDungeon::GanonsCastle
    );
}

#[test]
fn test_dungeon_to_mq_dungeon_exhaustive() {
    // Ensure all Dungeon variants can be converted
    use enum_iterator::all;

    for main_dungeon in all::<MainDungeon>() {
        let dungeon = Dungeon::Main(main_dungeon);
        // Should not panic
        let _ = dungeon_to_mq_dungeon(&dungeon);
    }

    // Test non-main dungeons explicitly
    let _ = dungeon_to_mq_dungeon(&Dungeon::IceCavern);
    let _ = dungeon_to_mq_dungeon(&Dungeon::BottomOfTheWell);
    let _ = dungeon_to_mq_dungeon(&Dungeon::GerudoTrainingGround);
    let _ = dungeon_to_mq_dungeon(&Dungeon::GanonsCastle);
}

// === mq_settings_from_knowledge Tests ===

#[test]
fn test_mq_settings_from_knowledge_empty() {
    // Empty HashMap should result in all-vanilla settings
    let mq_settings: HashMap<Dungeon, Mq> = HashMap::new();
    let settings = mq_settings_from_knowledge(&mq_settings);

    // All dungeons should be vanilla
    assert!(!settings.is_dungeon_mq(MqDungeon::DekuTree));
    assert!(!settings.is_dungeon_mq(MqDungeon::ForestTemple));
    assert!(!settings.is_dungeon_mq(MqDungeon::GanonsCastle));
}

#[test]
fn test_mq_settings_from_knowledge_single_mq() {
    let mut mq_settings: HashMap<Dungeon, Mq> = HashMap::new();
    mq_settings.insert(Dungeon::Main(MainDungeon::ForestTemple), Mq::Mq);

    let settings = mq_settings_from_knowledge(&mq_settings);

    // Only Forest Temple should be MQ
    assert!(settings.is_dungeon_mq(MqDungeon::ForestTemple));
    assert!(!settings.is_dungeon_mq(MqDungeon::DekuTree));
    assert!(!settings.is_dungeon_mq(MqDungeon::FireTemple));
}

#[test]
fn test_mq_settings_from_knowledge_multiple_mq() {
    let mut mq_settings: HashMap<Dungeon, Mq> = HashMap::new();
    mq_settings.insert(Dungeon::Main(MainDungeon::DekuTree), Mq::Mq);
    mq_settings.insert(Dungeon::Main(MainDungeon::ForestTemple), Mq::Mq);
    mq_settings.insert(Dungeon::GanonsCastle, Mq::Mq);
    mq_settings.insert(Dungeon::Main(MainDungeon::WaterTemple), Mq::Vanilla);

    let settings = mq_settings_from_knowledge(&mq_settings);

    // Check MQ dungeons
    assert!(settings.is_dungeon_mq(MqDungeon::DekuTree));
    assert!(settings.is_dungeon_mq(MqDungeon::ForestTemple));
    assert!(settings.is_dungeon_mq(MqDungeon::GanonsCastle));

    // Check vanilla dungeon
    assert!(!settings.is_dungeon_mq(MqDungeon::WaterTemple));

    // Check unspecified dungeons (should be vanilla)
    assert!(!settings.is_dungeon_mq(MqDungeon::FireTemple));
}

#[test]
fn test_mq_settings_from_knowledge_all_vanilla() {
    let mut mq_settings: HashMap<Dungeon, Mq> = HashMap::new();
    // Explicitly set everything to vanilla
    mq_settings.insert(Dungeon::Main(MainDungeon::DekuTree), Mq::Vanilla);
    mq_settings.insert(Dungeon::Main(MainDungeon::ForestTemple), Mq::Vanilla);
    mq_settings.insert(Dungeon::GanonsCastle, Mq::Vanilla);

    let settings = mq_settings_from_knowledge(&mq_settings);

    // All should be vanilla
    assert!(!settings.is_dungeon_mq(MqDungeon::DekuTree));
    assert!(!settings.is_dungeon_mq(MqDungeon::ForestTemple));
    assert!(!settings.is_dungeon_mq(MqDungeon::GanonsCastle));
}

#[test]
fn test_mq_settings_from_knowledge_all_mq() {
    use enum_iterator::all;

    let mut mq_settings: HashMap<Dungeon, Mq> = HashMap::new();

    // Set all main dungeons to MQ
    for main_dungeon in all::<MainDungeon>() {
        mq_settings.insert(Dungeon::Main(main_dungeon), Mq::Mq);
    }
    // Set mini dungeons to MQ
    mq_settings.insert(Dungeon::IceCavern, Mq::Mq);
    mq_settings.insert(Dungeon::BottomOfTheWell, Mq::Mq);
    mq_settings.insert(Dungeon::GerudoTrainingGround, Mq::Mq);
    mq_settings.insert(Dungeon::GanonsCastle, Mq::Mq);

    let settings = mq_settings_from_knowledge(&mq_settings);

    // All should be MQ
    assert!(settings.is_dungeon_mq(MqDungeon::DekuTree));
    assert!(settings.is_dungeon_mq(MqDungeon::DodongosCavern));
    assert!(settings.is_dungeon_mq(MqDungeon::JabuJabu));
    assert!(settings.is_dungeon_mq(MqDungeon::ForestTemple));
    assert!(settings.is_dungeon_mq(MqDungeon::FireTemple));
    assert!(settings.is_dungeon_mq(MqDungeon::WaterTemple));
    assert!(settings.is_dungeon_mq(MqDungeon::ShadowTemple));
    assert!(settings.is_dungeon_mq(MqDungeon::SpiritTemple));
    assert!(settings.is_dungeon_mq(MqDungeon::IceCavern));
    assert!(settings.is_dungeon_mq(MqDungeon::BottomOfTheWell));
    assert!(settings.is_dungeon_mq(MqDungeon::GerudoTrainingGround));
    assert!(settings.is_dungeon_mq(MqDungeon::GanonsCastle));
}

#[test]
fn test_mq_settings_from_knowledge_integration_with_filtering() {
    // Test that mq_settings_from_knowledge integrates properly with location filtering
    let mut mq_settings: HashMap<Dungeon, Mq> = HashMap::new();
    mq_settings.insert(Dungeon::Main(MainDungeon::DekuTree), Mq::Mq);

    let settings = mq_settings_from_knowledge(&mq_settings);

    // Vanilla Deku Tree should be inactive
    assert!(get_active_mapping("oot_deku_tree_compass_chest", &settings).is_none());

    // MQ Deku Tree should be active
    assert!(get_active_mapping("mq_oot_mq_deku_tree_compass_chest", &settings).is_some());

    // Other vanilla dungeons should still be active
    assert!(get_active_mapping("oot_forest_temple_compass", &settings).is_some());
}

// === Location Count Parity Tests ===

/// Test that all OoT locations from WorldDatabase are enumerable in the flag mapping.
///
/// This verifies that the flag_mapping module correctly loads all OoT locations
/// from the embedded world data without losing any. The flag_mapping module may
/// have additional hardcoded entries, but all YAML locations must be present.
#[test]
fn test_oot_location_count_matches_world_database() {
    // Get OoT locations from WorldDatabase
    let db = embedded_data::create_world_database()
        .expect("Failed to create world database from embedded data");

    let world_db_oot_count = db.locations_for_game(Game::Oot).count();

    // Verify the WorldDatabase has the expected OoT location count from YAML
    // OoT has: 31 (bosses) + 744 (dungeons) + 926 (dungeons_mq) + 1397 (overworld) = 3098
    const EXPECTED_YAML_OOT_LOCATIONS: usize = 3098;
    assert_eq!(
        world_db_oot_count, EXPECTED_YAML_OOT_LOCATIONS,
        "WorldDatabase should have {} OoT locations from YAML, got {}",
        EXPECTED_YAML_OOT_LOCATIONS, world_db_oot_count
    );

    // Verify that every location from WorldDatabase exists in the flag mapping
    // (i.e., the pipeline doesn't lose any locations)
    let mut missing_locations = Vec::new();
    for (location, _region_id) in db.locations_for_game(Game::Oot) {
        if get_mapping(&location.id).is_none() {
            missing_locations.push(location.id.clone());
        }
    }

    assert!(
        missing_locations.is_empty(),
        "Flag mapping is missing {} OoT locations from WorldDatabase: {:?}",
        missing_locations.len(),
        missing_locations
    );

    // Verify the flag_mapping count is at least the WorldDatabase count
    let flag_mapping_count = oot_location_count();
    assert!(
        flag_mapping_count >= world_db_oot_count,
        "oot_location_count() ({}) should be >= WorldDatabase OoT locations ({})",
        flag_mapping_count,
        world_db_oot_count
    );
}
