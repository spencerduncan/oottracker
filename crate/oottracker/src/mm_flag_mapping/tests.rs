//! Tests for MM flag mapping module.

use ootmm::events::mm_flags::owl_bits;
use ootmm::region::Game;

use crate::flag_mapping::CheckStatus;
use crate::mm_save::MmSave;

use super::checker::check_mm_location_status;
use super::queries::{
    get_all_mm_location_ids, get_mm_mapping, get_mm_mappings_by_flag_type,
    get_mm_mappings_for_scene, mm_location_count, mm_mapped_count, mm_stub_count,
};
use super::scenes as mm_scene;
use super::{MmFlagMapping, MmFlagType};

#[test]
fn test_mm_flag_type_scene_offset() {
    assert_eq!(MmFlagType::Chest.scene_offset(), Some(0x00));
    assert_eq!(MmFlagType::Switch0.scene_offset(), Some(0x04));
    assert_eq!(MmFlagType::Switch1.scene_offset(), Some(0x08));
    assert_eq!(MmFlagType::ClearedRoom.scene_offset(), Some(0x0C));
    assert_eq!(MmFlagType::Collectible.scene_offset(), Some(0x10));

    // Global flags should return None
    assert_eq!(MmFlagType::GoldSkulltula.scene_offset(), None);
    assert_eq!(MmFlagType::EventInf.scene_offset(), None);
    assert_eq!(MmFlagType::WeekEventReg.scene_offset(), None);
    assert_eq!(MmFlagType::StrayFairy.scene_offset(), None);
}

#[test]
fn test_mm_flag_type_is_scene_based() {
    assert!(MmFlagType::Chest.is_scene_based());
    assert!(MmFlagType::Switch0.is_scene_based());
    assert!(MmFlagType::Switch1.is_scene_based());
    assert!(MmFlagType::ClearedRoom.is_scene_based());
    assert!(MmFlagType::Collectible.is_scene_based());

    assert!(!MmFlagType::GoldSkulltula.is_scene_based());
    assert!(!MmFlagType::EventInf.is_scene_based());
    assert!(!MmFlagType::Boss.is_scene_based());
}

#[test]
fn test_mm_flag_mapping_stub() {
    let stub = MmFlagMapping::stub("test_location");
    assert!(stub.is_stub());
    assert!(!stub.is_mapped());
    assert_eq!(stub.location_id, "test_location");
    assert_eq!(stub.scene_id, None);
    assert_eq!(stub.flag_type, None);
    assert_eq!(stub.flag_bit, None);
}

#[test]
fn test_mm_flag_mapping_mapped() {
    let mapped = MmFlagMapping::mapped(
        "test_location",
        mm_scene::WOODFALL_TEMPLE,
        MmFlagType::Chest,
        0x01,
    );
    assert!(!mapped.is_stub());
    assert!(mapped.is_mapped());
    assert_eq!(mapped.location_id, "test_location");
    assert_eq!(mapped.scene_id, Some(mm_scene::WOODFALL_TEMPLE));
    assert_eq!(mapped.flag_type, Some(MmFlagType::Chest));
    assert_eq!(mapped.flag_bit, Some(0x01));
}

#[test]
fn test_mm_flag_mapping_global() {
    let global = MmFlagMapping::global("test_global", MmFlagType::EventInf, 0x10);
    assert!(!global.is_stub());
    assert!(global.is_mapped());
    assert_eq!(global.scene_id, None);
    assert_eq!(global.flag_type, Some(MmFlagType::EventInf));
}

#[test]
fn test_mm_mappings_loaded() {
    // Verify that MM_MAPPINGS is populated from world data
    let count = mm_location_count();
    assert!(
        count > 0,
        "MM_MAPPINGS should have locations from world data"
    );

    // Verify stub + mapped counts match total
    let stub_count = mm_stub_count();
    let mapped_count = mm_mapped_count();

    assert_eq!(
        stub_count + mapped_count,
        count,
        "stub + mapped should equal total"
    );

    // Verify we have some mapped chest and collectible locations
    assert!(
        mapped_count > 100,
        "Should have over 100 mapped locations (chests + heart pieces)"
    );
}

