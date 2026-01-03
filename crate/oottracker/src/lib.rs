#![deny(
    rust_2018_idioms,
    unused,
    unused_crate_dependencies,
    unused_import_braces,
    unused_qualifications,
    warnings
)]
#![allow(
    clippy::large_enum_variant,
    clippy::result_unit_err,
    clippy::too_many_arguments,
    clippy::wrong_self_convention,
    clippy::modulo_one
)]
#![forbid(unsafe_code)]

pub use crate::{ctx::TrackerCtx, knowledge::Knowledge, ram::GameType, ram::Ram, save::Save};
pub use ootmm::WorldDatabase;
use {
    crate::{
        info_tables::InfTable55,
        ram::Pad,
        save::{DungeonItems, GameMode},
    },
    async_proto::Protocol,
    enum_iterator::all,
    itertools::Itertools as _,
    ootr::{check::Check, model::*},
    semver::Version,
    serde::{Deserialize, Serialize},
    std::ops::{AddAssign, Sub},
};

use once_cell::sync::Lazy;
use ootmm::CheckTracker;

/// Global WorldDatabase initialized with embedded ootmm data.
///
/// This singleton is lazily initialized on first access and contains
/// all the world regions, locations, and logic from the embedded YAML files.
static WORLD_DATABASE: Lazy<WorldDatabase> =
    Lazy::new(|| ootmm::create_world_database().expect("Failed to load embedded world database"));

/// Returns a reference to the global WorldDatabase.
///
/// The database is lazily initialized on first access with all embedded
/// world data from the ootmm crate.
///
/// # Example
///
/// ```
/// use oottracker::world_database;
///
/// let db = world_database();
/// assert!(db.has_region("kokiri_forest"));
/// ```
///
/// # Panics
///
/// Panics if the embedded world data fails to parse. This should never
/// happen in a correctly built binary, as the embedded YAML is validated
/// at test time.
pub fn world_database() -> &'static WorldDatabase {
    &WORLD_DATABASE
}

/// Initializes the WorldDatabase eagerly.
///
/// This function can be called at application startup to ensure the
/// WorldDatabase is loaded before it's needed, avoiding any latency
/// on first access during gameplay.
///
/// # Example
///
/// ```
/// use oottracker::init_world_database;
///
/// // Call at startup to pre-load the database
/// init_world_database();
/// ```
///
/// # Panics
///
/// Panics if the embedded world data fails to parse.
pub fn init_world_database() {
    Lazy::force(&WORLD_DATABASE);
}

pub mod checks;
pub mod ctx;
#[cfg(feature = "firebase")]
pub mod firebase;
pub mod game_detection;
pub mod github;
pub mod info_tables;
mod item_ids;
pub mod knowledge;
pub mod logic_context;
pub mod mm_save;
pub mod mm_scene;
pub mod net;
pub mod proto;
pub mod ram;
pub mod region;
pub mod save;
mod scene;
mod text;
pub mod ui;
pub mod websocket;

#[cfg(test)]
pub mod test_utils;

#[derive(Debug, Default, Clone, PartialEq, Eq, Protocol, Deserialize, Serialize)]
pub struct ModelState {
    pub knowledge: Knowledge,
    pub tracker_ctx: TrackerCtx,
    pub ram: Ram,
    /// Check tracker for MM/combo randomizer tracking.
    /// Initialized when MM tracking is active (mm_save is Some).
    pub check_tracker: Option<CheckTracker>,
}

