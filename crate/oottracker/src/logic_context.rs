//! Logic context for Majora's Mask.
//!
//! This module provides `MmGameContext` which translates `MmSave` data into
//! logic conditions used by the OoTMM randomizer logic system.
//!
//! The context maps inventory items, masks, songs, and other game state to their
//! corresponding logic identifiers.

use crate::mm_save::{MmBottle, MmMagicCapacity, MmSave, MmShield, MmSword, MmUpgrades};
use ootmm::expr::EvalContext;
use std::collections::HashMap;

/// Logic identifiers for MM items.
///
/// These match the string identifiers used in the OoTMM logic system.
pub mod logic_ids {
    // Equipment items
    pub const OCARINA_OF_TIME: &str = "OCARINA_OF_TIME";
    pub const HEROS_BOW: &str = "HEROS_BOW";
    pub const FIRE_ARROW: &str = "FIRE_ARROW";
    pub const ICE_ARROW: &str = "ICE_ARROW";
    pub const LIGHT_ARROW: &str = "LIGHT_ARROW";
    pub const HOOKSHOT: &str = "HOOKSHOT";
    pub const BOMB: &str = "BOMB";
    pub const BOMBCHU: &str = "BOMBCHU";
    pub const POWDER_KEG: &str = "POWDER_KEG";
    pub const LENS_OF_TRUTH: &str = "LENS_OF_TRUTH";
    pub const PICTOGRAPH_BOX: &str = "PICTOGRAPH_BOX";
    pub const GREAT_FAIRY_SWORD: &str = "GREAT_FAIRY_SWORD";
    pub const MAGIC_BEAN: &str = "MAGIC_BEAN";
    pub const DEKU_STICK: &str = "DEKU_STICK";
    pub const DEKU_NUT: &str = "DEKU_NUT";

    // Transformation masks
    pub const DEKU_MASK: &str = "DEKU_MASK";
    pub const GORON_MASK: &str = "GORON_MASK";
    pub const ZORA_MASK: &str = "ZORA_MASK";
    pub const FIERCE_DEITY_MASK: &str = "FIERCE_DEITY_MASK";

    // Regular masks
    pub const POSTMAN_HAT: &str = "POSTMAN_HAT";
    pub const ALL_NIGHT_MASK: &str = "ALL_NIGHT_MASK";
    pub const BLAST_MASK: &str = "BLAST_MASK";
    pub const STONE_MASK: &str = "STONE_MASK";
    pub const GREAT_FAIRY_MASK: &str = "GREAT_FAIRY_MASK";
    pub const KEATON_MASK: &str = "KEATON_MASK";
    pub const BREMEN_MASK: &str = "BREMEN_MASK";
    pub const BUNNY_HOOD: &str = "BUNNY_HOOD";
    pub const DON_GERO_MASK: &str = "DON_GERO_MASK";
    pub const MASK_OF_SCENTS: &str = "MASK_OF_SCENTS";
    pub const ROMANI_MASK: &str = "ROMANI_MASK";
    pub const CIRCUS_LEADER_MASK: &str = "CIRCUS_LEADER_MASK";
    pub const KAFEI_MASK: &str = "KAFEI_MASK";
    pub const COUPLES_MASK: &str = "COUPLES_MASK";
    pub const MASK_OF_TRUTH: &str = "MASK_OF_TRUTH";
    pub const KAMARO_MASK: &str = "KAMARO_MASK";
    pub const GIBDO_MASK: &str = "GIBDO_MASK";
    pub const GARO_MASK: &str = "GARO_MASK";
    pub const CAPTAIN_HAT: &str = "CAPTAIN_HAT";
    pub const GIANT_MASK: &str = "GIANT_MASK";

    // Songs
    pub const SONG_OF_TIME: &str = "SONG_OF_TIME";
    pub const SONG_OF_HEALING: &str = "SONG_OF_HEALING";
    pub const EPONAS_SONG: &str = "EPONAS_SONG";
    pub const SONG_OF_SOARING: &str = "SONG_OF_SOARING";
    pub const SONG_OF_STORMS: &str = "SONG_OF_STORMS";
    pub const SONATA_OF_AWAKENING: &str = "SONATA_OF_AWAKENING";
    pub const GORON_LULLABY: &str = "GORON_LULLABY";
    pub const NEW_WAVE_BOSSA_NOVA: &str = "NEW_WAVE_BOSSA_NOVA";
    pub const ELEGY_OF_EMPTINESS: &str = "ELEGY_OF_EMPTINESS";
    pub const OATH_TO_ORDER: &str = "OATH_TO_ORDER";

    // Boss remains
    pub const ODOLWA_REMAINS: &str = "ODOLWA_REMAINS";
    pub const GOHT_REMAINS: &str = "GOHT_REMAINS";
    pub const GYORG_REMAINS: &str = "GYORG_REMAINS";
    pub const TWINMOLD_REMAINS: &str = "TWINMOLD_REMAINS";

    // Equipment
    pub const KOKIRI_SWORD: &str = "KOKIRI_SWORD";
    pub const RAZOR_SWORD: &str = "RAZOR_SWORD";
    pub const GILDED_SWORD: &str = "GILDED_SWORD";
    pub const HERO_SHIELD: &str = "HERO_SHIELD";
    pub const MIRROR_SHIELD: &str = "MIRROR_SHIELD";

    // Bottles
    pub const BOTTLE: &str = "BOTTLE";

    // Upgrades
    pub const MAGIC_METER: &str = "MAGIC_METER";
    pub const DOUBLE_MAGIC: &str = "DOUBLE_MAGIC";
    pub const DOUBLE_DEFENSE: &str = "DOUBLE_DEFENSE";
    pub const ADULT_WALLET: &str = "ADULT_WALLET";
    pub const GIANT_WALLET: &str = "GIANT_WALLET";
    pub const QUIVER_30: &str = "QUIVER_30";
    pub const QUIVER_40: &str = "QUIVER_40";
    pub const QUIVER_50: &str = "QUIVER_50";
    pub const BOMB_BAG_20: &str = "BOMB_BAG_20";
    pub const BOMB_BAG_30: &str = "BOMB_BAG_30";
    pub const BOMB_BAG_40: &str = "BOMB_BAG_40";

    // Dungeon-specific keys
    pub const SMALL_KEY_WOODFALL_TEMPLE: &str = "SMALL_KEY_WOODFALL_TEMPLE";
    pub const SMALL_KEY_SNOWHEAD_TEMPLE: &str = "SMALL_KEY_SNOWHEAD_TEMPLE";
    pub const SMALL_KEY_GREAT_BAY_TEMPLE: &str = "SMALL_KEY_GREAT_BAY_TEMPLE";
    pub const SMALL_KEY_STONE_TOWER_TEMPLE: &str = "SMALL_KEY_STONE_TOWER_TEMPLE";

    // Stray fairies
    pub const STRAY_FAIRY_CLOCK_TOWN: &str = "STRAY_FAIRY_CLOCK_TOWN";
    pub const STRAY_FAIRY_WOODFALL: &str = "STRAY_FAIRY_WOODFALL";
    pub const STRAY_FAIRY_SNOWHEAD: &str = "STRAY_FAIRY_SNOWHEAD";
    pub const STRAY_FAIRY_GREAT_BAY: &str = "STRAY_FAIRY_GREAT_BAY";
    pub const STRAY_FAIRY_STONE_TOWER: &str = "STRAY_FAIRY_STONE_TOWER";
}

/// Game context for MM logic evaluation.
///
/// This struct bridges `MmSave` data to the logic system by providing
/// methods to query game state using logic identifiers.
///
/// # Example
///
/// ```ignore
/// use oottracker::mm_save::MmSave;
/// use oottracker::logic_context::MmGameContext;
///
/// let save = MmSave::default();
/// let ctx = MmGameContext::new(&save);
///
/// // Check if player has an item
/// assert!(!ctx.has_item("HOOKSHOT"));
///
/// // Get inventory as a map for logic evaluation
/// let inventory = ctx.build_inventory();
/// ```
#[derive(Debug)]
pub struct MmGameContext<'a> {
    save: &'a MmSave,
}

impl<'a> MmGameContext<'a> {
    /// Create a new game context from an `MmSave` reference.
    pub fn new(save: &'a MmSave) -> Self {
        Self { save }
    }

    /// Get a reference to the underlying save data.
    pub fn save(&self) -> &MmSave {
        self.save
    }

    // ========================================================================
    // Equipment Item Accessors
    // ========================================================================

    /// Check if player has the Ocarina of Time.
    pub fn has_ocarina(&self) -> bool {
        self.save.has_ocarina()
    }

    /// Check if player has the Hero's Bow.
    pub fn has_bow(&self) -> bool {
        self.save.has_heros_bow()
    }

    /// Check if player has Fire Arrows.
    pub fn has_fire_arrow(&self) -> bool {
        self.save.has_fire_arrow()
    }

    /// Check if player has Ice Arrows.
    pub fn has_ice_arrow(&self) -> bool {
        self.save.has_ice_arrow()
    }

    /// Check if player has Light Arrows.
    pub fn has_light_arrow(&self) -> bool {
        self.save.has_light_arrow()
    }

    /// Check if player has the Hookshot.
    pub fn has_hookshot(&self) -> bool {
        self.save.has_hookshot()
    }

    /// Check if player has Bombs.
    pub fn has_bombs(&self) -> bool {
        self.save.has_bombs()
    }

    /// Check if player has Bombchus.
    pub fn has_bombchu(&self) -> bool {
        self.save.has_bombchu()
    }

    /// Check if player has Powder Kegs.
    pub fn has_powder_keg(&self) -> bool {
        self.save.has_powder_keg()
    }

    /// Check if player has the Lens of Truth.
    pub fn has_lens_of_truth(&self) -> bool {
        self.save.has_lens_of_truth()
    }

    /// Check if player has the Pictograph Box.
    pub fn has_pictograph_box(&self) -> bool {
        self.save.has_pictograph_box()
    }

    /// Check if player has the Great Fairy's Sword.
    pub fn has_great_fairy_sword(&self) -> bool {
        self.save.has_great_fairy_sword()
    }

    /// Check if player has Magic Beans.
    pub fn has_magic_bean(&self) -> bool {
        self.save.has_magic_bean()
    }

    /// Check if player has Deku Sticks.
    pub fn has_deku_stick(&self) -> bool {
        self.save.inventory.deku_sticks
    }

    /// Check if player has Deku Nuts.
    pub fn has_deku_nut(&self) -> bool {
        self.save.inventory.deku_nuts
    }

    // ========================================================================
    // Transformation Mask Accessors
    // ========================================================================

    /// Check if player has the Deku Mask.
    pub fn has_deku_mask(&self) -> bool {
        self.save.has_deku_mask()
    }

    /// Check if player has the Goron Mask.
    pub fn has_goron_mask(&self) -> bool {
        self.save.has_goron_mask()
    }

    /// Check if player has the Zora Mask.
    pub fn has_zora_mask(&self) -> bool {
        self.save.has_zora_mask()
    }

    /// Check if player has the Fierce Deity Mask.
    pub fn has_fierce_deity_mask(&self) -> bool {
        self.save.has_fierce_deity_mask()
    }

    // ========================================================================
    // Regular Mask Accessors
    // ========================================================================

    /// Check if player has the Postman's Hat.
    pub fn has_postman_hat(&self) -> bool {
        self.save.has_postman_hat()
    }

    /// Check if player has the All-Night Mask.
    pub fn has_all_night_mask(&self) -> bool {
        self.save.has_all_night_mask()
    }

    /// Check if player has the Blast Mask.
    pub fn has_blast_mask(&self) -> bool {
        self.save.has_blast_mask()
    }

    /// Check if player has the Stone Mask.
    pub fn has_stone_mask(&self) -> bool {
        self.save.has_stone_mask()
    }

    /// Check if player has the Great Fairy Mask.
    pub fn has_great_fairy_mask(&self) -> bool {
        self.save.has_great_fairy_mask()
    }

    /// Check if player has the Keaton Mask.
    pub fn has_keaton_mask(&self) -> bool {
        self.save.has_keaton_mask()
    }

    /// Check if player has the Bremen Mask.
    pub fn has_bremen_mask(&self) -> bool {
        self.save.has_bremen_mask()
    }

    /// Check if player has the Bunny Hood.
    pub fn has_bunny_hood(&self) -> bool {
        self.save.has_bunny_hood()
    }

    /// Check if player has Don Gero's Mask.
    pub fn has_don_gero_mask(&self) -> bool {
        self.save.has_don_gero_mask()
    }