#[test]
fn test_mm_location_ids_loaded() {
    // Verify that MM_LOCATION_IDS is populated
    let ids: Vec<_> = get_all_mm_location_ids().collect();
    assert!(!ids.is_empty(), "Should have MM location IDs");

    // All IDs should be unique
    let mut unique_ids = ids.clone();
    unique_ids.sort();
    unique_ids.dedup();
    assert_eq!(
        ids.len(),
        unique_ids.len(),
        "All location IDs should be unique"
    );
}

#[test]
fn test_get_mm_mapping() {
    // Try to get a mapping - we know at least some MM locations exist
    let ids: Vec<_> = get_all_mm_location_ids().collect();
    if !ids.is_empty() {
        let first_id = ids[0];
        let mapping = get_mm_mapping(first_id);
        assert!(
            mapping.is_some(),
            "Should find mapping for known location ID"
        );
        assert_eq!(mapping.unwrap().location_id, first_id);
    }

    // Non-existent location should return None
    assert!(get_mm_mapping("non_existent_location_xyz").is_none());
}

#[test]
fn test_scene_constants() {
    // Verify scene constants are reasonable
    // Note: Using runtime check to avoid clippy's const evaluation warnings
    let scenes: [u8; 5] = [
        mm_scene::WOODFALL_TEMPLE,
        mm_scene::SNOWHEAD_TEMPLE,
        mm_scene::GREAT_BAY_TEMPLE,
        mm_scene::STONE_TOWER_TEMPLE,
        mm_scene::CLOCK_TOWN_SOUTH,
    ];
    for scene in scenes {
        assert!(scene < mm_scene::MAX_SCENE_ID);
    }
}

#[test]
fn test_get_mm_mappings_by_flag_type() {
    // Get all chest mappings
    let chest_mappings: Vec<_> = get_mm_mappings_by_flag_type(MmFlagType::Chest).collect();
    // All returned mappings should be properly mapped
    assert!(!chest_mappings.is_empty(), "Should have chest mappings");
    assert!(
        chest_mappings.iter().all(|m| m.is_mapped()),
        "All returned mappings should be mapped"
    );
}

#[test]
fn test_get_mm_mappings_for_scene() {
    // Get Woodfall Temple mappings
    let woodfall_mappings: Vec<_> = get_mm_mappings_for_scene(mm_scene::WOODFALL_TEMPLE).collect();
    // Should have Woodfall Temple chest mappings
    assert!(
        !woodfall_mappings.is_empty(),
        "Should have Woodfall Temple mappings"
    );
    for mapping in &woodfall_mappings {
        assert_eq!(mapping.scene_id, Some(mm_scene::WOODFALL_TEMPLE));
    }
}

#[test]
fn test_owl_statue_mappings() {
    // All 10 owl statues should be mapped
    let owl_mappings: Vec<_> = get_mm_mappings_by_flag_type(MmFlagType::OwlStatue).collect();
    assert_eq!(owl_mappings.len(), 10, "Should have 10 owl statue mappings");

    // Verify all owl statues are properly mapped
    let owl_ids = [
        "mm_clock_town_owl_statue",
        "mm_milk_road_owl_statue",
        "mm_southern_swamp_owl_statue",
        "mm_woodfall_owl_statue",
        "mm_mountain_village_owl_statue",
        "mm_snowhead_owl_statue",
        "mm_zora_cape_owl_statue",
        "mm_great_bay_coast_owl_statue",
        "mm_ikana_canyon_owl_statue",
        "mm_stone_tower_owl_statue",
    ];

    for owl_id in owl_ids {
        let mapping = get_mm_mapping(owl_id);
        assert!(mapping.is_some(), "Should find mapping for {}", owl_id);

        let m = mapping.unwrap();
        assert!(m.is_mapped(), "{} should be mapped", owl_id);
        assert_eq!(
            m.flag_type,
            Some(MmFlagType::OwlStatue),
            "{} should have OwlStatue flag type",
            owl_id
        );
        assert!(
            m.scene_id.is_none(),
            "{} should be a global flag (no scene_id)",
            owl_id
        );
    }
}

