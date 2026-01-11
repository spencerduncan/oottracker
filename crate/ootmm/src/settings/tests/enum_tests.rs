//! State and mode enum tests.

use crate::settings::*;

// === RainbowBridgeMode Tests ===

#[test]
fn test_rainbow_bridge_mode_default() {
    let mode = RainbowBridgeMode::default();
    assert_eq!(mode, RainbowBridgeMode::Vanilla);
}

#[test]
fn test_rainbow_bridge_mode_as_str() {
    assert_eq!(RainbowBridgeMode::Vanilla.as_str(), "vanilla");
    assert_eq!(RainbowBridgeMode::Open.as_str(), "open");
    assert_eq!(RainbowBridgeMode::Medallions.as_str(), "medallions");
    assert_eq!(RainbowBridgeMode::Stones.as_str(), "stones");
    assert_eq!(RainbowBridgeMode::DungeonRewards.as_str(), "dungeonRewards");
    assert_eq!(RainbowBridgeMode::Skulltulas.as_str(), "skulltulas");
    assert_eq!(RainbowBridgeMode::Remains.as_str(), "remains");
    assert_eq!(RainbowBridgeMode::Custom.as_str(), "custom");
}

#[test]
fn test_rainbow_bridge_mode_parse() {
    assert_eq!(
        RainbowBridgeMode::parse("vanilla"),
        Some(RainbowBridgeMode::Vanilla)
    );
    assert_eq!(
        RainbowBridgeMode::parse("open"),
        Some(RainbowBridgeMode::Open)
    );
    assert_eq!(
        RainbowBridgeMode::parse("medallions"),
        Some(RainbowBridgeMode::Medallions)
    );
    assert_eq!(
        RainbowBridgeMode::parse("stones"),
        Some(RainbowBridgeMode::Stones)
    );
    assert_eq!(
        RainbowBridgeMode::parse("dungeonRewards"),
        Some(RainbowBridgeMode::DungeonRewards)
    );
    assert_eq!(
        RainbowBridgeMode::parse("skulltulas"),
        Some(RainbowBridgeMode::Skulltulas)
    );
    assert_eq!(
        RainbowBridgeMode::parse("remains"),
        Some(RainbowBridgeMode::Remains)
    );
    assert_eq!(
        RainbowBridgeMode::parse("custom"),
        Some(RainbowBridgeMode::Custom)
    );
    assert_eq!(RainbowBridgeMode::parse("invalid"), None);
}

#[test]
fn test_rainbow_bridge_mode_roundtrip() {
    for mode in [
        RainbowBridgeMode::Vanilla,
        RainbowBridgeMode::Open,
        RainbowBridgeMode::Medallions,
        RainbowBridgeMode::Stones,
        RainbowBridgeMode::DungeonRewards,
        RainbowBridgeMode::Skulltulas,
        RainbowBridgeMode::Remains,
        RainbowBridgeMode::Custom,
    ] {
        let s = mode.as_str();
        let parsed = RainbowBridgeMode::parse(s);
        assert_eq!(parsed, Some(mode));
    }
}

// === SongsMode Tests ===

#[test]
fn test_songs_mode_default() {
    let mode = SongsMode::default();
    assert_eq!(mode, SongsMode::SongsOnly);
}

#[test]
fn test_songs_mode_as_str() {
    assert_eq!(SongsMode::SongsOnly.as_str(), "songsOnly");
    assert_eq!(SongsMode::Anywhere.as_str(), "anywhere");
    assert_eq!(SongsMode::DungeonRewards.as_str(), "dungeonRewards");
}

#[test]
fn test_songs_mode_parse() {
    assert_eq!(SongsMode::parse("songsOnly"), Some(SongsMode::SongsOnly));
    assert_eq!(SongsMode::parse("anywhere"), Some(SongsMode::Anywhere));
    assert_eq!(
        SongsMode::parse("dungeonRewards"),
        Some(SongsMode::DungeonRewards)
    );
    assert_eq!(SongsMode::parse("invalid"), None);
}

// === DungeonRewardShuffle Tests ===

#[test]
fn test_dungeon_reward_shuffle_default() {
    let mode = DungeonRewardShuffle::default();
    assert_eq!(mode, DungeonRewardShuffle::Vanilla);
}

#[test]
fn test_dungeon_reward_shuffle_as_str() {
    assert_eq!(DungeonRewardShuffle::Vanilla.as_str(), "vanilla");
    assert_eq!(
        DungeonRewardShuffle::DungeonBlueWarps.as_str(),
        "dungeonBlueWarps"
    );
    assert_eq!(DungeonRewardShuffle::Anywhere.as_str(), "anywhere");
}

