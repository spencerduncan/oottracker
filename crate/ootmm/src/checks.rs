//! Check tracking system for OoTMM.
//!
//! This module provides tracking of completed checks (locations) and evaluation
//! of location accessibility based on game context and logic expressions.
//!
//! # Example
//!
//! ```
//! use ootmm::checks::{CheckTracker, GameContext};
//! use ootmm::world_database::WorldDatabase;
//!
//! // Create a world database with regions and locations
//! let mut db = WorldDatabase::new();
//! db.load_from_str(r#"
//! regions:
//!   - id: "oot_links_house"
//!     name: "Link's House"
//!     game: oot
//!     exits:
//!       - target: "kokiri_forest"
//!         exitType: overworld
//!   - id: "kokiri_forest"
//!     name: "Kokiri Forest"
//!     game: oot
//!     locations:
//!       - id: "kf_chest"
//!         name: "Kokiri Forest Chest"
//!         logic: "true"
//! "#).unwrap();
//!
//! // Create game context with player state
//! let ctx = GameContext::new();
//!
//! // Create check tracker
//! let mut tracker = CheckTracker::new();
//!
//! // Check accessibility and mark as completed
//! assert!(tracker.is_accessible("kf_chest", &db, &ctx));
//! tracker.mark_checked("kf_chest");
//! assert!(tracker.is_checked("kf_chest"));
//! ```

use std::collections::{HashMap, HashSet};

use async_proto::Protocol;
use serde::{Deserialize, Serialize};

use crate::expr::{eval_str, EvalContext, EvalError};
use crate::reachability::is_region_reachable;
use crate::region::Game;
use crate::world_database::WorldDatabase;

/// Error type for check tracking operations.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CheckError {
    /// Location not found in the world database.
    LocationNotFound(String),
    /// Error evaluating logic expression.
    LogicEvaluation { location: String, message: String },
    /// Location has no logic expression defined.
    NoLogicDefined(String),
}

impl std::fmt::Display for CheckError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CheckError::LocationNotFound(id) => write!(f, "location not found: {}", id),
            CheckError::LogicEvaluation { location, message } => {
                write!(f, "logic evaluation error for '{}': {}", location, message)
            }
            CheckError::NoLogicDefined(id) => write!(f, "no logic defined for location: {}", id),
        }
    }
}

impl std::error::Error for CheckError {}

impl From<EvalError> for CheckError {
    fn from(err: EvalError) -> Self {
        CheckError::LogicEvaluation {
            location: String::new(),
            message: err.to_string(),
        }
    }
}

/// Game context providing the current state for logic evaluation.
///
/// This struct holds the player's inventory, triggered events, game settings,
/// and other state needed to evaluate logic expressions.
#[derive(Debug, Clone, Default)]
pub struct GameContext {
    /// Items the player has, with quantities.
    items: HashMap<String, u32>,
    /// Events that have been triggered.
    events: HashSet<String>,
    /// Game settings.
    settings: HashMap<String, bool>,
    /// Enabled tricks.
    tricks: HashSet<String>,
    /// Whether the player is currently Adult Link.
    is_adult: bool,
    /// Current MM time (minutes since Day 1 6:00 AM).
    mm_time: u32,
}

impl GameContext {
    /// Creates a new game context with default values.
    ///
    /// Default state is:
    /// - No items
    /// - No events triggered
    /// - No settings enabled
    /// - No tricks enabled
    /// - Adult Link
    /// - Time 0 (Day 1, 6:00 AM)
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the player age to adult.
    #[must_use]
    pub fn with_adult(mut self) -> Self {
        self.is_adult = true;
        self
    }

    /// Sets the player age to child.
    #[must_use]
    pub fn with_child(mut self) -> Self {
        self.is_adult = false;
        self
    }

    /// Adds an item with the given quantity.
    #[must_use]
    pub fn with_item(mut self, item: impl Into<String>, count: u32) -> Self {
        let item_name = item.into().to_uppercase();
        *self.items.entry(item_name).or_default() += count;
        self
    }