    /// Check if player has the Mask of Scents.
    pub fn has_mask_of_scents(&self) -> bool {
        self.save.has_mask_of_scents()
    }

    /// Check if player has Romani's Mask.
    pub fn has_romani_mask(&self) -> bool {
        self.save.has_romani_mask()
    }

    /// Check if player has the Circus Leader's Mask.
    pub fn has_circus_leader_mask(&self) -> bool {
        self.save.has_circus_leader_mask()
    }

    /// Check if player has Kafei's Mask.
    pub fn has_kafei_mask(&self) -> bool {
        self.save.has_kafei_mask()
    }

    /// Check if player has the Couple's Mask.
    pub fn has_couples_mask(&self) -> bool {
        self.save.has_couples_mask()
    }

    /// Check if player has the Mask of Truth.
    pub fn has_mask_of_truth(&self) -> bool {
        self.save.has_mask_of_truth()
    }

    /// Check if player has Kamaro's Mask.
    pub fn has_kamaro_mask(&self) -> bool {
        self.save.has_kamaro_mask()
    }

    /// Check if player has the Gibdo Mask.
    pub fn has_gibdo_mask(&self) -> bool {
        self.save.has_gibdo_mask()
    }

    /// Check if player has the Garo's Mask.
    pub fn has_garo_mask(&self) -> bool {
        self.save.has_garo_mask()
    }

    /// Check if player has the Captain's Hat.
    pub fn has_captain_hat(&self) -> bool {
        self.save.has_captain_hat()
    }

    /// Check if player has the Giant's Mask.
    pub fn has_giant_mask(&self) -> bool {
        self.save.has_giant_mask()
    }

    // ========================================================================
    // Song Accessors
    // ========================================================================

    /// Check if player has Song of Time.
    pub fn has_song_of_time(&self) -> bool {
        self.save.has_song_of_time()
    }

    /// Check if player has Song of Healing.
    pub fn has_song_of_healing(&self) -> bool {
        self.save.has_song_of_healing()
    }

    /// Check if player has Epona's Song.
    pub fn has_eponas_song(&self) -> bool {
        self.save.has_eponas_song()
    }

    /// Check if player has Song of Soaring.
    pub fn has_song_of_soaring(&self) -> bool {
        self.save.has_song_of_soaring()
    }

    /// Check if player has Song of Storms.
    pub fn has_song_of_storms(&self) -> bool {
        self.save.has_song_of_storms()
    }

    /// Check if player has Sonata of Awakening.
    pub fn has_sonata_of_awakening(&self) -> bool {
        self.save.has_sonata_of_awakening()
    }

    /// Check if player has Goron Lullaby.
    pub fn has_goron_lullaby(&self) -> bool {
        self.save.has_goron_lullaby()
    }

    /// Check if player has New Wave Bossa Nova.
    pub fn has_new_wave_bossa_nova(&self) -> bool {
        self.save.has_new_wave_bossa_nova()
    }

    /// Check if player has Elegy of Emptiness.
    pub fn has_elegy_of_emptiness(&self) -> bool {
        self.save.has_elegy_of_emptiness()
    }

    /// Check if player has Oath to Order.
    pub fn has_oath_to_order(&self) -> bool {
        self.save.has_oath_to_order()
    }

    // ========================================================================
    // Boss Remains Accessors
    // ========================================================================

    /// Check if player has Odolwa's Remains.
    pub fn has_odolwa_remains(&self) -> bool {
        self.save.has_odolwa_remains()
    }

    /// Check if player has Goht's Remains.
    pub fn has_goht_remains(&self) -> bool {
        self.save.has_goht_remains()
    }

    /// Check if player has Gyorg's Remains.
    pub fn has_gyorg_remains(&self) -> bool {
        self.save.has_gyorg_remains()
    }

    /// Check if player has Twinmold's Remains.
    pub fn has_twinmold_remains(&self) -> bool {
        self.save.has_twinmold_remains()
    }

    /// Get the number of boss remains collected.
    pub fn boss_remains_count(&self) -> u8 {
        self.save.quest_items.num_remains()
    }

    // ========================================================================
    // Equipment Accessors (Sword/Shield)
    // ========================================================================

    /// Check if player has Kokiri Sword.
    pub fn has_kokiri_sword(&self) -> bool {
        matches!(
            self.save.sword,
            MmSword::KokiriSword | MmSword::RazorSword | MmSword::GildedSword
        )
    }

    /// Check if player has Razor Sword.
    pub fn has_razor_sword(&self) -> bool {
        matches!(self.save.sword, MmSword::RazorSword | MmSword::GildedSword)
    }

    /// Check if player has Gilded Sword.
    pub fn has_gilded_sword(&self) -> bool {
        matches!(self.save.sword, MmSword::GildedSword)
    }

    /// Check if player has Hero's Shield.
    pub fn has_hero_shield(&self) -> bool {
        matches!(
            self.save.shield,
            MmShield::HeroShield | MmShield::MirrorShield
        )
    }

    /// Check if player has Mirror Shield.
    pub fn has_mirror_shield(&self) -> bool {
        matches!(self.save.shield, MmShield::MirrorShield)
    }

    // ========================================================================
    // Upgrade Accessors
    // ========================================================================

    /// Check if player has magic.
    pub fn has_magic(&self) -> bool {
        self.save.has_magic()
    }

    /// Check if player has single magic meter.
    pub fn has_magic_meter(&self) -> bool {
        matches!(
            self.save.magic,
            MmMagicCapacity::Single | MmMagicCapacity::Double
        )
    }

    /// Check if player has double magic.
    pub fn has_double_magic(&self) -> bool {
        matches!(self.save.magic, MmMagicCapacity::Double)
    }

    /// Check if player has double defense.
    pub fn has_double_defense(&self) -> bool {
        self.save.double_defense
    }

    /// Get wallet upgrade level (0 = none, 1 = adult, 2 = giant).
    pub fn wallet_level(&self) -> u8 {
        let wallet = self.save.upgrades.wallet();
        if wallet.contains(MmUpgrades::GIANTS_WALLET) {
            2
        } else if wallet.contains(MmUpgrades::ADULTS_WALLET) {
            1
        } else {
            0
        }
    }

    /// Check if player has Adult Wallet.
    pub fn has_adult_wallet(&self) -> bool {
        self.wallet_level() >= 1
    }

    /// Check if player has Giant Wallet.
    pub fn has_giant_wallet(&self) -> bool {
        self.wallet_level() >= 2
    }

    /// Get quiver upgrade level (0 = none, 1 = 30, 2 = 40, 3 = 50).
    pub fn quiver_level(&self) -> u8 {
        let quiver = self.save.upgrades.quiver();
        if quiver.contains(MmUpgrades::QUIVER_50) {
            3
        } else if quiver.contains(MmUpgrades::QUIVER_40) {
            2
        } else if quiver.contains(MmUpgrades::QUIVER_30) {
            1
        } else {
            0
        }
    }

    /// Get bomb bag upgrade level (0 = none, 1 = 20, 2 = 30, 3 = 40).
    pub fn bomb_bag_level(&self) -> u8 {
        let bomb_bag = self.save.upgrades.bomb_bag();
        if bomb_bag.contains(MmUpgrades::BOMB_BAG_40) {
            3
        } else if bomb_bag.contains(MmUpgrades::BOMB_BAG_30) {
            2
        } else if bomb_bag.contains(MmUpgrades::BOMB_BAG_20) {
            1
        } else {
            0
        }
    }

    // ========================================================================
    // Bottle Accessors
    // ========================================================================

    /// Get the number of bottles the player has.
    pub fn bottle_count(&self) -> u8 {
        self.save
            .inventory
            .bottles
            .iter()
            .filter(|b| !matches!(b, MmBottle::None))
            .count() as u8
    }

    /// Check if player has at least one bottle.
    pub fn has_bottle(&self) -> bool {
        self.bottle_count() > 0
    }

    // ========================================================================
    // Dungeon Key Accessors
    // ========================================================================

    /// Get Woodfall Temple small key count.
    pub fn woodfall_keys(&self) -> u8 {
        self.save.small_keys.woodfall
    }

    /// Get Snowhead Temple small key count.
    pub fn snowhead_keys(&self) -> u8 {
        self.save.small_keys.snowhead
    }

    /// Get Great Bay Temple small key count.
    pub fn great_bay_keys(&self) -> u8 {
        self.save.small_keys.great_bay
    }

    /// Get Stone Tower Temple small key count.
    pub fn stone_tower_keys(&self) -> u8 {
        self.save.small_keys.stone_tower
    }

    // ========================================================================
    // Stray Fairy Accessors
    // ========================================================================

    /// Get Clock Town stray fairy count.
    pub fn clock_town_fairies(&self) -> u8 {
        self.save.stray_fairies.clock_town
    }

    /// Get Woodfall stray fairy count.
    pub fn woodfall_fairies(&self) -> u8 {
        self.save.stray_fairies.woodfall
    }

    /// Get Snowhead stray fairy count.
    pub fn snowhead_fairies(&self) -> u8 {
        self.save.stray_fairies.snowhead
    }

    /// Get Great Bay stray fairy count.
    pub fn great_bay_fairies(&self) -> u8 {
        self.save.stray_fairies.great_bay
    }

    /// Get Stone Tower stray fairy count.
    pub fn stone_tower_fairies(&self) -> u8 {
        self.save.stray_fairies.stone_tower
    }

    // ========================================================================
    // Logic Identifier Lookup
    // ========================================================================

