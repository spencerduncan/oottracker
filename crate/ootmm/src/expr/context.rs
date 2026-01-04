//! Game context for expression evaluation.
//!
//! This module provides the [`GameContext`] struct which implements the [`EvalContext`] trait.
//! It holds all game state needed for evaluating OoTMM logic expressions.

use crate::expr::EvalContext;
use crate::item::Item;
use crate::settings::RandomizerSettings;
use std::collections::{HashMap, HashSet};

/// The player's current age state.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Age {
    /// Child Link
    #[default]
    Child,
    /// Adult Link
    Adult,
}

impl Age {
    /// Returns true if the player is Adult Link.
    #[must_use]
    pub const fn is_adult(self) -> bool {
        matches!(self, Age::Adult)
    }

    /// Returns true if the player is Child Link.
    #[must_use]
    pub const fn is_child(self) -> bool {
        matches!(self, Age::Child)
    }
}

/// Game context for expression evaluation.
///
/// This struct holds all the game state needed for evaluating OoTMM logic expressions:
/// - Current inventory (items and counts)
/// - Current age (Child/Adult)
/// - Enabled tricks
/// - Randomizer settings
/// - Event flags
/// - MM time
///
/// # Example
///
/// ```
/// use ootmm::expr::{GameContext, Age, Evaluator};
///
/// let mut ctx = GameContext::new();
/// ctx.set_item("HOOKSHOT", 1);
/// ctx.set_age(Age::Adult);
/// ctx.add_event("FOREST_TEMPLE_CLEAR");
///
/// let evaluator = Evaluator::new(&ctx);
/// let result = evaluator.eval_str("has(HOOKSHOT) && is_adult").unwrap();
/// assert!(result);
/// ```
#[derive(Debug, Clone, Default)]
pub struct GameContext {
    /// Item inventory with counts (case-insensitive keys stored uppercase).
    inventory: HashMap<String, u32>,
    /// Current player age.
    age: Age,
    /// Set of enabled tricks.
    tricks: HashSet<String>,
    /// Randomizer settings configuration.
    randomizer_settings: RandomizerSettings,
    /// Legacy boolean settings (for backward compatibility).
    legacy_settings: HashMap<String, bool>,
    /// Set of triggered game events.
    events: HashSet<String>,
    /// Current MM time in minutes since Day 1 at 6:00 AM.
    /// Valid range: 0-4319 (72 hours = 4320 minutes).
    mm_time: u32,
}

impl GameContext {
    /// Creates a new empty game context.
    ///
    /// The context starts with:
    /// - Empty inventory
    /// - Child age (default)
    /// - No tricks enabled
    /// - No settings configured
    /// - No events triggered
    /// - MM time at 0 (Day 1, 6:00 AM)
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    // --- Inventory methods ---

    /// Sets an item count in the inventory.
    ///
    /// Item names are stored case-insensitively (converted to uppercase).
    /// Setting count to 0 removes the item from inventory.
    pub fn set_item(&mut self, item: &str, count: u32) {
        let key = item.to_uppercase();
        if count == 0 {
            self.inventory.remove(&key);
        } else {
            self.inventory.insert(key, count);
        }
    }

    /// Gets the count of an item in the inventory.
    ///
    /// Returns 0 if the item is not in the inventory.
    #[must_use]
    pub fn get_item(&self, item: &str) -> u32 {
        self.inventory
            .get(&item.to_uppercase())
            .copied()
            .unwrap_or(0)
    }

    /// Adds an item to the inventory using the Item enum.
    ///
    /// This method provides type-safe item addition and validates
    /// the item exists in the game.
    pub fn add_item(&mut self, item: Item, count: u32) {
        let name = format!("{:?}", item);
        // Convert enum variant to uppercase for storage
        let key = self.normalize_item_name(&name);
        if count == 0 {
            self.inventory.remove(&key);
        } else {
            let current = self.inventory.get(&key).copied().unwrap_or(0);
            self.inventory.insert(key, current + count);
        }
    }