#[test]
fn test_owl_statue_flag_bits() {
    // Verify each owl statue has a unique, correct bit mask
    let expected_bits = [
        ("mm_clock_town_owl_statue", 1 << owl_bits::OWL_CLOCK_TOWN),
        ("mm_milk_road_owl_statue", 1 << owl_bits::OWL_MILK_ROAD),
        (
            "mm_southern_swamp_owl_statue",
            1 << owl_bits::OWL_SOUTHERN_SWAMP,
        ),
        ("mm_woodfall_owl_statue", 1 << owl_bits::OWL_WOODFALL),
        (
            "mm_mountain_village_owl_statue",
            1 << owl_bits::OWL_MOUNTAIN_VILLAGE,
        ),
        ("mm_snowhead_owl_statue", 1 << owl_bits::OWL_SNOWHEAD),
        ("mm_zora_cape_owl_statue", 1 << owl_bits::OWL_ZORA_CAPE),
        (
            "mm_great_bay_coast_owl_statue",
            1 << owl_bits::OWL_GREAT_BAY,
        ),
        (
            "mm_ikana_canyon_owl_statue",
            1 << owl_bits::OWL_IKANA_CANYON,
        ),
        ("mm_stone_tower_owl_statue", 1 << owl_bits::OWL_STONE_TOWER),
    ];

    for (loc_id, expected_bit) in expected_bits {
        let mapping = get_mm_mapping(loc_id).expect("Mapping should exist");
        assert_eq!(
            mapping.flag_bit,
            Some(expected_bit),
            "{} should have flag_bit {}",
            loc_id,
            expected_bit
        );
    }
}

#[test]
fn test_shop_mappings() {
    // Verify shop mappings are populated
    let shop_mappings: Vec<_> = get_mm_mappings_by_flag_type(MmFlagType::Shop).collect();

    // We should have 19 shop mappings:
    // - 2 bomb shop items
    // - 8 trading post items
    // - 3 goron shop items
    // - 3 zora shop items
    // - 2 milk bar purchases
    // - 1 gorman track milk purchase
    assert_eq!(shop_mappings.len(), 19, "Should have 19 shop mappings");

    // Verify all shop mappings are properly mapped (not stubs)
    for mapping in &shop_mappings {
        assert!(mapping.is_mapped(), "Shop mapping should not be a stub");
        assert_eq!(
            mapping.flag_type,
            Some(MmFlagType::Shop),
            "Should be Shop flag type"
        );
    }

    // Verify specific shop items exist
    assert!(get_mm_mapping("mm_bomb_shop_item_1").is_some());
    assert!(get_mm_mapping("mm_trading_post_item_1").is_some());
    assert!(get_mm_mapping("mm_goron_shop_item_1").is_some());
    assert!(get_mm_mapping("mm_zora_shop_item_1").is_some());
    assert!(get_mm_mapping("mm_milk_bar_purchase_milk").is_some());
    assert!(get_mm_mapping("mm_gorman_track_milk_purchase").is_some());
}

#[test]
fn test_scrub_mappings() {
    // Verify scrub mappings are populated
    let scrub_mappings: Vec<_> = get_mm_mappings_by_flag_type(MmFlagType::Scrub).collect();

    // We should have 10 scrub mappings:
    // - 1 Clock Town business scrub
    // - 2 Southern Swamp scrubs
    // - 2 Goron Village scrubs
    // - 2 Zora Hall scrubs
    // - 1 Ikana Valley scrub
    // - 2 Termina Field scrub grotto
    assert_eq!(scrub_mappings.len(), 10, "Should have 10 scrub mappings");

    // Verify all scrub mappings are properly mapped (not stubs)
    for mapping in &scrub_mappings {
        assert!(mapping.is_mapped(), "Scrub mapping should not be a stub");
        assert_eq!(
            mapping.flag_type,
            Some(MmFlagType::Scrub),
            "Should be Scrub flag type"
        );
    }

    // Verify specific scrub locations exist
    assert!(get_mm_mapping("mm_clock_town_business_scrub").is_some());
    assert!(get_mm_mapping("mm_southern_swamp_scrub_deed").is_some());
    assert!(get_mm_mapping("mm_goron_village_scrub_deed").is_some());
    assert!(get_mm_mapping("mm_zora_hall_scrub_deed").is_some());
    assert!(get_mm_mapping("mm_ikana_valley_scrub_shop").is_some());
    assert!(get_mm_mapping("mm_termina_field_scrub").is_some());
}

