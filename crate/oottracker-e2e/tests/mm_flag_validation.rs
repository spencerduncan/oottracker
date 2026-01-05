//! MM flag address validation tests.
//!
//! These tests validate that the MM flag addresses defined in ootmm::events::mm_flags
//! correctly correspond to game memory locations. The tests use the RAM validation
//! infrastructure from oottracker-e2e to verify flag offsets and bit positions.
//!
//! # Test Categories
//!
//! - **Stray Fairies**: 62 total (1 Clock Town + 15 per dungeon × 4)
//! - **Owl Statues**: 10 total (bits 20-29 in quest status at offset 0x84)
//! - **Scene Flags**: Chests, collectibles, and heart pieces across all scenes

use ootmm::events::mm_flags::{
    mm_chest_mappings, mm_collectible_mappings, mm_owl_statue_mappings, mm_special_mappings,
    offsets, owl_bits, scene_ids, MmFlagType, MmSceneFlag,
};
use oottracker_e2e::{CompareMode, ExpectedValue, RamValidator, MM_SAVE_ADDR};

// ============================================================================
// MM Save Context Constants
// ============================================================================

/// MM save context size for validation.
const MM_SAVE_SIZE: usize = 0x48D0;

/// Helper to create a validator for MM save context.
fn mm_validator(name: &str) -> RamValidator {
    RamValidator::new(name, MM_SAVE_ADDR)
}

/// Helper to create mock MM save data with a specific flag set.
fn create_mm_save_data_with_scene_flag(
    scene_id: u8,
    flag_type: MmFlagType,
    bit_mask: u32,
) -> Vec<u8> {
    let mut data = vec![0u8; MM_SAVE_SIZE];

    // Calculate offset for this scene flag
    let scene_offset = offsets::SCENE_FLAGS + (scene_id as usize) * offsets::SCENE_SIZE;
    let flag_offset = scene_offset + flag_type.offset();

    // Set the flag bits (big-endian u32)
    if flag_offset + 4 <= data.len() {
        data[flag_offset] = (bit_mask >> 24) as u8;
        data[flag_offset + 1] = (bit_mask >> 16) as u8;
        data[flag_offset + 2] = (bit_mask >> 8) as u8;
        data[flag_offset + 3] = bit_mask as u8;
    }

    data
}

/// Helper to create mock MM save data with owl statue flags set.
fn create_mm_save_data_with_owl_statues(owl_bits_set: u32) -> Vec<u8> {
    let mut data = vec![0u8; MM_SAVE_SIZE];

    // Set quest status at offset 0x84 (big-endian u32)
    let offset = offsets::QUEST_STATUS;
    if offset + 4 <= data.len() {
        data[offset] = (owl_bits_set >> 24) as u8;
        data[offset + 1] = (owl_bits_set >> 16) as u8;
        data[offset + 2] = (owl_bits_set >> 8) as u8;
        data[offset + 3] = owl_bits_set as u8;
    }

    data
}

// ============================================================================
// Stray Fairy Flag Address Validation (62 total)
// ============================================================================

#[test]
fn test_stray_fairy_count() {
    let fairies = mm_special_mappings();
    assert_eq!(
        fairies.len(),
        62,
        "Expected 62 stray fairy mappings: 1 Clock Town + 15×4 dungeons (with some chest-based)"
    );
}

#[test]
fn test_clock_town_stray_fairy_address() {
    // Clock Town stray fairy is at scene 0x26 (Great Fairy Fountain), collectible flag, bit 0x02
    let fairies = mm_special_mappings();
    let ct_fairy = fairies
        .iter()
        .find(|f| f.location_name == "Clock Town Stray Fairy")
        .expect("Clock Town Stray Fairy not found");

    assert_eq!(ct_fairy.flag.scene_id, scene_ids::CLOCK_TOWN_FAIRY_FOUNTAIN);
    assert_eq!(ct_fairy.flag.flag_type, MmFlagType::Collectible);
    assert_eq!(ct_fairy.flag.bit_mask, 0x0000_0002);

    // Verify the save offset calculation
    let expected_offset = offsets::SCENE_FLAGS
        + (scene_ids::CLOCK_TOWN_FAIRY_FOUNTAIN as usize) * offsets::SCENE_SIZE
        + MmFlagType::Collectible.offset();
    assert_eq!(ct_fairy.flag.save_offset(), expected_offset);
}

