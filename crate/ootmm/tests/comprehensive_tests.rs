//! Comprehensive tests for the ootmm crate (T8.3).
//!
//! This module provides comprehensive test coverage for:
//! - Expression parser edge cases (~20 tests)
//! - WorldDatabase validation (~10 tests)
//! - Item name/ID mapping (~15 tests)
//! - Rando trait implementation (~10 tests)

// ============================================================================
// Expression Parser Edge Cases (~20 tests)
// ============================================================================

mod expression_parser_edge_cases {
    use ootmm::expr::{parse, Expr};

    // --- Nested Expression Tests ---

    #[test]
    fn test_deeply_nested_and_chain() {
        // 5 levels of AND nesting
        let expr = parse("a && b && c && d && e && f").unwrap();
        // Should be left-associative: ((((a && b) && c) && d) && e) && f
        match expr {
            Expr::And(_, _) => {}
            _ => panic!("expected And at top level"),
        }
    }

    #[test]
    fn test_deeply_nested_or_chain() {
        // 5 levels of OR nesting
        let expr = parse("a || b || c || d || e || f").unwrap();
        // Should be left-associative: ((((a || b) || c) || d) || e) || f
        match expr {
            Expr::Or(_, _) => {}
            _ => panic!("expected Or at top level"),
        }
    }

    #[test]
    fn test_mixed_nested_operators() {
        // Complex mix of AND/OR with nesting
        let expr = parse("a && b || c && d || e && f").unwrap();
        // Due to precedence: (a && b) || (c && d) || (e && f)
        // Then left-assoc OR: ((a && b) || (c && d)) || (e && f)
        assert!(matches!(expr, Expr::Or(_, _)));
    }

    #[test]
    fn test_parentheses_override_all_precedence() {
        // Parentheses should override natural precedence
        let expr = parse("((a || b) && (c || d)) && (e || f)").unwrap();
        assert!(matches!(expr, Expr::And(_, _)));
    }

    #[test]
    fn test_nested_not_operators() {
        let expr = parse("!!!!a").unwrap();
        // Should be !(!(!(!(a))))
        assert_eq!(
            expr,
            Expr::not(Expr::not(Expr::not(Expr::not(Expr::Ident("a".into())))))
        );
    }

    #[test]
    fn test_not_with_nested_and_or() {
        let expr = parse("!(a && b) || !(c || d)").unwrap();
        // Top level should be OR
        assert!(matches!(expr, Expr::Or(_, _)));
    }

    #[test]
    fn test_function_in_nested_expression() {
        let expr = parse("has(A) && (has(B) || has(C)) && has(D)").unwrap();
        assert!(matches!(expr, Expr::And(_, _)));
    }

    // --- Operator Precedence Edge Cases ---

    #[test]
    fn test_precedence_not_binds_tightest() {
        // !a && b || c should be ((!a) && b) || c
        let expr = parse("!a && b || c").unwrap();
        // Top level should be OR (lowest precedence)
        assert!(matches!(expr, Expr::Or(_, _)));
    }

    #[test]
    fn test_precedence_and_over_or_complex() {
        // a || b && c || d && e should parse as a || (b && c) || (d && e)
        let expr = parse("a || b && c || d && e").unwrap();
        // Left-associative: (a || (b && c)) || (d && e)
        assert!(matches!(expr, Expr::Or(_, _)));
    }

    #[test]
    fn test_precedence_with_functions() {
        // has(X) || has(Y) && has(Z) should be has(X) || (has(Y) && has(Z))
        let expr = parse("has(X) || has(Y) && has(Z)").unwrap();
        assert!(matches!(expr, Expr::Or(_, _)));
    }

    // --- Count/Setting Expression Tests ---

    #[test]
    fn test_count_expression_single_arg() {
        let expr = parse("count(SKULLTULA)").unwrap();
        assert_eq!(
            expr,
            Expr::call("count", vec![Expr::Ident("SKULLTULA".into())])
        );
    }

