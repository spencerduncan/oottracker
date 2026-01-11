//! Mask ownership tracking.

use bitflags::bitflags;

bitflags! {
    /// Transformation masks
    #[derive(Default)]
    pub struct MmTransformationMasks: u8 {
        const DEKU = 0x01;
        const GORON = 0x02;
        const ZORA = 0x04;
        const FIERCE_DEITY = 0x08;
    }
}

bitflags! {
    /// Regular collectible masks (first 16)
    #[derive(Default)]
    pub struct MmMasksLow: u16 {
        const POSTMAN = 1 << 0;
        const ALL_NIGHT = 1 << 1;
        const BLAST = 1 << 2;
        const STONE = 1 << 3;
        const GREAT_FAIRY = 1 << 4;
        const KEATON = 1 << 5;
        const BREMEN = 1 << 6;
        const BUNNY = 1 << 7;
        const DON_GERO = 1 << 8;
        const SCENTS = 1 << 9;
        const ROMANI = 1 << 10;
        const CIRCUS_LEADER = 1 << 11;  // Troupe Leader's Mask
        const KAFEI = 1 << 12;
        const COUPLES = 1 << 13;
        const TRUTH = 1 << 14;
        const KAMARO = 1 << 15;
    }
}

bitflags! {
    /// Regular collectible masks (last 8)
    #[derive(Default)]
    pub struct MmMasksHigh: u8 {
        const GIBDO = 1 << 0;
        const GARO = 1 << 1;
        const CAPTAIN = 1 << 2;
        const GIANT = 1 << 3;
    }
}

/// All mask ownership state
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct MmMasks {
    pub transformation: MmTransformationMasks,
    pub masks_low: MmMasksLow,
    pub masks_high: MmMasksHigh,
}

impl MmMasks {
    /// Total number of masks collected (excluding transformation masks)
    pub fn regular_mask_count(&self) -> u8 {
        self.masks_low.bits().count_ones() as u8 + self.masks_high.bits().count_ones() as u8
    }

    /// Total masks including transformation masks
    pub fn total_mask_count(&self) -> u8 {
        self.regular_mask_count() + self.transformation.bits().count_ones() as u8
    }
}