    /// Check if player has an item by its logic identifier.
    ///
    /// The identifier is case-insensitive and supports both UPPER_SNAKE_CASE
    /// and snake_case formats.
    ///
    /// # Example
    ///
    /// ```ignore
    /// let ctx = MmGameContext::new(&save);
    /// assert!(ctx.has_item("HOOKSHOT"));
    /// assert!(ctx.has_item("hookshot"));
    /// ```
    pub fn has_item(&self, item: &str) -> bool {
        let item_upper = item.to_uppercase();
        match item_upper.as_str() {
            // Equipment
            "OCARINA_OF_TIME" | "OCARINAOFTTIME" => self.has_ocarina(),
            "HEROS_BOW" | "HEROSBOW" => self.has_bow(),
            "FIRE_ARROW" | "FIREARROW" => self.has_fire_arrow(),
            "ICE_ARROW" | "ICEARROW" => self.has_ice_arrow(),
            "LIGHT_ARROW" | "LIGHTARROW" => self.has_light_arrow(),
            "HOOKSHOT" => self.has_hookshot(),
            "BOMB" | "BOMBS" => self.has_bombs(),
            "BOMBCHU" | "BOMBCHUS" => self.has_bombchu(),
            "POWDER_KEG" | "POWDERKEG" => self.has_powder_keg(),
            "LENS_OF_TRUTH" | "LENSOFTRUTH" => self.has_lens_of_truth(),
            "PICTOGRAPH_BOX" | "PICTOGRAPHBOX" => self.has_pictograph_box(),
            "GREAT_FAIRY_SWORD" | "GREATFAIRYSWORD" => self.has_great_fairy_sword(),
            "MAGIC_BEAN" | "MAGICBEAN" => self.has_magic_bean(),
            "DEKU_STICK" | "DEKUSTICK" => self.has_deku_stick(),
            "DEKU_NUT" | "DEKUNUT" => self.has_deku_nut(),

            // Transformation masks
            "DEKU_MASK" | "DEKUMASK" => self.has_deku_mask(),
            "GORON_MASK" | "GORONMASK" => self.has_goron_mask(),
            "ZORA_MASK" | "ZORAMASK" => self.has_zora_mask(),
            "FIERCE_DEITY_MASK" | "FIERCEDEITYMASK" => self.has_fierce_deity_mask(),

            // Regular masks
            "POSTMAN_HAT" | "POSTMANHAT" => self.has_postman_hat(),
            "ALL_NIGHT_MASK" | "ALLNIGHTMASK" => self.has_all_night_mask(),
            "BLAST_MASK" | "BLASTMASK" => self.has_blast_mask(),
            "STONE_MASK" | "STONEMASK" => self.has_stone_mask(),
            "GREAT_FAIRY_MASK" | "GREATFAIRYMASK" => self.has_great_fairy_mask(),
            "KEATON_MASK" | "KEATONMASK" => self.has_keaton_mask(),
            "BREMEN_MASK" | "BREMENMASK" => self.has_bremen_mask(),
            "BUNNY_HOOD" | "BUNNYHOOD" => self.has_bunny_hood(),
            "DON_GERO_MASK" | "DONGEROMASK" => self.has_don_gero_mask(),
            "MASK_OF_SCENTS" | "MASKOFSCENTS" => self.has_mask_of_scents(),
            "ROMANI_MASK" | "ROMANIMASK" => self.has_romani_mask(),
            "CIRCUS_LEADER_MASK" | "CIRCUSLEADERMASK" => self.has_circus_leader_mask(),
            "KAFEI_MASK" | "KAFEIMASK" => self.has_kafei_mask(),
            "COUPLES_MASK" | "COUPLESMASK" => self.has_couples_mask(),
            "MASK_OF_TRUTH" | "MASKOFTRUTH" => self.has_mask_of_truth(),
            "KAMARO_MASK" | "KAMAROMASK" => self.has_kamaro_mask(),
            "GIBDO_MASK" | "GIBDOMASK" => self.has_gibdo_mask(),
            "GARO_MASK" | "GAROMASK" => self.has_garo_mask(),
            "CAPTAIN_HAT" | "CAPTAINHAT" => self.has_captain_hat(),
            "GIANT_MASK" | "GIANTMASK" => self.has_giant_mask(),

            // Songs
            "SONG_OF_TIME" | "SONGOFTIME" => self.has_song_of_time(),
            "SONG_OF_HEALING" | "SONGOFHEALING" => self.has_song_of_healing(),
            "EPONAS_SONG" | "EPONASSONG" => self.has_eponas_song(),
            "SONG_OF_SOARING" | "SONGOFSOARING" => self.has_song_of_soaring(),
            "SONG_OF_STORMS" | "SONGOFSTORMS" => self.has_song_of_storms(),
            "SONATA_OF_AWAKENING" | "SONATAOFAWAKENING" => self.has_sonata_of_awakening(),
            "GORON_LULLABY" | "GORONLULLABY" => self.has_goron_lullaby(),
            "NEW_WAVE_BOSSA_NOVA" | "NEWWAVEBOSSANOVA" => self.has_new_wave_bossa_nova(),
            "ELEGY_OF_EMPTINESS" | "ELEGYOFEMPTINESS" => self.has_elegy_of_emptiness(),
            "OATH_TO_ORDER" | "OATHTOORDER" => self.has_oath_to_order(),

            // Boss remains
            "ODOLWA_REMAINS" | "ODOLWAREMAINS" => self.has_odolwa_remains(),
            "GOHT_REMAINS" | "GOHTREMAINS" => self.has_goht_remains(),
            "GYORG_REMAINS" | "GYORGREMAINS" => self.has_gyorg_remains(),
            "TWINMOLD_REMAINS" | "TWINMOLDREMAINS" => self.has_twinmold_remains(),

            // Swords
            "KOKIRI_SWORD" | "KOKIRISWORD" => self.has_kokiri_sword(),
            "RAZOR_SWORD" | "RAZORSWORD" => self.has_razor_sword(),
            "GILDED_SWORD" | "GILDEDSWORD" => self.has_gilded_sword(),

            // Shields
            "HERO_SHIELD" | "HEROSHIELD" => self.has_hero_shield(),
            "MIRROR_SHIELD" | "MIRRORSHIELD" => self.has_mirror_shield(),

            // Bottles
            "BOTTLE" => self.has_bottle(),

            // Upgrades
            "MAGIC" | "MAGIC_METER" | "MAGICMETER" => self.has_magic_meter(),
            "DOUBLE_MAGIC" | "DOUBLEMAGIC" => self.has_double_magic(),
            "DOUBLE_DEFENSE" | "DOUBLEDEFENSE" => self.has_double_defense(),
            "ADULT_WALLET" | "ADULTWALLET" => self.has_adult_wallet(),
            "GIANT_WALLET" | "GIANTWALLET" | "GIANTS_WALLET" | "GIANTSWALLET" => {
                self.has_giant_wallet()
            }

            _ => false,
        }
    }

    /// Get the count of an item by its logic identifier.
    ///
    /// For non-stackable items, returns 1 if owned, 0 otherwise.
    /// For stackable items (bottles, keys, fairies), returns the count.
    pub fn item_count(&self, item: &str) -> u32 {
        let item_upper = item.to_uppercase();
        match item_upper.as_str() {
            // Stackable items
            "BOTTLE" => u32::from(self.bottle_count()),
            "SMALL_KEY_WOODFALL_TEMPLE" | "SMALLKEYWOODFALLTEMPLE" => {
                u32::from(self.woodfall_keys())
            }
            "SMALL_KEY_SNOWHEAD_TEMPLE" | "SMALLKEYSNOWHEADTEMPLE" => {
                u32::from(self.snowhead_keys())
            }
            "SMALL_KEY_GREAT_BAY_TEMPLE" | "SMALLKEYGREATBAYTEMPLE" => {
                u32::from(self.great_bay_keys())
            }
            "SMALL_KEY_STONE_TOWER_TEMPLE" | "SMALLKEYSTONETOWERTEMPLE" => {
                u32::from(self.stone_tower_keys())
            }
            "STRAY_FAIRY_CLOCK_TOWN" | "STRAYFAIRYCLOCKTOWN" => {
                u32::from(self.clock_town_fairies())
            }
            "STRAY_FAIRY_WOODFALL" | "STRAYFAIRYWOODFALL" => u32::from(self.woodfall_fairies()),
            "STRAY_FAIRY_SNOWHEAD" | "STRAYFAIRYSNOWHEAD" => u32::from(self.snowhead_fairies()),
            "STRAY_FAIRY_GREAT_BAY" | "STRAYFAIRYGREATBAY" => u32::from(self.great_bay_fairies()),
            "STRAY_FAIRY_STONE_TOWER" | "STRAYFAIRYSTONETOWER" => {
                u32::from(self.stone_tower_fairies())
            }
            "BOSS_REMAINS" | "BOSSREMAINS" => u32::from(self.boss_remains_count()),

            // Non-stackable items return 1 if owned, 0 otherwise
            _ => {
                if self.has_item(item) {
                    1
                } else {
                    0
                }
            }
        }
    }

    /// Build a complete inventory map for logic evaluation.
    ///
    /// Returns a `HashMap` mapping logic identifiers to item counts.
    /// This can be used to populate a `GameContext` from the ootmm crate.
    pub fn build_inventory(&self) -> HashMap<String, u32> {
        let mut inventory = HashMap::new();

        // Helper to add item if owned
        let mut add_if_has = |id: &str, has: bool| {
            if has {
                inventory.insert(id.to_string(), 1);
            }
        };

        // Equipment items
        add_if_has(logic_ids::OCARINA_OF_TIME, self.has_ocarina());
        add_if_has(logic_ids::HEROS_BOW, self.has_bow());
        add_if_has(logic_ids::FIRE_ARROW, self.has_fire_arrow());
        add_if_has(logic_ids::ICE_ARROW, self.has_ice_arrow());
        add_if_has(logic_ids::LIGHT_ARROW, self.has_light_arrow());
        add_if_has(logic_ids::HOOKSHOT, self.has_hookshot());
        add_if_has(logic_ids::BOMB, self.has_bombs());
        add_if_has(logic_ids::BOMBCHU, self.has_bombchu());
        add_if_has(logic_ids::POWDER_KEG, self.has_powder_keg());
        add_if_has(logic_ids::LENS_OF_TRUTH, self.has_lens_of_truth());
        add_if_has(logic_ids::PICTOGRAPH_BOX, self.has_pictograph_box());
        add_if_has(logic_ids::GREAT_FAIRY_SWORD, self.has_great_fairy_sword());
        add_if_has(logic_ids::MAGIC_BEAN, self.has_magic_bean());
        add_if_has(logic_ids::DEKU_STICK, self.has_deku_stick());
        add_if_has(logic_ids::DEKU_NUT, self.has_deku_nut());

        // Transformation masks
        add_if_has(logic_ids::DEKU_MASK, self.has_deku_mask());
        add_if_has(logic_ids::GORON_MASK, self.has_goron_mask());
        add_if_has(logic_ids::ZORA_MASK, self.has_zora_mask());
        add_if_has(logic_ids::FIERCE_DEITY_MASK, self.has_fierce_deity_mask());

        // Regular masks
        add_if_has(logic_ids::POSTMAN_HAT, self.has_postman_hat());
        add_if_has(logic_ids::ALL_NIGHT_MASK, self.has_all_night_mask());
        add_if_has(logic_ids::BLAST_MASK, self.has_blast_mask());
        add_if_has(logic_ids::STONE_MASK, self.has_stone_mask());
        add_if_has(logic_ids::GREAT_FAIRY_MASK, self.has_great_fairy_mask());
        add_if_has(logic_ids::KEATON_MASK, self.has_keaton_mask());
        add_if_has(logic_ids::BREMEN_MASK, self.has_bremen_mask());
        add_if_has(logic_ids::BUNNY_HOOD, self.has_bunny_hood());
        add_if_has(logic_ids::DON_GERO_MASK, self.has_don_gero_mask());
        add_if_has(logic_ids::MASK_OF_SCENTS, self.has_mask_of_scents());
        add_if_has(logic_ids::ROMANI_MASK, self.has_romani_mask());
        add_if_has(logic_ids::CIRCUS_LEADER_MASK, self.has_circus_leader_mask());
        add_if_has(logic_ids::KAFEI_MASK, self.has_kafei_mask());
        add_if_has(logic_ids::COUPLES_MASK, self.has_couples_mask());
        add_if_has(logic_ids::MASK_OF_TRUTH, self.has_mask_of_truth());
        add_if_has(logic_ids::KAMARO_MASK, self.has_kamaro_mask());
        add_if_has(logic_ids::GIBDO_MASK, self.has_gibdo_mask());
        add_if_has(logic_ids::GARO_MASK, self.has_garo_mask());
        add_if_has(logic_ids::CAPTAIN_HAT, self.has_captain_hat());
        add_if_has(logic_ids::GIANT_MASK, self.has_giant_mask());

        // Songs
        add_if_has(logic_ids::SONG_OF_TIME, self.has_song_of_time());
        add_if_has(logic_ids::SONG_OF_HEALING, self.has_song_of_healing());
        add_if_has(logic_ids::EPONAS_SONG, self.has_eponas_song());
        add_if_has(logic_ids::SONG_OF_SOARING, self.has_song_of_soaring());
        add_if_has(logic_ids::SONG_OF_STORMS, self.has_song_of_storms());
        add_if_has(
            logic_ids::SONATA_OF_AWAKENING,
            self.has_sonata_of_awakening(),
        );
        add_if_has(logic_ids::GORON_LULLABY, self.has_goron_lullaby());
        add_if_has(
            logic_ids::NEW_WAVE_BOSSA_NOVA,
            self.has_new_wave_bossa_nova(),
        );
        add_if_has(logic_ids::ELEGY_OF_EMPTINESS, self.has_elegy_of_emptiness());
        add_if_has(logic_ids::OATH_TO_ORDER, self.has_oath_to_order());

        // Boss remains
        add_if_has(logic_ids::ODOLWA_REMAINS, self.has_odolwa_remains());
        add_if_has(logic_ids::GOHT_REMAINS, self.has_goht_remains());
        add_if_has(logic_ids::GYORG_REMAINS, self.has_gyorg_remains());
        add_if_has(logic_ids::TWINMOLD_REMAINS, self.has_twinmold_remains());

        // Swords
        add_if_has(logic_ids::KOKIRI_SWORD, self.has_kokiri_sword());
        add_if_has(logic_ids::RAZOR_SWORD, self.has_razor_sword());
        add_if_has(logic_ids::GILDED_SWORD, self.has_gilded_sword());

        // Shields
        add_if_has(logic_ids::HERO_SHIELD, self.has_hero_shield());
        add_if_has(logic_ids::MIRROR_SHIELD, self.has_mirror_shield());

        // Upgrades
        add_if_has(logic_ids::MAGIC_METER, self.has_magic_meter());
        add_if_has(logic_ids::DOUBLE_MAGIC, self.has_double_magic());
        add_if_has(logic_ids::DOUBLE_DEFENSE, self.has_double_defense());
        add_if_has(logic_ids::ADULT_WALLET, self.has_adult_wallet());
        add_if_has(logic_ids::GIANT_WALLET, self.has_giant_wallet());

        // Quiver upgrades
        if self.quiver_level() >= 1 {
            inventory.insert(logic_ids::QUIVER_30.to_string(), 1);
        }
        if self.quiver_level() >= 2 {
            inventory.insert(logic_ids::QUIVER_40.to_string(), 1);
        }
        if self.quiver_level() >= 3 {
            inventory.insert(logic_ids::QUIVER_50.to_string(), 1);
        }

        // Bomb bag upgrades
        if self.bomb_bag_level() >= 1 {
            inventory.insert(logic_ids::BOMB_BAG_20.to_string(), 1);
        }
        if self.bomb_bag_level() >= 2 {
            inventory.insert(logic_ids::BOMB_BAG_30.to_string(), 1);
        }
        if self.bomb_bag_level() >= 3 {
            inventory.insert(logic_ids::BOMB_BAG_40.to_string(), 1);
        }

        // Stackable items with counts
        let bottle_count = self.bottle_count();
        if bottle_count > 0 {
            inventory.insert(logic_ids::BOTTLE.to_string(), u32::from(bottle_count));
        }

        // Small keys
        let wf_keys = self.woodfall_keys();
        if wf_keys > 0 {
            inventory.insert(
                logic_ids::SMALL_KEY_WOODFALL_TEMPLE.to_string(),
                u32::from(wf_keys),
            );
        }
        let sh_keys = self.snowhead_keys();
        if sh_keys > 0 {
            inventory.insert(
                logic_ids::SMALL_KEY_SNOWHEAD_TEMPLE.to_string(),
                u32::from(sh_keys),
            );
        }
        let gb_keys = self.great_bay_keys();
        if gb_keys > 0 {
            inventory.insert(
                logic_ids::SMALL_KEY_GREAT_BAY_TEMPLE.to_string(),
                u32::from(gb_keys),
            );
        }
        let st_keys = self.stone_tower_keys();
        if st_keys > 0 {
            inventory.insert(
                logic_ids::SMALL_KEY_STONE_TOWER_TEMPLE.to_string(),
                u32::from(st_keys),
            );
        }

        // Stray fairies
        let ct_fairies = self.clock_town_fairies();
        if ct_fairies > 0 {
            inventory.insert(
                logic_ids::STRAY_FAIRY_CLOCK_TOWN.to_string(),
                u32::from(ct_fairies),
            );
        }
        let wf_fairies = self.woodfall_fairies();
        if wf_fairies > 0 {
            inventory.insert(
                logic_ids::STRAY_FAIRY_WOODFALL.to_string(),
                u32::from(wf_fairies),
            );
        }
        let sh_fairies = self.snowhead_fairies();
        if sh_fairies > 0 {
            inventory.insert(
                logic_ids::STRAY_FAIRY_SNOWHEAD.to_string(),
                u32::from(sh_fairies),
            );
        }
        let gb_fairies = self.great_bay_fairies();
        if gb_fairies > 0 {
            inventory.insert(
                logic_ids::STRAY_FAIRY_GREAT_BAY.to_string(),
                u32::from(gb_fairies),
            );
        }
        let st_fairies = self.stone_tower_fairies();
        if st_fairies > 0 {
            inventory.insert(
                logic_ids::STRAY_FAIRY_STONE_TOWER.to_string(),
                u32::from(st_fairies),
            );
        }

        inventory
    }

