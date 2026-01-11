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

// === Additional _as_str, _parse, and _serde_roundtrip tests ===

// === TingleShuffle Tests ===

#[test]
fn test_tingle_shuffle_as_str() {
    assert_eq!(TingleShuffle::Vanilla.as_str(), "vanilla");
    assert_eq!(TingleShuffle::Starting.as_str(), "starting");
    assert_eq!(TingleShuffle::Removed.as_str(), "removed");
    assert_eq!(TingleShuffle::Anywhere.as_str(), "anywhere");
    assert_eq!(TingleShuffle::OwnRegion.as_str(), "ownRegion");
}

#[test]
fn test_tingle_shuffle_parse() {
    assert_eq!(
        TingleShuffle::parse("vanilla"),
        Some(TingleShuffle::Vanilla)
    );
    assert_eq!(
        TingleShuffle::parse("starting"),
        Some(TingleShuffle::Starting)
    );
    assert_eq!(
        TingleShuffle::parse("removed"),
        Some(TingleShuffle::Removed)
    );
    assert_eq!(
        TingleShuffle::parse("anywhere"),
        Some(TingleShuffle::Anywhere)
    );
    assert_eq!(
        TingleShuffle::parse("ownRegion"),
        Some(TingleShuffle::OwnRegion)
    );
    assert_eq!(TingleShuffle::parse("invalid"), None);
}

#[test]
fn test_tingle_shuffle_serde_roundtrip() {
    let mode = TingleShuffle::OwnRegion;
    let json = serde_json::to_string(&mode).unwrap();
    let parsed: TingleShuffle = serde_json::from_str(&json).unwrap();
    assert_eq!(parsed, mode);
}

// === OwlShuffle Tests ===

#[test]
fn test_owl_shuffle_as_str() {
    assert_eq!(OwlShuffle::None.as_str(), "none");
    assert_eq!(OwlShuffle::Anywhere.as_str(), "anywhere");
}

#[test]
fn test_owl_shuffle_parse() {
    assert_eq!(OwlShuffle::parse("none"), Some(OwlShuffle::None));
    assert_eq!(OwlShuffle::parse("anywhere"), Some(OwlShuffle::Anywhere));
    assert_eq!(OwlShuffle::parse("invalid"), None);
}

#[test]
fn test_owl_shuffle_serde_roundtrip() {
    let mode = OwlShuffle::Anywhere;
    let json = serde_json::to_string(&mode).unwrap();
    let parsed: OwlShuffle = serde_json::from_str(&json).unwrap();
    assert_eq!(parsed, mode);
}

// === SkulltulaTokenShuffle Tests ===

#[test]
fn test_skulltula_token_shuffle_as_str() {
    assert_eq!(SkulltulaTokenShuffle::None.as_str(), "none");
    assert_eq!(SkulltulaTokenShuffle::Dungeons.as_str(), "dungeons");
    assert_eq!(SkulltulaTokenShuffle::Overworld.as_str(), "overworld");
    assert_eq!(SkulltulaTokenShuffle::All.as_str(), "all");
}

#[test]
fn test_skulltula_token_shuffle_parse() {
    assert_eq!(
        SkulltulaTokenShuffle::parse("none"),
        Some(SkulltulaTokenShuffle::None)
    );
    assert_eq!(
        SkulltulaTokenShuffle::parse("dungeons"),
        Some(SkulltulaTokenShuffle::Dungeons)
    );
    assert_eq!(
        SkulltulaTokenShuffle::parse("overworld"),
        Some(SkulltulaTokenShuffle::Overworld)
    );
    assert_eq!(
        SkulltulaTokenShuffle::parse("all"),
        Some(SkulltulaTokenShuffle::All)
    );
    assert_eq!(SkulltulaTokenShuffle::parse("invalid"), None);
}

#[test]
fn test_skulltula_token_shuffle_serde_roundtrip() {
    let mode = SkulltulaTokenShuffle::Dungeons;
    let json = serde_json::to_string(&mode).unwrap();
    let parsed: SkulltulaTokenShuffle = serde_json::from_str(&json).unwrap();
    assert_eq!(parsed, mode);
}

// === DekuTreeState Roundtrip ===

#[test]
fn test_deku_tree_state_roundtrip() {
    for state in [
        DekuTreeState::Closed,
        DekuTreeState::Open,
        DekuTreeState::Vanilla,
    ] {
        let s = state.as_str();
        assert_eq!(DekuTreeState::parse(s), Some(state));
    }
}

// === DoorOfTimeState Tests ===

#[test]
fn test_door_of_time_state_as_str() {
    assert_eq!(DoorOfTimeState::Closed.as_str(), "closed");
    assert_eq!(DoorOfTimeState::Open.as_str(), "open");
}