impl ModelState {
    pub fn update_knowledge(&mut self) {
        if self.ram.save.game_mode != GameMode::Gameplay {
            return;
        } //TODO read knowledge from inventory preview on file select?
          // immediate knowledge
          // read dungeon reward info if the player is looking at the dungeon info screen in the pause menu
        let button_pressed = match self.tracker_ctx.cfg_dungeon_info_enable {
            0 => false,
            1 => self.ram.input_p1_raw_pad.contains(Pad::A),
            2.. => self.ram.input_p1_raw_pad.contains(Pad::D_DOWN),
        };
        if button_pressed
            && self.ram.pause_state == 6
            && self.ram.pause_screen_idx == 0
            && !self.ram.pause_changing
            && self.tracker_ctx.cfg_dungeon_info_reward_enable
        {
            for (&location, &reward) in &self.tracker_ctx.cfg_dungeon_rewards {
                let mut known = true;
                if self.tracker_ctx.cfg_dungeon_info_reward_need_altar {
                    known &= match reward {
                        DungeonReward::Medallion(_) => self
                            .ram
                            .save
                            .inf_table
                            .55
                            .contains(InfTable55::TOT_ALTAR_READ_MEDALLION_LOCATIONS),
                        DungeonReward::Stone(_) => self
                            .ram
                            .save
                            .inf_table
                            .55
                            .contains(InfTable55::TOT_ALTAR_READ_STONE_LOCATIONS),
                    };
                }
                if self.tracker_ctx.cfg_dungeon_info_reward_need_compass {
                    match location {
                        DungeonRewardLocation::Dungeon(dungeon) => {
                            known &= self
                                .ram
                                .save
                                .dungeon_items
                                .get(Dungeon::Main(dungeon))
                                .contains(DungeonItems::COMPASS)
                        }
                        DungeonRewardLocation::LinksPocket => {}
                    }
                }
                if known {
                    self.knowledge
                        .dungeon_reward_locations
                        .insert(reward, location);
                }
            }
        }
        // read the current text box for various pieces of information
        if self.ram.current_text_box_id != 0 {
            if let Ok(new_knowledge) =
                self.knowledge.clone() & text::read_knowledge(&self.ram.text_box_contents[..])
            {
                self.knowledge = new_knowledge;
            } else {
                //TODO report/log error?
            }
        }

        // derived knowledge
        // dungeon reward shuffle doesn't exist yet, so if we have exactly 1 reward, it must have been on Links Pocket
        if let Ok(reward) = all()
            .filter(|reward| self.ram.save.quest_items.has(reward))
            .exactly_one()
        {
            self.knowledge
                .dungeon_reward_locations
                .insert(reward, DungeonRewardLocation::LinksPocket);
        }
        // dungeon reward shuffle doesn't exist yet, so if we know the locations of all but 1 reward, the 9th can be determined by process of elimination
        if let Some((reward,)) = all()
            .filter(|reward| !self.knowledge.dungeon_reward_locations.contains_key(reward))
            .collect_tuple()
        {
            let (dungeon,) = all()
                .filter(|dungeon| {
                    !self
                        .knowledge
                        .dungeon_reward_locations
                        .values()
                        .any(|&loc| loc == DungeonRewardLocation::Dungeon(*dungeon))
                })
                .collect_tuple()
                .expect("exactly one reward left but not exactly one reward location left");
            self.knowledge
                .dungeon_reward_locations
                .insert(reward, DungeonRewardLocation::Dungeon(dungeon));
        }
    }

