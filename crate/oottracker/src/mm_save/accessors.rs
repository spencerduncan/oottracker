//! Accessor methods for MmSave (has_* methods).

use crate::mm_save::{
    masks::{MmMasksHigh, MmMasksLow, MmTransformationMasks},
    quest_items::MmQuestItems,
    save::MmSave,
    types::MmMagicCapacity,
};

impl MmSave {
    // ========================================================================
    // Transformation Mask Accessors
    // ========================================================================

    /// Returns true if the player has the Deku Mask
    pub fn has_deku_mask(&self) -> bool {
        self.masks
            .transformation
            .contains(MmTransformationMasks::DEKU)
    }

    /// Returns true if the player has the Goron Mask
    pub fn has_goron_mask(&self) -> bool {
        self.masks
            .transformation
            .contains(MmTransformationMasks::GORON)
    }

    /// Returns true if the player has the Zora Mask
    pub fn has_zora_mask(&self) -> bool {
        self.masks
            .transformation
            .contains(MmTransformationMasks::ZORA)
    }

    /// Returns true if the player has the Fierce Deity Mask
    pub fn has_fierce_deity_mask(&self) -> bool {
        self.masks
            .transformation
            .contains(MmTransformationMasks::FIERCE_DEITY)
    }

    // ========================================================================
    // Collectible Mask Accessors
    // ========================================================================

    /// Returns true if the player has the Postman's Hat
    pub fn has_postman_hat(&self) -> bool {
        self.masks.masks_low.contains(MmMasksLow::POSTMAN)
    }

    /// Returns true if the player has the All-Night Mask
    pub fn has_all_night_mask(&self) -> bool {
        self.masks.masks_low.contains(MmMasksLow::ALL_NIGHT)
    }

    /// Returns true if the player has the Blast Mask
    pub fn has_blast_mask(&self) -> bool {
        self.masks.masks_low.contains(MmMasksLow::BLAST)
    }

    /// Returns true if the player has the Stone Mask
    pub fn has_stone_mask(&self) -> bool {
        self.masks.masks_low.contains(MmMasksLow::STONE)
    }

    /// Returns true if the player has the Great Fairy Mask
    pub fn has_great_fairy_mask(&self) -> bool {
        self.masks.masks_low.contains(MmMasksLow::GREAT_FAIRY)
    }

    /// Returns true if the player has the Keaton Mask
    pub fn has_keaton_mask(&self) -> bool {
        self.masks.masks_low.contains(MmMasksLow::KEATON)
    }

    /// Returns true if the player has the Bremen Mask
    pub fn has_bremen_mask(&self) -> bool {
        self.masks.masks_low.contains(MmMasksLow::BREMEN)
    }

    /// Returns true if the player has the Bunny Hood
    pub fn has_bunny_hood(&self) -> bool {
        self.masks.masks_low.contains(MmMasksLow::BUNNY)
    }

    /// Returns true if the player has Don Gero's Mask
    pub fn has_don_gero_mask(&self) -> bool {
        self.masks.masks_low.contains(MmMasksLow::DON_GERO)
    }

    /// Returns true if the player has the Mask of Scents
    pub fn has_mask_of_scents(&self) -> bool {
        self.masks.masks_low.contains(MmMasksLow::SCENTS)
    }

    /// Returns true if the player has Romani's Mask
    pub fn has_romani_mask(&self) -> bool {
        self.masks.masks_low.contains(MmMasksLow::ROMANI)
    }

    /// Returns true if the player has the Circus Leader's Mask (Troupe Leader's Mask)
    pub fn has_circus_leader_mask(&self) -> bool {
        self.masks.masks_low.contains(MmMasksLow::CIRCUS_LEADER)
    }

    /// Returns true if the player has Kafei's Mask
    pub fn has_kafei_mask(&self) -> bool {
        self.masks.masks_low.contains(MmMasksLow::KAFEI)
    }

    /// Returns true if the player has the Couple's Mask
    pub fn has_couples_mask(&self) -> bool {
        self.masks.masks_low.contains(MmMasksLow::COUPLES)
    }

    /// Returns true if the player has the Mask of Truth
    pub fn has_mask_of_truth(&self) -> bool {
        self.masks.masks_low.contains(MmMasksLow::TRUTH)
    }

    /// Returns true if the player has Kamaro's Mask
    pub fn has_kamaro_mask(&self) -> bool {
        self.masks.masks_low.contains(MmMasksLow::KAMARO)
    }

    /// Returns true if the player has the Gibdo Mask
    pub fn has_gibdo_mask(&self) -> bool {
        self.masks.masks_high.contains(MmMasksHigh::GIBDO)
    }

    /// Returns true if the player has the Garo's Mask
    pub fn has_garo_mask(&self) -> bool {
        self.masks.masks_high.contains(MmMasksHigh::GARO)
    }

    /// Returns true if the player has the Captain's Hat
    pub fn has_captain_hat(&self) -> bool {
        self.masks.masks_high.contains(MmMasksHigh::CAPTAIN)
    }

    /// Returns true if the player has the Giant's Mask
    pub fn has_giant_mask(&self) -> bool {
        self.masks.masks_high.contains(MmMasksHigh::GIANT)
    }