#[test]
fn test_door_of_time_state_parse() {
    assert_eq!(
        DoorOfTimeState::parse("closed"),
        Some(DoorOfTimeState::Closed)
    );
    assert_eq!(DoorOfTimeState::parse("open"), Some(DoorOfTimeState::Open));
    assert_eq!(DoorOfTimeState::parse("invalid"), None);
}

#[test]
fn test_door_of_time_state_serde_roundtrip() {
    let state = DoorOfTimeState::Open;
    let json = serde_json::to_string(&state).unwrap();
    let parsed: DoorOfTimeState = serde_json::from_str(&json).unwrap();
    assert_eq!(parsed, state);
}

// === KakarikoGateState Tests ===

#[test]
fn test_kakariko_gate_state_as_str() {
    assert_eq!(KakarikoGateState::Closed.as_str(), "closed");
    assert_eq!(KakarikoGateState::Open.as_str(), "open");
}

#[test]
fn test_kakariko_gate_state_parse() {
    assert_eq!(
        KakarikoGateState::parse("closed"),
        Some(KakarikoGateState::Closed)
    );
    assert_eq!(
        KakarikoGateState::parse("open"),
        Some(KakarikoGateState::Open)
    );
    assert_eq!(KakarikoGateState::parse("invalid"), None);
}

#[test]
fn test_kakariko_gate_state_serde_roundtrip() {
    let state = KakarikoGateState::Open;
    let json = serde_json::to_string(&state).unwrap();
    let parsed: KakarikoGateState = serde_json::from_str(&json).unwrap();
    assert_eq!(parsed, state);
}

// === GanonBossKeyMode Tests ===

#[test]
fn test_ganon_boss_key_mode_as_str() {
    assert_eq!(GanonBossKeyMode::Vanilla.as_str(), "vanilla");
    assert_eq!(GanonBossKeyMode::Removed.as_str(), "removed");
    assert_eq!(GanonBossKeyMode::Custom.as_str(), "custom");
}

#[test]
fn test_ganon_boss_key_mode_parse() {
    assert_eq!(
        GanonBossKeyMode::parse("vanilla"),
        Some(GanonBossKeyMode::Vanilla)
    );
    assert_eq!(
        GanonBossKeyMode::parse("removed"),
        Some(GanonBossKeyMode::Removed)
    );
    assert_eq!(
        GanonBossKeyMode::parse("custom"),
        Some(GanonBossKeyMode::Custom)
    );
    assert_eq!(GanonBossKeyMode::parse("invalid"), None);
}

#[test]
fn test_ganon_boss_key_mode_serde_roundtrip() {
    for mode in [
        GanonBossKeyMode::Vanilla,
        GanonBossKeyMode::Removed,
        GanonBossKeyMode::Custom,
    ] {
        let json = serde_json::to_string(&mode).unwrap();
        let parsed: GanonBossKeyMode = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed, mode);
    }
}

// === LacsMode Tests ===

#[test]
fn test_lacs_mode_as_str() {
    assert_eq!(LacsMode::Vanilla.as_str(), "vanilla");
    assert_eq!(LacsMode::Custom.as_str(), "custom");
}

#[test]
fn test_lacs_mode_parse() {
    assert_eq!(LacsMode::parse("vanilla"), Some(LacsMode::Vanilla));
    assert_eq!(LacsMode::parse("custom"), Some(LacsMode::Custom));
    assert_eq!(LacsMode::parse("invalid"), None);
}

#[test]
fn test_lacs_mode_serde_roundtrip() {
    for mode in [LacsMode::Vanilla, LacsMode::Custom] {
        let json = serde_json::to_string(&mode).unwrap();
        let parsed: LacsMode = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed, mode);
    }
}

// === MajoraChildMode Tests ===

#[test]
fn test_majora_child_mode_as_str() {
    assert_eq!(MajoraChildMode::Vanilla.as_str(), "vanilla");
    assert_eq!(MajoraChildMode::None.as_str(), "none");
    assert_eq!(MajoraChildMode::Custom.as_str(), "custom");
}

#[test]
fn test_majora_child_mode_parse() {
    assert_eq!(
        MajoraChildMode::parse("vanilla"),
        Some(MajoraChildMode::Vanilla)
    );
    assert_eq!(MajoraChildMode::parse("none"), Some(MajoraChildMode::None));
    assert_eq!(
        MajoraChildMode::parse("custom"),
        Some(MajoraChildMode::Custom)
    );
    assert_eq!(MajoraChildMode::parse("invalid"), None);
}