#[test]
fn test_clock_town_fairy_ram_validation() {
    let data = create_mm_save_data_with_scene_flag(
        scene_ids::CLOCK_TOWN_FAIRY_FOUNTAIN,
        MmFlagType::Collectible,
        0x0000_0002,
    );

    let flag = MmSceneFlag::new(
        scene_ids::CLOCK_TOWN_FAIRY_FOUNTAIN,
        MmFlagType::Collectible,
        0x0000_0002,
    );

    let validator = mm_validator("Clock Town Stray Fairy")
        .with_mode(CompareMode::BitsSet)
        .expect(ExpectedValue::u32_be(
            flag.save_offset() as u32,
            0x0000_0002,
            "Clock Town Stray Fairy Flag",
        ));

    let report = validator.validate_data(&data);
    assert!(
        report.passed(),
        "Clock Town Stray Fairy validation failed: {:?}",
        report.failures()
    );
}

#[test]
fn test_woodfall_temple_stray_fairies_addresses() {
    let fairies = mm_special_mappings();
    let wf_fairies: Vec<_> = fairies
        .iter()
        .filter(|f| f.location_name.starts_with("Woodfall Temple SF"))
        .collect();

    // Woodfall should have 15 fairies (12 collectible + 3 chest)
    assert_eq!(
        wf_fairies.len(),
        15,
        "Woodfall Temple should have 15 stray fairies"
    );

    // Verify all are in Woodfall Temple scene
    for fairy in &wf_fairies {
        assert_eq!(
            fairy.flag.scene_id,
            scene_ids::WOODFALL_TEMPLE,
            "Fairy {} should be in Woodfall Temple scene",
            fairy.location_name
        );
    }

    // Count by flag type
    let collectible_count = wf_fairies
        .iter()
        .filter(|f| f.flag.flag_type == MmFlagType::Collectible)
        .count();
    let chest_count = wf_fairies
        .iter()
        .filter(|f| f.flag.flag_type == MmFlagType::Chest)
        .count();

    assert_eq!(
        collectible_count, 12,
        "Woodfall should have 12 collectible-type fairies"
    );
    assert_eq!(chest_count, 3, "Woodfall should have 3 chest-type fairies");
}

#[test]
fn test_woodfall_fairy_ram_validation() {
    // Test first Woodfall fairy (Entrance)
    let data = create_mm_save_data_with_scene_flag(
        scene_ids::WOODFALL_TEMPLE,
        MmFlagType::Collectible,
        0x0000_0001,
    );

    let flag = MmSceneFlag::new(
        scene_ids::WOODFALL_TEMPLE,
        MmFlagType::Collectible,
        0x0000_0001,
    );

    let validator = mm_validator("Woodfall Temple SF Entrance")
        .with_mode(CompareMode::BitsSet)
        .expect(ExpectedValue::u32_be(
            flag.save_offset() as u32,
            0x0000_0001,
            "Woodfall Temple SF Entrance Flag",
        ));

    let report = validator.validate_data(&data);
    assert!(
        report.passed(),
        "Woodfall Temple SF Entrance validation failed: {:?}",
        report.failures()
    );
}

#[test]
fn test_snowhead_temple_stray_fairies_addresses() {
    let fairies = mm_special_mappings();
    let sh_fairies: Vec<_> = fairies
        .iter()
        .filter(|f| f.location_name.starts_with("Snowhead Temple SF"))
        .collect();

    // Snowhead should have 15 fairies (13 collectible + 2 chest)
    assert_eq!(
        sh_fairies.len(),
        15,
        "Snowhead Temple should have 15 stray fairies"
    );

    // Verify all are in Snowhead Temple scene
    for fairy in &sh_fairies {
        assert_eq!(
            fairy.flag.scene_id,
            scene_ids::SNOWHEAD_TEMPLE,
            "Fairy {} should be in Snowhead Temple scene",
            fairy.location_name
        );
    }

    let collectible_count = sh_fairies
        .iter()
        .filter(|f| f.flag.flag_type == MmFlagType::Collectible)
        .count();
    let chest_count = sh_fairies
        .iter()
        .filter(|f| f.flag.flag_type == MmFlagType::Chest)
        .count();

    assert_eq!(
        collectible_count, 13,
        "Snowhead should have 13 collectible-type fairies"
    );
    assert_eq!(chest_count, 2, "Snowhead should have 2 chest-type fairies");
}

