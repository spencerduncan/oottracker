//! Tests for MM save data structures.

#[cfg(test)]
mod tests {
    use crate::mm_save::{
        constants::{mm_item_ids, MM_PERM_SCENE_COUNT, MM_SIZE},
        dungeon_progress::{MmAllDungeonItems, MmDungeonItems, MmSmallKeys, MmStrayFairies},
        inventory::MmBottle,
        masks::{MmMasks, MmMasksHigh, MmMasksLow, MmTransformationMasks},
        offsets::{ootmm_offsets, vanilla_offsets},
        quest_items::MmQuestItems,
        reader::MmSaveReader,
        save::MmSave,
        stub::MmSaveStub,
        traits::{MmGameMode, MmSaveData},
        types::{MmDecodeError, MmMagicCapacity, MmShield, MmSword, PlayerForm},
        upgrades::MmUpgrades,
    };

    #[test]
    fn test_player_form_conversion() {
        assert_eq!(PlayerForm::try_from(0), Ok(PlayerForm::FierceDeity));
        assert_eq!(PlayerForm::try_from(1), Ok(PlayerForm::Goron));
        assert_eq!(PlayerForm::try_from(2), Ok(PlayerForm::Zora));
        assert_eq!(PlayerForm::try_from(3), Ok(PlayerForm::Deku));
        assert_eq!(PlayerForm::try_from(4), Ok(PlayerForm::Human));
        assert_eq!(PlayerForm::try_from(5), Err(5));
    }

    #[test]
    fn test_quest_items_remains_count() {
        let mut quest = MmQuestItems::empty();
        assert_eq!(quest.num_remains(), 0);

        quest.insert(MmQuestItems::REMAINS_ODOLWA);
        assert_eq!(quest.num_remains(), 1);

        quest.insert(MmQuestItems::REMAINS_GOHT);
        quest.insert(MmQuestItems::REMAINS_GYORG);
        assert_eq!(quest.num_remains(), 3);

        quest.insert(MmQuestItems::REMAINS_TWINMOLD);
        assert_eq!(quest.num_remains(), 4);
    }

    #[test]
    fn test_mask_count() {
        let mut masks = MmMasks::default();
        assert_eq!(masks.total_mask_count(), 0);

        masks.transformation = MmTransformationMasks::DEKU | MmTransformationMasks::GORON;
        assert_eq!(masks.total_mask_count(), 2);

        masks.masks_low = MmMasksLow::BUNNY | MmMasksLow::STONE | MmMasksLow::KEATON;
        assert_eq!(masks.regular_mask_count(), 3);
        assert_eq!(masks.total_mask_count(), 5);
    }

    #[test]
    fn test_stray_fairy_dungeon_total() {
        let mut fairies = MmStrayFairies::default();
        assert_eq!(fairies.dungeon_total(), 0);

        fairies.woodfall = 15;
        fairies.snowhead = 10;
        fairies.great_bay = 5;
        fairies.stone_tower = 0;

        assert_eq!(fairies.dungeon_total(), 30);
    }

    #[test]
    fn test_stub_default() {
        let stub = MmSaveStub::new();
        assert_eq!(stub.game_mode(), MmGameMode::Gameplay);
        assert!(!stub.get_save().inventory.ocarina);
        assert_eq!(
            stub.get_save().masks.transformation,
            MmTransformationMasks::empty()
        );
    }

    #[test]
    fn test_stub_sample_data() {
        let stub = MmSaveStub::with_sample_data();
        let save = stub.get_save();

        // Check sample inventory
        assert!(save.inventory.ocarina);
        assert!(save.inventory.bow);
        assert!(save.inventory.hookshot);

        // Check sample masks
        assert!(save
            .masks
            .transformation
            .contains(MmTransformationMasks::DEKU));
        assert!(save
            .masks
            .transformation
            .contains(MmTransformationMasks::GORON));
        assert!(save
            .masks
            .transformation
            .contains(MmTransformationMasks::ZORA));
        assert!(!save
            .masks
            .transformation
            .contains(MmTransformationMasks::FIERCE_DEITY));

        // Check remains
        assert!(save.quest_items.contains(MmQuestItems::REMAINS_ODOLWA));
        assert!(save.quest_items.contains(MmQuestItems::REMAINS_GOHT));
        assert!(!save.quest_items.contains(MmQuestItems::REMAINS_GYORG));
        assert_eq!(save.quest_items.num_remains(), 2);

        // Check songs
        assert!(save.quest_items.contains(MmQuestItems::SONG_HEALING));
        assert!(save.quest_items.contains(MmQuestItems::SONG_TIME));
        assert!(save.quest_items.contains(MmQuestItems::SONG_SOARING));

        // Check stray fairies
        assert_eq!(save.stray_fairies.woodfall, 15);
        assert!(stub.dungeon_fairies_complete(0)); // Woodfall
        assert!(!stub.dungeon_fairies_complete(1)); // Snowhead (8/15)
    }

    #[test]
    fn test_trait_methods() {
        let stub = MmSaveStub::with_sample_data();

        assert!(stub.has_transformation_mask(MmTransformationMasks::GORON));
        assert!(!stub.has_transformation_mask(MmTransformationMasks::FIERCE_DEITY));

        assert!(stub.has_remains(MmQuestItems::REMAINS_ODOLWA));
        assert!(!stub.has_remains(MmQuestItems::REMAINS_TWINMOLD));

        assert_eq!(stub.stray_fairy_count(0), 15);
        assert_eq!(stub.stray_fairy_count(1), 8);
        assert_eq!(stub.stray_fairy_count(2), 3);
        assert_eq!(stub.stray_fairy_count(3), 0);
    }

    #[test]
    fn test_upgrades() {
        let upgrades = MmUpgrades::ADULTS_WALLET | MmUpgrades::BOMB_BAG_30;

        assert_eq!(upgrades.wallet(), MmUpgrades::ADULTS_WALLET);
        assert_eq!(upgrades.bomb_bag(), MmUpgrades::BOMB_BAG_30);
        assert_eq!(upgrades.quiver(), MmUpgrades::empty());
    }

    #[test]
    fn test_game_mode_conversion() {
        assert_eq!(MmGameMode::try_from(0u32), Ok(MmGameMode::Gameplay));
        assert_eq!(MmGameMode::try_from(1u32), Ok(MmGameMode::TitleScreen));
        assert_eq!(MmGameMode::try_from(2u32), Ok(MmGameMode::FileSelect));
        assert_eq!(MmGameMode::try_from(4u32), Ok(MmGameMode::OwlSave));
        assert_eq!(MmGameMode::try_from(99u32), Err(99));
    }

    // ========================================================================
    // Real Parsing Tests
    // ========================================================================

    #[test]
    fn test_bottle_conversion() {
        use mm_item_ids::*;

        // Test None -> None
        assert_eq!(MmBottle::try_from(NONE), Ok(MmBottle::None));

        // Test various bottles
        assert_eq!(MmBottle::try_from(BOTTLE_EMPTY), Ok(MmBottle::Empty));
        assert_eq!(
            MmBottle::try_from(BOTTLE_RED_POTION),
            Ok(MmBottle::RedPotion)
        );
        assert_eq!(MmBottle::try_from(BOTTLE_FAIRY), Ok(MmBottle::Fairy));
        assert_eq!(
            MmBottle::try_from(BOTTLE_DEKU_PRINCESS),
            Ok(MmBottle::DekuPrincess)
        );
        assert_eq!(
            MmBottle::try_from(BOTTLE_CHATEAU_ROMANI),
            Ok(MmBottle::ChateauRomani)
        );

        // Test invalid value
        assert!(MmBottle::try_from(0xAB).is_err());

        // Test round-trip conversion
        let bottle = MmBottle::ChateauRomani;
        let raw: u8 = bottle.into();
        assert_eq!(MmBottle::try_from(raw), Ok(bottle));
    }

    #[test]
    fn test_from_save_data_size_validation() {
        // Too small
        let small_data = vec![0u8; 100];
        assert!(matches!(
            MmSave::from_save_data(&small_data),
            Err(MmDecodeError::Size(100))
        ));

        // Correct size should work
        let correct_data = vec![0u8; MM_SIZE];
        assert!(MmSave::from_save_data(&correct_data).is_ok());
    }

    #[test]
    fn test_from_save_data_parses_health() {
        use vanilla_offsets::*;

        let mut data = vec![0u8; MM_SIZE];

        // Set health capacity to 0x0140 (5 hearts = 80 in decimal)
        data[HEALTH_CAPACITY] = 0x01;
        data[HEALTH_CAPACITY + 1] = 0x40;

        // Set current health to 0x0100 (4 hearts)
        data[HEALTH] = 0x01;
        data[HEALTH + 1] = 0x00;

        let save = MmSave::from_save_data(&data).unwrap();
        assert_eq!(save.health_capacity, 0x0140);
        assert_eq!(save.health, 0x0100);
    }

    #[test]
    fn test_from_save_data_parses_rupees() {
        use vanilla_offsets::*;

        let mut data = vec![0u8; MM_SIZE];

        // Set rupees to 500 (0x01F4)
        data[RUPEES] = 0x01;
        data[RUPEES + 1] = 0xF4;

        let save = MmSave::from_save_data(&data).unwrap();
        assert_eq!(save.rupees, 500);
    }

    #[test]
    fn test_from_save_data_parses_sword_shield() {
        use vanilla_offsets::*;

        let mut data = vec![0u8; MM_SIZE];

        // Set Gilded Sword (3) and Mirror Shield (3)
        // Shield is in high nibble, sword in low nibble
        // Note: MirrorShield is value 3 (HylianShield is 2 for OoTMM support)
        data[SWORD_SHIELD] = 0x03 | (0x03 << 4);

        let save = MmSave::from_save_data(&data).unwrap();
        assert_eq!(save.sword, MmSword::GildedSword);
        assert_eq!(save.shield, MmShield::MirrorShield);
    }

