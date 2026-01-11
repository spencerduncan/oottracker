//! Cell types and definitions for the tracker UI.
//!
//! This module contains TrackerCellKind, TrackerCellId, and the cells! macro
//! that generates all tracker cell definitions.

#![allow(unused_qualifications)] // oottracker::ui::TrackerCellKind::SmallKeys vs oottracker::save::SmallKeys

#[cfg(feature = "iced")]
use iced::keyboard::Modifiers as KeyboardModifiers;
use {
    super::{
        accessibility::AccessibilityStatus,
        images::{ImageDir, ImageInfo},
        render::{CellOverlay, CellRender, CellStyle, LocationStyle},
    },
    crate::{
        checks::CheckExt as _, info_tables::*, knowledge::ProgressionMode, save::*, ModelState,
    },
    async_proto::Protocol,
    collect_mac::collect,
    itertools::Itertools as _,
    ootr::{
        check::Check,
        model::{Dungeon, DungeonReward, DungeonRewardLocation, MainDungeon, Medallion, Stone},
        region::Mq,
    },
    std::{borrow::Cow, collections::HashMap, fmt, iter},
};

/// Type alias for functions that check two boolean states from ModelState
pub type StatePairChecker = Box<dyn Fn(&ModelState) -> (bool, bool)>;

/// Type alias for functions that set a u8 value on ModelState
pub type StateU8Setter = Box<dyn Fn(&mut ModelState, u8)>;

/// Type alias for functions that return an image with active state from ModelState
pub type StateImageGetter = Box<dyn Fn(&ModelState) -> (bool, ImageInfo)>;

/// Type alias for functions that set small keys count
pub type SmallKeysSetter = Box<dyn Fn(&mut crate::save::SmallKeys, u8)>;

/// Type alias for functions that get a u8 value from ModelState
pub type StateU8Getter = Box<dyn Fn(&ModelState) -> u8>;

/// Type alias for functions that set MM small keys count
pub type MmSmallKeysSetter = Box<dyn Fn(&mut crate::mm_save::MmSmallKeys, u8)>;

/// Extension trait for HashMap<DungeonReward, DungeonRewardLocation>
pub trait DungeonRewardLocationExt {
    fn increment(&mut self, key: DungeonReward);
    fn decrement(&mut self, key: DungeonReward);
}

impl DungeonRewardLocationExt for HashMap<DungeonReward, DungeonRewardLocation> {
    fn increment(&mut self, key: DungeonReward) {
        match self.get(&key) {
            None => self.insert(key, DungeonRewardLocation::Dungeon(MainDungeon::DekuTree)),
            Some(DungeonRewardLocation::Dungeon(MainDungeon::DekuTree)) => self.insert(
                key,
                DungeonRewardLocation::Dungeon(MainDungeon::DodongosCavern),
            ),
            Some(DungeonRewardLocation::Dungeon(MainDungeon::DodongosCavern)) => {
                self.insert(key, DungeonRewardLocation::Dungeon(MainDungeon::JabuJabu))
            }
            Some(DungeonRewardLocation::Dungeon(MainDungeon::JabuJabu)) => self.insert(
                key,
                DungeonRewardLocation::Dungeon(MainDungeon::ForestTemple),
            ),
            Some(DungeonRewardLocation::Dungeon(MainDungeon::ForestTemple)) => {
                self.insert(key, DungeonRewardLocation::Dungeon(MainDungeon::FireTemple))
            }
            Some(DungeonRewardLocation::Dungeon(MainDungeon::FireTemple)) => self.insert(
                key,
                DungeonRewardLocation::Dungeon(MainDungeon::WaterTemple),
            ),
            Some(DungeonRewardLocation::Dungeon(MainDungeon::WaterTemple)) => self.insert(
                key,
                DungeonRewardLocation::Dungeon(MainDungeon::ShadowTemple),
            ),
            Some(DungeonRewardLocation::Dungeon(MainDungeon::ShadowTemple)) => self.insert(
                key,
                DungeonRewardLocation::Dungeon(MainDungeon::SpiritTemple),
            ),
            Some(DungeonRewardLocation::Dungeon(MainDungeon::SpiritTemple)) => {
                self.insert(key, DungeonRewardLocation::LinksPocket)
            }
            Some(DungeonRewardLocation::LinksPocket) => self.remove(&key),
        };
    }

