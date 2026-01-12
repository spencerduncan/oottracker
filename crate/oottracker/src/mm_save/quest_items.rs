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

    /// Convert from OoTMM quest item bit layout to our internal format.
    ///
    /// OoTMM uses a different bit layout than vanilla MM:
    /// - Heart pieces: bits 0-3 (same as our format but at different position)
    /// - songLullabyIntro: bit 7
    /// - notebook: bit 13
    /// - songSun: bit 14, songStorms: bit 15, songSoaring: bit 16
    /// - songEpona: bit 17, songHealing: bit 18, songTime: bit 19
    /// - songSaria: bit 20, songOrder: bit 21, songEmpty: bit 22
    /// - songNewWave: bit 23, songLullaby: bit 24, songAwakening: bit 25
    /// - remainsTwinmold: bit 28, remainsGyorg: bit 29
    /// - remainsGoht: bit 30, remainsOdolwa: bit 31
    pub fn from_ootmm_bits(ootmm_bits: u32) -> Self {
        let mut result = MmQuestItems::empty();

        // Convert boss remains (OoTMM bits 28-31 -> our bits 0-3)
        if ootmm_bits & (1 << 31) != 0 {
            result.insert(Self::REMAINS_ODOLWA);
        }
        if ootmm_bits & (1 << 30) != 0 {
            result.insert(Self::REMAINS_GOHT);
        }
        if ootmm_bits & (1 << 29) != 0 {
            result.insert(Self::REMAINS_GYORG);
        }
        if ootmm_bits & (1 << 28) != 0 {
            result.insert(Self::REMAINS_TWINMOLD);
        }

        // Convert songs (OoTMM -> our format)
        if ootmm_bits & (1 << 25) != 0 {
            result.insert(Self::SONG_AWAKENING);
        } // Sonata of Awakening
        if ootmm_bits & (1 << 24) != 0 {
            result.insert(Self::SONG_GORON);
        } // Goron Lullaby
        if ootmm_bits & (1 << 23) != 0 {
            result.insert(Self::SONG_ZORA);
        } // New Wave Bossa Nova
        if ootmm_bits & (1 << 22) != 0 {
            result.insert(Self::SONG_EMPTINESS);
        } // Elegy of Emptiness
        if ootmm_bits & (1 << 21) != 0 {
            result.insert(Self::SONG_ORDER);
        } // Oath to Order
        if ootmm_bits & (1 << 20) != 0 {
            result.insert(Self::SONG_SARIA);
        } // Saria's Song
        if ootmm_bits & (1 << 19) != 0 {
            result.insert(Self::SONG_TIME);
        } // Song of Time
        if ootmm_bits & (1 << 18) != 0 {
            result.insert(Self::SONG_HEALING);
        } // Song of Healing
        if ootmm_bits & (1 << 17) != 0 {
            result.insert(Self::SONG_EPONA);
        } // Epona's Song
        if ootmm_bits & (1 << 16) != 0 {
            result.insert(Self::SONG_SOARING);
        } // Song of Soaring
        if ootmm_bits & (1 << 15) != 0 {
            result.insert(Self::SONG_STORMS);
        } // Song of Storms
        if ootmm_bits & (1 << 14) != 0 {
            result.insert(Self::SONG_SUN);
        } // Sun's Song

        // Notebook (OoTMM bit 13 -> our bit 18)
        if ootmm_bits & (1 << 13) != 0 {
            result.insert(Self::NOTEBOOK);
        }

        // Lullaby intro (OoTMM bit 7 -> our bit 24)
        if ootmm_bits & (1 << 7) != 0 {
            result.insert(Self::SONG_LULLABY_INTRO);
        }

        // Heart pieces (OoTMM bits 0-3 -> our bits 28-31)
        let heart_pieces = ootmm_bits & 0xF;
        result |= MmQuestItems::from_bits_truncate(heart_pieces << 28);

        result
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
