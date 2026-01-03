//! Game context implementations for logic evaluation.
//!
//! This module provides game-specific implementations of the `EvalContext` trait
//! from the `ootmm` crate, allowing logic expressions to be evaluated against
//! actual game state.

use std::collections::{HashMap, HashSet};

use ootmm::expr::EvalContext;

use crate::mm_save::{MmBottle, MmMagicCapacity, MmSave, MmSword};

/// Context for evaluating logic expressions against Majora's Mask game state.
///
/// This struct implements the `EvalContext` trait from `ootmm::expr`, allowing
/// logic expressions to be evaluated against actual MM save data.
///
/// # Example
///
/// ```ignore
/// use oottracker::logic_context::MmGameContext;
/// use oottracker::mm_save::MmSave;
/// use ootmm::expr::eval_str;
///
/// let save = MmSave::default();
/// let ctx = MmGameContext::new(&save);
/// let result = eval_str("has(HOOKSHOT)", &ctx);
/// ```
pub struct MmGameContext<'a> {
    /// Reference to the MM save data
    save: &'a MmSave,
    /// Events that have occurred in the game
    events: HashSet<String>,
    /// Randomizer settings (name -> enabled)
    settings: HashMap<String, bool>,
    /// Enabled tricks for logic evaluation
    tricks: HashSet<String>,
}

impl<'a> MmGameContext<'a> {
    /// Create a new game context from MM save data.
    ///
    /// Creates a context with empty events, settings, and tricks.
    /// Use the builder methods to add these as needed.
    pub fn new(save: &'a MmSave) -> Self {
        Self {
            save,
            events: HashSet::new(),
            settings: HashMap::new(),
            tricks: HashSet::new(),
        }
    }

    /// Add an event to the context.
    pub fn with_event(mut self, event: &str) -> Self {
        self.events.insert(event.to_string());
        self
    }

    /// Add multiple events to the context.
    pub fn with_events(mut self, events: impl IntoIterator<Item = impl AsRef<str>>) -> Self {
        for event in events {
            self.events.insert(event.as_ref().to_string());
        }
        self
    }

    /// Set a setting value.
    pub fn with_setting(mut self, name: &str, value: bool) -> Self {
        self.settings.insert(name.to_string(), value);
        self
    }

    /// Add multiple settings to the context.
    pub fn with_settings(
        mut self,
        settings: impl IntoIterator<Item = (impl AsRef<str>, bool)>,
    ) -> Self {
        for (name, value) in settings {
            self.settings.insert(name.as_ref().to_string(), value);
        }
        self
    }

    /// Enable a trick.
    pub fn with_trick(mut self, trick: &str) -> Self {
        self.tricks.insert(trick.to_string());
        self
    }

    /// Enable multiple tricks.
    pub fn with_tricks(mut self, tricks: impl IntoIterator<Item = impl AsRef<str>>) -> Self {
        for trick in tricks {
            self.tricks.insert(trick.as_ref().to_string());
        }
        self
    }

    /// Get a reference to the underlying save data.
    pub fn save(&self) -> &MmSave {
        self.save
    }