    #[test]
    fn test_count_expression_with_number() {
        let expr = parse("count(SKULLTULA, 50)").unwrap();
        assert_eq!(
            expr,
            Expr::call(
                "count",
                vec![Expr::Ident("SKULLTULA".into()), Expr::Number(50)]
            )
        );
    }

    #[test]
    fn test_setting_expression_basic() {
        let expr = parse("setting(open_door_of_time)").unwrap();
        assert_eq!(
            expr,
            Expr::call("setting", vec![Expr::Ident("open_door_of_time".into())])
        );
    }

    #[test]
    fn test_setting_with_string_arg() {
        let expr = parse("setting(\"shuffle_songs\")").unwrap();
        assert_eq!(
            expr,
            Expr::call("setting", vec![Expr::String("shuffle_songs".into())])
        );
    }

    #[test]
    fn test_count_in_complex_expression() {
        let expr = parse("count(KEY, 3) && setting(shuffle_keys) || has(MASTER_KEY)").unwrap();
        // Top level OR
        assert!(matches!(expr, Expr::Or(_, _)));
    }

    #[test]
    fn test_nested_function_calls() {
        let expr = parse("outer(inner(x), another(y, z))").unwrap();
        assert_eq!(
            expr,
            Expr::call(
                "outer",
                vec![
                    Expr::call("inner", vec![Expr::Ident("x".into())]),
                    Expr::call(
                        "another",
                        vec![Expr::Ident("y".into()), Expr::Ident("z".into())]
                    )
                ]
            )
        );
    }

    // --- Additional Edge Cases ---

    #[test]
    fn test_expression_with_underscore_identifiers() {
        let expr = parse("is_adult && has_hookshot && can_use_bow").unwrap();
        assert!(matches!(expr, Expr::And(_, _)));
    }

    #[test]
    fn test_expression_with_numbers_in_identifiers() {
        let expr = parse("room1_clear && room2_clear").unwrap();
        assert!(matches!(expr, Expr::And(_, _)));
    }

    #[test]
    fn test_expression_boolean_literals_in_logic() {
        let expr = parse("true && has(ITEM) || false").unwrap();
        assert!(matches!(expr, Expr::Or(_, _)));
    }

    #[test]
    fn test_expression_all_literal_types() {
        let expr = parse("func(true, false, 42, \"string\", ident)").unwrap();
        assert_eq!(
            expr,
            Expr::call(
                "func",
                vec![
                    Expr::Bool(true),
                    Expr::Bool(false),
                    Expr::Number(42),
                    Expr::String("string".into()),
                    Expr::Ident("ident".into()),
                ]
            )
        );
    }

    #[test]
    fn test_realistic_ootmm_logic_expression() {
        // A realistic OoTMM randomizer logic expression
        let expr = parse(
            "(is_adult && (has(HOOKSHOT) || has(LONGSHOT))) || \
             (is_child && has(BOOMERANG) && setting(boomerang_access))",
        )
        .unwrap();
        assert!(matches!(expr, Expr::Or(_, _)));
    }

    #[test]
    fn test_complex_count_setting_combination() {
        let expr = parse(
            "count(GOLD_SKULLTULA, 50) && setting(bridge_condition) && \
             (has(LIGHT_MEDALLION) || count(MEDALLION, 6))",
        )
        .unwrap();
        assert!(matches!(expr, Expr::And(_, _)));
    }
}

// ============================================================================
// WorldDatabase Validation Tests (~10 tests)
// ============================================================================

mod world_database_validation {
    use ootmm::region::{Exit, ExitType, Game, Region};
    use ootmm::world_database::WorldDatabase;

    fn create_test_region(id: &str, name: &str, game: Game) -> Region {
        Region::new(id, name, game)
    }

    #[test]
    fn test_validate_empty_database() {
        let db = WorldDatabase::new();
        assert!(db.validate().is_ok());
    }