#[test]
fn test_snowhead_fairy_ram_validation() {
    let data = create_mm_save_data_with_scene_flag(
        scene_ids::SNOWHEAD_TEMPLE,
        MmFlagType::Collectible,
        0x0000_0001,
    );

    let flag = MmSceneFlag::new(
        scene_ids::SNOWHEAD_TEMPLE,
        MmFlagType::Collectible,
        0x0000_0001,
    );

    let validator = mm_validator("Snowhead Temple SF Bridge Under Platform")
        .with_mode(CompareMode::BitsSet)
        .expect(ExpectedValue::u32_be(
            flag.save_offset() as u32,
            0x0000_0001,
            "Snowhead Temple SF Bridge Under Platform Flag",
        ));

    let report = validator.validate_data(&data);
    assert!(
        report.passed(),
        "Snowhead Temple stray fairy validation failed: {:?}",
        report.failures()
    );
}

#[test]
fn test_great_bay_temple_stray_fairies_addresses() {
    let fairies = mm_special_mappings();
    let gb_fairies: Vec<_> = fairies
        .iter()
        .filter(|f| f.location_name.starts_with("Great Bay Temple SF"))
        .collect();

    // Great Bay should have 15 fairies (13 collectible + 2 chest)
    assert_eq!(
        gb_fairies.len(),
        15,
        "Great Bay Temple should have 15 stray fairies"
    );

    // Verify all are in Great Bay Temple scene
    for fairy in &gb_fairies {
        assert_eq!(
            fairy.flag.scene_id,
            scene_ids::GREAT_BAY_TEMPLE,
            "Fairy {} should be in Great Bay Temple scene",
            fairy.location_name
        );
    }

    let collectible_count = gb_fairies
        .iter()
        .filter(|f| f.flag.flag_type == MmFlagType::Collectible)
        .count();
    let chest_count = gb_fairies
        .iter()
        .filter(|f| f.flag.flag_type == MmFlagType::Chest)
        .count();

    assert_eq!(
        collectible_count, 13,
        "Great Bay should have 13 collectible-type fairies"
    );
    assert_eq!(chest_count, 2, "Great Bay should have 2 chest-type fairies");
}

#[test]
fn test_great_bay_fairy_ram_validation() {
    let data = create_mm_save_data_with_scene_flag(
        scene_ids::GREAT_BAY_TEMPLE,
        MmFlagType::Collectible,
        0x0000_0001,
    );

    let flag = MmSceneFlag::new(
        scene_ids::GREAT_BAY_TEMPLE,
        MmFlagType::Collectible,
        0x0000_0001,
    );

    let validator = mm_validator("Great Bay Temple SF Water Wheel Platform")
        .with_mode(CompareMode::BitsSet)
        .expect(ExpectedValue::u32_be(
            flag.save_offset() as u32,
            0x0000_0001,
            "Great Bay Temple SF Water Wheel Platform Flag",
        ));

    let report = validator.validate_data(&data);
    assert!(
        report.passed(),
        "Great Bay Temple stray fairy validation failed: {:?}",
        report.failures()
    );
}

#[test]
fn test_stone_tower_temple_stray_fairies_addresses() {
    let fairies = mm_special_mappings();

    // Normal Stone Tower fairies
    let st_normal_fairies: Vec<_> = fairies
        .iter()
        .filter(|f| {
            f.location_name.starts_with("Stone Tower Temple SF")
                && !f.location_name.contains("Inverted")
        })
        .collect();

    // Inverted Stone Tower fairies
    let st_inverted_fairies: Vec<_> = fairies
        .iter()
        .filter(|f| {
            f.location_name
                .starts_with("Stone Tower Temple Inverted SF")
        })
        .collect();

    // Normal: 8 collectible + 1 chest = 9
    assert_eq!(
        st_normal_fairies.len(),
        9,
        "Normal Stone Tower should have 9 stray fairy mappings"
    );

    // Inverted: 5 collectible + 2 chest = 7
    assert_eq!(
        st_inverted_fairies.len(),
        7,
        "Inverted Stone Tower should have 7 stray fairy mappings"
    );

    // Total should be 16 for Stone Tower (spread across two scenes)
    assert_eq!(
        st_normal_fairies.len() + st_inverted_fairies.len(),
        16,
        "Total Stone Tower fairies should be 16"
    );

    // Verify scene IDs
    for fairy in &st_normal_fairies {
        assert_eq!(
            fairy.flag.scene_id,
            scene_ids::STONE_TOWER_TEMPLE,
            "Normal fairy {} should be in Stone Tower Temple scene",
            fairy.location_name
        );
    }

    for fairy in &st_inverted_fairies {
        assert_eq!(
            fairy.flag.scene_id,
            scene_ids::STONE_TOWER_TEMPLE_INVERTED,
            "Inverted fairy {} should be in Stone Tower Temple Inverted scene",
            fairy.location_name
        );
    }
}

