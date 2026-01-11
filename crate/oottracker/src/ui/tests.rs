use super::*;
use crate::{
    knowledge::ProgressionMode,
    save::{AdultTradeItem, DungeonItems, Hookshot, MagicCapacity, QuestItems},
    ModelState,
};
use ootr::model::{DungeonReward, DungeonRewardLocation, MainDungeon, Medallion, Stone};

// ==========================================================================
// AccessibilityStatus Tests
// ==========================================================================

#[test]
fn test_accessibility_status_default() {
    let status = AccessibilityStatus::default();
    assert_eq!(status, AccessibilityStatus::Unknown);
}

#[test]
fn test_accessibility_status_icon() {
    assert_eq!(AccessibilityStatus::Accessible.icon(), "✓");
    assert_eq!(AccessibilityStatus::Inaccessible.icon(), "✗");
    assert_eq!(AccessibilityStatus::Checked.icon(), "●");
    assert_eq!(AccessibilityStatus::Unknown.icon(), "?");
}

#[test]
fn test_accessibility_status_description() {
    assert_eq!(AccessibilityStatus::Accessible.description(), "Accessible");
    assert_eq!(
        AccessibilityStatus::Inaccessible.description(),
        "Not yet accessible"
    );
    assert_eq!(
        AccessibilityStatus::Checked.description(),
        "Already checked"
    );
    assert_eq!(AccessibilityStatus::Unknown.description(), "Unknown");
}

#[test]
fn test_accessibility_status_from_check_status() {
    use crate::checks::CheckStatus;

    assert_eq!(
        AccessibilityStatus::from(CheckStatus::Checked),
        AccessibilityStatus::Checked
    );
    assert_eq!(
        AccessibilityStatus::from(CheckStatus::Reachable),
        AccessibilityStatus::Accessible
    );
    assert_eq!(
        AccessibilityStatus::from(CheckStatus::NotYetReachable),
        AccessibilityStatus::Inaccessible
    );
}

// ==========================================================================
// AccessibilitySummary Tests
// ==========================================================================

#[test]
fn test_accessibility_summary_new() {
    let summary = AccessibilitySummary::new();
    assert_eq!(summary.accessible, 0);
    assert_eq!(summary.inaccessible, 0);
    assert_eq!(summary.checked, 0);
    assert_eq!(summary.unknown, 0);
}

#[test]
fn test_accessibility_summary_default() {
    let summary = AccessibilitySummary::default();
    assert_eq!(summary.total(), 0);
}

#[test]
fn test_accessibility_summary_add() {
    let mut summary = AccessibilitySummary::new();

    summary.add(AccessibilityStatus::Accessible);
    assert_eq!(summary.accessible, 1);
    assert_eq!(summary.total(), 1);

    summary.add(AccessibilityStatus::Inaccessible);
    assert_eq!(summary.inaccessible, 1);
    assert_eq!(summary.total(), 2);

    summary.add(AccessibilityStatus::Checked);
    assert_eq!(summary.checked, 1);
    assert_eq!(summary.total(), 3);

    summary.add(AccessibilityStatus::Unknown);
    assert_eq!(summary.unknown, 1);
    assert_eq!(summary.total(), 4);
}

#[test]
fn test_accessibility_summary_total() {
    let mut summary = AccessibilitySummary::new();
    summary.accessible = 5;
    summary.inaccessible = 3;
    summary.checked = 2;
    summary.unknown = 1;
    assert_eq!(summary.total(), 11);
}

#[test]
fn test_accessibility_summary_display() {
    let mut summary = AccessibilitySummary::new();
    summary.accessible = 5;
    summary.inaccessible = 3;
    summary.checked = 2;
    summary.unknown = 1;
    let display = format!("{}", summary);
    assert_eq!(display, "✓5 / ✗3 / ●2 / ?1");
}

// ==========================================================================
// LocationAccessibility Tests
// ==========================================================================

#[test]
fn test_location_accessibility_new() {
    let loc = LocationAccessibility::new("Test Location", AccessibilityStatus::Accessible);
    assert_eq!(loc.name, "Test Location");
    assert_eq!(loc.status, AccessibilityStatus::Accessible);
}

#[test]
fn test_location_accessibility_new_with_string() {
    let name = String::from("Dynamic Name");
    let loc = LocationAccessibility::new(name, AccessibilityStatus::Checked);
    assert_eq!(loc.name, "Dynamic Name");
    assert_eq!(loc.status, AccessibilityStatus::Checked);
}

// ==========================================================================
// LocationAccessibilityList Tests
// ==========================================================================

#[test]
fn test_location_accessibility_list_new() {
    let list = LocationAccessibilityList::new();
    assert!(list.locations.is_empty());
    assert_eq!(list.summary.total(), 0);
}

#[test]
fn test_location_accessibility_list_default() {
    let list = LocationAccessibilityList::default();
    assert!(list.locations.is_empty());
}

#[test]
fn test_location_accessibility_list_add() {
    let mut list = LocationAccessibilityList::new();

    list.add("Location 1", AccessibilityStatus::Accessible);
    list.add("Location 2", AccessibilityStatus::Inaccessible);
    list.add("Location 3", AccessibilityStatus::Accessible);

    assert_eq!(list.locations.len(), 3);
    assert_eq!(list.summary.accessible, 2);
    assert_eq!(list.summary.inaccessible, 1);
}

#[test]
fn test_location_accessibility_list_accessible_filter() {
    let mut list = LocationAccessibilityList::new();
    list.add("Accessible 1", AccessibilityStatus::Accessible);
    list.add("Inaccessible 1", AccessibilityStatus::Inaccessible);
    list.add("Accessible 2", AccessibilityStatus::Accessible);

    let accessible: Vec<_> = list.accessible().collect();
    assert_eq!(accessible.len(), 2);
    assert_eq!(accessible[0].name, "Accessible 1");
    assert_eq!(accessible[1].name, "Accessible 2");
}

#[test]
fn test_location_accessibility_list_inaccessible_filter() {
    let mut list = LocationAccessibilityList::new();
    list.add("Accessible 1", AccessibilityStatus::Accessible);
    list.add("Inaccessible 1", AccessibilityStatus::Inaccessible);
    list.add("Inaccessible 2", AccessibilityStatus::Inaccessible);

    let inaccessible: Vec<_> = list.inaccessible().collect();
    assert_eq!(inaccessible.len(), 2);
    assert_eq!(inaccessible[0].name, "Inaccessible 1");
    assert_eq!(inaccessible[1].name, "Inaccessible 2");
}

