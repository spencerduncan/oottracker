//! Location status checking functions.

use std::collections::HashMap;

use once_cell::sync::Lazy;
use ootmm::embedded_data;
use ootmm::expr::eval_str;
use ootmm::is_region_reachable;
use ootmm::world_database::WorldDatabase;

use crate::save::Save;
use crate::ModelState;

use super::eval::OotEvalContext;
use super::mq::{get_active_mapped_locations, mq_settings_from_knowledge};
use super::queries::{get_location_logic, get_mapped_locations};
use super::types::{
    Accessibility, CheckStatus, CheckedLocationsSummary, FlagMapping, FlagType, LocationCheckResult,
};

/// Lazily initialized world database containing all location logic definitions.
static WORLD_DATABASE: Lazy<Option<WorldDatabase>> =
    Lazy::new(|| match embedded_data::create_world_database() {
        Ok(db) => Some(db),
        Err(e) => {
            eprintln!(
                "Warning: Failed to load world database for logic evaluation: {}",
                e
            );
            None
        }
    });

/// Evaluates accessibility for a location based on current save state.
///
/// Returns `Accessibility::Available` if the location's logic evaluates to true,
/// `Accessibility::Unavailable` if it evaluates to false, or `Accessibility::Unknown`
/// if the location is not found or has no logic defined.
fn evaluate_accessibility(location_id: &str, save: &Save) -> Accessibility {
    let Some(db) = WORLD_DATABASE.as_ref() else {
        return Accessibility::Unknown;
    };

    let Some((location, region_id)) = db.get_location(location_id) else {
        return Accessibility::Unknown;
    };

    // Get the region to determine which game this location belongs to
    let Some(region) = db.get_region(region_id) else {
        return Accessibility::Unknown;
    };

    let ctx = OotEvalContext::new(save);

    // Check if the region is reachable before evaluating location logic
    if !is_region_reachable(db, &ctx, region.game, region_id) {
        return Accessibility::Unavailable;
    }

    let Some(logic) = &location.logic else {
        // No logic defined means always accessible
        return Accessibility::Available;
    };

    match eval_str(logic, &ctx) {
        Ok(true) => Accessibility::Available,
        Ok(false) => Accessibility::Unavailable,
        Err(_) => Accessibility::Unknown,
    }
}

/// Checks if a specific location has been checked based on the current game state.
///
/// # Arguments
///
/// * `mapping` - The flag mapping for the location
/// * `model` - The current model state containing game memory
///
/// # Returns
///
/// `CheckStatus::Skipped` if the user has marked this location as skipped,
/// `CheckStatus::Checked` if the location flag is set,
/// `CheckStatus::Unchecked` if the flag is not set,
/// `CheckStatus::Unknown` if the location is unmapped or cannot be determined.
#[must_use]
pub fn check_location_status(mapping: &FlagMapping, model: &ModelState) -> CheckStatus {
    // Check if user has marked this location as skipped
    if model.skipped_locations.contains(mapping.location_id) {
        return CheckStatus::Skipped;
    }

    // If the mapping is a stub (unmapped), we can't determine the status
    if mapping.is_stub() {
        return CheckStatus::Unknown;
    }

    let flag_type = match mapping.flag_type {
        Some(ft) => ft,
        None => return CheckStatus::Unknown,
    };

    let flag_bit = match mapping.flag_bit {
        Some(fb) => fb,
        None => return CheckStatus::Unknown,
    };

    match flag_type {
        FlagType::Chest => {
            if let Some(scene_id) = mapping.scene_id {
                let scene_flags = model.ram.scene_flags();
                let chests = scene_flags.get_chest_flags(scene_id);
                if chests & flag_bit != 0 {
                    CheckStatus::Checked
                } else {
                    CheckStatus::Unchecked
                }
            } else {
                CheckStatus::Unknown
            }
        }
        FlagType::Switch => {
            if let Some(scene_id) = mapping.scene_id {
                let scene_flags = model.ram.scene_flags();
                let switches = scene_flags.get_switch_flags(scene_id);
                if switches & flag_bit != 0 {
                    CheckStatus::Checked
                } else {
                    CheckStatus::Unchecked
                }
            } else {
                CheckStatus::Unknown
            }
        }
        FlagType::RoomClear => {
            if let Some(scene_id) = mapping.scene_id {
                let scene_flags = model.ram.scene_flags();
                let room_clear = scene_flags.get_room_clear_flags(scene_id);
                if room_clear & flag_bit != 0 {
                    CheckStatus::Checked
                } else {
                    CheckStatus::Unchecked
                }
            } else {
                CheckStatus::Unknown
            }
        }
        FlagType::Collectible => {
            if let Some(scene_id) = mapping.scene_id {
                let scene_flags = model.ram.scene_flags();
                let collectible = scene_flags.get_collectible_flags(scene_id);
                if collectible & flag_bit != 0 {
                    CheckStatus::Checked
                } else {
                    CheckStatus::Unchecked
                }
            } else {
                CheckStatus::Unknown
            }
        }
        FlagType::GoldSkulltula => {
            // Gold Skulltulas use a different storage mechanism
            // Check the gold_skulltulas field in save data
            let gs_flags = model.ram.save.gold_skulltulas.get_raw_flags();
            if gs_flags & flag_bit != 0 {
                CheckStatus::Checked
            } else {
                CheckStatus::Unchecked
            }
        }
        FlagType::EventChkInf => {
            let event_flags = model.ram.save.event_chk_inf.get_raw_flags();
            if event_flags & flag_bit != 0 {
                CheckStatus::Checked
            } else {
                CheckStatus::Unchecked
            }
        }
        FlagType::ItemGetInf => {
            let item_flags = model.ram.save.item_get_inf.get_raw_flags();
            if item_flags & flag_bit != 0 {
                CheckStatus::Checked
            } else {
                CheckStatus::Unchecked
            }
        }
        FlagType::InfTable => {
            let inf_flags = model.ram.save.inf_table.get_raw_flags();
            if inf_flags & flag_bit != 0 {
                CheckStatus::Checked
            } else {
                CheckStatus::Unchecked
            }
        }
        // These flag types need special handling or are not yet implemented
        FlagType::Shop
        | FlagType::Scrub
        | FlagType::GreatFairy
        | FlagType::Boss
        | FlagType::Song
        | FlagType::Fishing
        | FlagType::Cow
        | FlagType::GossipStone => CheckStatus::Unknown,
    }
}