    /// Removes an item from the inventory.
    pub fn remove_item(&mut self, item: &str) {
        self.inventory.remove(&item.to_uppercase());
    }

    /// Clears all items from the inventory.
    pub fn clear_inventory(&mut self) {
        self.inventory.clear();
    }

    /// Returns the inventory as a reference to the internal HashMap.
    #[must_use]
    pub fn inventory(&self) -> &HashMap<String, u32> {
        &self.inventory
    }

    // --- Age methods ---

    /// Sets the current player age.
    pub fn set_age(&mut self, age: Age) {
        self.age = age;
    }

    /// Gets the current player age.
    #[must_use]
    pub const fn age(&self) -> Age {
        self.age
    }

    // --- Trick methods ---

    /// Enables a trick.
    pub fn add_trick(&mut self, trick: &str) {
        self.tricks.insert(trick.to_string());
    }

    /// Disables a trick.
    pub fn remove_trick(&mut self, trick: &str) {
        self.tricks.remove(trick);
    }

    /// Checks if a trick is enabled.
    #[must_use]
    pub fn has_trick(&self, trick: &str) -> bool {
        self.tricks.contains(trick)
    }

    /// Clears all tricks.
    pub fn clear_tricks(&mut self) {
        self.tricks.clear();
    }

    /// Returns the set of enabled tricks.
    #[must_use]
    pub fn tricks(&self) -> &HashSet<String> {
        &self.tricks
    }

    // --- Setting methods ---

    /// Sets a legacy boolean game setting value.
    ///
    /// For new code, prefer using `set_randomizer_settings` or `randomizer_settings_mut`.
    pub fn set_setting(&mut self, name: &str, value: bool) {
        self.legacy_settings.insert(name.to_string(), value);
    }

    /// Gets a legacy boolean game setting value.
    #[must_use]
    pub fn get_setting(&self, name: &str) -> Option<bool> {
        self.legacy_settings.get(name).copied()
    }

    /// Removes a legacy game setting.
    pub fn remove_setting(&mut self, name: &str) {
        self.legacy_settings.remove(name);
    }

    /// Clears all legacy settings.
    pub fn clear_settings(&mut self) {
        self.legacy_settings.clear();
    }

    /// Returns the legacy settings as a reference to the internal HashMap.
    #[must_use]
    pub fn settings(&self) -> &HashMap<String, bool> {
        &self.legacy_settings
    }

    /// Returns a reference to the randomizer settings.
    #[must_use]
    pub fn randomizer_settings(&self) -> &RandomizerSettings {
        &self.randomizer_settings
    }

    /// Returns a mutable reference to the randomizer settings.
    pub fn randomizer_settings_mut(&mut self) -> &mut RandomizerSettings {
        &mut self.randomizer_settings
    }

    /// Sets the randomizer settings.
    pub fn set_randomizer_settings(&mut self, settings: RandomizerSettings) {
        self.randomizer_settings = settings;
    }

    // --- Event methods ---

    /// Triggers a game event.
    pub fn add_event(&mut self, event: &str) {
        self.events.insert(event.to_string());
    }

    /// Removes a game event.
    pub fn remove_event(&mut self, event: &str) {
        self.events.remove(event);
    }

    /// Checks if an event has been triggered.
    #[must_use]
    pub fn has_event(&self, event: &str) -> bool {
        self.events.contains(event)
    }

    /// Clears all events.
    pub fn clear_events(&mut self) {
        self.events.clear();
    }

    /// Returns the set of triggered events.
    #[must_use]
    pub fn events(&self) -> &HashSet<String> {
        &self.events
    }

    // --- Time methods ---

    /// Sets the MM time.
    ///
    /// Time is represented as minutes since Day 1 at 6:00 AM.
    /// Valid range: 0-4319 (72 hours = 4320 minutes for the 3-day cycle).
    ///
    /// Values >= 4320 will be wrapped using modulo.
    pub fn set_mm_time(&mut self, time: u32) {
        self.mm_time = time % 4320;
    }