    fn decrement(&mut self, key: DungeonReward) {
        match self.get(&key) {
            None => self.insert(key, DungeonRewardLocation::LinksPocket),
            Some(DungeonRewardLocation::Dungeon(MainDungeon::DekuTree)) => self.remove(&key),
            Some(DungeonRewardLocation::Dungeon(MainDungeon::DodongosCavern)) => {
                self.insert(key, DungeonRewardLocation::Dungeon(MainDungeon::DekuTree))
            }
            Some(DungeonRewardLocation::Dungeon(MainDungeon::JabuJabu)) => self.insert(
                key,
                DungeonRewardLocation::Dungeon(MainDungeon::DodongosCavern),
            ),
            Some(DungeonRewardLocation::Dungeon(MainDungeon::ForestTemple)) => {
                self.insert(key, DungeonRewardLocation::Dungeon(MainDungeon::JabuJabu))
            }
            Some(DungeonRewardLocation::Dungeon(MainDungeon::FireTemple)) => self.insert(
                key,
                DungeonRewardLocation::Dungeon(MainDungeon::ForestTemple),
            ),
            Some(DungeonRewardLocation::Dungeon(MainDungeon::WaterTemple)) => {
                self.insert(key, DungeonRewardLocation::Dungeon(MainDungeon::FireTemple))
            }
            Some(DungeonRewardLocation::Dungeon(MainDungeon::ShadowTemple)) => self.insert(
                key,
                DungeonRewardLocation::Dungeon(MainDungeon::WaterTemple),
            ),
            Some(DungeonRewardLocation::Dungeon(MainDungeon::SpiritTemple)) => self.insert(
                key,
                DungeonRewardLocation::Dungeon(MainDungeon::ShadowTemple),
            ),
            Some(DungeonRewardLocation::LinksPocket) => self.insert(
                key,
                DungeonRewardLocation::Dungeon(MainDungeon::SpiritTemple),
            ),
        };
    }
}

pub enum TrackerCellKind {
    BigPoeTriforce, // auto-trackers show big Poe count unless at least 1 Triforce piece has been collected, manual mode only shows Triforce pieces
    BossKey {
        active: Box<dyn Fn(&AllDungeonItems) -> bool>,
        toggle: Box<dyn Fn(&mut AllDungeonItems)>,
        label: &'static str,
    },
    Composite {
        left_img: ImageInfo,
        right_img: ImageInfo,
        both_img: ImageInfo,
        active: StatePairChecker,
        toggle_left: Box<dyn Fn(&mut ModelState)>,
        toggle_right: Box<dyn Fn(&mut ModelState)>,
    },
    CompositeKeys {
        small: TrackerCellId,
        boss: TrackerCellId,
    },
    Count {
        dimmed_img: ImageInfo,
        img: ImageInfo,
        get: Box<dyn Fn(&ModelState) -> u8>,
        set: StateU8Setter,
        max: u8,
        step: u8,
    },
    /// Like Count, but with a dynamic max value from ModelState.
    /// Used for bottles where the max depends on settings.
    DynamicCount {
        dimmed_img: ImageInfo,
        img: ImageInfo,
        get: StateU8Getter,
        set: StateU8Setter,
        max_fn: StateU8Getter,
        step: u8,
    },
    FortressMq, // a cell kind used on Xopar's tracker to show whether Gerudo Fortress has 4 carpenters
    FreeReward,
    GoBk, // a combined go mode/BK mode/finished cell, used on the multiworld restream layout
    MagicLens, // magic meter with a Lens of Truth overlay, but auto-trackers/shift-click also show a different icon for double magic
    Medallion(Medallion),
    MedallionLocation(Medallion),
    MedallionWithLocation(Medallion),
    Mq(Dungeon),
    OptionalOverlay {
        main_img: ImageInfo,
        overlay_img: ImageInfo,
        active: StatePairChecker,
        toggle_main: Box<dyn Fn(&mut ModelState)>,
        toggle_overlay: Box<dyn Fn(&mut ModelState)>,
    },
    Overlay {
        main_img: ImageInfo,
        overlay_img: ImageInfo,
        active: StatePairChecker,
        toggle_main: Box<dyn Fn(&mut ModelState)>,
        toggle_overlay: Box<dyn Fn(&mut ModelState)>,
    },
    Sequence {
        idx: Box<dyn Fn(&ModelState) -> u8>,
        img: StateImageGetter,
        increment: Box<dyn Fn(&mut ModelState)>,
        decrement: Box<dyn Fn(&mut ModelState)>,
    },
    Simple {
        img: ImageInfo,
        active: Box<dyn Fn(&ModelState) -> bool>,
        toggle: Box<dyn Fn(&mut ModelState)>,
    },
    SmallKeys {
        get: Box<dyn Fn(&crate::save::SmallKeys) -> u8>,
        set: SmallKeysSetter,
        max_vanilla: u8,
        max_mq: u8,
        label: &'static str,
    },
    MmSmallKeys {
        get: Box<dyn Fn(&crate::mm_save::MmSmallKeys) -> u8>,
        set: MmSmallKeysSetter,
        max: u8,
        label: &'static str,
    },
    OotMap {
        active: Box<dyn Fn(&AllDungeonItems) -> bool>,
        toggle: Box<dyn Fn(&mut AllDungeonItems)>,
        label: &'static str,
    },
    OotCompass {
        active: Box<dyn Fn(&AllDungeonItems) -> bool>,
        toggle: Box<dyn Fn(&mut AllDungeonItems)>,
        label: &'static str,
    },
    MmBossKey {
        active: Box<dyn Fn(&crate::mm_save::MmAllDungeonItems) -> bool>,
        toggle: Box<dyn Fn(&mut crate::mm_save::MmAllDungeonItems)>,
        label: &'static str,
    },
    MmMap {
        active: Box<dyn Fn(&crate::mm_save::MmAllDungeonItems) -> bool>,
        toggle: Box<dyn Fn(&mut crate::mm_save::MmAllDungeonItems)>,
        label: &'static str,
    },
    MmCompass {
        active: Box<dyn Fn(&crate::mm_save::MmAllDungeonItems) -> bool>,
        toggle: Box<dyn Fn(&mut crate::mm_save::MmAllDungeonItems)>,
        label: &'static str,
    },
    Song {
        song: QuestItems,
        check: &'static str,
        toggle_overlay: Box<dyn Fn(&mut EventChkInf)>,
    },
    SongCheck {
        check: &'static str,
        toggle_overlay: Box<dyn Fn(&mut EventChkInf)>,
    },
    Spells, // composite Din's Fire & Farore's Wind, but auto-trackers/shift-click also toggle Nayru's Love
    Stone(Stone),
    StoneLocation(Stone),
    StoneWithLocation(Stone),
}