#[test]
fn test_dungeon_reward_shuffle_parse() {
    assert_eq!(
        DungeonRewardShuffle::parse("vanilla"),
        Some(DungeonRewardShuffle::Vanilla)
    );
    assert_eq!(
        DungeonRewardShuffle::parse("dungeonBlueWarps"),
        Some(DungeonRewardShuffle::DungeonBlueWarps)
    );
    assert_eq!(
        DungeonRewardShuffle::parse("anywhere"),
        Some(DungeonRewardShuffle::Anywhere)
    );
    assert_eq!(DungeonRewardShuffle::parse("invalid"), None);
}

// === ShuffleMode Tests ===

#[test]
fn test_shuffle_mode_default() {
    let mode = ShuffleMode::default();
    assert_eq!(mode, ShuffleMode::None);
}

#[test]
fn test_shuffle_mode_as_str() {
    assert_eq!(ShuffleMode::None.as_str(), "none");
    assert_eq!(ShuffleMode::Overworld.as_str(), "overworld");
    assert_eq!(ShuffleMode::Dungeon.as_str(), "dungeon");
    assert_eq!(ShuffleMode::All.as_str(), "all");
}

#[test]
fn test_shuffle_mode_parse() {
    assert_eq!(ShuffleMode::parse("none"), Some(ShuffleMode::None));
    assert_eq!(
        ShuffleMode::parse("overworld"),
        Some(ShuffleMode::Overworld)
    );
    assert_eq!(ShuffleMode::parse("dungeon"), Some(ShuffleMode::Dungeon));
    assert_eq!(ShuffleMode::parse("all"), Some(ShuffleMode::All));
    assert_eq!(ShuffleMode::parse("invalid"), None);
}

#[test]
fn test_shuffle_mode_is_shuffled() {
    assert!(!ShuffleMode::None.is_shuffled());
    assert!(ShuffleMode::Overworld.is_shuffled());
    assert!(ShuffleMode::Dungeon.is_shuffled());
    assert!(ShuffleMode::All.is_shuffled());
}

#[test]
fn test_shuffle_mode_serde_roundtrip() {
    let mode = ShuffleMode::Overworld;
    let json = serde_json::to_string(&mode).unwrap();
    let parsed: ShuffleMode = serde_json::from_str(&json).unwrap();
    assert_eq!(parsed, mode);
}

// === KeyShuffle Tests ===

#[test]
fn test_key_shuffle_default() {
    let mode = KeyShuffle::default();
    assert_eq!(mode, KeyShuffle::Vanilla);
}

#[test]
fn test_key_shuffle_as_str() {
    assert_eq!(KeyShuffle::Vanilla.as_str(), "vanilla");
    assert_eq!(KeyShuffle::OwnDungeon.as_str(), "ownDungeon");
    assert_eq!(KeyShuffle::Anywhere.as_str(), "anywhere");
    assert_eq!(KeyShuffle::Removed.as_str(), "removed");
}

#[test]
fn test_key_shuffle_parse() {
    assert_eq!(KeyShuffle::parse("vanilla"), Some(KeyShuffle::Vanilla));
    assert_eq!(
        KeyShuffle::parse("ownDungeon"),
        Some(KeyShuffle::OwnDungeon)
    );
    assert_eq!(KeyShuffle::parse("anywhere"), Some(KeyShuffle::Anywhere));
    assert_eq!(KeyShuffle::parse("removed"), Some(KeyShuffle::Removed));
    assert_eq!(KeyShuffle::parse("invalid"), None);
}

#[test]
fn test_key_shuffle_serde_roundtrip() {
    let mode = KeyShuffle::OwnDungeon;
    let json = serde_json::to_string(&mode).unwrap();
    let parsed: KeyShuffle = serde_json::from_str(&json).unwrap();
    assert_eq!(parsed, mode);
}

#[test]
fn test_key_shuffle_is_shuffled() {
    assert!(!KeyShuffle::Vanilla.is_shuffled());
    assert!(KeyShuffle::OwnDungeon.is_shuffled());
    assert!(KeyShuffle::Anywhere.is_shuffled());
    assert!(!KeyShuffle::Removed.is_shuffled());
}

// === MapCompassShuffle Tests ===

#[test]
fn test_map_compass_shuffle_default() {
    let mode = MapCompassShuffle::default();
    assert_eq!(mode, MapCompassShuffle::Vanilla);
}

#[test]
fn test_map_compass_shuffle_as_str() {
    assert_eq!(MapCompassShuffle::Vanilla.as_str(), "vanilla");
    assert_eq!(MapCompassShuffle::Starting.as_str(), "starting");
    assert_eq!(MapCompassShuffle::OwnDungeon.as_str(), "ownDungeon");
    assert_eq!(MapCompassShuffle::Anywhere.as_str(), "anywhere");
    assert_eq!(MapCompassShuffle::Removed.as_str(), "removed");
}

