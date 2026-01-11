//! MM Flag Mapping structure definition.
//!
//! This module defines the `MmFlagMapping` struct which represents the mapping
//! from a location ID to its flag address in memory for MM.

use super::types::MmFlagType;

/// Mapping from a location ID to its flag address in memory for MM.
///
/// This struct represents either a complete mapping (with scene_id, flag_type,
/// and flag_bit populated) or a stub mapping (with all optional fields as None).
///
/// Stub mappings are generated for all locations from world data and serve as
/// placeholders until the actual flag addresses are researched and filled in.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MmFlagMapping {
    /// The unique location identifier from OoTMM world data.
    pub location_id: &'static str,

    /// The scene ID where this flag is stored (None for unmapped stubs or global flags).
    pub scene_id: Option<u8>,

    /// The type of flag used for this location (None for unmapped stubs).
    pub flag_type: Option<MmFlagType>,

    /// The bit position within the flag word (None for unmapped stubs).
    /// For 32-bit flag words, this is typically 0-31 representing a single bit,
    /// or a full bitmask for multi-bit values.
    pub flag_bit: Option<u32>,
}

impl MmFlagMapping {
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
        flag_type: MmFlagType,
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
    pub const fn global(location_id: &'static str, flag_type: MmFlagType, flag_bit: u32) -> Self {
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
