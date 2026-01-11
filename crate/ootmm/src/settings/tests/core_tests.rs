//! Core RandomizerSettings tests.

use crate::settings::*;

#[test]
fn test_default_settings() {
    let settings = RandomizerSettings::default();
    assert!(!settings.ageless_boots);
    assert!(!settings.er_moon);
    assert!(settings.open_dungeons_oot.is_empty());
    assert!(settings.open_dungeons_mm.is_empty());
    assert_eq!(settings.deku_tree, DekuTreeState::Closed);
    assert_eq!(settings.logic_mode, LogicMode::Glitchless);
}

#[test]
fn test_bool_setting_lookup() {
    let mut settings = RandomizerSettings::new();
    settings.ageless_boots = true;
    settings.er_moon = true;

    assert_eq!(settings.get_bool_setting("agelessBoots"), Some(true));
    assert_eq!(settings.get_bool_setting("erMoon"), Some(true));
    assert_eq!(settings.get_bool_setting("skipZelda"), Some(false));
    assert_eq!(settings.get_bool_setting("unknownSetting"), None);
}

#[test]
fn test_value_setting_lookup_dungeons() {
    let mut settings = RandomizerSettings::new();
    settings
        .open_dungeons_oot
        .insert(OotDungeon::DodongosCavern);
    settings
        .open_dungeons_oot
        .insert(OotDungeon::BottomOfTheWell);
    settings.open_dungeons_mm.insert(MmDungeon::StoneTower);

    assert!(settings.check_setting_value("openDungeonsOot", "DC"));
    assert!(settings.check_setting_value("openDungeonsOot", "BotW"));
    assert!(!settings.check_setting_value("openDungeonsOot", "Shadow"));
    assert!(settings.check_setting_value("openDungeonsMm", "ST"));
    assert!(!settings.check_setting_value("openDungeonsMm", "WF"));
}

#[test]
fn test_value_setting_lookup_enums() {
    let mut settings = RandomizerSettings::new();
    settings.deku_tree = DekuTreeState::Open;
    settings.ganon_boss_key = GanonBossKeyMode::Removed;
    settings.age_change = AgeChangeMode::None;

    assert!(settings.check_setting_value("dekuTree", "open"));
    assert!(!settings.check_setting_value("dekuTree", "vanilla"));
    assert!(settings.check_setting_value("ganonBossKey", "removed"));
    assert!(settings.check_setting_value("ageChange", "none"));
}

#[test]
fn test_jp_layouts() {
    let mut settings = RandomizerSettings::new();
    settings.jp_layouts.insert(JpLayout::StoneTower);

    assert!(settings.check_setting_value("jpLayouts", "StoneTower"));
    assert!(!settings.check_setting_value("jpLayouts", "ST"));
}

#[test]
fn test_tricks() {
    let mut settings = RandomizerSettings::new();
    assert!(!settings.has_trick("hover_boost"));

    settings.enable_trick("hover_boost");
    assert!(settings.has_trick("hover_boost"));

    settings.disable_trick("hover_boost");
    assert!(!settings.has_trick("hover_boost"));
}

#[test]
fn test_serde_roundtrip() {
    let mut settings = RandomizerSettings::new();
    settings.ageless_boots = true;
    settings
        .open_dungeons_oot
        .insert(OotDungeon::DodongosCavern);
    settings.deku_tree = DekuTreeState::Open;
    settings.enable_trick("hover_boost");

    let json = serde_json::to_string(&settings).unwrap();
    let parsed: RandomizerSettings = serde_json::from_str(&json).unwrap();

    assert!(parsed.ageless_boots);
    assert!(parsed
        .open_dungeons_oot
        .contains(&OotDungeon::DodongosCavern));
    assert_eq!(parsed.deku_tree, DekuTreeState::Open);
    assert!(parsed.has_trick("hover_boost"));
}

#[test]
fn test_serde_yaml_roundtrip() {
    let mut settings = RandomizerSettings::new();
    settings.er_moon = true;
    settings.open_dungeons_mm.insert(MmDungeon::StoneTower);
    settings.ganon_boss_key = GanonBossKeyMode::Custom;

    let yaml = serde_yaml::to_string(&settings).unwrap();
    let parsed: RandomizerSettings = serde_yaml::from_str(&yaml).unwrap();

    assert!(parsed.er_moon);
    assert!(parsed.open_dungeons_mm.contains(&MmDungeon::StoneTower));
    assert_eq!(parsed.ganon_boss_key, GanonBossKeyMode::Custom);
}

