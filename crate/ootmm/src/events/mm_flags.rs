//! MM flag data structures and stub mappings.
//!
//! This module provides data structures for mapping MM in-game locations (chests,
//! collectibles, stray fairies, etc.) to their memory flag locations in save data.
//!
//! # Memory Layout
//!
//! MM save data base address: 0x1EF670
//!
//! Scene flags work similarly to OoT but with different offsets. Each scene has
//! flags for chests, switches, room clears, and collectibles.
//!
//! # Flag Types
//!
//! - **Chest flags**: Track which chests have been opened
//! - **Collectible flags**: Track collected items (skulltulas, heart pieces, etc.)
//! - **Special flags**: Track special collectibles like stray fairies

/// MM save data offsets for scene flags.
pub mod offsets {
    /// MM save data base address in RAM.
    pub const MM_SAVE_BASE: usize = 0x1EF670;

    /// Scene flags offset in save data.
    pub const SCENE_FLAGS: usize = 0x00;

    /// Size of each scene's flags (0x14 = 20 bytes).
    pub const SCENE_SIZE: usize = 0x14;

    /// Number of scenes in MM.
    pub const NUM_SCENES: usize = 0x78;

    /// Stray fairy counts offset in save data.
    pub const STRAY_FAIRY_COUNTS: usize = 0x0E94;

    /// Number of stray fairy count entries.
    pub const STRAY_FAIRY_COUNT_SIZE: usize = 10;
}

/// Type of flag in MM save data.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MmFlagType {
    /// Chest flag (offset 0x00 in scene flags).
    Chest,
    /// Switch flag (offset 0x04 in scene flags).
    Switch,
    /// Room clear flag (offset 0x08 in scene flags).
    RoomClear,
    /// Collectible flag (offset 0x0C in scene flags).
    Collectible,
    /// Special flags (offset 0x10 in scene flags).
    Special,
}

impl MmFlagType {
    /// Get the byte offset within a scene's flag block for this flag type.
    #[must_use]
    pub const fn offset(self) -> usize {
        match self {
            Self::Chest => 0x00,
            Self::Switch => 0x04,
            Self::RoomClear => 0x08,
            Self::Collectible => 0x0C,
            Self::Special => 0x10,
        }
    }
}

/// Memory flag location for MM scene-based flags.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MmSceneFlag {
    /// Scene ID (0x00-0x77).
    pub scene_id: u8,
    /// Type of flag (chest, collectible, etc.).
    pub flag_type: MmFlagType,
    /// Bit mask for the flag within the 4-byte flag word.
    pub bit_mask: u32,
}

impl MmSceneFlag {
    /// Create a new scene flag reference.
    #[must_use]
    pub const fn new(scene_id: u8, flag_type: MmFlagType, bit_mask: u32) -> Self {
        Self {
            scene_id,
            flag_type,
            bit_mask,
        }
    }

    /// Calculate the absolute offset in save data for this flag.
    #[must_use]
    pub const fn save_offset(&self) -> usize {
        offsets::SCENE_FLAGS
            + (self.scene_id as usize) * offsets::SCENE_SIZE
            + self.flag_type.offset()
    }
}

/// Mapping from a location name to its memory flag location.
///
/// This struct is used to map randomizer check locations to their corresponding
/// memory flags in save data, allowing the tracker to read game state.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MmFlagMapping {
    /// The name of the location as used in randomizer logic.
    pub location_name: String,
    /// The scene flag that tracks this location.
    pub flag: MmSceneFlag,
}

impl MmFlagMapping {
    /// Create a new flag mapping.
    #[must_use]
    pub fn new(location_name: impl Into<String>, flag: MmSceneFlag) -> Self {
        Self {
            location_name: location_name.into(),
            flag,
        }
    }
}