    #[test]
    fn test_from_save_data_parses_quest_items() {
        use vanilla_offsets::*;

        let mut data = vec![0u8; MM_SIZE];

        // Set boss remains: Odolwa (bit 0) and Goht (bit 1)
        // And Song of Time (bit 12)
        let quest_bits: u32 = 0x00001003; // REMAINS_ODOLWA | REMAINS_GOHT | SONG_TIME
        data[QUEST_ITEMS..QUEST_ITEMS + 4].copy_from_slice(&quest_bits.to_be_bytes());

        let save = MmSave::from_save_data(&data).unwrap();
        assert!(save.quest_items.contains(MmQuestItems::REMAINS_ODOLWA));
        assert!(save.quest_items.contains(MmQuestItems::REMAINS_GOHT));
        assert!(!save.quest_items.contains(MmQuestItems::REMAINS_GYORG));
        assert!(save.quest_items.contains(MmQuestItems::SONG_TIME));
        assert_eq!(save.quest_items.num_remains(), 2);
    }

    #[test]
    fn test_from_save_data_parses_stray_fairies() {
        use vanilla_offsets::*;

        let mut data = vec![0u8; MM_SIZE];

        data[STRAY_FAIRIES] = 1; // Clock Town
        data[STRAY_FAIRIES + 1] = 15; // Woodfall
        data[STRAY_FAIRIES + 2] = 10; // Snowhead
        data[STRAY_FAIRIES + 3] = 5; // Great Bay
        data[STRAY_FAIRIES + 4] = 0; // Stone Tower

        let save = MmSave::from_save_data(&data).unwrap();
        assert_eq!(save.stray_fairies.clock_town, 1);
        assert_eq!(save.stray_fairies.woodfall, 15);
        assert_eq!(save.stray_fairies.snowhead, 10);
        assert_eq!(save.stray_fairies.great_bay, 5);
        assert_eq!(save.stray_fairies.stone_tower, 0);
        assert_eq!(save.stray_fairies.dungeon_total(), 30);
    }

    #[test]
    fn test_from_save_data_parses_dungeon_items() {
        use vanilla_offsets::*;

        let mut data = vec![0u8; MM_SIZE];

        // N64 big-endian bitfield layout:
        // bit 7: bossKey (0x80), bit 6: compass (0x40), bit 5: map (0x20)
        // Woodfall: Map + Compass + Boss Key (0xE0)
        data[DUNGEON_ITEMS] = 0xE0;
        // Snowhead: Map only (0x20)
        data[DUNGEON_ITEMS + 1] = 0x20;
        // Great Bay: Compass only (0x40)
        data[DUNGEON_ITEMS + 2] = 0x40;
        // Stone Tower: Boss Key only (0x80)
        data[DUNGEON_ITEMS + 3] = 0x80;

        let save = MmSave::from_save_data(&data).unwrap();
        assert!(save.dungeon_items.woodfall.contains(MmDungeonItems::MAP));
        assert!(save
            .dungeon_items
            .woodfall
            .contains(MmDungeonItems::COMPASS));
        assert!(save
            .dungeon_items
            .woodfall
            .contains(MmDungeonItems::BOSS_KEY));
        assert!(save.dungeon_items.snowhead.contains(MmDungeonItems::MAP));
        assert!(!save
            .dungeon_items
            .snowhead
            .contains(MmDungeonItems::BOSS_KEY));
        assert!(save
            .dungeon_items
            .great_bay
            .contains(MmDungeonItems::COMPASS));
        assert!(save
            .dungeon_items
            .stone_tower
            .contains(MmDungeonItems::BOSS_KEY));
    }

    #[test]
    fn test_from_save_data_parses_skulltulas() {
        use vanilla_offsets::*;

        let mut data = vec![0u8; MM_SIZE];

        // Set swamp skulltulas to 30 (0x001E)
        data[SKULL_SWAMP] = 0x00;
        data[SKULL_SWAMP + 1] = 0x1E;

        // Set ocean skulltulas to 25 (0x0019)
        data[SKULL_OCEAN] = 0x00;
        data[SKULL_OCEAN + 1] = 0x19;

        let save = MmSave::from_save_data(&data).unwrap();
        assert_eq!(save.skull_tokens_swamp, 30);
        assert_eq!(save.skull_tokens_ocean, 25);
    }

    #[test]
    fn test_from_save_data_parses_time() {
        use vanilla_offsets::*;

        let mut data = vec![0u8; MM_SIZE];

        // Day 2
        data[DAY..DAY + 4].copy_from_slice(&2u32.to_be_bytes());

        // Time = 0x8000 (noon-ish)
        data[TIME..TIME + 2].copy_from_slice(&0x8000u16.to_be_bytes());

        // Night = 1
        data[IS_NIGHT] = 1;

        let save = MmSave::from_save_data(&data).unwrap();
        assert_eq!(save.day, 2);
        assert_eq!(save.time, 0x8000);
        assert!(save.is_night);
    }

    #[test]
    fn test_from_save_data_parses_permanent_scene_flags() {
        use vanilla_offsets::*;

        let mut data = vec![0u8; MM_SIZE];

        // Set first scene's chest flags
        let chest_flags: u32 = 0x12345678;
        data[PERM_SCENE_FLAGS..PERM_SCENE_FLAGS + 4].copy_from_slice(&chest_flags.to_be_bytes());

        let save = MmSave::from_save_data(&data).unwrap();
        assert_eq!(save.permanent_scene_flags.len(), MM_PERM_SCENE_COUNT);
        assert_eq!(save.permanent_scene_flags[0].chest, 0x12345678);
    }

    #[test]
    fn test_roundtrip_save_data() {
        // Create a save with some data
        let original = MmSave {
            player_form: PlayerForm::Goron,
            health_capacity: 0x0140,
            health: 0x0100,
            magic: MmMagicCapacity::Double,
            double_defense: true,
            rupees: 500,
            sword: MmSword::GildedSword,
            shield: MmShield::MirrorShield,
            quest_items: MmQuestItems::REMAINS_ODOLWA | MmQuestItems::SONG_TIME,
            upgrades: MmUpgrades::ADULTS_WALLET,
            dungeon_items: MmAllDungeonItems {
                woodfall: MmDungeonItems::MAP | MmDungeonItems::COMPASS,
                ..Default::default()
            },
            small_keys: MmSmallKeys {
                woodfall: 2,
                ..Default::default()
            },
            stray_fairies: MmStrayFairies {
                woodfall: 15,
                ..Default::default()
            },
            skull_tokens_swamp: 20,
            skull_tokens_ocean: 10,
            day: 2,
            time: 0x8000,
            is_night: true,
            ..Default::default()
        };

        // Serialize and deserialize
        let bytes = original.to_save_data();
        let parsed = MmSave::from_save_data(&bytes).unwrap();

        // Check key fields survived the roundtrip
        assert_eq!(parsed.player_form, original.player_form);
        assert_eq!(parsed.health_capacity, original.health_capacity);
        assert_eq!(parsed.health, original.health);
        assert_eq!(parsed.magic, original.magic);
        assert_eq!(parsed.double_defense, original.double_defense);
        assert_eq!(parsed.rupees, original.rupees);
        assert_eq!(parsed.sword, original.sword);
        assert_eq!(parsed.shield, original.shield);
        assert_eq!(parsed.quest_items, original.quest_items);
        assert_eq!(parsed.upgrades, original.upgrades);
        assert_eq!(parsed.dungeon_items, original.dungeon_items);
        assert_eq!(parsed.small_keys, original.small_keys);
        assert_eq!(parsed.stray_fairies, original.stray_fairies);
        assert_eq!(parsed.skull_tokens_swamp, original.skull_tokens_swamp);
        assert_eq!(parsed.skull_tokens_ocean, original.skull_tokens_ocean);
        assert_eq!(parsed.day, original.day);
        assert_eq!(parsed.time, original.time);
        assert_eq!(parsed.is_night, original.is_night);
    }

    #[test]
    fn test_mm_save_reader() {
        let data = vec![0u8; MM_SIZE];
        let reader = MmSaveReader::from_bytes(&data).unwrap();

        assert_eq!(reader.game_mode(), MmGameMode::Gameplay);
        assert_eq!(reader.get_save().health, 0);
    }

    #[test]
    fn test_mm_save_reader_update() {
        use ootmm_offsets::*;

        let data = vec![0u8; MM_SIZE];
        let mut reader = MmSaveReader::from_bytes(&data).unwrap();

        // Update with new data containing different rupees (at OoTMM offset 0x3A)
        let mut new_data = vec![0u8; MM_SIZE];
        new_data[RUPEES..RUPEES + 2].copy_from_slice(&200u16.to_be_bytes());

        reader.update(&new_data).unwrap();
        assert_eq!(reader.get_save().rupees, 200);
    }

    #[test]
    fn test_decode_error_display() {
        let err = MmDecodeError::Size(100);
        assert!(matches!(err, MmDecodeError::Size(100)));

        let err = MmDecodeError::Index(42);
        assert!(matches!(err, MmDecodeError::Index(42)));
    }

    // ========================================================================
    // Mask Accessor Tests
    // ========================================================================

    #[test]
    fn test_transformation_mask_accessors_default() {
        let save = MmSave::default();

        // All transformation masks should be false by default
        assert!(!save.has_deku_mask());
        assert!(!save.has_goron_mask());
        assert!(!save.has_zora_mask());
        assert!(!save.has_fierce_deity_mask());
    }

