//! Location status checking for MM locations.
//!
//! This module provides functions to check whether MM locations have been
//! collected based on save data.

use crate::flag_mapping::{get_location_logic, Accessibility, CheckStatus, LocationCheckResult};
use crate::mm_save::MmSave;

use super::queries::{get_all_mm_mappings, get_mm_mapped_locations};
use super::{MmFlagMapping, MmFlagType};

/// Checks if a specific MM location has been checked based on the save data.
///
/// # Arguments
///
/// * `mapping` - The flag mapping for the location
/// * `mm_save` - The MM save data
///
/// # Returns
///
/// `CheckStatus::Checked` if the location flag is set,
/// `CheckStatus::Unchecked` if the flag is not set,
/// `CheckStatus::Unknown` if the location is unmapped or cannot be determined.
#[must_use]
pub fn check_mm_location_status(mapping: &MmFlagMapping, mm_save: &MmSave) -> CheckStatus {
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
        MmFlagType::Chest => check_scene_flag(mapping, mm_save, flag_bit, |flags| flags.chest),
        MmFlagType::Switch0 => check_scene_flag(mapping, mm_save, flag_bit, |flags| flags.switch0),
        MmFlagType::Switch1 => check_scene_flag(mapping, mm_save, flag_bit, |flags| flags.switch1),
        MmFlagType::ClearedRoom => {
            check_scene_flag(mapping, mm_save, flag_bit, |flags| flags.cleared_room)
        }
        MmFlagType::Collectible => {
            check_scene_flag(mapping, mm_save, flag_bit, |flags| flags.collectible)
        }
        // These flag types need special handling or are not yet implemented
        MmFlagType::GoldSkulltula
        | MmFlagType::EventInf
        | MmFlagType::WeekEventReg
        | MmFlagType::ItemGetInf
        | MmFlagType::Shop
        | MmFlagType::Scrub
        | MmFlagType::GreatFairy
        | MmFlagType::Boss
        | MmFlagType::Song
        | MmFlagType::Cow
        | MmFlagType::StrayFairy
        | MmFlagType::OwlStatue
        | MmFlagType::MoonsTear
        | MmFlagType::GossipStone => CheckStatus::Unknown,
    }
}

/// Helper function to check scene-based flags.
fn check_scene_flag<F>(
    mapping: &MmFlagMapping,
    mm_save: &MmSave,
    flag_bit: u32,
    get_flag: F,
) -> CheckStatus
where
    F: Fn(&crate::mm_save::MmPermanentSceneFlags) -> u32,
{
    if let Some(scene_id) = mapping.scene_id {
        if let Some(scene_flags) = mm_save.permanent_scene_flags.get(scene_id as usize) {
            if get_flag(scene_flags) & flag_bit != 0 {
                CheckStatus::Checked
            } else {
                CheckStatus::Unchecked
            }
        } else {
            CheckStatus::Unknown
        }
    } else {
        CheckStatus::Unknown
    }
}

/// Returns all checked MM locations for the current save data.
///
/// # Arguments
///
/// * `mm_save` - The MM save data
///
/// # Returns
///
/// A vector of `LocationCheckResult` for all mapped MM locations.
pub fn get_all_mm_checked_locations(mm_save: &MmSave) -> Vec<LocationCheckResult> {
    get_mm_mapped_locations()
        .map(|mapping| LocationCheckResult {
            location_id: mapping.location_id.to_string(),
            status: check_mm_location_status(mapping, mm_save),
            is_mapped: mapping.is_mapped(),
            logic: get_location_logic(mapping.location_id),
            accessibility: Accessibility::Unknown, // MM logic evaluation not yet implemented
        })
        .collect()
}

/// Returns all MM locations (mapped and unmapped) with their check status.
///
/// # Arguments
///
/// * `mm_save` - Optional MM save data
///
/// # Returns
///
/// A vector of `LocationCheckResult` for all MM locations.
/// If mm_save is None, all locations will have Unknown status.
pub fn get_all_mm_locations_with_status(mm_save: Option<&MmSave>) -> Vec<LocationCheckResult> {
    match mm_save {
        Some(save) => get_all_mm_mappings()
            .map(|mapping| LocationCheckResult {
                location_id: mapping.location_id.to_string(),
                status: check_mm_location_status(mapping, save),
                is_mapped: mapping.is_mapped(),
                logic: get_location_logic(mapping.location_id),
                accessibility: Accessibility::Unknown, // MM logic evaluation not yet implemented
            })
            .collect(),
        None => get_all_mm_mappings()
            .map(|mapping| LocationCheckResult {
                location_id: mapping.location_id.to_string(),
                status: CheckStatus::Unknown,
                is_mapped: mapping.is_mapped(),
                logic: get_location_logic(mapping.location_id),
                accessibility: Accessibility::Unknown,
            })
            .collect(),
    }
}