// Include the render and click implementations
include!("cells_impl.rs");

use TrackerCellKind::*;

macro_rules! cells {
    ($($cell:ident: $kind:expr,)*) => {
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Protocol)]
        pub enum TrackerCellId {
            $(
                $cell,
            )*
        }

        impl TrackerCellId {
            pub fn kind(&self) -> TrackerCellKind {
                match self {
                    $(TrackerCellId::$cell => $kind,)*
                }
            }
        }

        impl fmt::Display for TrackerCellId {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                match self {
                    $(TrackerCellId::$cell => write!(f, stringify!($cell)),)*
                }
            }
        }
    }
}

// Include the cells! macro invocation with all cell definitions
include!("cells_defs.rs");

impl TrackerCellId {
    pub fn med_location(med: Medallion) -> TrackerCellId {
        match med {
            Medallion::Light => TrackerCellId::LightMedallionLocation,
            Medallion::Forest => TrackerCellId::ForestMedallionLocation,
            Medallion::Fire => TrackerCellId::FireMedallionLocation,
            Medallion::Water => TrackerCellId::WaterMedallionLocation,
            Medallion::Shadow => TrackerCellId::ShadowMedallionLocation,
            Medallion::Spirit => TrackerCellId::SpiritMedallionLocation,
        }
    }

    pub fn warp_song(med: Medallion) -> TrackerCellId {
        match med {
            Medallion::Light => TrackerCellId::Prelude,
            Medallion::Forest => TrackerCellId::Minuet,
            Medallion::Fire => TrackerCellId::Bolero,
            Medallion::Water => TrackerCellId::Serenade,
            Medallion::Shadow => TrackerCellId::Nocturne,
            Medallion::Spirit => TrackerCellId::Requiem,
        }
    }
}

impl From<Medallion> for TrackerCellId {
    fn from(med: Medallion) -> TrackerCellId {
        match med {
            Medallion::Light => TrackerCellId::LightMedallion,
            Medallion::Forest => TrackerCellId::ForestMedallion,
            Medallion::Fire => TrackerCellId::FireMedallion,
            Medallion::Water => TrackerCellId::WaterMedallion,
            Medallion::Shadow => TrackerCellId::ShadowMedallion,
            Medallion::Spirit => TrackerCellId::SpiritMedallion,
        }
    }
}