    // ========================================================================
    // Equipment Accessor Methods
    // ========================================================================

    /// Check if player has the Ocarina of Time
    pub fn has_ocarina(&self) -> bool {
        self.inventory.ocarina
    }

    /// Check if player has the Hero's Bow
    pub fn has_heros_bow(&self) -> bool {
        self.inventory.bow
    }

    /// Check if player has Fire Arrows
    pub fn has_fire_arrow(&self) -> bool {
        self.inventory.fire_arrows
    }

    /// Check if player has Ice Arrows
    pub fn has_ice_arrow(&self) -> bool {
        self.inventory.ice_arrows
    }

    /// Check if player has Light Arrows
    pub fn has_light_arrow(&self) -> bool {
        self.inventory.light_arrows
    }

    /// Check if player has the Hookshot
    pub fn has_hookshot(&self) -> bool {
        self.inventory.hookshot
    }

    /// Check if player has Bombs
    pub fn has_bombs(&self) -> bool {
        self.inventory.bombs
    }

    /// Check if player has Bombchus
    pub fn has_bombchu(&self) -> bool {
        self.inventory.bombchus
    }

    /// Check if player has Powder Kegs
    pub fn has_powder_keg(&self) -> bool {
        self.inventory.powder_keg
    }

    /// Check if player has the Lens of Truth
    pub fn has_lens_of_truth(&self) -> bool {
        self.inventory.lens
    }

    /// Check if player has the Pictograph Box
    pub fn has_pictograph_box(&self) -> bool {
        self.inventory.pictograph_box
    }

    /// Check if player has the Great Fairy's Sword
    pub fn has_great_fairy_sword(&self) -> bool {
        self.inventory.great_fairy_sword
    }

    /// Check if player has Magic Beans
    pub fn has_magic_bean(&self) -> bool {
        self.inventory.magic_beans
    }

    /// Check if player has magic (single or double)
    pub fn has_magic(&self) -> bool {
        self.magic != MmMagicCapacity::None
    }

    // ========================================================================
    // Song Accessor Methods
    // ========================================================================

    /// Check if player has Song of Time
    pub fn has_song_of_time(&self) -> bool {
        self.quest_items.contains(MmQuestItems::SONG_TIME)
    }

    /// Check if player has Song of Healing
    pub fn has_song_of_healing(&self) -> bool {
        self.quest_items.contains(MmQuestItems::SONG_HEALING)
    }

    /// Check if player has Epona's Song
    pub fn has_eponas_song(&self) -> bool {
        self.quest_items.contains(MmQuestItems::SONG_EPONA)
    }

    /// Check if player has Song of Soaring
    pub fn has_song_of_soaring(&self) -> bool {
        self.quest_items.contains(MmQuestItems::SONG_SOARING)
    }

    /// Check if player has Song of Storms
    pub fn has_song_of_storms(&self) -> bool {
        self.quest_items.contains(MmQuestItems::SONG_STORMS)
    }

    /// Check if player has Sonata of Awakening
    pub fn has_sonata_of_awakening(&self) -> bool {
        self.quest_items.contains(MmQuestItems::SONG_AWAKENING)
    }

    /// Check if player has Goron Lullaby
    pub fn has_goron_lullaby(&self) -> bool {
        self.quest_items.contains(MmQuestItems::SONG_GORON)
    }

    /// Check if player has New Wave Bossa Nova
    pub fn has_new_wave_bossa_nova(&self) -> bool {
        self.quest_items.contains(MmQuestItems::SONG_ZORA)
    }

    /// Check if player has Elegy of Emptiness
    pub fn has_elegy_of_emptiness(&self) -> bool {
        self.quest_items.contains(MmQuestItems::SONG_EMPTINESS)
    }

    /// Check if player has Oath to Order
    pub fn has_oath_to_order(&self) -> bool {
        self.quest_items.contains(MmQuestItems::SONG_ORDER)
    }

    // ========================================================================
    // Boss Remains Accessor Methods
    // ========================================================================

    /// Check if player has Odolwa's Remains
    pub fn has_odolwa_remains(&self) -> bool {
        self.quest_items.contains(MmQuestItems::REMAINS_ODOLWA)
    }

    /// Check if player has Goht's Remains
    pub fn has_goht_remains(&self) -> bool {
        self.quest_items.contains(MmQuestItems::REMAINS_GOHT)
    }

    /// Check if player has Gyorg's Remains
    pub fn has_gyorg_remains(&self) -> bool {
        self.quest_items.contains(MmQuestItems::REMAINS_GYORG)
    }

    /// Check if player has Twinmold's Remains
    pub fn has_twinmold_remains(&self) -> bool {
        self.quest_items.contains(MmQuestItems::REMAINS_TWINMOLD)
    }

    // ========================================================================
    // Bomber's Notebook Accessor Methods
    // ========================================================================

    /// Check if player has the Bomber's Notebook
    pub fn has_bombers_notebook(&self) -> bool {
        self.quest_items.contains(MmQuestItems::NOTEBOOK)
    }
}
