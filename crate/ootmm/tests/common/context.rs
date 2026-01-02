//! Mock implementations of EvalContext for testing.

use ootmm::expr::EvalContext;
use std::collections::{HashMap, HashSet};

/// A mock implementation of `EvalContext` for testing expression evaluation.
///
/// This allows tests to configure exactly which items, events, settings, and tricks
/// are available, making it easy to test specific evaluation scenarios.
///
/// # Example
///
/// ```ignore
/// use common::MockEvalContext;
///
/// let mut ctx = MockEvalContext::new();
/// ctx.add_item("HOOKSHOT", 1);
/// ctx.set_adult(true);
///
/// // Now expressions like "has(HOOKSHOT) && is_adult" will evaluate to true
/// ```
#[derive(Debug, Default)]
pub struct MockEvalContext {
    /// Items the player has, with their counts.
    items: HashMap<String, u32>,
    /// Events that have been triggered.
    events: HashSet<String>,
    /// Settings and their boolean values.
    settings: HashMap<String, bool>,
    /// Enabled tricks.
    tricks: HashSet<String>,
    /// Whether the player is an adult.
    is_adult: bool,
    /// Whether the player is a child.
    is_child: bool,
}

impl MockEvalContext {
    /// Creates a new empty mock context.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Creates a context configured as adult Link.
    #[must_use]
    pub fn adult() -> Self {
        Self {
            is_adult: true,
            is_child: false,
            ..Self::default()
        }
    }

    /// Creates a context configured as child Link.
    #[must_use]
    pub fn child() -> Self {
        Self {
            is_adult: false,
            is_child: true,
            ..Self::default()
        }
    }

    /// Adds an item with the given count.
    pub fn add_item(&mut self, name: impl Into<String>, count: u32) -> &mut Self {
        self.items.insert(name.into(), count);
        self
    }

    /// Adds an item with count of 1.
    pub fn with_item(mut self, name: impl Into<String>) -> Self {
        self.add_item(name, 1);
        self
    }

    /// Adds multiple items, each with count 1.
    pub fn with_items<I, S>(mut self, items: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        for item in items {
            self.add_item(item, 1);
        }
        self
    }

    /// Sets an event as triggered.
    pub fn add_event(&mut self, name: impl Into<String>) -> &mut Self {
        self.events.insert(name.into());
        self
    }

    /// Sets an event as triggered (builder pattern).
    pub fn with_event(mut self, name: impl Into<String>) -> Self {
        self.add_event(name);
        self
    }

    /// Sets a setting value.
    pub fn add_setting(&mut self, name: impl Into<String>, value: bool) -> &mut Self {
        self.settings.insert(name.into(), value);
        self
    }

    /// Sets a setting to true (builder pattern).
    pub fn with_setting(mut self, name: impl Into<String>) -> Self {
        self.add_setting(name, true);
        self
    }

    /// Enables a trick.
    pub fn add_trick(&mut self, name: impl Into<String>) -> &mut Self {
        self.tricks.insert(name.into());
        self
    }

    /// Enables a trick (builder pattern).
    pub fn with_trick(mut self, name: impl Into<String>) -> Self {
        self.add_trick(name);
        self
    }

    /// Sets whether the context represents adult Link.
    pub fn set_adult(&mut self, value: bool) -> &mut Self {
        self.is_adult = value;
        if value {
            self.is_child = false;
        }
        self
    }

    /// Sets whether the context represents child Link.
    pub fn set_child(&mut self, value: bool) -> &mut Self {
        self.is_child = value;
        if value {
            self.is_adult = false;
        }
        self
    }
}

impl EvalContext for MockEvalContext {
    fn has_item(&self, item: &str, count: u32) -> bool {
        self.items.get(item).copied().unwrap_or(0) >= count
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
        self.is_child
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_context_is_empty() {
        let ctx = MockEvalContext::new();
        assert!(!ctx.has_item("HOOKSHOT", 1));
        assert!(!ctx.event("MIDO_MOVED"));
        assert!(ctx.setting("skip_child_zelda").is_none());
        assert!(!ctx.trick("logic_grottos_without_agony"));
        assert!(!ctx.is_adult());
        assert!(!ctx.is_child());
    }

    #[test]
    fn test_adult_constructor() {
        let ctx = MockEvalContext::adult();
        assert!(ctx.is_adult());
        assert!(!ctx.is_child());
    }

    #[test]
    fn test_child_constructor() {
        let ctx = MockEvalContext::child();
        assert!(!ctx.is_adult());
        assert!(ctx.is_child());
    }

    #[test]
    fn test_builder_pattern() {
        let ctx = MockEvalContext::adult()
            .with_item("HOOKSHOT")
            .with_item("BOW")
            .with_event("MIDO_MOVED")
            .with_setting("shuffle_scrubs")
            .with_trick("logic_lens_botw");

        assert!(ctx.has_item("HOOKSHOT", 1));
        assert!(ctx.has_item("BOW", 1));
        assert!(!ctx.has_item("BOMBS", 1));
        assert!(ctx.event("MIDO_MOVED"));
        assert!(ctx.setting("shuffle_scrubs") == Some(true));
        assert!(ctx.trick("logic_lens_botw"));
    }

    #[test]
    fn test_item_counts() {
        let mut ctx = MockEvalContext::new();
        ctx.add_item("BOTTLE", 3);

        assert!(ctx.has_item("BOTTLE", 1));
        assert!(ctx.has_item("BOTTLE", 2));
        assert!(ctx.has_item("BOTTLE", 3));
        assert!(!ctx.has_item("BOTTLE", 4));
    }

    #[test]
    fn test_with_items() {
        let ctx = MockEvalContext::new().with_items(["SWORD", "SHIELD", "BOW"]);

        assert!(ctx.has_item("SWORD", 1));
        assert!(ctx.has_item("SHIELD", 1));
        assert!(ctx.has_item("BOW", 1));
    }

    #[test]
    fn test_set_adult_clears_child() {
        let mut ctx = MockEvalContext::child();
        assert!(ctx.is_child());
        assert!(!ctx.is_adult());

        ctx.set_adult(true);
        assert!(ctx.is_adult());
        assert!(!ctx.is_child());
    }

    #[test]
    fn test_set_child_clears_adult() {
        let mut ctx = MockEvalContext::adult();
        assert!(ctx.is_adult());
        assert!(!ctx.is_child());

        ctx.set_child(true);
        assert!(ctx.is_child());
        assert!(!ctx.is_adult());
    }
}
