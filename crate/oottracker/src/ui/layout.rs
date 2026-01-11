//! Layout configurations for the tracker UI.
//!
//! This module provides TrackerLayout, CellLayout, and DoubleTrackerLayout
//! for organizing tracker cells into various display configurations.

use {
    super::{
        cells::TrackerCellId,
        config::{Config, ElementOrder},
    },
    async_proto::Protocol,
    itertools::Itertools as _,
    ootr::model::{DungeonReward, Medallion, Stone},
};
#[cfg(feature = "rocket")]
use {
    rocket::{
        http::uri::fmt::{Formatter, Path, UriDisplay},
        request::FromParam,
    },
    std::fmt,
};

#[derive(Debug, Clone, Copy, PartialEq, Protocol)]
pub enum TrackerLayout {
    Default {
        auto: bool,
        meds: ElementOrder,
        warp_songs: ElementOrder,
    },
    MultiworldExpanded,
    MultiworldCollapsed,
    MultiworldEdit,
    RslLeft,
    RslRight,
    RslEdit,
    Rsl3Player,
    TsgMainWithRewardLocations,
    TsgMainWithRewardLocationsEdit,
    TriforcePieces,
    // MM-specific layouts
    MmDefault,
    MmMasks,
    MmBossRemains,
    MmStrayFairies,
    MmSongs,
    MmEquipment,
    // Dungeon item layouts (maps, compasses)
    DungeonItems,
    MmDungeonItems,
    // Combo layout for OoTMM randomizer
    Combo,
}

pub struct CellLayout {
    pub idx: usize,
    pub id: TrackerCellId,
    pub pos: [u16; 2],
    pub size: [u16; 2],
}

impl TrackerLayout {
    /// The default layout for auto-tracking, which replaces the Triforce piece count cell with a dynamic big Poe count/Triforce piece count cell.
    pub fn default_auto() -> TrackerLayout {
        TrackerLayout::new_auto(&Config::default())
    }

    /// The auto-tracking layout for this config, which replaces the Triforce piece count cell with a dynamic big Poe count/Triforce piece count cell.
    pub fn new_auto(config: &Config) -> TrackerLayout {
        TrackerLayout::Default {
            auto: true,
            meds: config.med_order,
            warp_songs: config.warp_song_order,
        }
    }