    /// Adds an event as triggered.
    #[must_use]
    pub fn with_event(mut self, event: impl Into<String>) -> Self {
        self.events.insert(event.into());
        self
    }

    /// Sets a game setting.
    #[must_use]
    pub fn with_setting(mut self, setting: impl Into<String>, value: bool) -> Self {
        self.settings.insert(setting.into(), value);
        self
    }

    /// Enables a trick.
    #[must_use]
    pub fn with_trick(mut self, trick: impl Into<String>) -> Self {
        self.tricks.insert(trick.into());
        self
    }

    /// Sets the MM time.
    #[must_use]
    pub fn with_mm_time(mut self, time: u32) -> Self {
        self.mm_time = time;
        self
    }

    /// Sets whether the player is adult.
    pub fn set_adult(&mut self, is_adult: bool) {
        self.is_adult = is_adult;
    }

    /// Adds an item with the given quantity (mutable version).
    pub fn add_item(&mut self, item: impl Into<String>, count: u32) {
        let item_name = item.into().to_uppercase();
        *self.items.entry(item_name).or_default() += count;
    }

    /// Removes an item or reduces its quantity.
    pub fn remove_item(&mut self, item: &str, count: u32) {
        let item_name = item.to_uppercase();
        if let Some(current) = self.items.get_mut(&item_name) {
            *current = current.saturating_sub(count);
            if *current == 0 {
                self.items.remove(&item_name);
            }
        }
    }

    /// Triggers an event.
    pub fn trigger_event(&mut self, event: impl Into<String>) {
        self.events.insert(event.into());
    }

    /// Clears an event.
    pub fn clear_event(&mut self, event: &str) {
        self.events.remove(event);
    }

    /// Sets a setting value.
    pub fn set_setting(&mut self, setting: impl Into<String>, value: bool) {
        self.settings.insert(setting.into(), value);
    }

    /// Enables a trick.
    pub fn enable_trick(&mut self, trick: impl Into<String>) {
        self.tricks.insert(trick.into());
    }

    /// Disables a trick.
    pub fn disable_trick(&mut self, trick: &str) {
        self.tricks.remove(trick);
    }

    /// Sets the MM time.
    pub fn set_mm_time(&mut self, time: u32) {
        self.mm_time = time;
    }

    /// Returns the item count for a given item.
    #[must_use]
    pub fn item_count(&self, item: &str) -> u32 {
        self.items.get(&item.to_uppercase()).copied().unwrap_or(0)
    }

    /// Returns whether an event has been triggered.
    #[must_use]
    pub fn has_event(&self, event: &str) -> bool {
        self.events.contains(event)
    }
}

impl EvalContext for GameContext {
    fn has_item(&self, item: &str, count: u32) -> bool {
        self.items
            .get(&item.to_uppercase())
            .map(|&c| c >= count)
            .unwrap_or(false)
    }

    fn event(&self, name: &str) -> bool {
        self.events.contains(name)
    }

    fn setting(&self, name: &str) -> Option<bool> {
        self.settings.get(name).copied()
    }

    fn trick(&self, name: &str) -> bool {
        self.tricks.contains(name)
    }

    fn is_adult(&self) -> bool {
        self.is_adult
    }

    fn is_child(&self) -> bool {
        !self.is_adult
    }

    fn mm_time(&self) -> u32 {
        self.mm_time
    }
}

/// Status of a check/location.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CheckStatus {
    /// The check has been completed.
    Checked,
    /// The check is reachable/accessible with current items and state.
    Accessible,
    /// The check is not yet reachable.
    Inaccessible,
    /// The check status is unknown (e.g., logic evaluation failed).
    Unknown,
}

/// A reference to an accessible check with its metadata.
#[derive(Debug, Clone)]
pub struct AccessibleCheck<'a> {
    /// The location ID.
    pub id: &'a str,
    /// The human-readable name.
    pub name: &'a str,
    /// The region ID containing this location.
    pub region_id: &'a str,
    /// Which game this location belongs to.
    pub game: Game,
}