    /// Check if the player has an item by name.
    ///
    /// This is the core item lookup logic that maps item names to save data.
    /// Item names are case-insensitive and support various aliases.
    fn check_item(&self, item: &str, count: u32) -> bool {
        let item_upper = item.to_uppercase();

        // For most items, count > 1 doesn't make sense (they're boolean)
        // but we support it for bottles and other countable items
        match item_upper.as_str() {
            // Inventory items
            "OCARINA" | "OCARINA_OF_TIME" => self.save.has_ocarina(),
            "BOW" | "HEROS_BOW" | "HERO_BOW" => self.save.has_heros_bow(),
            "FIRE_ARROW" | "FIRE_ARROWS" => self.save.has_fire_arrow(),
            "ICE_ARROW" | "ICE_ARROWS" => self.save.has_ice_arrow(),
            "LIGHT_ARROW" | "LIGHT_ARROWS" => self.save.has_light_arrow(),
            "HOOKSHOT" => self.save.has_hookshot(),
            "BOMBS" | "BOMB" => self.save.has_bombs(),
            "BOMBCHU" | "BOMBCHUS" => self.save.has_bombchu(),
            "DEKU_STICK" | "DEKU_STICKS" => self.save.inventory.deku_sticks,
            "DEKU_NUT" | "DEKU_NUTS" => self.save.inventory.deku_nuts,
            "MAGIC_BEAN" | "MAGIC_BEANS" => self.save.has_magic_bean(),
            "POWDER_KEG" | "POWDER_KEGS" => self.save.has_powder_keg(),
            "PICTOGRAPH_BOX" | "PICTOGRAPH" => self.save.has_pictograph_box(),
            "LENS" | "LENS_OF_TRUTH" => self.save.has_lens_of_truth(),
            "GREAT_FAIRY_SWORD" | "GREAT_FAIRYS_SWORD" => self.save.has_great_fairy_sword(),

            // Swords
            "KOKIRI_SWORD" => self.save.sword != MmSword::None,
            "RAZOR_SWORD" => matches!(self.save.sword, MmSword::RazorSword | MmSword::GildedSword),
            "GILDED_SWORD" => self.save.sword == MmSword::GildedSword,
            "SWORD" => self.save.sword != MmSword::None,

            // Shields
            "HERO_SHIELD" | "HEROS_SHIELD" => self.save.shield != crate::mm_save::MmShield::None,
            "MIRROR_SHIELD" => self.save.shield == crate::mm_save::MmShield::MirrorShield,
            "SHIELD" => self.save.shield != crate::mm_save::MmShield::None,

            // Transformation masks
            "DEKU_MASK" => self.save.has_deku_mask(),
            "GORON_MASK" => self.save.has_goron_mask(),
            "ZORA_MASK" => self.save.has_zora_mask(),
            "FIERCE_DEITY_MASK" | "FIERCE_DEITY" => self.save.has_fierce_deity_mask(),

            // Collectible masks
            "POSTMAN_HAT" | "POSTMANS_HAT" => self.save.has_postman_hat(),
            "ALL_NIGHT_MASK" => self.save.has_all_night_mask(),
            "BLAST_MASK" => self.save.has_blast_mask(),
            "STONE_MASK" => self.save.has_stone_mask(),
            "GREAT_FAIRY_MASK" => self.save.has_great_fairy_mask(),
            "KEATON_MASK" => self.save.has_keaton_mask(),
            "BREMEN_MASK" => self.save.has_bremen_mask(),
            "BUNNY_HOOD" => self.save.has_bunny_hood(),
            "DON_GERO_MASK" | "DON_GEROS_MASK" => self.save.has_don_gero_mask(),
            "MASK_OF_SCENTS" | "SCENTS_MASK" => self.save.has_mask_of_scents(),
            "ROMANI_MASK" | "ROMANIS_MASK" => self.save.has_romani_mask(),
            "CIRCUS_LEADER_MASK" | "TROUPE_LEADER_MASK" => self.save.has_circus_leader_mask(),
            "KAFEI_MASK" | "KAFEIS_MASK" => self.save.has_kafei_mask(),
            "COUPLES_MASK" | "COUPLE_MASK" => self.save.has_couples_mask(),
            "MASK_OF_TRUTH" | "TRUTH_MASK" => self.save.has_mask_of_truth(),
            "KAMARO_MASK" | "KAMAROS_MASK" => self.save.has_kamaro_mask(),
            "GIBDO_MASK" => self.save.has_gibdo_mask(),
            "GARO_MASK" | "GAROS_MASK" => self.save.has_garo_mask(),
            "CAPTAIN_HAT" | "CAPTAINS_HAT" => self.save.has_captain_hat(),
            "GIANT_MASK" | "GIANTS_MASK" => self.save.has_giant_mask(),

            // Songs
            "SONG_OF_TIME" => self.save.has_song_of_time(),
            "SONG_OF_HEALING" => self.save.has_song_of_healing(),
            "EPONAS_SONG" | "EPONA_SONG" => self.save.has_eponas_song(),
            "SONG_OF_SOARING" => self.save.has_song_of_soaring(),
            "SONG_OF_STORMS" => self.save.has_song_of_storms(),
            "SONATA_OF_AWAKENING" => self.save.has_sonata_of_awakening(),
            "GORON_LULLABY" => self.save.has_goron_lullaby(),
            "NEW_WAVE_BOSSA_NOVA" => self.save.has_new_wave_bossa_nova(),
            "ELEGY_OF_EMPTINESS" => self.save.has_elegy_of_emptiness(),
            "OATH_TO_ORDER" => self.save.has_oath_to_order(),

            // Boss remains
            "ODOLWA_REMAINS" | "ODOLWAS_REMAINS" => self.save.has_odolwa_remains(),
            "GOHT_REMAINS" | "GOHTS_REMAINS" => self.save.has_goht_remains(),
            "GYORG_REMAINS" | "GYORGS_REMAINS" => self.save.has_gyorg_remains(),
            "TWINMOLD_REMAINS" | "TWINMOLDS_REMAINS" => self.save.has_twinmold_remains(),

            // Bottles - support counting
            "BOTTLE" | "EMPTY_BOTTLE" => self.count_bottles() >= count,

            // Magic
            "MAGIC" => self.save.has_magic(),
            "DOUBLE_MAGIC" => self.save.magic == MmMagicCapacity::Double,

            // Double defense
            "DOUBLE_DEFENSE" => self.save.double_defense,

            // Small keys (support counting)
            "SMALL_KEY_WOODFALL" | "WOODFALL_SMALL_KEY" => {
                self.save.small_keys.woodfall as u32 >= count
            }
            "SMALL_KEY_SNOWHEAD" | "SNOWHEAD_SMALL_KEY" => {
                self.save.small_keys.snowhead as u32 >= count
            }
            "SMALL_KEY_GREAT_BAY" | "GREAT_BAY_SMALL_KEY" => {
                self.save.small_keys.great_bay as u32 >= count
            }
            "SMALL_KEY_STONE_TOWER" | "STONE_TOWER_SMALL_KEY" => {
                self.save.small_keys.stone_tower as u32 >= count
            }

            // Boss keys
            "BOSS_KEY_WOODFALL" | "WOODFALL_BOSS_KEY" => self
                .save
                .dungeon_items
                .woodfall
                .contains(crate::mm_save::MmDungeonItems::BOSS_KEY),
            "BOSS_KEY_SNOWHEAD" | "SNOWHEAD_BOSS_KEY" => self
                .save
                .dungeon_items
                .snowhead
                .contains(crate::mm_save::MmDungeonItems::BOSS_KEY),
            "BOSS_KEY_GREAT_BAY" | "GREAT_BAY_BOSS_KEY" => self
                .save
                .dungeon_items
                .great_bay
                .contains(crate::mm_save::MmDungeonItems::BOSS_KEY),
            "BOSS_KEY_STONE_TOWER" | "STONE_TOWER_BOSS_KEY" => self
                .save
                .dungeon_items
                .stone_tower
                .contains(crate::mm_save::MmDungeonItems::BOSS_KEY),

            // Stray fairies (support counting)
            "STRAY_FAIRY_CLOCK_TOWN" => self.save.stray_fairies.clock_town as u32 >= count,
            "STRAY_FAIRY_WOODFALL" => self.save.stray_fairies.woodfall as u32 >= count,
            "STRAY_FAIRY_SNOWHEAD" => self.save.stray_fairies.snowhead as u32 >= count,
            "STRAY_FAIRY_GREAT_BAY" => self.save.stray_fairies.great_bay as u32 >= count,
            "STRAY_FAIRY_STONE_TOWER" => self.save.stray_fairies.stone_tower as u32 >= count,

            // Skulltula tokens (support counting)
            "SKULL_TOKEN_SWAMP" | "SWAMP_SKULLTULA_TOKEN" => {
                self.save.skull_tokens_swamp as u32 >= count
            }
            "SKULL_TOKEN_OCEAN" | "OCEAN_SKULLTULA_TOKEN" => {
                self.save.skull_tokens_ocean as u32 >= count
            }

            // Unknown item
            _ => false,
        }
    }