/// Returns all checked locations for the current game state.
///
/// This function returns check status for all OoT locations. For combo randomizer
/// tracking that includes MM locations, use `get_all_checked_locations_combo`.
///
/// # Arguments
///
/// * `model` - The current model state containing game memory
///
/// # Returns
///
/// A vector of `LocationCheckResult` for all mapped OoT locations.
pub fn get_all_checked_locations(model: &ModelState) -> Vec<LocationCheckResult> {
    get_mapped_locations()
        .map(|mapping| {
            let location_id = mapping.location_id.to_string();
            let accessibility = evaluate_accessibility(&location_id, &model.ram.save);
            LocationCheckResult {
                location_id: location_id.clone(),
                status: check_location_status(mapping, model),
                is_mapped: mapping.is_mapped(),
                logic: get_location_logic(&location_id),
                accessibility,
            }
        })
        .collect()
}

/// Returns all checked locations for both OoT and MM (combo randomizer).
///
/// This function combines OoT and MM location checks for combo randomizer tracking.
/// MM locations are only included if `model.ram.mm_save` is Some.
///
/// # Arguments
///
/// * `model` - The current model state containing game memory
///
/// # Returns
///
/// A vector of `LocationCheckResult` for all OoT locations and MM locations (if available).
pub fn get_all_checked_locations_combo(model: &ModelState) -> Vec<LocationCheckResult> {
    use crate::mm_flag_mapping::{check_mm_location_status, get_mm_mapped_locations};

    // Get all OoT locations
    let mut results: Vec<LocationCheckResult> = get_mapped_locations()
        .map(|mapping| {
            let location_id = mapping.location_id.to_string();
            let accessibility = evaluate_accessibility(&location_id, &model.ram.save);
            LocationCheckResult {
                location_id: location_id.clone(),
                status: check_location_status(mapping, model),
                is_mapped: mapping.is_mapped(),
                logic: get_location_logic(&location_id),
                accessibility,
            }
        })
        .collect();

    // Add MM locations if MM save data is available
    if let Some(ref mm_save) = model.ram.mm_save {
        let mm_results: Vec<LocationCheckResult> = get_mm_mapped_locations()
            .map(|mapping| {
                let location_id = mapping.location_id.to_string();
                // MM accessibility evaluation would use MM save context
                // For now, mark as Unknown since we don't have MM logic evaluation yet
                LocationCheckResult {
                    location_id: location_id.clone(),
                    status: check_mm_location_status(mapping, mm_save),
                    is_mapped: mapping.is_mapped(),
                    logic: get_location_logic(&location_id),
                    accessibility: Accessibility::Unknown,
                }
            })
            .collect();
        results.extend(mm_results);
    }

    results
}

/// Returns all checked locations as a HashMap for efficient lookup.
///
/// # Arguments
///
/// * `model` - The current model state containing game memory
///
/// # Returns
///
/// A HashMap mapping location_id to CheckStatus.
pub fn get_checked_locations_map(model: &ModelState) -> HashMap<String, CheckStatus> {
    get_mapped_locations()
        .map(|mapping| {
            (
                mapping.location_id.to_string(),
                check_location_status(mapping, model),
            )
        })
        .collect()
}

