//! Boolean setting accessors for RandomizerSettings.

use crate::settings::core::RandomizerSettings;

impl RandomizerSettings {
    /// Checks if a boolean setting is enabled.
    ///
    /// This is used for `setting(name)` logic expressions.
    #[must_use]
    pub fn get_bool_setting(&self, name: &str) -> Option<bool> {
        match name {
            // Original boolean settings
            "agelessBoots" => Some(self.ageless_boots),
            "agelessHookshot" => Some(self.ageless_hookshot),
            "agelessStrength" => Some(self.ageless_strength),
            "alterLostWoodsExits" => Some(self.alter_lost_woods_exits),
            "erIndoorsExtra" => Some(self.er_indoors_extra),
            "erIndoorsGameLinks" => Some(self.er_indoors_game_links),
            "erIndoorsMajor" => Some(self.er_indoors_major),
            "erMoon" => Some(self.er_moon),
            "openMaskShop" => Some(self.open_mask_shop),
            "openMoon" => Some(self.open_moon),
            "openZdShortcut" => Some(self.open_zd_shortcut),
            "pondFishShuffle" => Some(self.pond_fish_shuffle),
            "restoreBrokenActors" => Some(self.restore_broken_actors),
            "skipZelda" => Some(self.skip_zelda),
            "timeTravelSword" => Some(self.time_travel_sword),

            // Shuffle settings
            "scrubShuffleOot" => Some(self.scrub_shuffle_oot),
            "scrubShuffleMm" => Some(self.scrub_shuffle_mm),
            "cowShuffleOot" => Some(self.cow_shuffle_oot),
            "cowShuffleMm" => Some(self.cow_shuffle_mm),
            "shuffleHivesOot" => Some(self.shuffle_hives_oot),
            "shuffleHivesMm" => Some(self.shuffle_hives_mm),
            "shufflePotsOot" => Some(self.shuffle_pots_oot),
            "shuffleGrassOot" => Some(self.shuffle_grass_oot),
            "shuffleGrassMm" => Some(self.shuffle_grass_mm),
            "shuffleFreestandingOot" => Some(self.shuffle_freestanding_oot),
            "shuffleFreestandingMm" => Some(self.shuffle_freestanding_mm),
            "shuffleWonderitemsOot" => Some(self.shuffle_wonderitems_oot),
            "shuffleWonderitemsMm" => Some(self.shuffle_wonderitems_mm),
            "shuffleSnowballsMm" => Some(self.shuffle_snowballs_mm),

            // Souls settings
            "soulsEnemyOot" => Some(self.souls_enemy_oot),
            "soulsEnemyMm" => Some(self.souls_enemy_mm),
            "soulsBossOot" => Some(self.souls_boss_oot),
            "soulsBossMm" => Some(self.souls_boss_mm),
            "soulsNpcOot" => Some(self.souls_npc_oot),
            "soulsNpcMm" => Some(self.souls_npc_mm),

            // Shared item settings
            "sharedSpinUpgrade" => Some(self.shared_spin_upgrade),
            "sharedBows" => Some(self.shared_bows),
            "sharedBombBags" => Some(self.shared_bomb_bags),
            "sharedMagicUpgrade" => Some(self.shared_magic_upgrade),
            "sharedWallets" => Some(self.shared_wallets),
            "sharedHealth" => Some(self.shared_health),
            "sharedShields" => Some(self.shared_shields),
            "sharedNutsSticks" => Some(self.shared_nuts_sticks),
            "sharedHookshot" => Some(self.shared_hookshot),
            "sharedLens" => Some(self.shared_lens),
            "sharedOcarina" => Some(self.shared_ocarina),
            "sharedMasks" => Some(self.shared_masks),
            "sharedOcarinasSongs" => Some(self.shared_ocarinas_songs),
            "sharedSongTime" => Some(self.shared_song_time),
            "sharedSongEpona" => Some(self.shared_song_epona),
            "sharedSongStorms" => Some(self.shared_song_storms),
            "sharedSongSun" => Some(self.shared_song_sun),
            "sharedSongSaria" => Some(self.shared_song_saria),
            "sharedSongZelda" => Some(self.shared_song_zelda),
            "sharedSongHealing" => Some(self.shared_song_healing),
            "sharedSongSoaring" => Some(self.shared_song_soaring),

            // Ageless settings
            "agelessSwords" => Some(self.ageless_swords),
            "agelessShields" => Some(self.ageless_shields),
            "agelessTunics" => Some(self.ageless_tunics),
            "agelessSticks" => Some(self.ageless_sticks),
            "agelessBombs" => Some(self.ageless_bombs),
            "agelessBoomerang" => Some(self.ageless_boomerang),
            "agelessHammer" => Some(self.ageless_hammer),
            "agelessChildTrade" => Some(self.ageless_child_trade),
            "agelessAdultTrade" => Some(self.ageless_adult_trade),

            // Cross-game settings
            "crossAge" => Some(self.cross_age),
            "crossGameFw" => Some(self.cross_game_fw),

            // MM-specific settings
            "spellFireMm" => Some(self.spell_fire_mm),
            "bootsIronMm" => Some(self.boots_iron_mm),
            "tunicGoronMm" => Some(self.tunic_goron_mm),
            "tunicZoraMm" => Some(self.tunic_zora_mm),
            "scaleGoldMm" => Some(self.scale_gold_mm),

            // QOL/Features settings
            "swordlessAdult" => Some(self.swordless_adult),
            "freeScarecrowOot" => Some(self.free_scarecrow_oot),
            "blueFireArrows" => Some(self.blue_fire_arrows),
            "sunlightArrows" => Some(self.sunlight_arrows),
            "fairyOcarinaMm" => Some(self.fairy_ocarina_mm),

            // Hints settings
            "generateSpoilerLog" => Some(self.generate_spoiler_log),
            "probabilisticFoolish" => Some(self.probabilistic_foolish),
            "hintImportance" => Some(self.hint_importance),

            // Traps settings
            "trapIce" => Some(self.trap_ice),
            "trapFire" => Some(self.trap_fire),
            "trapShock" => Some(self.trap_shock),
            "cloakTraps" => Some(self.cloak_traps),

            // Misc settings
            "clocks" => Some(self.clocks),
            "menuNotebook" => Some(self.menu_notebook),
            "coins" => Some(self.coins),
            "voidWarpMm" => Some(self.void_warp_mm),

            _ => None,
        }
    }
}
