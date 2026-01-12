//! Quest items including boss remains and songs.

use bitflags::bitflags;

bitflags! {
    /// Quest items including boss remains and songs
    #[derive(Default)]
    pub struct MmQuestItems: u32 {
        // Boss Remains (bits 0-3)
        const REMAINS_ODOLWA = 1 << 0;
        const REMAINS_GOHT = 1 << 1;
        const REMAINS_GYORG = 1 << 2;
        const REMAINS_TWINMOLD = 1 << 3;

        // Songs (bits 6-17)
        const SONG_AWAKENING = 1 << 6;    // Sonata of Awakening
        const SONG_GORON = 1 << 7;        // Goron Lullaby
        const SONG_ZORA = 1 << 8;         // New Wave Bossa Nova
        const SONG_EMPTINESS = 1 << 9;    // Elegy of Emptiness
        const SONG_ORDER = 1 << 10;       // Oath to Order
        const SONG_SARIA = 1 << 11;       // Saria's Song (unused in MM)
        const SONG_TIME = 1 << 12;        // Song of Time
        const SONG_HEALING = 1 << 13;     // Song of Healing
        const SONG_EPONA = 1 << 14;       // Epona's Song
        const SONG_SOARING = 1 << 15;     // Song of Soaring
        const SONG_STORMS = 1 << 16;      // Song of Storms
        const SONG_SUN = 1 << 17;         // Sun's Song (unused in MM)

        // Notebook (bit 18)
        const NOTEBOOK = 1 << 18;

        // Lullaby intro (bit 24)
        const SONG_LULLABY_INTRO = 1 << 24;

        // Heart pieces counter (bits 28-31)
        const HEART_PIECE_1 = 1 << 28;
        const HEART_PIECE_2 = 1 << 29;
        const HEART_PIECE_3 = 1 << 30;
        const HEART_PIECE_4 = 1 << 31;
    }
}

impl MmQuestItems {
    /// Returns the number of boss remains collected
    pub fn num_remains(&self) -> u8 {
        let mut count = 0;
        if self.contains(Self::REMAINS_ODOLWA) {
            count += 1;
        }
        if self.contains(Self::REMAINS_GOHT) {
            count += 1;
        }
        if self.contains(Self::REMAINS_GYORG) {
            count += 1;
        }
        if self.contains(Self::REMAINS_TWINMOLD) {
            count += 1;
        }
        count
    }

    /// Returns the heart piece count (0-3)
    pub fn heart_pieces(&self) -> u8 {
        ((self.bits() >> 28) & 0xF) as u8
    }

    /// Convert from OoTMM quest item bits to our internal format.
    ///
    /// OoTMM uses the same bit layout as vanilla MM:
    /// - Remains: bits 0-3 (Odolwa=0, Goht=1, Gyorg=2, Twinmold=3)
    /// - Songs: bits 6-17
    /// - Notebook: bit 18
    /// - Lullaby intro: bit 24
    /// - Heart pieces: bits 28-31
    pub fn from_ootmm_bits(ootmm_bits: u32) -> Self {
        Self::from_bits_truncate(ootmm_bits)
    }

    /// Convert our internal format to OoTMM quest item bit layout.
    ///
    /// This is the inverse of `from_ootmm_bits`.
    pub fn to_ootmm_bits(&self) -> u32 {
        let mut result = 0u32;

        // Convert boss remains (our bits 0-3 -> OoTMM bits 28-31)
        if self.contains(Self::REMAINS_ODOLWA) {
            result |= 1 << 31;
        }
        if self.contains(Self::REMAINS_GOHT) {
            result |= 1 << 30;
        }
        if self.contains(Self::REMAINS_GYORG) {
            result |= 1 << 29;
        }
        if self.contains(Self::REMAINS_TWINMOLD) {
            result |= 1 << 28;
        }

        // Convert songs (our format -> OoTMM)
        if self.contains(Self::SONG_AWAKENING) {
            result |= 1 << 25;
        }
        if self.contains(Self::SONG_GORON) {
            result |= 1 << 24;
        }
        if self.contains(Self::SONG_ZORA) {
            result |= 1 << 23;
        }
        if self.contains(Self::SONG_EMPTINESS) {
            result |= 1 << 22;
        }
        if self.contains(Self::SONG_ORDER) {
            result |= 1 << 21;
        }
        if self.contains(Self::SONG_SARIA) {
            result |= 1 << 20;
        }
        if self.contains(Self::SONG_TIME) {
            result |= 1 << 19;
        }
        if self.contains(Self::SONG_HEALING) {
            result |= 1 << 18;
        }
        if self.contains(Self::SONG_EPONA) {
            result |= 1 << 17;
        }
        if self.contains(Self::SONG_SOARING) {
            result |= 1 << 16;
        }
        if self.contains(Self::SONG_STORMS) {
            result |= 1 << 15;
        }
        if self.contains(Self::SONG_SUN) {
            result |= 1 << 14;
        }

        // Notebook (our bit 18 -> OoTMM bit 13)
        if self.contains(Self::NOTEBOOK) {
            result |= 1 << 13;
        }

        // Lullaby intro (our bit 24 -> OoTMM bit 7)
        if self.contains(Self::SONG_LULLABY_INTRO) {
            result |= 1 << 7;
        }

        // Heart pieces (our bits 28-31 -> OoTMM bits 0-3)
        let heart_pieces = (self.bits() >> 28) & 0xF;
        result |= heart_pieces;

        result
    }
}