#[test]
fn test_wonder_item_mappings() {
    // Verify wonder item mappings (using Collectible flag type)
    let collectible_mappings: Vec<_> =
        get_mm_mappings_by_flag_type(MmFlagType::Collectible).collect();

    // We should have many wonder items and soil items mapped as collectibles
    // Clock Town South: 3, Clock Town East: 9, Ikana Graveyard: 12,
    // Romani Ranch Fence: 6, Romani Ranch Barn: 2, Cucco Shack: 6,
    // Termina Field: 15, Deku Palace Soil: 3, Beans Grotto Soil: 3,
    // Romani Ranch Soil: 3, Termina Field Soil: 6
    // Total: 68 collectible mappings
    assert!(
        collectible_mappings.len() >= 68,
        "Should have at least 68 collectible (wonder/soil) mappings, got {}",
        collectible_mappings.len()
    );

    // Verify specific wonder items exist
    assert!(get_mm_mapping("mm_clock_town_south_wonder_item_1").is_some());
    assert!(get_mm_mapping("mm_clock_town_east_wonder_item_target_left_1").is_some());
    assert!(get_mm_mapping("mm_ikana_graveyard_wonder_item_01").is_some());
    assert!(get_mm_mapping("mm_romani_ranch_wonder_item_fence_1").is_some());
    assert!(get_mm_mapping("mm_cucco_shack_wonder_item_1").is_some());
    assert!(get_mm_mapping("mm_termina_field_wonder_item_hollow_trunk").is_some());

    // Verify specific soil items exist
    assert!(get_mm_mapping("mm_deku_palace_soil_item_1").is_some());
    assert!(get_mm_mapping("mm_beans_grotto_soil_item_1").is_some());
    assert!(get_mm_mapping("mm_romani_ranch_soil_days_2_3_item_1").is_some());
    assert!(get_mm_mapping("mm_termina_field_soil_observatory_item_1").is_some());
}

#[test]
fn test_total_shop_scrub_mappings() {
    // Verify total number of mapped (non-stub) locations
    let mapped_count = mm_mapped_count();

    // We added:
    // - 19 shop mappings
    // - 10 scrub mappings
    // - 68 wonder/soil item mappings (collectibles)
    // Total: 97 mappings
    assert!(
        mapped_count >= 97,
        "Should have at least 97 mapped locations, got {}",
        mapped_count
    );
}

#[test]
fn test_stray_fairy_great_fairy_mappings() {
    // Test that Great Fairy reward locations are mapped with StrayFairy type
    let great_fairy_locations = [
        ("mm_clock_town_great_fairy", 0),
        ("mm_clock_town_great_fairy_alt", 0),
        ("mm_woodfall_great_fairy", 1),
        ("mm_snowhead_great_fairy", 2),
        ("mm_great_bay_great_fairy", 3),
        ("mm_ikana_great_fairy", 4),
    ];

    for (location_id, expected_bit) in great_fairy_locations {
        let mapping = get_mm_mapping(location_id);
        assert!(
            mapping.is_some(),
            "Great Fairy location {} should exist",
            location_id
        );
        let mapping = mapping.unwrap();
        assert!(
            mapping.is_mapped(),
            "Great Fairy location {} should be mapped",
            location_id
        );
        assert_eq!(
            mapping.flag_type,
            Some(MmFlagType::StrayFairy),
            "Great Fairy location {} should use StrayFairy flag type",
            location_id
        );
        assert_eq!(
            mapping.flag_bit,
            Some(expected_bit),
            "Great Fairy location {} should have flag_bit {}",
            location_id,
            expected_bit
        );
        // Great Fairy rewards are global (no scene_id)
        assert_eq!(
            mapping.scene_id, None,
            "Great Fairy location {} should be global (no scene_id)",
            location_id
        );
    }
}