    #[test]
    fn test_validate_single_region_no_exits() {
        let mut db = WorldDatabase::new();
        db.add_region(create_test_region("region1", "Region One", Game::Oot))
            .unwrap();
        assert!(db.validate().is_ok());
    }

    #[test]
    fn test_validate_valid_exit_reference() {
        let mut db = WorldDatabase::new();

        let mut region1 = create_test_region("region1", "Region One", Game::Oot);
        region1.add_exit(Exit::new("region2", ExitType::Overworld));

        let region2 = create_test_region("region2", "Region Two", Game::Oot);

        db.add_region(region1).unwrap();
        db.add_region(region2).unwrap();

        assert!(db.validate().is_ok());
    }

    #[test]
    fn test_validate_invalid_exit_reference() {
        let mut db = WorldDatabase::new();

        let mut region1 = create_test_region("region1", "Region One", Game::Oot);
        region1.add_exit(Exit::new("nonexistent", ExitType::Overworld));

        db.add_region(region1).unwrap();

        let result = db.validate();
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("unknown region"));
    }

    #[test]
    fn test_validate_circular_exit_references() {
        // A -> B -> C -> A is valid as long as all regions exist
        let mut db = WorldDatabase::new();

        let mut region_a = create_test_region("A", "Region A", Game::Oot);
        region_a.add_exit(Exit::new("B", ExitType::Overworld));

        let mut region_b = create_test_region("B", "Region B", Game::Oot);
        region_b.add_exit(Exit::new("C", ExitType::Overworld));

        let mut region_c = create_test_region("C", "Region C", Game::Oot);
        region_c.add_exit(Exit::new("A", ExitType::Overworld));

        db.add_region(region_a).unwrap();
        db.add_region(region_b).unwrap();
        db.add_region(region_c).unwrap();

        assert!(db.validate().is_ok());
    }

    #[test]
    fn test_validate_self_referencing_exit() {
        // A region with an exit to itself should be valid
        let mut db = WorldDatabase::new();

        let mut region = create_test_region("self_ref", "Self Referencing", Game::Oot);
        region.add_exit(Exit::new("self_ref", ExitType::Overworld));

        db.add_region(region).unwrap();

        assert!(db.validate().is_ok());
    }

    #[test]
    fn test_validate_multiple_invalid_exits() {
        let mut db = WorldDatabase::new();

        let mut region = create_test_region("region1", "Region One", Game::Oot);
        region.add_exit(Exit::new("invalid1", ExitType::Overworld));
        region.add_exit(Exit::new("invalid2", ExitType::Dungeon));

        db.add_region(region).unwrap();

        let result = db.validate();
        assert!(result.is_err());
        // Should fail on first invalid exit
        assert!(result.unwrap_err().to_string().contains("unknown region"));
    }

    #[test]
    fn test_validate_mixed_valid_invalid_exits() {
        let mut db = WorldDatabase::new();

        let mut region1 = create_test_region("region1", "Region One", Game::Oot);
        region1.add_exit(Exit::new("region2", ExitType::Overworld)); // Valid
        region1.add_exit(Exit::new("invalid", ExitType::Dungeon)); // Invalid

        let region2 = create_test_region("region2", "Region Two", Game::Oot);

        db.add_region(region1).unwrap();
        db.add_region(region2).unwrap();

        let result = db.validate();
        assert!(result.is_err());
    }

    #[test]
    fn test_validate_cross_game_exits() {
        // OoT region referencing MM region should still be valid (OoTMM)
        let mut db = WorldDatabase::new();

        let mut oot_region = create_test_region("kokiri_forest", "Kokiri Forest", Game::Oot);
        oot_region.add_exit(Exit::new("clock_town", ExitType::Warp));

        let mm_region = create_test_region("clock_town", "Clock Town", Game::Mm);

        db.add_region(oot_region).unwrap();
        db.add_region(mm_region).unwrap();

        assert!(db.validate().is_ok());
    }

    #[test]
    fn test_validate_large_database() {
        let mut db = WorldDatabase::new();

        // Create 100 regions with valid interconnections
        for i in 0..100 {
            let mut region = create_test_region(
                &format!("region_{}", i),
                &format!("Region {}", i),
                Game::Oot,
            );

            // Each region connects to the next (wrapping)
            let next = (i + 1) % 100;
            region.add_exit(Exit::new(&format!("region_{}", next), ExitType::Overworld));

            db.add_region(region).unwrap();
        }

        assert!(db.validate().is_ok());
        assert_eq!(db.region_count(), 100);
        assert_eq!(db.exit_count(), 100);
    }
}

