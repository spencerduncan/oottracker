//! OoT flag address validation for emulator testing.
//!
//! This module validates that OoT flag addresses in flag_mapping.rs correctly
//! correspond to game memory. It provides specialized validators for scene flags
//! and global flags that can be used with the RAM validation harness.
//!
//! # Flag Structure
//!
//! OoT save data contains several types of flags stored in different memory regions
//! within the save context (base address 0x11A5D0):
//!
//! ## Scene Flags (at save context offset 0x00D4)
//!
//! Each scene has a 0x1C (28) byte entry containing:
//! - `chests` (u32 at offset 0x00): Opened chest flags
//! - `switches` (u32 at offset 0x04): Switch/trigger flags
//! - `room_clear` (u32 at offset 0x08): Room clear flags
//! - `collectible` (u32 at offset 0x0C): Collectible item flags
//! - `unused` (u32 at offset 0x10): Unused flags
//! - `visited_rooms` (u32 at offset 0x14): Room visited flags
//! - `visited_floors` (u32 at offset 0x18): Floor visited flags
//!
//! ## Global Flags (at various offsets)
//!
//! - Gold Skulltulas: 0x0E9C (0x18 bytes)
//! - Event flags (event_chk_inf): 0x0ED4 (0x1C bytes)
//! - Item get flags (item_get_inf): 0x0EF0 (0x08 bytes)
//! - Info table (inf_table): 0x0EF8 (0x3C bytes)
//!
//! # Example
//!
//! ```ignore
//! use oottracker_e2e::oot_flag_validation::{
//!     OotFlagValidator, SceneFlagType, GlobalFlagType,
//!     scene_flag_offset, global_flag_offset,
//! };
//! use oottracker_e2e::ram_validation::{RamValidator, ExpectedValue};
//!
//! // Validate Deku Tree chest flags
//! let validator = OotFlagValidator::scene_flags(0x00, SceneFlagType::Chest)
//!     .expect_bit_set(0x0000_0002, "Deku Tree Slingshot Chest");
//!
//! let report = validator.validate_data(&save_data);
//! assert!(report.passed());
//! ```

use crate::ram_validation::{CompareMode, ExpectedValue, RamValidator, OOT_SAVE_ADDR};

// ============================================================================
// OoT Save Context Offsets
// ============================================================================

/// Offset of scene flags array within save context.
pub const SCENE_FLAGS_OFFSET: u32 = 0x00D4;

/// Size of each scene's flag entry (0x1C = 28 bytes).
pub const SCENE_FLAG_ENTRY_SIZE: u32 = 0x1C;

/// Number of scenes in OoT.
pub const SCENE_COUNT: u8 = 101;

/// Offset of Gold Skulltula flags within save context.
pub const SKULLTULA_FLAGS_OFFSET: u32 = 0x0E9C;

/// Size of Gold Skulltula flags (0x18 = 24 bytes).
pub const SKULLTULA_FLAGS_SIZE: u32 = 0x18;

/// Offset of event check flags (event_chk_inf) within save context.
pub const EVENT_CHK_INF_OFFSET: u32 = 0x0ED4;

/// Size of event check flags (0x1C = 28 bytes).
pub const EVENT_CHK_INF_SIZE: u32 = 0x1C;

/// Offset of item get flags (item_get_inf) within save context.
pub const ITEM_GET_INF_OFFSET: u32 = 0x0EF0;

/// Size of item get flags (0x08 = 8 bytes).
pub const ITEM_GET_INF_SIZE: u32 = 0x08;

/// Offset of info table (inf_table) within save context.
pub const INF_TABLE_OFFSET: u32 = 0x0EF8;

/// Size of info table (0x3C = 60 bytes).
pub const INF_TABLE_SIZE: u32 = 0x3C;

// ============================================================================
// Scene Flag Types
// ============================================================================

/// Types of flags stored per-scene.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SceneFlagType {
    /// Chest opened flags (offset 0x00 within scene entry).
    Chest,
    /// Switch/trigger flags (offset 0x04 within scene entry).
    Switch,
    /// Room clear flags (offset 0x08 within scene entry).
    RoomClear,
    /// Collectible item flags (offset 0x0C within scene entry).
    Collectible,
    /// Unused flags (offset 0x10 within scene entry).
    Unused,
    /// Room visited flags (offset 0x14 within scene entry).
    VisitedRooms,
    /// Floor visited flags (offset 0x18 within scene entry).
    VisitedFloors,
}