#[test]
fn test_stone_tower_fairy_ram_validation() {
    // Test normal Stone Tower fairy
    let data = create_mm_save_data_with_scene_flag(
        scene_ids::STONE_TOWER_TEMPLE,
        MmFlagType::Collectible,
        0x0000_0001,
    );

    let flag = MmSceneFlag::new(
        scene_ids::STONE_TOWER_TEMPLE,
        MmFlagType::Collectible,
        0x0000_0001,
    );

    let validator = mm_validator("Stone Tower Temple SF Entrance")
        .with_mode(CompareMode::BitsSet)
        .expect(ExpectedValue::u32_be(
            flag.save_offset() as u32,
            0x0000_0001,
            "Stone Tower Temple SF Entrance Flag",
        ));

    let report = validator.validate_data(&data);
    assert!(
        report.passed(),
        "Stone Tower Temple stray fairy validation failed: {:?}",
        report.failures()
    );
}

#[test]
fn test_stone_tower_inverted_fairy_ram_validation() {
    // Test inverted Stone Tower fairy
    let data = create_mm_save_data_with_scene_flag(
        scene_ids::STONE_TOWER_TEMPLE_INVERTED,
        MmFlagType::Collectible,
        0x0000_0001,
    );

    let flag = MmSceneFlag::new(
        scene_ids::STONE_TOWER_TEMPLE_INVERTED,
        MmFlagType::Collectible,
        0x0000_0001,
    );

    let validator = mm_validator("Stone Tower Temple Inverted SF Entrance Air")
        .with_mode(CompareMode::BitsSet)
        .expect(ExpectedValue::u32_be(
            flag.save_offset() as u32,
            0x0000_0001,
            "Stone Tower Temple Inverted SF Entrance Air Flag",
        ));

    let report = validator.validate_data(&data);
    assert!(
        report.passed(),
        "Stone Tower Temple Inverted stray fairy validation failed: {:?}",
        report.failures()
    );
}

// ============================================================================
// Owl Statue Flag Address Validation (10 total)
// ============================================================================

#[test]
fn test_owl_statue_count() {
    let owls = mm_owl_statue_mappings();
    assert_eq!(owls.len(), 10, "Expected 10 owl statue mappings");
}

#[test]
fn test_owl_statue_bit_positions() {
    // Verify owl statue bit positions match documentation (bits 20-29)
    assert_eq!(owl_bits::OWL_CLOCK_TOWN, 20);
    assert_eq!(owl_bits::OWL_MILK_ROAD, 21);
    assert_eq!(owl_bits::OWL_SOUTHERN_SWAMP, 22);
    assert_eq!(owl_bits::OWL_WOODFALL, 23);
    assert_eq!(owl_bits::OWL_MOUNTAIN_VILLAGE, 24);
    assert_eq!(owl_bits::OWL_SNOWHEAD, 25);
    assert_eq!(owl_bits::OWL_ZORA_CAPE, 26);
    assert_eq!(owl_bits::OWL_GREAT_BAY, 27);
    assert_eq!(owl_bits::OWL_IKANA_CANYON, 28);
    assert_eq!(owl_bits::OWL_STONE_TOWER, 29);
}

#[test]
fn test_owl_statue_quest_status_offset() {
    // All owl statues should use QUEST_STATUS offset (0x84)
    let owls = mm_owl_statue_mappings();
    for owl in &owls {
        assert_eq!(
            owl.flag.save_offset(),
            offsets::QUEST_STATUS,
            "Owl statue {} should use QUEST_STATUS offset",
            owl.location_name
        );
    }
}

#[test]
fn test_clock_town_owl_statue_ram_validation() {
    // Clock Town owl is bit 20
    let owl_mask = 1u32 << owl_bits::OWL_CLOCK_TOWN;
    let data = create_mm_save_data_with_owl_statues(owl_mask);

    let validator = mm_validator("Clock Town Owl Statue")
        .with_mode(CompareMode::BitsSet)
        .expect(ExpectedValue::u32_be(
            offsets::QUEST_STATUS as u32,
            owl_mask,
            "Clock Town Owl Statue Bit",
        ));

    let report = validator.validate_data(&data);
    assert!(
        report.passed(),
        "Clock Town owl statue validation failed: {:?}",
        report.failures()
    );
}