    #[test]
    fn test_has_deku_mask() {
        let mut save = MmSave::default();
        assert!(!save.has_deku_mask());

        save.masks
            .transformation
            .insert(MmTransformationMasks::DEKU);
        assert!(save.has_deku_mask());

        // Other masks should remain unaffected
        assert!(!save.has_goron_mask());
        assert!(!save.has_zora_mask());
        assert!(!save.has_fierce_deity_mask());
    }

    #[test]
    fn test_has_goron_mask() {
        let mut save = MmSave::default();
        assert!(!save.has_goron_mask());

        save.masks
            .transformation
            .insert(MmTransformationMasks::GORON);
        assert!(save.has_goron_mask());

        // Other masks should remain unaffected
        assert!(!save.has_deku_mask());
        assert!(!save.has_zora_mask());
        assert!(!save.has_fierce_deity_mask());
    }

    #[test]
    fn test_has_zora_mask() {
        let mut save = MmSave::default();
        assert!(!save.has_zora_mask());

        save.masks
            .transformation
            .insert(MmTransformationMasks::ZORA);
        assert!(save.has_zora_mask());

        // Other masks should remain unaffected
        assert!(!save.has_deku_mask());
        assert!(!save.has_goron_mask());
        assert!(!save.has_fierce_deity_mask());
    }

    #[test]
    fn test_has_fierce_deity_mask() {
        let mut save = MmSave::default();
        assert!(!save.has_fierce_deity_mask());

        save.masks
            .transformation
            .insert(MmTransformationMasks::FIERCE_DEITY);
        assert!(save.has_fierce_deity_mask());

        // Other masks should remain unaffected
        assert!(!save.has_deku_mask());
        assert!(!save.has_goron_mask());
        assert!(!save.has_zora_mask());
    }

    #[test]
    fn test_all_transformation_masks() {
        let mut save = MmSave::default();

        // Set all transformation masks
        save.masks.transformation = MmTransformationMasks::DEKU
            | MmTransformationMasks::GORON
            | MmTransformationMasks::ZORA
            | MmTransformationMasks::FIERCE_DEITY;

        assert!(save.has_deku_mask());
        assert!(save.has_goron_mask());
        assert!(save.has_zora_mask());
        assert!(save.has_fierce_deity_mask());
    }

    #[test]
    fn test_collectible_mask_accessors_default() {
        let save = MmSave::default();

        // All collectible masks should be false by default
        assert!(!save.has_postman_hat());
        assert!(!save.has_all_night_mask());
        assert!(!save.has_blast_mask());
        assert!(!save.has_stone_mask());
        assert!(!save.has_great_fairy_mask());
        assert!(!save.has_keaton_mask());
        assert!(!save.has_bremen_mask());
        assert!(!save.has_bunny_hood());
        assert!(!save.has_don_gero_mask());
        assert!(!save.has_mask_of_scents());
        assert!(!save.has_romani_mask());
        assert!(!save.has_circus_leader_mask());
        assert!(!save.has_kafei_mask());
        assert!(!save.has_couples_mask());
        assert!(!save.has_mask_of_truth());
        assert!(!save.has_kamaro_mask());
        assert!(!save.has_gibdo_mask());
        assert!(!save.has_garo_mask());
        assert!(!save.has_captain_hat());
        assert!(!save.has_giant_mask());
    }

    #[test]
    fn test_has_postman_hat() {
        let mut save = MmSave::default();
        assert!(!save.has_postman_hat());

        save.masks.masks_low.insert(MmMasksLow::POSTMAN);
        assert!(save.has_postman_hat());
    }

    #[test]
    fn test_has_all_night_mask() {
        let mut save = MmSave::default();
        assert!(!save.has_all_night_mask());

        save.masks.masks_low.insert(MmMasksLow::ALL_NIGHT);
        assert!(save.has_all_night_mask());
    }

    #[test]
    fn test_has_blast_mask() {
        let mut save = MmSave::default();
        assert!(!save.has_blast_mask());

        save.masks.masks_low.insert(MmMasksLow::BLAST);
        assert!(save.has_blast_mask());
    }

    #[test]
    fn test_has_stone_mask() {
        let mut save = MmSave::default();
        assert!(!save.has_stone_mask());

        save.masks.masks_low.insert(MmMasksLow::STONE);
        assert!(save.has_stone_mask());
    }

    #[test]
    fn test_has_great_fairy_mask() {
        let mut save = MmSave::default();
        assert!(!save.has_great_fairy_mask());

        save.masks.masks_low.insert(MmMasksLow::GREAT_FAIRY);
        assert!(save.has_great_fairy_mask());
    }

    #[test]
    fn test_has_keaton_mask() {
        let mut save = MmSave::default();
        assert!(!save.has_keaton_mask());

        save.masks.masks_low.insert(MmMasksLow::KEATON);
        assert!(save.has_keaton_mask());
    }

    #[test]
    fn test_has_bremen_mask() {
        let mut save = MmSave::default();
        assert!(!save.has_bremen_mask());

        save.masks.masks_low.insert(MmMasksLow::BREMEN);
        assert!(save.has_bremen_mask());
    }

    #[test]
    fn test_has_bunny_hood() {
        let mut save = MmSave::default();
        assert!(!save.has_bunny_hood());

        save.masks.masks_low.insert(MmMasksLow::BUNNY);
        assert!(save.has_bunny_hood());
    }

    #[test]
    fn test_has_don_gero_mask() {
        let mut save = MmSave::default();
        assert!(!save.has_don_gero_mask());

        save.masks.masks_low.insert(MmMasksLow::DON_GERO);
        assert!(save.has_don_gero_mask());
    }

    #[test]
    fn test_has_mask_of_scents() {
        let mut save = MmSave::default();
        assert!(!save.has_mask_of_scents());

        save.masks.masks_low.insert(MmMasksLow::SCENTS);
        assert!(save.has_mask_of_scents());
    }

    #[test]
    fn test_has_romani_mask() {
        let mut save = MmSave::default();
        assert!(!save.has_romani_mask());

        save.masks.masks_low.insert(MmMasksLow::ROMANI);
        assert!(save.has_romani_mask());
    }

    #[test]
    fn test_has_circus_leader_mask() {
        let mut save = MmSave::default();
        assert!(!save.has_circus_leader_mask());

        save.masks.masks_low.insert(MmMasksLow::CIRCUS_LEADER);
        assert!(save.has_circus_leader_mask());
    }

    #[test]
    fn test_has_kafei_mask() {
        let mut save = MmSave::default();
        assert!(!save.has_kafei_mask());

        save.masks.masks_low.insert(MmMasksLow::KAFEI);
        assert!(save.has_kafei_mask());
    }

    #[test]
    fn test_has_couples_mask() {
        let mut save = MmSave::default();
        assert!(!save.has_couples_mask());

        save.masks.masks_low.insert(MmMasksLow::COUPLES);
        assert!(save.has_couples_mask());
    }

    #[test]
    fn test_has_mask_of_truth() {
        let mut save = MmSave::default();
        assert!(!save.has_mask_of_truth());

        save.masks.masks_low.insert(MmMasksLow::TRUTH);
        assert!(save.has_mask_of_truth());
    }

    #[test]
    fn test_has_kamaro_mask() {
        let mut save = MmSave::default();
        assert!(!save.has_kamaro_mask());

        save.masks.masks_low.insert(MmMasksLow::KAMARO);
        assert!(save.has_kamaro_mask());
    }

    #[test]
    fn test_has_gibdo_mask() {
        let mut save = MmSave::default();
        assert!(!save.has_gibdo_mask());

        save.masks.masks_high.insert(MmMasksHigh::GIBDO);
        assert!(save.has_gibdo_mask());
    }

    #[test]
    fn test_has_garo_mask() {
        let mut save = MmSave::default();
        assert!(!save.has_garo_mask());

        save.masks.masks_high.insert(MmMasksHigh::GARO);
        assert!(save.has_garo_mask());
    }

    #[test]
    fn test_has_captain_hat() {
        let mut save = MmSave::default();
        assert!(!save.has_captain_hat());

        save.masks.masks_high.insert(MmMasksHigh::CAPTAIN);
        assert!(save.has_captain_hat());
    }

    #[test]
    fn test_has_giant_mask() {
        let mut save = MmSave::default();
        assert!(!save.has_giant_mask());

        save.masks.masks_high.insert(MmMasksHigh::GIANT);
        assert!(save.has_giant_mask());
    }

    #[test]
    fn test_all_masks_low() {
        let mut save = MmSave::default();

        // Set all masks_low flags
        save.masks.masks_low = MmMasksLow::POSTMAN
            | MmMasksLow::ALL_NIGHT
            | MmMasksLow::BLAST
            | MmMasksLow::STONE
            | MmMasksLow::GREAT_FAIRY
            | MmMasksLow::KEATON
            | MmMasksLow::BREMEN
            | MmMasksLow::BUNNY
            | MmMasksLow::DON_GERO
            | MmMasksLow::SCENTS
            | MmMasksLow::ROMANI
            | MmMasksLow::CIRCUS_LEADER
            | MmMasksLow::KAFEI
            | MmMasksLow::COUPLES
            | MmMasksLow::TRUTH
            | MmMasksLow::KAMARO;

        assert!(save.has_postman_hat());
        assert!(save.has_all_night_mask());
        assert!(save.has_blast_mask());
        assert!(save.has_stone_mask());
        assert!(save.has_great_fairy_mask());
        assert!(save.has_keaton_mask());
        assert!(save.has_bremen_mask());
        assert!(save.has_bunny_hood());
        assert!(save.has_don_gero_mask());
        assert!(save.has_mask_of_scents());
        assert!(save.has_romani_mask());
        assert!(save.has_circus_leader_mask());
        assert!(save.has_kafei_mask());
        assert!(save.has_couples_mask());
        assert!(save.has_mask_of_truth());
        assert!(save.has_kamaro_mask());
    }