    /// Updates MM-specific knowledge from the current MM save state.
    ///
    /// This method processes MM save data to derive knowledge about:
    /// - Boss remains (dungeon rewards) obtained
    /// - Songs learned
    /// - Transformation masks acquired
    /// - Dungeon item collection status
    ///
    /// Similar to `update_knowledge()` for OoT, but handles MM-specific data structures.
    pub fn update_mm_knowledge(&mut self) {
        // Early return if MM save data is not available
        let Some(mm_save) = self.ram.mm_save.clone() else {
            return;
        };

        // Ensure check tracker is initialized for MM tracking
        self.ensure_check_tracker();

        // ========================================================================
        // Boss Remains Tracking
        // ========================================================================
        // Track boss remains as MM's equivalent of dungeon rewards.
        // When a boss remain is obtained, mark the corresponding boss location as checked.

        if mm_save
            .quest_items
            .contains(mm_save::MmQuestItems::REMAINS_ODOLWA)
        {
            self.mark_check("mm_woodfall_temple_boss");
        }
        if mm_save
            .quest_items
            .contains(mm_save::MmQuestItems::REMAINS_GOHT)
        {
            self.mark_check("mm_snowhead_temple_boss");
        }
        if mm_save
            .quest_items
            .contains(mm_save::MmQuestItems::REMAINS_GYORG)
        {
            self.mark_check("mm_great_bay_temple_boss");
        }
        if mm_save
            .quest_items
            .contains(mm_save::MmQuestItems::REMAINS_TWINMOLD)
        {
            self.mark_check("mm_stone_tower_temple_boss");
        }

        // ========================================================================
        // Song Tracking
        // ========================================================================
        // Track songs learned in MM. These can be important for logic.

        if mm_save
            .quest_items
            .contains(mm_save::MmQuestItems::SONG_TIME)
        {
            self.mark_check("mm_song_of_time");
        }
        if mm_save
            .quest_items
            .contains(mm_save::MmQuestItems::SONG_HEALING)
        {
            self.mark_check("mm_song_of_healing");
        }
        if mm_save
            .quest_items
            .contains(mm_save::MmQuestItems::SONG_EPONA)
        {
            self.mark_check("mm_eponas_song");
        }
        if mm_save
            .quest_items
            .contains(mm_save::MmQuestItems::SONG_SOARING)
        {
            self.mark_check("mm_song_of_soaring");
        }
        if mm_save
            .quest_items
            .contains(mm_save::MmQuestItems::SONG_STORMS)
        {
            self.mark_check("mm_song_of_storms");
        }
        if mm_save
            .quest_items
            .contains(mm_save::MmQuestItems::SONG_AWAKENING)
        {
            self.mark_check("mm_sonata_of_awakening");
        }
        if mm_save
            .quest_items
            .contains(mm_save::MmQuestItems::SONG_GORON)
        {
            self.mark_check("mm_goron_lullaby");
        }
        if mm_save
            .quest_items
            .contains(mm_save::MmQuestItems::SONG_ZORA)
        {
            self.mark_check("mm_new_wave_bossa_nova");
        }
        if mm_save
            .quest_items
            .contains(mm_save::MmQuestItems::SONG_EMPTINESS)
        {
            self.mark_check("mm_elegy_of_emptiness");
        }
        if mm_save
            .quest_items
            .contains(mm_save::MmQuestItems::SONG_ORDER)
        {
            self.mark_check("mm_oath_to_order");
        }

        // ========================================================================
        // Transformation Mask Tracking
        // ========================================================================
        // Track transformation masks which are key progression items in MM.

        if mm_save
            .masks
            .transformation
            .contains(mm_save::MmTransformationMasks::DEKU)
        {
            self.mark_check("mm_deku_mask");
        }
        if mm_save
            .masks
            .transformation
            .contains(mm_save::MmTransformationMasks::GORON)
        {
            self.mark_check("mm_goron_mask");
        }
        if mm_save
            .masks
            .transformation
            .contains(mm_save::MmTransformationMasks::ZORA)
        {
            self.mark_check("mm_zora_mask");
        }
        if mm_save
            .masks
            .transformation
            .contains(mm_save::MmTransformationMasks::FIERCE_DEITY)
        {
            self.mark_check("mm_fierce_deity_mask");
        }
        if mm_save
            .masks
            .masks_high
            .contains(mm_save::MmMasksHigh::GIANT)
        {
            self.mark_check("mm_giants_mask");
        }

        // ========================================================================
        // Key Item Tracking
        // ========================================================================
        // Track important inventory items that may be progression.

        if mm_save.inventory.ocarina {
            self.mark_check("mm_ocarina");
        }
        if mm_save.inventory.bow {
            self.mark_check("mm_bow");
        }
        if mm_save.inventory.hookshot {
            self.mark_check("mm_hookshot");
        }
        if mm_save.inventory.lens {
            self.mark_check("mm_lens_of_truth");
        }
        if mm_save.inventory.great_fairy_sword {
            self.mark_check("mm_great_fairy_sword");
        }
        if mm_save.inventory.fire_arrows {
            self.mark_check("mm_fire_arrows");
        }
        if mm_save.inventory.ice_arrows {
            self.mark_check("mm_ice_arrows");
        }
        if mm_save.inventory.light_arrows {
            self.mark_check("mm_light_arrows");
        }
        if mm_save.inventory.powder_keg {
            self.mark_check("mm_powder_keg");
        }
        if mm_save.inventory.pictograph_box {
            self.mark_check("mm_pictograph_box");
        }

        // ========================================================================
        // Dungeon Items Tracking
        // ========================================================================
        // Track dungeon maps, compasses, and boss keys.

        // Woodfall Temple
        if mm_save
            .dungeon_items
            .woodfall
            .contains(mm_save::MmDungeonItems::BOSS_KEY)
        {
            self.mark_check("mm_woodfall_boss_key");
        }
        // Snowhead Temple
        if mm_save
            .dungeon_items
            .snowhead
            .contains(mm_save::MmDungeonItems::BOSS_KEY)
        {
            self.mark_check("mm_snowhead_boss_key");
        }
        // Great Bay Temple
        if mm_save
            .dungeon_items
            .great_bay
            .contains(mm_save::MmDungeonItems::BOSS_KEY)
        {
            self.mark_check("mm_great_bay_boss_key");
        }
        // Stone Tower Temple
        if mm_save
            .dungeon_items
            .stone_tower
            .contains(mm_save::MmDungeonItems::BOSS_KEY)
        {
            self.mark_check("mm_stone_tower_boss_key");
        }

        // ========================================================================
        // Derived Knowledge
        // ========================================================================
        // Process derived knowledge based on MM save state.

        // If player has all 4 boss remains, they can access the Moon
        let num_remains = mm_save.quest_items.num_remains();
        if num_remains == 4 {
            self.mark_check("mm_moon_access");
        }

        // Track Bomber's Notebook if obtained
        if mm_save
            .quest_items
            .contains(mm_save::MmQuestItems::NOTEBOOK)
        {
            self.mark_check("mm_bombers_notebook");
        }
    }