#[test]
fn test_all_owl_statues_ram_validation() {
    // Test all owl statues combined
    let all_owls_mask: u32 = (1 << owl_bits::OWL_CLOCK_TOWN)
        | (1 << owl_bits::OWL_MILK_ROAD)
        | (1 << owl_bits::OWL_SOUTHERN_SWAMP)
        | (1 << owl_bits::OWL_WOODFALL)
        | (1 << owl_bits::OWL_MOUNTAIN_VILLAGE)
        | (1 << owl_bits::OWL_SNOWHEAD)
        | (1 << owl_bits::OWL_ZORA_CAPE)
        | (1 << owl_bits::OWL_GREAT_BAY)
        | (1 << owl_bits::OWL_IKANA_CANYON)
        | (1 << owl_bits::OWL_STONE_TOWER);

    let data = create_mm_save_data_with_owl_statues(all_owls_mask);

    // Verify all 10 bits are within the expected range (bits 20-29)
    assert_eq!(
        all_owls_mask, 0x3FF0_0000,
        "All owl bits should be 0x3FF00000"
    );

    let validator = mm_validator("All Owl Statues")
        .with_mode(CompareMode::BitsSet)
        .expect(ExpectedValue::u32_be(
            offsets::QUEST_STATUS as u32,
            all_owls_mask,
            "All Owl Statue Bits",
        ));

    let report = validator.validate_data(&data);
    assert!(
        report.passed(),
        "All owl statues validation failed: {:?}",
        report.failures()
    );
}

#[test]
fn test_individual_owl_statue_bits() {
    let owls = mm_owl_statue_mappings();

    let expected_bits = [
        ("Clock Town Owl Statue", 1u32 << 20),
        ("Milk Road Owl Statue", 1u32 << 21),
        ("Southern Swamp Owl Statue", 1u32 << 22),
        ("Woodfall Owl Statue", 1u32 << 23),
        ("Mountain Village Owl Statue", 1u32 << 24),
        ("Snowhead Owl Statue", 1u32 << 25),
        ("Zora Cape Owl Statue", 1u32 << 26),
        ("Great Bay Coast Owl Statue", 1u32 << 27),
        ("Ikana Canyon Owl Statue", 1u32 << 28),
        ("Stone Tower Owl Statue", 1u32 << 29),
    ];

    for (name, expected_mask) in expected_bits {
        let owl = owls
            .iter()
            .find(|o| o.location_name == name)
            .unwrap_or_else(|| panic!("Owl statue {} not found", name));

        assert_eq!(
            owl.flag.bit_mask, expected_mask,
            "Owl statue {} has wrong bit mask: expected 0x{:08X}, got 0x{:08X}",
            name, expected_mask, owl.flag.bit_mask
        );

        // Validate with mock data
        let data = create_mm_save_data_with_owl_statues(expected_mask);
        let validator =
            mm_validator(name)
                .with_mode(CompareMode::BitsSet)
                .expect(ExpectedValue::u32_be(
                    offsets::QUEST_STATUS as u32,
                    expected_mask,
                    format!("{} Bit", name),
                ));

        let report = validator.validate_data(&data);
        assert!(
            report.passed(),
            "Owl statue {} validation failed: {:?}",
            name,
            report.failures()
        );
    }
}

// ============================================================================
// Scene Flag Address Validation (Chests, Collectibles, Heart Pieces)
// ============================================================================

#[test]
fn test_chest_mappings_not_empty() {
    let chests = mm_chest_mappings();
    assert!(!chests.is_empty(), "Chest mappings should not be empty");
}

#[test]
fn test_collectible_mappings_not_empty() {
    let collectibles = mm_collectible_mappings();
    assert!(
        !collectibles.is_empty(),
        "Collectible mappings should not be empty"
    );
}

#[test]
fn test_woodfall_temple_chest_addresses() {
    let chests = mm_chest_mappings();
    let wf_chests: Vec<_> = chests
        .iter()
        .filter(|c| c.location_name.contains("woodfall_temple"))
        .collect();

    assert!(
        !wf_chests.is_empty(),
        "Should have Woodfall Temple chest mappings"
    );

    // Verify scene ID and flag type
    for chest in &wf_chests {
        assert_eq!(
            chest.flag.scene_id,
            scene_ids::WOODFALL_TEMPLE,
            "Woodfall chest {} should be in Woodfall Temple scene",
            chest.location_name
        );
        assert_eq!(
            chest.flag.flag_type,
            MmFlagType::Chest,
            "Woodfall chest {} should have Chest flag type",
            chest.location_name
        );
    }
}