#[test]
fn test_majora_child_mode_serde_roundtrip() {
    for mode in [
        MajoraChildMode::Vanilla,
        MajoraChildMode::None,
        MajoraChildMode::Custom,
    ] {
        let json = serde_json::to_string(&mode).unwrap();
        let parsed: MajoraChildMode = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed, mode);
    }
}

// === MoonCrashMode Tests ===

#[test]
fn test_moon_crash_mode_as_str() {
    assert_eq!(MoonCrashMode::Vanilla.as_str(), "vanilla");
    assert_eq!(MoonCrashMode::Cycle.as_str(), "cycle");
}

#[test]
fn test_moon_crash_mode_parse() {
    assert_eq!(
        MoonCrashMode::parse("vanilla"),
        Some(MoonCrashMode::Vanilla)
    );
    assert_eq!(MoonCrashMode::parse("cycle"), Some(MoonCrashMode::Cycle));
    assert_eq!(MoonCrashMode::parse("invalid"), None);
}

#[test]
fn test_moon_crash_mode_serde_roundtrip() {
    for mode in [MoonCrashMode::Vanilla, MoonCrashMode::Cycle] {
        let json = serde_json::to_string(&mode).unwrap();
        let parsed: MoonCrashMode = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed, mode);
    }
}

// === AgeChangeMode Tests ===

#[test]
fn test_age_change_mode_as_str() {
    assert_eq!(AgeChangeMode::TempleOfTime.as_str(), "templeOfTime");
    assert_eq!(AgeChangeMode::None.as_str(), "none");
    assert_eq!(AgeChangeMode::Oot.as_str(), "oot");
}

#[test]
fn test_age_change_mode_parse() {
    assert_eq!(
        AgeChangeMode::parse("templeOfTime"),
        Some(AgeChangeMode::TempleOfTime)
    );
    assert_eq!(AgeChangeMode::parse("none"), Some(AgeChangeMode::None));
    assert_eq!(AgeChangeMode::parse("oot"), Some(AgeChangeMode::Oot));
    assert_eq!(AgeChangeMode::parse("invalid"), None);
}

#[test]
fn test_age_change_mode_serde_roundtrip() {
    for mode in [
        AgeChangeMode::TempleOfTime,
        AgeChangeMode::None,
        AgeChangeMode::Oot,
    ] {
        let json = serde_json::to_string(&mode).unwrap();
        let parsed: AgeChangeMode = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed, mode);
    }
}

// === ClimbMostSurfacesState Tests ===

#[test]
fn test_climb_most_surfaces_state_as_str() {
    assert_eq!(ClimbMostSurfacesState::On.as_str(), "on");
    assert_eq!(ClimbMostSurfacesState::Off.as_str(), "off");
}

#[test]
fn test_climb_most_surfaces_state_parse() {
    assert_eq!(
        ClimbMostSurfacesState::parse("on"),
        Some(ClimbMostSurfacesState::On)
    );
    assert_eq!(
        ClimbMostSurfacesState::parse("off"),
        Some(ClimbMostSurfacesState::Off)
    );
    assert_eq!(ClimbMostSurfacesState::parse("invalid"), None);
}

#[test]
fn test_climb_most_surfaces_state_serde_roundtrip() {
    for state in [ClimbMostSurfacesState::On, ClimbMostSurfacesState::Off] {
        let json = serde_json::to_string(&state).unwrap();
        let parsed: ClimbMostSurfacesState = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed, state);
    }
}

// === HookshotAnywhereState Tests ===

#[test]
fn test_hookshot_anywhere_state_as_str() {
    assert_eq!(HookshotAnywhereState::On.as_str(), "on");
    assert_eq!(HookshotAnywhereState::Off.as_str(), "off");
}

#[test]
fn test_hookshot_anywhere_state_parse() {
    assert_eq!(
        HookshotAnywhereState::parse("on"),
        Some(HookshotAnywhereState::On)
    );
    assert_eq!(
        HookshotAnywhereState::parse("off"),
        Some(HookshotAnywhereState::Off)
    );
    assert_eq!(HookshotAnywhereState::parse("invalid"), None);
}

#[test]
fn test_hookshot_anywhere_state_serde_roundtrip() {
    for state in [HookshotAnywhereState::On, HookshotAnywhereState::Off] {
        let json = serde_json::to_string(&state).unwrap();
        let parsed: HookshotAnywhereState = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed, state);
    }
}

// === BeneathWellState Tests ===

#[test]
fn test_beneath_well_state_as_str() {
    assert_eq!(BeneathWellState::Vanilla.as_str(), "vanilla");
    assert_eq!(BeneathWellState::Open.as_str(), "open");
}

#[test]
fn test_beneath_well_state_parse() {
    assert_eq!(
        BeneathWellState::parse("vanilla"),
        Some(BeneathWellState::Vanilla)
    );
    assert_eq!(
        BeneathWellState::parse("open"),
        Some(BeneathWellState::Open)
    );
    assert_eq!(BeneathWellState::parse("invalid"), None);
}