    pub fn cells(&self) -> Vec<CellLayout> {
        use TrackerCellId::*;

        macro_rules! columns {
            ($width:expr, [$($id:expr,)*]) => {{
                vec![$($id),*]
                    .into_iter()
                    .enumerate()
                    .map(|(idx, id)| CellLayout { idx, id, pos: [idx as u16 % $width * 60 + 5, idx as u16 / $width * 60 + 5], size: [50, 50] })
                    .collect()
            }};
        }

        match self {
            Self::Default {
                auto,
                meds,
                warp_songs,
            } => meds
                .into_iter()
                .enumerate()
                .map(|(idx, med)| CellLayout {
                    idx,
                    id: TrackerCellId::med_location(med),
                    pos: [idx as u16 * 60 + 5, 5],
                    size: [50, 18],
                })
                .chain(meds.into_iter().enumerate().map(|(idx, med)| CellLayout {
                    idx: idx + 6,
                    id: TrackerCellId::from(med),
                    pos: [idx as u16 * 60 + 5, 33],
                    size: [50, 50],
                }))
                .chain(vec![
                    CellLayout {
                        idx: 12,
                        id: AdultTradeNoChicken,
                        pos: [5, 93],
                        size: [50, 50],
                    },
                    CellLayout {
                        idx: 13,
                        id: Skulltula,
                        pos: [65, 93],
                        size: [50, 50],
                    },
                    CellLayout {
                        idx: 14,
                        id: KokiriEmeraldLocation,
                        pos: [125, 93],
                        size: [30, 10],
                    },
                    CellLayout {
                        idx: 15,
                        id: GoronRubyLocation,
                        pos: [165, 93],
                        size: [30, 10],
                    },
                    CellLayout {
                        idx: 16,
                        id: ZoraSapphireLocation,
                        pos: [205, 93],
                        size: [30, 10],
                    },
                    CellLayout {
                        idx: 17,
                        id: Bottle,
                        pos: [245, 93],
                        size: [50, 50],
                    },
                    CellLayout {
                        idx: 18,
                        id: Scale,
                        pos: [305, 93],
                        size: [50, 50],
                    },
                    CellLayout {
                        idx: 19,
                        id: KokiriEmerald,
                        pos: [125, 113],
                        size: [30, 30],
                    },
                    CellLayout {
                        idx: 20,
                        id: GoronRuby,
                        pos: [165, 113],
                        size: [30, 30],
                    },
                    CellLayout {
                        idx: 21,
                        id: ZoraSapphire,
                        pos: [205, 113],
                        size: [30, 30],
                    },
                ])
                .chain(
                    vec![
                        Slingshot,
                        Bombs,
                        Boomerang,
                        Strength,
                        MagicLens,
                        Spells,
                        Hookshot,
                        Bow,
                        Arrows,
                        Hammer,
                        Boots,
                        MirrorShield,
                        ChildTrade,
                        Ocarina,
                        Beans,
                        SwordCard,
                        Tunics,
                        if *auto { BigPoeTriforce } else { Triforce },
                        ZeldasLullaby,
                        EponasSong,
                        SariasSong,
                        SunsSong,
                        SongOfTime,
                        SongOfStorms,
                    ]
                    .into_iter()
                    .chain(warp_songs.into_iter().map(TrackerCellId::warp_song))
                    .enumerate()
                    .map(|(idx, id)| CellLayout {
                        idx: idx + 22,
                        id,
                        pos: [idx as u16 % 6 * 60 + 5, idx as u16 / 6 * 60 + 153],
                        size: [50, 50],
                    }),
                )
                .collect(),
            Self::MultiworldExpanded => columns!(
                4,
                [
                    SwordCard,
                    Slingshot,
                    Skulltula,
                    GoBk,
                    Bombs,
                    Bow,
                    ZeldasLullaby,
                    Minuet,
                    Boomerang,
                    Hammer,
                    EponasSong,
                    Bolero,
                    Hookshot,
                    Spells,
                    SariasSong,
                    Serenade,
                    Bottle,
                    Arrows,
                    SunsSong,
                    Requiem,
                    MirrorShield,
                    Strength,
                    SongOfTime,
                    Nocturne,
                    Boots,
                    Scale,
                    SongOfStorms,
                    Prelude,
                ]
            ),
            Self::MultiworldCollapsed => columns!(
                10,
                [
                    SwordCard,
                    Bottle,
                    Skulltula,
                    Strength,
                    Scale,
                    Spells,
                    Slingshot,
                    Bombs,
                    Boomerang,
                    GoBk,
                    ZeldasLullaby,
                    EponasSong,
                    SariasSong,
                    SunsSong,
                    SongOfTime,
                    SongOfStorms,
                    Hookshot,
                    Bow,
                    Hammer,
                    Magic,
                    Minuet,
                    Bolero,
                    Serenade,
                    Requiem,
                    Nocturne,
                    Prelude,
                    MirrorShield,
                    Boots,
                    Arrows,
                    Tunics, //TODO replace tunics with wallets once images exist
                ]
            ),
            Self::MultiworldEdit => vec![
                KokiriEmeraldLocation,
                GoronRubyLocation,
                ZoraSapphireLocation,
                LightMedallionLocation,
                ForestMedallionLocation,
                FireMedallionLocation,
                WaterMedallionLocation,
                ShadowMedallionLocation,
                SpiritMedallionLocation,
            ]
            .into_iter()
            .enumerate()
            .map(|(idx, id)| CellLayout {
                idx,
                id,
                pos: [idx as u16 * 40 + 5, 5],
                size: [30, 10],
            })
            .chain(
                vec![
                    KokiriEmerald,
                    GoronRuby,
                    ZoraSapphire,
                    LightMedallion,
                    ForestMedallion,
                    FireMedallion,
                    WaterMedallion,
                    ShadowMedallion,
                    SpiritMedallion,
                ]
                .into_iter()
                .enumerate()
                .map(|(idx, id)| CellLayout {
                    idx: idx + 9,
                    id,
                    pos: [idx as u16 * 40 + 5, 25],
                    size: [30, 30],
                }),
            )
            .chain(
                vec![
                    SwordCard,
                    Bottle,
                    Skulltula,
                    Scale,
                    Tunics,
                    GoBk, //TODO replace tunics with wallets once images exist
                    Slingshot,
                    Bombs,
                    Boomerang,
                    Strength,
                    Magic,
                    Spells,
                    Hookshot,
                    Bow,
                    Arrows,
                    Hammer,
                    Boots,
                    MirrorShield,
                    ZeldasLullaby,
                    EponasSong,
                    SariasSong,
                    SunsSong,
                    SongOfTime,
                    SongOfStorms,
                    Minuet,
                    Bolero,
                    Serenade,
                    Requiem,
                    Nocturne,
                    Prelude,
                ]
                .into_iter()
                .enumerate()
                .map(|(idx, id)| CellLayout {
                    idx: idx + 18,
                    id,
                    pos: [idx as u16 % 6 * 60 + 5, idx as u16 / 6 * 60 + 65],
                    size: [50, 50],
                }),
            )
            .collect(),
            Self::RslLeft => columns!(
                9,
                [
                    Slingshot,
                    Bombs,
                    Boomerang,
                    Skulltula,
                    GoMode,
                    GanonMq,
                    GanonKeys,
                    DekuMq,
                    Blank,
                    Hookshot,
                    Bow,
                    Hammer,
                    ZeldasLullaby,
                    Minuet,
                    ForestMq,
                    ForestKeys,
                    DcMq,
                    Blank,
                    Bottle,
                    Strength,
                    Scale,
                    EponasSong,
                    Bolero,
                    FireMq,
                    FireKeys,
                    JabuMq,
                    Blank,
                    ChildTrade,
                    Beans,
                    SwordCard,
                    SariasSong,
                    Serenade,
                    WaterMq,
                    WaterKeys,
                    IceMq,
                    Blank,
                    AdultTrade,
                    Tunics,
                    Triforce,
                    SunsSong,
                    Requiem,
                    SpiritMq,
                    SpiritKeys,
                    WellMq,
                    WellSmallKeys,
                    MagicLens,
                    Spells,
                    Arrows,
                    SongOfTime,
                    Nocturne,
                    ShadowMq,
                    ShadowKeys,
                    FortressMq,
                    FortressSmallKeys,
                    MirrorShield,
                    Boots,
                    Ocarina,
                    SongOfStorms,
                    Prelude,
                    FreeReward,
                    Blank,
                    GtgMq,
                    GtgSmallKeys,
                ]
            ),
            Self::RslRight => Self::RslLeft
                .cells()
                .into_iter()
                .chunks(9)
                .into_iter()
                .enumerate()
                .flat_map(|(row_idx, row)| {
                    row.collect_vec().into_iter().rev().enumerate().map(
                        move |(col_idx, CellLayout { id, size, .. })| CellLayout {
                            idx: row_idx * 9 + col_idx,
                            id,
                            pos: [col_idx as u16 * 60 + 5, row_idx as u16 * 60 + 5],
                            size,
                        },
                    )
                })
                .collect(),
            Self::RslEdit => {
                let mut cells = Self::MultiworldEdit.cells();
                cells[23].id = GoMode; // unlike multiworld, RSL doesn't track BK mode
                cells[28].id = MagicLens; // lens is not necessarily a starting item in RSL
                let num_cells_mw = cells.len();
                cells.extend(
                    vec![
                        ForestMq,
                        FireMq,
                        WaterMq,
                        SpiritMq,
                        ShadowMq,
                        GanonMq,
                        ForestKeys,
                        FireKeys,
                        WaterKeys,
                        SpiritKeys,
                        ShadowKeys,
                        GanonKeys,
                        DekuMq,
                        DcMq,
                        JabuMq,
                        WellMq,
                        FortressMq,
                        GtgMq,
                        ChildTrade,
                        Beans,
                        IceMq,
                        WellSmallKeys,
                        FortressSmallKeys,
                        GtgSmallKeys,
                        AdultTrade,
                        Triforce,
                        Ocarina,
                        Blank,
                        Blank,
                        Blank,
                    ]
                    .into_iter()
                    .enumerate()
                    .map(|(idx, id)| CellLayout {
                        idx: idx + num_cells_mw,
                        id,
                        pos: [idx as u16 % 6 * 60 + 5, idx as u16 / 6 * 60 + 5],
                        size: [50, 50],
                    }),
                );
                cells
            }
            Self::Rsl3Player => columns!(
                10,
                [
                    ZeldasLullaby,
                    Minuet,
                    Slingshot,
                    Bottle,
                    MagicLens,
                    Hammer,
                    FreeReward,
                    Blank,
                    DekuMq,
                    GoMode,
                    EponasSong,
                    Bolero,
                    Bombs,
                    Strength,
                    Spells,
                    SwordCard,
                    ForestMq,
                    ForestKeys,
                    DcMq,
                    Triforce,
                    SariasSong,
                    Serenade,
                    Boomerang,
                    Scale,
                    Arrows,
                    Ocarina,
                    FireMq,
                    FireKeys,
                    JabuMq,
                    Skulltula,
                    SunsSong,
                    Requiem,
                    Hookshot,
                    ChildTrade,
                    MirrorShield,
                    AdultTrade,
                    WaterMq,
                    WaterKeys,
                    WellMq,
                    WellSmallKeys,
                    SongOfTime,
                    Nocturne,
                    Bow,
                    Beans,
                    Boots,
                    Tunics,
                    ShadowMq,
                    ShadowKeys,
                    FortressMq,
                    FortressSmallKeys,
                    SongOfStorms,
                    Prelude,
                    IceMq,
                    Blank,
                    GanonMq,
                    GanonKeys,
                    SpiritMq,
                    SpiritKeys,
                    GtgMq,
                    GtgSmallKeys,
                ]
            ),
            Self::TsgMainWithRewardLocations => columns!(
                3,
                [
                    SwordShield,
                    Slingshot,
                    GoBk,
                    Bombs,
                    Bow,
                    ForestMedallionWithLocation,
                    Boomerang,
                    Hammer,
                    FireMedallionWithLocation,
                    Hookshot,
                    DinsFarores,
                    WaterMedallionWithLocation,
                    Bottle,
                    Arrows,
                    ShadowMedallionWithLocation,
                    MirrorShield,
                    Strength,
                    SpiritMedallionWithLocation,
                    Boots,
                    Scale,
                    LightMedallionWithLocation,
                    KokiriEmeraldWithLocation,
                    GoronRubyWithLocation,
                    ZoraSapphireWithLocation,
                ]
            ),
            Self::TsgMainWithRewardLocationsEdit => columns!(
                4,
                [
                    SwordShield,
                    Slingshot,
                    GoBk,
                    Blank,
                    Bombs,
                    Bow,
                    ForestMedallion,
                    ForestMedallionLocation,
                    Boomerang,
                    Hammer,
                    FireMedallion,
                    FireMedallionLocation,
                    Hookshot,
                    DinsFarores,
                    WaterMedallion,
                    WaterMedallionLocation,
                    Bottle,
                    Arrows,
                    ShadowMedallion,
                    ShadowMedallionLocation,
                    MirrorShield,
                    Strength,
                    SpiritMedallion,
                    SpiritMedallionLocation,
                    Boots,
                    Scale,
                    LightMedallion,
                    LightMedallionLocation,
                    KokiriEmerald,
                    GoronRuby,
                    ZoraSapphire,
                    Blank,
                    KokiriEmeraldLocation,
                    GoronRubyLocation,
                    ZoraSapphireLocation,
                    Blank,
                ]
            ),
            Self::TriforcePieces => columns!(1, [Triforce,]),

            // MM Layouts
            Self::MmDefault => {
                columns!(
                    6,
                    [
                        MmOdolwaRemains,
                        MmGohtRemains,
                        MmGyorgRemains,
                        MmTwinmoldRemains,
                        MmBomberNotebook,
                        MmBottle,
                        MmDekuMask,
                        MmGoronMask,
                        MmZoraMask,
                        MmFierceDeityMask,
                        MmMagic,
                        MmDoubleDefense,
                        MmOcarina,
                        MmHerosBow,
                        MmHookshot,
                        MmBombs,
                        MmPowderKeg,
                        MmLensOfTruth,
                        MmSongOfTime,
                        MmSongOfHealing,
                        MmEponasSong,
                        MmSongOfSoaring,
                        MmSongOfStorms,
                        MmSonataOfAwakening,
                        MmGoronLullaby,
                        MmNewWaveBossaNova,
                        MmElegyOfEmptiness,
                        MmOathToOrder,
                        MmStrayFairyClockTown,
                        Blank,
                        MmStrayFairyWoodfall,
                        MmStrayFairySnowhead,
                        MmStrayFairyGreatBay,
                        MmStrayFairyStoneTower,
                        Blank,
                        Blank,
                    ]
                )
            }

            Self::MmMasks => {
                columns!(
                    6,
                    [
                        MmDekuMask,
                        MmGoronMask,
                        MmZoraMask,
                        MmFierceDeityMask,
                        Blank,
                        Blank,
                        MmPostmanHat,
                        MmAllNightMask,
                        MmBlastMask,
                        MmStoneMask,
                        MmGreatFairyMask,
                        MmKeatonMask,
                        MmBremenMask,
                        MmBunnyHood,
                        MmDonGeroMask,
                        MmMaskOfScents,
                        MmRomaniMask,
                        MmCircusLeaderMask,
                        MmKafeiMask,
                        MmCouplesMask,
                        MmMaskOfTruth,
                        MmKamaroMask,
                        MmGibdoMask,
                        MmGaroMask,
                        MmCaptainHat,
                        MmGiantMask,
                        Blank,
                        Blank,
                        Blank,
                        Blank,
                    ]
                )
            }

            Self::MmBossRemains => {
                columns!(
                    4,
                    [
                        MmOdolwaRemains,
                        MmGohtRemains,
                        MmGyorgRemains,
                        MmTwinmoldRemains,
                    ]
                )
            }

            Self::MmStrayFairies => {
                columns!(
                    5,
                    [
                        MmStrayFairyWoodfall,
                        MmStrayFairySnowhead,
                        MmStrayFairyGreatBay,
                        MmStrayFairyStoneTower,
                        MmStrayFairyClockTown,
                    ]
                )
            }

            Self::MmSongs => {
                columns!(
                    5,
                    [
                        MmSongOfTime,
                        MmSongOfHealing,
                        MmEponasSong,
                        MmSongOfSoaring,
                        MmSongOfStorms,
                        MmSonataOfAwakening,
                        MmGoronLullaby,
                        MmNewWaveBossaNova,
                        MmElegyOfEmptiness,
                        MmOathToOrder,
                    ]
                )
            }

            Self::MmEquipment => {
                columns!(
                    6,
                    [
                        MmOcarina,
                        MmHerosBow,
                        MmHookshot,
                        MmBombs,
                        MmBombchu,
                        MmPowderKeg,
                        MmLensOfTruth,
                        MmPictographBox,
                        MmGreatFairySword,
                        MmMagicBean,
                        MmSword,
                        MmShield,
                        MmFireArrow,
                        MmIceArrow,
                        MmLightArrow,
                        MmBottle,
                        MmWallet,
                        MmMagic,
                    ]
                )
            }

            Self::DungeonItems => {
                columns!(
                    6,
                    [
                        DekuMap,
                        DcMap,
                        JabuMap,
                        ForestMap,
                        FireMap,
                        WaterMap,
                        ShadowMap,
                        SpiritMap,
                        WellMap,
                        IceMap,
                        GanonMap,
                        Blank,
                        DekuCompass,
                        DcCompass,
                        JabuCompass,
                        ForestCompass,
                        FireCompass,
                        WaterCompass,
                        ShadowCompass,
                        SpiritCompass,
                        WellCompass,
                        IceCompass,
                        Blank,
                        Blank,
                        ForestBossKey,
                        FireBossKey,
                        WaterBossKey,
                        ShadowBossKey,
                        SpiritBossKey,
                        GanonBossKey,
                        ForestSmallKeys,
                        FireSmallKeys,
                        WaterSmallKeys,
                        ShadowSmallKeys,
                        SpiritSmallKeys,
                        WellSmallKeys,
                        FortressSmallKeys,
                        GtgSmallKeys,
                        GanonSmallKeys,
                        Blank,
                        Blank,
                        Blank,
                    ]
                )
            }

            Self::MmDungeonItems => {
                columns!(
                    4,
                    [
                        MmWoodfallMap,
                        MmSnowheadMap,
                        MmGreatBayMap,
                        MmStoneTowerMap,
                        MmWoodfallCompass,
                        MmSnowheadCompass,
                        MmGreatBayCompass,
                        MmStoneTowerCompass,
                        MmWoodfallSmallKeys,
                        MmSnowheadSmallKeys,
                        MmGreatBaySmallKeys,
                        MmStoneTowerSmallKeys,
                        MmWoodfallBossKey,
                        MmSnowheadBossKey,
                        MmGreatBayBossKey,
                        MmStoneTowerBossKey,
                    ]
                )
            }

            Self::Combo => {
                columns!(
                    12,
                    [
                        ForestMedallion,
                        FireMedallion,
                        WaterMedallion,
                        ShadowMedallion,
                        SpiritMedallion,
                        LightMedallion,
                        MmOdolwaRemains,
                        MmGohtRemains,
                        MmGyorgRemains,
                        MmTwinmoldRemains,
                        KokiriEmerald,
                        GoronRuby,
                        ZoraSapphire,
                        Skulltula,
                        NumBottles,
                        Scale,
                        MmDekuMask,
                        MmGoronMask,
                        MmZoraMask,
                        MmFierceDeityMask,
                        MmBottle,
                        MmWallet,
                        MmMagic,
                        MmDoubleDefense,
                        Slingshot,
                        Bombs,
                        Boomerang,
                        Strength,
                        MagicLens,
                        Spells,
                        Hookshot,
                        Bow,
                        Arrows,
                        Hammer,
                        Boots,
                        MirrorShield,
                        ZeldasLullaby,
                        EponasSong,
                        SariasSong,
                        SunsSong,
                        SongOfTime,
                        SongOfStorms,
                        MmSongOfTime,
                        MmSongOfHealing,
                        MmEponasSong,
                        MmSongOfSoaring,
                        MmSongOfStorms,
                        MmSonataOfAwakening,
                        Minuet,
                        Bolero,
                        Serenade,
                        Requiem,
                        Nocturne,
                        Prelude,
                        MmGoronLullaby,
                        MmNewWaveBossaNova,
                        MmElegyOfEmptiness,
                        MmOathToOrder,
                        MmBomberNotebook,
                        Triforce,
                        MmOcarina,
                        MmHerosBow,
                        MmFireArrow,
                        MmIceArrow,
                        MmLightArrow,
                        MmHookshot,
                        MmBombs,
                        MmBombchu,
                        MmPowderKeg,
                        MmLensOfTruth,
                        MmSword,
                        MmShield,
                        ChildTrade,
                        Ocarina,
                        Beans,
                        SwordCard,
                        Tunics,
                        AdultTradeNoChicken,
                        MmGreatFairySword,
                        MmPictographBox,
                        MmMagicBean,
                        MmPostmanHat,
                        MmAllNightMask,
                        MmBlastMask,
                        MmStoneMask,
                        MmGreatFairyMask,
                        MmKeatonMask,
                        MmBremenMask,
                        MmBunnyHood,
                        MmDonGeroMask,
                        MmMaskOfScents,
                        MmRomaniMask,
                        MmCircusLeaderMask,
                        MmKafeiMask,
                        MmCouplesMask,
                        MmMaskOfTruth,
                        MmKamaroMask,
                        MmGibdoMask,
                        MmGaroMask,
                        MmCaptainHat,
                        MmGiantMask,
                        OotHearts,
                        Blank,
                        MmHearts,
                        Blank,
                        DekuMap,
                        DcMap,
                        JabuMap,
                        ForestMap,
                        FireMap,
                        WaterMap,
                        ShadowMap,
                        SpiritMap,
                        WellMap,
                        IceMap,
                        GanonMap,
                        Blank,
                        DekuCompass,
                        DcCompass,
                        JabuCompass,
                        ForestCompass,
                        FireCompass,
                        WaterCompass,
                        ShadowCompass,
                        SpiritCompass,
                        WellCompass,
                        IceCompass,
                        Blank,
                        Blank,
                        ForestBossKey,
                        FireBossKey,
                        WaterBossKey,
                        ShadowBossKey,
                        SpiritBossKey,
                        GanonBossKey,
                        Blank,
                        Blank,
                        Blank,
                        Blank,
                        Blank,
                        Blank,
                        ForestSmallKeys,
                        FireSmallKeys,
                        WaterSmallKeys,
                        ShadowSmallKeys,
                        SpiritSmallKeys,
                        WellSmallKeys,
                        FortressSmallKeys,
                        GtgSmallKeys,
                        GanonSmallKeys,
                        Blank,
                        Blank,
                        Blank,
                        MmWoodfallBossKey,
                        MmSnowheadBossKey,
                        MmGreatBayBossKey,
                        MmStoneTowerBossKey,
                        MmWoodfallMap,
                        MmSnowheadMap,
                        MmGreatBayMap,
                        MmStoneTowerMap,
                        MmWoodfallCompass,
                        MmSnowheadCompass,
                        MmGreatBayCompass,
                        MmStoneTowerCompass,
                        MmWoodfallSmallKeys,
                        MmSnowheadSmallKeys,
                        MmGreatBaySmallKeys,
                        MmStoneTowerSmallKeys,
                        MmStrayFairyClockTown,
                        MmStrayFairyWoodfall,
                        MmStrayFairySnowhead,
                        MmStrayFairyGreatBay,
                        MmStrayFairyStoneTower,
                        Blank,
                        Blank,
                        Blank,
                    ]
                )
            }
        }
    }