    /// Ensures the check tracker is initialized when MM tracking is active.
    ///
    /// This method should be called when the model state is updated to ensure
    /// the check tracker is available when MM save data is present.
    pub fn ensure_check_tracker(&mut self) {
        if self.ram.mm_save.is_some() && self.check_tracker.is_none() {
            self.check_tracker = Some(CheckTracker::default());
        }
    }

    /// Marks a location as checked in the tracker.
    ///
    /// Does nothing if the check tracker is not initialized.
    pub fn mark_check(&mut self, location: &str) {
        if let Some(ref mut tracker) = self.check_tracker {
            tracker.mark_checked(location);
        }
    }

    /// Marks a location as unchecked in the tracker.
    ///
    /// Does nothing if the check tracker is not initialized.
    pub fn unmark_check(&mut self, location: &str) {
        if let Some(ref mut tracker) = self.check_tracker {
            tracker.mark_unchecked(location);
        }
    }

    /// Returns whether a location has been checked.
    ///
    /// Returns `false` if the check tracker is not initialized.
    pub fn is_checked(&self, location: &str) -> bool {
        self.check_tracker
            .as_ref()
            .is_some_and(|t| t.is_checked(location))
    }

    /// Returns the number of checked locations.
    ///
    /// Returns `0` if the check tracker is not initialized.
    pub fn checked_count(&self) -> usize {
        self.check_tracker
            .as_ref()
            .map(|t| t.checked_count())
            .unwrap_or(0)
    }

    /// Clears all checked locations in the tracker.
    ///
    /// Does nothing if the check tracker is not initialized.
    pub fn clear_checks(&mut self) {
        if let Some(ref mut tracker) = self.check_tracker {
            tracker.clear();
        }
    }

    /// Returns whether MM tracking is currently active.
    pub fn is_mm_tracking_active(&self) -> bool {
        self.ram.mm_save.is_some()
    }
}

impl AddAssign<ModelDelta> for ModelState {
    fn add_assign(&mut self, rhs: ModelDelta) {
        let ModelDelta {
            knowledge,
            tracker_ctx,
            ram,
            check_tracker,
        } = rhs;
        self.knowledge = knowledge;
        if let Some(tracker_ctx) = tracker_ctx {
            self.tracker_ctx = tracker_ctx
        }
        self.ram += ram;
        if let Some(check_tracker) = check_tracker {
            self.check_tracker = Some(check_tracker);
        }
    }
}

impl Sub<&ModelState> for &ModelState {
    type Output = ModelDelta;

    fn sub(self, rhs: &ModelState) -> ModelDelta {
        let ModelState {
            knowledge,
            tracker_ctx,
            ram,
            check_tracker,
        } = self;
        ModelDelta {
            knowledge: knowledge.clone(), //TODO only include new knowledge?
            tracker_ctx: (*tracker_ctx != rhs.tracker_ctx).then(|| tracker_ctx.clone()),
            ram: ram - &rhs.ram,
            check_tracker: if check_tracker != &rhs.check_tracker {
                check_tracker.clone()
            } else {
                None
            },
        }
    }
}

/// The difference between two model states.
#[derive(Debug, Clone, Protocol)]
pub struct ModelDelta {
    knowledge: Knowledge, //TODO use a separate knowledge delta format?\
    tracker_ctx: Option<TrackerCtx>,
    ram: ram::Delta,
    check_tracker: Option<CheckTracker>,
}

pub fn version() -> Version {
    let version =
        Version::parse(env!("CARGO_PKG_VERSION")).expect("failed to parse current version");
    assert_eq!(version, oottracker_derive::version!());
    version
}

#[cfg(test)]
mod world_database_tests {
    use super::*;

    #[test]
    fn test_world_database_loads_successfully() {
        let db = world_database();
        // Verify database is loaded and has content
        assert!(db.region_count() > 0, "WorldDatabase should have regions");
    }

