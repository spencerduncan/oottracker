//! Tests for MM save data structures.

#[cfg(test)]
mod tests {
    use crate::mm_save::{
        constants::{mm_item_ids, MM_PERM_SCENE_COUNT, MM_SIZE},
        dungeon_progress::{MmAllDungeonItems, MmDungeonItems, MmSmallKeys, MmStrayFairies},
        inventory::MmBottle,
        masks::{MmMasks, MmMasksHigh, MmMasksLow, MmTransformationMasks},
        offsets::vanilla_offsets,
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

        // Set Gilded Sword (3) and Mirror Shield (2)
        // Shield is in high nibble, sword in low nibble
        data[SWORD_SHIELD] = 0x03 | (0x02 << 4);

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

        // Woodfall: Map + Compass + Boss Key (0x07)
        data[DUNGEON_ITEMS] = 0x07;
        // Snowhead: Map only (0x04)
        data[DUNGEON_ITEMS + 1] = 0x04;
        // Great Bay: Compass only (0x02)
        data[DUNGEON_ITEMS + 2] = 0x02;
        // Stone Tower: Boss Key only (0x01)
        data[DUNGEON_ITEMS + 3] = 0x01;

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
        use vanilla_offsets::*;

        let data = vec![0u8; MM_SIZE];
        let mut reader = MmSaveReader::from_bytes(&data).unwrap();

        // Update with new data containing different rupees
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
    fn test_boss_remains_accessors_default() {
        let save = MmSave::default();

        // All remains should be false by default
        assert!(!save.has_odolwa_remains());
        assert!(!save.has_goht_remains());
        assert!(!save.has_gyorg_remains());
        assert!(!save.has_twinmold_remains());
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
        assert_eq!(MmShield::try_from(2), Ok(MmShield::MirrorShield));
        assert_eq!(MmShield::try_from(3), Err(3));
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

        // Zero hearts edge case
        save.health_capacity = 0;
        assert_eq!(
            save.heart_containers(),
            0,
            "Zero health_capacity = 0 hearts"
        );
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
}