#[test]
fn test_stray_fairy_collectible_mappings() {
    // Test Clock Town stray fairy
    let clock_town_fairy = get_mm_mapping("mm_clock_town_stray_fairy");
    assert!(
        clock_town_fairy.is_some(),
        "Clock Town stray fairy should exist"
    );
    let mapping = clock_town_fairy.unwrap();
    assert!(
        mapping.is_mapped(),
        "Clock Town stray fairy should be mapped"
    );
    assert_eq!(
        mapping.flag_type,
        Some(MmFlagType::Collectible),
        "Clock Town stray fairy should use Collectible flag type"
    );
    assert_eq!(
        mapping.scene_id,
        Some(mm_scene::LAUNDRY_POOL),
        "Clock Town stray fairy should be in Laundry Pool scene"
    );
}

#[test]
fn test_beneath_the_well_fairy_mappings() {
    // Test all 8 Beneath the Well fairy fountain fairies
    for i in 1..=8 {
        let location_id = format!("mm_beneath_the_well_fairy_fountain_fairy_{}", i);
        let mapping = get_mm_mapping(&location_id);
        assert!(
            mapping.is_some(),
            "Beneath the Well fairy {} should exist",
            i
        );
        let mapping = mapping.unwrap();
        assert!(
            mapping.is_mapped(),
            "Beneath the Well fairy {} should be mapped",
            i
        );
        assert_eq!(
            mapping.flag_type,
            Some(MmFlagType::Collectible),
            "Beneath the Well fairy {} should use Collectible flag type",
            i
        );
        assert_eq!(
            mapping.scene_id,
            Some(mm_scene::BENEATH_THE_WELL),
            "Beneath the Well fairy {} should be in Beneath the Well scene",
            i
        );
        // Each fairy should have a unique bit (power of 2)
        let expected_bit = 1u32 << (i - 1);
        assert_eq!(
            mapping.flag_bit,
            Some(expected_bit),
            "Beneath the Well fairy {} should have flag_bit 0x{:02X}",
            i,
            expected_bit
        );
    }
}

#[test]
fn test_stray_fairy_mappings_by_flag_type() {
    // Verify we have StrayFairy mappings now
    let stray_fairy_mappings: Vec<_> =
        get_mm_mappings_by_flag_type(MmFlagType::StrayFairy).collect();
    assert_eq!(
        stray_fairy_mappings.len(),
        6,
        "Should have 6 StrayFairy mappings (Great Fairy rewards)"
    );

    // Verify all are properly mapped
    for mapping in &stray_fairy_mappings {
        assert!(
            mapping.is_mapped(),
            "All StrayFairy mappings should be complete"
        );
        assert_eq!(
            mapping.scene_id, None,
            "StrayFairy mappings should be global"
        );
    }
}

#[test]
fn test_fairy_mappings_count() {
    // Total fairy mappings: 6 Great Fairy rewards + 1 Clock Town stray + 8 Beneath Well = 15
    let mapped_count = mm_mapped_count();
    assert!(
        mapped_count >= 15,
        "Should have at least 15 mapped locations (stray fairy fountains)"
    );
}

// ========================================================================
// check_mm_location_status tests
// ========================================================================

#[test]
fn test_check_mm_location_status_stub_returns_unknown() {
    // Create a stub mapping (unmapped location)
    let stub = MmFlagMapping::stub("test_unmapped_location");

    // Create a default MmSave
    let mm_save = MmSave::default();

    // Stub mappings should return Unknown
    let status = check_mm_location_status(&stub, &mm_save);
    assert_eq!(
        status,
        CheckStatus::Unknown,
        "Stub mapping should return Unknown status"
    );
}