    #[test]
    fn test_world_database_has_oot_regions() {
        let db = world_database();
        assert!(db.has_region("kokiri_forest"), "Should have Kokiri Forest");
        assert!(db.has_region("lost_woods"), "Should have Lost Woods");
    }

    #[test]
    fn test_world_database_has_mm_regions() {
        let db = world_database();
        assert!(
            db.has_region("clock_town_south"),
            "Should have Clock Town South"
        );
        assert!(db.has_region("termina_field"), "Should have Termina Field");
    }

    #[test]
    fn test_world_database_has_locations() {
        let db = world_database();
        // Check OoT location
        assert!(
            db.get_location("kf_midos_chest_top_left").is_some(),
            "Should have Mido's chest location"
        );
        // Check MM location
        assert!(
            db.get_location("ct_bank_reward_1").is_some(),
            "Should have Clock Town Bank location"
        );
    }

    #[test]
    fn test_world_database_returns_same_instance() {
        let db1 = world_database();
        let db2 = world_database();
        // Both calls should return reference to the same static instance
        assert!(std::ptr::eq(db1, db2), "Should return same instance");
    }

    #[test]
    fn test_init_world_database() {
        // Should not panic
        init_world_database();
        // After init, database should be loaded
        let db = world_database();
        assert!(db.region_count() > 0);
    }

    #[test]
    fn test_world_database_region_count() {
        let db = world_database();
        // Embedded data has 4 regions: kokiri_forest, lost_woods (OoT) + clock_town_south, termina_field (MM)
        assert_eq!(db.region_count(), 4, "Should have 4 embedded regions");
    }
}

#[cfg(test)]
mod update_mm_knowledge_tests {
    use super::*;

    #[test]
    fn test_update_mm_knowledge_no_mm_save() {
        let mut state = ModelState::default();
        // Should not panic when mm_save is None
        state.update_mm_knowledge();
        // Check tracker should not be initialized when no MM save
        assert!(state.check_tracker.is_none());
    }

    #[test]
    fn test_update_mm_knowledge_initializes_check_tracker() {
        let mut state = ModelState::default();
        state.ram.mm_save = Some(mm_save::MmSave::default());
        state.update_mm_knowledge();
        // Check tracker should be initialized
        assert!(state.check_tracker.is_some());
    }

    #[test]
    fn test_update_mm_knowledge_tracks_boss_remains() {
        let mut state = ModelState::default();
        let mut mm_save = mm_save::MmSave::default();

        // Add Odolwa's Remains
        mm_save.quest_items = mm_save::MmQuestItems::REMAINS_ODOLWA;
        state.ram.mm_save = Some(mm_save);
        state.update_mm_knowledge();

        assert!(state.is_checked("mm_woodfall_temple_boss"));
        assert!(!state.is_checked("mm_snowhead_temple_boss"));
        assert!(!state.is_checked("mm_great_bay_temple_boss"));
        assert!(!state.is_checked("mm_stone_tower_temple_boss"));
    }

    #[test]
    fn test_update_mm_knowledge_tracks_all_boss_remains() {
        let mut state = ModelState::default();
        let mut mm_save = mm_save::MmSave::default();

        // Add all boss remains
        mm_save.quest_items = mm_save::MmQuestItems::REMAINS_ODOLWA
            | mm_save::MmQuestItems::REMAINS_GOHT
            | mm_save::MmQuestItems::REMAINS_GYORG
            | mm_save::MmQuestItems::REMAINS_TWINMOLD;
        state.ram.mm_save = Some(mm_save);
        state.update_mm_knowledge();

        assert!(state.is_checked("mm_woodfall_temple_boss"));
        assert!(state.is_checked("mm_snowhead_temple_boss"));
        assert!(state.is_checked("mm_great_bay_temple_boss"));
        assert!(state.is_checked("mm_stone_tower_temple_boss"));
        assert!(state.is_checked("mm_moon_access"));
    }

    #[test]
    fn test_update_mm_knowledge_tracks_songs() {
        let mut state = ModelState::default();
        let mut mm_save = mm_save::MmSave::default();

        // Add some songs
        mm_save.quest_items = mm_save::MmQuestItems::SONG_TIME
            | mm_save::MmQuestItems::SONG_HEALING
            | mm_save::MmQuestItems::SONG_SOARING;
        state.ram.mm_save = Some(mm_save);
        state.update_mm_knowledge();

        assert!(state.is_checked("mm_song_of_time"));
        assert!(state.is_checked("mm_song_of_healing"));
        assert!(state.is_checked("mm_song_of_soaring"));
        assert!(!state.is_checked("mm_eponas_song"));
    }