// ============================================================================
// Item Name/ID Mapping Tests (~15 tests)
// ============================================================================

mod item_mapping_comprehensive {
    use ootmm::item::{Item, ItemCategory, MmItem, OotItem};
    use ootmm::items::{ItemMapping, ItemName};

    // --- All OoT Items Lookup Tests ---

    #[test]
    fn test_all_oot_swords_lookup() {
        let swords = [
            ("KokiriSword", OotItem::KokiriSword),
            ("MasterSword", OotItem::MasterSword),
            ("BiggoronSword", OotItem::BiggoronSword),
            ("GiantKnife", OotItem::GiantKnife),
        ];

        for (name, expected) in swords {
            assert_eq!(
                OotItem::by_name(name),
                Some(expected),
                "Failed for {}",
                name
            );
        }
    }

    #[test]
    fn test_all_oot_equipment_lookup() {
        let equipment = [
            "DekuStick",
            "DekuNut",
            "Bomb",
            "Bow",
            "FireArrow",
            "IceArrow",
            "LightArrow",
            "DinsFire",
            "FaroresWind",
            "NayrusLove",
            "Slingshot",
            "Boomerang",
            "Hookshot",
            "Longshot",
            "LensOfTruth",
            "MegatonHammer",
            "OcarinaOfTime",
        ];

        for name in equipment {
            assert!(
                OotItem::by_name(name).is_some(),
                "OoT equipment '{}' not found",
                name
            );
        }
    }

    #[test]
    fn test_all_oot_songs_lookup() {
        let songs = [
            "ZeldasLullaby",
            "EponasSong",
            "SariasSong",
            "SunsSong",
            "SongOfTime",
            "SongOfStorms",
            "MinuetOfForest",
            "BoleroOfFire",
            "SerenadeOfWater",
            "NocturneOfShadow",
            "RequiemOfSpirit",
            "PreludeOfLight",
            "ScarecrowSong",
        ];

        for name in songs {
            let item = OotItem::by_name(name);
            assert!(item.is_some(), "OoT song '{}' not found", name);
            assert!(item.unwrap().is_song(), "'{}' should be a song", name);
        }
    }

    #[test]
    fn test_all_oot_dungeon_keys_lookup() {
        let keys = [
            "SmallKey",
            "BossKey",
            "SmallKeyForestTemple",
            "SmallKeyFireTemple",
            "SmallKeyWaterTemple",
            "SmallKeyShadowTemple",
            "SmallKeySpiritTemple",
            "SmallKeyBottomOfTheWell",
            "SmallKeyGerudoFortress",
            "SmallKeyGerudoTrainingGround",
            "SmallKeyGanonsCastle",
            "BossKeyForestTemple",
            "BossKeyFireTemple",
            "BossKeyWaterTemple",
            "BossKeyShadowTemple",
            "BossKeySpiritTemple",
            "BossKeyGanonsCastle",
        ];

        for name in keys {
            let item = OotItem::by_name(name);
            assert!(item.is_some(), "OoT key '{}' not found", name);
            assert!(
                item.unwrap().is_dungeon_item(),
                "'{}' should be dungeon item",
                name
            );
        }
    }

    // --- All MM Items Lookup Tests ---