/// Tracks which checks have been completed and evaluates accessibility.
///
/// The `CheckTracker` maintains a set of completed location IDs and provides
/// methods to check and query location accessibility based on game state.
#[derive(Debug, Clone, Default, PartialEq, Eq, Protocol, Deserialize, Serialize)]
pub struct CheckTracker {
    /// Set of completed location IDs.
    checked: HashSet<String>,
}

impl CheckTracker {
    /// Creates a new empty check tracker.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Creates a check tracker with pre-checked locations.
    #[must_use]
    pub fn with_checked(checked: impl IntoIterator<Item = impl Into<String>>) -> Self {
        Self {
            checked: checked.into_iter().map(Into::into).collect(),
        }
    }

    /// Marks a location as checked/completed.
    ///
    /// # Arguments
    ///
    /// * `location` - The location ID to mark as checked
    pub fn mark_checked(&mut self, location: &str) {
        self.checked.insert(location.to_string());
    }

    /// Marks a location as unchecked/incomplete.
    ///
    /// # Arguments
    ///
    /// * `location` - The location ID to mark as unchecked
    pub fn mark_unchecked(&mut self, location: &str) {
        self.checked.remove(location);
    }

    /// Returns whether a location has been checked.
    #[must_use]
    pub fn is_checked(&self, location: &str) -> bool {
        self.checked.contains(location)
    }

    /// Returns the number of checked locations.
    #[must_use]
    pub fn checked_count(&self) -> usize {
        self.checked.len()
    }

    /// Returns an iterator over all checked location IDs.
    pub fn checked_locations(&self) -> impl Iterator<Item = &str> {
        self.checked.iter().map(String::as_str)
    }

    /// Clears all checked locations.
    pub fn clear(&mut self) {
        self.checked.clear();
    }

    /// Evaluates whether a location is accessible given the current game context.
    ///
    /// A location is accessible if:
    /// 1. It exists in the world database
    /// 2. Its logic expression evaluates to true (or it has no logic, meaning always accessible)
    ///
    /// # Arguments
    ///
    /// * `location` - The location ID to check
    /// * `world` - The world database containing location definitions
    /// * `context` - The game context for logic evaluation
    ///
    /// # Returns
    ///
    /// `true` if the location is accessible, `false` otherwise.
    /// Returns `false` if the location doesn't exist or logic evaluation fails.
    #[must_use]
    pub fn is_accessible(
        &self,
        location: &str,
        world: &WorldDatabase,
        context: &GameContext,
    ) -> bool {
        self.evaluate_accessibility(location, world, context)
            .unwrap_or(false)
    }

    /// Evaluates accessibility with detailed error information.
    ///
    /// # Arguments
    ///
    /// * `location` - The location ID to check
    /// * `world` - The world database containing location definitions
    /// * `context` - The game context for logic evaluation
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - The location doesn't exist in the world database
    /// - Logic evaluation fails
    pub fn evaluate_accessibility(
        &self,
        location: &str,
        world: &WorldDatabase,
        context: &GameContext,
    ) -> Result<bool, CheckError> {
        let (loc, region_id) = world
            .get_location(location)
            .ok_or_else(|| CheckError::LocationNotFound(location.to_string()))?;

        // Get the region to determine the game
        let region = world
            .get_region(region_id)
            .ok_or_else(|| CheckError::LocationNotFound(location.to_string()))?;

        // First check if the region is reachable from spawn
        if !is_region_reachable(world, context, region.game, region_id) {
            return Ok(false);
        }

        // If no logic is defined, the location is always accessible
        let Some(logic) = &loc.logic else {
            return Ok(true);
        };

        // Evaluate the logic expression
        eval_str(logic, context).map_err(|e| CheckError::LogicEvaluation {
            location: location.to_string(),
            message: e.to_string(),
        })
    }