#[test]
fn test_climb_most_surfaces_off() {
    let mut settings = RandomizerSettings::new();
    settings.climb_most_surfaces_oot = ClimbMostSurfacesState::Off;

    assert!(settings.check_setting_value("climbMostSurfacesOot", "off"));
    assert!(!settings.check_setting_value("climbMostSurfacesOot", "on"));
}

#[test]
fn test_hookshot_anywhere_off() {
    let mut settings = RandomizerSettings::new();
    settings.hookshot_anywhere_oot = HookshotAnywhereState::Off;

    assert!(settings.check_setting_value("hookshotAnywhereOot", "off"));
    assert!(!settings.check_setting_value("hookshotAnywhereOot", "on"));
}

#[test]
fn test_beneath_well_open() {
    let mut settings = RandomizerSettings::new();
    settings.beneath_well = BeneathWellState::Open;

    assert!(settings.check_setting_value("beneathWell", "open"));
    assert!(!settings.check_setting_value("beneathWell", "vanilla"));
}

#[test]
fn test_small_key_shuffle_anywhere() {
    let mut settings = RandomizerSettings::new();
    settings.small_key_shuffle_oot = SmallKeyShuffleOot::Anywhere;

    assert!(settings.check_setting_value("smallKeyShuffleOot", "anywhere"));
    assert!(!settings.check_setting_value("smallKeyShuffleOot", "vanilla"));
}

// === Bottle Count Tests ===

#[test]
fn test_bottle_count_default() {
    let settings = RandomizerSettings::default();
    assert_eq!(settings.bottle_count, 4);
    assert_eq!(settings.get_bottle_count(), 4);
}

#[test]
fn test_bottle_count_set_and_get() {
    let mut settings = RandomizerSettings::new();

    settings.set_bottle_count(3);
    assert_eq!(settings.get_bottle_count(), 3);

    settings.set_bottle_count(1);
    assert_eq!(settings.get_bottle_count(), 1);
}

#[test]
fn test_bottle_count_clamping() {
    let mut settings = RandomizerSettings::new();

    // Test upper bound clamping
    settings.set_bottle_count(10);
    assert_eq!(settings.get_bottle_count(), 4);

    // Test lower bound clamping
    settings.set_bottle_count(0);
    assert_eq!(settings.get_bottle_count(), 1);
}

#[test]
fn test_bottle_count_serde_roundtrip() {
    let mut settings = RandomizerSettings::new();
    settings.set_bottle_count(2);

    let json = serde_json::to_string(&settings).unwrap();
    let parsed: RandomizerSettings = serde_json::from_str(&json).unwrap();

    assert_eq!(parsed.get_bottle_count(), 2);
}

#[test]
fn test_bottle_count_defaults_in_deserialization() {
    // Test that missing bottle_count in JSON defaults to 4
    let json = r#"{"agelessBoots": true}"#;
    let parsed: RandomizerSettings = serde_json::from_str(json).unwrap();

    assert_eq!(parsed.get_bottle_count(), 4);
}

// === Starting Items Tests ===

#[test]
fn test_starting_items_operations() {
    let mut settings = RandomizerSettings::new();
    assert_eq!(settings.starting_item_quantity("Sword"), 0);
    assert!(!settings.has_starting_item("Sword"));

    settings.set_starting_item("Sword", 1);
    assert_eq!(settings.starting_item_quantity("Sword"), 1);
    assert!(settings.has_starting_item("Sword"));

    settings.set_starting_item("Bow", 3);
    assert_eq!(settings.starting_item_quantity("Bow"), 3);
    assert_eq!(settings.starting_items_count(), 2);

    settings.remove_starting_item("Sword");
    assert!(!settings.has_starting_item("Sword"));
    assert_eq!(settings.starting_items_count(), 1);

    // Setting to 0 removes the item
    settings.set_starting_item("Bow", 0);
    assert!(!settings.has_starting_item("Bow"));
    assert_eq!(settings.starting_items_count(), 0);
}