    // ========================================================================
    // Scene Flag Helpers (Boss Defeat Events)
    // ========================================================================

    /// Check if a boss was defeated (permanent flag).
    ///
    /// Checks the permanent scene flags for the given scene ID.
    /// Returns true if the `cleared_room` field is non-zero.
    fn check_boss_defeated_permanent(&self, scene_id: usize) -> bool {
        self.save
            .permanent_scene_flags
            .get(scene_id)
            .map(|f| f.cleared_room != 0)
            .unwrap_or(false)
    }

    /// Check if a boss was defeated (cycle-scoped flag).
    ///
    /// Checks the cycle scene flags for the given scene ID.
    /// Returns true if the `cleared_room` field is non-zero.
    fn check_boss_defeated_cycle(&self, scene_id: usize) -> bool {
        self.save
            .cycle_scene_flags
            .get(scene_id)
            .map(|f| f.cleared_room != 0)
            .unwrap_or(false)
    }

    // ========================================================================
    // Scene Flag Helpers (Dungeon Clear Events)
    // ========================================================================

    /// Check if a dungeon was cleared (permanent flag).
    ///
    /// Checks the permanent scene flags for the given scene ID.
    /// Returns true if the `cleared_floors` field is non-zero.
    fn check_dungeon_clear_permanent(&self, scene_id: usize) -> bool {
        self.save
            .permanent_scene_flags
            .get(scene_id)
            .map(|f| f.cleared_floors != 0)
            .unwrap_or(false)
    }

    /// Check if a dungeon was cleared (cycle-scoped flag).
    ///
    /// Checks the cycle scene flags for the given scene ID.
    /// Returns true if the `cleared_room` field is non-zero.
    /// Note: Uses cleared_room since cycle flags don't have cleared_floors.
    fn check_dungeon_clear_cycle(&self, scene_id: usize) -> bool {
        self.save
            .cycle_scene_flags
            .get(scene_id)
            .map(|f| f.cleared_room != 0)
            .unwrap_or(false)
    }
}

// =============================================================================
// EvalContext Implementation
// =============================================================================

