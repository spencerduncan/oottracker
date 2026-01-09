//! Region reachability analysis for determining accessible locations.

use std::collections::{HashSet, VecDeque};

use crate::expr::{eval_str, EvalContext};
use crate::region::Game;
use crate::world_database::WorldDatabase;

/// OoT spawn region - Link's House in Kokiri Forest
pub const OOT_SPAWN: &str = "oot_links_house";

/// MM spawn region - Clock Tower
pub const MM_SPAWN: &str = "mm_clock_tower";

/// Returns the spawn region ID for the given game.
#[must_use]
pub fn spawn_region(game: Game) -> &'static str {
    match game {
        Game::Oot => OOT_SPAWN,
        Game::Mm => MM_SPAWN,
    }
}

/// A set of reachable region IDs from spawn given the player's current items/state.
///
/// This struct holds the result of a reachability analysis and provides methods
/// to check if specific regions are accessible.
#[derive(Debug, Clone, Default)]
pub struct ReachableRegions {
    /// The set of region IDs that are reachable.
    regions: HashSet<String>,
}

impl ReachableRegions {
    /// Creates a new empty `ReachableRegions`.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Creates a `ReachableRegions` from an existing set.
    #[must_use]
    pub fn from_set(regions: HashSet<String>) -> Self {
        Self { regions }
    }

    /// Checks if a region is reachable.
    #[must_use]
    pub fn contains(&self, region_id: &str) -> bool {
        self.regions.contains(region_id)
    }

    /// Returns the number of reachable regions.
    #[must_use]
    pub fn len(&self) -> usize {
        self.regions.len()
    }

    /// Returns true if no regions are reachable.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.regions.is_empty()
    }

    /// Returns an iterator over the reachable region IDs.
    pub fn iter(&self) -> impl Iterator<Item = &String> {
        self.regions.iter()
    }

    /// Returns a reference to the underlying set.
    #[must_use]
    pub fn as_set(&self) -> &HashSet<String> {
        &self.regions
    }
}

