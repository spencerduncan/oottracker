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
        MmFlagType::EventInf => check_event_inf(mm_save, flag_bit),
        MmFlagType::WeekEventReg => check_week_event_reg(mm_save, flag_bit),
        // These flag types need special handling or are not yet implemented
        MmFlagType::GoldSkulltula
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

/// Check an EventInf flag.
///
/// The flag_bit encodes both the word index and bit mask:
/// - Bits 16-31: word_index (0-3)
/// - Bits 0-15: bit_mask (u16)
///
/// If word_index is 0, the flag_bit is used directly as the mask (backward compatible).
fn check_event_inf(mm_save: &MmSave, flag_bit: u32) -> CheckStatus {
    // Extract word index and mask from flag_bit
    // Format: (word_index << 16) | bit_mask
    // If word_index would be 0, flag_bit is just the mask (backward compatible)
    let word_index = (flag_bit >> 16) as usize;
    let mask = (flag_bit & 0xFFFF) as u16;

    // Validate word_index (0-3 for 4 u16 values)
    if word_index >= 4 {
        return CheckStatus::Unknown;
    }

    let value = mm_save.event_inf[word_index];
    if value & mask != 0 {
        CheckStatus::Checked
    } else {
        CheckStatus::Unchecked
    }
}

/// Check a WeekEventReg flag.
///
/// The flag_bit encodes both the byte index and bit mask:
/// - Bits 8-31: byte_index (0-99)
/// - Bits 0-7: bit_mask (u8)
///
/// If byte_index is 0, the flag_bit is used directly as the mask (backward compatible).
fn check_week_event_reg(mm_save: &MmSave, flag_bit: u32) -> CheckStatus {
    // Extract byte index and mask from flag_bit
    // Format: (byte_index << 8) | bit_mask
    // If byte_index would be 0, flag_bit is just the mask (backward compatible)
    let byte_index = (flag_bit >> 8) as usize;
    let mask = (flag_bit & 0xFF) as u8;

    // Safely access the byte at the given index
    match mm_save.week_event_reg.get(byte_index) {
        Some(&value) => {
            if value & mask != 0 {
                CheckStatus::Checked
            } else {
                CheckStatus::Unchecked
            }
        }
        None => CheckStatus::Unknown,
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