    #[test]
    fn test_all_masks_high() {
        let mut save = MmSave::default();

        // Set all masks_high flags
        save.masks.masks_high =
            MmMasksHigh::GIBDO | MmMasksHigh::GARO | MmMasksHigh::CAPTAIN | MmMasksHigh::GIANT;

        assert!(save.has_gibdo_mask());
        assert!(save.has_garo_mask());
        assert!(save.has_captain_hat());
        assert!(save.has_giant_mask());
    }

    #[test]
    fn test_mask_accessors_with_sample_data() {
        let stub = MmSaveStub::with_sample_data();
        let save = stub.get_save();

        // Sample data has: DEKU, GORON, ZORA (transformation)
        assert!(save.has_deku_mask());
        assert!(save.has_goron_mask());
        assert!(save.has_zora_mask());
        assert!(!save.has_fierce_deity_mask());

        // Sample data has: BUNNY, STONE, GREAT_FAIRY, BREMEN (collectible)
        assert!(save.has_bunny_hood());
        assert!(save.has_stone_mask());
        assert!(save.has_great_fairy_mask());
        assert!(save.has_bremen_mask());

        // These should not be set in sample data
        assert!(!save.has_postman_hat());
        assert!(!save.has_all_night_mask());
        assert!(!save.has_blast_mask());
        assert!(!save.has_keaton_mask());
        assert!(!save.has_don_gero_mask());
        assert!(!save.has_mask_of_scents());
        assert!(!save.has_romani_mask());
        assert!(!save.has_circus_leader_mask());
        assert!(!save.has_kafei_mask());
        assert!(!save.has_couples_mask());
        assert!(!save.has_mask_of_truth());
        assert!(!save.has_kamaro_mask());
        assert!(!save.has_gibdo_mask());
        assert!(!save.has_garo_mask());
        assert!(!save.has_captain_hat());
        assert!(!save.has_giant_mask());
    }

    // ========================================================================
    // Inventory Accessor Tests
    // ========================================================================

    #[test]
    fn test_equipment_accessors_default() {
        let save = MmSave::default();

        // All equipment should be false by default
        assert!(!save.has_ocarina());
        assert!(!save.has_heros_bow());
        assert!(!save.has_fire_arrow());
        assert!(!save.has_ice_arrow());
        assert!(!save.has_light_arrow());
        assert!(!save.has_hookshot());
        assert!(!save.has_bombs());
        assert!(!save.has_bombchu());
        assert!(!save.has_powder_keg());
        assert!(!save.has_lens_of_truth());
        assert!(!save.has_pictograph_box());
        assert!(!save.has_great_fairy_sword());
        assert!(!save.has_magic_bean());
        assert!(!save.has_magic());
    }

    #[test]
    fn test_equipment_accessors_with_items() {
        let mut save = MmSave::default();

        // Set some equipment
        save.inventory.ocarina = true;
        save.inventory.bow = true;
        save.inventory.fire_arrows = true;
        save.inventory.hookshot = true;
        save.inventory.bombs = true;
        save.inventory.lens = true;
        save.inventory.great_fairy_sword = true;
        save.magic = MmMagicCapacity::Double;

        // Check accessors return true for items we have
        assert!(save.has_ocarina());
        assert!(save.has_heros_bow());
        assert!(save.has_fire_arrow());
        assert!(save.has_hookshot());
        assert!(save.has_bombs());
        assert!(save.has_lens_of_truth());
        assert!(save.has_great_fairy_sword());
        assert!(save.has_magic());

        // Check accessors return false for items we don't have
        assert!(!save.has_ice_arrow());
        assert!(!save.has_light_arrow());
        assert!(!save.has_bombchu());
        assert!(!save.has_powder_keg());
        assert!(!save.has_pictograph_box());
        assert!(!save.has_magic_bean());
    }

    #[test]
    fn test_magic_accessor_single_magic() {
        let save = MmSave {
            magic: MmMagicCapacity::Single,
            ..Default::default()
        };
        assert!(save.has_magic());
    }

    #[test]
    fn test_magic_accessor_double_magic() {
        let save = MmSave {
            magic: MmMagicCapacity::Double,
            ..Default::default()
        };
        assert!(save.has_magic());
    }

    #[test]
    fn test_song_accessors_default() {
        let save = MmSave::default();

        // All songs should be false by default
        assert!(!save.has_song_of_time());
        assert!(!save.has_song_of_healing());
        assert!(!save.has_eponas_song());
        assert!(!save.has_song_of_soaring());
        assert!(!save.has_song_of_storms());
        assert!(!save.has_sonata_of_awakening());
        assert!(!save.has_goron_lullaby());
        assert!(!save.has_new_wave_bossa_nova());
        assert!(!save.has_elegy_of_emptiness());
        assert!(!save.has_oath_to_order());
    }

    #[test]
    fn test_song_accessors_with_songs() {
        // Set some songs
        let save = MmSave {
            quest_items: MmQuestItems::SONG_TIME
                | MmQuestItems::SONG_HEALING
                | MmQuestItems::SONG_EPONA
                | MmQuestItems::SONG_SOARING
                | MmQuestItems::SONG_AWAKENING
                | MmQuestItems::SONG_ORDER,
            ..Default::default()
        };

        // Check accessors return true for songs we have
        assert!(save.has_song_of_time());
        assert!(save.has_song_of_healing());
        assert!(save.has_eponas_song());
        assert!(save.has_song_of_soaring());
        assert!(save.has_sonata_of_awakening());
        assert!(save.has_oath_to_order());

        // Check accessors return false for songs we don't have
        assert!(!save.has_song_of_storms());
        assert!(!save.has_goron_lullaby());
        assert!(!save.has_new_wave_bossa_nova());
        assert!(!save.has_elegy_of_emptiness());
    }

    #[test]
    fn test_song_accessors_all_songs() {
        // Set all songs
        let save = MmSave {
            quest_items: MmQuestItems::SONG_TIME
                | MmQuestItems::SONG_HEALING
                | MmQuestItems::SONG_EPONA
                | MmQuestItems::SONG_SOARING
                | MmQuestItems::SONG_STORMS
                | MmQuestItems::SONG_AWAKENING
                | MmQuestItems::SONG_GORON
                | MmQuestItems::SONG_ZORA
                | MmQuestItems::SONG_EMPTINESS
                | MmQuestItems::SONG_ORDER,
            ..Default::default()
        };

        // All song accessors should return true
        assert!(save.has_song_of_time());
        assert!(save.has_song_of_healing());
        assert!(save.has_eponas_song());
        assert!(save.has_song_of_soaring());
        assert!(save.has_song_of_storms());
        assert!(save.has_sonata_of_awakening());
        assert!(save.has_goron_lullaby());
        assert!(save.has_new_wave_bossa_nova());
        assert!(save.has_elegy_of_emptiness());
        assert!(save.has_oath_to_order());
    }

    #[test]
    fn test_boss_remains_accessors_default() {
        let save = MmSave::default();

        // All remains should be false by default
        assert!(!save.has_odolwa_remains());
        assert!(!save.has_goht_remains());
        assert!(!save.has_gyorg_remains());
        assert!(!save.has_twinmold_remains());
    }

    #[test]
    fn test_boss_remains_accessors_with_remains() {
        // Set some boss remains
        let save = MmSave {
            quest_items: MmQuestItems::REMAINS_ODOLWA | MmQuestItems::REMAINS_GOHT,
            ..Default::default()
        };

        // Check accessors return true for remains we have
        assert!(save.has_odolwa_remains());
        assert!(save.has_goht_remains());

        // Check accessors return false for remains we don't have
        assert!(!save.has_gyorg_remains());
        assert!(!save.has_twinmold_remains());
    }

    #[test]
    fn test_boss_remains_accessors_all_remains() {
        // Set all boss remains
        let save = MmSave {
            quest_items: MmQuestItems::REMAINS_ODOLWA
                | MmQuestItems::REMAINS_GOHT
                | MmQuestItems::REMAINS_GYORG
                | MmQuestItems::REMAINS_TWINMOLD,
            ..Default::default()
        };

        // All remains accessors should return true
        assert!(save.has_odolwa_remains());
        assert!(save.has_goht_remains());
        assert!(save.has_gyorg_remains());
        assert!(save.has_twinmold_remains());
    }

    #[test]
    fn test_accessors_with_sample_data() {
        let stub = MmSaveStub::with_sample_data();
        let save = stub.get_save();

        // Check equipment from sample data
        assert!(save.has_ocarina());
        assert!(save.has_heros_bow());
        assert!(save.has_hookshot());
        assert!(save.has_bombs());
        assert!(save.has_lens_of_truth());
        assert!(save.has_magic());

        // Check songs from sample data
        assert!(save.has_song_of_time());
        assert!(save.has_song_of_healing());
        assert!(save.has_eponas_song());
        assert!(save.has_song_of_soaring());
        assert!(save.has_sonata_of_awakening());

        // Check boss remains from sample data
        assert!(save.has_odolwa_remains());
        assert!(save.has_goht_remains());
        assert!(!save.has_gyorg_remains());
        assert!(!save.has_twinmold_remains());
    }

    #[test]
    fn test_equipment_accessors_from_parsed_data() {
        use vanilla_offsets::*;

        let mut data = vec![0u8; MM_SIZE];

        // Set inventory items at their slot positions
        // Ocarina is slot 0, Bow is slot 1, Hookshot is slot 15
        data[INVENTORY] = mm_item_ids::OCARINA;
        data[INVENTORY + 1] = mm_item_ids::BOW;
        data[INVENTORY + 2] = mm_item_ids::FIRE_ARROW;
        data[INVENTORY + 15] = mm_item_ids::HOOKSHOT;

        // Set magic level
        data[MAGIC_LEVEL] = 2; // Double magic

        let save = MmSave::from_save_data(&data).unwrap();

        assert!(save.has_ocarina());
        assert!(save.has_heros_bow());
        assert!(save.has_fire_arrow());
        assert!(save.has_hookshot());
        assert!(save.has_magic());

        // Items not set should be false
        assert!(!save.has_ice_arrow());
        assert!(!save.has_bombs());
    }