// ==========================================================================
// CellStyle Tests
// ==========================================================================

#[test]
fn test_cell_style_equality() {
    assert_eq!(CellStyle::Normal, CellStyle::Normal);
    assert_eq!(CellStyle::Dimmed, CellStyle::Dimmed);
    assert_eq!(CellStyle::LeftDimmed, CellStyle::LeftDimmed);
    assert_eq!(CellStyle::RightDimmed, CellStyle::RightDimmed);
    assert_ne!(CellStyle::Normal, CellStyle::Dimmed);
}

#[test]
fn test_cell_style_clone() {
    let style = CellStyle::LeftDimmed;
    let cloned = style;
    assert_eq!(style, cloned);
}

// ==========================================================================
// CellOverlay Tests
// ==========================================================================

#[test]
fn test_cell_overlay_none() {
    let overlay = CellOverlay::None;
    assert_eq!(overlay, CellOverlay::None);
}

#[test]
fn test_cell_overlay_count() {
    let overlay = CellOverlay::Count {
        count: 5,
        max: 100,
        count_img: ImageInfo::new("skulls"),
    };
    if let CellOverlay::Count { count, max, .. } = overlay {
        assert_eq!(count, 5);
        assert_eq!(max, 100);
    } else {
        panic!("Expected Count variant");
    }
}

#[test]
fn test_cell_overlay_image() {
    let overlay = CellOverlay::Image(ImageInfo::new("lens"));
    if let CellOverlay::Image(img) = &overlay {
        assert_eq!(img.name, "lens");
    } else {
        panic!("Expected Image variant");
    }
}

#[test]
fn test_cell_overlay_location() {
    let overlay = CellOverlay::Location {
        loc: ImageInfo::new("forest_text"),
        style: LocationStyle::Normal,
    };
    if let CellOverlay::Location { style, .. } = overlay {
        assert_eq!(style, LocationStyle::Normal);
    } else {
        panic!("Expected Location variant");
    }
}

// ==========================================================================
// LocationStyle Tests
// ==========================================================================

#[test]
fn test_location_style_equality() {
    assert_eq!(LocationStyle::Normal, LocationStyle::Normal);
    assert_eq!(LocationStyle::Dimmed, LocationStyle::Dimmed);
    assert_eq!(LocationStyle::Mq, LocationStyle::Mq);
    assert_ne!(LocationStyle::Normal, LocationStyle::Mq);
}

// ==========================================================================
// CellRender Tests
// ==========================================================================

#[test]
fn test_cell_render_new() {
    let render = CellRender::new(ImageInfo::new("test"), CellStyle::Normal, CellOverlay::None);
    assert_eq!(render.style, CellStyle::Normal);
    assert_eq!(render.overlay, CellOverlay::None);
    assert!(render.accessibility.is_none());
}

#[test]
fn test_cell_render_with_accessibility() {
    let render = CellRender::new(ImageInfo::new("test"), CellStyle::Normal, CellOverlay::None)
        .with_accessibility(AccessibilityStatus::Accessible);

    assert_eq!(render.accessibility, Some(AccessibilityStatus::Accessible));
}

#[test]
fn test_cell_render_equality() {
    let render1 = CellRender::new(ImageInfo::new("test"), CellStyle::Normal, CellOverlay::None);
    let render2 = CellRender::new(ImageInfo::new("test"), CellStyle::Normal, CellOverlay::None);
    assert_eq!(render1, render2);
}

#[test]
fn test_cell_render_inequality_style() {
    let render1 = CellRender::new(ImageInfo::new("test"), CellStyle::Normal, CellOverlay::None);
    let render2 = CellRender::new(ImageInfo::new("test"), CellStyle::Dimmed, CellOverlay::None);
    assert_ne!(render1, render2);
}

// ==========================================================================
// ImageInfo Tests
// ==========================================================================

#[test]
fn test_image_info_new() {
    let img = ImageInfo::new("sword");
    assert_eq!(img.dir, ImageDir::Xopar);
    assert_eq!(img.name, "sword");
}

#[test]
fn test_image_info_extra() {
    let img = ImageInfo::extra("boss_key");
    assert_eq!(img.dir, ImageDir::Extra);
    assert_eq!(img.name, "boss_key");
}

#[test]
fn test_image_info_mm() {
    let img = ImageInfo::mm("mask");
    assert_eq!(img.dir, ImageDir::Mm);
    assert_eq!(img.name, "mask");
}

#[test]
fn test_image_info_equality() {
    let img1 = ImageInfo::new("sword");
    let img2 = ImageInfo::new("sword");
    assert_eq!(img1, img2);
}

#[test]
fn test_image_info_inequality_name() {
    let img1 = ImageInfo::new("sword");
    let img2 = ImageInfo::new("shield");
    assert_ne!(img1, img2);
}

#[test]
fn test_image_info_inequality_dir() {
    let img1 = ImageInfo::new("item");
    let img2 = ImageInfo::extra("item");
    assert_ne!(img1, img2);
}

// ==========================================================================
// ImageDir Tests
// ==========================================================================

#[test]
fn test_image_dir_to_string_normal() {
    assert_eq!(
        ImageDir::Xopar.to_string(ImageDirContext::Normal),
        "xopar-images"
    );
    assert_eq!(
        ImageDir::Extra.to_string(ImageDirContext::Normal),
        "extra-images"
    );
    assert_eq!(ImageDir::Mm.to_string(ImageDirContext::Normal), "mm-images");
}

#[test]
fn test_image_dir_to_string_dimmed() {
    assert_eq!(
        ImageDir::Xopar.to_string(ImageDirContext::Dimmed),
        "xopar-images-dimmed"
    );
    assert_eq!(
        ImageDir::Extra.to_string(ImageDirContext::Dimmed),
        "extra-images-dimmed"
    );
    assert_eq!(
        ImageDir::Mm.to_string(ImageDirContext::Dimmed),
        "mm-images-dimmed"
    );
}

