//! Core randomizer settings struct.
//!
//! This module defines the main `RandomizerSettings` struct that contains
//! all configuration options for an OoTMM randomizer seed.

use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};

use super::dungeons::{MmDungeon, MqDungeon, OotDungeon};
use super::special::{JunkLocations, SpecialCondition, StartingItems, WorldFlags};
use super::state_modes::*;

/// Complete randomizer settings configuration.
///
/// This struct contains all settings that can affect logic evaluation
/// in an OoTMM randomizer seed.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RandomizerSettings {
    // === Boolean Settings ===
    // These are evaluated with `setting(name)` returning true/false
    /// Allow boots to be used without age constraints.
    #[serde(default)]
    pub ageless_boots: bool,

    /// Allow hookshot to be used without age constraints.
    #[serde(default)]
    pub ageless_hookshot: bool,

    /// Allow strength upgrades to be used without age constraints.
    #[serde(default)]
    pub ageless_strength: bool,

    /// Modify Lost Woods exits.
    #[serde(default)]
    pub alter_lost_woods_exits: bool,

    /// Enable entrance randomization for extra indoor locations.
    #[serde(default)]
    pub er_indoors_extra: bool,

    /// Enable entrance randomization for game link indoor locations.
    #[serde(default)]
    pub er_indoors_game_links: bool,

    /// Enable entrance randomization for major indoor locations.
    #[serde(default)]
    pub er_indoors_major: bool,

    /// Enable entrance randomization for the Moon.
    #[serde(default)]
    pub er_moon: bool,

    /// Open the Mask Shop without requirements.
    #[serde(default)]
    pub open_mask_shop: bool,

    /// Open Moon access without requirements.
    #[serde(default)]
    pub open_moon: bool,

    /// Open Zora's Domain shortcut.
    #[serde(default)]
    pub open_zd_shortcut: bool,

    /// Enable fishing pond fish shuffling.
    #[serde(default)]
    pub pond_fish_shuffle: bool,

    /// Restore broken actors in dungeons.
    #[serde(default)]
    pub restore_broken_actors: bool,

    /// Skip Child Zelda meeting requirement.
    #[serde(default)]
    pub skip_zelda: bool,

    /// Require Master Sword for time travel.
    #[serde(default)]
    pub time_travel_sword: bool,

    // === Shuffle Settings ===
    /// Scrub shuffle (OoT).
    #[serde(default)]
    pub scrub_shuffle_oot: bool,

    /// Scrub shuffle (MM).
    #[serde(default)]
    pub scrub_shuffle_mm: bool,

    /// Cow shuffle (OoT).
    #[serde(default)]
    pub cow_shuffle_oot: bool,

    /// Cow shuffle (MM).
    #[serde(default)]
    pub cow_shuffle_mm: bool,

    /// Beehive shuffle (OoT).
    #[serde(default)]
    pub shuffle_hives_oot: bool,

    /// Beehive shuffle (MM).
    #[serde(default)]
    pub shuffle_hives_mm: bool,

    /// Pot shuffle (OoT).
    #[serde(default)]
    pub shuffle_pots_oot: bool,

    /// Grass shuffle (OoT).
    #[serde(default)]
    pub shuffle_grass_oot: bool,

    /// Grass shuffle (MM).
    #[serde(default)]
    pub shuffle_grass_mm: bool,

    /// Freestanding items shuffle (OoT).
    #[serde(default)]
    pub shuffle_freestanding_oot: bool,

    /// Freestanding items shuffle (MM).
    #[serde(default)]
    pub shuffle_freestanding_mm: bool,

    /// Wonder items shuffle (OoT).
    #[serde(default)]
    pub shuffle_wonderitems_oot: bool,

    /// Wonder items shuffle (MM).
    #[serde(default)]
    pub shuffle_wonderitems_mm: bool,

    /// Snowball shuffle (MM).
    #[serde(default)]
    pub shuffle_snowballs_mm: bool,

    // === Souls Settings ===
    /// Enemy souls (OoT).
    #[serde(default)]
    pub souls_enemy_oot: bool,

    /// Enemy souls (MM).
    #[serde(default)]
    pub souls_enemy_mm: bool,

    /// Boss souls (OoT).
    #[serde(default)]
    pub souls_boss_oot: bool,

    /// Boss souls (MM).
    #[serde(default)]
    pub souls_boss_mm: bool,

    /// NPC souls (OoT).
    #[serde(default)]
    pub souls_npc_oot: bool,

    /// NPC souls (MM).
    #[serde(default)]
    pub souls_npc_mm: bool,

    // === Shared Item Settings ===
    /// Shared spin attack upgrade between games.
    #[serde(default)]
    pub shared_spin_upgrade: bool,

    /// Shared bows between games.
    #[serde(default)]
    pub shared_bows: bool,

    /// Shared bomb bags between games.
    #[serde(default)]
    pub shared_bomb_bags: bool,

    /// Shared magic upgrade between games.
    #[serde(default)]
    pub shared_magic_upgrade: bool,

    /// Shared wallets between games.
    #[serde(default)]
    pub shared_wallets: bool,

    /// Shared health between games.
    #[serde(default)]
    pub shared_health: bool,

    /// Shared shields between games.
    #[serde(default)]
    pub shared_shields: bool,

    /// Shared nuts and sticks between games.
    #[serde(default)]
    pub shared_nuts_sticks: bool,

    /// Shared hookshot between games.
    #[serde(default)]
    pub shared_hookshot: bool,

    /// Shared Lens of Truth between games.
    #[serde(default)]
    pub shared_lens: bool,

    /// Shared ocarina between games.
    #[serde(default)]
    pub shared_ocarina: bool,

    /// Shared masks between games.
    #[serde(default)]
    pub shared_masks: bool,

    /// Shared ocarina songs between games.
    #[serde(default)]
    pub shared_ocarinas_songs: bool,

    /// Shared Song of Time between games.
    #[serde(default)]
    pub shared_song_time: bool,

    /// Shared Epona's Song between games.
    #[serde(default)]
    pub shared_song_epona: bool,

    /// Shared Song of Storms between games.
    #[serde(default)]
    pub shared_song_storms: bool,

    /// Shared Sun's Song between games.
    #[serde(default)]
    pub shared_song_sun: bool,

    /// Shared Saria's Song between games.
    #[serde(default)]
    pub shared_song_saria: bool,

    /// Shared Zelda's Lullaby between games.
    #[serde(default)]
    pub shared_song_zelda: bool,

    /// Shared Song of Healing between games.
    #[serde(default)]
    pub shared_song_healing: bool,

    /// Shared Song of Soaring between games.
    #[serde(default)]
    pub shared_song_soaring: bool,

    // === Ageless Settings ===
    /// Ageless swords.
    #[serde(default)]
    pub ageless_swords: bool,

    /// Ageless shields.
    #[serde(default)]
    pub ageless_shields: bool,

    /// Ageless tunics.
    #[serde(default)]
    pub ageless_tunics: bool,

    /// Ageless sticks.
    #[serde(default)]
    pub ageless_sticks: bool,

    /// Ageless bombs.
    #[serde(default)]
    pub ageless_bombs: bool,

    /// Ageless boomerang.
    #[serde(default)]
    pub ageless_boomerang: bool,

    /// Ageless hammer.
    #[serde(default)]
    pub ageless_hammer: bool,

    /// Ageless child trade items.
    #[serde(default)]
    pub ageless_child_trade: bool,

    /// Ageless adult trade items.
    #[serde(default)]
    pub ageless_adult_trade: bool,

    // === Cross-Game Settings ===
    /// Cross-age play enabled.
    #[serde(default)]
    pub cross_age: bool,

    /// Cross-game Farore's Wind enabled.
    #[serde(default)]
    pub cross_game_fw: bool,

    // === MM-Specific Settings ===
    /// Fire spell available in MM.
    #[serde(default)]
    pub spell_fire_mm: bool,

    /// Iron Boots available in MM.
    #[serde(default)]
    pub boots_iron_mm: bool,

    /// Goron Tunic available in MM.
    #[serde(default)]
    pub tunic_goron_mm: bool,

    /// Zora Tunic available in MM.
    #[serde(default)]
    pub tunic_zora_mm: bool,

    /// Golden Scale available in MM.
    #[serde(default)]
    pub scale_gold_mm: bool,

    // === QOL/Features Settings ===
    /// Swordless adult allowed.
    #[serde(default)]
    pub swordless_adult: bool,

    /// Free scarecrow song in OoT.
    #[serde(default)]
    pub free_scarecrow_oot: bool,

    /// Blue Fire Arrows enabled.
    #[serde(default)]
    pub blue_fire_arrows: bool,

    /// Sunlight Arrows enabled.
    #[serde(default)]
    pub sunlight_arrows: bool,

    /// Fairy Ocarina available in MM.
    #[serde(default)]
    pub fairy_ocarina_mm: bool,

    // === Hints Settings ===
    /// Generate spoiler log.
    #[serde(default)]
    pub generate_spoiler_log: bool,

    /// Probabilistic foolish hints.
    #[serde(default)]
    pub probabilistic_foolish: bool,

    /// Hint importance enabled.
    #[serde(default)]
    pub hint_importance: bool,

    // === Traps Settings ===
    /// Ice traps enabled.
    #[serde(default)]
    pub trap_ice: bool,

    /// Fire traps enabled.
    #[serde(default)]
    pub trap_fire: bool,

    /// Shock traps enabled.
    #[serde(default)]
    pub trap_shock: bool,

    /// Cloak traps (disguised traps).
    #[serde(default)]
    pub cloak_traps: bool,

    // === Misc Settings ===
    /// Clocks shuffled.
    #[serde(default)]
    pub clocks: bool,

    /// Menu notebook enabled.
    #[serde(default)]
    pub menu_notebook: bool,

    /// Coins enabled.
    #[serde(default)]
    pub coins: bool,

    /// Void warp in MM enabled.
    #[serde(default)]
    pub void_warp_mm: bool,

    // === Enumerated Settings ===
    // These are evaluated with `setting(name, value)`
    /// Set of OoT dungeons that are open without requirements.
    #[serde(default)]
    pub open_dungeons_oot: HashSet<OotDungeon>,

    /// Set of MM dungeons that are open without requirements.
    #[serde(default)]
    pub open_dungeons_mm: HashSet<MmDungeon>,

    /// Set of OoT dungeons that use Master Quest layouts.
    ///
    /// When a dungeon is in this set, its checks use MQ flag mappings
    /// instead of vanilla mappings. This affects which locations are
    /// tracked and their memory flag addresses.
    #[serde(default)]
    pub mq_dungeons: HashSet<MqDungeon>,

    /// Deku Tree entrance state.
    #[serde(default)]
    pub deku_tree: DekuTreeState,

    /// Door of Time state.
    #[serde(default)]
    pub door_of_time: DoorOfTimeState,

    /// Kakariko Village gate state.
    #[serde(default)]
    pub kakariko_gate: KakarikoGateState,

    /// Ganon's Castle Boss Key mode.
    #[serde(default)]
    pub ganon_boss_key: GanonBossKeyMode,

    /// Light Arrow Cutscene mode.
    #[serde(default)]
    pub lacs: LacsMode,

    /// Majora child mode.
    #[serde(default)]
    pub majora_child: MajoraChildMode,

    /// Moon crash behavior.
    #[serde(default)]
    pub moon_crash: MoonCrashMode,

    /// Age change mode.
    #[serde(default)]
    pub age_change: AgeChangeMode,

    /// Climb Most Surfaces glitch state (OoT).
    #[serde(default)]
    pub climb_most_surfaces_oot: ClimbMostSurfacesState,

    /// Hookshot Anywhere glitch state (OoT).
    #[serde(default)]
    pub hookshot_anywhere_oot: HookshotAnywhereState,

    /// Beneath the Well state.
    #[serde(default)]
    pub beneath_well: BeneathWellState,

    /// Entrance Randomizer overworld state.
    #[serde(default)]
    pub er_overworld: ErOverworldState,

    /// Entrance Randomizer grottos state.
    #[serde(default)]
    pub er_grottos: ErGrottosState,

    /// Boss Warp Pads mode.
    #[serde(default)]
    pub boss_warp_pads: BossWarpPadsMode,

    /// Clear state for MM dungeons.
    #[serde(default)]
    pub clear_state_dungeons_mm: HashSet<ClearStateDungeonsMm>,

    /// Japan-specific layouts enabled.
    #[serde(default)]
    pub jp_layouts: HashSet<JpLayout>,

    /// Small key shuffle mode for OoT.
    #[serde(default)]
    pub small_key_shuffle_oot: SmallKeyShuffleOot,

    /// Shuffle pots mode for MM.
    #[serde(default)]
    pub shuffle_pots_mm: ShufflePotsMm,

    /// Logic rules mode.
    #[serde(default)]
    pub logic_mode: LogicMode,

    /// Set of enabled logic tricks.
    #[serde(default)]
    pub logic_tricks: HashSet<String>,

    /// Maximum number of bottles (for shared bottle randomizer settings).
    ///
    /// In OoTMM with shared bottles, players may have fewer than 4 bottles.
    /// Valid range is 1-4, defaults to 4.
    #[serde(default = "default_bottle_count")]
    pub bottle_count: u8,

    // === Game Mode Settings ===
    /// Rainbow Bridge access requirements mode.
    #[serde(default)]
    pub rainbow_bridge: RainbowBridgeMode,

    /// Song shuffle mode.
    #[serde(default)]
    pub songs: SongsMode,

    /// Dungeon reward shuffle mode.
    #[serde(default)]
    pub dungeon_reward_shuffle: DungeonRewardShuffle,

    // === Shop/Price Settings ===
    /// Shop shuffle mode for OoT.
    #[serde(default)]
    pub shop_shuffle_oot: ShopShuffleMode,

    /// Shop shuffle mode for MM.
    #[serde(default)]
    pub shop_shuffle_mm: ShopShuffleMode,

    /// Price mode for OoT shops.
    #[serde(default)]
    pub price_oot_shops: PriceMode,

    /// Price mode for OoT scrubs.
    #[serde(default)]
    pub price_oot_scrubs: PriceMode,

    /// Price mode for MM shops.
    #[serde(default)]
    pub price_mm_shops: PriceMode,

    /// Price mode for Tingle maps.
    #[serde(default)]
    pub tingle_prices: PriceMode,

    // === Fairy Shuffle Settings ===
    /// Town fairy shuffle mode (Clock Town stray fairies).
    #[serde(default)]
    pub town_fairy_shuffle: TownFairyShuffle,

    /// Stray fairy shuffle mode for chest fairies.
    #[serde(default)]
    pub stray_fairy_chest_shuffle: StrayFairyShuffle,

    /// Stray fairy shuffle mode for other fairies.
    #[serde(default)]
    pub stray_fairy_other_shuffle: StrayFairyShuffle,

    // === Cross-Warp Settings ===
    /// Cross-warp mode for OoT.
    #[serde(default)]
    pub cross_warp_oot: CrossWarpMode,

    /// Cross-warp mode for MM.
    #[serde(default)]
    pub cross_warp_mm: CrossWarpMode,

    // === Miscellaneous Enum Settings ===
    /// Chest Size Matches Contents mode.
    #[serde(default)]
    pub csmc: CsmcMode,

    /// Bombchu behavior mode.
    #[serde(default)]
    pub bombchu_behavior: BombchuBehavior,

    /// Auto-invert camera mode.
    #[serde(default)]
    pub auto_invert: AutoInvertMode,

    /// Starting age for the player.
    #[serde(default)]
    pub starting_age: StartingAge,

    /// Damage multiplier.
    #[serde(default)]
    pub damage_multiplier: DamageMultiplier,

    /// Item pool size.
    #[serde(default)]
    pub item_pool: ItemPool,

    /// Traps quantity in the item pool.
    #[serde(default)]
    pub traps_quantity: TrapsQuantity,

    // === Collection Fields ===
    /// Special conditions for custom requirements.
    #[serde(default)]
    pub special_conditions: HashMap<String, SpecialCondition>,

    /// Starting items and their quantities.
    #[serde(default)]
    pub starting_items: StartingItems,

    /// Locations designated as junk.
    #[serde(default)]
    pub junk_locations: JunkLocations,

    /// World flags affecting gameplay.
    #[serde(default)]
    pub world_flags: WorldFlags,
}