#[test]
fn test_starting_items_iterator() {
    let mut settings = RandomizerSettings::new();
    settings.set_starting_item("Sword", 1);
    settings.set_starting_item("Shield", 2);
    settings.set_starting_item("Bow", 3);

    let items: Vec<_> = settings.starting_items_iter().collect();
    assert_eq!(items.len(), 3);
}

#[test]
fn test_starting_items_serde_roundtrip() {
    let mut settings = RandomizerSettings::new();
    settings.set_starting_item("Kokiri_Sword", 1);
    settings.set_starting_item("Deku_Shield", 1);
    settings.set_starting_item("Bombs", 20);

    let json = serde_json::to_string(&settings).unwrap();
    let parsed: RandomizerSettings = serde_json::from_str(&json).unwrap();

    assert_eq!(parsed.starting_item_quantity("Kokiri_Sword"), 1);
    assert_eq!(parsed.starting_item_quantity("Deku_Shield"), 1);
    assert_eq!(parsed.starting_item_quantity("Bombs"), 20);
}

// === Junk Locations Tests ===

#[test]
fn test_junk_locations_operations() {
    let mut settings = RandomizerSettings::new();
    assert!(!settings.is_junk_location("oot_chest_1"));
    assert_eq!(settings.junk_locations_count(), 0);

    settings.add_junk_location("oot_chest_1");
    assert!(settings.is_junk_location("oot_chest_1"));
    assert_eq!(settings.junk_locations_count(), 1);

    settings.add_junk_location("mm_chest_2");
    assert_eq!(settings.junk_locations_count(), 2);

    settings.remove_junk_location("oot_chest_1");
    assert!(!settings.is_junk_location("oot_chest_1"));
    assert_eq!(settings.junk_locations_count(), 1);
}

#[test]
fn test_junk_locations_iterator() {
    let mut settings = RandomizerSettings::new();
    settings.add_junk_location("loc1");
    settings.add_junk_location("loc2");
    settings.add_junk_location("loc3");

    let locations: Vec<_> = settings.junk_locations_iter().collect();
    assert_eq!(locations.len(), 3);
}

#[test]
fn test_junk_locations_serde_roundtrip() {
    let mut settings = RandomizerSettings::new();
    settings.add_junk_location("oot_kokiri_chest");
    settings.add_junk_location("mm_clock_tower_chest");

    let json = serde_json::to_string(&settings).unwrap();
    let parsed: RandomizerSettings = serde_json::from_str(&json).unwrap();

    assert!(parsed.is_junk_location("oot_kokiri_chest"));
    assert!(parsed.is_junk_location("mm_clock_tower_chest"));
}

// === World Flags Settings Methods Tests ===

#[test]
fn test_world_flags_settings_accessors() {
    let mut settings = RandomizerSettings::new();

    // Default values (false for all booleans)
    assert!(!settings.is_oot_enabled());
    assert!(!settings.is_mm_enabled());
    assert!(!settings.world_shared_items());
    assert!(!settings.world_shared_masks());

    // Modify and check
    settings.world_flags.oot_enabled = true;
    assert!(settings.is_oot_enabled());

    settings.world_flags.mm_enabled = true;
    assert!(settings.is_mm_enabled());

    settings.world_flags.shared_items = true;
    assert!(settings.world_shared_items());

    settings.world_flags.shared_masks = true;
    assert!(settings.world_shared_masks());
}

// === Comprehensive JSON Roundtrip Test ===

