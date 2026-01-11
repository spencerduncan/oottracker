//! Enum setting accessors for RandomizerSettings.

use crate::settings::core::RandomizerSettings;
use crate::settings::dungeons::{MmDungeon, MqDungeon, OotDungeon};
use crate::settings::state_modes::{ClearStateDungeonsMm, JpLayout};

impl RandomizerSettings {
    /// Checks if a setting has a specific value.
    ///
    /// This is used for `setting(name, value)` logic expressions.
    #[must_use]
    pub fn check_setting_value(&self, name: &str, value: &str) -> bool {
        match name {
            "openDungeonsOot" => OotDungeon::parse(value)
                .map(|d| self.open_dungeons_oot.contains(&d))
                .unwrap_or(false),
            "openDungeonsMm" => MmDungeon::parse(value)
                .map(|d| self.open_dungeons_mm.contains(&d))
                .unwrap_or(false),
            "mqDungeons" => MqDungeon::parse(value)
                .map(|d| self.mq_dungeons.contains(&d))
                .unwrap_or(false),
            "dekuTree" => self.deku_tree.as_str() == value,
            "doorOfTime" => self.door_of_time.as_str() == value,
            "kakarikoGate" => self.kakariko_gate.as_str() == value,
            "ganonBossKey" => self.ganon_boss_key.as_str() == value,
            "lacs" => self.lacs.as_str() == value,
            "majoraChild" => self.majora_child.as_str() == value,
            "moonCrash" => self.moon_crash.as_str() == value,
            "ageChange" => self.age_change.as_str() == value,
            "climbMostSurfacesOot" => self.climb_most_surfaces_oot.as_str() == value,
            "hookshotAnywhereOot" => self.hookshot_anywhere_oot.as_str() == value,
            "beneathWell" => self.beneath_well.as_str() == value,
            "erOverworld" => self.er_overworld.as_str() == value,
            "erGrottos" => self.er_grottos.as_str() == value,
            "bossWarpPads" => self.boss_warp_pads.as_str() == value,
            "clearStateDungeonsMm" => ClearStateDungeonsMm::parse(value)
                .map(|d| self.clear_state_dungeons_mm.contains(&d))
                .unwrap_or(false),
            "jpLayouts" => JpLayout::parse(value)
                .map(|l| self.jp_layouts.contains(&l))
                .unwrap_or(false),
            "smallKeyShuffleOot" => self.small_key_shuffle_oot.as_str() == value,
            "shufflePotsMm" => self.shuffle_pots_mm.as_str() == value,
            "logicMode" => self.logic_mode.as_str() == value,
            // Game mode settings
            "rainbowBridge" => self.rainbow_bridge.as_str() == value,
            "songs" => self.songs.as_str() == value,
            "dungeonRewardShuffle" => self.dungeon_reward_shuffle.as_str() == value,
            // Shop/price settings
            "shopShuffleOot" => self.shop_shuffle_oot.as_str() == value,
            "shopShuffleMm" => self.shop_shuffle_mm.as_str() == value,
            "priceOotShops" => self.price_oot_shops.as_str() == value,
            "priceOotScrubs" => self.price_oot_scrubs.as_str() == value,
            "priceMmShops" => self.price_mm_shops.as_str() == value,
            "tinglePrices" => self.tingle_prices.as_str() == value,
            // Fairy shuffle settings
            "townFairyShuffle" => self.town_fairy_shuffle.as_str() == value,
            "strayFairyChestShuffle" => self.stray_fairy_chest_shuffle.as_str() == value,
            "strayFairyOtherShuffle" => self.stray_fairy_other_shuffle.as_str() == value,
            // Cross-warp settings
            "crossWarpOot" => self.cross_warp_oot.as_str() == value,
            "crossWarpMm" => self.cross_warp_mm.as_str() == value,
            // Miscellaneous enum settings
            "csmc" => self.csmc.as_str() == value,
            "bombchuBehavior" => self.bombchu_behavior.as_str() == value,
            "autoInvert" => self.auto_invert.as_str() == value,
            "startingAge" => self.starting_age.as_str() == value,
            "damageMultiplier" => self.damage_multiplier.as_str() == value,
            "itemPool" => self.item_pool.as_str() == value,
            "trapsQuantity" => self.traps_quantity.as_str() == value,
            _ => false,
        }
    }

    /// Checks if a logic trick is enabled.
    #[must_use]
    pub fn has_trick(&self, trick: &str) -> bool {
        self.logic_tricks.contains(trick)
    }

    /// Enables a logic trick.
    pub fn enable_trick(&mut self, trick: impl Into<String>) {
        self.logic_tricks.insert(trick.into());
    }

    /// Disables a logic trick.
    pub fn disable_trick(&mut self, trick: &str) {
        self.logic_tricks.remove(trick);
    }

    // === Bottle Count Methods ===

    /// Returns the maximum bottle count for this seed.
    ///
    /// For shared bottle randomizer settings, this may be less than 4.
    #[must_use]
    pub fn get_bottle_count(&self) -> u8 {
        self.bottle_count.clamp(1, 4)
    }

    /// Sets the maximum bottle count.
    ///
    /// The value is clamped to the valid range of 1-4.
    pub fn set_bottle_count(&mut self, count: u8) {
        self.bottle_count = count.clamp(1, 4);
    }
}
