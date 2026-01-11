//! Scene flag structures for permanent and cycle-based flags.

/// Permanent scene flags for a single scene
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct MmPermanentSceneFlags {
    pub chest: u32,
    pub switch0: u32,
    pub switch1: u32,
    pub cleared_room: u32,
    pub collectible: u32,
    pub cleared_floors: u32,
    pub rooms: u32,
}

/// Cycle-based scene flags (reset on Song of Time)
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct MmCycleSceneFlags {
    pub chest: u32,
    pub switch0: u32,
    pub switch1: u32,
    pub cleared_room: u32,
    pub collectible: u32,
}