    #[test]
    fn test_all_mm_transformation_masks_lookup() {
        let masks = [
            ("DekuMask", MmItem::DekuMask),
            ("GoronMask", MmItem::GoronMask),
            ("ZoraMask", MmItem::ZoraMask),
            ("FierceDeityMask", MmItem::FierceDeityMask),
        ];

        for (name, expected) in masks {
            let item = MmItem::by_name(name);
            assert_eq!(item, Some(expected), "Failed for {}", name);
            assert!(
                item.unwrap().is_transformation_mask(),
                "'{}' should be transformation mask",
                name
            );
        }
    }

    #[test]
    fn test_all_mm_regular_masks_lookup() {
        let masks = [
            "PostmanHat",
            "AllNightMask",
            "BlastMask",
            "StoneMask",
            "GreatFairyMask",
            "KeatonMask",
            "BremenMask",
            "BunnyHood",
            "DonGeroMask",
            "MaskOfScents",
            "RomaniMask",
            "CircusLeaderMask",
            "KafeiMask",
            "CouplesMask",
            "MaskOfTruth",
            "KamaroMask",
            "GibdoMask",
            "GaroMask",
            "CaptainHat",
            "GiantMask",
        ];

        for name in masks {
            let item = MmItem::by_name(name);
            assert!(item.is_some(), "MM mask '{}' not found", name);
            assert!(item.unwrap().is_mask(), "'{}' should be a mask", name);
        }
    }

    #[test]
    fn test_all_mm_songs_lookup() {
        let songs = [
            "SongOfTime",
            "SongOfHealing",
            "EponasSong",
            "SongOfSoaring",
            "SongOfStorms",
            "SonataOfAwakening",
            "GoronLullaby",
            "NewWaveBossaNova",
            "ElegyOfEmptiness",
            "OathToOrder",
        ];

        for name in songs {
            let item = MmItem::by_name(name);
            assert!(item.is_some(), "MM song '{}' not found", name);
            assert!(item.unwrap().is_song(), "'{}' should be a song", name);
        }
    }

    #[test]
    fn test_all_mm_boss_remains_lookup() {
        let remains = [
            ("OdolwaRemains", MmItem::OdolwaRemains),
            ("GohtRemains", MmItem::GohtRemains),
            ("GyorgRemains", MmItem::GyorgRemains),
            ("TwinmoldRemains", MmItem::TwinmoldRemains),
        ];

        for (name, expected) in remains {
            let item = MmItem::by_name(name);
            assert_eq!(item, Some(expected), "Failed for {}", name);
            assert!(
                item.unwrap().is_boss_remain(),
                "'{}' should be boss remain",
                name
            );
        }
    }

    #[test]
    fn test_all_mm_stray_fairies_lookup() {
        let fairies = [
            "StrayFairy",
            "StrayFairyWoodfall",
            "StrayFairySnowhead",
            "StrayFairyGreatBay",
            "StrayFairyStoneTower",
            "StrayFairyClockTown",
        ];

        for name in fairies {
            let item = MmItem::by_name(name);
            assert!(item.is_some(), "MM stray fairy '{}' not found", name);
        }
    }

    // --- Combined Item Tests ---

    #[test]
    fn test_combined_item_oot_priority() {
        // Items existing in both games should return OoT variant via Item::by_name
        let shared_items = ["Hookshot", "Bomb", "Bow", "LensOfTruth"];

        for name in shared_items {
            let item = Item::by_name(name);
            assert!(item.is_some(), "Item '{}' not found", name);
            assert!(
                matches!(item.unwrap(), Item::Oot(_)),
                "'{}' should be OoT variant",
                name
            );
        }
    }

    #[test]
    fn test_mm_specific_items_via_combined() {
        // MM-only items should return MM variant
        let mm_only = [
            "DekuMask",
            "OdolwaRemains",
            "SongOfHealing",
            "GreatFairySword",
        ];

        for name in mm_only {
            let item = Item::by_name(name);
            assert!(item.is_some(), "Item '{}' not found", name);
            assert!(
                matches!(item.unwrap(), Item::Mm(_)),
                "'{}' should be MM variant",
                name
            );
        }
    }