#[test]
fn test_beneath_well_state_serde_roundtrip() {
    for state in [BeneathWellState::Vanilla, BeneathWellState::Open] {
        let json = serde_json::to_string(&state).unwrap();
        let parsed: BeneathWellState = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed, state);
    }
}

// === ErOverworldState Tests ===

#[test]
fn test_er_overworld_state_as_str() {
    assert_eq!(ErOverworldState::None.as_str(), "none");
    assert_eq!(ErOverworldState::Full.as_str(), "full");
}

#[test]
fn test_er_overworld_state_parse() {
    assert_eq!(
        ErOverworldState::parse("none"),
        Some(ErOverworldState::None)
    );
    assert_eq!(
        ErOverworldState::parse("full"),
        Some(ErOverworldState::Full)
    );
    assert_eq!(ErOverworldState::parse("invalid"), None);
}

#[test]
fn test_er_overworld_state_serde_roundtrip() {
    for state in [ErOverworldState::None, ErOverworldState::Full] {
        let json = serde_json::to_string(&state).unwrap();
        let parsed: ErOverworldState = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed, state);
    }
}

// === ErGrottosState Tests ===

#[test]
fn test_er_grottos_state_as_str() {
    assert_eq!(ErGrottosState::None.as_str(), "none");
    assert_eq!(ErGrottosState::Full.as_str(), "full");
}

#[test]
fn test_er_grottos_state_parse() {
    assert_eq!(ErGrottosState::parse("none"), Some(ErGrottosState::None));
    assert_eq!(ErGrottosState::parse("full"), Some(ErGrottosState::Full));
    assert_eq!(ErGrottosState::parse("invalid"), None);
}

#[test]
fn test_er_grottos_state_serde_roundtrip() {
    for state in [ErGrottosState::None, ErGrottosState::Full] {
        let json = serde_json::to_string(&state).unwrap();
        let parsed: ErGrottosState = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed, state);
    }
}

// === BossWarpPadsMode Tests ===

#[test]
fn test_boss_warp_pads_mode_as_str() {
    assert_eq!(BossWarpPadsMode::Vanilla.as_str(), "vanilla");
    assert_eq!(BossWarpPadsMode::Remains.as_str(), "remains");
}

#[test]
fn test_boss_warp_pads_mode_parse() {
    assert_eq!(
        BossWarpPadsMode::parse("vanilla"),
        Some(BossWarpPadsMode::Vanilla)
    );
    assert_eq!(
        BossWarpPadsMode::parse("remains"),
        Some(BossWarpPadsMode::Remains)
    );
    assert_eq!(BossWarpPadsMode::parse("invalid"), None);
}

#[test]
fn test_boss_warp_pads_mode_serde_roundtrip() {
    for mode in [BossWarpPadsMode::Vanilla, BossWarpPadsMode::Remains] {
        let json = serde_json::to_string(&mode).unwrap();
        let parsed: BossWarpPadsMode = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed, mode);
    }
}

// === ClearStateDungeonsMm Serde Roundtrip ===

#[test]
fn test_clear_state_dungeons_mm_serde_roundtrip() {
    for state in [ClearStateDungeonsMm::Woodfall, ClearStateDungeonsMm::Both] {
        let json = serde_json::to_string(&state).unwrap();
        let parsed: ClearStateDungeonsMm = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed, state);
    }
}

// === JpLayout Serde Roundtrip ===

#[test]
fn test_jp_layout_serde_roundtrip() {
    for layout in [
        JpLayout::GreatBayCoast,
        JpLayout::StoneTowerEntrance,
        JpLayout::StoneTower,
    ] {
        let json = serde_json::to_string(&layout).unwrap();
        let parsed: JpLayout = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed, layout);
    }
}

// === SmallKeyShuffleOot Tests ===

#[test]
fn test_small_key_shuffle_oot_as_str() {
    assert_eq!(SmallKeyShuffleOot::Vanilla.as_str(), "vanilla");
    assert_eq!(SmallKeyShuffleOot::Dungeon.as_str(), "dungeon");
    assert_eq!(SmallKeyShuffleOot::Anywhere.as_str(), "anywhere");
}

#[test]
fn test_small_key_shuffle_oot_parse() {
    assert_eq!(
        SmallKeyShuffleOot::parse("vanilla"),
        Some(SmallKeyShuffleOot::Vanilla)
    );
    assert_eq!(
        SmallKeyShuffleOot::parse("dungeon"),
        Some(SmallKeyShuffleOot::Dungeon)
    );
    assert_eq!(
        SmallKeyShuffleOot::parse("anywhere"),
        Some(SmallKeyShuffleOot::Anywhere)
    );
    assert_eq!(SmallKeyShuffleOot::parse("invalid"), None);
}