    #[test]
    fn test_song_accessors_from_parsed_data() {
        use vanilla_offsets::*;

        let mut data = vec![0u8; MM_SIZE];

        // Set quest items with Song of Time (bit 12) and Song of Healing (bit 13)
        let quest_bits: u32 = MmQuestItems::SONG_TIME.bits() | MmQuestItems::SONG_HEALING.bits();
        data[QUEST_ITEMS..QUEST_ITEMS + 4].copy_from_slice(&quest_bits.to_be_bytes());

        let save = MmSave::from_save_data(&data).unwrap();

        assert!(save.has_song_of_time());
        assert!(save.has_song_of_healing());
        assert!(!save.has_eponas_song());
        assert!(!save.has_song_of_soaring());
    }

    #[test]
    fn test_boss_remains_accessors_from_parsed_data() {
        use vanilla_offsets::*;

        let mut data = vec![0u8; MM_SIZE];

        // Set quest items with Odolwa and Gyorg remains
        let quest_bits: u32 =
            MmQuestItems::REMAINS_ODOLWA.bits() | MmQuestItems::REMAINS_GYORG.bits();
        data[QUEST_ITEMS..QUEST_ITEMS + 4].copy_from_slice(&quest_bits.to_be_bytes());

        let save = MmSave::from_save_data(&data).unwrap();

        assert!(save.has_odolwa_remains());
        assert!(!save.has_goht_remains());
        assert!(save.has_gyorg_remains());
        assert!(!save.has_twinmold_remains());
    }

    // ========================================================================
    // Mask Parsing from Raw Bytes Tests
    // ========================================================================

    #[test]
    fn test_parse_masks_transformation_masks_from_raw() {
        use vanilla_offsets::*;

        let mut data = vec![0u8; MM_SIZE];

        // Place transformation masks in mask inventory slots
        data[MASKS] = mm_item_ids::MASK_DEKU;
        data[MASKS + 1] = mm_item_ids::MASK_GORON;
        data[MASKS + 2] = mm_item_ids::MASK_ZORA;
        data[MASKS + 3] = mm_item_ids::MASK_FIERCE_DEITY;

        let save = MmSave::from_save_data(&data).unwrap();

        assert!(save
            .masks
            .transformation
            .contains(MmTransformationMasks::DEKU));
        assert!(save
            .masks
            .transformation
            .contains(MmTransformationMasks::GORON));
        assert!(save
            .masks
            .transformation
            .contains(MmTransformationMasks::ZORA));
        assert!(save
            .masks
            .transformation
            .contains(MmTransformationMasks::FIERCE_DEITY));
    }

    #[test]
    fn test_parse_masks_collectible_masks_low_from_raw() {
        use vanilla_offsets::*;

        let mut data = vec![0u8; MM_SIZE];

        // Place collectible masks (low bits) in mask inventory slots
        data[MASKS] = mm_item_ids::MASK_POSTMAN;
        data[MASKS + 1] = mm_item_ids::MASK_ALL_NIGHT;
        data[MASKS + 2] = mm_item_ids::MASK_BLAST;
        data[MASKS + 3] = mm_item_ids::MASK_STONE;
        data[MASKS + 4] = mm_item_ids::MASK_GREAT_FAIRY;
        data[MASKS + 5] = mm_item_ids::MASK_KEATON;
        data[MASKS + 6] = mm_item_ids::MASK_BREMEN;
        data[MASKS + 7] = mm_item_ids::MASK_BUNNY;
        data[MASKS + 8] = mm_item_ids::MASK_DON_GERO;
        data[MASKS + 9] = mm_item_ids::MASK_SCENTS;
        data[MASKS + 10] = mm_item_ids::MASK_ROMANI;
        data[MASKS + 11] = mm_item_ids::MASK_CIRCUS_LEADER;
        data[MASKS + 12] = mm_item_ids::MASK_KAFEI;
        data[MASKS + 13] = mm_item_ids::MASK_COUPLES;
        data[MASKS + 14] = mm_item_ids::MASK_TRUTH;
        data[MASKS + 15] = mm_item_ids::MASK_KAMARO;

        let save = MmSave::from_save_data(&data).unwrap();

        assert!(save.masks.masks_low.contains(MmMasksLow::POSTMAN));
        assert!(save.masks.masks_low.contains(MmMasksLow::ALL_NIGHT));
        assert!(save.masks.masks_low.contains(MmMasksLow::BLAST));
        assert!(save.masks.masks_low.contains(MmMasksLow::STONE));
        assert!(save.masks.masks_low.contains(MmMasksLow::GREAT_FAIRY));
        assert!(save.masks.masks_low.contains(MmMasksLow::KEATON));
        assert!(save.masks.masks_low.contains(MmMasksLow::BREMEN));
        assert!(save.masks.masks_low.contains(MmMasksLow::BUNNY));
        assert!(save.masks.masks_low.contains(MmMasksLow::DON_GERO));
        assert!(save.masks.masks_low.contains(MmMasksLow::SCENTS));
        assert!(save.masks.masks_low.contains(MmMasksLow::ROMANI));
        assert!(save.masks.masks_low.contains(MmMasksLow::CIRCUS_LEADER));
        assert!(save.masks.masks_low.contains(MmMasksLow::KAFEI));
        assert!(save.masks.masks_low.contains(MmMasksLow::COUPLES));
        assert!(save.masks.masks_low.contains(MmMasksLow::TRUTH));
        assert!(save.masks.masks_low.contains(MmMasksLow::KAMARO));
        assert_eq!(save.masks.masks_low.bits().count_ones(), 16);
    }

    #[test]
    fn test_parse_masks_collectible_masks_high_from_raw() {
        use vanilla_offsets::*;

        let mut data = vec![0u8; MM_SIZE];

        // Place high-bit collectible masks
        data[MASKS] = mm_item_ids::MASK_GIBDO;
        data[MASKS + 1] = mm_item_ids::MASK_GARO;
        data[MASKS + 2] = mm_item_ids::MASK_CAPTAIN;
        data[MASKS + 3] = mm_item_ids::MASK_GIANT;

        let save = MmSave::from_save_data(&data).unwrap();

        assert!(save.masks.masks_high.contains(MmMasksHigh::GIBDO));
        assert!(save.masks.masks_high.contains(MmMasksHigh::GARO));
        assert!(save.masks.masks_high.contains(MmMasksHigh::CAPTAIN));
        assert!(save.masks.masks_high.contains(MmMasksHigh::GIANT));
        assert_eq!(save.masks.masks_high.bits().count_ones(), 4);
    }

    #[test]
    fn test_parse_masks_empty_slots() {
        use vanilla_offsets::*;

        let mut data = vec![0u8; MM_SIZE];

        // Fill mask inventory with NONE (0xFF)
        for i in 0..24 {
            data[MASKS + i] = mm_item_ids::NONE;
        }

        let save = MmSave::from_save_data(&data).unwrap();

        assert!(save.masks.transformation.is_empty());
        assert!(save.masks.masks_low.is_empty());
        assert!(save.masks.masks_high.is_empty());
        assert_eq!(save.masks.total_mask_count(), 0);
    }

    #[test]
    fn test_parse_masks_invalid_ids_ignored() {
        use vanilla_offsets::*;

        let mut data = vec![0u8; MM_SIZE];

        // Put some invalid mask IDs (regular item IDs, not masks)
        data[MASKS] = mm_item_ids::OCARINA; // Not a mask
        data[MASKS + 1] = mm_item_ids::BOW; // Not a mask
        data[MASKS + 2] = 0xAA; // Invalid ID
        data[MASKS + 3] = mm_item_ids::MASK_DEKU; // Valid mask

        let save = MmSave::from_save_data(&data).unwrap();

        // Only the Deku Mask should be recognized
        assert!(save
            .masks
            .transformation
            .contains(MmTransformationMasks::DEKU));
        assert_eq!(save.masks.total_mask_count(), 1);
    }

    #[test]
    fn test_parse_masks_all_24_slots() {
        use vanilla_offsets::*;

        let mut data = vec![0u8; MM_SIZE];

        // Put all 24 masks in their slots
        // Transformation masks (4)
        data[MASKS] = mm_item_ids::MASK_DEKU;
        data[MASKS + 1] = mm_item_ids::MASK_GORON;
        data[MASKS + 2] = mm_item_ids::MASK_ZORA;
        data[MASKS + 3] = mm_item_ids::MASK_FIERCE_DEITY;

        // Low masks (16)
        data[MASKS + 4] = mm_item_ids::MASK_POSTMAN;
        data[MASKS + 5] = mm_item_ids::MASK_ALL_NIGHT;
        data[MASKS + 6] = mm_item_ids::MASK_BLAST;
        data[MASKS + 7] = mm_item_ids::MASK_STONE;
        data[MASKS + 8] = mm_item_ids::MASK_GREAT_FAIRY;
        data[MASKS + 9] = mm_item_ids::MASK_KEATON;
        data[MASKS + 10] = mm_item_ids::MASK_BREMEN;
        data[MASKS + 11] = mm_item_ids::MASK_BUNNY;
        data[MASKS + 12] = mm_item_ids::MASK_DON_GERO;
        data[MASKS + 13] = mm_item_ids::MASK_SCENTS;
        data[MASKS + 14] = mm_item_ids::MASK_ROMANI;
        data[MASKS + 15] = mm_item_ids::MASK_CIRCUS_LEADER;
        data[MASKS + 16] = mm_item_ids::MASK_KAFEI;
        data[MASKS + 17] = mm_item_ids::MASK_COUPLES;
        data[MASKS + 18] = mm_item_ids::MASK_TRUTH;
        data[MASKS + 19] = mm_item_ids::MASK_KAMARO;

        // High masks (4)
        data[MASKS + 20] = mm_item_ids::MASK_GIBDO;
        data[MASKS + 21] = mm_item_ids::MASK_GARO;
        data[MASKS + 22] = mm_item_ids::MASK_CAPTAIN;
        data[MASKS + 23] = mm_item_ids::MASK_GIANT;

        let save = MmSave::from_save_data(&data).unwrap();

        // Total should be 4 transformation + 16 low + 4 high = 24
        assert_eq!(save.masks.transformation.bits().count_ones(), 4);
        assert_eq!(save.masks.masks_low.bits().count_ones(), 16);
        assert_eq!(save.masks.masks_high.bits().count_ones(), 4);
        assert_eq!(save.masks.total_mask_count(), 24);
    }

