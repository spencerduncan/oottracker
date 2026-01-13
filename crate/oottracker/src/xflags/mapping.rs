//! Location ID to xflag bit position mappings.
//!
//! This module provides mappings from location IDs to their xflag bit positions.
//! The xflag bit position is determined by OoTMM at build time based on the
//! `Xflag(scene_id, setup_id, room_id, slice_id, id)` tuple for each randomized item.
//!
//! ## Mapping Sources
//!
//! OoTMM generates these mappings internally during its build process.
//! The mappings here need to match OoTMM's xflag assignments.
//!
//! ## XFlag Capacity
//!
//! - OoT: 745 bits (XFLAGS_COUNT_OOT = 0x2e9)
//! - MM: 842 bits (XFLAGS_COUNT_MM = 0x34a)
//!
//! ## OoT Actors Using XFlags (from OoTMM source)
//!
//! - EN_ITEM00: Freestanding items (rupees, hearts)
//! - POT, FLYING_POT: Pots with randomized contents
//! - EN_KUSA: Grass/bushes with randomized drops
//! - OBJ_KIBAKO, OBJ_KIBAKO2: Crates
//! - OBJ_COMB: Beehives
//! - EN_ELF: Fairies
//! - OBJ_MURE, OBJ_MURE2, OBJ_MURE3: Groups (grass, rupee circles)
//! - EN_WONDER_ITEM, SHOT_SUN: Wonder items, sun's song spots
//! - OBJ_HAMISHI, EN_ISHI: Rocks
//! - BG_ICICLE, BG_ICE_SHELTER: Ice
//! - OBJ_BEAN: Bean patches
//!
//! ## MM Actors Using XFlags (from OoTMM source)
//!
//! - EN_ITEM00, POT, FLYING_POT: Items, pots
//! - EN_KUSA, EN_KUSA2, OBJ_GRASS: All grass types
//! - OBJ_KIBAKO, OBJ_KIBAKO2, OBJ_TARU: Crates, barrels
//! - OBJ_COMB, OBJ_FLOWERPOT: Beehives, potted plants
//! - OBJ_SNOWBALL, OBJ_SNOWBALL2: Snowballs
//! - EN_HIT_TAG, EN_INVISIBLE_RUPPE: Wonder items, invisible rupees
//! - EN_ELF: Fairies
//!
//! ## Location Types NOT Using XFlags
//!
//! These still need traditional flag checking:
//! - Chests: Scene chest flags
//! - NPC rewards: EventInf/WeekEventReg
//! - Boss defeats: Quest items bitfield
//! - Songs: Quest items bitfield
//! - Shop/scrub purchases: Specific flags
//!
//! ## World Data Location Types
//!
//! From YAML world data, these `locationType` values are xflag candidates:
//! - `freestanding`: Actor-based collectibles (pots, grass, rocks, rupees, etc.)
//!
//! Non-xflag location types:
//! - `chest`: Scene chest flags
//! - `npc`: EventInf/WeekEventReg
//! - `boss`: Quest items
//! - `song`: Quest items
//! - `shop`, `scrub`: Purchase flags
//! - `collectible`: Usually Gold Skulltulas (separate flag system)
//! - `cow`, `fishing`, `fairy`, `event`: Various special handlers

use std::collections::HashMap;

use once_cell::sync::Lazy;

/// MM location ID to xflag bit position mapping.
///
/// This maps location IDs (like "mm_termina_field_grass_rupee_1") to their
/// corresponding bit position in the MM xflags array.
///
/// Note: The actual mappings need to be populated from OoTMM's xsanity.ts output.
/// Currently contains placeholder data for common locations.
pub static MM_XFLAG_LOCATIONS: Lazy<HashMap<&'static str, u16>> = Lazy::new(|| {
    let mut m = HashMap::new();

    // TODO: Populate from OoTMM xsanity.ts output
    // The mappings below are placeholders showing the expected format.
    // Actual bit positions need to be extracted from OoTMM build output.

    // Example freestanding items (EN_ITEM00)
    // These are items that appear in the world without a chest
    register_mm_freestanding(&mut m);

    // Example pot items (POT, FLYING_POT)
    register_mm_pots(&mut m);

    // Example grass items (EN_KUSA, OBJ_GRASS)
    register_mm_grass(&mut m);

    // Example crate items (OBJ_KIBAKO)
    register_mm_crates(&mut m);

    // Example beehive items (OBJ_COMB)
    register_mm_beehives(&mut m);

    m
});