#[test]
fn test_woodfall_temple_map_chest_ram_validation() {
    // Woodfall Temple map chest: scene 0x1F, chest flag, bit 0x01
    let data = create_mm_save_data_with_scene_flag(
        scene_ids::WOODFALL_TEMPLE,
        MmFlagType::Chest,
        0x0000_0001,
    );

    let flag = MmSceneFlag::new(scene_ids::WOODFALL_TEMPLE, MmFlagType::Chest, 0x0000_0001);

    let validator = mm_validator("Woodfall Temple Map Chest")
        .with_mode(CompareMode::BitsSet)
        .expect(ExpectedValue::u32_be(
            flag.save_offset() as u32,
            0x0000_0001,
            "Woodfall Temple Map Chest Flag",
        ));

    let report = validator.validate_data(&data);
    assert!(
        report.passed(),
        "Woodfall Temple map chest validation failed: {:?}",
        report.failures()
    );
}

#[test]
fn test_snowhead_temple_chest_addresses() {
    let chests = mm_chest_mappings();
    let sh_chests: Vec<_> = chests
        .iter()
        .filter(|c| c.location_name.contains("snowhead_temple"))
        .collect();

    assert!(
        !sh_chests.is_empty(),
        "Should have Snowhead Temple chest mappings"
    );

    for chest in &sh_chests {
        assert_eq!(
            chest.flag.scene_id,
            scene_ids::SNOWHEAD_TEMPLE,
            "Snowhead chest {} should be in Snowhead Temple scene",
            chest.location_name
        );
    }
}

#[test]
fn test_great_bay_temple_chest_addresses() {
    let chests = mm_chest_mappings();
    let gb_chests: Vec<_> = chests
        .iter()
        .filter(|c| c.location_name.contains("great_bay_temple"))
        .collect();

    assert!(
        !gb_chests.is_empty(),
        "Should have Great Bay Temple chest mappings"
    );

    for chest in &gb_chests {
        assert_eq!(
            chest.flag.scene_id,
            scene_ids::GREAT_BAY_TEMPLE,
            "Great Bay chest {} should be in Great Bay Temple scene",
            chest.location_name
        );
    }
}

#[test]
fn test_stone_tower_temple_chest_addresses() {
    let chests = mm_chest_mappings();
    let st_chests: Vec<_> = chests
        .iter()
        .filter(|c| {
            c.location_name.contains("stone_tower_temple") && !c.location_name.contains("inverted")
        })
        .collect();

    assert!(
        !st_chests.is_empty(),
        "Should have Stone Tower Temple chest mappings"
    );

    for chest in &st_chests {
        assert_eq!(
            chest.flag.scene_id,
            scene_ids::STONE_TOWER_TEMPLE,
            "Stone Tower chest {} should be in Stone Tower Temple scene",
            chest.location_name
        );
    }
}

#[test]
fn test_stone_tower_inverted_chest_addresses() {
    let chests = mm_chest_mappings();
    let st_inv_chests: Vec<_> = chests
        .iter()
        .filter(|c| c.location_name.contains("stone_tower_temple_inverted"))
        .collect();

    assert!(
        !st_inv_chests.is_empty(),
        "Should have Stone Tower Temple Inverted chest mappings"
    );

    for chest in &st_inv_chests {
        assert_eq!(
            chest.flag.scene_id,
            scene_ids::STONE_TOWER_TEMPLE_INVERTED,
            "Stone Tower Inverted chest {} should be in Stone Tower Temple Inverted scene",
            chest.location_name
        );
    }
}

#[test]
fn test_swamp_spider_house_skulltula_addresses() {
    let collectibles = mm_collectible_mappings();
    let swamp_skulls: Vec<_> = collectibles
        .iter()
        .filter(|c| c.location_name.contains("swamp_skulltula"))
        .collect();

    // Swamp Spider House has 30 skulltulas
    assert_eq!(
        swamp_skulls.len(),
        30,
        "Swamp Spider House should have 30 skulltula mappings"
    );

    // All should be in scene 0x27 with Collectible flag type
    for skull in &swamp_skulls {
        assert_eq!(
            skull.flag.scene_id, 0x27,
            "Swamp skulltula {} should be in scene 0x27",
            skull.location_name
        );
        assert_eq!(
            skull.flag.flag_type,
            MmFlagType::Collectible,
            "Swamp skulltula {} should have Collectible flag type",
            skull.location_name
        );
    }
}

#[test]
fn test_swamp_skulltula_ram_validation() {
    // First Swamp skulltula: scene 0x27, collectible flag, bit 0x01
    let data = create_mm_save_data_with_scene_flag(0x27, MmFlagType::Collectible, 0x0000_0001);

    let flag = MmSceneFlag::new(0x27, MmFlagType::Collectible, 0x0000_0001);

    let validator = mm_validator("Swamp Skulltula Main Room Near Ceiling")
        .with_mode(CompareMode::BitsSet)
        .expect(ExpectedValue::u32_be(
            flag.save_offset() as u32,
            0x0000_0001,
            "Swamp Skulltula Main Room Near Ceiling Flag",
        ));

    let report = validator.validate_data(&data);
    assert!(
        report.passed(),
        "Swamp skulltula validation failed: {:?}",
        report.failures()
    );
}