#[test]
fn test_image_dir_to_string_count() {
    assert_eq!(
        ImageDir::Xopar.to_string(ImageDirContext::Count(5)),
        "xopar-images-count"
    );
    assert_eq!(
        ImageDir::Extra.to_string(ImageDirContext::Count(10)),
        "extra-images-count"
    );
}

#[test]
fn test_image_dir_to_string_overlay_only() {
    assert_eq!(
        ImageDir::Xopar.to_string(ImageDirContext::OverlayOnly),
        "xopar-overlays"
    );
    assert_eq!(
        ImageDir::Extra.to_string(ImageDirContext::OverlayOnly),
        "extra-overlays"
    );
    assert_eq!(
        ImageDir::Mm.to_string(ImageDirContext::OverlayOnly),
        "mm-overlays"
    );
}

// ==========================================================================
// ElementOrder Tests
// ==========================================================================

#[test]
fn test_element_order_light_shadow_spirit() {
    let order = ElementOrder::LightShadowSpirit;
    let meds: Vec<_> = order.into_iter().collect();
    assert_eq!(meds.len(), 6);
    assert_eq!(meds[0], Medallion::Light);
    assert_eq!(meds[4], Medallion::Shadow);
    assert_eq!(meds[5], Medallion::Spirit);
}

#[test]
fn test_element_order_light_spirit_shadow() {
    let order = ElementOrder::LightSpiritShadow;
    let meds: Vec<_> = order.into_iter().collect();
    assert_eq!(meds[0], Medallion::Light);
    assert_eq!(meds[4], Medallion::Spirit);
    assert_eq!(meds[5], Medallion::Shadow);
}

#[test]
fn test_element_order_shadow_spirit_light() {
    let order = ElementOrder::ShadowSpiritLight;
    let meds: Vec<_> = order.into_iter().collect();
    assert_eq!(meds[0], Medallion::Forest);
    assert_eq!(meds[3], Medallion::Shadow);
    assert_eq!(meds[5], Medallion::Light);
}

#[test]
fn test_element_order_spirit_shadow_light() {
    let order = ElementOrder::SpiritShadowLight;
    let meds: Vec<_> = order.into_iter().collect();
    assert_eq!(meds[0], Medallion::Forest);
    assert_eq!(meds[3], Medallion::Spirit);
    assert_eq!(meds[5], Medallion::Light);
}

#[test]
fn test_element_order_display() {
    assert_eq!(
        format!("{}", ElementOrder::LightShadowSpirit),
        "Light first, Shadow before Spirit"
    );
    assert_eq!(
        format!("{}", ElementOrder::LightSpiritShadow),
        "Light first, Spirit before Shadow"
    );
    assert_eq!(
        format!("{}", ElementOrder::ShadowSpiritLight),
        "Shadow before Spirit, Light last"
    );
    assert_eq!(
        format!("{}", ElementOrder::SpiritShadowLight),
        "Spirit before Shadow, Light last"
    );
}

// ==========================================================================
// LayoutPreference Tests
// ==========================================================================

#[test]
fn test_layout_preference_default() {
    let pref = LayoutPreference::default();
    assert_eq!(pref, LayoutPreference::Oot);
}

#[test]
fn test_layout_preference_display() {
    assert_eq!(format!("{}", LayoutPreference::Oot), "Ocarina of Time");
    assert_eq!(format!("{}", LayoutPreference::Mm), "Majora's Mask");
    assert_eq!(format!("{}", LayoutPreference::Combo), "Combo (OoT + MM)");
    assert_eq!(
        format!("{}", LayoutPreference::DungeonItems),
        "OoT Dungeon Items"
    );
    assert_eq!(
        format!("{}", LayoutPreference::MmDungeonItems),
        "MM Dungeon Items"
    );
    assert_eq!(
        format!("{}", LayoutPreference::MmStrayFairies),
        "MM Stray Fairies"
    );
}

// ==========================================================================
// TrackerCellId Tests
// ==========================================================================

#[test]
fn test_tracker_cell_id_from_medallion() {
    assert_eq!(
        TrackerCellId::from(Medallion::Light),
        TrackerCellId::LightMedallion
    );
    assert_eq!(
        TrackerCellId::from(Medallion::Forest),
        TrackerCellId::ForestMedallion
    );
    assert_eq!(
        TrackerCellId::from(Medallion::Fire),
        TrackerCellId::FireMedallion
    );
    assert_eq!(
        TrackerCellId::from(Medallion::Water),
        TrackerCellId::WaterMedallion
    );
    assert_eq!(
        TrackerCellId::from(Medallion::Shadow),
        TrackerCellId::ShadowMedallion
    );
    assert_eq!(
        TrackerCellId::from(Medallion::Spirit),
        TrackerCellId::SpiritMedallion
    );
}

#[test]
fn test_tracker_cell_id_med_location() {
    assert_eq!(
        TrackerCellId::med_location(Medallion::Light),
        TrackerCellId::LightMedallionLocation
    );
    assert_eq!(
        TrackerCellId::med_location(Medallion::Forest),
        TrackerCellId::ForestMedallionLocation
    );
}

#[test]
fn test_tracker_cell_id_warp_song() {
    assert_eq!(
        TrackerCellId::warp_song(Medallion::Light),
        TrackerCellId::Prelude
    );
    assert_eq!(
        TrackerCellId::warp_song(Medallion::Forest),
        TrackerCellId::Minuet
    );
    assert_eq!(
        TrackerCellId::warp_song(Medallion::Fire),
        TrackerCellId::Bolero
    );
    assert_eq!(
        TrackerCellId::warp_song(Medallion::Water),
        TrackerCellId::Serenade
    );
    assert_eq!(
        TrackerCellId::warp_song(Medallion::Shadow),
        TrackerCellId::Nocturne
    );
    assert_eq!(
        TrackerCellId::warp_song(Medallion::Spirit),
        TrackerCellId::Requiem
    );
}

// ==========================================================================
// TrackerCellKind Render Tests - Cell Rendering Logic
// ==========================================================================

#[test]
fn test_medallion_render_inactive() {
    let state = ModelState::default();
    let cell = TrackerCellId::ForestMedallion.kind();
    let render = cell.render(&state);

    assert_eq!(render.style, CellStyle::Dimmed);
    assert_eq!(render.overlay, CellOverlay::None);
    assert!(render.img.name.contains("forest"));
}