#[test]
fn test_small_key_shuffle_oot_serde_roundtrip() {
    for mode in [
        SmallKeyShuffleOot::Vanilla,
        SmallKeyShuffleOot::Dungeon,
        SmallKeyShuffleOot::Anywhere,
    ] {
        let json = serde_json::to_string(&mode).unwrap();
        let parsed: SmallKeyShuffleOot = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed, mode);
    }
}

// === ShufflePotsMm Tests ===

#[test]
fn test_shuffle_pots_mm_as_str() {
    assert_eq!(ShufflePotsMm::None.as_str(), "none");
    assert_eq!(ShufflePotsMm::All.as_str(), "all");
}

#[test]
fn test_shuffle_pots_mm_parse() {
    assert_eq!(ShufflePotsMm::parse("none"), Some(ShufflePotsMm::None));
    assert_eq!(ShufflePotsMm::parse("all"), Some(ShufflePotsMm::All));
    assert_eq!(ShufflePotsMm::parse("invalid"), None);
}

#[test]
fn test_shuffle_pots_mm_serde_roundtrip() {
    for mode in [ShufflePotsMm::None, ShufflePotsMm::All] {
        let json = serde_json::to_string(&mode).unwrap();
        let parsed: ShufflePotsMm = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed, mode);
    }
}

// === LogicMode Tests ===

#[test]
fn test_logic_mode_as_str() {
    assert_eq!(LogicMode::Glitchless.as_str(), "glitchless");
    assert_eq!(LogicMode::Glitched.as_str(), "glitched");
    assert_eq!(LogicMode::NoLogic.as_str(), "noLogic");
}

#[test]
fn test_logic_mode_serde_roundtrip() {
    for mode in [
        LogicMode::Glitchless,
        LogicMode::Glitched,
        LogicMode::NoLogic,
    ] {
        let json = serde_json::to_string(&mode).unwrap();
        let parsed: LogicMode = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed, mode);
    }
}

// === ShopShuffleMode Tests ===

#[test]
fn test_shop_shuffle_mode_as_str() {
    assert_eq!(ShopShuffleMode::None.as_str(), "none");
    assert_eq!(ShopShuffleMode::OwnShop.as_str(), "ownShop");
    assert_eq!(ShopShuffleMode::All.as_str(), "all");
}

#[test]
fn test_shop_shuffle_mode_parse() {
    assert_eq!(ShopShuffleMode::parse("none"), Some(ShopShuffleMode::None));
    assert_eq!(
        ShopShuffleMode::parse("ownShop"),
        Some(ShopShuffleMode::OwnShop)
    );
    assert_eq!(ShopShuffleMode::parse("all"), Some(ShopShuffleMode::All));
    assert_eq!(ShopShuffleMode::parse("invalid"), None);
}

#[test]
fn test_shop_shuffle_mode_serde_roundtrip() {
    for mode in [
        ShopShuffleMode::None,
        ShopShuffleMode::OwnShop,
        ShopShuffleMode::All,
    ] {
        let json = serde_json::to_string(&mode).unwrap();
        let parsed: ShopShuffleMode = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed, mode);
    }
}

// === PriceMode Tests ===

#[test]
fn test_price_mode_as_str() {
    assert_eq!(PriceMode::Vanilla.as_str(), "vanilla");
    assert_eq!(PriceMode::Weighted.as_str(), "weighted");
    assert_eq!(PriceMode::Random.as_str(), "random");
    assert_eq!(PriceMode::Fixed.as_str(), "fixed");
}

#[test]
fn test_price_mode_parse() {
    assert_eq!(PriceMode::parse("vanilla"), Some(PriceMode::Vanilla));
    assert_eq!(PriceMode::parse("weighted"), Some(PriceMode::Weighted));
    assert_eq!(PriceMode::parse("random"), Some(PriceMode::Random));
    assert_eq!(PriceMode::parse("fixed"), Some(PriceMode::Fixed));
    assert_eq!(PriceMode::parse("invalid"), None);
}

#[test]
fn test_price_mode_serde_roundtrip() {
    for mode in [
        PriceMode::Vanilla,
        PriceMode::Weighted,
        PriceMode::Random,
        PriceMode::Fixed,
    ] {
        let json = serde_json::to_string(&mode).unwrap();
        let parsed: PriceMode = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed, mode);
    }
}

// === TownFairyShuffle Tests ===

#[test]
fn test_town_fairy_shuffle_as_str() {
    assert_eq!(TownFairyShuffle::Vanilla.as_str(), "vanilla");
    assert_eq!(TownFairyShuffle::Anywhere.as_str(), "anywhere");
}

