//! Region reachability analysis for determining accessible locations.

use crate::region::Game;

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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::embedded_data::create_world_database;

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
}