    /// Returns the number of columns in this layout based on cell positions.
    pub fn column_count(&self) -> usize {
        let cells = self.cells();
        if cells.is_empty() {
            return 0;
        }
        cells
            .iter()
            .map(|c| ((c.pos[0].saturating_sub(5)) / 60) as usize + 1)
            .max()
            .unwrap_or(0)
    }

    /// Returns the number of rows in this layout based on cell positions.
    pub fn row_count(&self) -> usize {
        let cells = self.cells();
        if cells.is_empty() {
            return 0;
        }
        let mut y_positions: Vec<u16> = cells.iter().map(|c| c.pos[1]).collect();
        y_positions.sort_unstable();
        y_positions.dedup();
        y_positions.len()
    }

    /// Returns the pixel dimensions (width, height) of this layout.
    pub fn pixel_dimensions(&self) -> (u32, u32) {
        let cells = self.cells();
        if cells.is_empty() {
            return (0, 0);
        }
        let max_x = cells
            .iter()
            .map(|c| c.pos[0] as u32 + c.size[0] as u32)
            .max()
            .unwrap_or(0);
        let max_y = cells
            .iter()
            .map(|c| c.pos[1] as u32 + c.size[1] as u32)
            .max()
            .unwrap_or(0);
        (max_x + 5, max_y + 5)
    }
}

