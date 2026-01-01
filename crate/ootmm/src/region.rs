//! Region and location types for world data.
//! TODO: Implement (Issue #25)

/// A game region/area.
#[derive(Debug, Clone)]
pub struct Region {
    pub name: String,
    pub game: Game,
    // TODO: Add locations, exits, events
}

/// Which game a region belongs to.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Game {
    Oot,
    Mm,
}