/// Returns the default bottle count (4).
fn default_bottle_count() -> u8 {
    4
}

impl Default for RandomizerSettings {
    /// Creates default "casual" settings configuration.
    ///
    /// Default settings represent a standard playthrough without
    /// any open dungeons, entrance randomization, or glitch logic.
    fn default() -> Self {
        Self {
            // Boolean settings default to false
            ageless_boots: false,
            ageless_hookshot: false,
            ageless_strength: false,
            alter_lost_woods_exits: false,
            er_indoors_extra: false,
            er_indoors_game_links: false,
            er_indoors_major: false,
            er_moon: false,
            open_mask_shop: false,
            open_moon: false,
            open_zd_shortcut: false,
            pond_fish_shuffle: false,
            restore_broken_actors: false,
            skip_zelda: false,
            time_travel_sword: false,

            // Shuffle settings
            scrub_shuffle_oot: false,
            scrub_shuffle_mm: false,
            cow_shuffle_oot: false,
            cow_shuffle_mm: false,
            shuffle_hives_oot: false,
            shuffle_hives_mm: false,
            shuffle_pots_oot: false,
            shuffle_grass_oot: false,
            shuffle_grass_mm: false,
            shuffle_freestanding_oot: false,
            shuffle_freestanding_mm: false,
            shuffle_wonderitems_oot: false,
            shuffle_wonderitems_mm: false,
            shuffle_snowballs_mm: false,

            // Souls settings
            souls_enemy_oot: false,
            souls_enemy_mm: false,
            souls_boss_oot: false,
            souls_boss_mm: false,
            souls_npc_oot: false,
            souls_npc_mm: false,

            // Shared item settings
            shared_spin_upgrade: false,
            shared_bows: false,
            shared_bomb_bags: false,
            shared_magic_upgrade: false,
            shared_wallets: false,
            shared_health: false,
            shared_shields: false,
            shared_nuts_sticks: false,
            shared_hookshot: false,
            shared_lens: false,
            shared_ocarina: false,
            shared_masks: false,
            shared_ocarinas_songs: false,
            shared_song_time: false,
            shared_song_epona: false,
            shared_song_storms: false,
            shared_song_sun: false,
            shared_song_saria: false,
            shared_song_zelda: false,
            shared_song_healing: false,
            shared_song_soaring: false,

            // Ageless settings
            ageless_swords: false,
            ageless_shields: false,
            ageless_tunics: false,
            ageless_sticks: false,
            ageless_bombs: false,
            ageless_boomerang: false,
            ageless_hammer: false,
            ageless_child_trade: false,
            ageless_adult_trade: false,

            // Cross-game settings
            cross_age: false,
            cross_game_fw: false,

            // MM-specific settings
            spell_fire_mm: false,
            boots_iron_mm: false,
            tunic_goron_mm: false,
            tunic_zora_mm: false,
            scale_gold_mm: false,

            // QOL/Features settings
            swordless_adult: false,
            free_scarecrow_oot: false,
            blue_fire_arrows: false,
            sunlight_arrows: false,
            fairy_ocarina_mm: false,

            // Hints settings
            generate_spoiler_log: false,
            probabilistic_foolish: false,
            hint_importance: false,

            // Traps settings
            trap_ice: false,
            trap_fire: false,
            trap_shock: false,
            cloak_traps: false,

            // Misc settings
            clocks: false,
            menu_notebook: false,
            coins: false,
            void_warp_mm: false,

            // Set settings default to empty
            open_dungeons_oot: HashSet::new(),
            open_dungeons_mm: HashSet::new(),
            mq_dungeons: HashSet::new(),
            clear_state_dungeons_mm: HashSet::new(),
            jp_layouts: HashSet::new(),
            logic_tricks: HashSet::new(),

            // Enum settings default to their Default variants
            deku_tree: DekuTreeState::default(),
            door_of_time: DoorOfTimeState::default(),
            kakariko_gate: KakarikoGateState::default(),
            ganon_boss_key: GanonBossKeyMode::default(),
            lacs: LacsMode::default(),
            majora_child: MajoraChildMode::default(),
            moon_crash: MoonCrashMode::default(),
            age_change: AgeChangeMode::default(),
            climb_most_surfaces_oot: ClimbMostSurfacesState::default(),
            hookshot_anywhere_oot: HookshotAnywhereState::default(),
            beneath_well: BeneathWellState::default(),
            er_overworld: ErOverworldState::default(),
            er_grottos: ErGrottosState::default(),
            boss_warp_pads: BossWarpPadsMode::default(),
            small_key_shuffle_oot: SmallKeyShuffleOot::default(),
            shuffle_pots_mm: ShufflePotsMm::default(),
            logic_mode: LogicMode::default(),
            bottle_count: 4,

            // Game mode settings
            rainbow_bridge: RainbowBridgeMode::default(),
            songs: SongsMode::default(),
            dungeon_reward_shuffle: DungeonRewardShuffle::default(),

            // Shop/price settings
            shop_shuffle_oot: ShopShuffleMode::default(),
            shop_shuffle_mm: ShopShuffleMode::default(),
            price_oot_shops: PriceMode::default(),
            price_oot_scrubs: PriceMode::default(),
            price_mm_shops: PriceMode::default(),
            tingle_prices: PriceMode::default(),

            // Fairy shuffle settings
            town_fairy_shuffle: TownFairyShuffle::default(),
            stray_fairy_chest_shuffle: StrayFairyShuffle::default(),
            stray_fairy_other_shuffle: StrayFairyShuffle::default(),

            // Cross-warp settings
            cross_warp_oot: CrossWarpMode::default(),
            cross_warp_mm: CrossWarpMode::default(),

            // Miscellaneous enum settings
            csmc: CsmcMode::default(),
            bombchu_behavior: BombchuBehavior::default(),
            auto_invert: AutoInvertMode::default(),
            starting_age: StartingAge::default(),
            damage_multiplier: DamageMultiplier::default(),
            item_pool: ItemPool::default(),
            traps_quantity: TrapsQuantity::default(),

            // Collection fields
            special_conditions: HashMap::new(),
            starting_items: StartingItems::new(),
            junk_locations: JunkLocations::new(),
            world_flags: WorldFlags::default(),
        }
    }
}

impl RandomizerSettings {
    /// Creates a new settings instance with default values.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }
}