#[test]
fn test_ocean_spider_house_skulltula_addresses() {
    let collectibles = mm_collectible_mappings();
    let ocean_skulls: Vec<_> = collectibles
        .iter()
        .filter(|c| c.location_name.contains("ocean_skulltula"))
        .collect();

    // Ocean Spider House has 30 skulltulas
    assert_eq!(
        ocean_skulls.len(),
        30,
        "Ocean Spider House should have 30 skulltula mappings"
    );

    // All should be in scene 0x28 with Collectible flag type
    for skull in &ocean_skulls {
        assert_eq!(
            skull.flag.scene_id, 0x28,
            "Ocean skulltula {} should be in scene 0x28",
            skull.location_name
        );
        assert_eq!(
            skull.flag.flag_type,
            MmFlagType::Collectible,
            "Ocean skulltula {} should have Collectible flag type",
            skull.location_name
        );
    }
}

#[test]
fn test_heart_piece_collectible_addresses() {
    let collectibles = mm_collectible_mappings();
    let heart_pieces: Vec<_> = collectibles
        .iter()
        .filter(|c| c.location_name.contains("_hp"))
        .collect();

    assert!(
        !heart_pieces.is_empty(),
        "Should have heart piece collectible mappings"
    );

    // All heart pieces should have Collectible flag type
    for hp in &heart_pieces {
        assert_eq!(
            hp.flag.flag_type,
            MmFlagType::Collectible,
            "Heart piece {} should have Collectible flag type",
            hp.location_name
        );
    }
}

// ============================================================================
// Scene Flag Offset Calculations
// ============================================================================

#[test]
fn test_scene_flag_offset_calculation() {
    // Scene 0 offsets
    let scene0_chest = MmSceneFlag::new(0, MmFlagType::Chest, 0x01);
    assert_eq!(scene0_chest.save_offset(), 0x00);

    let scene0_switch = MmSceneFlag::new(0, MmFlagType::Switch, 0x01);
    assert_eq!(scene0_switch.save_offset(), 0x04);

    let scene0_room_clear = MmSceneFlag::new(0, MmFlagType::RoomClear, 0x01);
    assert_eq!(scene0_room_clear.save_offset(), 0x08);

    let scene0_collectible = MmSceneFlag::new(0, MmFlagType::Collectible, 0x01);
    assert_eq!(scene0_collectible.save_offset(), 0x0C);

    let scene0_special = MmSceneFlag::new(0, MmFlagType::Special, 0x01);
    assert_eq!(scene0_special.save_offset(), 0x10);
}

#[test]
fn test_scene_flag_offset_scene_1() {
    // Scene 1 should be offset by SCENE_SIZE (0x14)
    let scene1_chest = MmSceneFlag::new(1, MmFlagType::Chest, 0x01);
    assert_eq!(scene1_chest.save_offset(), 0x14);

    let scene1_collectible = MmSceneFlag::new(1, MmFlagType::Collectible, 0x01);
    assert_eq!(scene1_collectible.save_offset(), 0x14 + 0x0C);
}

#[test]
fn test_clock_town_scene_offsets() {
    // Clock Town scenes are 0x6C-0x6F
    let ct_south_chest = MmSceneFlag::new(0x6C, MmFlagType::Chest, 0x01);
    assert_eq!(ct_south_chest.save_offset(), 0x6C * offsets::SCENE_SIZE);

    let ct_north_chest = MmSceneFlag::new(0x6D, MmFlagType::Chest, 0x01);
    assert_eq!(ct_north_chest.save_offset(), 0x6D * offsets::SCENE_SIZE);

    let ct_east_chest = MmSceneFlag::new(0x6E, MmFlagType::Chest, 0x01);
    assert_eq!(ct_east_chest.save_offset(), 0x6E * offsets::SCENE_SIZE);
}

// ============================================================================
// Memory Layout Validation
// ============================================================================

#[test]
fn test_mm_save_base_address() {
    assert_eq!(offsets::MM_SAVE_BASE, 0x1EF670);
}

#[test]
fn test_quest_status_offset() {
    assert_eq!(offsets::QUEST_STATUS, 0x84);
}

#[test]
fn test_scene_flags_offset() {
    assert_eq!(offsets::SCENE_FLAGS, 0x00);
}

