//! Type definitions for OoT flag mappings.

/// Types of flags used to track location checks in OoT save data.
///
/// Each location in the game is tracked by one of these flag types,
/// stored in specific memory regions within the save context.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum FlagType {
    /// Chest opened flags (scene flags offset 0x00).
    /// Each bit represents a chest in the scene.
    Chest,

    /// Switch/trigger flags (scene flags offset 0x04).
    /// Includes crystal switches, floor switches, etc.
    Switch,

    /// Room clear flags (scene flags offset 0x08).
    /// Set when all enemies in a room are defeated.
    RoomClear,

    /// Collectible item flags (scene flags offset 0x0C).
    /// Freestanding items, rupees, hearts, etc.
    Collectible,

    /// Gold Skulltula flags (separate from scene flags).
    /// Stored in dedicated skulltula section at 0x0E9C.
    GoldSkulltula,

    /// Event check flags (event_chk_inf).
    /// Global events like cutscenes watched, NPCs talked to.
    EventChkInf,

    /// Item get flags (item_get_inf).
    /// Tracks specific item acquisitions.
    ItemGetInf,

    /// Info table flags (inf_table).
    /// NPC conversation flags, misc game state.
    InfTable,

    /// Shop item flags.
    /// Tracks purchased shop items.
    Shop,

    /// Scrub/merchant purchase flags.
    /// Business scrubs and other merchants.
    Scrub,

    /// Great Fairy reward flags.
    /// Tracks fairy fountain upgrades received.
    GreatFairy,

    /// Boss defeated flags.
    /// Typically stored in switch or event flags.
    Boss,

    /// Song learned flags.
    /// Stored in quest items bitfield.
    Song,

    /// Fishing pond flags.
    /// Special handling for fishing rewards.
    Fishing,

    /// Cow/milk flags.
    /// Playing Epona's Song to cows.
    Cow,

    /// Gossip stone flags.
    /// Hints from gossip stones (if shuffled).
    GossipStone,
}

impl FlagType {
    /// Returns the byte offset within scene flags for scene-based flag types.
    ///
    /// Returns `None` for global flag types that aren't stored per-scene.
    #[must_use]
    pub const fn scene_offset(&self) -> Option<usize> {
        match self {
            FlagType::Chest => Some(0x00),
            FlagType::Switch => Some(0x04),
            FlagType::RoomClear => Some(0x08),
            FlagType::Collectible => Some(0x0C),
            // These are global, not per-scene
            FlagType::GoldSkulltula
            | FlagType::EventChkInf
            | FlagType::ItemGetInf
            | FlagType::InfTable
            | FlagType::Shop
            | FlagType::Scrub
            | FlagType::GreatFairy
            | FlagType::Boss
            | FlagType::Song
            | FlagType::Fishing
            | FlagType::Cow
            | FlagType::GossipStone => None,
        }
    }

    /// Returns whether this flag type is stored per-scene.
    #[must_use]
    pub const fn is_scene_based(&self) -> bool {
        self.scene_offset().is_some()
    }
}

/// Mapping from a location ID to its flag address in memory.
///
/// This struct represents either a complete mapping (with scene_id, flag_type,
/// and flag_bit populated) or a stub mapping (with all optional fields as None).
///
/// Stub mappings are generated for all locations from world data and serve as
/// placeholders until the actual flag addresses are researched and filled in.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FlagMapping {
    /// The unique location identifier from OoTMM world data.
    pub location_id: &'static str,

    /// The scene ID where this flag is stored (None for unmapped stubs or global flags).
    pub scene_id: Option<u8>,

    /// The type of flag used for this location (None for unmapped stubs).
    pub flag_type: Option<FlagType>,

    /// The bit position within the flag word (None for unmapped stubs).
    /// For 32-bit flag words, this is typically 0-31 representing a single bit,
    /// or a full bitmask for multi-bit values.
    pub flag_bit: Option<u32>,
}

impl FlagMapping {
    /// Creates a new unmapped stub mapping for a location.
    #[must_use]
    pub const fn stub(location_id: &'static str) -> Self {
        Self {
            location_id,
            scene_id: None,
            flag_type: None,
            flag_bit: None,
        }
    }

    /// Creates a new fully mapped location.
    #[must_use]
    pub const fn mapped(
        location_id: &'static str,
        scene_id: u8,
        flag_type: FlagType,
        flag_bit: u32,
    ) -> Self {
        Self {
            location_id,
            scene_id: Some(scene_id),
            flag_type: Some(flag_type),
            flag_bit: Some(flag_bit),
        }
    }

    /// Creates a new global flag mapping (no scene_id).
    #[must_use]
    pub const fn global(location_id: &'static str, flag_type: FlagType, flag_bit: u32) -> Self {
        Self {
            location_id,
            scene_id: None,
            flag_type: Some(flag_type),
            flag_bit: Some(flag_bit),
        }
    }

    /// Returns whether this is an unmapped stub.
    #[must_use]
    pub const fn is_stub(&self) -> bool {
        self.flag_type.is_none()
    }

    /// Returns whether this mapping is complete (has flag type and bit).
    #[must_use]
    pub const fn is_mapped(&self) -> bool {
        self.flag_type.is_some() && self.flag_bit.is_some()
    }
}

/// Status of a location check.
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum CheckStatus {
    /// Location has been checked (item collected).
    Checked,
    /// Location has not been checked yet.
    Unchecked,
    /// User has decided to skip this location (won't complete it).
    Skipped,
    /// Check status cannot be determined (unmapped or unknown).
    Unknown,
}

/// Accessibility status based on logic evaluation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, serde::Serialize, serde::Deserialize)]
pub enum Accessibility {
    /// The location is accessible with current items and state.
    Available,
    /// The location is not accessible yet.
    Unavailable,
    /// Accessibility cannot be determined (no logic defined or evaluation failed).
    #[default]
    Unknown,
}

/// Result of checking a location.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct LocationCheckResult {
    /// The location ID.
    pub location_id: String,
    /// Whether the location has been checked.
    pub status: CheckStatus,
    /// Whether this location has a valid flag mapping.
    pub is_mapped: bool,
    /// Logic expression required to access this location (if available).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logic: Option<String>,
    /// Accessibility based on logic evaluation.
    #[serde(default)]
    pub accessibility: Accessibility,
}

/// Summary of checked locations.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CheckedLocationsSummary {
    /// Total number of mapped locations.
    pub total_mapped: usize,
    /// Number of checked locations.
    pub checked_count: usize,
    /// Number of unchecked locations.
    pub unchecked_count: usize,
    /// Number of skipped locations.
    pub skipped_count: usize,
    /// Number of locations with unknown status.
    pub unknown_count: usize,
    /// Number of available (accessible) unchecked locations.
    #[serde(default)]
    pub available_count: usize,
    /// Number of unavailable (inaccessible) unchecked locations.
    #[serde(default)]
    pub unavailable_count: usize,
    /// List of location check results.
    pub locations: Vec<LocationCheckResult>,
    /// Current scene ID from RAM (used for auto-scrolling the check tracker).
    /// This is the OoT scene ID when in OoT, or the MM scene ID when in MM.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_scene_id: Option<u8>,
    /// Indicates which game is currently active (for combo randomizer).
    /// Values: "oot" for Ocarina of Time, "mm" for Majora's Mask, None if unknown.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_game: Option<String>,
}