#[test]
fn test_medallion_render_active() {
    let mut state = ModelState::default();
    state
        .ram
        .save
        .quest_items
        .toggle(QuestItems::FOREST_MEDALLION);

    let cell = TrackerCellId::ForestMedallion.kind();
    let render = cell.render(&state);

    assert_eq!(render.style, CellStyle::Normal);
}

#[test]
fn test_stone_render_inactive() {
    let state = ModelState::default();
    let cell = TrackerCellId::KokiriEmerald.kind();
    let render = cell.render(&state);

    assert_eq!(render.style, CellStyle::Dimmed);
}

#[test]
fn test_stone_render_active() {
    let mut state = ModelState::default();
    state
        .ram
        .save
        .quest_items
        .toggle(QuestItems::KOKIRI_EMERALD);

    let cell = TrackerCellId::KokiriEmerald.kind();
    let render = cell.render(&state);

    assert_eq!(render.style, CellStyle::Normal);
}

#[test]
fn test_count_cell_render_zero() {
    let state = ModelState::default();
    let cell = TrackerCellId::Skulltula.kind();
    let render = cell.render(&state);

    assert_eq!(render.style, CellStyle::Dimmed);
    assert_eq!(render.overlay, CellOverlay::None);
}

#[test]
fn test_count_cell_render_nonzero() {
    let mut state = ModelState::default();
    state.ram.save.skull_tokens = 25;

    let cell = TrackerCellId::Skulltula.kind();
    let render = cell.render(&state);

    assert_eq!(render.style, CellStyle::Normal);
    if let CellOverlay::CountWithMax { count, max, .. } = render.overlay {
        assert_eq!(count, 25);
        assert_eq!(max, 100);
    } else {
        panic!("Expected CountWithMax overlay");
    }
}

#[test]
fn test_count_with_max_overlay_construction() {
    let overlay = CellOverlay::CountWithMax {
        count: 2,
        max: 4,
        count_img: ImageInfo::new("bottle"),
    };
    if let CellOverlay::CountWithMax { count, max, .. } = overlay {
        assert_eq!(count, 2);
        assert_eq!(max, 4);
    } else {
        panic!("Expected CountWithMax variant");
    }
}

#[test]
fn test_num_bottles_cell_render_zero() {
    let state = ModelState::default();
    let cell = TrackerCellId::NumBottles.kind();
    let render = cell.render(&state);

    assert_eq!(render.style, CellStyle::Dimmed);
    assert_eq!(render.overlay, CellOverlay::None);
}

#[test]
fn test_num_bottles_cell_render_nonzero() {
    let mut state = ModelState::default();
    state.ram.save.inv.set_emptiable_bottles(2);

    let cell = TrackerCellId::NumBottles.kind();
    let render = cell.render(&state);

    assert_eq!(render.style, CellStyle::Normal);
    if let CellOverlay::CountWithMax { count, max, .. } = render.overlay {
        assert_eq!(count, 2);
        assert_eq!(max, 4);
    } else {
        panic!("Expected CountWithMax overlay for NumBottles");
    }
}

#[test]
fn test_num_bottles_cell_render_max() {
    let mut state = ModelState::default();
    state.ram.save.inv.set_emptiable_bottles(4);

    let cell = TrackerCellId::NumBottles.kind();
    let render = cell.render(&state);

    assert_eq!(render.style, CellStyle::Normal);
    if let CellOverlay::CountWithMax { count, max, .. } = render.overlay {
        assert_eq!(count, 4);
        assert_eq!(max, 4);
    } else {
        panic!("Expected CountWithMax overlay for NumBottles at max");
    }
}

#[test]
fn test_num_bottles_cell_render_with_custom_max() {
    // Set custom max bottles (for shared bottle randomizer settings)
    let mut state = ModelState {
        max_bottles: 2,
        ..Default::default()
    };
    state.ram.save.inv.set_emptiable_bottles(2);

    let cell = TrackerCellId::NumBottles.kind();
    let render = cell.render(&state);

    assert_eq!(render.style, CellStyle::Normal);
    if let CellOverlay::CountWithMax { count, max, .. } = render.overlay {
        assert_eq!(count, 2);
        assert_eq!(max, 2); // Max should now be 2, not 4
    } else {
        panic!("Expected CountWithMax overlay for NumBottles with custom max");
    }
}

#[test]
fn test_go_mode_render_normal() {
    let state = ModelState::default();
    let cell = TrackerCellId::GoMode.kind();
    let render = cell.render(&state);

    // Default progression mode is Normal, so GoMode should be dimmed
    assert_eq!(render.style, CellStyle::Dimmed);
}

#[test]
fn test_go_mode_render_active() {
    let mut state = ModelState::default();
    state.knowledge.progression_mode = ProgressionMode::Go;

    let cell = TrackerCellId::GoMode.kind();
    let render = cell.render(&state);

    assert_eq!(render.style, CellStyle::Normal);
}

#[test]
fn test_magic_lens_render_no_magic() {
    let state = ModelState::default();
    let cell = TrackerCellId::MagicLens.kind();
    let render = cell.render(&state);

    assert_eq!(render.style, CellStyle::Dimmed);
    assert_eq!(render.overlay, CellOverlay::None);
}

#[test]
fn test_magic_lens_render_with_magic() {
    let mut state = ModelState::default();
    state.ram.save.magic = MagicCapacity::Small;

    let cell = TrackerCellId::MagicLens.kind();
    let render = cell.render(&state);

    assert_eq!(render.style, CellStyle::Normal);
}

#[test]
fn test_magic_lens_render_with_lens() {
    let mut state = ModelState::default();
    state.ram.save.magic = MagicCapacity::Small;
    state.ram.save.inv.lens = true;

    let cell = TrackerCellId::MagicLens.kind();
    let render = cell.render(&state);

    if let CellOverlay::Image(img) = &render.overlay {
        assert_eq!(img.name, "lens");
    } else {
        panic!("Expected Image overlay with lens");
    }
}

#[test]
fn test_spells_render_inactive() {
    let state = ModelState::default();
    let cell = TrackerCellId::Spells.kind();
    let render = cell.render(&state);

    assert_eq!(render.style, CellStyle::Dimmed);
}