    /// Returns a list of all accessible checks that haven't been completed.
    ///
    /// # Arguments
    ///
    /// * `world` - The world database containing location definitions
    /// * `context` - The game context for logic evaluation
    ///
    /// # Returns
    ///
    /// A vector of `AccessibleCheck` structs for all accessible, unchecked locations.
    #[must_use]
    pub fn get_accessible_checks<'a>(
        &self,
        world: &'a WorldDatabase,
        context: &GameContext,
    ) -> Vec<AccessibleCheck<'a>> {
        world
            .locations()
            .filter_map(|(loc, region_id)| {
                // Skip already checked locations
                if self.is_checked(&loc.id) {
                    return None;
                }

                // Check if accessible
                let is_accessible = self
                    .evaluate_accessibility(&loc.id, world, context)
                    .unwrap_or(false);

                if is_accessible {
                    let region = world.get_region(region_id)?;
                    Some(AccessibleCheck {
                        id: &loc.id,
                        name: &loc.name,
                        region_id,
                        game: region.game,
                    })
                } else {
                    None
                }
            })
            .collect()
    }

    /// Returns accessible checks for a specific game.
    ///
    /// # Arguments
    ///
    /// * `world` - The world database containing location definitions
    /// * `context` - The game context for logic evaluation
    /// * `game` - The game to filter by (OoT or MM)
    #[must_use]
    pub fn get_accessible_checks_for_game<'a>(
        &self,
        world: &'a WorldDatabase,
        context: &GameContext,
        game: Game,
    ) -> Vec<AccessibleCheck<'a>> {
        self.get_accessible_checks(world, context)
            .into_iter()
            .filter(|check| check.game == game)
            .collect()
    }

    /// Returns the status of a specific location.
    ///
    /// # Arguments
    ///
    /// * `location` - The location ID to check
    /// * `world` - The world database containing location definitions
    /// * `context` - The game context for logic evaluation
    #[must_use]
    pub fn get_status(
        &self,
        location: &str,
        world: &WorldDatabase,
        context: &GameContext,
    ) -> CheckStatus {
        if self.is_checked(location) {
            return CheckStatus::Checked;
        }

        match self.evaluate_accessibility(location, world, context) {
            Ok(true) => CheckStatus::Accessible,
            Ok(false) => CheckStatus::Inaccessible,
            Err(_) => CheckStatus::Unknown,
        }
    }

    /// Returns a summary of all location statuses.
    ///
    /// # Arguments
    ///
    /// * `world` - The world database containing location definitions
    /// * `context` - The game context for logic evaluation
    ///
    /// # Returns
    ///
    /// A tuple of (checked, accessible, inaccessible, unknown) counts.
    #[must_use]
    pub fn get_summary(
        &self,
        world: &WorldDatabase,
        context: &GameContext,
    ) -> (usize, usize, usize, usize) {
        let mut checked = 0;
        let mut accessible = 0;
        let mut inaccessible = 0;
        let mut unknown = 0;

        for (loc, _) in world.locations() {
            match self.get_status(&loc.id, world, context) {
                CheckStatus::Checked => checked += 1,
                CheckStatus::Accessible => accessible += 1,
                CheckStatus::Inaccessible => inaccessible += 1,
                CheckStatus::Unknown => unknown += 1,
            }
        }

        (checked, accessible, inaccessible, unknown)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::region::Game;

    fn create_test_world() -> WorldDatabase {
        let mut db = WorldDatabase::new();
        db.load_from_str(
            r#"
regions:
  # OoT spawn region with exits to test regions
  - id: "oot_links_house"
    name: "Link's House"
    game: oot
    exits:
      - target: "kokiri_forest"
        exitType: overworld
      - target: "lost_woods"
        exitType: overworld
      - target: "forest_temple"
        exitType: dungeon
  - id: "kokiri_forest"
    name: "Kokiri Forest"
    game: oot
    locations:
      - id: "kf_midos_chest"
        name: "Mido's House Chest"
        locationType: chest
      - id: "kf_sword_chest"
        name: "Kokiri Sword Chest"
        locationType: chest
        logic: "true"
      - id: "kf_shop_item"
        name: "Shop Item"
        locationType: shop
        logic: "has(RUPEES, 40)"
  - id: "lost_woods"
    name: "Lost Woods"
    game: oot
    locations:
      - id: "lw_skull_kid"
        name: "Skull Kid Gift"
        locationType: npc
        logic: "has(OCARINA)"
      - id: "lw_target"
        name: "Target Shooting"
        locationType: event
        logic: "is_child && has(SLINGSHOT)"
  - id: "forest_temple"
    name: "Forest Temple"
    game: oot
    locations:
      - id: "ft_first_chest"
        name: "First Chest"
        locationType: chest
        logic: "is_adult && has(HOOKSHOT)"
  # MM spawn region with exit to test region
  - id: "mm_clock_tower"
    name: "Clock Tower"
    game: mm
    exits:
      - target: "clock_town"
        exitType: overworld
  - id: "clock_town"
    name: "Clock Town"
    game: mm
    locations:
      - id: "ct_chest"
        name: "Clock Town Chest"
        locationType: chest
        logic: "true"
"#,
        )
        .unwrap();
        db
    }

    // --- GameContext tests ---

    #[test]
    fn test_game_context_new() {
        let ctx = GameContext::new();
        assert!(!ctx.is_adult()); // Default is not adult
        assert!(ctx.is_child());
        assert_eq!(ctx.mm_time(), 0);
    }

    #[test]
    fn test_game_context_with_items() {
        let ctx = GameContext::new()
            .with_item("HOOKSHOT", 1)
            .with_item("BOMBS", 20);

        assert!(ctx.has_item("HOOKSHOT", 1));
        assert!(ctx.has_item("hookshot", 1)); // Case insensitive
        assert!(ctx.has_item("BOMBS", 10));
        assert!(ctx.has_item("BOMBS", 20));
        assert!(!ctx.has_item("BOMBS", 21));
        assert!(!ctx.has_item("BOW", 1));
    }

    #[test]
    fn test_game_context_with_events() {
        let ctx = GameContext::new()
            .with_event("MIDO_MOVED")
            .with_event("DEKU_TREE_CLEAR");

        assert!(ctx.event("MIDO_MOVED"));
        assert!(ctx.event("DEKU_TREE_CLEAR"));
        assert!(!ctx.event("GANON_DEFEATED"));
    }

    #[test]
    fn test_game_context_with_settings() {
        let ctx = GameContext::new()
            .with_setting("skip_child_zelda", true)
            .with_setting("shuffle_songs", false);

        assert_eq!(ctx.setting("skip_child_zelda"), Some(true));
        assert_eq!(ctx.setting("shuffle_songs"), Some(false));
        assert_eq!(ctx.setting("nonexistent"), None);
    }

    #[test]
    fn test_game_context_with_tricks() {
        let ctx = GameContext::new()
            .with_trick("hover_boost")
            .with_trick("bomb_jump");

        assert!(ctx.trick("hover_boost"));
        assert!(ctx.trick("bomb_jump"));
        assert!(!ctx.trick("superslide"));
    }

    #[test]
    fn test_game_context_mutable_operations() {
        let mut ctx = GameContext::new();

        ctx.add_item("HOOKSHOT", 1);
        assert!(ctx.has_item("HOOKSHOT", 1));

        ctx.add_item("BOMBS", 10);
        ctx.add_item("BOMBS", 10);
        assert!(ctx.has_item("BOMBS", 20));

        ctx.remove_item("BOMBS", 5);
        assert!(ctx.has_item("BOMBS", 15));
        assert!(!ctx.has_item("BOMBS", 16));

        ctx.remove_item("BOMBS", 100);
        assert!(!ctx.has_item("BOMBS", 1));

        ctx.trigger_event("TEST_EVENT");
        assert!(ctx.has_event("TEST_EVENT"));
        ctx.clear_event("TEST_EVENT");
        assert!(!ctx.has_event("TEST_EVENT"));

        ctx.enable_trick("hover_boost");
        assert!(ctx.trick("hover_boost"));
        ctx.disable_trick("hover_boost");
        assert!(!ctx.trick("hover_boost"));
    }

    #[test]
    fn test_game_context_age() {
        let ctx = GameContext::new().with_adult();
        assert!(ctx.is_adult());
        assert!(!ctx.is_child());

        let ctx = GameContext::new().with_child();
        assert!(!ctx.is_adult());
        assert!(ctx.is_child());
    }

    // --- CheckTracker tests ---

    #[test]
    fn test_check_tracker_new() {
        let tracker = CheckTracker::new();
        assert_eq!(tracker.checked_count(), 0);
        assert!(!tracker.is_checked("any_location"));
    }

    #[test]
    fn test_check_tracker_with_checked() {
        let tracker = CheckTracker::with_checked(["loc1", "loc2", "loc3"]);
        assert_eq!(tracker.checked_count(), 3);
        assert!(tracker.is_checked("loc1"));
        assert!(tracker.is_checked("loc2"));
        assert!(tracker.is_checked("loc3"));
        assert!(!tracker.is_checked("loc4"));
    }

    #[test]
    fn test_mark_checked() {
        let mut tracker = CheckTracker::new();

        tracker.mark_checked("kf_chest");
        assert!(tracker.is_checked("kf_chest"));
        assert_eq!(tracker.checked_count(), 1);

        // Marking again should not increase count
        tracker.mark_checked("kf_chest");
        assert_eq!(tracker.checked_count(), 1);

        tracker.mark_checked("lw_chest");
        assert_eq!(tracker.checked_count(), 2);
    }

    #[test]
    fn test_mark_unchecked() {
        let mut tracker = CheckTracker::new();

        tracker.mark_checked("kf_chest");
        tracker.mark_checked("lw_chest");
        assert_eq!(tracker.checked_count(), 2);

        tracker.mark_unchecked("kf_chest");
        assert!(!tracker.is_checked("kf_chest"));
        assert!(tracker.is_checked("lw_chest"));
        assert_eq!(tracker.checked_count(), 1);

        // Unchecking non-existent should not panic
        tracker.mark_unchecked("nonexistent");
        assert_eq!(tracker.checked_count(), 1);
    }

    #[test]
    fn test_checked_locations_iterator() {
        let tracker = CheckTracker::with_checked(["loc1", "loc2"]);

        let locations: HashSet<_> = tracker.checked_locations().collect();
        assert_eq!(locations.len(), 2);
        assert!(locations.contains("loc1"));
        assert!(locations.contains("loc2"));
    }

    #[test]
    fn test_clear() {
        let mut tracker = CheckTracker::with_checked(["loc1", "loc2", "loc3"]);
        assert_eq!(tracker.checked_count(), 3);

        tracker.clear();
        assert_eq!(tracker.checked_count(), 0);
        assert!(!tracker.is_checked("loc1"));
    }

    // --- is_accessible tests ---

    #[test]
    fn test_is_accessible_no_logic() {
        let world = create_test_world();
        let ctx = GameContext::new();
        let tracker = CheckTracker::new();

        // Location with no logic is always accessible
        assert!(tracker.is_accessible("kf_midos_chest", &world, &ctx));
    }

    #[test]
    fn test_is_accessible_true_logic() {
        let world = create_test_world();
        let ctx = GameContext::new();
        let tracker = CheckTracker::new();

        // Location with "true" logic is always accessible
        assert!(tracker.is_accessible("kf_sword_chest", &world, &ctx));
    }

    #[test]
    fn test_is_accessible_item_requirement() {
        let world = create_test_world();
        let tracker = CheckTracker::new();

        // Without ocarina, skull kid is not accessible
        let ctx = GameContext::new();
        assert!(!tracker.is_accessible("lw_skull_kid", &world, &ctx));

        // With ocarina, skull kid is accessible
        let ctx = GameContext::new().with_item("OCARINA", 1);
        assert!(tracker.is_accessible("lw_skull_kid", &world, &ctx));
    }

    #[test]
    fn test_is_accessible_age_and_item() {
        let world = create_test_world();
        let tracker = CheckTracker::new();

        // Need to be adult with hookshot
        let ctx = GameContext::new().with_child();
        assert!(!tracker.is_accessible("ft_first_chest", &world, &ctx));

        let ctx = GameContext::new().with_adult();
        assert!(!tracker.is_accessible("ft_first_chest", &world, &ctx));

        let ctx = GameContext::new().with_child().with_item("HOOKSHOT", 1);
        assert!(!tracker.is_accessible("ft_first_chest", &world, &ctx));

        let ctx = GameContext::new().with_adult().with_item("HOOKSHOT", 1);
        assert!(tracker.is_accessible("ft_first_chest", &world, &ctx));
    }

    #[test]
    fn test_is_accessible_child_with_slingshot() {
        let world = create_test_world();
        let tracker = CheckTracker::new();

        let ctx = GameContext::new().with_adult().with_item("SLINGSHOT", 1);
        assert!(!tracker.is_accessible("lw_target", &world, &ctx));

        let ctx = GameContext::new().with_child();
        assert!(!tracker.is_accessible("lw_target", &world, &ctx));

        let ctx = GameContext::new().with_child().with_item("SLINGSHOT", 1);
        assert!(tracker.is_accessible("lw_target", &world, &ctx));
    }

    #[test]
    fn test_is_accessible_nonexistent_location() {
        let world = create_test_world();
        let ctx = GameContext::new();
        let tracker = CheckTracker::new();

        // Non-existent location should return false
        assert!(!tracker.is_accessible("nonexistent", &world, &ctx));
    }

    // --- evaluate_accessibility tests ---

    #[test]
    fn test_evaluate_accessibility_not_found() {
        let world = create_test_world();
        let ctx = GameContext::new();
        let tracker = CheckTracker::new();

        let result = tracker.evaluate_accessibility("nonexistent", &world, &ctx);
        assert!(matches!(result, Err(CheckError::LocationNotFound(_))));
    }

    #[test]
    fn test_evaluate_accessibility_success() {
        let world = create_test_world();
        let ctx = GameContext::new().with_item("RUPEES", 40);
        let tracker = CheckTracker::new();

        let result = tracker.evaluate_accessibility("kf_shop_item", &world, &ctx);
        assert!(result.is_ok());
        assert!(result.unwrap());
    }

    // --- get_accessible_checks tests ---

    #[test]
    fn test_get_accessible_checks() {
        let world = create_test_world();
        let ctx = GameContext::new().with_child();
        let tracker = CheckTracker::new();

        let accessible = tracker.get_accessible_checks(&world, &ctx);

        // Should include locations with no logic or "true" logic
        let ids: Vec<_> = accessible.iter().map(|c| c.id).collect();
        assert!(ids.contains(&"kf_midos_chest"));
        assert!(ids.contains(&"kf_sword_chest"));
        assert!(ids.contains(&"ct_chest"));

        // Should not include locations requiring items
        assert!(!ids.contains(&"lw_skull_kid"));
        assert!(!ids.contains(&"ft_first_chest"));
    }

    #[test]
    fn test_get_accessible_checks_excludes_checked() {
        let world = create_test_world();
        let ctx = GameContext::new();
        let mut tracker = CheckTracker::new();

        // Mark some as checked
        tracker.mark_checked("kf_midos_chest");
        tracker.mark_checked("kf_sword_chest");

        let accessible = tracker.get_accessible_checks(&world, &ctx);
        let ids: Vec<_> = accessible.iter().map(|c| c.id).collect();

        // Should not include checked locations
        assert!(!ids.contains(&"kf_midos_chest"));
        assert!(!ids.contains(&"kf_sword_chest"));

        // Should still include unchecked accessible locations
        assert!(ids.contains(&"ct_chest"));
    }

    #[test]
    fn test_get_accessible_checks_with_items() {
        let world = create_test_world();
        let ctx = GameContext::new()
            .with_child()
            .with_item("OCARINA", 1)
            .with_item("SLINGSHOT", 1);
        let tracker = CheckTracker::new();

        let accessible = tracker.get_accessible_checks(&world, &ctx);
        let ids: Vec<_> = accessible.iter().map(|c| c.id).collect();

        // Should now include locations requiring ocarina or slingshot
        assert!(ids.contains(&"lw_skull_kid"));
        assert!(ids.contains(&"lw_target"));
    }

    #[test]
    fn test_get_accessible_checks_metadata() {
        let world = create_test_world();
        let ctx = GameContext::new();
        let tracker = CheckTracker::new();

        let accessible = tracker.get_accessible_checks(&world, &ctx);
        let kf_check = accessible
            .iter()
            .find(|c| c.id == "kf_midos_chest")
            .unwrap();

        assert_eq!(kf_check.name, "Mido's House Chest");
        assert_eq!(kf_check.region_id, "kokiri_forest");
        assert_eq!(kf_check.game, Game::Oot);
    }

    #[test]
    fn test_get_accessible_checks_for_game() {
        let world = create_test_world();
        let ctx = GameContext::new();
        let tracker = CheckTracker::new();

        let oot_checks = tracker.get_accessible_checks_for_game(&world, &ctx, Game::Oot);
        let mm_checks = tracker.get_accessible_checks_for_game(&world, &ctx, Game::Mm);

        // OoT checks
        let oot_ids: Vec<_> = oot_checks.iter().map(|c| c.id).collect();
        assert!(oot_ids.contains(&"kf_midos_chest"));
        assert!(!oot_ids.contains(&"ct_chest"));

        // MM checks
        let mm_ids: Vec<_> = mm_checks.iter().map(|c| c.id).collect();
        assert!(mm_ids.contains(&"ct_chest"));
        assert!(!mm_ids.contains(&"kf_midos_chest"));
    }

    // --- get_status tests ---

    #[test]
    fn test_get_status() {
        let world = create_test_world();
        let ctx = GameContext::new();
        let mut tracker = CheckTracker::new();

        // Checked location
        tracker.mark_checked("kf_sword_chest");
        assert_eq!(
            tracker.get_status("kf_sword_chest", &world, &ctx),
            CheckStatus::Checked
        );

        // Accessible location
        assert_eq!(
            tracker.get_status("kf_midos_chest", &world, &ctx),
            CheckStatus::Accessible
        );

        // Inaccessible location
        assert_eq!(
            tracker.get_status("ft_first_chest", &world, &ctx),
            CheckStatus::Inaccessible
        );

        // Unknown (non-existent) location
        assert_eq!(
            tracker.get_status("nonexistent", &world, &ctx),
            CheckStatus::Unknown
        );
    }

    // --- get_summary tests ---

    #[test]
    fn test_get_summary() {
        let world = create_test_world();
        let ctx = GameContext::new();
        let mut tracker = CheckTracker::new();

        tracker.mark_checked("kf_midos_chest");

        let (checked, accessible, inaccessible, unknown) = tracker.get_summary(&world, &ctx);

        assert_eq!(checked, 1);
        // kf_sword_chest (true), ct_chest (true) = 2 accessible
        assert_eq!(accessible, 2);
        // lw_skull_kid, lw_target, ft_first_chest, kf_shop_item = 4 inaccessible
        assert_eq!(inaccessible, 4);
        assert_eq!(unknown, 0);
    }

    // --- CheckError tests ---

    #[test]
    fn test_check_error_display() {
        let err = CheckError::LocationNotFound("test_loc".to_string());
        assert_eq!(err.to_string(), "location not found: test_loc");

        let err = CheckError::LogicEvaluation {
            location: "test_loc".to_string(),
            message: "parse error".to_string(),
        };
        assert_eq!(
            err.to_string(),
            "logic evaluation error for 'test_loc': parse error"
        );

        let err = CheckError::NoLogicDefined("test_loc".to_string());
        assert_eq!(err.to_string(), "no logic defined for location: test_loc");
    }
}
