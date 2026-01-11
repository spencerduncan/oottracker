//! Collection accessor methods for RandomizerSettings.
//!
//! This module contains methods for managing starting items,
//! junk locations, special conditions, and world flags.

use crate::settings::core::RandomizerSettings;
use crate::settings::special::SpecialCondition;

impl RandomizerSettings {
    // === Special Condition Methods ===

    /// Gets a special condition by name.
    #[must_use]
    pub fn get_special_condition(&self, name: &str) -> Option<&SpecialCondition> {
        self.special_conditions.get(name)
    }

    /// Gets the bridge special condition.
    ///
    /// Returns the special condition for custom rainbow bridge requirements
    /// when `rainbow_bridge` is set to `Custom`.
    #[must_use]
    pub fn bridge_condition(&self) -> Option<&SpecialCondition> {
        self.special_conditions.get("bridge")
    }

    /// Checks if a special condition exists.
    #[must_use]
    pub fn has_special_condition(&self, name: &str) -> bool {
        self.special_conditions.contains_key(name)
    }

    /// Sets a special condition.
    pub fn set_special_condition(&mut self, name: impl Into<String>, condition: SpecialCondition) {
        self.special_conditions.insert(name.into(), condition);
    }

    /// Removes a special condition.
    pub fn remove_special_condition(&mut self, name: &str) {
        self.special_conditions.remove(name);
    }

    /// Returns an iterator over special conditions.
    pub fn special_conditions_iter(&self) -> impl Iterator<Item = (&String, &SpecialCondition)> {
        self.special_conditions.iter()
    }

    /// Returns the number of special conditions.
    #[must_use]
    pub fn special_conditions_count(&self) -> usize {
        self.special_conditions.len()
    }

    // === Junk Location Methods ===

    /// Checks if a location is designated as junk.
    #[must_use]
    pub fn is_junk_location(&self, location: &str) -> bool {
        self.junk_locations.contains(location)
    }

    /// Adds a location to the junk locations set.
    pub fn add_junk_location(&mut self, location: impl Into<String>) {
        self.junk_locations.insert(location.into());
    }

    /// Removes a location from the junk locations set.
    pub fn remove_junk_location(&mut self, location: &str) {
        self.junk_locations.remove(location);
    }

    /// Returns an iterator over junk locations.
    pub fn junk_locations_iter(&self) -> impl Iterator<Item = &String> {
        self.junk_locations.iter()
    }

    /// Returns the number of junk locations.
    #[must_use]
    pub fn junk_locations_count(&self) -> usize {
        self.junk_locations.len()
    }

    // === Starting Items Methods ===

    /// Returns the quantity of a starting item.
    ///
    /// Returns 0 if the item is not in the starting items.
    #[must_use]
    pub fn starting_item_quantity(&self, item: &str) -> u32 {
        self.starting_items.get(item).copied().unwrap_or(0)
    }

    /// Checks if a starting item is present (quantity > 0).
    #[must_use]
    pub fn has_starting_item(&self, item: &str) -> bool {
        self.starting_item_quantity(item) > 0
    }

    /// Adds or updates a starting item quantity.
    pub fn set_starting_item(&mut self, item: impl Into<String>, quantity: u32) {
        if quantity > 0 {
            self.starting_items.insert(item.into(), quantity);
        } else {
            self.starting_items.remove(&item.into());
        }
    }

    /// Removes a starting item.
    pub fn remove_starting_item(&mut self, item: &str) {
        self.starting_items.remove(item);
    }

    /// Returns an iterator over starting items and their quantities.
    pub fn starting_items_iter(&self) -> impl Iterator<Item = (&String, &u32)> {
        self.starting_items.iter()
    }

    /// Returns the number of distinct starting items.
    #[must_use]
    pub fn starting_items_count(&self) -> usize {
        self.starting_items.len()
    }

    // === World Flags Accessors ===

    /// Returns whether OoT world is enabled.
    #[must_use]
    pub fn is_oot_enabled(&self) -> bool {
        self.world_flags.oot_enabled
    }

    /// Returns whether MM world is enabled.
    #[must_use]
    pub fn is_mm_enabled(&self) -> bool {
        self.world_flags.mm_enabled
    }

    /// Returns whether shared items are enabled in world flags.
    #[must_use]
    pub fn world_shared_items(&self) -> bool {
        self.world_flags.shared_items
    }

    /// Returns whether shared masks are enabled in world flags.
    #[must_use]
    pub fn world_shared_masks(&self) -> bool {
        self.world_flags.shared_masks
    }
}