#[test]
fn test_check_mm_location_status_chest_checked() {
    use crate::mm_save::MmPermanentSceneFlags;

    // Create a mapped chest location for Woodfall Temple
    let mapping = MmFlagMapping::mapped(
        "test_chest_location",
        mm_scene::WOODFALL_TEMPLE,
        MmFlagType::Chest,
        0x01, // bit 0
    );

    // Create MmSave with the chest flag set
    let mut mm_save = MmSave::default();
    // Ensure we have enough scene flag entries
    while mm_save.permanent_scene_flags.len() <= mm_scene::WOODFALL_TEMPLE as usize {
        mm_save
            .permanent_scene_flags
            .push(MmPermanentSceneFlags::default());
    }
    // Set the chest flag bit
    mm_save.permanent_scene_flags[mm_scene::WOODFALL_TEMPLE as usize].chest = 0x01;

    // Status should be Checked
    let status = check_mm_location_status(&mapping, &mm_save);
    assert_eq!(
        status,
        CheckStatus::Checked,
        "Chest with flag set should return Checked status"
    );
}

#[test]
fn test_check_mm_location_status_chest_unchecked() {
    use crate::mm_save::MmPermanentSceneFlags;

    // Create a mapped chest location for Woodfall Temple
    let mapping = MmFlagMapping::mapped(
        "test_chest_location",
        mm_scene::WOODFALL_TEMPLE,
        MmFlagType::Chest,
        0x01, // bit 0
    );

    // Create MmSave with the chest flag NOT set
    let mut mm_save = MmSave::default();
    // Ensure we have enough scene flag entries
    while mm_save.permanent_scene_flags.len() <= mm_scene::WOODFALL_TEMPLE as usize {
        mm_save
            .permanent_scene_flags
            .push(MmPermanentSceneFlags::default());
    }
    // Chest flags are 0 (not set)
    mm_save.permanent_scene_flags[mm_scene::WOODFALL_TEMPLE as usize].chest = 0x00;

    // Status should be Unchecked
    let status = check_mm_location_status(&mapping, &mm_save);
    assert_eq!(
        status,
        CheckStatus::Unchecked,
        "Chest with flag not set should return Unchecked status"
    );
}

#[test]
fn test_check_mm_location_status_out_of_bounds_scene_returns_unknown() {
    // Create a mapped location with an out-of-bounds scene ID
    let mapping = MmFlagMapping::mapped(
        "test_oob_location",
        255, // Very high scene ID that won't exist
        MmFlagType::Chest,
        0x01,
    );

    // Create MmSave with default (small) permanent_scene_flags
    let mm_save = MmSave::default();

    // Out of bounds scene should return Unknown
    let status = check_mm_location_status(&mapping, &mm_save);
    assert_eq!(
        status,
        CheckStatus::Unknown,
        "Out of bounds scene_id should return Unknown status"
    );
}

#[test]
fn test_check_mm_location_status_collectible_checked() {
    use crate::mm_save::MmPermanentSceneFlags;

    // Test collectible flag type as well
    let mapping = MmFlagMapping::mapped(
        "test_collectible_location",
        mm_scene::TERMINA_FIELD,
        MmFlagType::Collectible,
        0x04, // bit 2
    );

    // Create MmSave with the collectible flag set
    let mut mm_save = MmSave::default();
    while mm_save.permanent_scene_flags.len() <= mm_scene::TERMINA_FIELD as usize {
        mm_save
            .permanent_scene_flags
            .push(MmPermanentSceneFlags::default());
    }
    mm_save.permanent_scene_flags[mm_scene::TERMINA_FIELD as usize].collectible = 0x04;

    let status = check_mm_location_status(&mapping, &mm_save);
    assert_eq!(
        status,
        CheckStatus::Checked,
        "Collectible with flag set should return Checked status"
    );
}