#[test]
fn test_map_compass_shuffle_parse() {
    assert_eq!(
        MapCompassShuffle::parse("vanilla"),
        Some(MapCompassShuffle::Vanilla)
    );
    assert_eq!(
        MapCompassShuffle::parse("starting"),
        Some(MapCompassShuffle::Starting)
    );
    assert_eq!(
        MapCompassShuffle::parse("ownDungeon"),
        Some(MapCompassShuffle::OwnDungeon)
    );
    assert_eq!(
        MapCompassShuffle::parse("anywhere"),
        Some(MapCompassShuffle::Anywhere)
    );
    assert_eq!(
        MapCompassShuffle::parse("removed"),
        Some(MapCompassShuffle::Removed)
    );
    assert_eq!(MapCompassShuffle::parse("invalid"), None);
}

#[test]
fn test_map_compass_shuffle_is_shuffled() {
    assert!(!MapCompassShuffle::Vanilla.is_shuffled());
    assert!(!MapCompassShuffle::Starting.is_shuffled());
    assert!(MapCompassShuffle::OwnDungeon.is_shuffled());
    assert!(MapCompassShuffle::Anywhere.is_shuffled());
    assert!(!MapCompassShuffle::Removed.is_shuffled());
}

// === DekuTreeState Tests ===

#[test]
fn test_deku_tree_state_default() {
    let state = DekuTreeState::default();
    assert_eq!(state, DekuTreeState::Closed);
}

#[test]
fn test_deku_tree_state_as_str() {
    assert_eq!(DekuTreeState::Closed.as_str(), "closed");
    assert_eq!(DekuTreeState::Open.as_str(), "open");
    assert_eq!(DekuTreeState::Vanilla.as_str(), "vanilla");
}

#[test]
fn test_deku_tree_state_parse() {
    assert_eq!(DekuTreeState::parse("closed"), Some(DekuTreeState::Closed));
    assert_eq!(DekuTreeState::parse("open"), Some(DekuTreeState::Open));
    assert_eq!(
        DekuTreeState::parse("vanilla"),
        Some(DekuTreeState::Vanilla)
    );
    assert_eq!(DekuTreeState::parse("invalid"), None);
}

#[test]
fn test_deku_tree_state_serde_roundtrip() {
    let state = DekuTreeState::Open;
    let json = serde_json::to_string(&state).unwrap();
    let parsed: DekuTreeState = serde_json::from_str(&json).unwrap();
    assert_eq!(parsed, state);
}

// === Additional State Tests ===

#[test]
fn test_door_of_time_state_default() {
    let state = DoorOfTimeState::default();
    assert_eq!(state, DoorOfTimeState::Closed);
}

#[test]
fn test_kakariko_gate_state_default() {
    let state = KakarikoGateState::default();
    assert_eq!(state, KakarikoGateState::Closed);
}

#[test]
fn test_ganon_boss_key_mode_default() {
    let mode = GanonBossKeyMode::default();
    assert_eq!(mode, GanonBossKeyMode::Vanilla);
}

#[test]
fn test_lacs_mode_default() {
    let mode = LacsMode::default();
    assert_eq!(mode, LacsMode::Vanilla);
}

#[test]
fn test_majora_child_mode_default() {
    let mode = MajoraChildMode::default();
    assert_eq!(mode, MajoraChildMode::Vanilla);
}

#[test]
fn test_moon_crash_mode_default() {
    let mode = MoonCrashMode::default();
    assert_eq!(mode, MoonCrashMode::Vanilla);
}

#[test]
fn test_age_change_mode_default() {
    let mode = AgeChangeMode::default();
    assert_eq!(mode, AgeChangeMode::TempleOfTime);
}

#[test]
fn test_climb_most_surfaces_state_default() {
    let state = ClimbMostSurfacesState::default();
    assert_eq!(state, ClimbMostSurfacesState::On);
}

#[test]
fn test_hookshot_anywhere_state_default() {
    let state = HookshotAnywhereState::default();
    assert_eq!(state, HookshotAnywhereState::On);
}

#[test]
fn test_beneath_well_state_default() {
    let state = BeneathWellState::default();
    assert_eq!(state, BeneathWellState::Vanilla);
}

#[test]
fn test_er_overworld_state_default() {
    let state = ErOverworldState::default();
    assert_eq!(state, ErOverworldState::None);
}

#[test]
fn test_er_grottos_state_default() {
    let state = ErGrottosState::default();
    assert_eq!(state, ErGrottosState::None);
}

#[test]
fn test_boss_warp_pads_mode_default() {
    let mode = BossWarpPadsMode::default();
    assert_eq!(mode, BossWarpPadsMode::Vanilla);
}

// === ClearStateDungeonsMm Tests ===

#[test]
fn test_clear_state_dungeons_mm_as_str() {
    assert_eq!(ClearStateDungeonsMm::Woodfall.as_str(), "WF");
    assert_eq!(ClearStateDungeonsMm::Both.as_str(), "both");
}