impl Default for TrackerLayout {
    fn default() -> Self {
        Self::from(&Config::default())
    }
}

impl From<&Config> for TrackerLayout {
    fn from(config: &Config) -> Self {
        Self::Default {
            auto: false,
            meds: config.med_order,
            warp_songs: config.warp_song_order,
        }
    }
}

impl From<&Option<Config>> for TrackerLayout {
    fn from(config: &Option<Config>) -> Self {
        config.as_ref().map(Self::from).unwrap_or_default()
    }
}

#[cfg(feature = "rocket")]
impl fmt::Display for TrackerLayout {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Default { .. } if *self == TrackerLayout::default() => write!(f, "default"),
            Self::Default { .. } => unimplemented!(),
            Self::MultiworldExpanded => write!(f, "mw-expanded"),
            Self::MultiworldCollapsed => write!(f, "mw-collapsed"),
            Self::MultiworldEdit => write!(f, "mw-edit"),
            Self::RslLeft => write!(f, "rsl-left"),
            Self::RslRight => write!(f, "rsl-right"),
            Self::RslEdit => write!(f, "rsl-edit"),
            Self::Rsl3Player => write!(f, "rsl-3player"),
            Self::TsgMainWithRewardLocations => write!(f, "tsg-main-locs"),
            Self::TsgMainWithRewardLocationsEdit => write!(f, "tsg-main-locs-edit"),
            Self::TriforcePieces => write!(f, "triforce-pieces"),
            Self::MmDefault => write!(f, "mm-default"),
            Self::MmMasks => write!(f, "mm-masks"),
            Self::MmBossRemains => write!(f, "mm-boss-remains"),
            Self::MmStrayFairies => write!(f, "mm-stray-fairies"),
            Self::MmSongs => write!(f, "mm-songs"),
            Self::MmEquipment => write!(f, "mm-equipment"),
            Self::DungeonItems => write!(f, "dungeon-items"),
            Self::MmDungeonItems => write!(f, "mm-dungeon-items"),
            Self::Combo => write!(f, "combo"),
        }
    }
}