/// Get MM chest flag mappings.
///
/// Returns a vector of mappings from chest location names to their scene flags.
///
/// # Note
///
/// This is a stub function that returns an empty vector. Actual chest mappings
/// will be populated in a future issue.
#[must_use]
pub fn mm_chest_mappings() -> Vec<MmFlagMapping> {
    // TODO: Populate with actual chest mappings
    // Example format:
    // MmFlagMapping::new(
    //     "MM Clock Town Chest",
    //     MmSceneFlag::new(0x6C, MmFlagType::Chest, 0x0000_0001),
    // )
    Vec::new()
}

/// Get MM collectible flag mappings.
///
/// Returns a vector of mappings from collectible location names to their scene flags.
/// This includes items like skull tokens, heart pieces, etc.
///
/// # Note
///
/// This is a stub function that returns an empty vector. Actual collectible mappings
/// will be populated in a future issue.
#[must_use]
pub fn mm_collectible_mappings() -> Vec<MmFlagMapping> {
    // TODO: Populate with actual collectible mappings
    Vec::new()
}

/// Get MM special flag mappings.
///
/// Returns a vector of mappings for special collectibles that don't fit into
/// the standard chest/collectible categories. This includes:
/// - Stray fairies (dungeon and overworld)
/// - Other unique collectibles
///
/// # Note
///
/// This is a stub function that returns an empty vector. Actual special mappings
/// will be populated in a future issue.
#[must_use]
pub fn mm_special_mappings() -> Vec<MmFlagMapping> {
    // TODO: Populate with actual special mappings (stray fairies, etc.)
    Vec::new()
}

/// Get all MM flag mappings combined.
///
/// Returns a vector containing all chest, collectible, and special mappings.
///
/// # Note
///
/// This is a stub function that returns an empty vector until the individual
/// mapping functions are populated.
#[must_use]
pub fn mm_all_mappings() -> Vec<MmFlagMapping> {
    let mut all = mm_chest_mappings();
    all.extend(mm_collectible_mappings());
    all.extend(mm_special_mappings());
    all
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_flag_type_offsets() {
        assert_eq!(MmFlagType::Chest.offset(), 0x00);
        assert_eq!(MmFlagType::Switch.offset(), 0x04);
        assert_eq!(MmFlagType::RoomClear.offset(), 0x08);
        assert_eq!(MmFlagType::Collectible.offset(), 0x0C);
        assert_eq!(MmFlagType::Special.offset(), 0x10);
    }

    #[test]
    fn test_scene_flag_save_offset() {
        // Scene 0, chest flag
        let flag = MmSceneFlag::new(0, MmFlagType::Chest, 0x01);
        assert_eq!(flag.save_offset(), 0x00);

        // Scene 1, collectible flag
        let flag = MmSceneFlag::new(1, MmFlagType::Collectible, 0x01);
        // Scene 1 offset = 0x14, collectible offset = 0x0C
        assert_eq!(flag.save_offset(), 0x14 + 0x0C);

        // Scene 0x6C (Clock Town), chest flag
        let flag = MmSceneFlag::new(0x6C, MmFlagType::Chest, 0x01);
        assert_eq!(flag.save_offset(), 0x6C * 0x14);
    }

    #[test]
    fn test_flag_mapping_creation() {
        let mapping = MmFlagMapping::new(
            "Test Location",
            MmSceneFlag::new(0x6C, MmFlagType::Chest, 0x01),
        );
        assert_eq!(mapping.location_name, "Test Location");
        assert_eq!(mapping.flag.scene_id, 0x6C);
        assert_eq!(mapping.flag.flag_type, MmFlagType::Chest);
        assert_eq!(mapping.flag.bit_mask, 0x01);
    }

    #[test]
    fn test_stub_functions_return_empty() {
        assert!(mm_chest_mappings().is_empty());
        assert!(mm_collectible_mappings().is_empty());
        assert!(mm_special_mappings().is_empty());
        assert!(mm_all_mappings().is_empty());
    }

    #[test]
    fn test_offsets_constants() {
        assert_eq!(offsets::MM_SAVE_BASE, 0x1EF670);
        assert_eq!(offsets::SCENE_SIZE, 0x14);
        assert_eq!(offsets::NUM_SCENES, 0x78);
    }
}