    /// Count the number of bottles the player has.
    fn count_bottles(&self) -> u32 {
        self.save
            .inventory
            .bottles
            .iter()
            .filter(|&&b| b != MmBottle::None)
            .count() as u32
    }

    /// Calculate MM time in minutes since Day 1 at 6:00 AM.
    ///
    /// The MM time system:
    /// - Each day is 24 in-game hours (1440 minutes)
    /// - Day 1 starts at 6:00 AM (time = 0x4000 in game units)
    /// - Game time unit: 0x10000 = 24 hours, so 0x4000 = 6:00 AM
    /// - Full cycle: 0-4319 minutes (72 hours = 4320 minutes)
    fn calculate_mm_time(&self) -> u32 {
        // Convert game time units to minutes since 6:00 AM
        // Game time: 0x0000-0xFFFF maps to 00:00-24:00
        // 0x4000 = 6:00 AM, 0xC000 = 6:00 PM
        let time_units = self.save.time as u32;

        // Convert to minutes within the current day (0-1439)
        // 0x10000 time units = 1440 minutes
        // Each time unit = 1440 / 65536 minutes
        let minutes_since_midnight = (time_units * 1440) / 0x10000;

        // Adjust for 6 AM start (game day starts at 6 AM)
        // If time is before 6 AM (< 360 minutes), it's actually the previous calendar day
        let minutes_since_6am = if minutes_since_midnight >= 360 {
            minutes_since_midnight - 360
        } else {
            // Before 6 AM: this is 18:00 (6 PM) to 24:00 (midnight) + midnight to 6 AM
            // = 1080 (6 PM to midnight) + minutes_since_midnight
            1080 + minutes_since_midnight
        };

        // Day is 1-indexed in the game, we want 0-indexed for calculation
        let day_offset = self.save.day.saturating_sub(1);

        // Total time: (day * 1440) + time within day
        // Clamp to valid range (0-4319)
        let total = (day_offset * 1440) + minutes_since_6am;
        total.min(4319)
    }
}