#[test]
fn test_check_mm_location_status_switch0_checked() {
    use crate::mm_save::MmPermanentSceneFlags;

    // Test switch0 flag type
    let mapping = MmFlagMapping::mapped(
        "test_switch0_location",
        mm_scene::SNOWHEAD_TEMPLE,
        MmFlagType::Switch0,
        0x08, // bit 3
    );

    let mut mm_save = MmSave::default();
    while mm_save.permanent_scene_flags.len() <= mm_scene::SNOWHEAD_TEMPLE as usize {
        mm_save
            .permanent_scene_flags
            .push(MmPermanentSceneFlags::default());
    }
    mm_save.permanent_scene_flags[mm_scene::SNOWHEAD_TEMPLE as usize].switch0 = 0x08;

    let status = check_mm_location_status(&mapping, &mm_save);
    assert_eq!(
        status,
        CheckStatus::Checked,
        "Switch0 with flag set should return Checked status"
    );
}

#[test]
fn test_check_mm_location_status_event_inf() {
    // Test EventInf flag checking
    let mapping = MmFlagMapping::global("test_event_location", MmFlagType::EventInf, 0x01);

    let mut mm_save = MmSave::default();

    // Test unchecked state
    let status = check_mm_location_status(&mapping, &mm_save);
    assert_eq!(
        status,
        CheckStatus::Unchecked,
        "EventInf with flag not set should return Unchecked"
    );

    // Test checked state - set bit 0 in word 0
    mm_save.event_inf[0] = 0x01;
    let status = check_mm_location_status(&mapping, &mm_save);
    assert_eq!(
        status,
        CheckStatus::Checked,
        "EventInf with flag set should return Checked"
    );

    // Test EventInf with different word index (word 1, mask 0x0004)
    let mapping_word1 = MmFlagMapping::global(
        "test_event_word1",
        MmFlagType::EventInf,
        (1 << 16) | 0x0004, // word_index=1, mask=0x0004
    );
    mm_save.event_inf[1] = 0x0004;
    let status = check_mm_location_status(&mapping_word1, &mm_save);
    assert_eq!(
        status,
        CheckStatus::Checked,
        "EventInf with word_index=1 flag set should return Checked"
    );
}

#[test]
fn test_check_mm_location_status_week_event_reg() {
    // Test WeekEventReg flag checking
    let mapping = MmFlagMapping::global("test_week_event", MmFlagType::WeekEventReg, 0x01);

    let mut mm_save = MmSave {
        week_event_reg: vec![0u8; 100],
        ..Default::default()
    };

    // Test unchecked state
    let status = check_mm_location_status(&mapping, &mm_save);
    assert_eq!(
        status,
        CheckStatus::Unchecked,
        "WeekEventReg with flag not set should return Unchecked"
    );

    // Test checked state - set bit 0 in byte 0
    mm_save.week_event_reg[0] = 0x01;
    let status = check_mm_location_status(&mapping, &mm_save);
    assert_eq!(
        status,
        CheckStatus::Checked,
        "WeekEventReg with flag set should return Checked"
    );

    // Test WeekEventReg with different byte index (byte 25, mask 0x02 - like boss flags)
    let mapping_byte25 = MmFlagMapping::global(
        "test_boss_flag",
        MmFlagType::WeekEventReg,
        (25 << 8) | 0x02, // byte_index=25, mask=0x02
    );
    mm_save.week_event_reg[25] = 0x02;
    let status = check_mm_location_status(&mapping_byte25, &mm_save);
    assert_eq!(
        status,
        CheckStatus::Checked,
        "WeekEventReg with byte_index=25 flag set should return Checked"
    );
}

// === Xflag Tests ===

#[test]
fn test_xflag_type_scene_offset() {
    // Xflag should be a global flag type (not scene-based)
    assert_eq!(MmFlagType::Xflag.scene_offset(), None);
    assert!(!MmFlagType::Xflag.is_scene_based());
}