/// Computes the set of reachable regions from the spawn point using BFS.
///
/// Starting from the spawn region for the given game, this function performs
/// a breadth-first search through the world graph. For each region, it checks
/// all exits and evaluates their logic expressions against the provided context.
/// If an exit's logic passes (or is `None`), the target region is added to the
/// frontier.
///
/// # Arguments
///
/// * `db` - The world database containing all regions and exits
/// * `ctx` - The evaluation context providing item/event state for logic checks
/// * `game` - Which game (OoT or MM) to compute reachability for
///
/// # Returns
///
/// A `ReachableRegions` containing all region IDs accessible from spawn.
///
/// # Example
///
/// ```ignore
/// use ootmm::reachability::compute_reachable_regions;
/// use ootmm::expr::GameContext;
/// use ootmm::region::Game;
///
/// let db = /* load world database */;
/// let ctx = GameContext::new();
/// let reachable = compute_reachable_regions(&db, &ctx, Game::Oot);
///
/// if reachable.contains("kokiri_forest") {
///     println!("Kokiri Forest is reachable!");
/// }
/// ```
#[must_use]
pub fn compute_reachable_regions<C: EvalContext>(
    db: &WorldDatabase,
    ctx: &C,
    game: Game,
) -> ReachableRegions {
    let mut visited: HashSet<String> = HashSet::new();
    let mut frontier: VecDeque<String> = VecDeque::new();

    // Start from the spawn region
    let spawn = spawn_region(game);
    if db.get_region(spawn).is_some() {
        frontier.push_back(spawn.to_string());
        visited.insert(spawn.to_string());
    }

    // BFS through reachable regions
    while let Some(current_id) = frontier.pop_front() {
        let Some(region) = db.get_region(&current_id) else {
            continue;
        };

        // Check each exit from this region
        for exit in &region.exits {
            // Skip if already visited
            if visited.contains(&exit.target) {
                continue;
            }

            // Check if the exit's target region exists
            if db.get_region(&exit.target).is_none() {
                continue;
            }

            // Evaluate exit logic
            let can_pass = match &exit.logic {
                None => true, // No logic means always accessible
                Some(logic) => {
                    // Evaluate the logic expression
                    eval_str(logic, ctx).unwrap_or(false)
                }
            };

            if can_pass {
                visited.insert(exit.target.clone());
                frontier.push_back(exit.target.clone());
            }
        }
    }

    ReachableRegions::from_set(visited)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::embedded_data::create_world_database;
    use crate::expr::GameContext;
    use crate::region::{Exit, ExitType, Region};

    #[test]
    fn test_oot_spawn_exists() {
        let db = create_world_database().expect("Failed to load world database");
        assert!(
            db.get_region(OOT_SPAWN).is_some(),
            "OoT spawn region '{}' not found in world database",
            OOT_SPAWN
        );
    }

    #[test]
    fn test_mm_spawn_exists() {
        let db = create_world_database().expect("Failed to load world database");
        assert!(
            db.get_region(MM_SPAWN).is_some(),
            "MM spawn region '{}' not found in world database",
            MM_SPAWN
        );
    }

    #[test]
    fn test_spawn_region_helper() {
        assert_eq!(spawn_region(Game::Oot), OOT_SPAWN);
        assert_eq!(spawn_region(Game::Mm), MM_SPAWN);
    }

    // ===== ReachableRegions struct tests =====

    #[test]
    fn test_reachable_regions_new() {
        let reachable = ReachableRegions::new();
        assert!(reachable.is_empty());
        assert_eq!(reachable.len(), 0);
    }

    #[test]
    fn test_reachable_regions_from_set() {
        let mut set = HashSet::new();
        set.insert("region_a".to_string());
        set.insert("region_b".to_string());

        let reachable = ReachableRegions::from_set(set);
        assert_eq!(reachable.len(), 2);
        assert!(reachable.contains("region_a"));
        assert!(reachable.contains("region_b"));
        assert!(!reachable.contains("region_c"));
    }

    #[test]
    fn test_reachable_regions_iter() {
        let mut set = HashSet::new();
        set.insert("a".to_string());
        set.insert("b".to_string());

        let reachable = ReachableRegions::from_set(set);
        let collected: HashSet<_> = reachable.iter().cloned().collect();
        assert!(collected.contains("a"));
        assert!(collected.contains("b"));
    }

    #[test]
    fn test_reachable_regions_as_set() {
        let mut set = HashSet::new();
        set.insert("x".to_string());
        let reachable = ReachableRegions::from_set(set);
        assert!(reachable.as_set().contains("x"));
    }

    // ===== compute_reachable_regions tests =====

    /// Mock context with no items for testing limited reachability.
    struct EmptyContext;

    impl EvalContext for EmptyContext {
        fn has_item(&self, _item: &str, _count: u32) -> bool {
            false
        }
        fn event(&self, _name: &str) -> bool {
            false
        }
        fn setting(&self, _name: &str) -> Option<bool> {
            None
        }
        fn trick(&self, _name: &str) -> bool {
            false
        }
        fn is_adult(&self) -> bool {
            false
        }
        fn is_child(&self) -> bool {
            true
        }
        fn mm_time(&self) -> u32 {
            0
        }
    }

    #[test]
    fn test_spawn_region_always_reachable_oot() {
        let db = create_world_database().expect("Failed to load world database");
        let ctx = EmptyContext;

        let reachable = compute_reachable_regions(&db, &ctx, Game::Oot);

        // Spawn region should always be reachable
        assert!(
            reachable.contains(OOT_SPAWN),
            "OoT spawn region should always be reachable"
        );
        assert!(!reachable.is_empty());
    }

    #[test]
    fn test_spawn_region_always_reachable_mm() {
        let db = create_world_database().expect("Failed to load world database");
        let ctx = EmptyContext;

        let reachable = compute_reachable_regions(&db, &ctx, Game::Mm);

        // Spawn region should always be reachable
        assert!(
            reachable.contains(MM_SPAWN),
            "MM spawn region should always be reachable"
        );
        assert!(!reachable.is_empty());
    }

    #[test]
    fn test_limited_reachability_with_no_items() {
        // Create a simple test database with controlled regions
        let mut db = WorldDatabase::new();

        // Spawn region with exits
        let mut spawn = Region::new(OOT_SPAWN, "Link's House", Game::Oot);
        spawn.add_exit(Exit::new("adjacent_region", ExitType::Normal));
        spawn.add_exit(Exit::new("locked_region", ExitType::Normal).with_logic("has(KEY)"));
        db.add_region(spawn).unwrap();

        // Adjacent region (no logic required)
        let adjacent = Region::new("adjacent_region", "Adjacent", Game::Oot);
        db.add_region(adjacent).unwrap();

        // Locked region (requires KEY item)
        let locked = Region::new("locked_region", "Locked", Game::Oot);
        db.add_region(locked).unwrap();

        // Test with empty context (no items)
        let ctx = EmptyContext;
        let reachable = compute_reachable_regions(&db, &ctx, Game::Oot);

        // Should reach spawn and adjacent, but not locked
        assert!(reachable.contains(OOT_SPAWN));
        assert!(reachable.contains("adjacent_region"));
        assert!(
            !reachable.contains("locked_region"),
            "Locked region should not be reachable without KEY"
        );
        assert_eq!(reachable.len(), 2);
    }

    #[test]
    fn test_reachability_with_items() {
        // Create a simple test database
        let mut db = WorldDatabase::new();

        let mut spawn = Region::new(OOT_SPAWN, "Link's House", Game::Oot);
        spawn.add_exit(Exit::new("locked_region", ExitType::Normal).with_logic("has(HOOKSHOT)"));
        db.add_region(spawn).unwrap();

        let locked = Region::new("locked_region", "Locked", Game::Oot);
        db.add_region(locked).unwrap();

        // Test with GameContext that has the required item
        let mut ctx = GameContext::new();
        ctx.set_item("HOOKSHOT", 1);

        let reachable = compute_reachable_regions(&db, &ctx, Game::Oot);

        assert!(reachable.contains(OOT_SPAWN));
        assert!(
            reachable.contains("locked_region"),
            "Locked region should be reachable with HOOKSHOT"
        );
        assert_eq!(reachable.len(), 2);
    }

    #[test]
    fn test_reachability_chain() {
        // Test that BFS properly traverses a chain of regions
        let mut db = WorldDatabase::new();

        let mut region_a = Region::new(OOT_SPAWN, "A", Game::Oot);
        region_a.add_exit(Exit::new("region_b", ExitType::Normal));
        db.add_region(region_a).unwrap();

        let mut region_b = Region::new("region_b", "B", Game::Oot);
        region_b.add_exit(Exit::new("region_c", ExitType::Normal));
        db.add_region(region_b).unwrap();

        let mut region_c = Region::new("region_c", "C", Game::Oot);
        region_c.add_exit(Exit::new("region_d", ExitType::Normal));
        db.add_region(region_c).unwrap();

        let region_d = Region::new("region_d", "D", Game::Oot);
        db.add_region(region_d).unwrap();

        let ctx = EmptyContext;
        let reachable = compute_reachable_regions(&db, &ctx, Game::Oot);

        assert!(reachable.contains(OOT_SPAWN));
        assert!(reachable.contains("region_b"));
        assert!(reachable.contains("region_c"));
        assert!(reachable.contains("region_d"));
        assert_eq!(reachable.len(), 4);
    }

    #[test]
    fn test_reachability_with_cycles() {
        // Test that BFS handles cycles correctly (doesn't infinite loop)
        let mut db = WorldDatabase::new();

        let mut region_a = Region::new(OOT_SPAWN, "A", Game::Oot);
        region_a.add_exit(Exit::new("region_b", ExitType::Normal));
        db.add_region(region_a).unwrap();

        let mut region_b = Region::new("region_b", "B", Game::Oot);
        region_b.add_exit(Exit::new(OOT_SPAWN, ExitType::Normal)); // Back to spawn
        region_b.add_exit(Exit::new("region_c", ExitType::Normal));
        db.add_region(region_b).unwrap();

        let mut region_c = Region::new("region_c", "C", Game::Oot);
        region_c.add_exit(Exit::new("region_b", ExitType::Normal)); // Back to B
        db.add_region(region_c).unwrap();

        let ctx = EmptyContext;
        let reachable = compute_reachable_regions(&db, &ctx, Game::Oot);

        assert_eq!(reachable.len(), 3);
        assert!(reachable.contains(OOT_SPAWN));
        assert!(reachable.contains("region_b"));
        assert!(reachable.contains("region_c"));
    }

    #[test]
    fn test_reachability_missing_spawn() {
        // Test behavior when spawn region doesn't exist
        let db = WorldDatabase::new(); // Empty database
        let ctx = EmptyContext;

        let reachable = compute_reachable_regions(&db, &ctx, Game::Oot);

        // Should return empty set if spawn doesn't exist
        assert!(reachable.is_empty());
    }

    #[test]
    fn test_reachability_invalid_exit_target() {
        // Test that exits pointing to non-existent regions are handled
        let mut db = WorldDatabase::new();

        let mut spawn = Region::new(OOT_SPAWN, "Spawn", Game::Oot);
        spawn.add_exit(Exit::new("nonexistent_region", ExitType::Normal));
        spawn.add_exit(Exit::new("valid_region", ExitType::Normal));
        db.add_region(spawn).unwrap();

        let valid = Region::new("valid_region", "Valid", Game::Oot);
        db.add_region(valid).unwrap();

        let ctx = EmptyContext;
        let reachable = compute_reachable_regions(&db, &ctx, Game::Oot);

        // Should reach spawn and valid, but not crash on nonexistent
        assert!(reachable.contains(OOT_SPAWN));
        assert!(reachable.contains("valid_region"));
        assert!(!reachable.contains("nonexistent_region"));
        assert_eq!(reachable.len(), 2);
    }

    #[test]
    fn test_reachability_logic_error_handled() {
        // Test that logic evaluation errors are handled gracefully
        let mut db = WorldDatabase::new();

        let mut spawn = Region::new(OOT_SPAWN, "Spawn", Game::Oot);
        spawn.add_exit(Exit::new("error_region", ExitType::Normal).with_logic("invalid((syntax"));
        spawn.add_exit(Exit::new("valid_region", ExitType::Normal).with_logic("true"));
        db.add_region(spawn).unwrap();

        let error = Region::new("error_region", "Error", Game::Oot);
        db.add_region(error).unwrap();

        let valid = Region::new("valid_region", "Valid", Game::Oot);
        db.add_region(valid).unwrap();

        let ctx = EmptyContext;
        let reachable = compute_reachable_regions(&db, &ctx, Game::Oot);

        // Should still work, treating invalid logic as unreachable
        assert!(reachable.contains(OOT_SPAWN));
        assert!(reachable.contains("valid_region"));
        assert!(
            !reachable.contains("error_region"),
            "Region with invalid logic should not be reachable"
        );
    }

    #[test]
    fn test_reachability_with_real_database() {
        // Integration test with the actual world database
        let db = create_world_database().expect("Failed to load world database");
        let ctx = GameContext::new();

        // Test OoT reachability
        let oot_reachable = compute_reachable_regions(&db, &ctx, Game::Oot);
        assert!(oot_reachable.contains(OOT_SPAWN));
        assert!(
            oot_reachable.len() >= 1,
            "Should reach at least spawn region"
        );

        // Test MM reachability
        let mm_reachable = compute_reachable_regions(&db, &ctx, Game::Mm);
        assert!(mm_reachable.contains(MM_SPAWN));
        assert!(mm_reachable.len() >= 1, "Should reach at least spawn region");
    }

    #[test]
    fn test_reachability_age_dependent_exits() {
        // Test exits that depend on age
        let mut db = WorldDatabase::new();

        let mut spawn = Region::new(OOT_SPAWN, "Spawn", Game::Oot);
        spawn.add_exit(Exit::new("adult_region", ExitType::Normal).with_logic("is_adult"));
        spawn.add_exit(Exit::new("child_region", ExitType::Normal).with_logic("is_child"));
        db.add_region(spawn).unwrap();

        let adult = Region::new("adult_region", "Adult Only", Game::Oot);
        db.add_region(adult).unwrap();

        let child = Region::new("child_region", "Child Only", Game::Oot);
        db.add_region(child).unwrap();

        // Test as child (EmptyContext.is_child() returns true)
        let ctx = EmptyContext;
        let reachable = compute_reachable_regions(&db, &ctx, Game::Oot);

        assert!(reachable.contains(OOT_SPAWN));
        assert!(reachable.contains("child_region"));
        assert!(!reachable.contains("adult_region"));
    }
}