    /// Gets the current MM time.
    #[must_use]
    pub const fn get_mm_time(&self) -> u32 {
        self.mm_time
    }

    // --- Helper methods ---

    /// Normalizes an item name for case-insensitive lookup.
    ///
    /// Handles both PascalCase enum variants and snake_case/UPPER_CASE names.
    fn normalize_item_name(&self, name: &str) -> String {
        // If it contains parentheses, extract just the inner part
        // e.g., "Oot(Hookshot)" -> "HOOKSHOT"
        if let Some(start) = name.find('(') {
            if let Some(end) = name.find(')') {
                return name[start + 1..end].to_uppercase();
            }
        }
        name.to_uppercase()
    }

    /// Looks up an item by name using the Item enum.
    ///
    /// This validates that the item name corresponds to a real game item.
    #[must_use]
    pub fn lookup_item(name: &str) -> Option<Item> {
        Item::by_name(name)
    }
}

impl EvalContext for GameContext {
    fn has_item(&self, item: &str, count: u32) -> bool {
        let key = item.to_uppercase();
        self.inventory
            .get(&key)
            .map(|&c| c >= count)
            .unwrap_or(false)
    }

    fn event(&self, name: &str) -> bool {
        self.events.contains(name)
    }

    fn setting(&self, name: &str) -> Option<bool> {
        // First check randomizer settings
        if let Some(value) = self.randomizer_settings.get_bool_setting(name) {
            return Some(value);
        }
        // Fall back to legacy settings
        self.legacy_settings.get(name).copied()
    }

    fn setting_value(&self, name: &str, value: &str) -> bool {
        self.randomizer_settings.check_setting_value(name, value)
    }

    fn trick(&self, name: &str) -> bool {
        // Check both the context tricks and randomizer settings tricks
        self.tricks.contains(name) || self.randomizer_settings.has_trick(name)
    }

    fn is_adult(&self) -> bool {
        self.age.is_adult()
    }

    fn is_child(&self) -> bool {
        self.age.is_child()
    }

    fn mm_time(&self) -> u32 {
        self.mm_time
    }
}

/// Builder for [`GameContext`].
///
/// Provides a fluent API for constructing game contexts.
///
/// # Example
///
/// ```
/// use ootmm::expr::{GameContextBuilder, Age};
///
/// let ctx = GameContextBuilder::new()
///     .with_item("HOOKSHOT", 1)
///     .with_item("BOMBS", 20)
///     .with_age(Age::Adult)
///     .with_event("FOREST_TEMPLE_CLEAR")
///     .with_setting("shuffle_songs", true)
///     .with_trick("hover_boost")
///     .with_mm_time(360) // Noon on Day 1
///     .build();
/// ```
#[derive(Debug, Clone, Default)]
pub struct GameContextBuilder {
    ctx: GameContext,
}

impl GameContextBuilder {
    /// Creates a new builder with default values.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Adds an item to the inventory.
    #[must_use]
    pub fn with_item(mut self, item: &str, count: u32) -> Self {
        self.ctx.set_item(item, count);
        self
    }

    /// Adds an item using the Item enum.
    #[must_use]
    pub fn with_item_enum(mut self, item: Item, count: u32) -> Self {
        self.ctx.add_item(item, count);
        self
    }

    /// Sets the player age.
    #[must_use]
    pub fn with_age(mut self, age: Age) -> Self {
        self.ctx.set_age(age);
        self
    }

    /// Sets the player to Adult Link.
    #[must_use]
    pub fn as_adult(mut self) -> Self {
        self.ctx.set_age(Age::Adult);
        self
    }

    /// Sets the player to Child Link.
    #[must_use]
    pub fn as_child(mut self) -> Self {
        self.ctx.set_age(Age::Child);
        self
    }

    /// Adds an event.
    #[must_use]
    pub fn with_event(mut self, event: &str) -> Self {
        self.ctx.add_event(event);
        self
    }