#[test]
fn test_check_xflag_status() {
    use crate::xflags::XFLAGS_BYTES_MM;

    // Create a mapping for an xflag location
    let mapping = MmFlagMapping::global("test_xflag_location", MmFlagType::Xflag, 42);

    // Test with no xflags data (None) - should return Unknown
    let mut mm_save = MmSave {
        week_event_reg: vec![0u8; 100],
        ..Default::default()
    };
    let status = check_mm_location_status(&mapping, &mm_save);
    assert_eq!(
        status,
        CheckStatus::Unknown,
        "Xflag with no xflags data should return Unknown"
    );

    // Set up xflags data with bit 42 NOT set
    let mut xflags = [0u8; XFLAGS_BYTES_MM];
    mm_save.xflags = Some(xflags);
    let status = check_mm_location_status(&mapping, &mm_save);
    assert_eq!(
        status,
        CheckStatus::Unchecked,
        "Xflag bit 42 not set should return Unchecked"
    );

    // Set bit 42 (byte 5, bit 2)
    xflags[42 / 8] |= 1 << (42 % 8);
    mm_save.xflags = Some(xflags);
    let status = check_mm_location_status(&mapping, &mm_save);
    assert_eq!(
        status,
        CheckStatus::Checked,
        "Xflag bit 42 set should return Checked"
    );
}

#[test]
fn test_check_xflag_various_bits() {
    use crate::xflags::XFLAGS_BYTES_MM;

    // Test various bit positions to ensure the bit manipulation is correct
    let test_bits = [0, 7, 8, 15, 100, 500, 841]; // Edge cases and various positions

    for &bit in &test_bits {
        let mapping = MmFlagMapping::global("test_xflag", MmFlagType::Xflag, bit);
        let mut mm_save = MmSave {
            week_event_reg: vec![0u8; 100],
            ..Default::default()
        };

        // Set the bit
        let mut xflags = [0u8; XFLAGS_BYTES_MM];
        xflags[bit as usize / 8] |= 1 << (bit % 8);
        mm_save.xflags = Some(xflags);

        let status = check_mm_location_status(&mapping, &mm_save);
        assert_eq!(
            status,
            CheckStatus::Checked,
            "Xflag bit {} should be Checked when set",
            bit
        );
    }
}

// === Location Count Parity Tests ===

/// Test that all MM locations from WorldDatabase are enumerable in the flag mapping.
///
/// This verifies that the mm_flag_mapping module correctly loads all MM locations
/// from the embedded world data without losing any. The mm_flag_mapping module may
/// have additional hardcoded entries, but all YAML locations must be present.
#[test]
fn test_mm_location_count_matches_world_database() {
    use ootmm::embedded_data;

    // Get MM locations from WorldDatabase
    let db = embedded_data::create_world_database()
        .expect("Failed to create world database from embedded data");

    let world_db_mm_count = db.locations_for_game(Game::Mm).count();

    // Verify the WorldDatabase has the expected MM location count from YAML
    // MM has: 46 (bosses) + 934 (dungeons) + 1785 (overworld) + 2 (us_variant) = 2767
    const EXPECTED_YAML_MM_LOCATIONS: usize = 2767;
    assert_eq!(
        world_db_mm_count, EXPECTED_YAML_MM_LOCATIONS,
        "WorldDatabase should have {} MM locations from YAML, got {}",
        EXPECTED_YAML_MM_LOCATIONS, world_db_mm_count
    );

    // Verify that every location from WorldDatabase exists in the flag mapping
    // (i.e., the pipeline doesn't lose any locations)
    let mut missing_locations = Vec::new();
    for (location, _region_id) in db.locations_for_game(Game::Mm) {
        if get_mm_mapping(&location.id).is_none() {
            missing_locations.push(location.id.clone());
        }
    }

    assert!(
        missing_locations.is_empty(),
        "MM flag mapping is missing {} locations from WorldDatabase: {:?}",
        missing_locations.len(),
        missing_locations
    );

    // Verify the mm_flag_mapping count is at least the WorldDatabase count
    let flag_mapping_count = mm_location_count();
    assert!(
        flag_mapping_count >= world_db_mm_count,
        "mm_location_count() ({}) should be >= WorldDatabase MM locations ({})",
        flag_mapping_count,
        world_db_mm_count
    );
}