    // ========================================================================
    // Inventory Parsing from Raw Bytes Tests
    // ========================================================================

    #[test]
    fn test_parse_inventory_all_items_from_raw() {
        use vanilla_offsets::*;

        let mut data = vec![0u8; MM_SIZE];

        // Set all inventory items at their correct slot positions
        data[INVENTORY] = mm_item_ids::OCARINA; // slot 0
        data[INVENTORY + 1] = mm_item_ids::BOW; // slot 1
        data[INVENTORY + 2] = mm_item_ids::FIRE_ARROW; // slot 2
        data[INVENTORY + 3] = mm_item_ids::ICE_ARROW; // slot 3
        data[INVENTORY + 4] = mm_item_ids::LIGHT_ARROW; // slot 4
        data[INVENTORY + 6] = mm_item_ids::BOMB; // slot 6
        data[INVENTORY + 7] = mm_item_ids::BOMBCHU; // slot 7
        data[INVENTORY + 8] = mm_item_ids::DEKU_STICK; // slot 8
        data[INVENTORY + 9] = mm_item_ids::DEKU_NUT; // slot 9
        data[INVENTORY + 10] = mm_item_ids::MAGIC_BEAN; // slot 10
        data[INVENTORY + 12] = mm_item_ids::POWDER_KEG; // slot 12
        data[INVENTORY + 13] = mm_item_ids::PICTOGRAPH_BOX; // slot 13
        data[INVENTORY + 14] = mm_item_ids::LENS; // slot 14
        data[INVENTORY + 15] = mm_item_ids::HOOKSHOT; // slot 15
        data[INVENTORY + 16] = mm_item_ids::GREAT_FAIRY_SWORD; // slot 16

        let save = MmSave::from_save_data(&data).unwrap();

        assert!(save.inventory.ocarina);
        assert!(save.inventory.bow);
        assert!(save.inventory.fire_arrows);
        assert!(save.inventory.ice_arrows);
        assert!(save.inventory.light_arrows);
        assert!(save.inventory.bombs);
        assert!(save.inventory.bombchus);
        assert!(save.inventory.deku_sticks);
        assert!(save.inventory.deku_nuts);
        assert!(save.inventory.magic_beans);
        assert!(save.inventory.powder_keg);
        assert!(save.inventory.pictograph_box);
        assert!(save.inventory.lens);
        assert!(save.inventory.hookshot);
        assert!(save.inventory.great_fairy_sword);
    }

    #[test]
    fn test_parse_inventory_empty_from_raw() {
        let mut data = vec![0u8; MM_SIZE];

        // Fill inventory slots with NONE (0xFF)
        for i in 0..24 {
            data[vanilla_offsets::INVENTORY + i] = mm_item_ids::NONE;
        }

        let save = MmSave::from_save_data(&data).unwrap();

        assert!(!save.inventory.ocarina);
        assert!(!save.inventory.bow);
        assert!(!save.inventory.fire_arrows);
        assert!(!save.inventory.ice_arrows);
        assert!(!save.inventory.light_arrows);
        assert!(!save.inventory.bombs);
        assert!(!save.inventory.bombchus);
        assert!(!save.inventory.deku_sticks);
        assert!(!save.inventory.deku_nuts);
        assert!(!save.inventory.magic_beans);
        assert!(!save.inventory.powder_keg);
        assert!(!save.inventory.pictograph_box);
        assert!(!save.inventory.lens);
        assert!(!save.inventory.hookshot);
        assert!(!save.inventory.great_fairy_sword);
    }

    #[test]
    fn test_parse_inventory_wrong_slot_item() {
        use vanilla_offsets::*;

        let mut data = vec![0u8; MM_SIZE];

        // Put bow item ID in ocarina slot - should not detect ocarina
        data[INVENTORY] = mm_item_ids::BOW;

        let save = MmSave::from_save_data(&data).unwrap();

        // Ocarina should be false because the value doesn't match expected
        assert!(!save.inventory.ocarina);
        // Bow should also be false because it's not in the right slot
        assert!(!save.inventory.bow);
    }

    #[test]
    fn test_parse_inventory_bottles_all_types() {
        use vanilla_offsets::*;

        let mut data = vec![0u8; MM_SIZE];

        // Set bottles in slots 18-23
        data[INVENTORY + 18] = mm_item_ids::BOTTLE_EMPTY;
        data[INVENTORY + 19] = mm_item_ids::BOTTLE_RED_POTION;
        data[INVENTORY + 20] = mm_item_ids::BOTTLE_FAIRY;
        data[INVENTORY + 21] = mm_item_ids::BOTTLE_DEKU_PRINCESS;
        data[INVENTORY + 22] = mm_item_ids::BOTTLE_CHATEAU_ROMANI;
        data[INVENTORY + 23] = mm_item_ids::BOTTLE_ZORA_EGG;

        let save = MmSave::from_save_data(&data).unwrap();

        assert_eq!(save.inventory.bottles[0], MmBottle::Empty);
        assert_eq!(save.inventory.bottles[1], MmBottle::RedPotion);
        assert_eq!(save.inventory.bottles[2], MmBottle::Fairy);
        assert_eq!(save.inventory.bottles[3], MmBottle::DekuPrincess);
        assert_eq!(save.inventory.bottles[4], MmBottle::ChateauRomani);
        assert_eq!(save.inventory.bottles[5], MmBottle::ZoraEgg);
    }

    #[test]
    fn test_parse_inventory_bottles_empty_slots() {
        let data = vec![0u8; MM_SIZE];

        let save = MmSave::from_save_data(&data).unwrap();

        // All bottles should be None when slots are zero
        for bottle in &save.inventory.bottles {
            assert_eq!(*bottle, MmBottle::None);
        }
    }

    #[test]
    fn test_parse_inventory_bottles_all_variants() {
        use vanilla_offsets::*;

        let bottle_types = [
            (mm_item_ids::BOTTLE_EMPTY, MmBottle::Empty),
            (mm_item_ids::BOTTLE_GREEN_POTION, MmBottle::GreenPotion),
            (mm_item_ids::BOTTLE_BLUE_POTION, MmBottle::BluePotion),
            (mm_item_ids::BOTTLE_MILK, MmBottle::Milk),
            (mm_item_ids::BOTTLE_MILK_HALF, MmBottle::MilkHalf),
            (mm_item_ids::BOTTLE_FISH, MmBottle::Fish),
            (mm_item_ids::BOTTLE_BUG, MmBottle::Bug),
            (mm_item_ids::BOTTLE_BLUE_FIRE, MmBottle::BlueFire),
            (mm_item_ids::BOTTLE_POE, MmBottle::Poe),
            (mm_item_ids::BOTTLE_BIG_POE, MmBottle::BigPoe),
            (mm_item_ids::BOTTLE_WATER, MmBottle::Water),
            (
                mm_item_ids::BOTTLE_HOT_SPRING_WATER,
                MmBottle::HotSpringWater,
            ),
            (mm_item_ids::BOTTLE_GOLD_DUST, MmBottle::GoldDust),
            (mm_item_ids::BOTTLE_MUSHROOM, MmBottle::MagicalMushroom),
            (mm_item_ids::BOTTLE_SEAHORSE, MmBottle::SeaHorse),
            (mm_item_ids::BOTTLE_MYSTERY_MILK, MmBottle::MysteryMilk),
            (
                mm_item_ids::BOTTLE_MYSTERY_MILK_SPOILED,
                MmBottle::MysteryMilkSpoiled,
            ),
        ];

        for (raw_id, expected_bottle) in bottle_types {
            let mut data = vec![0u8; MM_SIZE];
            data[INVENTORY + 18] = raw_id;

            let save = MmSave::from_save_data(&data).unwrap();
            assert_eq!(
                save.inventory.bottles[0], expected_bottle,
                "Failed for bottle ID 0x{:02X}",
                raw_id
            );
        }
    }

    // ========================================================================
    // Small Keys Parsing Tests
    // ========================================================================

    #[test]
    fn test_parse_small_keys_from_raw() {
        use vanilla_offsets::*;

        let mut data = vec![0u8; MM_SIZE];

        data[SMALL_KEYS] = 3; // Woodfall
        data[SMALL_KEYS + 1] = 2; // Snowhead
        data[SMALL_KEYS + 2] = 1; // Great Bay
        data[SMALL_KEYS + 3] = 4; // Stone Tower

        let save = MmSave::from_save_data(&data).unwrap();

        assert_eq!(save.small_keys.woodfall, 3);
        assert_eq!(save.small_keys.snowhead, 2);
        assert_eq!(save.small_keys.great_bay, 1);
        assert_eq!(save.small_keys.stone_tower, 4);
    }