    #[test]
    fn test_item_name_wrapper_roundtrip() {
        // Test ItemName roundtrip conversions
        let oot_item = OotItem::MasterSword;
        let name = ItemName::from(oot_item);
        let back: OotItem = name.try_into().unwrap();
        assert_eq!(oot_item, back);

        let mm_item = MmItem::DekuMask;
        let name = ItemName::from(mm_item);
        let back: MmItem = name.try_into().unwrap();
        assert_eq!(mm_item, back);
    }

    #[test]
    fn test_item_mapping_batch_parsing() {
        let mapper = ItemMapping::new();

        // Test OoT batch parsing
        let oot_names = [
            "MasterSword",
            "Hookshot",
            "BoleroOfFire",
            "InvalidItem",
            "Boomerang",
        ];
        let (items, failed) = mapper.parse_oot_items(oot_names);
        assert_eq!(items.len(), 4); // MasterSword, Hookshot, BoleroOfFire, Boomerang
        assert_eq!(failed.len(), 1); // InvalidItem
        assert_eq!(failed[0], "InvalidItem");

        // Test MM batch parsing
        let mm_names = ["DekuMask", "GoronMask", "Boomerang", "SongOfHealing"];
        let (items, failed) = mapper.parse_mm_items(mm_names);
        assert_eq!(items.len(), 3); // DekuMask, GoronMask, SongOfHealing
        assert_eq!(failed.len(), 1); // Boomerang (OoT only)
    }

    #[test]
    fn test_item_category_correctness() {
        // Verify categories are correctly assigned
        assert_eq!(
            Item::Oot(OotItem::MasterSword).category(),
            ItemCategory::Sword
        );
        assert_eq!(
            Item::Oot(OotItem::Hookshot).category(),
            ItemCategory::Equipment
        );
        assert_eq!(
            Item::Oot(OotItem::ZeldasLullaby).category(),
            ItemCategory::Song
        );
        assert_eq!(
            Item::Oot(OotItem::SmallKeyFireTemple).category(),
            ItemCategory::SmallKey
        );
        assert_eq!(
            Item::Mm(MmItem::DekuMask).category(),
            ItemCategory::TransformationMask
        );
        assert_eq!(Item::Mm(MmItem::BunnyHood).category(), ItemCategory::Mask);
        assert_eq!(
            Item::Mm(MmItem::OdolwaRemains).category(),
            ItemCategory::QuestItem
        );
    }

    #[test]
    fn test_snake_case_to_pascal_case_mapping() {
        // Both naming conventions should work
        let test_cases = [
            ("master_sword", "MasterSword"),
            ("deku_mask", "DekuMask"),
            ("song_of_time", "SongOfTime"),
            ("fierce_deity_mask", "FierceDeityMask"),
            ("small_key_fire_temple", "SmallKeyFireTemple"),
        ];

        for (snake, pascal) in test_cases {
            let snake_item = Item::by_name(snake);
            let pascal_item = Item::by_name(pascal);
            assert!(snake_item.is_some(), "snake_case '{}' not found", snake);
            assert!(pascal_item.is_some(), "PascalCase '{}' not found", pascal);
            assert_eq!(
                snake_item, pascal_item,
                "'{}' and '{}' should be equal",
                snake, pascal
            );
        }
    }
}

// ============================================================================
// Rando Trait Implementation Tests (~10 tests)
// ============================================================================

mod rando_trait_implementation {
    use ootmm::rando::{OotmmRando, OotmmRegionName};
    use ootr::item::Item as OotrItem;
    use ootr::Rando;
    use std::collections::HashSet;

    #[test]
    fn test_ootmm_rando_creation() {
        let rando = OotmmRando::new();
        assert!(rando.is_ok(), "OotmmRando::new() should succeed");
    }