#[test]
fn test_spells_render_dins_only() {
    let mut state = ModelState::default();
    state.ram.save.inv.dins_fire = true;

    let cell = TrackerCellId::Spells.kind();
    let render = cell.render(&state);

    // With only Din's Fire, left part is active
    assert_eq!(render.style, CellStyle::Normal);
}

// ==========================================================================
// TrackerCellKind Click Tests - State Transitions
// ==========================================================================

#[test]
fn test_medallion_click_toggles() {
    let mut state = ModelState::default();
    let cell = TrackerCellId::ForestMedallion.kind();

    // Should not have medallion initially
    assert!(!state
        .ram
        .save
        .quest_items
        .contains(QuestItems::FOREST_MEDALLION));

    // Click to toggle on
    cell.click(&mut state);
    assert!(state
        .ram
        .save
        .quest_items
        .contains(QuestItems::FOREST_MEDALLION));

    // Click to toggle off
    cell.click(&mut state);
    assert!(!state
        .ram
        .save
        .quest_items
        .contains(QuestItems::FOREST_MEDALLION));
}

#[test]
fn test_stone_click_toggles() {
    let mut state = ModelState::default();
    let cell = TrackerCellId::KokiriEmerald.kind();

    assert!(!state
        .ram
        .save
        .quest_items
        .contains(QuestItems::KOKIRI_EMERALD));

    cell.click(&mut state);
    assert!(state
        .ram
        .save
        .quest_items
        .contains(QuestItems::KOKIRI_EMERALD));

    cell.click(&mut state);
    assert!(!state
        .ram
        .save
        .quest_items
        .contains(QuestItems::KOKIRI_EMERALD));
}

#[test]
fn test_count_click_increments() {
    let mut state = ModelState::default();
    let cell = TrackerCellId::Skulltula.kind();

    assert_eq!(state.ram.save.skull_tokens, 0);

    cell.click(&mut state);
    assert_eq!(state.ram.save.skull_tokens, 1);

    cell.click(&mut state);
    assert_eq!(state.ram.save.skull_tokens, 2);
}

#[test]
fn test_count_click_wraps_at_max() {
    let mut state = ModelState::default();
    state.ram.save.skull_tokens = 100;

    let cell = TrackerCellId::Skulltula.kind();
    cell.click(&mut state);

    assert_eq!(state.ram.save.skull_tokens, 0);
}

#[test]
fn test_go_mode_click_toggles() {
    let mut state = ModelState::default();
    let cell = TrackerCellId::GoMode.kind();

    assert_eq!(state.knowledge.progression_mode, ProgressionMode::Normal);

    cell.click(&mut state);
    assert_eq!(state.knowledge.progression_mode, ProgressionMode::Go);

    cell.click(&mut state);
    assert_eq!(state.knowledge.progression_mode, ProgressionMode::Normal);
}

#[test]
fn test_go_bk_click_cycles() {
    let mut state = ModelState::default();
    let cell = TrackerCellId::GoBk.kind();

    assert_eq!(state.knowledge.progression_mode, ProgressionMode::Normal);

    cell.click(&mut state);
    assert_eq!(state.knowledge.progression_mode, ProgressionMode::Go);

    cell.click(&mut state);
    assert_eq!(state.knowledge.progression_mode, ProgressionMode::Bk);

    cell.click(&mut state);
    assert_eq!(state.knowledge.progression_mode, ProgressionMode::Done);

    cell.click(&mut state);
    assert_eq!(state.knowledge.progression_mode, ProgressionMode::Normal);
}

#[test]
fn test_magic_lens_click_cycles() {
    let mut state = ModelState::default();
    let cell = TrackerCellId::MagicLens.kind();

    // Start with no magic
    assert_eq!(state.ram.save.magic, MagicCapacity::None);
    assert!(!state.ram.save.inv.lens);

    // First click: gain small magic
    cell.click(&mut state);
    assert_eq!(state.ram.save.magic, MagicCapacity::Small);
    assert!(!state.ram.save.inv.lens);

    // Second click: lose magic, gain lens
    cell.click(&mut state);
    assert_eq!(state.ram.save.magic, MagicCapacity::None);
    assert!(state.ram.save.inv.lens);

    // Third click: gain magic again
    cell.click(&mut state);
    assert_eq!(state.ram.save.magic, MagicCapacity::Small);
    assert!(state.ram.save.inv.lens);
}

#[test]
fn test_spells_click_cycles() {
    let mut state = ModelState::default();
    let cell = TrackerCellId::Spells.kind();

    // Start with nothing
    assert!(!state.ram.save.inv.dins_fire);
    assert!(!state.ram.save.inv.farores_wind);

    // First click: gain Din's Fire
    cell.click(&mut state);
    assert!(state.ram.save.inv.dins_fire);
    assert!(!state.ram.save.inv.farores_wind);

    // Second click: lose Din's, gain Farore's
    cell.click(&mut state);
    assert!(!state.ram.save.inv.dins_fire);
    assert!(state.ram.save.inv.farores_wind);

    // Third click: gain Din's
    cell.click(&mut state);
    assert!(state.ram.save.inv.dins_fire);
    assert!(state.ram.save.inv.farores_wind);
}

#[test]
fn test_sequence_cell_increment() {
    let mut state = ModelState::default();
    let cell = TrackerCellId::AdultTrade.kind();

    // Start with no trade item
    assert_eq!(state.ram.save.inv.adult_trade_item, AdultTradeItem::None);

    // Click to increment
    cell.click(&mut state);
    assert_eq!(
        state.ram.save.inv.adult_trade_item,
        AdultTradeItem::PocketEgg
    );

    cell.click(&mut state);
    assert_eq!(
        state.ram.save.inv.adult_trade_item,
        AdultTradeItem::PocketCucco
    );
}

#[test]
fn test_small_keys_click_increments() {
    let mut state = ModelState::default();
    let cell = TrackerCellId::ForestSmallKeys.kind();

    let initial = state.ram.save.small_keys.forest_temple;
    cell.click(&mut state);
    assert_eq!(state.ram.save.small_keys.forest_temple, initial + 1);
}