    #[test]
    fn test_update_mm_knowledge_tracks_transformation_masks() {
        let mut state = ModelState::default();
        let mut mm_save = mm_save::MmSave::default();

        // Add transformation masks
        mm_save.masks.transformation = mm_save::MmTransformationMasks::DEKU
            | mm_save::MmTransformationMasks::GORON
            | mm_save::MmTransformationMasks::ZORA;
        state.ram.mm_save = Some(mm_save);
        state.update_mm_knowledge();

        assert!(state.is_checked("mm_deku_mask"));
        assert!(state.is_checked("mm_goron_mask"));
        assert!(state.is_checked("mm_zora_mask"));
        assert!(!state.is_checked("mm_fierce_deity_mask"));
    }

    #[test]
    fn test_update_mm_knowledge_tracks_key_items() {
        let mut state = ModelState::default();
        let mut mm_save = mm_save::MmSave::default();

        // Add key items
        mm_save.inventory.ocarina = true;
        mm_save.inventory.bow = true;
        mm_save.inventory.hookshot = true;
        state.ram.mm_save = Some(mm_save);
        state.update_mm_knowledge();

        assert!(state.is_checked("mm_ocarina"));
        assert!(state.is_checked("mm_bow"));
        assert!(state.is_checked("mm_hookshot"));
        assert!(!state.is_checked("mm_lens_of_truth"));
    }

    #[test]
    fn test_update_mm_knowledge_tracks_boss_keys() {
        let mut state = ModelState::default();
        let mut mm_save = mm_save::MmSave::default();

        // Add boss keys
        mm_save.dungeon_items.woodfall = mm_save::MmDungeonItems::BOSS_KEY;
        mm_save.dungeon_items.snowhead = mm_save::MmDungeonItems::BOSS_KEY;
        state.ram.mm_save = Some(mm_save);
        state.update_mm_knowledge();

        assert!(state.is_checked("mm_woodfall_boss_key"));
        assert!(state.is_checked("mm_snowhead_boss_key"));
        assert!(!state.is_checked("mm_great_bay_boss_key"));
        assert!(!state.is_checked("mm_stone_tower_boss_key"));
    }

    #[test]
    fn test_update_mm_knowledge_tracks_bombers_notebook() {
        let mut state = ModelState::default();
        let mut mm_save = mm_save::MmSave::default();

        // Add Bomber's Notebook
        mm_save.quest_items = mm_save::MmQuestItems::NOTEBOOK;
        state.ram.mm_save = Some(mm_save);
        state.update_mm_knowledge();

        assert!(state.is_checked("mm_bombers_notebook"));
    }

    #[test]
    fn test_update_mm_knowledge_tracks_giants_mask() {
        let mut state = ModelState::default();
        let mut mm_save = mm_save::MmSave::default();

        // Add Giant's Mask (in masks_high, not transformation masks)
        mm_save.masks.masks_high = mm_save::MmMasksHigh::GIANT;
        state.ram.mm_save = Some(mm_save);
        state.update_mm_knowledge();

        assert!(state.is_checked("mm_giants_mask"));
    }

    #[test]
    fn test_update_mm_knowledge_moon_access_requires_all_remains() {
        let mut state = ModelState::default();
        let mut mm_save = mm_save::MmSave::default();

        // Add only 3 remains - should NOT mark moon access
        mm_save.quest_items = mm_save::MmQuestItems::REMAINS_ODOLWA
            | mm_save::MmQuestItems::REMAINS_GOHT
            | mm_save::MmQuestItems::REMAINS_GYORG;
        state.ram.mm_save = Some(mm_save);
        state.update_mm_knowledge();

        assert!(!state.is_checked("mm_moon_access"));

        // Add the 4th remain
        let mut mm_save2 = mm_save::MmSave::default();
        mm_save2.quest_items = mm_save::MmQuestItems::REMAINS_ODOLWA
            | mm_save::MmQuestItems::REMAINS_GOHT
            | mm_save::MmQuestItems::REMAINS_GYORG
            | mm_save::MmQuestItems::REMAINS_TWINMOLD;
        state.ram.mm_save = Some(mm_save2);
        state.update_mm_knowledge();

        assert!(state.is_checked("mm_moon_access"));
    }
}