#[test]
fn test_comprehensive_json_roundtrip() {
    let mut settings = RandomizerSettings::new();

    // Set various boolean settings
    settings.er_moon = true;
    settings.skip_zelda = true;
    settings.ageless_boots = true;
    settings.shared_bows = true;

    // Set enum settings
    settings.ganon_boss_key = GanonBossKeyMode::Custom;
    settings.logic_mode = LogicMode::Glitched;
    settings.rainbow_bridge = RainbowBridgeMode::Medallions;
    settings.starting_age = StartingAge::Adult;
    settings.damage_multiplier = DamageMultiplier::Double;

    // Set collection fields
    settings
        .open_dungeons_oot
        .insert(OotDungeon::DodongosCavern);
    settings.open_dungeons_mm.insert(MmDungeon::StoneTower);
    settings.set_dungeon_mq(MqDungeon::ForestTemple);
    settings.enable_trick("OOT_LENS");

    // Set complex types
    settings.set_starting_item("Sword", 1);
    settings.add_junk_location("oot_chest");
    settings.set_special_condition("bridge", SpecialCondition::with_medallions(6));

    // Set world flags
    settings.world_flags.shared_items = true;

    // Serialize and deserialize
    let json = serde_json::to_string(&settings).unwrap();
    let parsed: RandomizerSettings = serde_json::from_str(&json).unwrap();

    // Verify all fields
    assert!(parsed.er_moon);
    assert!(parsed.skip_zelda);
    assert!(parsed.ageless_boots);
    assert!(parsed.shared_bows);
    assert_eq!(parsed.ganon_boss_key, GanonBossKeyMode::Custom);
    assert_eq!(parsed.logic_mode, LogicMode::Glitched);
    assert_eq!(parsed.rainbow_bridge, RainbowBridgeMode::Medallions);
    assert_eq!(parsed.starting_age, StartingAge::Adult);
    assert_eq!(parsed.damage_multiplier, DamageMultiplier::Double);
    assert!(parsed
        .open_dungeons_oot
        .contains(&OotDungeon::DodongosCavern));
    assert!(parsed.open_dungeons_mm.contains(&MmDungeon::StoneTower));
    assert!(parsed.is_dungeon_mq(MqDungeon::ForestTemple));
    assert!(parsed.has_trick("OOT_LENS"));
    assert_eq!(parsed.starting_item_quantity("Sword"), 1);
    assert!(parsed.is_junk_location("oot_chest"));
    assert_eq!(parsed.bridge_condition().unwrap().medallions, 6);
    assert!(parsed.world_shared_items());
}

// === Comprehensive YAML Roundtrip Test ===

#[test]
fn test_comprehensive_yaml_roundtrip() {
    let mut settings = RandomizerSettings::new();

    // Set various settings
    settings.er_moon = true;
    settings.open_mask_shop = true;
    settings.deku_tree = DekuTreeState::Open;
    settings.door_of_time = DoorOfTimeState::Open;
    settings.kakariko_gate = KakarikoGateState::Open;
    settings.boss_warp_pads = BossWarpPadsMode::Remains;
    settings.csmc = CsmcMode::Always;
    settings.item_pool = ItemPool::Scarce;
    settings.traps_quantity = TrapsQuantity::Many;

    settings.open_dungeons_oot.insert(OotDungeon::Shadow);
    settings.open_dungeons_oot.insert(OotDungeon::Water);
    settings.jp_layouts.insert(JpLayout::StoneTower);

    settings.set_starting_item("Bow", 1);
    settings.set_starting_item("Bombs", 20);
    settings.world_flags.mm_enabled = false;

    // Serialize and deserialize via YAML
    let yaml = serde_yaml::to_string(&settings).unwrap();
    let parsed: RandomizerSettings = serde_yaml::from_str(&yaml).unwrap();

    // Verify fields
    assert!(parsed.er_moon);
    assert!(parsed.open_mask_shop);
    assert_eq!(parsed.deku_tree, DekuTreeState::Open);
    assert_eq!(parsed.door_of_time, DoorOfTimeState::Open);
    assert_eq!(parsed.kakariko_gate, KakarikoGateState::Open);
    assert_eq!(parsed.boss_warp_pads, BossWarpPadsMode::Remains);
    assert_eq!(parsed.csmc, CsmcMode::Always);
    assert_eq!(parsed.item_pool, ItemPool::Scarce);
    assert_eq!(parsed.traps_quantity, TrapsQuantity::Many);
    assert!(parsed.open_dungeons_oot.contains(&OotDungeon::Shadow));
    assert!(parsed.open_dungeons_oot.contains(&OotDungeon::Water));
    assert!(parsed.jp_layouts.contains(&JpLayout::StoneTower));
    assert_eq!(parsed.starting_item_quantity("Bow"), 1);
    assert_eq!(parsed.starting_item_quantity("Bombs"), 20);
    assert!(!parsed.is_mm_enabled());
}