/// OOT location ID to xflag bit position mapping.
///
/// This maps location IDs to their corresponding bit position in the OOT xflags array.
///
/// Note: The actual mappings need to be populated from OoTMM's xsanity.ts output.
pub static OOT_XFLAG_LOCATIONS: Lazy<HashMap<&'static str, u16>> = Lazy::new(|| {
    let mut m = HashMap::new();

    // TODO: Populate from OoTMM xsanity.ts output
    // Placeholder mappings for OOT locations

    register_oot_freestanding(&mut m);
    register_oot_pots(&mut m);
    register_oot_grass(&mut m);
    register_oot_crates(&mut m);

    m
});

/// Returns the xflag bit position for an MM location ID, if mapped.
#[must_use]
pub fn get_mm_xflag_bit(location_id: &str) -> Option<u16> {
    MM_XFLAG_LOCATIONS.get(location_id).copied()
}

/// Returns the xflag bit position for an OOT location ID, if mapped.
#[must_use]
pub fn get_oot_xflag_bit(location_id: &str) -> Option<u16> {
    OOT_XFLAG_LOCATIONS.get(location_id).copied()
}

/// Returns the number of mapped MM xflag locations.
#[must_use]
pub fn mm_xflag_location_count() -> usize {
    MM_XFLAG_LOCATIONS.len()
}

/// Returns the number of mapped OOT xflag locations.
#[must_use]
pub fn oot_xflag_location_count() -> usize {
    OOT_XFLAG_LOCATIONS.len()
}

/// Returns true if the location ID is an xflag-based location for MM.
#[must_use]
pub fn is_mm_xflag_location(location_id: &str) -> bool {
    MM_XFLAG_LOCATIONS.contains_key(location_id)
}

/// Returns true if the location ID is an xflag-based location for OOT.
#[must_use]
pub fn is_oot_xflag_location(location_id: &str) -> bool {
    OOT_XFLAG_LOCATIONS.contains_key(location_id)
}

/// Returns true if a location ID appears to be an xflag-eligible location
/// based on naming patterns from world data.
///
/// This checks if the location name contains keywords indicating it's a
/// freestanding actor-based collectible (pots, grass, crates, rocks, etc.).
///
/// Note: This is a heuristic based on naming conventions. The actual xflag
/// tracking is determined by OoTMM's build process.
#[must_use]
pub fn is_xflag_eligible_location(location_id: &str) -> bool {
    // Keywords indicating xflag-eligible actors
    let xflag_keywords = [
        "_pot_",
        "_pot", // Pots
        "_grass_",
        "_grass", // Grass
        "_crate_",
        "_crate", // Crates
        "_rock_",
        "_rock", // Rocks
        "_rupee_",
        "_rupee", // Freestanding rupees
        "_hive_",
        "_hive",
        "_beehive_",
        "_beehive", // Beehives
        "_butterfly_",
        "_butterfly", // Butterflies
        "_wonderitem_",
        "_wonderitem",
        "_wonder_",
        "_wonder", // Wonder items
        "_soil_",
        "_soil",
        "_bean_",
        "_bean", // Bean patches
        "_snowball_",
        "_snowball", // Snowballs (MM)
        "_barrel_",
        "_barrel", // Barrels (MM)
    ];

    let loc_lower = location_id.to_lowercase();
    xflag_keywords.iter().any(|kw| loc_lower.contains(kw))
}