    /// Adds a legacy boolean setting.
    #[must_use]
    pub fn with_setting(mut self, name: &str, value: bool) -> Self {
        self.ctx.set_setting(name, value);
        self
    }

    /// Sets the randomizer settings.
    #[must_use]
    pub fn with_randomizer_settings(mut self, settings: RandomizerSettings) -> Self {
        self.ctx.set_randomizer_settings(settings);
        self
    }

    /// Enables a trick.
    #[must_use]
    pub fn with_trick(mut self, trick: &str) -> Self {
        self.ctx.add_trick(trick);
        self
    }

    /// Sets the MM time.
    #[must_use]
    pub fn with_mm_time(mut self, time: u32) -> Self {
        self.ctx.set_mm_time(time);
        self
    }

    /// Builds the [`GameContext`].
    #[must_use]
    pub fn build(self) -> GameContext {
        self.ctx
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::expr::{eval_str, Evaluator};
    use crate::item::{MmItem, OotItem};

    // --- GameContext basic tests ---

    #[test]
    fn test_new_context() {
        let ctx = GameContext::new();
        assert!(ctx.inventory().is_empty());
        assert_eq!(ctx.age(), Age::Child);
        assert!(ctx.tricks().is_empty());
        assert!(ctx.settings().is_empty());
        assert!(ctx.events().is_empty());
        assert_eq!(ctx.get_mm_time(), 0);
    }

    #[test]
    fn test_default_context() {
        let ctx = GameContext::default();
        assert_eq!(ctx.age(), Age::Child);
        assert_eq!(ctx.get_mm_time(), 0);
    }

    // --- Inventory tests ---

    #[test]
    fn test_set_and_get_item() {
        let mut ctx = GameContext::new();
        ctx.set_item("HOOKSHOT", 1);
        assert_eq!(ctx.get_item("HOOKSHOT"), 1);
        assert_eq!(ctx.get_item("hookshot"), 1); // Case insensitive
        assert_eq!(ctx.get_item("Hookshot"), 1);
    }

    #[test]
    fn test_set_item_zero_removes() {
        let mut ctx = GameContext::new();
        ctx.set_item("HOOKSHOT", 1);
        assert_eq!(ctx.get_item("HOOKSHOT"), 1);
        ctx.set_item("HOOKSHOT", 0);
        assert_eq!(ctx.get_item("HOOKSHOT"), 0);
    }

    #[test]
    fn test_get_missing_item() {
        let ctx = GameContext::new();
        assert_eq!(ctx.get_item("NONEXISTENT"), 0);
    }

    #[test]
    fn test_add_item_enum() {
        let mut ctx = GameContext::new();
        ctx.add_item(Item::Oot(OotItem::Hookshot), 1);
        // The item should be accessible (stored as normalized name)
        assert!(ctx.get_item("HOOKSHOT") > 0);
    }

    #[test]
    fn test_remove_item() {
        let mut ctx = GameContext::new();
        ctx.set_item("HOOKSHOT", 1);
        ctx.remove_item("HOOKSHOT");
        assert_eq!(ctx.get_item("HOOKSHOT"), 0);
    }

    #[test]
    fn test_clear_inventory() {
        let mut ctx = GameContext::new();
        ctx.set_item("HOOKSHOT", 1);
        ctx.set_item("BOW", 1);
        ctx.clear_inventory();
        assert!(ctx.inventory().is_empty());
    }

    // --- Age tests ---

    #[test]
    fn test_age_enum() {
        assert!(Age::Adult.is_adult());
        assert!(!Age::Adult.is_child());
        assert!(Age::Child.is_child());
        assert!(!Age::Child.is_adult());
    }

    #[test]
    fn test_set_age() {
        let mut ctx = GameContext::new();
        assert_eq!(ctx.age(), Age::Child); // Default
        ctx.set_age(Age::Adult);
        assert_eq!(ctx.age(), Age::Adult);
    }

    // --- Trick tests ---

    #[test]
    fn test_tricks() {
        let mut ctx = GameContext::new();
        assert!(!ctx.has_trick("hover_boost"));
        ctx.add_trick("hover_boost");
        assert!(ctx.has_trick("hover_boost"));
        ctx.remove_trick("hover_boost");
        assert!(!ctx.has_trick("hover_boost"));
    }

    #[test]
    fn test_clear_tricks() {
        let mut ctx = GameContext::new();
        ctx.add_trick("trick1");
        ctx.add_trick("trick2");
        ctx.clear_tricks();
        assert!(ctx.tricks().is_empty());
    }

    // --- Setting tests ---

    #[test]
    fn test_settings() {
        let mut ctx = GameContext::new();
        assert_eq!(ctx.get_setting("shuffle_songs"), None);
        ctx.set_setting("shuffle_songs", true);
        assert_eq!(ctx.get_setting("shuffle_songs"), Some(true));
        ctx.set_setting("shuffle_songs", false);
        assert_eq!(ctx.get_setting("shuffle_songs"), Some(false));
    }

    #[test]
    fn test_remove_setting() {
        let mut ctx = GameContext::new();
        ctx.set_setting("option", true);
        ctx.remove_setting("option");
        assert_eq!(ctx.get_setting("option"), None);
    }

    #[test]
    fn test_clear_settings() {
        let mut ctx = GameContext::new();
        ctx.set_setting("opt1", true);
        ctx.set_setting("opt2", false);
        ctx.clear_settings();
        assert!(ctx.settings().is_empty());
    }

    // --- Event tests ---

    #[test]
    fn test_events() {
        let mut ctx = GameContext::new();
        assert!(!ctx.has_event("MIDO_MOVED"));
        ctx.add_event("MIDO_MOVED");
        assert!(ctx.has_event("MIDO_MOVED"));
        ctx.remove_event("MIDO_MOVED");
        assert!(!ctx.has_event("MIDO_MOVED"));
    }

    #[test]
    fn test_clear_events() {
        let mut ctx = GameContext::new();
        ctx.add_event("event1");
        ctx.add_event("event2");
        ctx.clear_events();
        assert!(ctx.events().is_empty());
    }

    // --- Time tests ---

    #[test]
    fn test_mm_time() {
        let mut ctx = GameContext::new();
        assert_eq!(ctx.get_mm_time(), 0);
        ctx.set_mm_time(360); // Noon
        assert_eq!(ctx.get_mm_time(), 360);
    }

    #[test]
    fn test_mm_time_wraps() {
        let mut ctx = GameContext::new();
        ctx.set_mm_time(4320); // Should wrap to 0
        assert_eq!(ctx.get_mm_time(), 0);
        ctx.set_mm_time(4500); // Should wrap to 180
        assert_eq!(ctx.get_mm_time(), 180);
    }

    // --- EvalContext implementation tests ---

    #[test]
    fn test_eval_context_has_item() {
        let mut ctx = GameContext::new();
        ctx.set_item("HOOKSHOT", 1);
        assert!(ctx.has_item("HOOKSHOT", 1));
        assert!(ctx.has_item("hookshot", 1)); // Case insensitive
        assert!(!ctx.has_item("HOOKSHOT", 2));
        assert!(!ctx.has_item("BOW", 1));
    }

    #[test]
    fn test_eval_context_event() {
        let mut ctx = GameContext::new();
        ctx.add_event("TEST_EVENT");
        assert!(ctx.event("TEST_EVENT"));
        assert!(!ctx.event("OTHER_EVENT"));
    }

    #[test]
    fn test_eval_context_setting() {
        let mut ctx = GameContext::new();
        ctx.set_setting("option", true);
        assert_eq!(ctx.setting("option"), Some(true));
        assert_eq!(ctx.setting("missing"), None);
    }

    #[test]
    fn test_eval_context_trick() {
        let mut ctx = GameContext::new();
        ctx.add_trick("hover_boost");
        assert!(ctx.trick("hover_boost"));
        assert!(!ctx.trick("other_trick"));
    }

    #[test]
    fn test_eval_context_age() {
        let mut ctx = GameContext::new();
        assert!(ctx.is_child());
        assert!(!ctx.is_adult());

        ctx.set_age(Age::Adult);
        assert!(ctx.is_adult());
        assert!(!ctx.is_child());
    }

    #[test]
    fn test_eval_context_mm_time() {
        let mut ctx = GameContext::new();
        ctx.set_mm_time(720); // Night time
        assert_eq!(EvalContext::mm_time(&ctx), 720);
    }

    #[test]
    fn test_eval_context_is_day() {
        let mut ctx = GameContext::new();
        ctx.set_mm_time(0); // 6 AM = day
        assert!(ctx.is_day());
        assert!(!ctx.is_night());

        ctx.set_mm_time(360); // Noon = day
        assert!(ctx.is_day());

        ctx.set_mm_time(719); // Just before 6 PM = day
        assert!(ctx.is_day());

        ctx.set_mm_time(720); // 6 PM = night
        assert!(!ctx.is_day());
        assert!(ctx.is_night());

        ctx.set_mm_time(1080); // Midnight = night
        assert!(ctx.is_night());
    }

    // --- Builder tests ---

    #[test]
    fn test_builder_basic() {
        let ctx = GameContextBuilder::new()
            .with_item("HOOKSHOT", 1)
            .with_age(Age::Adult)
            .build();

        assert_eq!(ctx.get_item("HOOKSHOT"), 1);
        assert_eq!(ctx.age(), Age::Adult);
    }

    #[test]
    fn test_builder_full() {
        let ctx = GameContextBuilder::new()
            .with_item("HOOKSHOT", 1)
            .with_item("BOMBS", 20)
            .as_adult()
            .with_event("BOSS_DEFEATED")
            .with_setting("shuffle_songs", true)
            .with_trick("hover_boost")
            .with_mm_time(360)
            .build();

        assert_eq!(ctx.get_item("HOOKSHOT"), 1);
        assert_eq!(ctx.get_item("BOMBS"), 20);
        assert!(ctx.is_adult());
        assert!(ctx.has_event("BOSS_DEFEATED"));
        assert_eq!(ctx.get_setting("shuffle_songs"), Some(true));
        assert!(ctx.has_trick("hover_boost"));
        assert_eq!(ctx.get_mm_time(), 360);
    }

    #[test]
    fn test_builder_as_child() {
        let ctx = GameContextBuilder::new().as_child().build();
        assert!(ctx.is_child());
    }

    #[test]
    fn test_builder_with_item_enum() {
        let ctx = GameContextBuilder::new()
            .with_item_enum(Item::Oot(OotItem::MasterSword), 1)
            .build();
        // Should have the item stored
        assert!(ctx.get_item("MASTERSWORD") > 0);
    }

    // --- Integration tests with evaluator ---

    #[test]
    fn test_evaluator_has_item() {
        let ctx = GameContextBuilder::new()
            .with_item("HOOKSHOT", 1)
            .as_adult()
            .build();

        assert!(eval_str("has(HOOKSHOT)", &ctx).unwrap());
        assert!(!eval_str("has(BOW)", &ctx).unwrap());
    }

    #[test]
    fn test_evaluator_can_use_adult_item() {
        let ctx_adult = GameContextBuilder::new()
            .with_item("HOOKSHOT", 1)
            .as_adult()
            .build();

        let ctx_child = GameContextBuilder::new()
            .with_item("HOOKSHOT", 1)
            .as_child()
            .build();

        assert!(eval_str("can_use(HOOKSHOT)", &ctx_adult).unwrap());
        assert!(!eval_str("can_use(HOOKSHOT)", &ctx_child).unwrap());
    }

    #[test]
    fn test_evaluator_can_use_child_item() {
        let ctx_adult = GameContextBuilder::new()
            .with_item("BOOMERANG", 1)
            .as_adult()
            .build();

        let ctx_child = GameContextBuilder::new()
            .with_item("BOOMERANG", 1)
            .as_child()
            .build();

        assert!(!eval_str("can_use(BOOMERANG)", &ctx_adult).unwrap());
        assert!(eval_str("can_use(BOOMERANG)", &ctx_child).unwrap());
    }

    #[test]
    fn test_evaluator_event() {
        let ctx = GameContextBuilder::new()
            .with_event("FOREST_TEMPLE_CLEAR")
            .build();

        assert!(eval_str("event(FOREST_TEMPLE_CLEAR)", &ctx).unwrap());
        assert!(!eval_str("event(FIRE_TEMPLE_CLEAR)", &ctx).unwrap());
    }

    #[test]
    fn test_evaluator_setting() {
        let ctx = GameContextBuilder::new()
            .with_setting("shuffle_songs", true)
            .with_setting("skip_child_zelda", false)
            .build();

        assert!(eval_str("setting(shuffle_songs)", &ctx).unwrap());
        assert!(!eval_str("setting(skip_child_zelda)", &ctx).unwrap());
        assert!(!eval_str("setting(missing_setting)", &ctx).unwrap());
    }

    #[test]
    fn test_evaluator_trick() {
        let ctx = GameContextBuilder::new().with_trick("hover_boost").build();

        assert!(eval_str("trick(hover_boost)", &ctx).unwrap());
        assert!(!eval_str("trick(other_trick)", &ctx).unwrap());
    }

    #[test]
    fn test_evaluator_complex_expression() {
        let ctx = GameContextBuilder::new()
            .with_item("HOOKSHOT", 1)
            .with_item("BOW", 1)
            .as_adult()
            .with_event("FOREST_TEMPLE_CLEAR")
            .build();

        assert!(eval_str(
            "is_adult && has(HOOKSHOT) && has(BOW) && event(FOREST_TEMPLE_CLEAR)",
            &ctx
        )
        .unwrap());

        assert!(eval_str("can_use(HOOKSHOT) && can_use(BOW)", &ctx).unwrap());

        assert!(!eval_str("is_child && has(HOOKSHOT)", &ctx).unwrap());
    }

    #[test]
    fn test_evaluator_with_evaluator_struct() {
        let ctx = GameContextBuilder::new()
            .with_item("BOMBS", 20)
            .as_child()
            .build();

        let evaluator = Evaluator::new(&ctx);

        assert!(evaluator.eval_str("has(BOMBS)").unwrap());
        assert!(evaluator.eval_str("has(BOMBS, 10)").unwrap());
        assert!(evaluator.eval_str("has(BOMBS, 20)").unwrap());
        assert!(!evaluator.eval_str("has(BOMBS, 21)").unwrap());
        assert!(evaluator.eval_str("is_child").unwrap());
        assert!(!evaluator.eval_str("is_adult").unwrap());
    }

    // --- Item lookup tests ---

    #[test]
    fn test_lookup_item() {
        assert!(GameContext::lookup_item("MasterSword").is_some());
        assert!(GameContext::lookup_item("master_sword").is_some());
        assert!(GameContext::lookup_item("DekuMask").is_some());
        assert!(GameContext::lookup_item("NotAnItem").is_none());
    }

    #[test]
    fn test_lookup_item_oot() {
        let item = GameContext::lookup_item("Hookshot").unwrap();
        assert!(matches!(item, Item::Oot(OotItem::Hookshot)));
    }

    #[test]
    fn test_lookup_item_mm() {
        let item = GameContext::lookup_item("DekuMask").unwrap();
        assert!(matches!(item, Item::Mm(MmItem::DekuMask)));
    }

    // --- Clone and Debug tests ---

    #[test]
    fn test_context_clone() {
        let ctx1 = GameContextBuilder::new()
            .with_item("HOOKSHOT", 1)
            .as_adult()
            .build();

        let ctx2 = ctx1.clone();
        assert_eq!(ctx2.get_item("HOOKSHOT"), 1);
        assert!(ctx2.is_adult());
    }
}