#[test]
fn test_town_fairy_shuffle_parse() {
    assert_eq!(
        TownFairyShuffle::parse("vanilla"),
        Some(TownFairyShuffle::Vanilla)
    );
    assert_eq!(
        TownFairyShuffle::parse("anywhere"),
        Some(TownFairyShuffle::Anywhere)
    );
    assert_eq!(TownFairyShuffle::parse("invalid"), None);
}

#[test]
fn test_town_fairy_shuffle_serde_roundtrip() {
    for mode in [TownFairyShuffle::Vanilla, TownFairyShuffle::Anywhere] {
        let json = serde_json::to_string(&mode).unwrap();
        let parsed: TownFairyShuffle = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed, mode);
    }
}

// === StrayFairyShuffle Tests ===

#[test]
fn test_stray_fairy_shuffle_as_str() {
    assert_eq!(StrayFairyShuffle::Vanilla.as_str(), "vanilla");
    assert_eq!(StrayFairyShuffle::Starting.as_str(), "starting");
    assert_eq!(StrayFairyShuffle::Removed.as_str(), "removed");
    assert_eq!(StrayFairyShuffle::OwnDungeon.as_str(), "ownDungeon");
    assert_eq!(StrayFairyShuffle::Anywhere.as_str(), "anywhere");
}

#[test]
fn test_stray_fairy_shuffle_parse() {
    assert_eq!(
        StrayFairyShuffle::parse("vanilla"),
        Some(StrayFairyShuffle::Vanilla)
    );
    assert_eq!(
        StrayFairyShuffle::parse("starting"),
        Some(StrayFairyShuffle::Starting)
    );
    assert_eq!(
        StrayFairyShuffle::parse("removed"),
        Some(StrayFairyShuffle::Removed)
    );
    assert_eq!(
        StrayFairyShuffle::parse("ownDungeon"),
        Some(StrayFairyShuffle::OwnDungeon)
    );
    assert_eq!(
        StrayFairyShuffle::parse("anywhere"),
        Some(StrayFairyShuffle::Anywhere)
    );
    assert_eq!(StrayFairyShuffle::parse("invalid"), None);
}

#[test]
fn test_stray_fairy_shuffle_serde_roundtrip() {
    for mode in [
        StrayFairyShuffle::Vanilla,
        StrayFairyShuffle::Starting,
        StrayFairyShuffle::Removed,
        StrayFairyShuffle::OwnDungeon,
        StrayFairyShuffle::Anywhere,
    ] {
        let json = serde_json::to_string(&mode).unwrap();
        let parsed: StrayFairyShuffle = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed, mode);
    }
}

// === CrossWarpMode Tests ===

#[test]
fn test_cross_warp_mode_as_str() {
    assert_eq!(CrossWarpMode::None.as_str(), "none");
    assert_eq!(CrossWarpMode::ChildOnly.as_str(), "childOnly");
    assert_eq!(CrossWarpMode::AdultOnly.as_str(), "adultOnly");
    assert_eq!(CrossWarpMode::Full.as_str(), "full");
}

#[test]
fn test_cross_warp_mode_parse() {
    assert_eq!(CrossWarpMode::parse("none"), Some(CrossWarpMode::None));
    assert_eq!(
        CrossWarpMode::parse("childOnly"),
        Some(CrossWarpMode::ChildOnly)
    );
    assert_eq!(
        CrossWarpMode::parse("adultOnly"),
        Some(CrossWarpMode::AdultOnly)
    );
    assert_eq!(CrossWarpMode::parse("full"), Some(CrossWarpMode::Full));
    assert_eq!(CrossWarpMode::parse("invalid"), None);
}

#[test]
fn test_cross_warp_mode_serde_roundtrip() {
    for mode in [
        CrossWarpMode::None,
        CrossWarpMode::ChildOnly,
        CrossWarpMode::AdultOnly,
        CrossWarpMode::Full,
    ] {
        let json = serde_json::to_string(&mode).unwrap();
        let parsed: CrossWarpMode = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed, mode);
    }
}

// === CsmcMode Tests ===

#[test]
fn test_csmc_mode_as_str() {
    assert_eq!(CsmcMode::Never.as_str(), "never");
    assert_eq!(CsmcMode::Always.as_str(), "always");
    assert_eq!(CsmcMode::Agony.as_str(), "agony");
}

#[test]
fn test_csmc_mode_parse() {
    assert_eq!(CsmcMode::parse("never"), Some(CsmcMode::Never));
    assert_eq!(CsmcMode::parse("always"), Some(CsmcMode::Always));
    assert_eq!(CsmcMode::parse("agony"), Some(CsmcMode::Agony));
    assert_eq!(CsmcMode::parse("invalid"), None);
}