/// OoT xflag location categories based on world data analysis.
///
/// From analyzing oot_overworld.yaml, oot_dungeons.yaml, oot_dungeons_mq.yaml:
/// - Pots: 574 locations
/// - Grass: 341 locations
/// - Crates: 230 locations
/// - Rocks: 127 locations
/// - Rupees/freestanding: 115 locations
/// - Butterflies: 59 locations
/// - Soil/bean: 30 locations
/// - Beehives: 21 locations
/// - Wonder items: 7 locations
/// - Other: 192 locations
///
/// Total freestanding: 1696 locations
/// XFlag capacity: 745 bits
///
/// Not all freestanding locations use xflags - only those that are
/// actually randomized in OoTMM.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OotXflagCategory {
    Pot,
    Grass,
    Crate,
    Rock,
    Rupee,
    Beehive,
    Butterfly,
    Soil,
    WonderItem,
    Other,
}

/// Categorize an OoT location ID by its xflag type.
#[must_use]
pub fn categorize_oot_xflag_location(location_id: &str) -> Option<OotXflagCategory> {
    let loc = location_id.to_lowercase();

    if loc.contains("_pot_") || loc.ends_with("_pot") {
        Some(OotXflagCategory::Pot)
    } else if loc.contains("_grass_") || loc.ends_with("_grass") {
        Some(OotXflagCategory::Grass)
    } else if loc.contains("_crate_") || loc.ends_with("_crate") {
        Some(OotXflagCategory::Crate)
    } else if loc.contains("_rock_") || loc.ends_with("_rock") {
        Some(OotXflagCategory::Rock)
    } else if loc.contains("_rupee_") || loc.ends_with("_rupee") {
        Some(OotXflagCategory::Rupee)
    } else if loc.contains("_hive_")
        || loc.ends_with("_hive")
        || loc.contains("_beehive_")
        || loc.ends_with("_beehive")
    {
        Some(OotXflagCategory::Beehive)
    } else if loc.contains("_butterfly_") || loc.ends_with("_butterfly") {
        Some(OotXflagCategory::Butterfly)
    } else if loc.contains("_soil_")
        || loc.ends_with("_soil")
        || loc.contains("_bean_")
        || loc.ends_with("_bean")
    {
        Some(OotXflagCategory::Soil)
    } else if loc.contains("_wonderitem_")
        || loc.ends_with("_wonderitem")
        || loc.contains("_wonder_")
        || loc.ends_with("_wonder")
    {
        Some(OotXflagCategory::WonderItem)
    } else if is_xflag_eligible_location(location_id) {
        Some(OotXflagCategory::Other)
    } else {
        None
    }
}

// =============================================================================
// MM Location Registration Functions
// =============================================================================

/// Register MM freestanding item locations.
fn register_mm_freestanding(m: &mut HashMap<&'static str, u16>) {
    // Termina Field freestanding items
    // TODO: Get actual bit positions from OoTMM
    // Format: location_id -> xflag_bit_position
    m.insert("mm_termina_field_freestanding_rupee_near_clock_town", 0);
    m.insert("mm_termina_field_freestanding_rupee_near_swamp", 1);
    m.insert("mm_termina_field_freestanding_rupee_near_mountain", 2);
    m.insert("mm_termina_field_freestanding_rupee_near_ocean", 3);
    m.insert("mm_termina_field_freestanding_rupee_near_ikana", 4);
}

/// Register MM pot item locations.
fn register_mm_pots(m: &mut HashMap<&'static str, u16>) {
    // Clock Town pots
    // TODO: Get actual bit positions from OoTMM
    m.insert("mm_south_clock_town_pot_1", 100);
    m.insert("mm_south_clock_town_pot_2", 101);
    m.insert("mm_south_clock_town_pot_3", 102);
    m.insert("mm_north_clock_town_pot_1", 103);
    m.insert("mm_north_clock_town_pot_2", 104);
    m.insert("mm_east_clock_town_pot_1", 105);
    m.insert("mm_west_clock_town_pot_1", 106);

    // Stock Pot Inn pots
    m.insert("mm_stock_pot_inn_pot_1", 110);
    m.insert("mm_stock_pot_inn_pot_2", 111);

    // Astral Observatory pots
    m.insert("mm_astral_observatory_pot_1", 115);
    m.insert("mm_astral_observatory_pot_2", 116);
}