/// Returns a summary of checked locations for the current game state.
///
/// This function includes both OoT and MM locations when MM save data is available,
/// making it suitable for combo randomizer tracking.
///
/// # Arguments
///
/// * `model` - The current model state containing game memory
///
/// # Returns
///
/// A `CheckedLocationsSummary` containing counts and individual location statuses.
pub fn get_checked_locations_summary(model: &ModelState) -> CheckedLocationsSummary {
    // Use combo version to include both OoT and MM locations
    let locations = get_all_checked_locations_combo(model);
    let total_mapped = locations.len();
    let checked_count = locations
        .iter()
        .filter(|l| l.status == CheckStatus::Checked)
        .count();
    let unchecked_count = locations
        .iter()
        .filter(|l| l.status == CheckStatus::Unchecked)
        .count();
    let skipped_count = locations
        .iter()
        .filter(|l| l.status == CheckStatus::Skipped)
        .count();
    let unknown_count = locations
        .iter()
        .filter(|l| l.status == CheckStatus::Unknown)
        .count();
    // Count available/unavailable only for unchecked locations
    let available_count = locations
        .iter()
        .filter(|l| {
            l.status == CheckStatus::Unchecked && l.accessibility == Accessibility::Available
        })
        .count();
    let unavailable_count = locations
        .iter()
        .filter(|l| {
            l.status == CheckStatus::Unchecked && l.accessibility == Accessibility::Unavailable
        })
        .count();

    // Determine current scene and game from RAM state
    let current_scene_id = Some(model.ram.current_scene_id);

    // Determine which game is active based on MM save data presence
    // If MM save is present and has valid data, we might be in MM
    let current_game = if model.ram.mm_save.is_some() {
        // In combo mode - could be either game
        // Scene ID 0xFF typically indicates no active scene
        if model.ram.current_scene_id == 0xFF {
            None
        } else {
            // Default to OoT for now - more sophisticated detection would
            // require checking game detection state which isn't stored in ModelState
            Some("oot".to_string())
        }
    } else {
        // No MM save means standalone OoT
        Some("oot".to_string())
    };

    CheckedLocationsSummary {
        total_mapped,
        checked_count,
        unchecked_count,
        skipped_count,
        unknown_count,
        available_count,
        unavailable_count,
        locations,
        current_scene_id,
        current_game,
    }
}

/// Returns all checked locations for the current game state, filtered by MQ settings.
///
/// This function filters out locations that don't match the current MQ/vanilla
/// settings for each dungeon as stored in the model's Knowledge.
///
/// # Arguments
///
/// * `model` - The current model state containing game memory and knowledge
///
/// # Returns
///
/// A vector of `LocationCheckResult` for all active locations based on MQ settings.
pub fn get_all_checked_locations_filtered(model: &ModelState) -> Vec<LocationCheckResult> {
    let settings = mq_settings_from_knowledge(&model.knowledge.mq);
    get_active_mapped_locations(&settings)
        .map(|mapping| {
            let location_id = mapping.location_id.to_string();
            let accessibility = evaluate_accessibility(&location_id, &model.ram.save);
            LocationCheckResult {
                location_id: location_id.clone(),
                status: check_location_status(mapping, model),
                is_mapped: mapping.is_mapped(),
                logic: get_location_logic(&location_id),
                accessibility,
            }
        })
        .collect()
}

/// Returns a summary of checked locations filtered by MQ settings from Knowledge.
///
/// This function filters out locations that don't match the current MQ/vanilla
/// settings for each dungeon. For example, if the Deku Tree is set to vanilla
/// in Knowledge.mq, then Master Quest Deku Tree locations will be excluded.
///
/// # Arguments
///
/// * `model` - The current model state containing game memory and knowledge
///
/// # Returns
///
/// A `CheckedLocationsSummary` containing counts and individual location statuses,
/// filtered to only include locations matching the current MQ settings.
pub fn get_checked_locations_summary_filtered(model: &ModelState) -> CheckedLocationsSummary {
    let locations = get_all_checked_locations_filtered(model);
    let total_mapped = locations.len();
    let checked_count = locations
        .iter()
        .filter(|l| l.status == CheckStatus::Checked)
        .count();
    let unchecked_count = locations
        .iter()
        .filter(|l| l.status == CheckStatus::Unchecked)
        .count();
    let skipped_count = locations
        .iter()
        .filter(|l| l.status == CheckStatus::Skipped)
        .count();
    let unknown_count = locations
        .iter()
        .filter(|l| l.status == CheckStatus::Unknown)
        .count();
    // Count available/unavailable only for unchecked locations
    let available_count = locations
        .iter()
        .filter(|l| {
            l.status == CheckStatus::Unchecked && l.accessibility == Accessibility::Available
        })
        .count();
    let unavailable_count = locations
        .iter()
        .filter(|l| {
            l.status == CheckStatus::Unchecked && l.accessibility == Accessibility::Unavailable
        })
        .count();

    // Determine current scene and game from RAM state
    let current_scene_id = Some(model.ram.current_scene_id);

    // Determine which game is active based on MM save data presence
    let current_game = if model.ram.mm_save.is_some() {
        if model.ram.current_scene_id == 0xFF {
            None
        } else {
            Some("oot".to_string())
        }
    } else {
        Some("oot".to_string())
    };

    CheckedLocationsSummary {
        total_mapped,
        checked_count,
        unchecked_count,
        skipped_count,
        unknown_count,
        available_count,
        unavailable_count,
        locations,
        current_scene_id,
        current_game,
    }
}