    #[test]
    fn test_ootmm_rando_item_table_populated() {
        let rando = OotmmRando::new().unwrap();
        let items = rando.item_table().unwrap();

        // Should have items from both games
        assert!(!items.is_empty(), "Item table should not be empty");

        // Check for specific items and verify their values
        assert_eq!(
            items.get("MasterSword"),
            Some(&OotrItem("MasterSword".to_string())),
            "MasterSword should have correct value"
        );
        assert_eq!(
            items.get("Hookshot"),
            Some(&OotrItem("Hookshot".to_string())),
            "Hookshot should have correct value"
        );
        assert_eq!(
            items.get("DekuMask"),
            Some(&OotrItem("DekuMask".to_string())),
            "DekuMask should have correct value"
        );
        assert_eq!(
            items.get("OdolwaRemains"),
            Some(&OotrItem("OdolwaRemains".to_string())),
            "OdolwaRemains should have correct value"
        );
    }

    #[test]
    fn test_ootmm_rando_escaped_items() {
        let rando = OotmmRando::new().unwrap();
        let escaped = rando.escaped_items().unwrap();

        // Should contain major progression items
        assert!(!escaped.is_empty(), "Escaped items should not be empty");

        // Key progression items should be escaped with correct values
        assert_eq!(
            escaped.get("Hookshot"),
            Some(&OotrItem("Hookshot".to_string())),
            "Hookshot should be escaped with correct value"
        );
        assert_eq!(
            escaped.get("DekuMask"),
            Some(&OotrItem("DekuMask".to_string())),
            "DekuMask should be escaped with correct value"
        );
        assert_eq!(
            escaped.get("ForestMedallion"),
            Some(&OotrItem("ForestMedallion".to_string())),
            "ForestMedallion should be escaped with correct value"
        );
        assert_eq!(
            escaped.get("OdolwaRemains"),
            Some(&OotrItem("OdolwaRemains".to_string())),
            "OdolwaRemains should be escaped with correct value"
        );
    }

    #[test]
    fn test_ootmm_rando_regions_loaded() {
        let rando = OotmmRando::new().unwrap();
        let regions = rando.regions().unwrap();

        // Should have regions from embedded data
        assert!(!regions.is_empty(), "Regions should be loaded");
    }

    #[test]
    fn test_ootmm_rando_root_region() {
        let root = OotmmRando::root();
        assert_eq!(root.as_ref(), "Root");
    }

    #[test]
    fn test_ootmm_rando_setting_infos() {
        let rando = OotmmRando::new().unwrap();
        let settings = rando.setting_infos().unwrap();

        // Should have expected number of settings
        assert!(
            settings.len() >= 26,
            "Should have at least 26 settings, found {}",
            settings.len()
        );

        // Verify specific setting values exist
        let expected_settings = [
            "open_door_of_time",
            "open_kakariko",
            "open_gerudo_fortress",
            "shuffle_songs",
            "shuffle_smallkeys",
            "shuffle_bosskeys",
            "bombchus_in_logic",
            "logic_rules",
            "starting_age",
            "damage_multiplier",
        ];

        for setting in expected_settings {
            assert!(
                settings.contains(setting),
                "Should have '{}' setting",
                setting
            );
        }
    }

    #[test]
    fn test_ootmm_rando_logic_tricks_default_empty() {
        let rando = OotmmRando::new().unwrap();
        let tricks = rando.logic_tricks().unwrap();

        // By default no tricks are enabled
        assert!(tricks.is_empty(), "Default tricks should be empty");
    }

    #[test]
    fn test_ootmm_rando_enable_trick() {
        let mut rando = OotmmRando::new().unwrap();

        rando.enable_trick("lens_skip");
        rando.enable_trick("hover_boots_trick");

        let tricks = rando.logic_tricks().unwrap();
        assert_eq!(tricks.len(), 2, "Should have exactly 2 tricks enabled");

        // Verify exact trick values
        let mut expected: HashSet<String> = HashSet::new();
        expected.insert("lens_skip".to_string());
        expected.insert("hover_boots_trick".to_string());
        assert_eq!(*tricks, expected, "Tricks should match expected values");
    }