#[test]
fn test_small_keys_click_wraps() {
    let mut state = ModelState::default();
    // Forest Temple has max 6 keys (MQ has 6, vanilla has 5)
    state.ram.save.small_keys.forest_temple = 6;

    let cell = TrackerCellId::ForestSmallKeys.kind();
    cell.click(&mut state);

    assert_eq!(state.ram.save.small_keys.forest_temple, 0);
}

// ==========================================================================
// DungeonRewardLocationExt Tests
// ==========================================================================

#[test]
fn test_dungeon_reward_location_increment() {
    use std::collections::HashMap;

    let mut locations: HashMap<DungeonReward, DungeonRewardLocation> = HashMap::new();
    let reward = DungeonReward::Medallion(Medallion::Forest);

    // Start with no location
    assert!(!locations.contains_key(&reward));

    // Increment to first dungeon
    locations.increment(reward);
    assert_eq!(
        locations.get(&reward),
        Some(&DungeonRewardLocation::Dungeon(MainDungeon::DekuTree))
    );

    // Continue incrementing
    locations.increment(reward);
    assert_eq!(
        locations.get(&reward),
        Some(&DungeonRewardLocation::Dungeon(MainDungeon::DodongosCavern))
    );
}

#[test]
fn test_dungeon_reward_location_decrement() {
    use std::collections::HashMap;

    let mut locations: HashMap<DungeonReward, DungeonRewardLocation> = HashMap::new();
    let reward = DungeonReward::Stone(Stone::KokiriEmerald);

    // Start with no location, decrement wraps to LinksPocket
    locations.decrement(reward);
    assert_eq!(
        locations.get(&reward),
        Some(&DungeonRewardLocation::LinksPocket)
    );

    // Decrement again
    locations.decrement(reward);
    assert_eq!(
        locations.get(&reward),
        Some(&DungeonRewardLocation::Dungeon(MainDungeon::SpiritTemple))
    );
}

#[test]
fn test_dungeon_reward_location_full_cycle() {
    use std::collections::HashMap;

    let mut locations: HashMap<DungeonReward, DungeonRewardLocation> = HashMap::new();
    let reward = DungeonReward::Medallion(Medallion::Light);

    // Cycle through all locations
    locations.increment(reward); // DekuTree
    locations.increment(reward); // DodongosCavern
    locations.increment(reward); // JabuJabu
    locations.increment(reward); // ForestTemple
    locations.increment(reward); // FireTemple
    locations.increment(reward); // WaterTemple
    locations.increment(reward); // ShadowTemple
    locations.increment(reward); // SpiritTemple
    locations.increment(reward); // LinksPocket
    assert_eq!(
        locations.get(&reward),
        Some(&DungeonRewardLocation::LinksPocket)
    );

    // One more increment removes the entry
    locations.increment(reward);
    assert!(!locations.contains_key(&reward));
}

// ==========================================================================
// Config Tests
// ==========================================================================

#[test]
fn test_config_default() {
    let config = Config::default();
    assert_eq!(config.med_order, ElementOrder::LightShadowSpirit);
    assert_eq!(config.warp_song_order, ElementOrder::SpiritShadowLight);
    assert_eq!(config.layout_preference, LayoutPreference::Oot);
    assert_eq!(config.version, CONFIG_VERSION);
}

// ==========================================================================
// TrackerLayout Tests
// ==========================================================================

#[test]
fn test_tracker_layout_default() {
    let layout = TrackerLayout::default();
    // Default layout should exist
    assert!(matches!(layout, TrackerLayout::Default { .. }));
}

#[test]
fn test_tracker_layout_from_config() {
    let config = Config::default();
    let layout = TrackerLayout::from(&config);
    assert!(matches!(layout, TrackerLayout::Default { .. }));
}

#[test]
fn test_tracker_layout_column_count_default() {
    let layout = TrackerLayout::default();
    // Default layout has 6 columns
    assert_eq!(layout.column_count(), 6);
}

#[test]
fn test_tracker_layout_column_count_various() {
    // MultiworldExpanded uses 4 columns
    assert_eq!(TrackerLayout::MultiworldExpanded.column_count(), 4);
    // MultiworldCollapsed uses 10 columns
    assert_eq!(TrackerLayout::MultiworldCollapsed.column_count(), 10);
    // TriforcePieces uses 1 column
    assert_eq!(TrackerLayout::TriforcePieces.column_count(), 1);
    // Combo uses 12 columns
    assert_eq!(TrackerLayout::Combo.column_count(), 12);
}

#[test]
fn test_tracker_layout_row_count_default() {
    let layout = TrackerLayout::default();
    // Default layout has multiple rows with varying positions
    let row_count = layout.row_count();
    assert!(row_count > 0, "Default layout should have rows");
}

#[test]
fn test_tracker_layout_row_count_various() {
    // TriforcePieces has 1 row
    assert_eq!(TrackerLayout::TriforcePieces.row_count(), 1);
    // MmBossRemains has 1 row (4 cells in 4 columns)
    assert_eq!(TrackerLayout::MmBossRemains.row_count(), 1);
    // MmSongs has 2 rows (10 cells in 5 columns)
    assert_eq!(TrackerLayout::MmSongs.row_count(), 2);
}

#[test]
fn test_tracker_layout_pixel_dimensions_default() {
    let layout = TrackerLayout::default();
    let (width, height) = layout.pixel_dimensions();
    // Default layout is 6 columns wide (6 * 60 = 360)
    assert_eq!(width, 360);
    // Height depends on number of rows and their positioning
    assert!(height > 0, "Default layout should have non-zero height");
}

#[test]
fn test_tracker_layout_pixel_dimensions_various() {
    // TriforcePieces: 1 column (1 * 60 = 60), 1 row
    let (width, height) = TrackerLayout::TriforcePieces.pixel_dimensions();
    assert_eq!(width, 60);
    assert_eq!(height, 60);

    // MultiworldExpanded: 4 columns (4 * 60 = 240)
    let (width, _) = TrackerLayout::MultiworldExpanded.pixel_dimensions();
    assert_eq!(width, 240);

    // Combo: 12 columns (12 * 60 = 720)
    let (width, _) = TrackerLayout::Combo.pixel_dimensions();
    assert_eq!(width, 720);
}