#[cfg(feature = "rocket")]
impl<'a> FromParam<'a> for TrackerLayout {
    type Error = ();

    fn from_param(param: &'a str) -> Result<Self, ()> {
        Ok(match param {
            "default" => Self::default(),
            "mw-expanded" => Self::MultiworldExpanded,
            "mw-collapsed" => Self::MultiworldCollapsed,
            "mw-edit" => Self::MultiworldEdit,
            "rsl-left" => Self::RslLeft,
            "rsl-right" => Self::RslRight,
            "rsl-edit" => Self::RslEdit,
            "rsl-3player" => Self::Rsl3Player,
            "tsg-main-locs" => Self::TsgMainWithRewardLocations,
            "tsg-main-locs-edit" => Self::TsgMainWithRewardLocationsEdit,
            "triforce-pieces" => Self::TriforcePieces,
            "mm-default" => Self::MmDefault,
            "mm-masks" => Self::MmMasks,
            "mm-boss-remains" => Self::MmBossRemains,
            "mm-stray-fairies" => Self::MmStrayFairies,
            "mm-songs" => Self::MmSongs,
            "mm-equipment" => Self::MmEquipment,
            "dungeon-items" => Self::DungeonItems,
            "mm-dungeon-items" => Self::MmDungeonItems,
            "combo" => Self::Combo,
            _ => return Err(()),
        })
    }
}

