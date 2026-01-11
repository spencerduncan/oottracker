//! Master Quest dungeon accessor methods for RandomizerSettings.

use crate::settings::core::RandomizerSettings;
use crate::settings::dungeons::MqDungeon;

impl RandomizerSettings {
    /// Checks if a dungeon is set to Master Quest.
    #[must_use]
    pub fn is_dungeon_mq(&self, dungeon: MqDungeon) -> bool {
        self.mq_dungeons.contains(&dungeon)
    }

    /// Checks if a dungeon is set to Master Quest by its string identifier.
    #[must_use]
    pub fn is_dungeon_mq_by_name(&self, name: &str) -> bool {
        MqDungeon::parse(name)
            .map(|d| self.mq_dungeons.contains(&d))
            .unwrap_or(false)
    }

    /// Sets a dungeon to Master Quest mode.
    pub fn set_dungeon_mq(&mut self, dungeon: MqDungeon) {
        self.mq_dungeons.insert(dungeon);
    }

    /// Sets a dungeon to vanilla (non-MQ) mode.
    pub fn set_dungeon_vanilla(&mut self, dungeon: MqDungeon) {
        self.mq_dungeons.remove(&dungeon);
    }

    /// Sets all dungeons to Master Quest mode.
    pub fn set_all_dungeons_mq(&mut self) {
        for &dungeon in MqDungeon::all() {
            self.mq_dungeons.insert(dungeon);
        }
    }

    /// Sets all dungeons to vanilla (non-MQ) mode.
    pub fn set_all_dungeons_vanilla(&mut self) {
        self.mq_dungeons.clear();
    }

    /// Returns the location ID prefix for a dungeon based on its MQ status.
    ///
    /// This is used to determine which set of flag mappings to use.
    #[must_use]
    pub fn get_dungeon_location_prefix(&self, dungeon: MqDungeon) -> &'static str {
        if self.is_dungeon_mq(dungeon) {
            dungeon.mq_location_prefix()
        } else {
            dungeon.vanilla_location_prefix()
        }
    }

    /// Determines if a location ID should be active based on MQ settings.
    ///
    /// Returns `true` if the location matches the current MQ/vanilla state
    /// of its dungeon, or if the location is not in an MQ-able dungeon.
    #[must_use]
    pub fn is_location_active(&self, location_id: &str) -> bool {
        // Check if this is an MQ dungeon location
        if let Some(dungeon) = MqDungeon::from_location_id(location_id) {
            let is_mq_location = location_id.starts_with("mq_oot_");
            let dungeon_is_mq = self.is_dungeon_mq(dungeon);

            // Location is active if its MQ status matches the dungeon's setting
            is_mq_location == dungeon_is_mq
        } else {
            // Non-dungeon locations are always active
            true
        }
    }

    /// Returns the count of MQ dungeons.
    #[must_use]
    pub fn mq_dungeon_count(&self) -> usize {
        self.mq_dungeons.len()
    }
}