#[test]
fn test_tracker_layout_dimensions_consistency() {
    // Test that all layout variants have valid dimensions
    let layouts = [
        TrackerLayout::default(),
        TrackerLayout::MultiworldExpanded,
        TrackerLayout::MultiworldCollapsed,
        TrackerLayout::MultiworldEdit,
        TrackerLayout::RslLeft,
        TrackerLayout::RslRight,
        TrackerLayout::RslEdit,
        TrackerLayout::Rsl3Player,
        TrackerLayout::TsgMainWithRewardLocations,
        TrackerLayout::TsgMainWithRewardLocationsEdit,
        TrackerLayout::TriforcePieces,
        TrackerLayout::MmDefault,
        TrackerLayout::MmMasks,
        TrackerLayout::MmBossRemains,
        TrackerLayout::MmStrayFairies,
        TrackerLayout::MmSongs,
        TrackerLayout::MmEquipment,
        TrackerLayout::Combo,
    ];

    for layout in layouts {
        let cols = layout.column_count();
        let rows = layout.row_count();
        let (width, height) = layout.pixel_dimensions();

        assert!(cols > 0, "Layout should have at least one column");
        assert!(rows > 0, "Layout should have at least one row");
        assert!(width > 0, "Layout should have non-zero width");
        assert!(height > 0, "Layout should have non-zero height");

        // Width should be columns * 60 (grid spacing)
        assert_eq!(
            width,
            (cols * 60) as u32,
            "Width should match column count * 60"
        );
    }
}

// ==========================================================================
// Cell Visibility Condition Tests
// ==========================================================================

#[test]
fn test_composite_keys_visibility_both_inactive() {
    let state = ModelState::default();
    let cell = TrackerCellId::ForestKeys.kind();
    let render = cell.render(&state);

    // No boss key and no small keys should be dimmed
    assert_eq!(render.style, CellStyle::Dimmed);
}

#[test]
fn test_composite_keys_visibility_small_keys_only() {
    let mut state = ModelState::default();
    state.ram.save.small_keys.forest_temple = 3;

    let cell = TrackerCellId::ForestKeys.kind();
    let render = cell.render(&state);

    // Small keys but no boss key: left dimmed (boss key side)
    assert_eq!(render.style, CellStyle::LeftDimmed);
}

#[test]
fn test_composite_keys_visibility_boss_key_only() {
    let mut state = ModelState::default();
    state
        .ram
        .save
        .dungeon_items
        .forest_temple
        .insert(DungeonItems::BOSS_KEY);

    let cell = TrackerCellId::ForestKeys.kind();
    let render = cell.render(&state);

    // Boss key but no small keys: right dimmed (small key side)
    assert_eq!(render.style, CellStyle::RightDimmed);
}

#[test]
fn test_composite_keys_visibility_both_active() {
    let mut state = ModelState::default();
    state.ram.save.small_keys.forest_temple = 3;
    state
        .ram
        .save
        .dungeon_items
        .forest_temple
        .insert(DungeonItems::BOSS_KEY);

    let cell = TrackerCellId::ForestKeys.kind();
    let render = cell.render(&state);

    // Both active: normal
    assert_eq!(render.style, CellStyle::Normal);
}

#[test]
fn test_overlay_cell_visibility() {
    let mut state = ModelState::default();
    let cell = TrackerCellId::Hookshot.kind();

    // Without hookshot, dimmed
    let render = cell.render(&state);
    assert_eq!(render.style, CellStyle::Dimmed);

    // With hookshot, normal
    state.ram.save.inv.hookshot = Hookshot::Hookshot;
    let render = cell.render(&state);
    assert_eq!(render.style, CellStyle::Normal);
}

// ==========================================================================
// Additional Active/Inactive State Tests
// ==========================================================================

#[test]
fn test_all_medallions_inactive_by_default() {
    let state = ModelState::default();

    for med in [
        TrackerCellId::LightMedallion,
        TrackerCellId::ForestMedallion,
        TrackerCellId::FireMedallion,
        TrackerCellId::WaterMedallion,
        TrackerCellId::ShadowMedallion,
        TrackerCellId::SpiritMedallion,
    ] {
        let render = med.kind().render(&state);
        assert_eq!(
            render.style,
            CellStyle::Dimmed,
            "Medallion {:?} should be dimmed by default",
            med
        );
    }
}

#[test]
fn test_all_stones_inactive_by_default() {
    let state = ModelState::default();

    for stone in [
        TrackerCellId::KokiriEmerald,
        TrackerCellId::GoronRuby,
        TrackerCellId::ZoraSapphire,
    ] {
        let render = stone.kind().render(&state);
        assert_eq!(
            render.style,
            CellStyle::Dimmed,
            "Stone {:?} should be dimmed by default",
            stone
        );
    }
}

#[test]
fn test_song_render_inactive() {
    let state = ModelState::default();
    let cell = TrackerCellId::ZeldasLullaby.kind();
    let render = cell.render(&state);

    assert_eq!(render.style, CellStyle::Dimmed);
}

#[test]
fn test_song_click_toggles() {
    let mut state = ModelState::default();
    let cell = TrackerCellId::ZeldasLullaby.kind();

    assert!(!state
        .ram
        .save
        .quest_items
        .contains(QuestItems::ZELDAS_LULLABY));

    cell.click(&mut state);
    assert!(state
        .ram
        .save
        .quest_items
        .contains(QuestItems::ZELDAS_LULLABY));

    cell.click(&mut state);
    assert!(!state
        .ram
        .save
        .quest_items
        .contains(QuestItems::ZELDAS_LULLABY));
}

#[test]
fn test_warp_song_render_and_click() {
    let mut state = ModelState::default();
    let cell = TrackerCellId::Minuet.kind();

    // Initially dimmed
    let render = cell.render(&state);
    assert_eq!(render.style, CellStyle::Dimmed);

    // Click to activate
    cell.click(&mut state);
    let render = cell.render(&state);
    assert_eq!(render.style, CellStyle::Normal);
}

// ==========================================================================
// MM Items Toggle Tests (Issue #515)
// ==========================================================================