impl EvalContext for MmGameContext<'_> {
    fn has_item(&self, item: &str, count: u32) -> bool {
        self.check_item(item, count)
    }

    fn event(&self, name: &str) -> bool {
        self.events.contains(name)
    }

    fn setting(&self, name: &str) -> Option<bool> {
        self.settings.get(name).copied()
    }

    fn trick(&self, name: &str) -> bool {
        self.tricks.contains(name)
    }

    fn is_adult(&self) -> bool {
        // In MM, Link is always the same age (young Link equivalent)
        // For logic purposes, we return false for is_adult
        false
    }

    fn is_child(&self) -> bool {
        // In MM, Link is always the same age (young Link equivalent)
        // For logic purposes, we return true for is_child
        true
    }

    fn mm_time(&self) -> u32 {
        self.calculate_mm_time()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mm_save::{
        MmAllDungeonItems, MmBottle, MmDungeonItems, MmInventory, MmMagicCapacity, MmMasks,
        MmMasksHigh, MmMasksLow, MmQuestItems, MmShield, MmSmallKeys, MmStrayFairies, MmSword,
        MmTransformationMasks, PlayerForm,
    };

    /// Create a default save for testing
    fn default_save() -> MmSave {
        MmSave::default()
    }

    /// Create a save with specific inventory items
    fn save_with_inventory(inventory: MmInventory) -> MmSave {
        MmSave {
            inventory,
            ..Default::default()
        }
    }

    // ========================================================================
    // Basic Context Tests
    // ========================================================================

    #[test]
    fn test_context_creation() {
        let save = default_save();
        let ctx = MmGameContext::new(&save);
        assert!(!ctx.is_adult());
        assert!(ctx.is_child());
    }

    #[test]
    fn test_context_with_events() {
        let save = default_save();
        let ctx = MmGameContext::new(&save)
            .with_event("WOODFALL_CLEAR")
            .with_event("SNOWHEAD_CLEAR");

        assert!(ctx.event("WOODFALL_CLEAR"));
        assert!(ctx.event("SNOWHEAD_CLEAR"));
        assert!(!ctx.event("GREAT_BAY_CLEAR"));
    }

    #[test]
    fn test_context_with_settings() {
        let save = default_save();
        let ctx = MmGameContext::new(&save)
            .with_setting("shuffle_songs", true)
            .with_setting("keysanity", false);

        assert_eq!(ctx.setting("shuffle_songs"), Some(true));
        assert_eq!(ctx.setting("keysanity"), Some(false));
        assert_eq!(ctx.setting("nonexistent"), None);
    }

    #[test]
    fn test_context_with_tricks() {
        let save = default_save();
        let ctx = MmGameContext::new(&save)
            .with_trick("goron_bomb_jump")
            .with_trick("zora_clip");

        assert!(ctx.trick("goron_bomb_jump"));
        assert!(ctx.trick("zora_clip"));
        assert!(!ctx.trick("nonexistent_trick"));
    }

    // ========================================================================
    // Inventory Item Tests
    // ========================================================================

    #[test]
    fn test_has_ocarina() {
        let save = save_with_inventory(MmInventory {
            ocarina: true,
            ..Default::default()
        });
        let ctx = MmGameContext::new(&save);

        assert!(ctx.has_item("OCARINA", 1));
        assert!(ctx.has_item("ocarina", 1));
        assert!(ctx.has_item("OCARINA_OF_TIME", 1));
    }

    #[test]
    fn test_has_bow() {
        let save = save_with_inventory(MmInventory {
            bow: true,
            ..Default::default()
        });
        let ctx = MmGameContext::new(&save);

        assert!(ctx.has_item("BOW", 1));
        assert!(ctx.has_item("HEROS_BOW", 1));
        assert!(ctx.has_item("HERO_BOW", 1));
    }

    #[test]
    fn test_has_arrows() {
        let save = save_with_inventory(MmInventory {
            fire_arrows: true,
            ice_arrows: true,
            light_arrows: false,
            ..Default::default()
        });
        let ctx = MmGameContext::new(&save);

        assert!(ctx.has_item("FIRE_ARROW", 1));
        assert!(ctx.has_item("FIRE_ARROWS", 1));
        assert!(ctx.has_item("ICE_ARROW", 1));
        assert!(!ctx.has_item("LIGHT_ARROW", 1));
    }

    #[test]
    fn test_has_hookshot() {
        let save = save_with_inventory(MmInventory {
            hookshot: true,
            ..Default::default()
        });
        let ctx = MmGameContext::new(&save);

        assert!(ctx.has_item("HOOKSHOT", 1));
    }

    #[test]
    fn test_has_bombs_and_bombchus() {
        let save = save_with_inventory(MmInventory {
            bombs: true,
            bombchus: true,
            ..Default::default()
        });
        let ctx = MmGameContext::new(&save);

        assert!(ctx.has_item("BOMBS", 1));
        assert!(ctx.has_item("BOMB", 1));
        assert!(ctx.has_item("BOMBCHU", 1));
        assert!(ctx.has_item("BOMBCHUS", 1));
    }

    #[test]
    fn test_has_lens_of_truth() {
        let save = save_with_inventory(MmInventory {
            lens: true,
            ..Default::default()
        });
        let ctx = MmGameContext::new(&save);

        assert!(ctx.has_item("LENS", 1));
        assert!(ctx.has_item("LENS_OF_TRUTH", 1));
    }

    #[test]
    fn test_missing_item() {
        let save = default_save();
        let ctx = MmGameContext::new(&save);

        assert!(!ctx.has_item("HOOKSHOT", 1));
        assert!(!ctx.has_item("BOW", 1));
        assert!(!ctx.has_item("NONEXISTENT_ITEM", 1));
    }

    // ========================================================================
    // Sword and Shield Tests
    // ========================================================================

    #[test]
    fn test_swords() {
        let save = MmSave {
            sword: MmSword::GildedSword,
            ..Default::default()
        };
        let ctx = MmGameContext::new(&save);

        assert!(ctx.has_item("SWORD", 1));
        assert!(ctx.has_item("KOKIRI_SWORD", 1));
        assert!(ctx.has_item("RAZOR_SWORD", 1));
        assert!(ctx.has_item("GILDED_SWORD", 1));
    }

    #[test]
    fn test_razor_sword_not_gilded() {
        let save = MmSave {
            sword: MmSword::RazorSword,
            ..Default::default()
        };
        let ctx = MmGameContext::new(&save);

        assert!(ctx.has_item("SWORD", 1));
        assert!(ctx.has_item("KOKIRI_SWORD", 1));
        assert!(ctx.has_item("RAZOR_SWORD", 1));
        assert!(!ctx.has_item("GILDED_SWORD", 1));
    }

    #[test]
    fn test_shields() {
        let save = MmSave {
            shield: MmShield::MirrorShield,
            ..Default::default()
        };
        let ctx = MmGameContext::new(&save);

        assert!(ctx.has_item("SHIELD", 1));
        assert!(ctx.has_item("HERO_SHIELD", 1));
        assert!(ctx.has_item("MIRROR_SHIELD", 1));
    }

    // ========================================================================
    // Mask Tests
    // ========================================================================

    #[test]
    fn test_transformation_masks() {
        let save = MmSave {
            masks: MmMasks {
                transformation: MmTransformationMasks::DEKU
                    | MmTransformationMasks::GORON
                    | MmTransformationMasks::ZORA,
                ..Default::default()
            },
            ..Default::default()
        };
        let ctx = MmGameContext::new(&save);

        assert!(ctx.has_item("DEKU_MASK", 1));
        assert!(ctx.has_item("GORON_MASK", 1));
        assert!(ctx.has_item("ZORA_MASK", 1));
        assert!(!ctx.has_item("FIERCE_DEITY_MASK", 1));
    }

    #[test]
    fn test_collectible_masks() {
        let save = MmSave {
            masks: MmMasks {
                masks_low: MmMasksLow::BUNNY | MmMasksLow::STONE | MmMasksLow::BLAST,
                masks_high: MmMasksHigh::CAPTAIN,
                ..Default::default()
            },
            ..Default::default()
        };
        let ctx = MmGameContext::new(&save);

        assert!(ctx.has_item("BUNNY_HOOD", 1));
        assert!(ctx.has_item("STONE_MASK", 1));
        assert!(ctx.has_item("BLAST_MASK", 1));
        assert!(ctx.has_item("CAPTAIN_HAT", 1));
        assert!(!ctx.has_item("GREAT_FAIRY_MASK", 1));
    }

    // ========================================================================
    // Song Tests
    // ========================================================================

    #[test]
    fn test_songs() {
        let save = MmSave {
            quest_items: MmQuestItems::SONG_TIME
                | MmQuestItems::SONG_HEALING
                | MmQuestItems::SONG_SOARING
                | MmQuestItems::SONG_AWAKENING,
            ..Default::default()
        };
        let ctx = MmGameContext::new(&save);

        assert!(ctx.has_item("SONG_OF_TIME", 1));
        assert!(ctx.has_item("SONG_OF_HEALING", 1));
        assert!(ctx.has_item("SONG_OF_SOARING", 1));
        assert!(ctx.has_item("SONATA_OF_AWAKENING", 1));
        assert!(!ctx.has_item("GORON_LULLABY", 1));
        assert!(!ctx.has_item("OATH_TO_ORDER", 1));
    }

    // ========================================================================
    // Boss Remains Tests
    // ========================================================================

    #[test]
    fn test_boss_remains() {
        let save = MmSave {
            quest_items: MmQuestItems::REMAINS_ODOLWA | MmQuestItems::REMAINS_GOHT,
            ..Default::default()
        };
        let ctx = MmGameContext::new(&save);

        assert!(ctx.has_item("ODOLWA_REMAINS", 1));
        assert!(ctx.has_item("GOHT_REMAINS", 1));
        assert!(!ctx.has_item("GYORG_REMAINS", 1));
        assert!(!ctx.has_item("TWINMOLD_REMAINS", 1));
    }

    // ========================================================================
    // Bottle Tests
    // ========================================================================

    #[test]
    fn test_bottles_count() {
        let save = save_with_inventory(MmInventory {
            bottles: [
                MmBottle::Empty,
                MmBottle::RedPotion,
                MmBottle::None,
                MmBottle::None,
                MmBottle::Fairy,
                MmBottle::None,
            ],
            ..Default::default()
        });
        let ctx = MmGameContext::new(&save);

        assert!(ctx.has_item("BOTTLE", 1));
        assert!(ctx.has_item("BOTTLE", 2));
        assert!(ctx.has_item("BOTTLE", 3));
        assert!(!ctx.has_item("BOTTLE", 4));
    }

    #[test]
    fn test_no_bottles() {
        let save = default_save();
        let ctx = MmGameContext::new(&save);

        assert!(!ctx.has_item("BOTTLE", 1));
    }

    // ========================================================================
    // Magic Tests
    // ========================================================================

    #[test]
    fn test_magic() {
        let save = MmSave {
            magic: MmMagicCapacity::Single,
            ..Default::default()
        };
        let ctx = MmGameContext::new(&save);

        assert!(ctx.has_item("MAGIC", 1));
        assert!(!ctx.has_item("DOUBLE_MAGIC", 1));
    }

    #[test]
    fn test_double_magic() {
        let save = MmSave {
            magic: MmMagicCapacity::Double,
            ..Default::default()
        };
        let ctx = MmGameContext::new(&save);

        assert!(ctx.has_item("MAGIC", 1));
        assert!(ctx.has_item("DOUBLE_MAGIC", 1));
    }

    // ========================================================================
    // Dungeon Item Tests
    // ========================================================================

    #[test]
    fn test_small_keys() {
        let save = MmSave {
            small_keys: MmSmallKeys {
                woodfall: 3,
                snowhead: 1,
                great_bay: 0,
                stone_tower: 4,
            },
            ..Default::default()
        };
        let ctx = MmGameContext::new(&save);

        assert!(ctx.has_item("SMALL_KEY_WOODFALL", 1));
        assert!(ctx.has_item("SMALL_KEY_WOODFALL", 3));
        assert!(!ctx.has_item("SMALL_KEY_WOODFALL", 4));

        assert!(ctx.has_item("SMALL_KEY_SNOWHEAD", 1));
        assert!(!ctx.has_item("SMALL_KEY_SNOWHEAD", 2));

        assert!(!ctx.has_item("SMALL_KEY_GREAT_BAY", 1));

        assert!(ctx.has_item("SMALL_KEY_STONE_TOWER", 4));
    }

    #[test]
    fn test_boss_keys() {
        let save = MmSave {
            dungeon_items: MmAllDungeonItems {
                woodfall: MmDungeonItems::BOSS_KEY,
                snowhead: MmDungeonItems::empty(),
                great_bay: MmDungeonItems::BOSS_KEY | MmDungeonItems::MAP,
                stone_tower: MmDungeonItems::COMPASS,
            },
            ..Default::default()
        };
        let ctx = MmGameContext::new(&save);

        assert!(ctx.has_item("BOSS_KEY_WOODFALL", 1));
        assert!(!ctx.has_item("BOSS_KEY_SNOWHEAD", 1));
        assert!(ctx.has_item("BOSS_KEY_GREAT_BAY", 1));
        assert!(!ctx.has_item("BOSS_KEY_STONE_TOWER", 1));
    }

    #[test]
    fn test_stray_fairies() {
        let save = MmSave {
            stray_fairies: MmStrayFairies {
                clock_town: 1,
                woodfall: 15,
                snowhead: 10,
                great_bay: 5,
                stone_tower: 0,
            },
            ..Default::default()
        };
        let ctx = MmGameContext::new(&save);

        assert!(ctx.has_item("STRAY_FAIRY_CLOCK_TOWN", 1));
        assert!(!ctx.has_item("STRAY_FAIRY_CLOCK_TOWN", 2));

        assert!(ctx.has_item("STRAY_FAIRY_WOODFALL", 15));
        assert!(!ctx.has_item("STRAY_FAIRY_WOODFALL", 16));

        assert!(ctx.has_item("STRAY_FAIRY_SNOWHEAD", 10));
        assert!(ctx.has_item("STRAY_FAIRY_GREAT_BAY", 5));
        assert!(!ctx.has_item("STRAY_FAIRY_STONE_TOWER", 1));
    }

    // ========================================================================
    // Skulltula Token Tests
    // ========================================================================

    #[test]
    fn test_skulltula_tokens() {
        let save = MmSave {
            skull_tokens_swamp: 30,
            skull_tokens_ocean: 15,
            ..Default::default()
        };
        let ctx = MmGameContext::new(&save);

        assert!(ctx.has_item("SKULL_TOKEN_SWAMP", 30));
        assert!(!ctx.has_item("SKULL_TOKEN_SWAMP", 31));

        assert!(ctx.has_item("SKULL_TOKEN_OCEAN", 15));
        assert!(!ctx.has_item("SKULL_TOKEN_OCEAN", 16));
    }

    // ========================================================================
    // Time Tests
    // ========================================================================

    #[test]
    fn test_mm_time_day_1_morning() {
        // Day 1, 6:00 AM (game time = 0x4000)
        let save = MmSave {
            day: 1,
            time: 0x4000,
            ..Default::default()
        };
        let ctx = MmGameContext::new(&save);

        // Should be 0 minutes since Day 1 at 6 AM
        assert_eq!(ctx.mm_time(), 0);
        assert!(ctx.is_day());
        assert!(!ctx.is_night());
    }

    #[test]
    fn test_mm_time_day_1_noon() {
        // Day 1, 12:00 PM (game time = 0x8000)
        let save = MmSave {
            day: 1,
            time: 0x8000,
            ..Default::default()
        };
        let ctx = MmGameContext::new(&save);

        // Should be 360 minutes (6 hours) since Day 1 at 6 AM
        assert_eq!(ctx.mm_time(), 360);
        assert!(ctx.is_day());
    }

    #[test]
    fn test_mm_time_day_1_evening() {
        // Day 1, 6:00 PM (game time = 0xC000)
        let save = MmSave {
            day: 1,
            time: 0xC000,
            ..Default::default()
        };
        let ctx = MmGameContext::new(&save);

        // Should be 720 minutes (12 hours) since Day 1 at 6 AM
        assert_eq!(ctx.mm_time(), 720);
        assert!(ctx.is_night());
    }

    #[test]
    fn test_mm_time_day_2() {
        // Day 2, 6:00 AM
        let save = MmSave {
            day: 2,
            time: 0x4000,
            ..Default::default()
        };
        let ctx = MmGameContext::new(&save);

        // Should be 1440 minutes (24 hours) since Day 1 at 6 AM
        assert_eq!(ctx.mm_time(), 1440);
    }

    #[test]
    fn test_mm_time_day_3() {
        // Day 3, 6:00 AM
        let save = MmSave {
            day: 3,
            time: 0x4000,
            ..Default::default()
        };
        let ctx = MmGameContext::new(&save);

        // Should be 2880 minutes (48 hours) since Day 1 at 6 AM
        assert_eq!(ctx.mm_time(), 2880);
    }

    #[test]
    fn test_mm_time_clamped() {
        // Extreme day value should be clamped
        let save = MmSave {
            day: 100,
            time: 0x4000,
            ..Default::default()
        };
        let ctx = MmGameContext::new(&save);

        // Should be clamped to 4319 (max valid time)
        assert_eq!(ctx.mm_time(), 4319);
    }

    // ========================================================================
    // is_adult / is_child Tests
    // ========================================================================

    #[test]
    fn test_always_child_in_mm() {
        let save = default_save();
        let ctx = MmGameContext::new(&save);

        // MM Link is always "child" for logic purposes
        assert!(ctx.is_child());
        assert!(!ctx.is_adult());
    }

    #[test]
    fn test_child_regardless_of_form() {
        // Even as Fierce Deity, is_child should be true for logic purposes
        let save = MmSave {
            player_form: PlayerForm::FierceDeity,
            ..Default::default()
        };
        let ctx = MmGameContext::new(&save);

        assert!(ctx.is_child());
        assert!(!ctx.is_adult());
    }

    // ========================================================================
    // Builder Pattern Tests
    // ========================================================================

    #[test]
    fn test_builder_with_multiple_events() {
        let save = default_save();
        let ctx = MmGameContext::new(&save).with_events(["EVENT_A", "EVENT_B", "EVENT_C"]);

        assert!(ctx.event("EVENT_A"));
        assert!(ctx.event("EVENT_B"));
        assert!(ctx.event("EVENT_C"));
    }

    #[test]
    fn test_builder_with_multiple_settings() {
        let save = default_save();
        let ctx = MmGameContext::new(&save).with_settings([
            ("setting_a", true),
            ("setting_b", false),
            ("setting_c", true),
        ]);

        assert_eq!(ctx.setting("setting_a"), Some(true));
        assert_eq!(ctx.setting("setting_b"), Some(false));
        assert_eq!(ctx.setting("setting_c"), Some(true));
    }

    #[test]
    fn test_builder_with_multiple_tricks() {
        let save = default_save();
        let ctx = MmGameContext::new(&save).with_tricks(["trick_a", "trick_b"]);

        assert!(ctx.trick("trick_a"));
        assert!(ctx.trick("trick_b"));
        assert!(!ctx.trick("trick_c"));
    }

    #[test]
    fn test_chained_builder() {
        let save = save_with_inventory(MmInventory {
            hookshot: true,
            ..Default::default()
        });
        let ctx = MmGameContext::new(&save)
            .with_event("BOSS_DEFEATED")
            .with_setting("keysanity", true)
            .with_trick("hookshot_jump");

        assert!(ctx.has_item("HOOKSHOT", 1));
        assert!(ctx.event("BOSS_DEFEATED"));
        assert_eq!(ctx.setting("keysanity"), Some(true));
        assert!(ctx.trick("hookshot_jump"));
    }

    // ========================================================================
    // Double Defense Test
    // ========================================================================

    #[test]
    fn test_double_defense() {
        let save = MmSave {
            double_defense: true,
            ..Default::default()
        };
        let ctx = MmGameContext::new(&save);

        assert!(ctx.has_item("DOUBLE_DEFENSE", 1));
    }

    #[test]
    fn test_no_double_defense() {
        let save = default_save();
        let ctx = MmGameContext::new(&save);

        assert!(!ctx.has_item("DOUBLE_DEFENSE", 1));
    }
}
