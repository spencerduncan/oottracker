//! Core enums and types for MM save data.

use {derive_more::From, std::num::TryFromIntError};

// ============================================================================
// Decode Error Type
// ============================================================================

/// Errors that can occur when decoding MM save data from raw bytes
#[derive(Debug, From, Clone)]
pub enum MmDecodeError {
    /// A single byte assertion failed
    AssertEq {
        offset: u16,
        expected: u8,
        found: u8,
    },
    /// A range assertion failed
    AssertEqRange {
        start: u16,
        end: u16,
        expected: Vec<u8>,
        found: Vec<u8>,
    },
    /// Index out of bounds
    Index(u16),
    /// Range out of bounds
    IndexRange { start: u16, end: u16 },
    /// Save data is wrong size
    Size(usize),
    /// Unexpected value at offset
    UnexpectedValue {
        offset: u16,
        field: &'static str,
        value: u8,
    },
    /// Unexpected value in range
    UnexpectedValueRange {
        start: u16,
        end: u16,
        field: &'static str,
        value: Vec<u8>,
    },
    /// Integer conversion error
    #[from]
    TryFromInt(TryFromIntError),
}

// ============================================================================
// Player Form
// ============================================================================

/// Player transformation forms in MM
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum PlayerForm {
    FierceDeity = 0,
    Goron = 1,
    Zora = 2,
    Deku = 3,
    #[default]
    Human = 4,
}

impl TryFrom<u8> for PlayerForm {
    type Error = u8;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(PlayerForm::FierceDeity),
            1 => Ok(PlayerForm::Goron),
            2 => Ok(PlayerForm::Zora),
            3 => Ok(PlayerForm::Deku),
            4 => Ok(PlayerForm::Human),
            _ => Err(value),
        }
    }
}

// ============================================================================
// Magic Capacity
// ============================================================================

/// Magic capacity levels
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum MmMagicCapacity {
    #[default]
    None = 0,
    Single = 1,
    Double = 2,
}

impl TryFrom<u8> for MmMagicCapacity {
    type Error = u8;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(MmMagicCapacity::None),
            1 => Ok(MmMagicCapacity::Single),
            2 => Ok(MmMagicCapacity::Double),
            _ => Err(value),
        }
    }
}

// ============================================================================
// Sword and Shield
// ============================================================================

/// MM sword levels
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum MmSword {
    #[default]
    None = 0,
    KokiriSword = 1,
    RazorSword = 2,
    GildedSword = 3,
}

impl TryFrom<u8> for MmSword {
    type Error = u8;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(MmSword::None),
            1 => Ok(MmSword::KokiriSword),
            2 => Ok(MmSword::RazorSword),
            3 => Ok(MmSword::GildedSword),
            _ => Err(value),
        }
    }
}

/// MM shield types
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum MmShield {
    #[default]
    None = 0,
    /// Deku Shield (OoT) / Hero's Shield (MM) - value 1
    HeroShield = 1,
    /// Hylian Shield (OoT only) - value 2
    HylianShield = 2,
    /// Mirror Shield - value 3
    MirrorShield = 3,
}

impl TryFrom<u8> for MmShield {
    type Error = u8;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(MmShield::None),
            1 => Ok(MmShield::HeroShield),
            2 => Ok(MmShield::HylianShield),
            3 => Ok(MmShield::MirrorShield),
            _ => Err(value),
        }
    }
}