impl SceneFlagType {
    /// Returns the byte offset within a scene flag entry for this flag type.
    #[must_use]
    pub const fn offset_within_scene(&self) -> u32 {
        match self {
            SceneFlagType::Chest => 0x00,
            SceneFlagType::Switch => 0x04,
            SceneFlagType::RoomClear => 0x08,
            SceneFlagType::Collectible => 0x0C,
            SceneFlagType::Unused => 0x10,
            SceneFlagType::VisitedRooms => 0x14,
            SceneFlagType::VisitedFloors => 0x18,
        }
    }

    /// Returns a human-readable name for this flag type.
    #[must_use]
    pub const fn name(&self) -> &'static str {
        match self {
            SceneFlagType::Chest => "Chest",
            SceneFlagType::Switch => "Switch",
            SceneFlagType::RoomClear => "RoomClear",
            SceneFlagType::Collectible => "Collectible",
            SceneFlagType::Unused => "Unused",
            SceneFlagType::VisitedRooms => "VisitedRooms",
            SceneFlagType::VisitedFloors => "VisitedFloors",
        }
    }
}

// ============================================================================
// Global Flag Types
// ============================================================================

/// Types of global flags (not stored per-scene).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GlobalFlagType {
    /// Gold Skulltula collection flags.
    GoldSkulltula,
    /// Event check flags (event_chk_inf).
    EventChkInf,
    /// Item get flags (item_get_inf).
    ItemGetInf,
    /// Info table flags (inf_table).
    InfTable,
}

impl GlobalFlagType {
    /// Returns the offset within save context for this global flag type.
    #[must_use]
    pub const fn save_offset(&self) -> u32 {
        match self {
            GlobalFlagType::GoldSkulltula => SKULLTULA_FLAGS_OFFSET,
            GlobalFlagType::EventChkInf => EVENT_CHK_INF_OFFSET,
            GlobalFlagType::ItemGetInf => ITEM_GET_INF_OFFSET,
            GlobalFlagType::InfTable => INF_TABLE_OFFSET,
        }
    }

    /// Returns the size of this global flag region.
    #[must_use]
    pub const fn size(&self) -> u32 {
        match self {
            GlobalFlagType::GoldSkulltula => SKULLTULA_FLAGS_SIZE,
            GlobalFlagType::EventChkInf => EVENT_CHK_INF_SIZE,
            GlobalFlagType::ItemGetInf => ITEM_GET_INF_SIZE,
            GlobalFlagType::InfTable => INF_TABLE_SIZE,
        }
    }

    /// Returns a human-readable name for this flag type.
    #[must_use]
    pub const fn name(&self) -> &'static str {
        match self {
            GlobalFlagType::GoldSkulltula => "GoldSkulltula",
            GlobalFlagType::EventChkInf => "EventChkInf",
            GlobalFlagType::ItemGetInf => "ItemGetInf",
            GlobalFlagType::InfTable => "InfTable",
        }
    }
}

// ============================================================================
// Address Calculation Functions
// ============================================================================

/// Calculates the save context offset for a scene flag.
///
/// # Arguments
///
/// * `scene_id` - The scene ID (0-100)
/// * `flag_type` - The type of scene flag
///
/// # Returns
///
/// The offset within the save context where this flag is stored.
///
/// # Example
///
/// ```ignore
/// // Deku Tree (scene 0x00) chest flags
/// let offset = scene_flag_offset(0x00, SceneFlagType::Chest);
/// assert_eq!(offset, 0x00D4); // SCENE_FLAGS_OFFSET + 0x00 * 0x1C + 0x00
///
/// // Dodongo's Cavern (scene 0x01) switch flags
/// let offset = scene_flag_offset(0x01, SceneFlagType::Switch);
/// assert_eq!(offset, 0x00F4); // SCENE_FLAGS_OFFSET + 0x01 * 0x1C + 0x04
/// ```
#[must_use]
pub const fn scene_flag_offset(scene_id: u8, flag_type: SceneFlagType) -> u32 {
    SCENE_FLAGS_OFFSET + (scene_id as u32) * SCENE_FLAG_ENTRY_SIZE + flag_type.offset_within_scene()
}