#[cfg(feature = "rocket")]
rocket::http::impl_from_uri_param_identity!([Path] TrackerLayout);

#[cfg(feature = "rocket")]
impl UriDisplay<Path> for TrackerLayout {
    fn fmt(&self, f: &mut Formatter<'_, Path>) -> fmt::Result {
        f.write_raw(format!("{}", self))
    }
}

/// A layout for a tracker displaying data from two players at once.
#[derive(Protocol)]
pub enum DoubleTrackerLayout {
    DungeonRewards,
}

impl DoubleTrackerLayout {
    pub fn cells(&self) -> Vec<DungeonReward> {
        match self {
            DoubleTrackerLayout::DungeonRewards => vec![
                DungeonReward::Stone(Stone::KokiriEmerald),
                DungeonReward::Stone(Stone::GoronRuby),
                DungeonReward::Stone(Stone::ZoraSapphire),
                DungeonReward::Medallion(Medallion::Forest),
                DungeonReward::Medallion(Medallion::Fire),
                DungeonReward::Medallion(Medallion::Water),
                DungeonReward::Medallion(Medallion::Shadow),
                DungeonReward::Medallion(Medallion::Spirit),
                DungeonReward::Medallion(Medallion::Light),
            ],
        }
    }
}

#[cfg(feature = "rocket")]
impl<'a> FromParam<'a> for DoubleTrackerLayout {
    type Error = ();

    fn from_param(param: &'a str) -> Result<DoubleTrackerLayout, ()> {
        Ok(match param {
            "dungeon-rewards" => DoubleTrackerLayout::DungeonRewards,
            _ => return Err(()),
        })
    }
}

#[cfg(feature = "rocket")]
impl fmt::Display for DoubleTrackerLayout {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DoubleTrackerLayout::DungeonRewards => write!(f, "dungeon-rewards"),
        }
    }
}
