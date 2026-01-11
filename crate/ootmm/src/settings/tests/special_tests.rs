//! SpecialCondition and WorldFlags tests.

use crate::settings::*;

// === SpecialCondition Tests ===

#[test]
fn test_special_condition_default() {
    let cond = SpecialCondition::default();
    assert_eq!(cond.stones, 0);
    assert_eq!(cond.medallions, 0);
    assert_eq!(cond.dungeon_rewards, 0);
    assert_eq!(cond.skulltulas, 0);
    assert_eq!(cond.remains, 0);
    assert!(!cond.has_requirements());
}

#[test]
fn test_special_condition_new() {
    let cond = SpecialCondition::new();
    assert!(!cond.has_requirements());
}

#[test]
fn test_special_condition_with_medallions() {
    let cond = SpecialCondition::with_medallions(6);
    assert_eq!(cond.medallions, 6);
    assert_eq!(cond.stones, 0);
    assert!(cond.has_requirements());
}

#[test]
fn test_special_condition_with_stones() {
    let cond = SpecialCondition::with_stones(3);
    assert_eq!(cond.stones, 3);
    assert_eq!(cond.medallions, 0);
    assert!(cond.has_requirements());
}

#[test]
fn test_special_condition_has_requirements() {
    let mut cond = SpecialCondition::default();
    assert!(!cond.has_requirements());

    cond.stones = 1;
    assert!(cond.has_requirements());

    cond = SpecialCondition::default();
    cond.medallions = 1;
    assert!(cond.has_requirements());

    cond = SpecialCondition::default();
    cond.dungeon_rewards = 1;
    assert!(cond.has_requirements());

    cond = SpecialCondition::default();
    cond.skulltulas = 1;
    assert!(cond.has_requirements());

    cond = SpecialCondition::default();
    cond.remains = 1;
    assert!(cond.has_requirements());
}

#[test]
fn test_special_condition_serde_json_roundtrip() {
    let cond = SpecialCondition {
        stones: 3,
        medallions: 6,
        dungeon_rewards: 9,
        skulltulas: 50,
        remains: 4,
    };
    let json = serde_json::to_string(&cond).unwrap();
    let parsed: SpecialCondition = serde_json::from_str(&json).unwrap();
    assert_eq!(parsed.stones, 3);
    assert_eq!(parsed.medallions, 6);
    assert_eq!(parsed.dungeon_rewards, 9);
    assert_eq!(parsed.skulltulas, 50);
    assert_eq!(parsed.remains, 4);
}

#[test]
fn test_special_condition_serde_yaml_roundtrip() {
    let cond = SpecialCondition {
        stones: 3,
        medallions: 6,
        dungeon_rewards: 0,
        skulltulas: 0,
        remains: 4,
    };
    let yaml = serde_yaml::to_string(&cond).unwrap();
    let parsed: SpecialCondition = serde_yaml::from_str(&yaml).unwrap();
    assert_eq!(parsed.stones, 3);
    assert_eq!(parsed.medallions, 6);
    assert_eq!(parsed.remains, 4);
}

#[test]
fn test_special_condition_partial_deserialization() {
    // Test that missing fields default to 0
    let json = r#"{"medallions": 6}"#;
    let parsed: SpecialCondition = serde_json::from_str(json).unwrap();
    assert_eq!(parsed.medallions, 6);
    assert_eq!(parsed.stones, 0);
    assert_eq!(parsed.dungeon_rewards, 0);
}

// === Special Condition Settings Methods Tests ===