/// Calculates the absolute RAM address for a scene flag.
///
/// # Arguments
///
/// * `scene_id` - The scene ID (0-100)
/// * `flag_type` - The type of scene flag
///
/// # Returns
///
/// The absolute address in RDRAM where this flag is stored.
#[must_use]
pub const fn scene_flag_address(scene_id: u8, flag_type: SceneFlagType) -> u32 {
    OOT_SAVE_ADDR + scene_flag_offset(scene_id, flag_type)
}

/// Calculates the save context offset for a global flag.
///
/// # Arguments
///
/// * `flag_type` - The type of global flag
/// * `byte_offset` - Optional byte offset within the flag region
///
/// # Returns
///
/// The offset within the save context where this flag is stored.
#[must_use]
pub const fn global_flag_offset(flag_type: GlobalFlagType, byte_offset: u32) -> u32 {
    flag_type.save_offset() + byte_offset
}

/// Calculates the absolute RAM address for a global flag.
///
/// # Arguments
///
/// * `flag_type` - The type of global flag
/// * `byte_offset` - Optional byte offset within the flag region
///
/// # Returns
///
/// The absolute address in RDRAM where this flag is stored.
#[must_use]
pub const fn global_flag_address(flag_type: GlobalFlagType, byte_offset: u32) -> u32 {
    OOT_SAVE_ADDR + global_flag_offset(flag_type, byte_offset)
}

// ============================================================================
// OoT Flag Validator Builder
// ============================================================================

/// Builder for creating OoT flag validators.
///
/// This provides a convenient API for creating validators that check
/// specific flag addresses in OoT save data.
#[derive(Debug, Clone)]
pub struct OotFlagValidator {
    validator: RamValidator,
}

impl OotFlagValidator {
    /// Creates a new OoT flag validator with a custom name.
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            validator: RamValidator::oot_save().with_mode(CompareMode::Exact),
        }
        .with_name(name)
    }

    /// Sets the validator name.
    pub fn with_name(mut self, name: impl Into<String>) -> Self {
        self.validator.name = name.into();
        self
    }

    /// Creates a validator for a specific scene's flags.
    pub fn scene_flags(scene_id: u8, flag_type: SceneFlagType) -> Self {
        let name = format!("Scene {} {} Flags", scene_id, flag_type.name());
        Self::new(name)
    }

    /// Creates a validator for global flags.
    pub fn global_flags(flag_type: GlobalFlagType) -> Self {
        let name = format!("{} Flags", flag_type.name());
        Self::new(name)
    }

    /// Adds an expectation for a scene flag value.
    ///
    /// # Arguments
    ///
    /// * `scene_id` - The scene ID
    /// * `flag_type` - The type of scene flag
    /// * `expected_value` - The expected u32 value
    /// * `description` - Human-readable description
    pub fn expect_scene_flag(
        mut self,
        scene_id: u8,
        flag_type: SceneFlagType,
        expected_value: u32,
        description: impl Into<String>,
    ) -> Self {
        let offset = scene_flag_offset(scene_id, flag_type);
        let desc = description.into();
        self.validator
            .expectations
            .push(ExpectedValue::u32_be(offset, expected_value, desc));
        self
    }

    /// Adds an expectation that specific bits are set in a scene flag.
    pub fn expect_scene_flag_bits_set(
        mut self,
        scene_id: u8,
        flag_type: SceneFlagType,
        bits: u32,
        description: impl Into<String>,
    ) -> Self {
        let offset = scene_flag_offset(scene_id, flag_type);
        let desc = description.into();
        let mut expected = ExpectedValue::u32_be(offset, bits, desc);
        expected.critical = true;
        self.validator.expectations.push(expected);
        self.validator.mode = CompareMode::BitsSet;
        self
    }

    /// Adds an expectation for a global flag value.
    ///
    /// # Arguments
    ///
    /// * `flag_type` - The type of global flag
    /// * `byte_offset` - Byte offset within the flag region
    /// * `expected_value` - The expected u32 value
    /// * `description` - Human-readable description
    pub fn expect_global_flag(
        mut self,
        flag_type: GlobalFlagType,
        byte_offset: u32,
        expected_value: u32,
        description: impl Into<String>,
    ) -> Self {
        let offset = global_flag_offset(flag_type, byte_offset);
        let desc = description.into();
        self.validator
            .expectations
            .push(ExpectedValue::u32_be(offset, expected_value, desc));
        self
    }

    /// Adds an expectation for a global flag byte value.
    pub fn expect_global_flag_byte(
        mut self,
        flag_type: GlobalFlagType,
        byte_offset: u32,
        expected_value: u8,
        description: impl Into<String>,
    ) -> Self {
        let offset = global_flag_offset(flag_type, byte_offset);
        let desc = description.into();
        self.validator
            .expectations
            .push(ExpectedValue::byte(offset, expected_value, desc));
        self
    }

    /// Adds an expectation that specific bits are set in a global flag.
    pub fn expect_global_flag_bits_set(
        mut self,
        flag_type: GlobalFlagType,
        byte_offset: u32,
        bits: u32,
        description: impl Into<String>,
    ) -> Self {
        let offset = global_flag_offset(flag_type, byte_offset);
        let desc = description.into();
        let mut expected = ExpectedValue::u32_be(offset, bits, desc);
        expected.critical = true;
        self.validator.expectations.push(expected);
        self.validator.mode = CompareMode::BitsSet;
        self
    }

    /// Sets the comparison mode.
    pub fn with_mode(mut self, mode: CompareMode) -> Self {
        self.validator.mode = mode;
        self
    }

    /// Returns the inner RamValidator.
    pub fn build(self) -> RamValidator {
        self.validator
    }

    /// Validates against raw data (for unit testing without emulator).
    pub fn validate_data(&self, data: &[u8]) -> crate::ram_validation::ValidationReport {
        self.validator.validate_data(data)
    }
}