#[test]
fn test_mm_small_keys_click_creates_mm_save_when_none() {
    let mut state = ModelState::default();

    // Verify mm_save is initially None
    assert!(state.ram.mm_save.is_none());

    let cell = TrackerCellId::MmWoodfallSmallKeys.kind();

    // Click to increment small keys
    cell.click(&mut state);

    // mm_save should now be Some and key count should be 1
    assert!(state.ram.mm_save.is_some());
    assert_eq!(state.ram.mm_save.as_ref().unwrap().small_keys.woodfall, 1);
}

#[test]
fn test_mm_boss_key_click_creates_mm_save_when_none() {
    let mut state = ModelState::default();

    // Verify mm_save is initially None
    assert!(state.ram.mm_save.is_none());

    let cell = TrackerCellId::MmWoodfallBossKey.kind();

    // Click to toggle boss key
    cell.click(&mut state);

    // mm_save should now be Some and boss key should be set
    assert!(state.ram.mm_save.is_some());
    assert!(state
        .ram
        .mm_save
        .as_ref()
        .unwrap()
        .dungeon_items
        .woodfall
        .contains(crate::mm_save::MmDungeonItems::BOSS_KEY));
}

#[test]
fn test_mm_map_click_creates_mm_save_when_none() {
    let mut state = ModelState::default();

    // Verify mm_save is initially None
    assert!(state.ram.mm_save.is_none());

    let cell = TrackerCellId::MmWoodfallMap.kind();

    // Click to toggle map
    cell.click(&mut state);

    // mm_save should now be Some and map should be set
    assert!(state.ram.mm_save.is_some());
    assert!(state
        .ram
        .mm_save
        .as_ref()
        .unwrap()
        .dungeon_items
        .woodfall
        .contains(crate::mm_save::MmDungeonItems::MAP));
}

#[test]
fn test_mm_compass_click_creates_mm_save_when_none() {
    let mut state = ModelState::default();

    // Verify mm_save is initially None
    assert!(state.ram.mm_save.is_none());

    let cell = TrackerCellId::MmWoodfallCompass.kind();

    // Click to toggle compass
    cell.click(&mut state);

    // mm_save should now be Some and compass should be set
    assert!(state.ram.mm_save.is_some());
    assert!(state
        .ram
        .mm_save
        .as_ref()
        .unwrap()
        .dungeon_items
        .woodfall
        .contains(crate::mm_save::MmDungeonItems::COMPASS));
}

// ==========================================================================
// Accessibility Rendering Tests
// ==========================================================================

#[test]
fn test_medallion_with_location_accessibility_none_when_not_possessed() {
    let state = ModelState::default();
    let cell = TrackerCellId::ForestMedallionWithLocation.kind();
    let render = cell.render(&state);

    // When medallion is not possessed, accessibility should be None
    assert!(
        render.accessibility.is_none(),
        "Accessibility should be None when medallion is not possessed"
    );
    assert_eq!(render.style, CellStyle::Dimmed);
}

#[test]
fn test_medallion_with_location_accessibility_checked_when_possessed() {
    let mut state = ModelState::default();
    state
        .ram
        .save
        .quest_items
        .insert(QuestItems::FOREST_MEDALLION);

    let cell = TrackerCellId::ForestMedallionWithLocation.kind();
    let render = cell.render(&state);

    // When medallion is possessed, accessibility should be Checked
    assert_eq!(
        render.accessibility,
        Some(AccessibilityStatus::Checked),
        "Accessibility should be Checked when medallion is possessed"
    );
    assert_eq!(render.style, CellStyle::Normal);
}

#[test]
fn test_stone_with_location_accessibility_none_when_not_possessed() {
    let state = ModelState::default();
    let cell = TrackerCellId::KokiriEmeraldWithLocation.kind();
    let render = cell.render(&state);

    // When stone is not possessed, accessibility should be None
    assert!(
        render.accessibility.is_none(),
        "Accessibility should be None when stone is not possessed"
    );
    assert_eq!(render.style, CellStyle::Dimmed);
}

#[test]
fn test_stone_with_location_accessibility_checked_when_possessed() {
    let mut state = ModelState::default();
    state
        .ram
        .save
        .quest_items
        .insert(QuestItems::KOKIRI_EMERALD);

    let cell = TrackerCellId::KokiriEmeraldWithLocation.kind();
    let render = cell.render(&state);

    // When stone is possessed, accessibility should be Checked
    assert_eq!(
        render.accessibility,
        Some(AccessibilityStatus::Checked),
        "Accessibility should be Checked when stone is possessed"
    );
    assert_eq!(render.style, CellStyle::Normal);
}

#[test]
fn test_song_accessibility_none_when_check_not_completed() {
    let state = ModelState::default();
    let cell = TrackerCellId::ZeldasLullaby.kind();
    let render = cell.render(&state);

    // When song check is not completed, accessibility should be None
    assert!(
        render.accessibility.is_none(),
        "Accessibility should be None when song check is not completed"
    );
}

#[test]
fn test_song_check_accessibility_none_when_not_completed() {
    let state = ModelState::default();
    let cell = TrackerCellId::ZeldasLullabyCheck.kind();
    let render = cell.render(&state);

    // When check is not completed, accessibility should be None
    assert!(
        render.accessibility.is_none(),
        "Accessibility should be None when check is not completed"
    );
}

#[test]
#[cfg(feature = "rocket")]
fn test_cell_overlay_count_renders_without_max_when_zero() {
    use rocket_util::ToHtml;
    let render = CellRender::new(
        ImageInfo::new("triforce"),
        CellStyle::Normal,
        CellOverlay::CountWithMax {
            count: 5,
            max: 0,
            count_img: ImageInfo::new("force"),
        },
    );
    let html = render.to_html().0;
    assert!(html.contains(">5<"));
    // Should not contain "X / Y" format (the slash in img src path is fine)
    assert!(!html.contains(" / "));
}

#[test]
#[cfg(feature = "rocket")]
fn test_cell_overlay_count_renders_with_max_when_nonzero() {
    use rocket_util::ToHtml;
    let render = CellRender::new(
        ImageInfo::new("skulltula"),
        CellStyle::Normal,
        CellOverlay::CountWithMax {
            count: 25,
            max: 100,
            count_img: ImageInfo::new("skulls"),
        },
    );
    let html = render.to_html().0;
    assert!(html.contains("25 / 100"));
}