#[test]
fn test_clear_state_dungeons_mm_parse() {
    assert_eq!(
        ClearStateDungeonsMm::parse("WF"),
        Some(ClearStateDungeonsMm::Woodfall)
    );
    assert_eq!(
        ClearStateDungeonsMm::parse("both"),
        Some(ClearStateDungeonsMm::Both)
    );
    assert_eq!(ClearStateDungeonsMm::parse("invalid"), None);
}

// === JpLayout Tests ===

#[test]
fn test_jp_layout_as_str() {
    assert_eq!(JpLayout::GreatBayCoast.as_str(), "GreatBayCoast");
    assert_eq!(JpLayout::StoneTowerEntrance.as_str(), "ST");
    assert_eq!(JpLayout::StoneTower.as_str(), "StoneTower");
}

#[test]
fn test_jp_layout_parse() {
    assert_eq!(
        JpLayout::parse("GreatBayCoast"),
        Some(JpLayout::GreatBayCoast)
    );
    assert_eq!(JpLayout::parse("ST"), Some(JpLayout::StoneTowerEntrance));
    assert_eq!(JpLayout::parse("StoneTower"), Some(JpLayout::StoneTower));
    assert_eq!(JpLayout::parse("invalid"), None);
}

// === Various Mode Tests ===

#[test]
fn test_small_key_shuffle_oot_default() {
    let mode = SmallKeyShuffleOot::default();
    assert_eq!(mode, SmallKeyShuffleOot::Vanilla);
}

#[test]
fn test_shuffle_pots_mm_default() {
    let mode = ShufflePotsMm::default();
    assert_eq!(mode, ShufflePotsMm::None);
}

#[test]
fn test_logic_mode_default() {
    let mode = LogicMode::default();
    assert_eq!(mode, LogicMode::Glitchless);
}

#[test]
fn test_logic_mode_parse() {
    assert_eq!(LogicMode::parse("glitchless"), Some(LogicMode::Glitchless));
    assert_eq!(LogicMode::parse("glitched"), Some(LogicMode::Glitched));
    assert_eq!(LogicMode::parse("noLogic"), Some(LogicMode::NoLogic));
    assert_eq!(LogicMode::parse("no_logic"), Some(LogicMode::NoLogic));
    assert_eq!(LogicMode::parse("invalid"), None);
}

#[test]
fn test_shop_shuffle_mode_default() {
    let mode = ShopShuffleMode::default();
    assert_eq!(mode, ShopShuffleMode::None);
}

#[test]
fn test_price_mode_default() {
    let mode = PriceMode::default();
    assert_eq!(mode, PriceMode::Vanilla);
}

#[test]
fn test_town_fairy_shuffle_default() {
    let mode = TownFairyShuffle::default();
    assert_eq!(mode, TownFairyShuffle::Vanilla);
}

#[test]
fn test_stray_fairy_shuffle_default() {
    let mode = StrayFairyShuffle::default();
    assert_eq!(mode, StrayFairyShuffle::Vanilla);
}

#[test]
fn test_cross_warp_mode_default() {
    let mode = CrossWarpMode::default();
    assert_eq!(mode, CrossWarpMode::None);
}

#[test]
fn test_csmc_mode_default() {
    let mode = CsmcMode::default();
    assert_eq!(mode, CsmcMode::Never);
}

#[test]
fn test_bombchu_behavior_default() {
    let mode = BombchuBehavior::default();
    assert_eq!(mode, BombchuBehavior::Normal);
}

#[test]
fn test_auto_invert_mode_default() {
    let mode = AutoInvertMode::default();
    assert_eq!(mode, AutoInvertMode::Off);
}

#[test]
fn test_starting_age_default() {
    let age = StartingAge::default();
    assert_eq!(age, StartingAge::Child);
}

#[test]
fn test_damage_multiplier_default() {
    let mult = DamageMultiplier::default();
    assert_eq!(mult, DamageMultiplier::Normal);
}

#[test]
fn test_item_pool_default() {
    let pool = ItemPool::default();
    assert_eq!(pool, ItemPool::Normal);
}

#[test]
fn test_traps_quantity_default() {
    let qty = TrapsQuantity::default();
    assert_eq!(qty, TrapsQuantity::None);
}

#[test]
fn test_tingle_shuffle_default() {
    let mode = TingleShuffle::default();
    assert_eq!(mode, TingleShuffle::Vanilla);
}

#[test]
fn test_owl_shuffle_default() {
    let mode = OwlShuffle::default();
    assert_eq!(mode, OwlShuffle::None);
}

#[test]
fn test_skulltula_token_shuffle_default() {
    let mode = SkulltulaTokenShuffle::default();
    assert_eq!(mode, SkulltulaTokenShuffle::None);
}