// ============================================================================
// Scene ID Constants (for convenience)
// ============================================================================

/// Scene ID constants matching flag_mapping.rs
pub mod scene {
    // Dungeons
    pub const DEKU_TREE: u8 = 0x00;
    pub const DODONGOS_CAVERN: u8 = 0x01;
    pub const JABU_JABU: u8 = 0x02;
    pub const FOREST_TEMPLE: u8 = 0x03;
    pub const FIRE_TEMPLE: u8 = 0x04;
    pub const WATER_TEMPLE: u8 = 0x05;
    pub const SPIRIT_TEMPLE: u8 = 0x06;
    pub const SHADOW_TEMPLE: u8 = 0x07;
    pub const BOTTOM_OF_THE_WELL: u8 = 0x08;
    pub const ICE_CAVERN: u8 = 0x09;
    pub const GANONS_TOWER: u8 = 0x0A;
    pub const GERUDO_TRAINING_GROUND: u8 = 0x0B;
    pub const THIEVES_HIDEOUT: u8 = 0x0C;
    pub const GANONS_CASTLE: u8 = 0x0D;

    // Boss Rooms
    pub const DEKU_TREE_BOSS: u8 = 0x11;
    pub const DODONGOS_CAVERN_BOSS: u8 = 0x12;

    // Houses/Interiors
    pub const MIDOS_HOUSE: u8 = 0x28;
    pub const IMPAS_HOUSE: u8 = 0x37;
    pub const GREAT_FAIRY_FOUNTAIN_UPGRADES: u8 = 0x3B;
    pub const GROTTOS: u8 = 0x3E;

    // Overworld
    pub const KOKIRI_FOREST: u8 = 0x55;
    pub const HYRULE_FIELD: u8 = 0x51;
    pub const KAKARIKO_VILLAGE: u8 = 0x52;
    pub const GRAVEYARD: u8 = 0x53;
    pub const ZORA_RIVER: u8 = 0x54;
    pub const LAKE_HYLIA: u8 = 0x57;
    pub const ZORAS_DOMAIN: u8 = 0x58;
    pub const ZORAS_FOUNTAIN: u8 = 0x59;
    pub const GERUDO_VALLEY: u8 = 0x5A;
    pub const LOST_WOODS: u8 = 0x5B;
    pub const DESERT_COLOSSUS: u8 = 0x5C;
    pub const GERUDO_FORTRESS: u8 = 0x5D;
    pub const HAUNTED_WASTELAND: u8 = 0x5E;
    pub const DEATH_MOUNTAIN_TRAIL: u8 = 0x60;
    pub const DEATH_MOUNTAIN_CRATER: u8 = 0x61;
    pub const GORON_CITY: u8 = 0x62;
}

// ============================================================================
// Pre-defined Validators
// ============================================================================

/// Creates a validator that checks all Deku Tree chest flags.
pub fn deku_tree_chests_validator() -> OotFlagValidator {
    OotFlagValidator::new("Deku Tree Chest Flags").expect_scene_flag(
        scene::DEKU_TREE,
        SceneFlagType::Chest,
        0x0000_007E,
        "All Deku Tree Chests",
    )
}