    #[test]
    fn test_parse_small_keys_0xff_as_zero() {
        use vanilla_offsets::*;

        let mut data = vec![0u8; MM_SIZE];

        // 0xFF means no keys collected yet (uninitialized)
        data[SMALL_KEYS] = 0xFF;
        data[SMALL_KEYS + 1] = 0xFF;
        data[SMALL_KEYS + 2] = 0xFF;
        data[SMALL_KEYS + 3] = 0xFF;

        let save = MmSave::from_save_data(&data).unwrap();

        // 0xFF should be treated as 0
        assert_eq!(save.small_keys.woodfall, 0);
        assert_eq!(save.small_keys.snowhead, 0);
        assert_eq!(save.small_keys.great_bay, 0);
        assert_eq!(save.small_keys.stone_tower, 0);
    }

    #[test]
    fn test_parse_small_keys_mixed_values() {
        use vanilla_offsets::*;

        let mut data = vec![0u8; MM_SIZE];

        data[SMALL_KEYS] = 0xFF; // Should become 0
        data[SMALL_KEYS + 1] = 2;
        data[SMALL_KEYS + 2] = 0xFF; // Should become 0
        data[SMALL_KEYS + 3] = 3;

        let save = MmSave::from_save_data(&data).unwrap();

        assert_eq!(save.small_keys.woodfall, 0);
        assert_eq!(save.small_keys.snowhead, 2);
        assert_eq!(save.small_keys.great_bay, 0);
        assert_eq!(save.small_keys.stone_tower, 3);
    }

    #[test]
    fn test_small_keys_accessor_methods() {
        let keys = MmSmallKeys {
            woodfall: 1,
            snowhead: 2,
            great_bay: 3,
            stone_tower: 4,
        };

        assert_eq!(keys.snowhead(), 2);
        assert_eq!(keys.great_bay(), 3);
        assert_eq!(keys.stone_tower(), 4);
    }

    // ========================================================================
    // Song Parsing from Raw Bytes Tests
    // ========================================================================

    #[test]
    fn test_parse_all_songs_from_raw() {
        use vanilla_offsets::*;

        let mut data = vec![0u8; MM_SIZE];

        // Set all song bits
        let quest_bits: u32 = MmQuestItems::SONG_AWAKENING.bits()
            | MmQuestItems::SONG_GORON.bits()
            | MmQuestItems::SONG_ZORA.bits()
            | MmQuestItems::SONG_EMPTINESS.bits()
            | MmQuestItems::SONG_ORDER.bits()
            | MmQuestItems::SONG_TIME.bits()
            | MmQuestItems::SONG_HEALING.bits()
            | MmQuestItems::SONG_EPONA.bits()
            | MmQuestItems::SONG_SOARING.bits()
            | MmQuestItems::SONG_STORMS.bits()
            | MmQuestItems::SONG_LULLABY_INTRO.bits();

        data[QUEST_ITEMS..QUEST_ITEMS + 4].copy_from_slice(&quest_bits.to_be_bytes());

        let save = MmSave::from_save_data(&data).unwrap();

        assert!(save.quest_items.contains(MmQuestItems::SONG_AWAKENING));
        assert!(save.quest_items.contains(MmQuestItems::SONG_GORON));
        assert!(save.quest_items.contains(MmQuestItems::SONG_ZORA));
        assert!(save.quest_items.contains(MmQuestItems::SONG_EMPTINESS));
        assert!(save.quest_items.contains(MmQuestItems::SONG_ORDER));
        assert!(save.quest_items.contains(MmQuestItems::SONG_TIME));
        assert!(save.quest_items.contains(MmQuestItems::SONG_HEALING));
        assert!(save.quest_items.contains(MmQuestItems::SONG_EPONA));
        assert!(save.quest_items.contains(MmQuestItems::SONG_SOARING));
        assert!(save.quest_items.contains(MmQuestItems::SONG_STORMS));
        assert!(save.quest_items.contains(MmQuestItems::SONG_LULLABY_INTRO));
    }

    #[test]
    fn test_parse_songs_individual_bits() {
        use vanilla_offsets::*;

        // Test each song bit individually
        let songs = [
            (MmQuestItems::SONG_AWAKENING, "Sonata of Awakening"),
            (MmQuestItems::SONG_GORON, "Goron Lullaby"),
            (MmQuestItems::SONG_ZORA, "New Wave Bossa Nova"),
            (MmQuestItems::SONG_EMPTINESS, "Elegy of Emptiness"),
            (MmQuestItems::SONG_ORDER, "Oath to Order"),
            (MmQuestItems::SONG_TIME, "Song of Time"),
            (MmQuestItems::SONG_HEALING, "Song of Healing"),
            (MmQuestItems::SONG_EPONA, "Epona's Song"),
            (MmQuestItems::SONG_SOARING, "Song of Soaring"),
            (MmQuestItems::SONG_STORMS, "Song of Storms"),
        ];

        for (song_flag, name) in songs {
            let mut data = vec![0u8; MM_SIZE];
            data[QUEST_ITEMS..QUEST_ITEMS + 4].copy_from_slice(&song_flag.bits().to_be_bytes());

            let save = MmSave::from_save_data(&data).unwrap();
            assert!(
                save.quest_items.contains(song_flag),
                "Failed to parse {}",
                name
            );
        }
    }

    // ========================================================================
    // Edge Cases and Error Handling Tests
    // ========================================================================

    #[test]
    fn test_sword_try_from_all_variants() {
        assert_eq!(MmSword::try_from(0), Ok(MmSword::None));
        assert_eq!(MmSword::try_from(1), Ok(MmSword::KokiriSword));
        assert_eq!(MmSword::try_from(2), Ok(MmSword::RazorSword));
        assert_eq!(MmSword::try_from(3), Ok(MmSword::GildedSword));
        assert_eq!(MmSword::try_from(4), Err(4));
        assert_eq!(MmSword::try_from(255), Err(255));
    }

    #[test]
    fn test_shield_try_from_all_variants() {
        assert_eq!(MmShield::try_from(0), Ok(MmShield::None));
        assert_eq!(MmShield::try_from(1), Ok(MmShield::HeroShield));
        assert_eq!(MmShield::try_from(2), Ok(MmShield::HylianShield));
        assert_eq!(MmShield::try_from(3), Ok(MmShield::MirrorShield));
        assert_eq!(MmShield::try_from(4), Err(4));
        assert_eq!(MmShield::try_from(255), Err(255));
    }

    #[test]
    fn test_magic_capacity_try_from_all_variants() {
        assert_eq!(MmMagicCapacity::try_from(0), Ok(MmMagicCapacity::None));
        assert_eq!(MmMagicCapacity::try_from(1), Ok(MmMagicCapacity::Single));
        assert_eq!(MmMagicCapacity::try_from(2), Ok(MmMagicCapacity::Double));
        assert_eq!(MmMagicCapacity::try_from(3), Err(3));
        assert_eq!(MmMagicCapacity::try_from(255), Err(255));
    }

    #[test]
    fn test_parse_invalid_player_form_defaults_to_human() {
        use vanilla_offsets::*;

        let mut data = vec![0u8; MM_SIZE];
        data[PLAYER_FORM] = 99; // Invalid form

        let save = MmSave::from_save_data(&data).unwrap();
        assert_eq!(save.player_form, PlayerForm::Human);
    }

    #[test]
    fn test_parse_invalid_magic_defaults_to_none() {
        use vanilla_offsets::*;

        let mut data = vec![0u8; MM_SIZE];
        data[MAGIC_LEVEL] = 99; // Invalid magic level

        let save = MmSave::from_save_data(&data).unwrap();
        assert_eq!(save.magic, MmMagicCapacity::None);
    }

    #[test]
    fn test_parse_invalid_sword_defaults_to_none() {
        use vanilla_offsets::*;

        let mut data = vec![0u8; MM_SIZE];
        data[SWORD_SHIELD] = 0x0F; // Invalid sword value (15)

        let save = MmSave::from_save_data(&data).unwrap();
        assert_eq!(save.sword, MmSword::None);
    }

    #[test]
    fn test_parse_invalid_shield_defaults_to_none() {
        use vanilla_offsets::*;

        let mut data = vec![0u8; MM_SIZE];
        data[SWORD_SHIELD] = 0xF0; // Invalid shield value (15 << 4)

        let save = MmSave::from_save_data(&data).unwrap();
        assert_eq!(save.shield, MmShield::None);
    }

    #[test]
    fn test_heart_pieces_parsing() {
        let mut quest = MmQuestItems::empty();
        assert_eq!(quest.heart_pieces(), 0);

        quest.insert(MmQuestItems::HEART_PIECE_1);
        assert_eq!(quest.heart_pieces(), 1);

        quest.insert(MmQuestItems::HEART_PIECE_2);
        assert_eq!(quest.heart_pieces(), 3);

        quest.insert(MmQuestItems::HEART_PIECE_3);
        assert_eq!(quest.heart_pieces(), 7);

        // Test maximum heart pieces
        quest = MmQuestItems::HEART_PIECE_1
            | MmQuestItems::HEART_PIECE_2
            | MmQuestItems::HEART_PIECE_3
            | MmQuestItems::HEART_PIECE_4;
        assert_eq!(quest.heart_pieces(), 15);
    }

    #[test]
    fn test_bombers_notebook_accessor() {
        let mut save = MmSave::default();
        assert!(!save.has_bombers_notebook());

        save.quest_items.insert(MmQuestItems::NOTEBOOK);
        assert!(save.has_bombers_notebook());
    }

    #[test]
    fn test_bombers_notebook_from_raw() {
        use vanilla_offsets::*;

        let mut data = vec![0u8; MM_SIZE];
        let quest_bits: u32 = MmQuestItems::NOTEBOOK.bits();
        data[QUEST_ITEMS..QUEST_ITEMS + 4].copy_from_slice(&quest_bits.to_be_bytes());

        let save = MmSave::from_save_data(&data).unwrap();
        assert!(save.has_bombers_notebook());
    }