impl EvalContext for MmGameContext<'_> {
    /// Check if the player has at least `count` of the specified item.
    ///
    /// Uses `item_count()` to get the actual count and compares against the requested count.
    fn has_item(&self, item: &str, count: u32) -> bool {
        self.item_count(item) >= count
    }

    /// Check if a game event has occurred.
    ///
    /// Checks memory flags for game events like boss defeats and dungeon clears.
    /// Event names are case-insensitive.
    ///
    /// # Supported Events
    ///
    /// ## Boss Defeats (Permanent)
    /// These persist across Song of Time resets:
    /// - `ODOLWA_DEFEATED` - Woodfall Temple boss
    /// - `GOHT_DEFEATED` - Snowhead Temple boss
    /// - `GYORG_DEFEATED` - Great Bay Temple boss
    /// - `TWINMOLD_DEFEATED` - Stone Tower Temple boss
    ///
    /// ## Boss Defeats (Cycle-Scoped)
    /// These reset with Song of Time:
    /// - `ODOLWA_DEFEATED_CYCLE`
    /// - `GOHT_DEFEATED_CYCLE`
    /// - `GYORG_DEFEATED_CYCLE`
    /// - `TWINMOLD_DEFEATED_CYCLE`
    ///
    /// ## Dungeon Clears (Permanent)
    /// These persist across Song of Time resets:
    /// - `WOODFALL_TEMPLE_CLEAR` - Woodfall Temple completed
    /// - `SNOWHEAD_TEMPLE_CLEAR` - Snowhead Temple completed
    /// - `GREAT_BAY_TEMPLE_CLEAR` - Great Bay Temple completed
    /// - `STONE_TOWER_TEMPLE_CLEAR` - Stone Tower Temple completed
    ///
    /// ## Dungeon Clears (Cycle-Scoped)
    /// These reset with Song of Time:
    /// - `WOODFALL_TEMPLE_CLEAR_CYCLE`
    /// - `SNOWHEAD_TEMPLE_CLEAR_CYCLE`
    /// - `GREAT_BAY_TEMPLE_CLEAR_CYCLE`
    /// - `STONE_TOWER_TEMPLE_CLEAR_CYCLE`
    fn event(&self, name: &str) -> bool {
        match name.to_uppercase().as_str() {
            // Boss defeats (permanent)
            "ODOLWA_DEFEATED" => self.check_boss_defeated_permanent(0x1A),
            "GOHT_DEFEATED" => self.check_boss_defeated_permanent(0x24),
            "GYORG_DEFEATED" => self.check_boss_defeated_permanent(0x4F),
            "TWINMOLD_DEFEATED" => self.check_boss_defeated_permanent(0x36),

            // Boss defeats (cycle-scoped)
            "ODOLWA_DEFEATED_CYCLE" => self.check_boss_defeated_cycle(0x1A),
            "GOHT_DEFEATED_CYCLE" => self.check_boss_defeated_cycle(0x24),
            "GYORG_DEFEATED_CYCLE" => self.check_boss_defeated_cycle(0x4F),
            "TWINMOLD_DEFEATED_CYCLE" => self.check_boss_defeated_cycle(0x36),

            // Dungeon clears (permanent)
            "WOODFALL_TEMPLE_CLEAR" => self.check_dungeon_clear_permanent(0x1A),
            "SNOWHEAD_TEMPLE_CLEAR" => self.check_dungeon_clear_permanent(0x24),
            "GREAT_BAY_TEMPLE_CLEAR" => self.check_dungeon_clear_permanent(0x4F),
            "STONE_TOWER_TEMPLE_CLEAR" => self.check_dungeon_clear_permanent(0x36),

            // Dungeon clears (cycle-scoped)
            "WOODFALL_TEMPLE_CLEAR_CYCLE" => self.check_dungeon_clear_cycle(0x1A),
            "SNOWHEAD_TEMPLE_CLEAR_CYCLE" => self.check_dungeon_clear_cycle(0x24),
            "GREAT_BAY_TEMPLE_CLEAR_CYCLE" => self.check_dungeon_clear_cycle(0x4F),
            "STONE_TOWER_TEMPLE_CLEAR_CYCLE" => self.check_dungeon_clear_cycle(0x36),

            _ => false,
        }
    }

    /// Get the value of a setting.
    ///
    /// Currently returns None as settings are not yet implemented for MM.
    fn setting(&self, _name: &str) -> Option<bool> {
        None
    }

    /// Check if a trick is enabled.
    ///
    /// Currently returns false as tricks are not yet implemented for MM.
    fn trick(&self, _name: &str) -> bool {
        false
    }

    /// Check if the player is currently Adult Link.
    ///
    /// In Majora's Mask, Link is always a child, so this returns false.
    fn is_adult(&self) -> bool {
        false
    }

    /// Check if the player is currently Child Link.
    ///
    /// In Majora's Mask, Link is always a child, so this returns true.
    fn is_child(&self) -> bool {
        true
    }

    /// Get the current MM time as a numeric value.
    ///
    /// Currently returns 0 as time tracking is not yet implemented.
    fn mm_time(&self) -> u32 {
        0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mm_save::{
        MmCycleSceneFlags, MmInventory, MmMagicCapacity, MmMasksHigh, MmMasksLow,
        MmPermanentSceneFlags, MmQuestItems, MmShield, MmSmallKeys, MmStrayFairies, MmSword,
        MmTransformationMasks, MmUpgrades,
    };

    /// Create a default save for testing.
    fn make_save() -> MmSave {
        MmSave::default()
    }

    /// Create a save with specific items for testing.
    fn make_save_with_items() -> MmSave {
        let mut save = MmSave::default();
        save.inventory.ocarina = true;
        save.inventory.bow = true;
        save.inventory.hookshot = true;
        save.inventory.bombs = true;
        save.inventory.lens = true;
        save
    }

    // ========================================================================
    // Basic Context Tests
    // ========================================================================

    #[test]
    fn test_new_context() {
        let save = make_save();
        let ctx = MmGameContext::new(&save);
        assert!(std::ptr::eq(ctx.save(), &save));
    }

    #[test]
    fn test_empty_save_has_nothing() {
        let save = make_save();
        let ctx = MmGameContext::new(&save);

        assert!(!ctx.has_ocarina());
        assert!(!ctx.has_bow());
        assert!(!ctx.has_hookshot());
        assert!(!ctx.has_deku_mask());
        assert!(!ctx.has_song_of_time());
    }

    // ========================================================================
    // Equipment Tests
    // ========================================================================

    #[test]
    fn test_equipment_items() {
        let save = make_save_with_items();
        let ctx = MmGameContext::new(&save);

        assert!(ctx.has_ocarina());
        assert!(ctx.has_bow());
        assert!(ctx.has_hookshot());
        assert!(ctx.has_bombs());
        assert!(ctx.has_lens_of_truth());

        // Items not in save
        assert!(!ctx.has_fire_arrow());
        assert!(!ctx.has_bombchu());
        assert!(!ctx.has_powder_keg());
    }

    #[test]
    fn test_all_equipment_accessors() {
        let mut save = make_save();
        save.inventory = MmInventory {
            ocarina: true,
            bow: true,
            fire_arrows: true,
            ice_arrows: true,
            light_arrows: true,
            bombs: true,
            bombchus: true,
            deku_sticks: true,
            deku_nuts: true,
            magic_beans: true,
            powder_keg: true,
            pictograph_box: true,
            lens: true,
            hookshot: true,
            great_fairy_sword: true,
            bottles: Default::default(),
        };

        let ctx = MmGameContext::new(&save);

        assert!(ctx.has_ocarina());
        assert!(ctx.has_bow());
        assert!(ctx.has_fire_arrow());
        assert!(ctx.has_ice_arrow());
        assert!(ctx.has_light_arrow());
        assert!(ctx.has_hookshot());
        assert!(ctx.has_bombs());
        assert!(ctx.has_bombchu());
        assert!(ctx.has_powder_keg());
        assert!(ctx.has_lens_of_truth());
        assert!(ctx.has_pictograph_box());
        assert!(ctx.has_great_fairy_sword());
        assert!(ctx.has_magic_bean());
        assert!(ctx.has_deku_stick());
        assert!(ctx.has_deku_nut());
    }

    // ========================================================================
    // Transformation Mask Tests
    // ========================================================================

    #[test]
    fn test_transformation_masks() {
        let mut save = make_save();
        save.masks.transformation = MmTransformationMasks::DEKU
            | MmTransformationMasks::GORON
            | MmTransformationMasks::ZORA
            | MmTransformationMasks::FIERCE_DEITY;

        let ctx = MmGameContext::new(&save);

        assert!(ctx.has_deku_mask());
        assert!(ctx.has_goron_mask());
        assert!(ctx.has_zora_mask());
        assert!(ctx.has_fierce_deity_mask());
    }

    #[test]
    fn test_partial_transformation_masks() {
        let mut save = make_save();
        save.masks.transformation = MmTransformationMasks::DEKU | MmTransformationMasks::GORON;

        let ctx = MmGameContext::new(&save);

        assert!(ctx.has_deku_mask());
        assert!(ctx.has_goron_mask());
        assert!(!ctx.has_zora_mask());
        assert!(!ctx.has_fierce_deity_mask());
    }

    // ========================================================================
    // Regular Mask Tests
    // ========================================================================

    #[test]
    fn test_regular_masks_low() {
        let mut save = make_save();
        save.masks.masks_low = MmMasksLow::POSTMAN
            | MmMasksLow::ALL_NIGHT
            | MmMasksLow::BUNNY
            | MmMasksLow::KEATON
            | MmMasksLow::TRUTH;

        let ctx = MmGameContext::new(&save);

        assert!(ctx.has_postman_hat());
        assert!(ctx.has_all_night_mask());
        assert!(ctx.has_bunny_hood());
        assert!(ctx.has_keaton_mask());
        assert!(ctx.has_mask_of_truth());

        // Not set
        assert!(!ctx.has_blast_mask());
        assert!(!ctx.has_stone_mask());
    }

    #[test]
    fn test_regular_masks_high() {
        let mut save = make_save();
        save.masks.masks_high =
            MmMasksHigh::GIBDO | MmMasksHigh::GARO | MmMasksHigh::CAPTAIN | MmMasksHigh::GIANT;

        let ctx = MmGameContext::new(&save);

        assert!(ctx.has_gibdo_mask());
        assert!(ctx.has_garo_mask());
        assert!(ctx.has_captain_hat());
        assert!(ctx.has_giant_mask());
    }

    #[test]
    fn test_all_regular_masks() {
        let mut save = make_save();
        save.masks.masks_low = MmMasksLow::all();
        save.masks.masks_high = MmMasksHigh::all();

        let ctx = MmGameContext::new(&save);

        assert!(ctx.has_postman_hat());
        assert!(ctx.has_all_night_mask());
        assert!(ctx.has_blast_mask());
        assert!(ctx.has_stone_mask());
        assert!(ctx.has_great_fairy_mask());
        assert!(ctx.has_keaton_mask());
        assert!(ctx.has_bremen_mask());
        assert!(ctx.has_bunny_hood());
        assert!(ctx.has_don_gero_mask());
        assert!(ctx.has_mask_of_scents());
        assert!(ctx.has_romani_mask());
        assert!(ctx.has_circus_leader_mask());
        assert!(ctx.has_kafei_mask());
        assert!(ctx.has_couples_mask());
        assert!(ctx.has_mask_of_truth());
        assert!(ctx.has_kamaro_mask());
        assert!(ctx.has_gibdo_mask());
        assert!(ctx.has_garo_mask());
        assert!(ctx.has_captain_hat());
        assert!(ctx.has_giant_mask());
    }

    // ========================================================================
    // Song Tests
    // ========================================================================

    #[test]
    fn test_songs() {
        let mut save = make_save();
        save.quest_items = MmQuestItems::SONG_TIME
            | MmQuestItems::SONG_HEALING
            | MmQuestItems::SONG_SOARING
            | MmQuestItems::SONG_AWAKENING;

        let ctx = MmGameContext::new(&save);

        assert!(ctx.has_song_of_time());
        assert!(ctx.has_song_of_healing());
        assert!(ctx.has_song_of_soaring());
        assert!(ctx.has_sonata_of_awakening());

        assert!(!ctx.has_eponas_song());
        assert!(!ctx.has_song_of_storms());
        assert!(!ctx.has_goron_lullaby());
    }

    #[test]
    fn test_all_songs() {
        let mut save = make_save();
        save.quest_items = MmQuestItems::SONG_TIME
            | MmQuestItems::SONG_HEALING
            | MmQuestItems::SONG_EPONA
            | MmQuestItems::SONG_SOARING
            | MmQuestItems::SONG_STORMS
            | MmQuestItems::SONG_AWAKENING
            | MmQuestItems::SONG_GORON
            | MmQuestItems::SONG_ZORA
            | MmQuestItems::SONG_EMPTINESS
            | MmQuestItems::SONG_ORDER;

        let ctx = MmGameContext::new(&save);

        assert!(ctx.has_song_of_time());
        assert!(ctx.has_song_of_healing());
        assert!(ctx.has_eponas_song());
        assert!(ctx.has_song_of_soaring());
        assert!(ctx.has_song_of_storms());
        assert!(ctx.has_sonata_of_awakening());
        assert!(ctx.has_goron_lullaby());
        assert!(ctx.has_new_wave_bossa_nova());
        assert!(ctx.has_elegy_of_emptiness());
        assert!(ctx.has_oath_to_order());
    }

    // ========================================================================
    // Boss Remains Tests
    // ========================================================================

    #[test]
    fn test_boss_remains() {
        let mut save = make_save();
        save.quest_items = MmQuestItems::REMAINS_ODOLWA | MmQuestItems::REMAINS_GOHT;

        let ctx = MmGameContext::new(&save);

        assert!(ctx.has_odolwa_remains());
        assert!(ctx.has_goht_remains());
        assert!(!ctx.has_gyorg_remains());
        assert!(!ctx.has_twinmold_remains());
        assert_eq!(ctx.boss_remains_count(), 2);
    }

    #[test]
    fn test_all_boss_remains() {
        let mut save = make_save();
        save.quest_items = MmQuestItems::REMAINS_ODOLWA
            | MmQuestItems::REMAINS_GOHT
            | MmQuestItems::REMAINS_GYORG
            | MmQuestItems::REMAINS_TWINMOLD;

        let ctx = MmGameContext::new(&save);

        assert!(ctx.has_odolwa_remains());
        assert!(ctx.has_goht_remains());
        assert!(ctx.has_gyorg_remains());
        assert!(ctx.has_twinmold_remains());
        assert_eq!(ctx.boss_remains_count(), 4);
    }

    // ========================================================================
    // Sword/Shield Tests
    // ========================================================================

    #[test]
    fn test_swords() {
        let mut save = make_save();
        save.sword = MmSword::KokiriSword;
        let ctx = MmGameContext::new(&save);
        assert!(ctx.has_kokiri_sword());
        assert!(!ctx.has_razor_sword());
        assert!(!ctx.has_gilded_sword());

        save.sword = MmSword::RazorSword;
        let ctx = MmGameContext::new(&save);
        assert!(ctx.has_kokiri_sword()); // Progressive
        assert!(ctx.has_razor_sword());
        assert!(!ctx.has_gilded_sword());

        save.sword = MmSword::GildedSword;
        let ctx = MmGameContext::new(&save);
        assert!(ctx.has_kokiri_sword()); // Progressive
        assert!(ctx.has_razor_sword()); // Progressive
        assert!(ctx.has_gilded_sword());
    }

    #[test]
    fn test_shields() {
        let mut save = make_save();
        save.shield = MmShield::HeroShield;
        let ctx = MmGameContext::new(&save);
        assert!(ctx.has_hero_shield());
        assert!(!ctx.has_mirror_shield());

        save.shield = MmShield::MirrorShield;
        let ctx = MmGameContext::new(&save);
        assert!(ctx.has_hero_shield()); // Progressive
        assert!(ctx.has_mirror_shield());
    }

    // ========================================================================
    // Upgrade Tests
    // ========================================================================

    #[test]
    fn test_magic() {
        let mut save = make_save();
        save.magic = MmMagicCapacity::None;
        let ctx = MmGameContext::new(&save);
        assert!(!ctx.has_magic());
        assert!(!ctx.has_magic_meter());
        assert!(!ctx.has_double_magic());

        save.magic = MmMagicCapacity::Single;
        let ctx = MmGameContext::new(&save);
        assert!(ctx.has_magic());
        assert!(ctx.has_magic_meter());
        assert!(!ctx.has_double_magic());

        save.magic = MmMagicCapacity::Double;
        let ctx = MmGameContext::new(&save);
        assert!(ctx.has_magic());
        assert!(ctx.has_magic_meter());
        assert!(ctx.has_double_magic());
    }

    #[test]
    fn test_wallets() {
        let mut save = make_save();
        save.upgrades = MmUpgrades::empty();
        let ctx = MmGameContext::new(&save);
        assert_eq!(ctx.wallet_level(), 0);
        assert!(!ctx.has_adult_wallet());
        assert!(!ctx.has_giant_wallet());

        save.upgrades = MmUpgrades::ADULTS_WALLET;
        let ctx = MmGameContext::new(&save);
        assert_eq!(ctx.wallet_level(), 1);
        assert!(ctx.has_adult_wallet());
        assert!(!ctx.has_giant_wallet());

        save.upgrades = MmUpgrades::GIANTS_WALLET;
        let ctx = MmGameContext::new(&save);
        assert_eq!(ctx.wallet_level(), 2);
        assert!(ctx.has_adult_wallet());
        assert!(ctx.has_giant_wallet());
    }

    #[test]
    fn test_quiver_levels() {
        let mut save = make_save();
        save.upgrades = MmUpgrades::empty();
        let ctx = MmGameContext::new(&save);
        assert_eq!(ctx.quiver_level(), 0);

        save.upgrades = MmUpgrades::QUIVER_30;
        let ctx = MmGameContext::new(&save);
        assert_eq!(ctx.quiver_level(), 1);

        save.upgrades = MmUpgrades::QUIVER_40;
        let ctx = MmGameContext::new(&save);
        assert_eq!(ctx.quiver_level(), 2);

        save.upgrades = MmUpgrades::QUIVER_50;
        let ctx = MmGameContext::new(&save);
        assert_eq!(ctx.quiver_level(), 3);
    }

    #[test]
    fn test_bomb_bag_levels() {
        let mut save = make_save();
        save.upgrades = MmUpgrades::empty();
        let ctx = MmGameContext::new(&save);
        assert_eq!(ctx.bomb_bag_level(), 0);

        save.upgrades = MmUpgrades::BOMB_BAG_20;
        let ctx = MmGameContext::new(&save);
        assert_eq!(ctx.bomb_bag_level(), 1);

        save.upgrades = MmUpgrades::BOMB_BAG_30;
        let ctx = MmGameContext::new(&save);
        assert_eq!(ctx.bomb_bag_level(), 2);

        save.upgrades = MmUpgrades::BOMB_BAG_40;
        let ctx = MmGameContext::new(&save);
        assert_eq!(ctx.bomb_bag_level(), 3);
    }

    // ========================================================================
    // Bottle Tests
    // ========================================================================

    #[test]
    fn test_bottles() {
        let mut save = make_save();
        let ctx = MmGameContext::new(&save);
        assert_eq!(ctx.bottle_count(), 0);
        assert!(!ctx.has_bottle());

        save.inventory.bottles[0] = MmBottle::Empty;
        let ctx = MmGameContext::new(&save);
        assert_eq!(ctx.bottle_count(), 1);
        assert!(ctx.has_bottle());

        save.inventory.bottles[1] = MmBottle::RedPotion;
        save.inventory.bottles[2] = MmBottle::Fairy;
        let ctx = MmGameContext::new(&save);
        assert_eq!(ctx.bottle_count(), 3);
        assert!(ctx.has_bottle());
    }

    // ========================================================================
    // Dungeon Key Tests
    // ========================================================================

    #[test]
    fn test_small_keys() {
        let mut save = make_save();
        save.small_keys = MmSmallKeys {
            woodfall: 1,
            snowhead: 3,
            great_bay: 0,
            stone_tower: 4,
        };

        let ctx = MmGameContext::new(&save);
        assert_eq!(ctx.woodfall_keys(), 1);
        assert_eq!(ctx.snowhead_keys(), 3);
        assert_eq!(ctx.great_bay_keys(), 0);
        assert_eq!(ctx.stone_tower_keys(), 4);
    }

    // ========================================================================
    // Stray Fairy Tests
    // ========================================================================

    #[test]
    fn test_stray_fairies() {
        let mut save = make_save();
        save.stray_fairies = MmStrayFairies {
            clock_town: 1,
            woodfall: 15,
            snowhead: 10,
            great_bay: 5,
            stone_tower: 0,
        };

        let ctx = MmGameContext::new(&save);
        assert_eq!(ctx.clock_town_fairies(), 1);
        assert_eq!(ctx.woodfall_fairies(), 15);
        assert_eq!(ctx.snowhead_fairies(), 10);
        assert_eq!(ctx.great_bay_fairies(), 5);
        assert_eq!(ctx.stone_tower_fairies(), 0);
    }

    // ========================================================================
    // has_item Tests (String Lookup)
    // ========================================================================

    #[test]
    fn test_has_item_equipment() {
        let save = make_save_with_items();
        let ctx = MmGameContext::new(&save);

        // Different case formats
        assert!(ctx.has_item("HOOKSHOT"));
        assert!(ctx.has_item("hookshot"));
        assert!(ctx.has_item("Hookshot"));
        assert!(ctx.has_item("HEROS_BOW"));
        assert!(ctx.has_item("heros_bow"));
        assert!(ctx.has_item("HEROSBOW"));

        // Missing items
        assert!(!ctx.has_item("FIRE_ARROW"));
        assert!(!ctx.has_item("POWDER_KEG"));
    }

    #[test]
    fn test_has_item_masks() {
        let mut save = make_save();
        save.masks.transformation = MmTransformationMasks::DEKU | MmTransformationMasks::GORON;
        save.masks.masks_low = MmMasksLow::BUNNY | MmMasksLow::KEATON;

        let ctx = MmGameContext::new(&save);

        assert!(ctx.has_item("DEKU_MASK"));
        assert!(ctx.has_item("DEKUMASK"));
        assert!(ctx.has_item("deku_mask"));
        assert!(ctx.has_item("GORON_MASK"));
        assert!(ctx.has_item("BUNNY_HOOD"));
        assert!(ctx.has_item("KEATON_MASK"));

        assert!(!ctx.has_item("ZORA_MASK"));
        assert!(!ctx.has_item("STONE_MASK"));
    }

    #[test]
    fn test_has_item_songs() {
        let mut save = make_save();
        save.quest_items = MmQuestItems::SONG_TIME | MmQuestItems::SONG_SOARING;

        let ctx = MmGameContext::new(&save);

        assert!(ctx.has_item("SONG_OF_TIME"));
        assert!(ctx.has_item("SONGOFTIME"));
        assert!(ctx.has_item("SONG_OF_SOARING"));

        assert!(!ctx.has_item("SONG_OF_HEALING"));
        assert!(!ctx.has_item("EPONAS_SONG"));
    }

    #[test]
    fn test_has_item_boss_remains() {
        let mut save = make_save();
        save.quest_items = MmQuestItems::REMAINS_ODOLWA | MmQuestItems::REMAINS_GYORG;

        let ctx = MmGameContext::new(&save);

        assert!(ctx.has_item("ODOLWA_REMAINS"));
        assert!(ctx.has_item("GYORG_REMAINS"));
        assert!(!ctx.has_item("GOHT_REMAINS"));
        assert!(!ctx.has_item("TWINMOLD_REMAINS"));
    }

    #[test]
    fn test_has_item_unknown() {
        let save = make_save();
        let ctx = MmGameContext::new(&save);

        assert!(!ctx.has_item("NOT_AN_ITEM"));
        assert!(!ctx.has_item("INVALID"));
        assert!(!ctx.has_item(""));
    }

    // ========================================================================
    // item_count Tests
    // ========================================================================

    #[test]
    fn test_item_count_non_stackable() {
        let save = make_save_with_items();
        let ctx = MmGameContext::new(&save);

        assert_eq!(ctx.item_count("HOOKSHOT"), 1);
        assert_eq!(ctx.item_count("HEROS_BOW"), 1);
        assert_eq!(ctx.item_count("FIRE_ARROW"), 0);
    }

    #[test]
    fn test_item_count_bottles() {
        let mut save = make_save();
        save.inventory.bottles[0] = MmBottle::Empty;
        save.inventory.bottles[1] = MmBottle::Fairy;

        let ctx = MmGameContext::new(&save);
        assert_eq!(ctx.item_count("BOTTLE"), 2);
    }

    #[test]
    fn test_item_count_keys() {
        let mut save = make_save();
        save.small_keys = MmSmallKeys {
            woodfall: 1,
            snowhead: 3,
            great_bay: 1,
            stone_tower: 4,
        };

        let ctx = MmGameContext::new(&save);
        assert_eq!(ctx.item_count("SMALL_KEY_WOODFALL_TEMPLE"), 1);
        assert_eq!(ctx.item_count("SMALL_KEY_SNOWHEAD_TEMPLE"), 3);
        assert_eq!(ctx.item_count("SMALL_KEY_GREAT_BAY_TEMPLE"), 1);
        assert_eq!(ctx.item_count("SMALL_KEY_STONE_TOWER_TEMPLE"), 4);
    }

    #[test]
    fn test_item_count_fairies() {
        let mut save = make_save();
        save.stray_fairies = MmStrayFairies {
            clock_town: 1,
            woodfall: 15,
            snowhead: 10,
            great_bay: 5,
            stone_tower: 0,
        };

        let ctx = MmGameContext::new(&save);
        assert_eq!(ctx.item_count("STRAY_FAIRY_CLOCK_TOWN"), 1);
        assert_eq!(ctx.item_count("STRAY_FAIRY_WOODFALL"), 15);
        assert_eq!(ctx.item_count("STRAY_FAIRY_SNOWHEAD"), 10);
        assert_eq!(ctx.item_count("STRAY_FAIRY_GREAT_BAY"), 5);
        assert_eq!(ctx.item_count("STRAY_FAIRY_STONE_TOWER"), 0);
    }

    #[test]
    fn test_item_count_boss_remains() {
        let mut save = make_save();
        save.quest_items =
            MmQuestItems::REMAINS_ODOLWA | MmQuestItems::REMAINS_GOHT | MmQuestItems::REMAINS_GYORG;

        let ctx = MmGameContext::new(&save);
        assert_eq!(ctx.item_count("BOSS_REMAINS"), 3);
    }

    // ========================================================================
    // build_inventory Tests
    // ========================================================================

    #[test]
    fn test_build_inventory_empty() {
        let save = make_save();
        let ctx = MmGameContext::new(&save);
        let inventory = ctx.build_inventory();

        assert!(inventory.is_empty());
    }

    #[test]
    fn test_build_inventory_with_items() {
        let mut save = make_save();
        save.inventory.ocarina = true;
        save.inventory.hookshot = true;
        save.masks.transformation = MmTransformationMasks::DEKU;
        save.quest_items = MmQuestItems::SONG_TIME;
        save.inventory.bottles[0] = MmBottle::Empty;
        save.inventory.bottles[1] = MmBottle::Fairy;
        save.small_keys.woodfall = 1;
        save.stray_fairies.woodfall = 15;

        let ctx = MmGameContext::new(&save);
        let inventory = ctx.build_inventory();

        assert_eq!(inventory.get(logic_ids::OCARINA_OF_TIME), Some(&1));
        assert_eq!(inventory.get(logic_ids::HOOKSHOT), Some(&1));
        assert_eq!(inventory.get(logic_ids::DEKU_MASK), Some(&1));
        assert_eq!(inventory.get(logic_ids::SONG_OF_TIME), Some(&1));
        assert_eq!(inventory.get(logic_ids::BOTTLE), Some(&2));
        assert_eq!(
            inventory.get(logic_ids::SMALL_KEY_WOODFALL_TEMPLE),
            Some(&1)
        );
        assert_eq!(inventory.get(logic_ids::STRAY_FAIRY_WOODFALL), Some(&15));

        // Items not owned should not be in inventory
        assert!(!inventory.contains_key(logic_ids::FIRE_ARROW));
        assert!(!inventory.contains_key(logic_ids::GORON_MASK));
    }

    #[test]
    fn test_build_inventory_upgrades() {
        let mut save = make_save();
        save.magic = MmMagicCapacity::Double;
        save.double_defense = true;
        save.upgrades = MmUpgrades::ADULTS_WALLET | MmUpgrades::QUIVER_40 | MmUpgrades::BOMB_BAG_30;

        let ctx = MmGameContext::new(&save);
        let inventory = ctx.build_inventory();

        assert_eq!(inventory.get(logic_ids::MAGIC_METER), Some(&1));
        assert_eq!(inventory.get(logic_ids::DOUBLE_MAGIC), Some(&1));
        assert_eq!(inventory.get(logic_ids::DOUBLE_DEFENSE), Some(&1));
        assert_eq!(inventory.get(logic_ids::ADULT_WALLET), Some(&1));
        assert_eq!(inventory.get(logic_ids::QUIVER_30), Some(&1));
        assert_eq!(inventory.get(logic_ids::QUIVER_40), Some(&1));
        assert!(!inventory.contains_key(logic_ids::QUIVER_50));
        assert_eq!(inventory.get(logic_ids::BOMB_BAG_20), Some(&1));
        assert_eq!(inventory.get(logic_ids::BOMB_BAG_30), Some(&1));
        assert!(!inventory.contains_key(logic_ids::BOMB_BAG_40));
    }

    #[test]
    fn test_build_inventory_all_masks() {
        let mut save = make_save();
        save.masks.transformation = MmTransformationMasks::all();
        save.masks.masks_low = MmMasksLow::all();
        save.masks.masks_high = MmMasksHigh::all();

        let ctx = MmGameContext::new(&save);
        let inventory = ctx.build_inventory();

        // Verify all masks are in inventory
        assert!(inventory.contains_key(logic_ids::DEKU_MASK));
        assert!(inventory.contains_key(logic_ids::GORON_MASK));
        assert!(inventory.contains_key(logic_ids::ZORA_MASK));
        assert!(inventory.contains_key(logic_ids::FIERCE_DEITY_MASK));
        assert!(inventory.contains_key(logic_ids::POSTMAN_HAT));
        assert!(inventory.contains_key(logic_ids::ALL_NIGHT_MASK));
        assert!(inventory.contains_key(logic_ids::BLAST_MASK));
        assert!(inventory.contains_key(logic_ids::BUNNY_HOOD));
        assert!(inventory.contains_key(logic_ids::GIBDO_MASK));
        assert!(inventory.contains_key(logic_ids::GARO_MASK));
        assert!(inventory.contains_key(logic_ids::GIANT_MASK));
    }

    #[test]
    fn test_build_inventory_all_songs() {
        let mut save = make_save();
        save.quest_items = MmQuestItems::SONG_TIME
            | MmQuestItems::SONG_HEALING
            | MmQuestItems::SONG_EPONA
            | MmQuestItems::SONG_SOARING
            | MmQuestItems::SONG_STORMS
            | MmQuestItems::SONG_AWAKENING
            | MmQuestItems::SONG_GORON
            | MmQuestItems::SONG_ZORA
            | MmQuestItems::SONG_EMPTINESS
            | MmQuestItems::SONG_ORDER;

        let ctx = MmGameContext::new(&save);
        let inventory = ctx.build_inventory();

        assert!(inventory.contains_key(logic_ids::SONG_OF_TIME));
        assert!(inventory.contains_key(logic_ids::SONG_OF_HEALING));
        assert!(inventory.contains_key(logic_ids::EPONAS_SONG));
        assert!(inventory.contains_key(logic_ids::SONG_OF_SOARING));
        assert!(inventory.contains_key(logic_ids::SONG_OF_STORMS));
        assert!(inventory.contains_key(logic_ids::SONATA_OF_AWAKENING));
        assert!(inventory.contains_key(logic_ids::GORON_LULLABY));
        assert!(inventory.contains_key(logic_ids::NEW_WAVE_BOSSA_NOVA));
        assert!(inventory.contains_key(logic_ids::ELEGY_OF_EMPTINESS));
        assert!(inventory.contains_key(logic_ids::OATH_TO_ORDER));
    }

    // ========================================================================
    // Logic IDs Module Tests
    // ========================================================================

    #[test]
    fn test_logic_ids_exist() {
        // Verify all logic IDs are non-empty strings
        // Note: These are compile-time constants, so the assertions are always true
        let ids: [&str; 4] = [
            logic_ids::OCARINA_OF_TIME,
            logic_ids::DEKU_MASK,
            logic_ids::SONG_OF_TIME,
            logic_ids::HOOKSHOT,
        ];
        for id in ids {
            assert!(!id.is_empty());
        }
    }

    // ========================================================================
    // EvalContext Implementation Tests
    // ========================================================================

    #[test]
    fn test_eval_context_has_item_bottles_with_count() {
        let mut save = make_save();
        // Add 3 bottles
        save.inventory.bottles[0] = MmBottle::Empty;
        save.inventory.bottles[1] = MmBottle::RedPotion;
        save.inventory.bottles[2] = MmBottle::Fairy;

        let ctx = MmGameContext::new(&save);

        // Test via EvalContext trait method
        assert!(EvalContext::has_item(&ctx, "BOTTLE", 1));
        assert!(EvalContext::has_item(&ctx, "BOTTLE", 2));
        assert!(EvalContext::has_item(&ctx, "BOTTLE", 3));
        assert!(!EvalContext::has_item(&ctx, "BOTTLE", 4));
    }

    #[test]
    fn test_eval_context_has_item_hookshot() {
        let mut save = make_save();
        save.inventory.hookshot = true;

        let ctx = MmGameContext::new(&save);

        // Has hookshot with count 1 should return true
        assert!(EvalContext::has_item(&ctx, "HOOKSHOT", 1));
        // Non-stackable item with count 2 should return false
        assert!(!EvalContext::has_item(&ctx, "HOOKSHOT", 2));
    }

    #[test]
    fn test_eval_context_has_item_missing() {
        let save = make_save();
        let ctx = MmGameContext::new(&save);

        assert!(!EvalContext::has_item(&ctx, "HOOKSHOT", 1));
        assert!(!EvalContext::has_item(&ctx, "BOTTLE", 1));
    }

    #[test]
    fn test_eval_context_has_item_keys() {
        let mut save = make_save();
        save.small_keys.woodfall = 2;
        save.small_keys.snowhead = 3;

        let ctx = MmGameContext::new(&save);

        assert!(EvalContext::has_item(&ctx, "SMALL_KEY_WOODFALL_TEMPLE", 1));
        assert!(EvalContext::has_item(&ctx, "SMALL_KEY_WOODFALL_TEMPLE", 2));
        assert!(!EvalContext::has_item(&ctx, "SMALL_KEY_WOODFALL_TEMPLE", 3));

        assert!(EvalContext::has_item(&ctx, "SMALL_KEY_SNOWHEAD_TEMPLE", 3));
        assert!(!EvalContext::has_item(&ctx, "SMALL_KEY_SNOWHEAD_TEMPLE", 4));
    }

    #[test]
    fn test_eval_context_has_item_stray_fairies() {
        let mut save = make_save();
        save.stray_fairies.woodfall = 15;

        let ctx = MmGameContext::new(&save);

        assert!(EvalContext::has_item(&ctx, "STRAY_FAIRY_WOODFALL", 1));
        assert!(EvalContext::has_item(&ctx, "STRAY_FAIRY_WOODFALL", 15));
        assert!(!EvalContext::has_item(&ctx, "STRAY_FAIRY_WOODFALL", 16));
    }

    #[test]
    fn test_eval_context_is_child() {
        let save = make_save();
        let ctx = MmGameContext::new(&save);

        // In MM, Link is always a child
        assert!(EvalContext::is_child(&ctx));
        assert!(!EvalContext::is_adult(&ctx));
    }

    #[test]
    fn test_eval_context_setting_trick_stub() {
        let save = make_save();
        let ctx = MmGameContext::new(&save);

        // Stub implementations
        assert_eq!(EvalContext::setting(&ctx, "any_setting"), None);
        assert!(!EvalContext::trick(&ctx, "any_trick"));
    }

    #[test]
    fn test_eval_context_mm_time_stub() {
        let save = make_save();
        let ctx = MmGameContext::new(&save);

        assert_eq!(EvalContext::mm_time(&ctx), 0);
    }

    #[test]
    fn test_eval_context_is_day_is_night() {
        let save = make_save();
        let ctx = MmGameContext::new(&save);

        // With mm_time = 0 (default), is_day should be true
        assert!(EvalContext::is_day(&ctx));
        assert!(!EvalContext::is_night(&ctx));
    }

    #[test]
    fn test_evalcontext_setting_returns_none() {
        let save = make_save();
        let ctx = MmGameContext::new(&save);

        // setting() should return None for any setting (stub implementation)
        assert_eq!(ctx.setting("any_setting"), None);
        assert_eq!(ctx.setting("shuffle_songs"), None);
        assert_eq!(ctx.setting("open_forest"), None);
        assert_eq!(ctx.setting(""), None);
        assert_eq!(ctx.setting("UPPERCASE_SETTING"), None);
    }

    #[test]
    fn test_evalcontext_trick_returns_false() {
        let save = make_save();
        let ctx = MmGameContext::new(&save);

        // trick() should return false for any trick (stub implementation)
        assert!(!ctx.trick("any_trick"));
        assert!(!ctx.trick("hover_boost"));
        assert!(!ctx.trick("bomb_hover"));
        assert!(!ctx.trick(""));
        assert!(!ctx.trick("UPPERCASE_TRICK"));
    }

    #[test]
    fn test_evalcontext_no_panic_on_unknown_settings_tricks() {
        let save = make_save();
        let ctx = MmGameContext::new(&save);

        // Should not panic on any input
        let _ = ctx.setting("!@#$%^&*()");
        let _ = ctx.setting("very_long_setting_name_that_definitely_does_not_exist");
        let _ = ctx.trick("!@#$%^&*()");
        let _ = ctx.trick("very_long_trick_name_that_definitely_does_not_exist");
    }

    // ========================================================================
    // Event Tests (Boss Defeats)
    // ========================================================================

    /// Create a permanent scene flag with cleared_room set.
    fn make_perm_flag_cleared() -> MmPermanentSceneFlags {
        MmPermanentSceneFlags {
            chest: 0,
            switch0: 0,
            switch1: 0,
            cleared_room: 1,
            collectible: 0,
            cleared_floors: 0,
            rooms: 0,
        }
    }

    /// Create an empty permanent scene flag.
    fn make_perm_flag_empty() -> MmPermanentSceneFlags {
        MmPermanentSceneFlags {
            chest: 0,
            switch0: 0,
            switch1: 0,
            cleared_room: 0,
            collectible: 0,
            cleared_floors: 0,
            rooms: 0,
        }
    }

    /// Create a cycle scene flag with cleared_room set.
    fn make_cycle_flag_cleared() -> MmCycleSceneFlags {
        MmCycleSceneFlags {
            chest: 0,
            switch0: 0,
            switch1: 0,
            cleared_room: 1,
            collectible: 0,
        }
    }

    /// Create an empty cycle scene flag.
    fn make_cycle_flag_empty() -> MmCycleSceneFlags {
        MmCycleSceneFlags {
            chest: 0,
            switch0: 0,
            switch1: 0,
            cleared_room: 0,
            collectible: 0,
        }
    }

    /// Create a save with enough scene flag slots for testing.
    fn make_save_with_scene_flags(
        perm_flags: Vec<MmPermanentSceneFlags>,
        cycle_flags: Vec<MmCycleSceneFlags>,
    ) -> MmSave {
        MmSave {
            permanent_scene_flags: perm_flags,
            cycle_scene_flags: cycle_flags,
            ..Default::default()
        }
    }

    #[test]
    fn test_event_odolwa_defeated_permanent() {
        // Scene ID 0x1A = 26
        let mut perm_flags: Vec<MmPermanentSceneFlags> =
            (0..27).map(|_| make_perm_flag_empty()).collect();
        perm_flags[0x1A] = make_perm_flag_cleared();

        let save = make_save_with_scene_flags(perm_flags, vec![]);
        let ctx = MmGameContext::new(&save);

        assert!(EvalContext::event(&ctx, "ODOLWA_DEFEATED"));
        assert!(!EvalContext::event(&ctx, "GOHT_DEFEATED"));
    }

    #[test]
    fn test_event_goht_defeated_permanent() {
        // Scene ID 0x24 = 36
        let mut perm_flags: Vec<MmPermanentSceneFlags> =
            (0..37).map(|_| make_perm_flag_empty()).collect();
        perm_flags[0x24] = make_perm_flag_cleared();

        let save = make_save_with_scene_flags(perm_flags, vec![]);
        let ctx = MmGameContext::new(&save);

        assert!(EvalContext::event(&ctx, "GOHT_DEFEATED"));
        assert!(!EvalContext::event(&ctx, "ODOLWA_DEFEATED"));
    }

    #[test]
    fn test_event_gyorg_defeated_permanent() {
        // Scene ID 0x4F = 79
        let mut perm_flags: Vec<MmPermanentSceneFlags> =
            (0..80).map(|_| make_perm_flag_empty()).collect();
        perm_flags[0x4F] = make_perm_flag_cleared();

        let save = make_save_with_scene_flags(perm_flags, vec![]);
        let ctx = MmGameContext::new(&save);

        assert!(EvalContext::event(&ctx, "GYORG_DEFEATED"));
    }

    #[test]
    fn test_event_twinmold_defeated_permanent() {
        // Scene ID 0x36 = 54
        let mut perm_flags: Vec<MmPermanentSceneFlags> =
            (0..55).map(|_| make_perm_flag_empty()).collect();
        perm_flags[0x36] = make_perm_flag_cleared();

        let save = make_save_with_scene_flags(perm_flags, vec![]);
        let ctx = MmGameContext::new(&save);

        assert!(EvalContext::event(&ctx, "TWINMOLD_DEFEATED"));
    }

    #[test]
    fn test_event_goht_defeated_cycle() {
        // Scene ID 0x24 = 36
        let mut cycle_flags: Vec<MmCycleSceneFlags> =
            (0..37).map(|_| make_cycle_flag_empty()).collect();
        cycle_flags[0x24] = make_cycle_flag_cleared();

        let save = make_save_with_scene_flags(vec![], cycle_flags);
        let ctx = MmGameContext::new(&save);

        assert!(EvalContext::event(&ctx, "GOHT_DEFEATED_CYCLE"));
        // Permanent flag not set
        assert!(!EvalContext::event(&ctx, "GOHT_DEFEATED"));
    }

    #[test]
    fn test_event_all_cycle_bosses() {
        // Create enough flags for all bosses
        let mut cycle_flags: Vec<MmCycleSceneFlags> =
            (0..80).map(|_| make_cycle_flag_empty()).collect();
        cycle_flags[0x1A] = make_cycle_flag_cleared(); // Odolwa
        cycle_flags[0x24] = make_cycle_flag_cleared(); // Goht
        cycle_flags[0x4F] = make_cycle_flag_cleared(); // Gyorg
        cycle_flags[0x36] = make_cycle_flag_cleared(); // Twinmold

        let save = make_save_with_scene_flags(vec![], cycle_flags);
        let ctx = MmGameContext::new(&save);

        assert!(EvalContext::event(&ctx, "ODOLWA_DEFEATED_CYCLE"));
        assert!(EvalContext::event(&ctx, "GOHT_DEFEATED_CYCLE"));
        assert!(EvalContext::event(&ctx, "GYORG_DEFEATED_CYCLE"));
        assert!(EvalContext::event(&ctx, "TWINMOLD_DEFEATED_CYCLE"));
    }

    #[test]
    fn test_event_case_insensitive() {
        let mut perm_flags: Vec<MmPermanentSceneFlags> =
            (0..27).map(|_| make_perm_flag_empty()).collect();
        perm_flags[0x1A] = make_perm_flag_cleared();

        let save = make_save_with_scene_flags(perm_flags, vec![]);
        let ctx = MmGameContext::new(&save);

        // All case variations should work
        assert!(EvalContext::event(&ctx, "ODOLWA_DEFEATED"));
        assert!(EvalContext::event(&ctx, "odolwa_defeated"));
        assert!(EvalContext::event(&ctx, "Odolwa_Defeated"));
        assert!(EvalContext::event(&ctx, "OdOlWa_DeFeAtEd"));
    }

    #[test]
    fn test_event_unknown_returns_false() {
        let save = make_save();
        let ctx = MmGameContext::new(&save);

        assert!(!EvalContext::event(&ctx, "UNKNOWN_EVENT"));
        assert!(!EvalContext::event(&ctx, "NOT_A_BOSS"));
        assert!(!EvalContext::event(&ctx, ""));
        assert!(!EvalContext::event(&ctx, "MAJORA_DEFEATED"));
    }

    #[test]
    fn test_event_empty_flags_returns_false() {
        // Empty flags should return false for all events
        let save = make_save();
        let ctx = MmGameContext::new(&save);

        assert!(!EvalContext::event(&ctx, "ODOLWA_DEFEATED"));
        assert!(!EvalContext::event(&ctx, "GOHT_DEFEATED"));
        assert!(!EvalContext::event(&ctx, "GYORG_DEFEATED"));
        assert!(!EvalContext::event(&ctx, "TWINMOLD_DEFEATED"));
        assert!(!EvalContext::event(&ctx, "ODOLWA_DEFEATED_CYCLE"));
        assert!(!EvalContext::event(&ctx, "GOHT_DEFEATED_CYCLE"));
        assert!(!EvalContext::event(&ctx, "GYORG_DEFEATED_CYCLE"));
        assert!(!EvalContext::event(&ctx, "TWINMOLD_DEFEATED_CYCLE"));
    }

    #[test]
    fn test_event_permanent_vs_cycle_independence() {
        // Permanent flag set, cycle not set
        let mut perm_flags: Vec<MmPermanentSceneFlags> =
            (0..80).map(|_| make_perm_flag_empty()).collect();
        perm_flags[0x1A] = make_perm_flag_cleared();

        let cycle_flags: Vec<MmCycleSceneFlags> =
            (0..80).map(|_| make_cycle_flag_empty()).collect();

        let save = make_save_with_scene_flags(perm_flags, cycle_flags);
        let ctx = MmGameContext::new(&save);

        // Permanent should be true, cycle should be false
        assert!(EvalContext::event(&ctx, "ODOLWA_DEFEATED"));
        assert!(!EvalContext::event(&ctx, "ODOLWA_DEFEATED_CYCLE"));
    }

    #[test]
    fn test_check_boss_defeated_permanent_helper() {
        let mut perm_flags: Vec<MmPermanentSceneFlags> =
            (0..80).map(|_| make_perm_flag_empty()).collect();
        perm_flags[0x1A] = make_perm_flag_cleared();

        let save = make_save_with_scene_flags(perm_flags, vec![]);
        let ctx = MmGameContext::new(&save);

        assert!(ctx.check_boss_defeated_permanent(0x1A));
        assert!(!ctx.check_boss_defeated_permanent(0x24));
        assert!(!ctx.check_boss_defeated_permanent(0x4F));
        assert!(!ctx.check_boss_defeated_permanent(0x36));
    }

    #[test]
    fn test_check_boss_defeated_cycle_helper() {
        let mut cycle_flags: Vec<MmCycleSceneFlags> =
            (0..80).map(|_| make_cycle_flag_empty()).collect();
        cycle_flags[0x24] = make_cycle_flag_cleared();

        let save = make_save_with_scene_flags(vec![], cycle_flags);
        let ctx = MmGameContext::new(&save);

        assert!(!ctx.check_boss_defeated_cycle(0x1A));
        assert!(ctx.check_boss_defeated_cycle(0x24));
        assert!(!ctx.check_boss_defeated_cycle(0x4F));
        assert!(!ctx.check_boss_defeated_cycle(0x36));
    }

    #[test]
    fn test_check_boss_defeated_out_of_bounds() {
        let save = make_save();
        let ctx = MmGameContext::new(&save);

        // Out of bounds should return false, not panic
        assert!(!ctx.check_boss_defeated_permanent(1000));
        assert!(!ctx.check_boss_defeated_cycle(1000));
    }

    // ========================================================================
    // Event Tests (Dungeon Clears)
    // ========================================================================

    /// Create a permanent scene flag with cleared_floors set.
    fn make_perm_flag_dungeon_cleared() -> MmPermanentSceneFlags {
        MmPermanentSceneFlags {
            chest: 0,
            switch0: 0,
            switch1: 0,
            cleared_room: 0,
            collectible: 0,
            cleared_floors: 1,
            rooms: 0,
        }
    }

    #[test]
    fn test_event_woodfall_temple_clear_permanent() {
        // Scene ID 0x1A = 26
        let mut perm_flags: Vec<MmPermanentSceneFlags> =
            (0..27).map(|_| make_perm_flag_empty()).collect();
        perm_flags[0x1A] = make_perm_flag_dungeon_cleared();

        let save = make_save_with_scene_flags(perm_flags, vec![]);
        let ctx = MmGameContext::new(&save);

        assert!(EvalContext::event(&ctx, "WOODFALL_TEMPLE_CLEAR"));
        assert!(!EvalContext::event(&ctx, "SNOWHEAD_TEMPLE_CLEAR"));
    }

    #[test]
    fn test_event_snowhead_temple_clear_permanent() {
        // Scene ID 0x24 = 36
        let mut perm_flags: Vec<MmPermanentSceneFlags> =
            (0..37).map(|_| make_perm_flag_empty()).collect();
        perm_flags[0x24] = make_perm_flag_dungeon_cleared();

        let save = make_save_with_scene_flags(perm_flags, vec![]);
        let ctx = MmGameContext::new(&save);

        assert!(EvalContext::event(&ctx, "SNOWHEAD_TEMPLE_CLEAR"));
        assert!(!EvalContext::event(&ctx, "WOODFALL_TEMPLE_CLEAR"));
    }

    #[test]
    fn test_event_great_bay_temple_clear_permanent() {
        // Scene ID 0x4F = 79
        let mut perm_flags: Vec<MmPermanentSceneFlags> =
            (0..80).map(|_| make_perm_flag_empty()).collect();
        perm_flags[0x4F] = make_perm_flag_dungeon_cleared();

        let save = make_save_with_scene_flags(perm_flags, vec![]);
        let ctx = MmGameContext::new(&save);

        assert!(EvalContext::event(&ctx, "GREAT_BAY_TEMPLE_CLEAR"));
    }

    #[test]
    fn test_event_stone_tower_temple_clear_permanent() {
        // Scene ID 0x36 = 54
        let mut perm_flags: Vec<MmPermanentSceneFlags> =
            (0..55).map(|_| make_perm_flag_empty()).collect();
        perm_flags[0x36] = make_perm_flag_dungeon_cleared();

        let save = make_save_with_scene_flags(perm_flags, vec![]);
        let ctx = MmGameContext::new(&save);

        assert!(EvalContext::event(&ctx, "STONE_TOWER_TEMPLE_CLEAR"));
    }

    #[test]
    fn test_event_all_cycle_dungeon_clears() {
        // Create enough flags for all dungeons
        let mut cycle_flags: Vec<MmCycleSceneFlags> =
            (0..80).map(|_| make_cycle_flag_empty()).collect();
        cycle_flags[0x1A] = make_cycle_flag_cleared(); // Woodfall
        cycle_flags[0x24] = make_cycle_flag_cleared(); // Snowhead
        cycle_flags[0x4F] = make_cycle_flag_cleared(); // Great Bay
        cycle_flags[0x36] = make_cycle_flag_cleared(); // Stone Tower

        let save = make_save_with_scene_flags(vec![], cycle_flags);
        let ctx = MmGameContext::new(&save);

        assert!(EvalContext::event(&ctx, "WOODFALL_TEMPLE_CLEAR_CYCLE"));
        assert!(EvalContext::event(&ctx, "SNOWHEAD_TEMPLE_CLEAR_CYCLE"));
        assert!(EvalContext::event(&ctx, "GREAT_BAY_TEMPLE_CLEAR_CYCLE"));
        assert!(EvalContext::event(&ctx, "STONE_TOWER_TEMPLE_CLEAR_CYCLE"));
    }

    #[test]
    fn test_event_dungeon_clear_case_insensitive() {
        let mut perm_flags: Vec<MmPermanentSceneFlags> =
            (0..27).map(|_| make_perm_flag_empty()).collect();
        perm_flags[0x1A] = make_perm_flag_dungeon_cleared();

        let save = make_save_with_scene_flags(perm_flags, vec![]);
        let ctx = MmGameContext::new(&save);

        // All case variations should work
        assert!(EvalContext::event(&ctx, "WOODFALL_TEMPLE_CLEAR"));
        assert!(EvalContext::event(&ctx, "woodfall_temple_clear"));
        assert!(EvalContext::event(&ctx, "Woodfall_Temple_Clear"));
        assert!(EvalContext::event(&ctx, "WoOdFaLl_TeMpLe_ClEaR"));
    }

    #[test]
    fn test_event_dungeon_clear_empty_flags_returns_false() {
        // Empty flags should return false for all dungeon clear events
        let save = make_save();
        let ctx = MmGameContext::new(&save);

        assert!(!EvalContext::event(&ctx, "WOODFALL_TEMPLE_CLEAR"));
        assert!(!EvalContext::event(&ctx, "SNOWHEAD_TEMPLE_CLEAR"));
        assert!(!EvalContext::event(&ctx, "GREAT_BAY_TEMPLE_CLEAR"));
        assert!(!EvalContext::event(&ctx, "STONE_TOWER_TEMPLE_CLEAR"));
        assert!(!EvalContext::event(&ctx, "WOODFALL_TEMPLE_CLEAR_CYCLE"));
        assert!(!EvalContext::event(&ctx, "SNOWHEAD_TEMPLE_CLEAR_CYCLE"));
        assert!(!EvalContext::event(&ctx, "GREAT_BAY_TEMPLE_CLEAR_CYCLE"));
        assert!(!EvalContext::event(&ctx, "STONE_TOWER_TEMPLE_CLEAR_CYCLE"));
    }

    #[test]
    fn test_event_dungeon_clear_permanent_vs_cycle_independence() {
        // Permanent flag set (cleared_floors), cycle not set
        let mut perm_flags: Vec<MmPermanentSceneFlags> =
            (0..80).map(|_| make_perm_flag_empty()).collect();
        perm_flags[0x1A] = make_perm_flag_dungeon_cleared();

        let cycle_flags: Vec<MmCycleSceneFlags> =
            (0..80).map(|_| make_cycle_flag_empty()).collect();

        let save = make_save_with_scene_flags(perm_flags, cycle_flags);
        let ctx = MmGameContext::new(&save);

        // Permanent should be true, cycle should be false
        assert!(EvalContext::event(&ctx, "WOODFALL_TEMPLE_CLEAR"));
        assert!(!EvalContext::event(&ctx, "WOODFALL_TEMPLE_CLEAR_CYCLE"));
    }

    #[test]
    fn test_check_dungeon_clear_permanent_helper() {
        let mut perm_flags: Vec<MmPermanentSceneFlags> =
            (0..80).map(|_| make_perm_flag_empty()).collect();
        perm_flags[0x1A] = make_perm_flag_dungeon_cleared();

        let save = make_save_with_scene_flags(perm_flags, vec![]);
        let ctx = MmGameContext::new(&save);

        assert!(ctx.check_dungeon_clear_permanent(0x1A));
        assert!(!ctx.check_dungeon_clear_permanent(0x24));
        assert!(!ctx.check_dungeon_clear_permanent(0x4F));
        assert!(!ctx.check_dungeon_clear_permanent(0x36));
    }

    #[test]
    fn test_check_dungeon_clear_cycle_helper() {
        let mut cycle_flags: Vec<MmCycleSceneFlags> =
            (0..80).map(|_| make_cycle_flag_empty()).collect();
        cycle_flags[0x24] = make_cycle_flag_cleared();

        let save = make_save_with_scene_flags(vec![], cycle_flags);
        let ctx = MmGameContext::new(&save);

        assert!(!ctx.check_dungeon_clear_cycle(0x1A));
        assert!(ctx.check_dungeon_clear_cycle(0x24));
        assert!(!ctx.check_dungeon_clear_cycle(0x4F));
        assert!(!ctx.check_dungeon_clear_cycle(0x36));
    }

    #[test]
    fn test_check_dungeon_clear_out_of_bounds() {
        let save = make_save();
        let ctx = MmGameContext::new(&save);

        // Out of bounds should return false, not panic
        assert!(!ctx.check_dungeon_clear_permanent(1000));
        assert!(!ctx.check_dungeon_clear_cycle(1000));
    }

    #[test]
    fn test_boss_defeated_vs_dungeon_clear_independence() {
        // Set only cleared_room (boss defeat), not cleared_floors (dungeon clear)
        let mut perm_flags: Vec<MmPermanentSceneFlags> =
            (0..80).map(|_| make_perm_flag_empty()).collect();
        perm_flags[0x1A] = make_perm_flag_cleared(); // Only cleared_room set

        let save = make_save_with_scene_flags(perm_flags, vec![]);
        let ctx = MmGameContext::new(&save);

        // Boss defeat should be true, dungeon clear should be false
        assert!(EvalContext::event(&ctx, "ODOLWA_DEFEATED"));
        assert!(!EvalContext::event(&ctx, "WOODFALL_TEMPLE_CLEAR"));
    }
}