#[test]
fn test_scene_size() {
    assert_eq!(offsets::SCENE_SIZE, 0x14);
}

#[test]
fn test_stray_fairy_counts_offset() {
    assert_eq!(offsets::STRAY_FAIRY_COUNTS, 0x0E94);
}

#[test]
fn test_flag_type_offsets() {
    assert_eq!(MmFlagType::Chest.offset(), 0x00);
    assert_eq!(MmFlagType::Switch.offset(), 0x04);
    assert_eq!(MmFlagType::RoomClear.offset(), 0x08);
    assert_eq!(MmFlagType::Collectible.offset(), 0x0C);
    assert_eq!(MmFlagType::Special.offset(), 0x10);
}

// ============================================================================
// Full Stray Fairy ExpectedValue Spec Generation
// ============================================================================

#[test]
fn test_generate_stray_fairy_expected_values() {
    let fairies = mm_special_mappings();

    // Generate ExpectedValue specs for all stray fairies
    let mut expected_values = Vec::new();

    for fairy in &fairies {
        let offset = fairy.flag.save_offset() as u32;
        let expected = ExpectedValue::u32_be(offset, fairy.flag.bit_mask, &fairy.location_name);
        expected_values.push(expected);
    }

    assert_eq!(
        expected_values.len(),
        62,
        "Should generate 62 ExpectedValue specs"
    );

    // Verify each spec has valid offset and non-zero bit mask
    for ev in &expected_values {
        assert!(
            ev.offset < MM_SAVE_SIZE as u32,
            "Offset should be within save data"
        );
        assert_eq!(ev.expected.len(), 4, "Should be u32 (4 bytes)");
    }
}

// ============================================================================
// Full Owl Statue ExpectedValue Spec Generation
// ============================================================================

#[test]
fn test_generate_owl_statue_expected_values() {
    let owls = mm_owl_statue_mappings();

    // Generate ExpectedValue specs for all owl statues
    let expected_values: Vec<_> = owls
        .iter()
        .map(|owl| {
            ExpectedValue::u32_be(
                offsets::QUEST_STATUS as u32,
                owl.flag.bit_mask,
                &owl.location_name,
            )
        })
        .collect();

    assert_eq!(
        expected_values.len(),
        10,
        "Should generate 10 ExpectedValue specs"
    );

    // All should use the same offset (QUEST_STATUS)
    for ev in &expected_values {
        assert_eq!(
            ev.offset,
            offsets::QUEST_STATUS as u32,
            "All owl statues should use QUEST_STATUS offset"
        );
    }
}

// ============================================================================
// Comprehensive Flag Validation with BitsSet Mode
// ============================================================================

#[test]
fn test_multiple_flags_bits_set_validation() {
    // Test setting multiple chest flags in same scene
    let multi_flags = 0x0000_000F; // First 4 chest bits set
    let data = create_mm_save_data_with_scene_flag(
        scene_ids::WOODFALL_TEMPLE,
        MmFlagType::Chest,
        multi_flags,
    );

    let flag = MmSceneFlag::new(scene_ids::WOODFALL_TEMPLE, MmFlagType::Chest, multi_flags);

    // Verify we can check for subset of bits
    let validator = mm_validator("Woodfall Temple First 4 Chests")
        .with_mode(CompareMode::BitsSet)
        .expect(ExpectedValue::u32_be(
            flag.save_offset() as u32,
            0x0000_0001, // Just check first bit
            "First Chest",
        ))
        .expect(ExpectedValue::u32_be(
            flag.save_offset() as u32,
            0x0000_0002, // Check second bit
            "Second Chest",
        ));

    let report = validator.validate_data(&data);
    assert!(
        report.passed(),
        "Multiple chest validation failed: {:?}",
        report.failures()
    );
}

#[test]
fn test_flag_not_set_fails_validation() {
    // Create data with NO flags set
    let data = vec![0u8; MM_SAVE_SIZE];

    let flag = MmSceneFlag::new(scene_ids::WOODFALL_TEMPLE, MmFlagType::Chest, 0x0000_0001);

    let validator = mm_validator("Woodfall Temple Map Chest")
        .with_mode(CompareMode::BitsSet)
        .expect(ExpectedValue::u32_be(
            flag.save_offset() as u32,
            0x0000_0001,
            "Woodfall Temple Map Chest Flag",
        ));

    let report = validator.validate_data(&data);
    assert!(
        !report.passed(),
        "Validation should fail when flag is not set"
    );
    assert_eq!(report.fail_count(), 1, "Should have exactly 1 failure");
}