/// Creates a validator that checks all Dodongo's Cavern chest flags.
pub fn dodongos_cavern_chests_validator() -> OotFlagValidator {
    OotFlagValidator::new("Dodongo's Cavern Chest Flags").expect_scene_flag(
        scene::DODONGOS_CAVERN,
        SceneFlagType::Chest,
        0x0000_0570,
        "All Dodongo's Cavern Chests",
    )
}

/// Creates a validator for checking skulltula collection.
pub fn skulltula_validator(expected_count: u8) -> OotFlagValidator {
    OotFlagValidator::new("Gold Skulltula Collection").expect_global_flag_byte(
        GlobalFlagType::GoldSkulltula,
        0x00,
        expected_count,
        "Skulltula Count",
    )
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_scene_flag_offset_calculation() {
        // Deku Tree (scene 0x00) chest flags at 0x00D4 + 0x00 * 0x1C + 0x00
        assert_eq!(
            scene_flag_offset(0x00, SceneFlagType::Chest),
            SCENE_FLAGS_OFFSET
        );

        // Deku Tree switch flags at 0x00D4 + 0x00 * 0x1C + 0x04
        assert_eq!(
            scene_flag_offset(0x00, SceneFlagType::Switch),
            SCENE_FLAGS_OFFSET + 0x04
        );

        // Dodongo's Cavern (scene 0x01) chest flags at 0x00D4 + 0x01 * 0x1C + 0x00
        assert_eq!(
            scene_flag_offset(0x01, SceneFlagType::Chest),
            SCENE_FLAGS_OFFSET + SCENE_FLAG_ENTRY_SIZE
        );

        // Forest Temple (scene 0x03) collectible flags at 0x00D4 + 0x03 * 0x1C + 0x0C
        assert_eq!(
            scene_flag_offset(0x03, SceneFlagType::Collectible),
            SCENE_FLAGS_OFFSET + 3 * SCENE_FLAG_ENTRY_SIZE + 0x0C
        );
    }

    #[test]
    fn test_global_flag_offset_calculation() {
        // Gold Skulltulas at 0x0E9C
        assert_eq!(
            global_flag_offset(GlobalFlagType::GoldSkulltula, 0),
            SKULLTULA_FLAGS_OFFSET
        );

        // Event flags at 0x0ED4
        assert_eq!(
            global_flag_offset(GlobalFlagType::EventChkInf, 0),
            EVENT_CHK_INF_OFFSET
        );

        // Item get flags at 0x0EF0
        assert_eq!(
            global_flag_offset(GlobalFlagType::ItemGetInf, 0),
            ITEM_GET_INF_OFFSET
        );

        // Info table at 0x0EF8
        assert_eq!(
            global_flag_offset(GlobalFlagType::InfTable, 0),
            INF_TABLE_OFFSET
        );
    }

    #[test]
    fn test_scene_flag_address_calculation() {
        // Deku Tree chest flags absolute address
        let expected = OOT_SAVE_ADDR + SCENE_FLAGS_OFFSET;
        assert_eq!(scene_flag_address(0x00, SceneFlagType::Chest), expected);
    }

    #[test]
    fn test_validator_scene_flags() {
        // Create simulated save data
        let mut data = vec![0u8; 0x1500];

        // Set some Deku Tree chest flags at the correct offset
        let deku_chest_offset = scene_flag_offset(0x00, SceneFlagType::Chest) as usize;
        // Set chest flag 0x0000_0002 (Slingshot Chest)
        data[deku_chest_offset + 3] = 0x02;

        let validator = OotFlagValidator::scene_flags(scene::DEKU_TREE, SceneFlagType::Chest)
            .expect_scene_flag(
                scene::DEKU_TREE,
                SceneFlagType::Chest,
                0x0000_0002,
                "Deku Tree Slingshot Chest",
            );

        let report = validator.validate_data(&data);
        assert!(report.passed(), "Validation should pass: {}", report);
    }

    #[test]
    fn test_validator_scene_flags_fails_on_mismatch() {
        let data = vec![0u8; 0x1500]; // All zeros

        let validator = OotFlagValidator::scene_flags(scene::DEKU_TREE, SceneFlagType::Chest)
            .expect_scene_flag(
                scene::DEKU_TREE,
                SceneFlagType::Chest,
                0x0000_0002,
                "Deku Tree Slingshot Chest",
            );

        let report = validator.validate_data(&data);
        assert!(!report.passed(), "Validation should fail when flag not set");
    }

    #[test]
    fn test_validator_global_skulltula_flags() {
        let mut data = vec![0u8; 0x1500];

        // Set skulltula count at 0x0E9C
        let skulltula_offset = global_flag_offset(GlobalFlagType::GoldSkulltula, 0) as usize;
        data[skulltula_offset] = 10; // 10 skulltulas collected

        let validator = OotFlagValidator::global_flags(GlobalFlagType::GoldSkulltula)
            .expect_global_flag_byte(GlobalFlagType::GoldSkulltula, 0, 10, "Skulltula Count");

        let report = validator.validate_data(&data);
        assert!(report.passed(), "Validation should pass: {}", report);
    }

    #[test]
    fn test_validator_event_chk_inf() {
        let mut data = vec![0u8; 0x1500];

        // Set event flag at 0x0ED4 + 0x00
        let event_offset = global_flag_offset(GlobalFlagType::EventChkInf, 0) as usize;
        // Set big-endian u32 value 0x0000_0001
        data[event_offset + 3] = 0x01;

        let validator = OotFlagValidator::global_flags(GlobalFlagType::EventChkInf)
            .expect_global_flag(
                GlobalFlagType::EventChkInf,
                0,
                0x0000_0001,
                "First Event Flag",
            );

        let report = validator.validate_data(&data);
        assert!(report.passed(), "Validation should pass: {}", report);
    }

    #[test]
    fn test_bits_set_comparison_mode() {
        let mut data = vec![0u8; 0x1500];

        // Set multiple chest flags
        let deku_chest_offset = scene_flag_offset(0x00, SceneFlagType::Chest) as usize;
        // Set flags 0x0000_007E (all 6 Deku Tree chests)
        data[deku_chest_offset + 3] = 0x7E;

        // Check that specific bits are set
        let validator = OotFlagValidator::scene_flags(scene::DEKU_TREE, SceneFlagType::Chest)
            .expect_scene_flag_bits_set(
                scene::DEKU_TREE,
                SceneFlagType::Chest,
                0x0000_0002, // Only check Slingshot Chest bit
                "Deku Tree Slingshot Chest",
            );

        let report = validator.validate_data(&data);
        assert!(
            report.passed(),
            "BitsSet mode should pass when bit is set: {}",
            report
        );
    }

    #[test]
    fn test_multiple_scene_expectations() {
        let mut data = vec![0u8; 0x1500];

        // Set Deku Tree chests
        let deku_offset = scene_flag_offset(scene::DEKU_TREE, SceneFlagType::Chest) as usize;
        data[deku_offset + 3] = 0x7E;

        // Set Dodongo's Cavern chests
        let dodongo_offset =
            scene_flag_offset(scene::DODONGOS_CAVERN, SceneFlagType::Chest) as usize;
        data[dodongo_offset + 2] = 0x05;
        data[dodongo_offset + 3] = 0x70;

        let validator = OotFlagValidator::new("Multiple Dungeon Chests")
            .expect_scene_flag(
                scene::DEKU_TREE,
                SceneFlagType::Chest,
                0x0000_007E,
                "All Deku Tree Chests",
            )
            .expect_scene_flag(
                scene::DODONGOS_CAVERN,
                SceneFlagType::Chest,
                0x0000_0570,
                "All Dodongo's Cavern Chests",
            );

        let report = validator.validate_data(&data);
        assert!(report.passed(), "All expectations should pass: {}", report);
    }

    #[test]
    fn test_scene_flag_type_offsets() {
        assert_eq!(SceneFlagType::Chest.offset_within_scene(), 0x00);
        assert_eq!(SceneFlagType::Switch.offset_within_scene(), 0x04);
        assert_eq!(SceneFlagType::RoomClear.offset_within_scene(), 0x08);
        assert_eq!(SceneFlagType::Collectible.offset_within_scene(), 0x0C);
        assert_eq!(SceneFlagType::Unused.offset_within_scene(), 0x10);
        assert_eq!(SceneFlagType::VisitedRooms.offset_within_scene(), 0x14);
        assert_eq!(SceneFlagType::VisitedFloors.offset_within_scene(), 0x18);
    }

    #[test]
    fn test_global_flag_type_offsets() {
        assert_eq!(
            GlobalFlagType::GoldSkulltula.save_offset(),
            SKULLTULA_FLAGS_OFFSET
        );
        assert_eq!(
            GlobalFlagType::EventChkInf.save_offset(),
            EVENT_CHK_INF_OFFSET
        );
        assert_eq!(
            GlobalFlagType::ItemGetInf.save_offset(),
            ITEM_GET_INF_OFFSET
        );
        assert_eq!(GlobalFlagType::InfTable.save_offset(), INF_TABLE_OFFSET);
    }

    #[test]
    fn test_global_flag_type_sizes() {
        assert_eq!(GlobalFlagType::GoldSkulltula.size(), SKULLTULA_FLAGS_SIZE);
        assert_eq!(GlobalFlagType::EventChkInf.size(), EVENT_CHK_INF_SIZE);
        assert_eq!(GlobalFlagType::ItemGetInf.size(), ITEM_GET_INF_SIZE);
        assert_eq!(GlobalFlagType::InfTable.size(), INF_TABLE_SIZE);
    }

    // ========================================================================
    // Test Scenarios with Expected Bit Patterns
    // ========================================================================

    /// Test scenario: Fresh game start
    #[test]
    fn test_scenario_new_game() {
        let data = vec![0u8; 0x1500]; // All flags cleared

        let validator = OotFlagValidator::new("New Game - No Flags Set")
            .expect_scene_flag(
                scene::DEKU_TREE,
                SceneFlagType::Chest,
                0x0000_0000,
                "No Deku Tree Chests Opened",
            )
            .expect_global_flag_byte(
                GlobalFlagType::GoldSkulltula,
                0,
                0,
                "No Skulltulas Collected",
            );

        let report = validator.validate_data(&data);
        assert!(report.passed(), "New game should have all flags clear");
    }

    /// Test scenario: Deku Tree completed
    #[test]
    fn test_scenario_deku_tree_complete() {
        let mut data = vec![0u8; 0x1500];

        // Set all 6 Deku Tree chest flags (bits 1-6 = 0x7E)
        let deku_chest_offset = scene_flag_offset(scene::DEKU_TREE, SceneFlagType::Chest) as usize;
        data[deku_chest_offset + 3] = 0x7E;

        // Set some Deku Tree switch flags
        let deku_switch_offset =
            scene_flag_offset(scene::DEKU_TREE, SceneFlagType::Switch) as usize;
        data[deku_switch_offset + 3] = 0x58; // Some switches pressed

        let validator = OotFlagValidator::new("Deku Tree Complete")
            .expect_scene_flag(
                scene::DEKU_TREE,
                SceneFlagType::Chest,
                0x0000_007E,
                "All Deku Tree Chests Opened",
            )
            .expect_scene_flag_bits_set(
                scene::DEKU_TREE,
                SceneFlagType::Switch,
                0x0000_0040, // Boss door switch
                "Boss Door Switch",
            );

        let report = validator.validate_data(&data);
        assert!(
            report.passed(),
            "Deku Tree complete scenario should pass: {}",
            report
        );
    }

    /// Test scenario: Multiple dungeons completed
    #[test]
    fn test_scenario_child_dungeons_complete() {
        let mut data = vec![0u8; 0x1500];

        // Deku Tree chests
        let deku_offset = scene_flag_offset(scene::DEKU_TREE, SceneFlagType::Chest) as usize;
        data[deku_offset + 3] = 0x7E;

        // Dodongo's Cavern chests
        let dodongo_offset =
            scene_flag_offset(scene::DODONGOS_CAVERN, SceneFlagType::Chest) as usize;
        data[dodongo_offset + 2] = 0x05;
        data[dodongo_offset + 3] = 0x70;

        // Jabu Jabu chests
        let jabu_offset = scene_flag_offset(scene::JABU_JABU, SceneFlagType::Chest) as usize;
        data[jabu_offset + 3] = 0x16;

        // Some skulltulas
        let skulltula_offset = global_flag_offset(GlobalFlagType::GoldSkulltula, 0) as usize;
        data[skulltula_offset] = 15;

        let validator = OotFlagValidator::new("Child Dungeons Complete")
            .expect_scene_flag(
                scene::DEKU_TREE,
                SceneFlagType::Chest,
                0x0000_007E,
                "All Deku Tree Chests",
            )
            .expect_scene_flag(
                scene::DODONGOS_CAVERN,
                SceneFlagType::Chest,
                0x0000_0570,
                "All Dodongo's Cavern Chests",
            )
            .expect_scene_flag(
                scene::JABU_JABU,
                SceneFlagType::Chest,
                0x0000_0016,
                "All Jabu Jabu Chests",
            )
            .expect_global_flag_byte(
                GlobalFlagType::GoldSkulltula,
                0,
                15,
                "15 Skulltulas Collected",
            );

        let report = validator.validate_data(&data);
        assert!(
            report.passed(),
            "Child dungeons scenario should pass: {}",
            report
        );
    }

    /// Test individual chest flag validation against expected bit patterns
    #[test]
    fn test_deku_tree_individual_chest_bits() {
        // Test each individual chest flag from scene.rs
        let chest_flags: [(u32, &str); 6] = [
            (0x0000_0002, "Deku Tree Slingshot Chest"),
            (0x0000_0004, "Deku Tree Compass Chest"),
            (0x0000_0008, "Deku Tree Map Chest"),
            (0x0000_0010, "Deku Tree Basement Chest"),
            (0x0000_0020, "Deku Tree Slingshot Room Side Chest"),
            (0x0000_0040, "Deku Tree Compass Room Side Chest"),
        ];

        for (bit, name) in chest_flags {
            let mut data = vec![0u8; 0x1500];
            let offset = scene_flag_offset(scene::DEKU_TREE, SceneFlagType::Chest) as usize;

            // Set the specific bit
            let byte_idx = (32 - bit.leading_zeros() as usize - 1) / 8;
            let bit_in_byte = bit >> (byte_idx * 8);
            data[offset + 3 - byte_idx] = bit_in_byte as u8;

            let validator = OotFlagValidator::new(format!("Test: {}", name))
                .with_mode(CompareMode::BitsSet)
                .expect_scene_flag_bits_set(scene::DEKU_TREE, SceneFlagType::Chest, bit, name);

            let report = validator.validate_data(&data);
            assert!(
                report.passed(),
                "Individual chest flag {} (0x{:08X}) should validate: {}",
                name,
                bit,
                report
            );
        }
    }

    /// Test global event flag validation
    #[test]
    fn test_event_chk_inf_validation() {
        let mut data = vec![0u8; 0x1500];

        // Set some event flags at different byte offsets
        let event_base = global_flag_offset(GlobalFlagType::EventChkInf, 0) as usize;

        // Event byte 0: Some flags set
        data[event_base + 3] = 0x0F;

        // Event byte 4: More flags
        data[event_base + 7] = 0x80;

        let validator = OotFlagValidator::new("Event Flags Test")
            .expect_global_flag(
                GlobalFlagType::EventChkInf,
                0,
                0x0000_000F,
                "First Event Word",
            )
            .expect_global_flag(
                GlobalFlagType::EventChkInf,
                4,
                0x0000_0080,
                "Second Event Word",
            );

        let report = validator.validate_data(&data);
        assert!(report.passed(), "Event flags should validate: {}", report);
    }

    /// Test info table validation
    #[test]
    fn test_inf_table_validation() {
        let mut data = vec![0u8; 0x1500];

        let inf_offset = global_flag_offset(GlobalFlagType::InfTable, 0) as usize;
        data[inf_offset] = 0x12;
        data[inf_offset + 1] = 0x34;
        data[inf_offset + 2] = 0x56;
        data[inf_offset + 3] = 0x78;

        let validator = OotFlagValidator::new("Info Table Test").expect_global_flag(
            GlobalFlagType::InfTable,
            0,
            0x12345678,
            "First Info Word",
        );

        let report = validator.validate_data(&data);
        assert!(report.passed(), "Info table should validate: {}", report);
    }

    /// Test item get flags validation
    #[test]
    fn test_item_get_inf_validation() {
        let mut data = vec![0u8; 0x1500];

        let item_offset = global_flag_offset(GlobalFlagType::ItemGetInf, 0) as usize;
        data[item_offset + 3] = 0xFF; // All items in first word

        let validator = OotFlagValidator::new("Item Get Flags Test").expect_global_flag(
            GlobalFlagType::ItemGetInf,
            0,
            0x0000_00FF,
            "First Item Get Word",
        );

        let report = validator.validate_data(&data);
        assert!(
            report.passed(),
            "Item get flags should validate: {}",
            report
        );
    }
}