    #[test]
    fn test_dungeon_items_get_method() {
        let items = MmAllDungeonItems {
            woodfall: MmDungeonItems::MAP | MmDungeonItems::COMPASS,
            snowhead: MmDungeonItems::BOSS_KEY,
            great_bay: MmDungeonItems::MAP,
            stone_tower: MmDungeonItems::all(),
        };

        assert_eq!(items.get(0), MmDungeonItems::MAP | MmDungeonItems::COMPASS);
        assert_eq!(items.get(1), MmDungeonItems::BOSS_KEY);
        assert_eq!(items.get(2), MmDungeonItems::MAP);
        assert_eq!(items.get(3), MmDungeonItems::all());
        assert_eq!(items.get(4), MmDungeonItems::default()); // Out of bounds
        assert_eq!(items.get(99), MmDungeonItems::default()); // Way out of bounds
    }

    #[test]
    fn test_cycle_scene_flags_parsing() {
        use vanilla_offsets::*;

        let mut data = vec![0u8; MM_SIZE];

        // Set first cycle scene's chest flags
        let chest_flags: u32 = 0xDEADBEEF;
        data[CYCLE_SCENE_FLAGS..CYCLE_SCENE_FLAGS + 4].copy_from_slice(&chest_flags.to_be_bytes());

        let save = MmSave::from_save_data(&data).unwrap();
        assert_eq!(save.cycle_scene_flags.len(), 120);
        assert_eq!(save.cycle_scene_flags[0].chest, 0xDEADBEEF);
    }

    #[test]
    fn test_decode_error_variants() {
        // Test various decode error types exist and can be created
        let _err = MmDecodeError::AssertEq {
            offset: 100,
            expected: 0,
            found: 1,
        };

        let _err = MmDecodeError::AssertEqRange {
            start: 0,
            end: 4,
            expected: vec![0, 0, 0, 0],
            found: vec![1, 2, 3, 4],
        };

        let _err = MmDecodeError::Index(42);

        let _err = MmDecodeError::IndexRange { start: 0, end: 100 };

        let _err = MmDecodeError::Size(50);

        let _err = MmDecodeError::UnexpectedValue {
            offset: 10,
            field: "test_field",
            value: 99,
        };

        let _err = MmDecodeError::UnexpectedValueRange {
            start: 0,
            end: 4,
            field: "test_field",
            value: vec![1, 2, 3, 4],
        };
    }

    #[test]
    fn test_from_save_data_exactly_mm_size() {
        // Test with exactly MM_SIZE bytes
        let data = vec![0u8; MM_SIZE];
        assert!(MmSave::from_save_data(&data).is_ok());
    }

    #[test]
    fn test_from_save_data_larger_than_mm_size() {
        // Data larger than MM_SIZE should still work (only reads MM_SIZE)
        let data = vec![0u8; MM_SIZE + 100];
        assert!(MmSave::from_save_data(&data).is_ok());
    }

    #[test]
    fn test_to_save_data_produces_correct_size() {
        let save = MmSave::default();
        let data = save.to_save_data();
        assert_eq!(data.len(), MM_SIZE);
    }

    #[test]
    fn test_parse_upgrades_from_raw() {
        use vanilla_offsets::*;

        let mut data = vec![0u8; MM_SIZE];

        let upgrade_bits: u32 = MmUpgrades::ADULTS_WALLET.bits()
            | MmUpgrades::BOMB_BAG_30.bits()
            | MmUpgrades::QUIVER_40.bits();
        data[UPGRADES..UPGRADES + 4].copy_from_slice(&upgrade_bits.to_be_bytes());

        let save = MmSave::from_save_data(&data).unwrap();

        assert_eq!(save.upgrades.wallet(), MmUpgrades::ADULTS_WALLET);
        assert_eq!(save.upgrades.bomb_bag(), MmUpgrades::BOMB_BAG_30);
        assert_eq!(save.upgrades.quiver(), MmUpgrades::QUIVER_40);
    }

    #[test]
    fn test_upgrades_set_wallet() {
        let mut upgrades = MmUpgrades::empty();

        upgrades.set_wallet(MmUpgrades::ADULTS_WALLET);
        assert_eq!(upgrades.wallet(), MmUpgrades::ADULTS_WALLET);

        upgrades.set_wallet(MmUpgrades::GIANTS_WALLET);
        assert_eq!(upgrades.wallet(), MmUpgrades::GIANTS_WALLET);

        // Setting to empty should clear
        upgrades.set_wallet(MmUpgrades::empty());
        assert_eq!(upgrades.wallet(), MmUpgrades::empty());
    }

    #[test]
    fn test_double_defense_parsing() {
        use vanilla_offsets::*;

        let mut data = vec![0u8; MM_SIZE];

        // No double defense
        data[DOUBLE_DEFENSE] = 0;
        let save = MmSave::from_save_data(&data).unwrap();
        assert!(!save.double_defense);

        // Has double defense (any non-zero value)
        data[DOUBLE_DEFENSE] = 1;
        let save = MmSave::from_save_data(&data).unwrap();
        assert!(save.double_defense);

        data[DOUBLE_DEFENSE] = 255;
        let save = MmSave::from_save_data(&data).unwrap();
        assert!(save.double_defense);
    }

    #[test]
    #[allow(clippy::field_reassign_with_default)]
    fn test_heart_containers_calculation() {
        let mut save = MmSave::default();

        // 3 hearts = 0x30 (48 in decimal, 48/16 = 3)
        save.health_capacity = 0x30;
        assert_eq!(save.heart_containers(), 3, "3 hearts should equal 0x30");

        // 20 hearts = 0x140 (320 in decimal, 320/16 = 20)
        save.health_capacity = 0x140;
        assert_eq!(save.heart_containers(), 20, "20 hearts should equal 0x140");

        // 10 hearts = 0xA0 (160 in decimal, 160/16 = 10)
        save.health_capacity = 0xA0;
        assert_eq!(save.heart_containers(), 10, "10 hearts should equal 0xA0");

        // Partial hearts should round down
        // 0x35 = 53, 53/16 = 3 (rounds down from 3.3125)
        save.health_capacity = 0x35;
        assert_eq!(
            save.heart_containers(),
            3,
            "Partial hearts should round down"
        );

        // 0x4F = 79, 79/16 = 4 (rounds down from 4.9375)
        save.health_capacity = 0x4F;
        assert_eq!(
            save.heart_containers(),
            4,
            "Partial hearts should round down to 4"
        );

        // Zero hearts edge case
        save.health_capacity = 0;
        assert_eq!(
            save.heart_containers(),
            0,
            "Zero health_capacity = 0 hearts"
        );
    }

    #[test]
    fn test_heart_containers_parsing() {
        use vanilla_offsets::*;

        let mut data = vec![0u8; MM_SIZE];

        // Set health_capacity at offset HEALTH_CAPACITY (0x002C) - big-endian u16
        // 3 hearts = 0x0030
        data[HEALTH_CAPACITY] = 0x00;
        data[HEALTH_CAPACITY + 1] = 0x30;

        let save = MmSave::from_save_data(&data).unwrap();
        assert_eq!(save.health_capacity, 0x30, "Should parse 3 hearts (0x30)");
        assert_eq!(save.heart_containers(), 3);

        // Test 20 hearts = 0x0140
        data[HEALTH_CAPACITY] = 0x01;
        data[HEALTH_CAPACITY + 1] = 0x40;

        let save = MmSave::from_save_data(&data).unwrap();
        assert_eq!(
            save.health_capacity, 0x140,
            "Should parse 20 hearts (0x140)"
        );
        assert_eq!(save.heart_containers(), 20);

        // Test 12 hearts = 0x00C0
        data[HEALTH_CAPACITY] = 0x00;
        data[HEALTH_CAPACITY + 1] = 0xC0;

        let save = MmSave::from_save_data(&data).unwrap();
        assert_eq!(save.health_capacity, 0xC0, "Should parse 12 hearts (0xC0)");
        assert_eq!(save.heart_containers(), 12);
    }

    #[test]
    fn test_heart_pieces_method() {
        // Test the heart_pieces() method on MmQuestItems
        let mut quest_items = MmQuestItems::empty();

        // No heart pieces
        assert_eq!(quest_items.heart_pieces(), 0);

        // 1 heart piece
        quest_items = MmQuestItems::HEART_PIECE_1;
        assert_eq!(quest_items.heart_pieces(), 1);

        // 2 heart pieces
        quest_items = MmQuestItems::HEART_PIECE_1 | MmQuestItems::HEART_PIECE_2;
        assert_eq!(quest_items.heart_pieces(), 3); // bits 28+29 set = 0011 = 3

        // Actually test the bit counting correctly
        // HEART_PIECE_1 = 1 << 28, HEART_PIECE_2 = 1 << 29
        // The heart_pieces() method does (bits >> 28) & 0xF

        // Test individual piece bits
        quest_items = MmQuestItems::from_bits_truncate(1 << 28);
        assert_eq!(quest_items.heart_pieces(), 1, "Bit 28 should be 1 piece");

        quest_items = MmQuestItems::from_bits_truncate(2 << 28);
        assert_eq!(quest_items.heart_pieces(), 2, "Bits should be 2 pieces");

        quest_items = MmQuestItems::from_bits_truncate(3 << 28);
        assert_eq!(quest_items.heart_pieces(), 3, "Bits should be 3 pieces");

        // Test with other quest items set
        quest_items = MmQuestItems::REMAINS_ODOLWA
            | MmQuestItems::SONG_TIME
            | MmQuestItems::from_bits_truncate(2 << 28);
        assert_eq!(
            quest_items.heart_pieces(),
            2,
            "Should have 2 heart pieces with other items"
        );
    }
}
