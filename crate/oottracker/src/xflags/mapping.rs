//! Location ID to xflag bit position mappings.
//!
//! This module provides mappings from location IDs to their xflag bit positions.
//! The xflag bit position is determined by OoTMM at build time based on the
//! `Xflag(scene_id, setup_id, room_id, slice_id, id)` tuple for each randomized item.
//!
//! ## Mapping Sources
//!
//! OoTMM generates these mappings in `packages/core/scripts/xsanity.ts`.
//! The mappings here should match the output of that build process.
//!
//! ## Location Types Using XFlags
//!
//! XFlags are used for actor-based collectibles:
//! - Freestanding items (EN_ITEM00): Rupees, hearts, fairies
//! - Pots (POT, FLYING_POT): With randomized contents
//! - Grass/bushes (EN_KUSA, EN_KUSA2, OBJ_GRASS): With randomized drops
//! - Crates (OBJ_KIBAKO, OBJ_KIBAKO2, OBJ_TARU): With randomized contents
//! - Beehives (OBJ_COMB): With randomized rewards
//! - Snowballs (OBJ_SNOWBALL, OBJ_SNOWBALL2)
//! - Wonder items (EN_HIT_TAG, EN_INVISIBLE_RUPPE)
//! - Fairies (EN_ELF)
//!
//! ## Location Types NOT Using XFlags
//!
//! These still need traditional flag checking:
//! - Chests: Scene chest flags
//! - NPC rewards: EventInf/WeekEventReg
//! - Boss defeats: Quest items bitfield
//! - Songs: Quest items bitfield
//! - Shop/scrub purchases: Specific flags

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
}