#[test]
fn test_special_conditions_operations() {
    let mut settings = RandomizerSettings::new();
    assert!(!settings.has_special_condition("bridge"));
    assert!(settings.get_special_condition("bridge").is_none());
    assert!(settings.bridge_condition().is_none());
    assert_eq!(settings.special_conditions_count(), 0);

    let bridge_cond = SpecialCondition::with_medallions(6);
    settings.set_special_condition("bridge", bridge_cond);

    assert!(settings.has_special_condition("bridge"));
    assert!(settings.get_special_condition("bridge").is_some());
    assert!(settings.bridge_condition().is_some());
    assert_eq!(settings.bridge_condition().unwrap().medallions, 6);
    assert_eq!(settings.special_conditions_count(), 1);

    settings.remove_special_condition("bridge");
    assert!(!settings.has_special_condition("bridge"));
    assert_eq!(settings.special_conditions_count(), 0);
}

#[test]
fn test_special_conditions_iterator() {
    let mut settings = RandomizerSettings::new();
    settings.set_special_condition("bridge", SpecialCondition::with_medallions(6));
    settings.set_special_condition("lacs", SpecialCondition::with_stones(3));

    let conditions: Vec<_> = settings.special_conditions_iter().collect();
    assert_eq!(conditions.len(), 2);
}

#[test]
fn test_special_conditions_serde_roundtrip() {
    let mut settings = RandomizerSettings::new();
    settings.set_special_condition("bridge", SpecialCondition::with_medallions(6));
    settings.set_special_condition("lacs", SpecialCondition::with_stones(3));

    let json = serde_json::to_string(&settings).unwrap();
    let parsed: RandomizerSettings = serde_json::from_str(&json).unwrap();

    assert_eq!(
        parsed.get_special_condition("bridge").unwrap().medallions,
        6
    );
    assert_eq!(parsed.get_special_condition("lacs").unwrap().stones, 3);
}

// === WorldFlags Tests ===

#[test]
fn test_world_flags_default() {
    // Default trait derives default values (false for booleans)
    let flags = WorldFlags::default();
    assert!(!flags.oot_enabled);
    assert!(!flags.mm_enabled);
    assert!(!flags.shared_items);
    assert!(!flags.shared_masks);
}

#[test]
fn test_world_flags_new() {
    // new() uses Default, so also defaults to false
    let flags = WorldFlags::new();
    assert!(!flags.is_oot_enabled());
    assert!(!flags.is_mm_enabled());
}

#[test]
fn test_world_flags_accessors() {
    let mut flags = WorldFlags::default();
    // Start with default values (false)
    assert!(!flags.is_oot_enabled());
    assert!(!flags.is_mm_enabled());

    // Set to true and verify
    flags.oot_enabled = true;
    assert!(flags.is_oot_enabled());

    flags.mm_enabled = true;
    assert!(flags.is_mm_enabled());
}

#[test]
fn test_world_flags_serde_json_roundtrip() {
    let flags = WorldFlags {
        oot_enabled: true,
        mm_enabled: false,
        shared_items: true,
        shared_masks: true,
    };
    let json = serde_json::to_string(&flags).unwrap();
    let parsed: WorldFlags = serde_json::from_str(&json).unwrap();
    assert!(parsed.oot_enabled);
    assert!(!parsed.mm_enabled);
    assert!(parsed.shared_items);
    assert!(parsed.shared_masks);
}

#[test]
fn test_world_flags_serde_yaml_roundtrip() {
    let flags = WorldFlags {
        oot_enabled: false,
        mm_enabled: true,
        shared_items: false,
        shared_masks: true,
    };
    let yaml = serde_yaml::to_string(&flags).unwrap();
    let parsed: WorldFlags = serde_yaml::from_str(&yaml).unwrap();
    assert!(!parsed.oot_enabled);
    assert!(parsed.mm_enabled);
    assert!(!parsed.shared_items);
    assert!(parsed.shared_masks);
}

#[test]
fn test_world_flags_partial_deserialization() {
    // Test that missing fields use default values
    let json = r#"{"sharedItems": true}"#;
    let parsed: WorldFlags = serde_json::from_str(json).unwrap();
    assert!(parsed.oot_enabled); // defaults to true
    assert!(parsed.mm_enabled); // defaults to true
    assert!(parsed.shared_items);
    assert!(!parsed.shared_masks); // defaults to false
}
