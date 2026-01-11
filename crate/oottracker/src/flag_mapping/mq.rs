//! Master Quest integration for flag mappings.

use std::collections::HashMap;

use ootmm::settings::{MqDungeon, RandomizerSettings};
use ootr::{
    model::{Dungeon, MainDungeon},
    region::Mq,
};

use super::mappings::OOT_MAPPINGS;
use super::types::FlagMapping;

// Re-export MqDungeon for convenience
pub use ootmm::settings::MqDungeon as MqDungeonType;

/// Determines if an MQ location should be active based on settings.
///
/// This function handles MQ locations that `MqDungeon::from_location_id` might not
/// recognize due to non-standard naming patterns (e.g., `mq_oot_mq_ganon_pot_*`
/// instead of `mq_oot_mq_ganon_castle_*`).
fn is_mq_location_active(location_id: &str, settings: &RandomizerSettings) -> bool {
    // First try the standard detection
    if let Some(dungeon) = MqDungeon::from_location_id(location_id) {
        return settings.is_dungeon_mq(dungeon);
    }

    // For MQ locations that from_location_id doesn't recognize,
    // try additional pattern matching based on dungeon names in the location ID
    // Pattern: mq_oot_mq_<dungeon_shortname>_*

    // Ganon's Castle locations (handles mq_oot_mq_ganon_pot_* pattern)
    if location_id.contains("_ganon_") {
        return settings.is_dungeon_mq(MqDungeon::GanonsCastle);
    }

    // If we can't determine the dungeon, the MQ location should not be active
    // with default settings (conservative approach)
    false
}

/// Helper to check if a location is active for given settings.
pub(crate) fn is_location_active_for_settings(
    location_id: &str,
    settings: &RandomizerSettings,
) -> bool {
    if location_id.starts_with("mq_oot_") {
        is_mq_location_active(location_id, settings)
    } else {
        settings.is_location_active(location_id)
    }
}

/// Returns an iterator over active location mappings based on MQ settings.
///
/// This filters out locations that belong to dungeons where the MQ setting
/// doesn't match the location type (vanilla vs MQ).
///
/// # Arguments
///
/// * `settings` - The randomizer settings containing MQ dungeon selections
pub fn get_active_mappings(
    settings: &RandomizerSettings,
) -> impl Iterator<Item = &'static FlagMapping> + '_ {
    OOT_MAPPINGS
        .values()
        .filter(move |m| is_location_active_for_settings(m.location_id, settings))
}

/// Returns the flag mapping for a location, considering MQ settings.
///
/// This function checks if the location is in an MQ-able dungeon and
/// returns the mapping only if the location matches the current MQ setting.
///
/// # Arguments
///
/// * `location_id` - The location ID to look up
/// * `settings` - The randomizer settings containing MQ dungeon selections
///
/// # Returns
///
/// `Some(mapping)` if the location exists and is active for current settings,
/// `None` if the location doesn't exist or is inactive due to MQ settings.
#[must_use]
pub fn get_active_mapping(
    location_id: &str,
    settings: &RandomizerSettings,
) -> Option<&'static FlagMapping> {
    let mapping = OOT_MAPPINGS.get(location_id)?;
    if is_location_active_for_settings(location_id, settings) {
        Some(mapping)
    } else {
        None
    }
}

/// Returns the count of active (non-MQ-filtered) locations for given settings.
#[must_use]
pub fn active_location_count(settings: &RandomizerSettings) -> usize {
    OOT_MAPPINGS
        .values()
        .filter(|m| is_location_active_for_settings(m.location_id, settings))
        .count()
}

/// Returns the count of active mapped (non-stub) locations for given settings.
#[must_use]
pub fn active_mapped_count(settings: &RandomizerSettings) -> usize {
    OOT_MAPPINGS
        .values()
        .filter(|m| m.is_mapped() && is_location_active_for_settings(m.location_id, settings))
        .count()
}

/// Returns an iterator over active mapped (non-stub) locations for given settings.
pub fn get_active_mapped_locations(
    settings: &RandomizerSettings,
) -> impl Iterator<Item = &'static FlagMapping> + '_ {
    OOT_MAPPINGS
        .values()
        .filter(move |m| m.is_mapped() && is_location_active_for_settings(m.location_id, settings))
}

/// Returns all mappings for a specific dungeon based on its MQ status.
///
/// This returns either vanilla or MQ mappings depending on the dungeon's
/// setting in the provided settings.
pub fn get_dungeon_mappings(
    dungeon: MqDungeon,
    settings: &RandomizerSettings,
) -> impl Iterator<Item = &'static FlagMapping> + '_ {
    let prefix = settings.get_dungeon_location_prefix(dungeon);
    OOT_MAPPINGS
        .values()
        .filter(move |m| m.location_id.starts_with(prefix))
}

/// Converts a Dungeon enum to the corresponding MqDungeon enum.
///
/// This mapping is needed to bridge between the ootr Dungeon enum used in
/// Knowledge.mq and the ootmm MqDungeon enum used in RandomizerSettings.
pub fn dungeon_to_mq_dungeon(dungeon: &Dungeon) -> MqDungeon {
    match dungeon {
        Dungeon::Main(MainDungeon::DekuTree) => MqDungeon::DekuTree,
        Dungeon::Main(MainDungeon::DodongosCavern) => MqDungeon::DodongosCavern,
        Dungeon::Main(MainDungeon::JabuJabu) => MqDungeon::JabuJabu,
        Dungeon::Main(MainDungeon::ForestTemple) => MqDungeon::ForestTemple,
        Dungeon::Main(MainDungeon::FireTemple) => MqDungeon::FireTemple,
        Dungeon::Main(MainDungeon::WaterTemple) => MqDungeon::WaterTemple,
        Dungeon::Main(MainDungeon::ShadowTemple) => MqDungeon::ShadowTemple,
        Dungeon::Main(MainDungeon::SpiritTemple) => MqDungeon::SpiritTemple,
        Dungeon::IceCavern => MqDungeon::IceCavern,
        Dungeon::BottomOfTheWell => MqDungeon::BottomOfTheWell,
        Dungeon::GerudoTrainingGround => MqDungeon::GerudoTrainingGround,
        Dungeon::GanonsCastle => MqDungeon::GanonsCastle,
    }
}

/// Creates RandomizerSettings from Knowledge MQ settings.
///
/// This function converts the `Knowledge.mq` HashMap<Dungeon, Mq> into a
/// `RandomizerSettings` struct with the appropriate MQ dungeon selections.
///
/// # Arguments
///
/// * `mq_settings` - A HashMap mapping dungeons to their MQ status from Knowledge
///
/// # Returns
///
/// A `RandomizerSettings` with MQ dungeons configured according to the Knowledge.
#[must_use]
pub fn mq_settings_from_knowledge(mq_settings: &HashMap<Dungeon, Mq>) -> RandomizerSettings {
    let mut settings = RandomizerSettings::new();

    for (dungeon, mq) in mq_settings {
        let mq_dungeon = dungeon_to_mq_dungeon(dungeon);
        match mq {
            Mq::Mq => settings.set_dungeon_mq(mq_dungeon),
            Mq::Vanilla => settings.set_dungeon_vanilla(mq_dungeon),
        }
    }

    settings
}