/// Register MM grass item locations.
fn register_mm_grass(m: &mut HashMap<&'static str, u16>) {
    // Termina Field grass
    // TODO: Get actual bit positions from OoTMM
    m.insert("mm_termina_field_grass_near_clock_town_1", 200);
    m.insert("mm_termina_field_grass_near_clock_town_2", 201);
    m.insert("mm_termina_field_grass_near_swamp_1", 202);
    m.insert("mm_termina_field_grass_near_swamp_2", 203);

    // Southern Swamp grass
    m.insert("mm_southern_swamp_grass_1", 210);
    m.insert("mm_southern_swamp_grass_2", 211);

    // Mountain Village grass
    m.insert("mm_mountain_village_grass_1", 220);
    m.insert("mm_mountain_village_grass_2", 221);
}

/// Register MM crate item locations.
fn register_mm_crates(m: &mut HashMap<&'static str, u16>) {
    // Clock Town crates
    // TODO: Get actual bit positions from OoTMM
    m.insert("mm_south_clock_town_crate_1", 300);
    m.insert("mm_laundry_pool_crate_1", 301);
    m.insert("mm_laundry_pool_crate_2", 302);

    // Milk Bar crates
    m.insert("mm_milk_bar_crate_1", 305);

    // Termina Field crates
    m.insert("mm_termina_field_crate_near_observatory", 310);
}

/// Register MM beehive item locations.
fn register_mm_beehives(m: &mut HashMap<&'static str, u16>) {
    // Various beehive locations
    // TODO: Get actual bit positions from OoTMM
    m.insert("mm_termina_field_beehive_1", 400);
    m.insert("mm_southern_swamp_beehive_1", 401);
    m.insert("mm_mountain_village_beehive_1", 402);
    m.insert("mm_great_bay_coast_beehive_1", 403);
}

// =============================================================================
// OOT Location Registration Functions
// =============================================================================

/// Register OOT freestanding item locations.
fn register_oot_freestanding(m: &mut HashMap<&'static str, u16>) {
    // Kokiri Forest freestanding items
    // TODO: Get actual bit positions from OoTMM
    m.insert("oot_kokiri_forest_freestanding_rupee_1", 0);
    m.insert("oot_kokiri_forest_freestanding_rupee_2", 1);

    // Lost Woods freestanding items
    m.insert("oot_lost_woods_freestanding_rupee_1", 10);
    m.insert("oot_lost_woods_freestanding_rupee_2", 11);

    // Hyrule Field freestanding items
    m.insert("oot_hyrule_field_freestanding_rupee_1", 20);
}

/// Register OOT pot item locations.
fn register_oot_pots(m: &mut HashMap<&'static str, u16>) {
    // Kokiri Forest pots
    // TODO: Get actual bit positions from OoTMM
    m.insert("oot_kokiri_shop_pot_1", 100);
    m.insert("oot_kokiri_shop_pot_2", 101);

    // Castle Town pots
    m.insert("oot_castle_town_pot_1", 110);
    m.insert("oot_castle_town_pot_2", 111);
}

/// Register OOT grass item locations.
fn register_oot_grass(m: &mut HashMap<&'static str, u16>) {
    // Kokiri Forest grass
    // TODO: Get actual bit positions from OoTMM
    m.insert("oot_kokiri_forest_grass_1", 200);
    m.insert("oot_kokiri_forest_grass_2", 201);

    // Hyrule Field grass
    m.insert("oot_hyrule_field_grass_1", 210);
    m.insert("oot_hyrule_field_grass_2", 211);
}