#[test]
fn test_csmc_mode_serde_roundtrip() {
    for mode in [CsmcMode::Never, CsmcMode::Always, CsmcMode::Agony] {
        let json = serde_json::to_string(&mode).unwrap();
        let parsed: CsmcMode = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed, mode);
    }
}

// === BombchuBehavior Tests ===

#[test]
fn test_bombchu_behavior_as_str() {
    assert_eq!(BombchuBehavior::Normal.as_str(), "normal");
    assert_eq!(BombchuBehavior::BombsOrLogic.as_str(), "bombsOrLogic");
    assert_eq!(BombchuBehavior::AsBombs.as_str(), "asBombs");
}

#[test]
fn test_bombchu_behavior_parse() {
    assert_eq!(
        BombchuBehavior::parse("normal"),
        Some(BombchuBehavior::Normal)
    );
    assert_eq!(
        BombchuBehavior::parse("bombsOrLogic"),
        Some(BombchuBehavior::BombsOrLogic)
    );
    assert_eq!(
        BombchuBehavior::parse("asBombs"),
        Some(BombchuBehavior::AsBombs)
    );
    assert_eq!(BombchuBehavior::parse("invalid"), None);
}

#[test]
fn test_bombchu_behavior_serde_roundtrip() {
    for mode in [
        BombchuBehavior::Normal,
        BombchuBehavior::BombsOrLogic,
        BombchuBehavior::AsBombs,
    ] {
        let json = serde_json::to_string(&mode).unwrap();
        let parsed: BombchuBehavior = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed, mode);
    }
}

// === AutoInvertMode Tests ===

#[test]
fn test_auto_invert_mode_as_str() {
    assert_eq!(AutoInvertMode::Off.as_str(), "off");
    assert_eq!(AutoInvertMode::FirstPerson.as_str(), "firstPerson");
    assert_eq!(AutoInvertMode::Always.as_str(), "always");
}

#[test]
fn test_auto_invert_mode_parse() {
    assert_eq!(AutoInvertMode::parse("off"), Some(AutoInvertMode::Off));
    assert_eq!(
        AutoInvertMode::parse("firstPerson"),
        Some(AutoInvertMode::FirstPerson)
    );
    assert_eq!(
        AutoInvertMode::parse("always"),
        Some(AutoInvertMode::Always)
    );
    assert_eq!(AutoInvertMode::parse("invalid"), None);
}

#[test]
fn test_auto_invert_mode_serde_roundtrip() {
    for mode in [
        AutoInvertMode::Off,
        AutoInvertMode::FirstPerson,
        AutoInvertMode::Always,
    ] {
        let json = serde_json::to_string(&mode).unwrap();
        let parsed: AutoInvertMode = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed, mode);
    }
}

// === StartingAge Tests ===

#[test]
fn test_starting_age_as_str() {
    assert_eq!(StartingAge::Child.as_str(), "child");
    assert_eq!(StartingAge::Adult.as_str(), "adult");
    assert_eq!(StartingAge::Random.as_str(), "random");
}

#[test]
fn test_starting_age_parse() {
    assert_eq!(StartingAge::parse("child"), Some(StartingAge::Child));
    assert_eq!(StartingAge::parse("adult"), Some(StartingAge::Adult));
    assert_eq!(StartingAge::parse("random"), Some(StartingAge::Random));
    assert_eq!(StartingAge::parse("invalid"), None);
}

#[test]
fn test_starting_age_serde_roundtrip() {
    for age in [StartingAge::Child, StartingAge::Adult, StartingAge::Random] {
        let json = serde_json::to_string(&age).unwrap();
        let parsed: StartingAge = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed, age);
    }
}

// === DamageMultiplier Tests ===

#[test]
fn test_damage_multiplier_as_str() {
    assert_eq!(DamageMultiplier::Half.as_str(), "half");
    assert_eq!(DamageMultiplier::Normal.as_str(), "normal");
    assert_eq!(DamageMultiplier::Double.as_str(), "double");
    assert_eq!(DamageMultiplier::Quadruple.as_str(), "quadruple");
    assert_eq!(DamageMultiplier::Ohko.as_str(), "ohko");
}

#[test]
fn test_damage_multiplier_parse() {
    assert_eq!(
        DamageMultiplier::parse("half"),
        Some(DamageMultiplier::Half)
    );
    assert_eq!(
        DamageMultiplier::parse("normal"),
        Some(DamageMultiplier::Normal)
    );
    assert_eq!(
        DamageMultiplier::parse("double"),
        Some(DamageMultiplier::Double)
    );
    assert_eq!(
        DamageMultiplier::parse("quadruple"),
        Some(DamageMultiplier::Quadruple)
    );
    assert_eq!(
        DamageMultiplier::parse("ohko"),
        Some(DamageMultiplier::Ohko)
    );
    assert_eq!(DamageMultiplier::parse("invalid"), None);
}