    #[test]
    fn test_ootmm_rando_set_logic_tricks() {
        let mut rando = OotmmRando::new().unwrap();

        let mut tricks = HashSet::new();
        tricks.insert("trick_a".to_string());
        tricks.insert("trick_b".to_string());
        tricks.insert("trick_c".to_string());

        rando.set_logic_tricks(tricks.clone());

        let result = rando.logic_tricks().unwrap();
        assert_eq!(result.len(), 3, "Should have exactly 3 tricks");

        // Verify exact trick values match what was set
        assert_eq!(*result, tricks, "Tricks should match the input set exactly");
    }

    #[test]
    fn test_ootmm_region_name_equality() {
        let name1 = OotmmRegionName::new("Kokiri Forest");
        let name2 = OotmmRegionName::new("Kokiri Forest");
        let name3 = OotmmRegionName::new("Lost Woods");

        assert_eq!(name1, name2);
        assert_ne!(name1, name3);
        assert!(name1 == "Kokiri Forest");
        assert!(!(name1 == "Lost Woods"));
    }

    #[test]
    fn test_ootmm_region_name_from_static_str() {
        let name: OotmmRegionName = "Test Region".into();
        assert_eq!(name.as_ref(), "Test Region");
        assert_eq!(format!("{}", name), "Test Region");
    }

    #[test]
    fn test_ootmm_rando_world_database_access() {
        let rando = OotmmRando::new().unwrap();
        let db = rando.world_database();

        // Should have access to the world database
        assert!(db.region_count() > 0, "World database should have regions");
    }
}

// ============================================================================
// Additional Integration Tests
// ============================================================================

mod integration_tests {
    use ootmm::expr::parse;
    use ootmm::item::{Item, MmItem, OotItem};
    use ootmm::items::ItemName;

    // Test the full flow: parse expression -> items referenced
    #[test]
    fn test_expression_item_references() {
        let expr = parse("has(Hookshot) && has(DekuMask)").unwrap();
        let displayed = expr.to_string();
        // Should be able to re-parse the displayed form
        let reparsed = parse(&displayed).unwrap();
        assert_eq!(expr, reparsed);
    }

    #[test]
    fn test_item_name_integration() {
        // ItemName should work seamlessly with Item lookups
        let names = ["MasterSword", "DekuMask", "OathToOrder"];

        for name_str in names {
            let name = ItemName::new(name_str);
            assert!(name.is_valid(), "'{}' should be valid", name_str);

            let item: Item = name.try_into().unwrap();
            let back_name = ItemName::from(item);
            assert_eq!(back_name.as_str(), name_str);
        }
    }

    #[test]
    fn test_item_max_count_sanity() {
        // Verify max counts are sensible
        assert!(OotItem::GoldSkulltula.max_count() == 100);
        assert!(OotItem::PieceOfHeart.max_count() == 36);
        assert!(OotItem::SmallKeyFireTemple.max_count() == 8);
        assert!(MmItem::StrayFairyWoodfall.max_count() == 15);
        assert!(MmItem::PieceOfHeart.max_count() == 52);

        // Non-stackable items should have max 1
        assert!(OotItem::MasterSword.max_count() == 1);
        assert!(MmItem::FierceDeityMask.max_count() == 1);
    }

    #[test]
    fn test_item_stackability() {
        // Verify stackability
        assert!(OotItem::SmallKeyFireTemple.is_stackable());
        assert!(OotItem::GoldSkulltula.is_stackable());
        assert!(MmItem::StrayFairyWoodfall.is_stackable());

        assert!(!OotItem::MasterSword.is_stackable());
        assert!(!MmItem::Hookshot.is_stackable());
    }
}