/// Register OOT crate item locations.
fn register_oot_crates(m: &mut HashMap<&'static str, u16>) {
    // Lon Lon Ranch crates
    // TODO: Get actual bit positions from OoTMM
    m.insert("oot_lon_lon_ranch_crate_1", 300);
    m.insert("oot_lon_lon_ranch_crate_2", 301);

    // Gerudo Valley crates
    m.insert("oot_gerudo_valley_crate_1", 310);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mm_xflag_location_lookup() {
        // Test that we can look up known locations
        assert!(get_mm_xflag_bit("mm_termina_field_freestanding_rupee_near_clock_town").is_some());
        assert!(get_mm_xflag_bit("mm_south_clock_town_pot_1").is_some());

        // Test unknown location returns None
        assert!(get_mm_xflag_bit("mm_nonexistent_location").is_none());
    }

    #[test]
    fn test_oot_xflag_location_lookup() {
        // Test that we can look up known locations
        assert!(get_oot_xflag_bit("oot_kokiri_forest_freestanding_rupee_1").is_some());
        assert!(get_oot_xflag_bit("oot_kokiri_shop_pot_1").is_some());

        // Test unknown location returns None
        assert!(get_oot_xflag_bit("oot_nonexistent_location").is_none());
    }

    #[test]
    fn test_mm_xflag_location_count() {
        // Should have some placeholder mappings
        assert!(mm_xflag_location_count() > 0);
    }

    #[test]
    fn test_oot_xflag_location_count() {
        // Should have some placeholder mappings
        assert!(oot_xflag_location_count() > 0);
    }

    #[test]
    fn test_is_xflag_location() {
        // MM locations
        assert!(is_mm_xflag_location("mm_south_clock_town_pot_1"));
        assert!(!is_mm_xflag_location("mm_unknown_chest"));

        // OOT locations
        assert!(is_oot_xflag_location(
            "oot_kokiri_forest_freestanding_rupee_1"
        ));
        assert!(!is_oot_xflag_location("oot_unknown_chest"));
    }

    #[test]
    fn test_bit_positions_in_valid_range() {
        use crate::xflags::{XFLAGS_COUNT_MM, XFLAGS_COUNT_OOT};

        // All MM bit positions should be within valid range
        for (_, &bit) in MM_XFLAG_LOCATIONS.iter() {
            assert!(
                (bit as usize) < XFLAGS_COUNT_MM,
                "MM xflag bit {} exceeds max {}",
                bit,
                XFLAGS_COUNT_MM
            );
        }

        // All OOT bit positions should be within valid range
        for (_, &bit) in OOT_XFLAG_LOCATIONS.iter() {
            assert!(
                (bit as usize) < XFLAGS_COUNT_OOT,
                "OOT xflag bit {} exceeds max {}",
                bit,
                XFLAGS_COUNT_OOT
            );
        }
    }

    // === XFlag Eligibility Tests ===

    #[test]
    fn test_is_xflag_eligible_location_pots() {
        assert!(is_xflag_eligible_location("oot_kokiri_shop_pot_1"));
        assert!(is_xflag_eligible_location("mm_south_clock_town_pot_2"));
        assert!(is_xflag_eligible_location("oot_deku_tree_pot"));
    }

    #[test]
    fn test_is_xflag_eligible_location_grass() {
        assert!(is_xflag_eligible_location("oot_kokiri_forest_grass_1"));
        assert!(is_xflag_eligible_location("mm_termina_field_grass"));
    }

    #[test]
    fn test_is_xflag_eligible_location_crates() {
        assert!(is_xflag_eligible_location("oot_lon_lon_ranch_crate_1"));
        assert!(is_xflag_eligible_location("mm_milk_bar_crate"));
    }

    #[test]
    fn test_is_xflag_eligible_location_rocks() {
        assert!(is_xflag_eligible_location(
            "oot_death_mountain_trail_rock_1"
        ));
        assert!(is_xflag_eligible_location("oot_zora_river_rock"));
    }

    #[test]
    fn test_is_xflag_eligible_location_other() {
        assert!(is_xflag_eligible_location("oot_forest_temple_beehive_1"));
        assert!(is_xflag_eligible_location("oot_sacred_meadow_butterfly_1"));
        assert!(is_xflag_eligible_location("oot_lost_woods_soil_1"));
    }

    #[test]
    fn test_is_xflag_eligible_location_non_xflag() {
        // Chests don't use xflags
        assert!(!is_xflag_eligible_location("oot_deku_tree_compass_chest"));
        assert!(!is_xflag_eligible_location(
            "mm_woodfall_temple_boss_key_chest"
        ));

        // NPCs don't use xflags
        assert!(!is_xflag_eligible_location("oot_malon_epona_song"));
        assert!(!is_xflag_eligible_location("mm_postman_freedom_reward"));

        // Skulltulas don't use xflags (separate system)
        assert!(!is_xflag_eligible_location("oot_deku_tree_gs_basement"));
    }

    // === Category Tests ===

    #[test]
    fn test_categorize_oot_xflag_location_pot() {
        assert_eq!(
            categorize_oot_xflag_location("oot_kokiri_shop_pot_1"),
            Some(OotXflagCategory::Pot)
        );
        assert_eq!(
            categorize_oot_xflag_location("oot_deku_tree_pot"),
            Some(OotXflagCategory::Pot)
        );
    }

    #[test]
    fn test_categorize_oot_xflag_location_grass() {
        assert_eq!(
            categorize_oot_xflag_location("oot_kokiri_forest_grass_1"),
            Some(OotXflagCategory::Grass)
        );
    }

    #[test]
    fn test_categorize_oot_xflag_location_crate() {
        assert_eq!(
            categorize_oot_xflag_location("oot_lon_lon_ranch_crate_1"),
            Some(OotXflagCategory::Crate)
        );
    }

    #[test]
    fn test_categorize_oot_xflag_location_rock() {
        assert_eq!(
            categorize_oot_xflag_location("oot_death_mountain_trail_rock_1"),
            Some(OotXflagCategory::Rock)
        );
    }

    #[test]
    fn test_categorize_oot_xflag_location_rupee() {
        assert_eq!(
            categorize_oot_xflag_location("oot_hyrule_field_rupee_1"),
            Some(OotXflagCategory::Rupee)
        );
    }

    #[test]
    fn test_categorize_oot_xflag_location_beehive() {
        assert_eq!(
            categorize_oot_xflag_location("oot_forest_temple_beehive_1"),
            Some(OotXflagCategory::Beehive)
        );
        assert_eq!(
            categorize_oot_xflag_location("oot_grotto_hive_1"),
            Some(OotXflagCategory::Beehive)
        );
        // Unnumbered hive locations (real examples from world data)
        assert_eq!(
            categorize_oot_xflag_location("oot_death_mountain_trail_cow_grotto_hive"),
            Some(OotXflagCategory::Beehive)
        );
        assert_eq!(
            categorize_oot_xflag_location("oot_gerudo_valley_grotto_hive"),
            Some(OotXflagCategory::Beehive)
        );
    }

    #[test]
    fn test_categorize_oot_xflag_location_butterfly() {
        assert_eq!(
            categorize_oot_xflag_location("oot_sacred_meadow_butterfly_1"),
            Some(OotXflagCategory::Butterfly)
        );
    }

    #[test]
    fn test_categorize_oot_xflag_location_soil() {
        assert_eq!(
            categorize_oot_xflag_location("oot_lost_woods_soil_1"),
            Some(OotXflagCategory::Soil)
        );
        assert_eq!(
            categorize_oot_xflag_location("oot_kokiri_forest_bean_1"),
            Some(OotXflagCategory::Soil)
        );
    }

    #[test]
    fn test_categorize_oot_xflag_location_wonderitem() {
        assert_eq!(
            categorize_oot_xflag_location("oot_zora_river_wonderitem_1"),
            Some(OotXflagCategory::WonderItem)
        );
    }

    #[test]
    fn test_categorize_oot_xflag_location_non_xflag() {
        // Chests return None
        assert_eq!(
            categorize_oot_xflag_location("oot_deku_tree_compass_chest"),
            None
        );
        // NPCs return None
        assert_eq!(categorize_oot_xflag_location("oot_malon_epona_song"), None);
    }
}