#[test]
fn test_damage_multiplier_serde_roundtrip() {
    for mult in [
        DamageMultiplier::Half,
        DamageMultiplier::Normal,
        DamageMultiplier::Double,
        DamageMultiplier::Quadruple,
        DamageMultiplier::Ohko,
    ] {
        let json = serde_json::to_string(&mult).unwrap();
        let parsed: DamageMultiplier = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed, mult);
    }
}

// === ItemPool Tests ===

#[test]
fn test_item_pool_as_str() {
    assert_eq!(ItemPool::Plentiful.as_str(), "plentiful");
    assert_eq!(ItemPool::Normal.as_str(), "normal");
    assert_eq!(ItemPool::Scarce.as_str(), "scarce");
    assert_eq!(ItemPool::Minimal.as_str(), "minimal");
}

#[test]
fn test_item_pool_parse() {
    assert_eq!(ItemPool::parse("plentiful"), Some(ItemPool::Plentiful));
    assert_eq!(ItemPool::parse("normal"), Some(ItemPool::Normal));
    assert_eq!(ItemPool::parse("scarce"), Some(ItemPool::Scarce));
    assert_eq!(ItemPool::parse("minimal"), Some(ItemPool::Minimal));
    assert_eq!(ItemPool::parse("invalid"), None);
}

#[test]
fn test_item_pool_serde_roundtrip() {
    for pool in [
        ItemPool::Plentiful,
        ItemPool::Normal,
        ItemPool::Scarce,
        ItemPool::Minimal,
    ] {
        let json = serde_json::to_string(&pool).unwrap();
        let parsed: ItemPool = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed, pool);
    }
}

// === TrapsQuantity Tests ===

#[test]
fn test_traps_quantity_as_str() {
    assert_eq!(TrapsQuantity::None.as_str(), "none");
    assert_eq!(TrapsQuantity::Few.as_str(), "few");
    assert_eq!(TrapsQuantity::Normal.as_str(), "normal");
    assert_eq!(TrapsQuantity::Many.as_str(), "many");
    assert_eq!(TrapsQuantity::Onslaught.as_str(), "onslaught");
}

#[test]
fn test_traps_quantity_parse() {
    assert_eq!(TrapsQuantity::parse("none"), Some(TrapsQuantity::None));
    assert_eq!(TrapsQuantity::parse("few"), Some(TrapsQuantity::Few));
    assert_eq!(TrapsQuantity::parse("normal"), Some(TrapsQuantity::Normal));
    assert_eq!(TrapsQuantity::parse("many"), Some(TrapsQuantity::Many));
    assert_eq!(
        TrapsQuantity::parse("onslaught"),
        Some(TrapsQuantity::Onslaught)
    );
    assert_eq!(TrapsQuantity::parse("invalid"), None);
}

#[test]
fn test_traps_quantity_serde_roundtrip() {
    for qty in [
        TrapsQuantity::None,
        TrapsQuantity::Few,
        TrapsQuantity::Normal,
        TrapsQuantity::Many,
        TrapsQuantity::Onslaught,
    ] {
        let json = serde_json::to_string(&qty).unwrap();
        let parsed: TrapsQuantity = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed, qty);
    }
}

// === MapCompassShuffle Serde Roundtrip ===

#[test]
fn test_map_compass_shuffle_serde_roundtrip() {
    let mode = MapCompassShuffle::OwnDungeon;
    let json = serde_json::to_string(&mode).unwrap();
    let parsed: MapCompassShuffle = serde_json::from_str(&json).unwrap();
    assert_eq!(parsed, mode);
}

// === DungeonRewardShuffle Roundtrip ===

#[test]
fn test_dungeon_reward_shuffle_roundtrip() {
    for mode in [
        DungeonRewardShuffle::Vanilla,
        DungeonRewardShuffle::DungeonBlueWarps,
        DungeonRewardShuffle::Anywhere,
    ] {
        let s = mode.as_str();
        let parsed = DungeonRewardShuffle::parse(s);
        assert_eq!(parsed, Some(mode));
    }
}

// === SongsMode Roundtrip ===

#[test]
fn test_songs_mode_roundtrip() {
    for mode in [
        SongsMode::SongsOnly,
        SongsMode::Anywhere,
        SongsMode::DungeonRewards,
    ] {
        let s = mode.as_str();
        let parsed = SongsMode::parse(s);
        assert_eq!(parsed, Some(mode));
    }
}
