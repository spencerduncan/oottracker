#![allow(unused_qualifications)] // oottracker::ui::TrackerCellKind::SmallKeys vs oottracker::save::SmallKeys

#[cfg(feature = "iced")]
use iced::keyboard::Modifiers as KeyboardModifiers;
use {
    crate::{
        checks::CheckExt as _, info_tables::*, knowledge::ProgressionMode, save::*, ModelState,
    },
    async_proto::Protocol,
    collect_mac::collect,
    derivative::Derivative,
    directories::ProjectDirs,
    image::DynamicImage,
    itertools::Itertools as _,
    ootr::{
        check::Check,
        model::{Dungeon, DungeonReward, DungeonRewardLocation, MainDungeon, Medallion, Stone},
        region::Mq,
    },
    serde::{Deserialize, Serialize},
    std::{borrow::Cow, collections::HashMap, fmt, io, iter, path::PathBuf, sync::Arc, vec},
    tokio::{
        fs::{self, File},
        io::AsyncReadExt as _,
    },
    wheel::FromArc,
};
#[cfg(feature = "rocket")]
use {
    rocket::{
        http::uri::fmt::{Formatter, Path, UriDisplay},
        request::FromParam,
        response::content::RawHtml,
    },
    rocket_util::{html, ToHtml},
};

/// Type alias for functions that check two boolean states from ModelState
type StatePairChecker = Box<dyn Fn(&ModelState) -> (bool, bool)>;

/// Type alias for functions that set a u8 value on ModelState
type StateU8Setter = Box<dyn Fn(&mut ModelState, u8)>;

/// Type alias for functions that return an image with active state from ModelState
type StateImageGetter = Box<dyn Fn(&ModelState) -> (bool, ImageInfo)>;

/// Type alias for functions that set small keys count
type SmallKeysSetter = Box<dyn Fn(&mut crate::save::SmallKeys, u8)>;

/// Type alias for functions that set MM small keys count
type MmSmallKeysSetter = Box<dyn Fn(&mut crate::mm_save::MmSmallKeys, u8)>;

const VERSION: u8 = 0;

/// Accessibility status for a location/check.
///
/// This enum represents whether a location can be reached with the player's
/// current items and game state.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash, Protocol, Deserialize, Serialize)]
pub enum AccessibilityStatus {
    /// The location is accessible with current items.
    Accessible,
    /// The location is not accessible with current items.
    Inaccessible,
    /// The location has already been checked/collected.
    Checked,
    /// The accessibility status cannot be determined.
    #[default]
    Unknown,
}

impl AccessibilityStatus {
    /// Returns the CSS class name for this accessibility status.
    #[cfg(feature = "rocket")]
    pub fn css_class(&self) -> &'static str {
        match self {
            Self::Accessible => "accessible",
            Self::Inaccessible => "inaccessible",
            Self::Checked => "checked",
            Self::Unknown => "unknown",
        }
    }

    /// Returns an icon character representing the accessibility status.
    pub fn icon(&self) -> &'static str {
        match self {
            Self::Accessible => "✓",
            Self::Inaccessible => "✗",
            Self::Checked => "●",
            Self::Unknown => "?",
        }
    }

    /// Returns a human-readable description of the status.
    pub fn description(&self) -> &'static str {
        match self {
            Self::Accessible => "Accessible",
            Self::Inaccessible => "Not yet accessible",
            Self::Checked => "Already checked",
            Self::Unknown => "Unknown",
        }
    }
}

/// Summary of location accessibility counts.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Protocol, Deserialize, Serialize)]
pub struct AccessibilitySummary {
    /// Number of accessible locations.
    pub accessible: u32,
    /// Number of inaccessible locations.
    pub inaccessible: u32,
    /// Number of already checked locations.
    pub checked: u32,
    /// Number of locations with unknown status.
    pub unknown: u32,
}

impl AccessibilitySummary {
    /// Creates a new empty summary.
    pub fn new() -> Self {
        Self::default()
    }

    /// Returns the total number of locations.
    pub fn total(&self) -> u32 {
        self.accessible + self.inaccessible + self.checked + self.unknown
    }

    /// Adds a location with the given status to the summary.
    pub fn add(&mut self, status: AccessibilityStatus) {
        match status {
            AccessibilityStatus::Accessible => self.accessible += 1,
            AccessibilityStatus::Inaccessible => self.inaccessible += 1,
            AccessibilityStatus::Checked => self.checked += 1,
            AccessibilityStatus::Unknown => self.unknown += 1,
        }
    }
}

impl fmt::Display for AccessibilitySummary {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "✓{} / ✗{} / ●{} / ?{}",
            self.accessible, self.inaccessible, self.checked, self.unknown
        )
    }
}

#[cfg(feature = "rocket")]
impl ToHtml for AccessibilitySummary {
    fn to_html(&self) -> RawHtml<String> {
        html! {
            div(class = "accessibility-summary") {
                span(class = "accessible", title = "Accessible") : format!("✓{}", self.accessible);
                span(class = "separator") : " / ";
                span(class = "inaccessible", title = "Not yet accessible") : format!("✗{}", self.inaccessible);
                span(class = "separator") : " / ";
                span(class = "checked", title = "Already checked") : format!("●{}", self.checked);
            }
        }
    }
}

impl From<crate::checks::CheckStatus> for AccessibilityStatus {
    fn from(status: crate::checks::CheckStatus) -> Self {
        match status {
            crate::checks::CheckStatus::Checked => Self::Checked,
            crate::checks::CheckStatus::Reachable => Self::Accessible,
            crate::checks::CheckStatus::NotYetReachable => Self::Inaccessible,
        }
    }
}

/// Represents a single location with its accessibility information.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LocationAccessibility {
    /// The name of the location.
    pub name: String,
    /// The accessibility status of the location.
    pub status: AccessibilityStatus,
}

impl LocationAccessibility {
    /// Creates a new location accessibility entry.
    pub fn new(name: impl Into<String>, status: AccessibilityStatus) -> Self {
        Self {
            name: name.into(),
            status,
        }
    }
}

#[cfg(feature = "rocket")]
impl ToHtml for LocationAccessibility {
    fn to_html(&self) -> RawHtml<String> {
        html! {
            div(class = format!("location {}", self.status.css_class())) {
                span(class = "status-icon") : self.status.icon();
                span(class = "location-name") : &self.name;
            }
        }
    }
}

/// A list of locations with their accessibility status for display.
#[derive(Debug, Clone, Default)]
pub struct LocationAccessibilityList {
    /// The locations with their accessibility status.
    pub locations: Vec<LocationAccessibility>,
    /// Summary counts.
    pub summary: AccessibilitySummary,
}

impl LocationAccessibilityList {
    /// Creates a new empty list.
    pub fn new() -> Self {
        Self::default()
    }

    /// Adds a location to the list.
    pub fn add(&mut self, name: impl Into<String>, status: AccessibilityStatus) {
        self.locations
            .push(LocationAccessibility::new(name, status));
        self.summary.add(status);
    }

    /// Returns only the accessible locations.
    pub fn accessible(&self) -> impl Iterator<Item = &LocationAccessibility> {
        self.locations
            .iter()
            .filter(|loc| loc.status == AccessibilityStatus::Accessible)
    }

    /// Returns only the inaccessible locations.
    pub fn inaccessible(&self) -> impl Iterator<Item = &LocationAccessibility> {
        self.locations
            .iter()
            .filter(|loc| loc.status == AccessibilityStatus::Inaccessible)
    }
}

#[cfg(feature = "rocket")]
impl ToHtml for LocationAccessibilityList {
    fn to_html(&self) -> RawHtml<String> {
        html! {
            div(class = "location-accessibility-list") {
                div(class = "summary") {
                    : &self.summary;
                }
                div(class = "locations") {
                    @for loc in &self.locations {
                        : loc;
                    }
                }
            }
        }
    }
}

#[derive(Debug, FromArc, Clone)]
pub enum Error {
    #[from_arc]
    Io(Arc<io::Error>),
    #[from_arc]
    Json(Arc<serde_json::Error>),
    MissingHomeDir,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::Io(e) => write!(f, "I/O error: {}", e),
            Error::Json(e) => e.fmt(f),
            Error::MissingHomeDir => write!(f, "could not find your user folder"),
        }
    }
}

#[derive(Derivative, Debug, Clone, Deserialize, Serialize)]
#[derivative(Default)]
#[serde(rename_all = "camelCase")]
pub struct Config {
    #[derivative(Default(value = "ElementOrder::LightShadowSpirit"))]
    #[serde(default = "default_med_order")]
    pub med_order: ElementOrder,
    #[derivative(Default(value = "ElementOrder::SpiritShadowLight"))]
    #[serde(default = "default_warp_song_order")]
    pub warp_song_order: ElementOrder,
    #[serde(default)]
    pub layout_preference: LayoutPreference,
    pub auto_update_check: Option<bool>,
    /// Path to an MP3 file to play when items are collected
    #[serde(default)]
    pub item_fanfare_path: Option<PathBuf>,
    #[derivative(Default(value = "VERSION"))]
    pub version: u8,
}

impl Config {
    /// If the config file doesn't exist, this returns `Ok(None)`, so that the welcome message can be displayed.
    pub async fn new() -> Result<Option<Config>, Error> {
        let dirs = dirs()?;
        let mut file = match File::open(dirs.config_dir().join("config.json")).await {
            Ok(file) => file,
            Err(e) if e.kind() == io::ErrorKind::NotFound => return Ok(None),
            Err(e) => return Err(e.into()),
        };
        let mut buf = String::default();
        file.read_to_string(&mut buf).await?;
        Ok(Some(serde_json::from_str(&buf)?)) //TODO use async-json instead
    }

    pub fn new_sync() -> Result<Option<Config>, Error> {
        let dirs = dirs()?;
        match std::fs::File::open(dirs.config_dir().join("config.json")) {
            Ok(file) => Ok(Some(serde_json::from_reader(file)?)),
            Err(e) if e.kind() == io::ErrorKind::NotFound => Ok(None),
            Err(e) => Err(e.into()),
        }
    }

    pub async fn save(&self) -> Result<(), Error> {
        let dirs = dirs()?;
        let buf = serde_json::to_vec_pretty(self)?; //TODO use async-json instead
        fs::create_dir_all(dirs.config_dir()).await?;
        fs::write(dirs.config_dir().join("config.json"), &buf).await?;
        Ok(())
    }

    pub fn save_sync(&self) -> Result<(), Error> {
        let dirs = dirs()?;
        let buf = serde_json::to_vec_pretty(self)?; //TODO indent by 4 spaces, sort object keys, add trailing newline
        std::fs::create_dir_all(dirs.config_dir())?;
        std::fs::write(dirs.config_dir().join("config.json"), &buf)?;
        Ok(())
    }
}

#[derive(
    Debug, Clone, Copy, PartialEq, Eq, enum_iterator::Sequence, Deserialize, Serialize, Protocol,
)]
#[serde(rename_all = "camelCase")]
pub enum ElementOrder {
    LightShadowSpirit,
    LightSpiritShadow,
    ShadowSpiritLight,
    SpiritShadowLight,
}

impl IntoIterator for ElementOrder {
    type IntoIter = vec::IntoIter<Medallion>;
    type Item = Medallion;

    fn into_iter(self) -> vec::IntoIter<Medallion> {
        use Medallion::*;

        match self {
            ElementOrder::LightShadowSpirit => vec![Light, Forest, Fire, Water, Shadow, Spirit],
            ElementOrder::LightSpiritShadow => vec![Light, Forest, Fire, Water, Spirit, Shadow],
            ElementOrder::ShadowSpiritLight => vec![Forest, Fire, Water, Shadow, Spirit, Light],
            ElementOrder::SpiritShadowLight => vec![Forest, Fire, Water, Spirit, Shadow, Light],
        }
        .into_iter()
    }
}

impl fmt::Display for ElementOrder {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ElementOrder::LightShadowSpirit => write!(f, "Light first, Shadow before Spirit"),
            ElementOrder::LightSpiritShadow => write!(f, "Light first, Spirit before Shadow"),
            ElementOrder::ShadowSpiritLight => write!(f, "Shadow before Spirit, Light last"),
            ElementOrder::SpiritShadowLight => write!(f, "Spirit before Shadow, Light last"),
        }
    }
}

/// Layout preference for the tracker GUI, allowing selection between OoT, MM, or Combo tracker views.
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Default,
    enum_iterator::Sequence,
    Deserialize,
    Serialize,
    Protocol,
)]
#[serde(rename_all = "camelCase")]
pub enum LayoutPreference {
    /// Ocarina of Time tracker layout (default)
    #[default]
    Oot,
    /// Majora's Mask tracker layout
    Mm,
    /// Combined OoT/MM tracker layout
    Combo,
    /// OoT dungeon items (maps, compasses)
    DungeonItems,
    /// MM dungeon items (maps, compasses, keys)
    MmDungeonItems,
    /// MM stray fairy counters
    MmStrayFairies,
}

impl fmt::Display for LayoutPreference {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LayoutPreference::Oot => write!(f, "Ocarina of Time"),
            LayoutPreference::Mm => write!(f, "Majora's Mask"),
            LayoutPreference::Combo => write!(f, "Combo (OoT + MM)"),
            LayoutPreference::DungeonItems => write!(f, "OoT Dungeon Items"),
            LayoutPreference::MmDungeonItems => write!(f, "MM Dungeon Items"),
            LayoutPreference::MmStrayFairies => write!(f, "MM Stray Fairies"),
        }
    }
}

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
    },
    OotCompass {
        active: Box<dyn Fn(&AllDungeonItems) -> bool>,
        toggle: Box<dyn Fn(&mut AllDungeonItems)>,
        loc: ImageInfo,
    },
    MmBossKey {
        active: Box<dyn Fn(&crate::mm_save::MmAllDungeonItems) -> bool>,
        toggle: Box<dyn Fn(&mut crate::mm_save::MmAllDungeonItems)>,
        label: &'static str,
    },
    MmMap {
        active: Box<dyn Fn(&crate::mm_save::MmAllDungeonItems) -> bool>,
        toggle: Box<dyn Fn(&mut crate::mm_save::MmAllDungeonItems)>,
    },
    MmCompass {
        active: Box<dyn Fn(&crate::mm_save::MmAllDungeonItems) -> bool>,
        toggle: Box<dyn Fn(&mut crate::mm_save::MmAllDungeonItems)>,
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

impl TrackerCellKind {
    pub fn render(&self, state: &ModelState) -> CellRender {
        match self {
            BigPoeTriforce => {
                if state.ram.save.triforce_pieces() > 0 {
                    CellRender {
                        img: ImageInfo::new("triforce"),
                        style: CellStyle::Normal,
                        overlay: CellOverlay::Count {
                            count: state.ram.save.triforce_pieces(),
                            max: 0, // Triforce max is configurable and unknown
                            count_img: ImageInfo::new("force"),
                        },
                        accessibility: None,
                        label: None,
                    }
                } else if state.ram.save.big_poes > 0 {
                    //TODO show dimmed Triforce icon if it's known that it's TH
                    CellRender {
                        img: ImageInfo::extra("big_poe"),
                        style: CellStyle::Normal,
                        overlay: CellOverlay::Count {
                            count: state.ram.save.big_poes,
                            max: 10,
                            count_img: ImageInfo::extra("poes"),
                        },
                        accessibility: None,
                        label: None,
                    }
                } else {
                    CellRender {
                        img: ImageInfo::extra("big_poe"),
                        style: CellStyle::Dimmed,
                        overlay: CellOverlay::None,
                        accessibility: None,
                        label: None,
                    }
                }
            }
            BossKey { active, label, .. } => CellRender {
                img: ImageInfo::extra("boss_key"),
                style: if active(&state.ram.save.dungeon_items) {
                    CellStyle::Normal
                } else {
                    CellStyle::Dimmed
                },
                overlay: CellOverlay::None,
                accessibility: None,
                label: Some(label.to_string()),
            },
            OotMap { active, .. } => CellRender {
                img: ImageInfo::extra("map"),
                style: if active(&state.ram.save.dungeon_items) {
                    CellStyle::Normal
                } else {
                    CellStyle::Dimmed
                },
                overlay: CellOverlay::None,
                accessibility: None,
                label: None,
            },
            OotCompass { active, loc, .. } => CellRender {
                img: ImageInfo::extra("compass"),
                style: if active(&state.ram.save.dungeon_items) {
                    CellStyle::Normal
                } else {
                    CellStyle::Dimmed
                },
                overlay: CellOverlay::Location {
                    loc: loc.clone(),
                    style: LocationStyle::Normal,
                },
                accessibility: None,
                label: None,
            },
            MmBossKey { active, label, .. } => CellRender {
                img: ImageInfo::extra("boss_key"),
                style: if state
                    .ram
                    .mm_save
                    .as_ref()
                    .is_some_and(|mm| active(&mm.dungeon_items))
                {
                    CellStyle::Normal
                } else {
                    CellStyle::Dimmed
                },
                overlay: CellOverlay::None,
                accessibility: None,
                label: Some(label.to_string()),
            },
            MmMap { active, .. } => CellRender {
                img: ImageInfo::extra("map"),
                style: if state
                    .ram
                    .mm_save
                    .as_ref()
                    .is_some_and(|mm| active(&mm.dungeon_items))
                {
                    CellStyle::Normal
                } else {
                    CellStyle::Dimmed
                },
                overlay: CellOverlay::None,
                accessibility: None,
                label: None,
            },
            MmCompass { active, .. } => CellRender {
                img: ImageInfo::extra("compass"),
                style: if state
                    .ram
                    .mm_save
                    .as_ref()
                    .is_some_and(|mm| active(&mm.dungeon_items))
                {
                    CellStyle::Normal
                } else {
                    CellStyle::Dimmed
                },
                overlay: CellOverlay::None,
                accessibility: None,
                label: None,
            },
            Composite {
                left_img,
                right_img,
                both_img,
                active,
                ..
            } => {
                let is_active = active(state);
                let img = match is_active {
                    (false, false) | (true, true) => both_img,
                    (false, true) => right_img,
                    (true, false) => left_img,
                }
                .clone();
                CellRender {
                    img,
                    style: if let (false, false) = is_active {
                        CellStyle::Dimmed
                    } else {
                        CellStyle::Normal
                    },
                    overlay: CellOverlay::None,
                    accessibility: None,
                    label: None,
                }
            }
            CompositeKeys { boss, small } => {
                let (has_boss_key, num_small_keys, max_keys, label) = if let (
                    BossKey { active, label, .. },
                    TrackerCellKind::SmallKeys {
                        get,
                        max_vanilla,
                        max_mq,
                        ..
                    },
                ) = (boss.kind(), small.kind())
                {
                    (
                        active(&state.ram.save.dungeon_items),
                        get(&state.ram.save.small_keys),
                        max_vanilla.max(max_mq),
                        label,
                    )
                } else {
                    unimplemented!("CompositeKeys that aren't SmallKeys + BossKey")
                };
                CellRender {
                    img: ImageInfo::extra("keys"),
                    style: match (has_boss_key, num_small_keys) {
                        (false, 0) => CellStyle::Dimmed,
                        (false, _) => CellStyle::LeftDimmed,
                        (true, 0) => CellStyle::RightDimmed,
                        (true, _) => CellStyle::Normal,
                    },
                    overlay: if num_small_keys > 0 {
                        CellOverlay::Count {
                            count: num_small_keys,
                            max: max_keys,
                            count_img: ImageInfo::new("UNIMPLEMENTED"), //TODO
                        }
                    } else {
                        CellOverlay::None
                    },
                    accessibility: None,
                    label: Some(label.to_string()),
                }
            }
            Count {
                dimmed_img,
                img,
                get,
                max,
                ..
            } => {
                let count = get(state);
                let (style, overlay) = if count == 0 {
                    (CellStyle::Dimmed, CellOverlay::None)
                } else {
                    (
                        CellStyle::Normal,
                        CellOverlay::CountWithMax {
                            count,
                            max: *max,
                            count_img: img.clone(),
                        },
                    )
                };
                CellRender {
                    img: dimmed_img.clone(),
                    style,
                    overlay,
                    accessibility: None,
                    label: None,
                }
            }
            FortressMq => {
                CellRender {
                    img: ImageInfo::extra("blank"),
                    style: CellStyle::Normal,
                    overlay: CellOverlay::Location {
                        loc: ImageInfo::extra("fort_text"),
                        style: if state
                            .knowledge
                            .string_settings
                            .get("gerudo_fortress")
                            .is_some_and(|values| values.iter().eq(iter::once("normal")))
                        {
                            LocationStyle::Mq
                        } else {
                            LocationStyle::Normal
                        }, //TODO dim if unknown?
                    },
                    accessibility: None,
                    label: None,
                }
            }
            FreeReward => {
                let reward = state
                    .knowledge
                    .dungeon_reward_locations
                    .iter()
                    .filter_map(|(reward, &loc)| {
                        if loc == DungeonRewardLocation::LinksPocket {
                            Some(reward)
                        } else {
                            None
                        }
                    })
                    .exactly_one()
                    .ok();
                CellRender {
                    img: ImageInfo {
                        dir: if reward.is_some() {
                            ImageDir::Xopar
                        } else {
                            ImageDir::Extra
                        },
                        name: match reward {
                            Some(DungeonReward::Medallion(med)) => Cow::Owned(format!(
                                "{}_medallion",
                                med.element().to_ascii_lowercase()
                            )),
                            Some(DungeonReward::Stone(Stone::KokiriEmerald)) => {
                                Cow::Borrowed("kokiri_emerald")
                            }
                            Some(DungeonReward::Stone(Stone::GoronRuby)) => {
                                Cow::Borrowed("goron_ruby")
                            }
                            Some(DungeonReward::Stone(Stone::ZoraSapphire)) => {
                                Cow::Borrowed("zora_sapphire")
                            }
                            None => Cow::Borrowed("blank"), //TODO "unknown dungeon reward" image?
                        },
                    },
                    style: CellStyle::Normal,
                    overlay: CellOverlay::Location {
                        loc: ImageInfo::new("free_text"),
                        style: LocationStyle::Normal,
                    },
                    accessibility: None,
                    label: None,
                }
            }
            GoBk => CellRender {
                img: ImageInfo::extra(match state.knowledge.progression_mode {
                    ProgressionMode::Done => "blank",
                    ProgressionMode::Bk => "bk_mode",
                    ProgressionMode::Go | ProgressionMode::Normal => "go_mode",
                }),
                style: if state.knowledge.progression_mode == ProgressionMode::Normal {
                    CellStyle::Dimmed
                } else {
                    CellStyle::Normal
                },
                overlay: CellOverlay::None, //TODO overlay with finish time?
                accessibility: None,
                label: None,
            },
            MagicLens => CellRender {
                img: if state.ram.save.magic == MagicCapacity::Large {
                    ImageInfo::new("magic")
                } else {
                    ImageInfo::extra("small_magic")
                },
                style: if state.ram.save.magic == MagicCapacity::None {
                    CellStyle::Dimmed
                } else {
                    CellStyle::Normal
                },
                overlay: if state.ram.save.inv.lens {
                    CellOverlay::Image(ImageInfo::new("lens"))
                } else {
                    CellOverlay::None
                },
                accessibility: None,
                label: None,
            },
            Medallion(med) => CellRender {
                img: ImageInfo::new(format!("{}_medallion", med.element().to_ascii_lowercase())),
                style: if state.ram.save.quest_items.has(*med) {
                    CellStyle::Normal
                } else {
                    CellStyle::Dimmed
                },
                overlay: CellOverlay::None,
                accessibility: None,
                label: None,
            },
            MedallionLocation(med) => {
                let location = state
                    .knowledge
                    .dungeon_reward_locations
                    .get(&DungeonReward::Medallion(*med));
                CellRender {
                    img: ImageInfo::new(match location {
                        None => "unknown_text",
                        Some(DungeonRewardLocation::Dungeon(MainDungeon::DekuTree)) => "deku_text",
                        Some(DungeonRewardLocation::Dungeon(MainDungeon::DodongosCavern)) => {
                            "dc_text"
                        }
                        Some(DungeonRewardLocation::Dungeon(MainDungeon::JabuJabu)) => "jabu_text",
                        Some(DungeonRewardLocation::Dungeon(MainDungeon::ForestTemple)) => {
                            "forest_text"
                        }
                        Some(DungeonRewardLocation::Dungeon(MainDungeon::FireTemple)) => {
                            "fire_text"
                        }
                        Some(DungeonRewardLocation::Dungeon(MainDungeon::WaterTemple)) => {
                            "water_text"
                        }
                        Some(DungeonRewardLocation::Dungeon(MainDungeon::ShadowTemple)) => {
                            "shadow_text"
                        }
                        Some(DungeonRewardLocation::Dungeon(MainDungeon::SpiritTemple)) => {
                            "spirit_text"
                        }
                        Some(DungeonRewardLocation::LinksPocket) => "free_text",
                    }),
                    style: if location.is_some() {
                        CellStyle::Normal
                    } else {
                        CellStyle::Dimmed
                    },
                    overlay: CellOverlay::None,
                    accessibility: None,
                    label: None,
                }
            }
            MedallionWithLocation(med) => {
                let location = state
                    .knowledge
                    .dungeon_reward_locations
                    .get(&DungeonReward::Medallion(*med));
                let has_medallion = state.ram.save.quest_items.has(*med);
                CellRender {
                    img: ImageInfo::new(format!(
                        "{}_medallion",
                        med.element().to_ascii_lowercase()
                    )),
                    style: if has_medallion {
                        CellStyle::Normal
                    } else {
                        CellStyle::Dimmed
                    },
                    overlay: CellOverlay::Location {
                        loc: ImageInfo::new(match location {
                            None => "unknown_text",
                            Some(DungeonRewardLocation::Dungeon(MainDungeon::DekuTree)) => {
                                "deku_text"
                            }
                            Some(DungeonRewardLocation::Dungeon(MainDungeon::DodongosCavern)) => {
                                "dc_text"
                            }
                            Some(DungeonRewardLocation::Dungeon(MainDungeon::JabuJabu)) => {
                                "jabu_text"
                            }
                            Some(DungeonRewardLocation::Dungeon(MainDungeon::ForestTemple)) => {
                                "forest_text"
                            }
                            Some(DungeonRewardLocation::Dungeon(MainDungeon::FireTemple)) => {
                                "fire_text"
                            }
                            Some(DungeonRewardLocation::Dungeon(MainDungeon::WaterTemple)) => {
                                "water_text"
                            }
                            Some(DungeonRewardLocation::Dungeon(MainDungeon::ShadowTemple)) => {
                                "shadow_text"
                            }
                            Some(DungeonRewardLocation::Dungeon(MainDungeon::SpiritTemple)) => {
                                "spirit_text"
                            }
                            Some(DungeonRewardLocation::LinksPocket) => "free_text",
                        }),
                        style: if location.is_some() {
                            LocationStyle::Normal
                        } else {
                            LocationStyle::Dimmed
                        },
                    },
                    // Show accessibility status: Checked if medallion obtained
                    accessibility: if has_medallion {
                        Some(AccessibilityStatus::Checked)
                    } else {
                        None
                    },
                    label: None,
                }
            }
            Mq(dungeon) => {
                let reward = if let Dungeon::Main(main_dungeon) = *dungeon {
                    state
                        .knowledge
                        .dungeon_reward_locations
                        .iter()
                        .filter_map(|(reward, &loc)| {
                            if loc == DungeonRewardLocation::Dungeon(main_dungeon) {
                                Some(reward)
                            } else {
                                None
                            }
                        })
                        .exactly_one()
                        .ok()
                } else {
                    None
                };
                CellRender {
                    img: ImageInfo {
                        dir: if reward.is_some() {
                            ImageDir::Xopar
                        } else {
                            ImageDir::Extra
                        },
                        name: match reward {
                            Some(DungeonReward::Medallion(med)) => Cow::Owned(format!(
                                "{}_medallion",
                                med.element().to_ascii_lowercase()
                            )),
                            Some(DungeonReward::Stone(Stone::KokiriEmerald)) => {
                                Cow::Borrowed("kokiri_emerald")
                            }
                            Some(DungeonReward::Stone(Stone::GoronRuby)) => {
                                Cow::Borrowed("goron_ruby")
                            }
                            Some(DungeonReward::Stone(Stone::ZoraSapphire)) => {
                                Cow::Borrowed("zora_sapphire")
                            }
                            None => Cow::Borrowed("blank"), //TODO "unknown dungeon reward" image? (only for dungeons that have rewards)
                        },
                    },
                    style: if reward.is_some_and(|&reward| state.ram.save.quest_items.has(reward)) {
                        CellStyle::Normal
                    } else {
                        CellStyle::Dimmed
                    },
                    overlay: CellOverlay::Location {
                        loc: ImageInfo {
                            dir: if let Dungeon::Main(_) = dungeon {
                                ImageDir::Xopar
                            } else {
                                ImageDir::Extra
                            },
                            name: Cow::Borrowed(match dungeon {
                                Dungeon::Main(MainDungeon::DekuTree) => "deku_text",
                                Dungeon::Main(MainDungeon::DodongosCavern) => "dc_text",
                                Dungeon::Main(MainDungeon::JabuJabu) => "jabu_text",
                                Dungeon::Main(MainDungeon::ForestTemple) => "forest_text",
                                Dungeon::Main(MainDungeon::FireTemple) => "fire_text",
                                Dungeon::Main(MainDungeon::WaterTemple) => "water_text",
                                Dungeon::Main(MainDungeon::ShadowTemple) => "shadow_text",
                                Dungeon::Main(MainDungeon::SpiritTemple) => "spirit_text",
                                Dungeon::IceCavern => "ice_text",
                                Dungeon::BottomOfTheWell => "well_text",
                                Dungeon::GerudoTrainingGround => "gtg_text",
                                Dungeon::GanonsCastle => "ganon_text",
                            }),
                        },
                        style: if state.knowledge.mq.get(dungeon) == Some(&Mq::Mq) {
                            LocationStyle::Mq
                        } else {
                            LocationStyle::Normal
                        },
                    },
                    accessibility: None,
                    label: None,
                }
            }
            OptionalOverlay {
                main_img,
                overlay_img,
                active,
                ..
            }
            | Overlay {
                main_img,
                overlay_img,
                active,
                ..
            } => {
                let (main_active, overlay_active) = active(state);
                CellRender {
                    img: main_img.clone(),
                    style: if main_active {
                        CellStyle::Normal
                    } else {
                        CellStyle::Dimmed
                    },
                    overlay: if overlay_active {
                        CellOverlay::Image(overlay_img.clone())
                    } else {
                        CellOverlay::None
                    },
                    accessibility: None,
                    label: None,
                }
            }
            Sequence { img, .. } => {
                let (is_active, img) = img(state);
                CellRender {
                    img,
                    style: if is_active {
                        CellStyle::Normal
                    } else {
                        CellStyle::Dimmed
                    },
                    overlay: CellOverlay::None,
                    accessibility: None,
                    label: None,
                }
            }
            Simple { img, active, .. } => CellRender {
                img: img.clone(),
                style: if active(state) {
                    CellStyle::Normal
                } else {
                    CellStyle::Dimmed
                },
                overlay: CellOverlay::None,
                accessibility: None,
                label: None,
            },
            TrackerCellKind::SmallKeys {
                get,
                max_vanilla,
                max_mq,
                label,
                ..
            } => {
                let num_small_keys = get(&state.ram.save.small_keys);
                let max_keys = *max_vanilla.max(max_mq);
                CellRender {
                    img: ImageInfo::extra("small_key"),
                    style: if num_small_keys > 0 {
                        CellStyle::Normal
                    } else {
                        CellStyle::Dimmed
                    },
                    overlay: if num_small_keys > 0 {
                        CellOverlay::Count {
                            count: num_small_keys,
                            max: max_keys,
                            count_img: ImageInfo::new("UNIMPLEMENTED"), //TODO
                        }
                    } else {
                        CellOverlay::None
                    },
                    accessibility: None,
                    label: Some(label.to_string()),
                }
            }
            TrackerCellKind::MmSmallKeys { get, max, label, .. } => {
                let num_small_keys = state
                    .ram
                    .mm_save
                    .as_ref()
                    .map(|s| get(&s.small_keys))
                    .unwrap_or(0);
                CellRender {
                    img: ImageInfo::extra("small_key"),
                    style: if num_small_keys > 0 {
                        CellStyle::Normal
                    } else {
                        CellStyle::Dimmed
                    },
                    overlay: if num_small_keys > 0 {
                        CellOverlay::Count {
                            count: num_small_keys,
                            max: *max,
                            count_img: ImageInfo::new("UNIMPLEMENTED"), //TODO
                        }
                    } else {
                        CellOverlay::None
                    },
                    accessibility: None,
                    label: Some(label.to_string()),
                }
            }
            Song { song, check, .. } => {
                let is_check_completed = Check::<ootr_static::Rando>::Location(check.to_string())
                    .checked(state)
                    .unwrap_or(None)
                    .unwrap_or(false);
                CellRender {
                    img: ImageInfo::new(match *song {
                        QuestItems::ZELDAS_LULLABY => "lullaby",
                        QuestItems::EPONAS_SONG => "epona",
                        QuestItems::SARIAS_SONG => "saria",
                        QuestItems::SUNS_SONG => "sun",
                        QuestItems::SONG_OF_TIME => "time",
                        QuestItems::SONG_OF_STORMS => "storms",
                        QuestItems::MINUET_OF_FOREST => "minuet",
                        QuestItems::BOLERO_OF_FIRE => "bolero",
                        QuestItems::SERENADE_OF_WATER => "serenade",
                        QuestItems::NOCTURNE_OF_SHADOW => "nocturne",
                        QuestItems::REQUIEM_OF_SPIRIT => "requiem",
                        QuestItems::PRELUDE_OF_LIGHT => "prelude",
                        _ => unreachable!(),
                    }),
                    style: if state.ram.save.quest_items.contains(*song) {
                        CellStyle::Normal
                    } else {
                        CellStyle::Dimmed
                    },
                    overlay: if is_check_completed {
                        //TODO allow ootr_dynamic::Rando
                        CellOverlay::Image(ImageInfo::new("check"))
                    } else {
                        CellOverlay::None
                    },
                    // Show accessibility status: Checked if song location has been collected
                    accessibility: if is_check_completed {
                        Some(AccessibilityStatus::Checked)
                    } else {
                        None
                    },
                    label: None,
                }
            }
            SongCheck { check, .. } => {
                let is_checked = Check::<ootr_static::Rando>::Location(check.to_string())
                    .checked(state)
                    .unwrap_or(None)
                    .unwrap_or(false);
                CellRender {
                    img: ImageInfo::extra("blank"),
                    style: CellStyle::Normal,
                    overlay: if is_checked {
                        //TODO allow ootr_dynamic::Rando
                        CellOverlay::Image(ImageInfo::new("check"))
                    } else {
                        CellOverlay::None
                    },
                    // Show accessibility status: Checked if song location has been collected
                    accessibility: if is_checked {
                        Some(AccessibilityStatus::Checked)
                    } else {
                        None
                    },
                    label: None,
                }
            }
            Spells => CellRender {
                img: match (
                    state.ram.save.inv.dins_fire,
                    state.ram.save.inv.farores_wind,
                    state.ram.save.inv.nayrus_love,
                ) {
                    (false, false, false) | (true, true, false) => {
                        ImageInfo::new("composite_magic")
                    } //TODO use "spells" for dimmed instead if shift-click is available or auto-tracking?
                    (false, false, true) => ImageInfo::extra("nayrus_love"),
                    (false, true, false) => ImageInfo::new("faores_wind"),
                    (false, true, true) => ImageInfo::extra("farores_nayrus"),
                    (true, false, false) => ImageInfo::new("dins_fire"),
                    (true, false, true) => ImageInfo::extra("dins_nayrus"),
                    (true, true, true) => ImageInfo::extra("spells"),
                },
                style: if !state.ram.save.inv.dins_fire
                    && !state.ram.save.inv.farores_wind
                    && !state.ram.save.inv.nayrus_love
                {
                    CellStyle::Dimmed
                } else {
                    CellStyle::Normal
                },
                overlay: CellOverlay::None,
                accessibility: None,
                label: None,
            },
            Stone(stone) => CellRender {
                img: ImageInfo::new(match *stone {
                    Stone::KokiriEmerald => "kokiri_emerald",
                    Stone::GoronRuby => "goron_ruby",
                    Stone::ZoraSapphire => "zora_sapphire",
                }),
                style: if state.ram.save.quest_items.has(*stone) {
                    CellStyle::Normal
                } else {
                    CellStyle::Dimmed
                },
                overlay: CellOverlay::None,
                accessibility: None,
                label: None,
            },
            StoneLocation(stone) => {
                let location = state
                    .knowledge
                    .dungeon_reward_locations
                    .get(&DungeonReward::Stone(*stone));
                CellRender {
                    img: ImageInfo::new(match location {
                        None => "unknown_text",
                        Some(DungeonRewardLocation::Dungeon(MainDungeon::DekuTree)) => "deku_text",
                        Some(DungeonRewardLocation::Dungeon(MainDungeon::DodongosCavern)) => {
                            "dc_text"
                        }
                        Some(DungeonRewardLocation::Dungeon(MainDungeon::JabuJabu)) => "jabu_text",
                        Some(DungeonRewardLocation::Dungeon(MainDungeon::ForestTemple)) => {
                            "forest_text"
                        }
                        Some(DungeonRewardLocation::Dungeon(MainDungeon::FireTemple)) => {
                            "fire_text"
                        }
                        Some(DungeonRewardLocation::Dungeon(MainDungeon::WaterTemple)) => {
                            "water_text"
                        }
                        Some(DungeonRewardLocation::Dungeon(MainDungeon::ShadowTemple)) => {
                            "shadow_text"
                        }
                        Some(DungeonRewardLocation::Dungeon(MainDungeon::SpiritTemple)) => {
                            "spirit_text"
                        }
                        Some(DungeonRewardLocation::LinksPocket) => "free_text",
                    }),
                    style: if location.is_some() {
                        CellStyle::Normal
                    } else {
                        CellStyle::Dimmed
                    },
                    overlay: CellOverlay::None,
                    accessibility: None,
                    label: None,
                }
            }
            StoneWithLocation(stone) => {
                let location = state
                    .knowledge
                    .dungeon_reward_locations
                    .get(&DungeonReward::Stone(*stone));
                let has_stone = state.ram.save.quest_items.has(*stone);
                CellRender {
                    img: ImageInfo::new(match *stone {
                        Stone::KokiriEmerald => "kokiri_emerald",
                        Stone::GoronRuby => "goron_ruby",
                        Stone::ZoraSapphire => "zora_sapphire",
                    }),
                    style: if has_stone {
                        CellStyle::Normal
                    } else {
                        CellStyle::Dimmed
                    },
                    overlay: CellOverlay::Location {
                        loc: ImageInfo::new(match location {
                            None => "unknown_text",
                            Some(DungeonRewardLocation::Dungeon(MainDungeon::DekuTree)) => {
                                "deku_text"
                            }
                            Some(DungeonRewardLocation::Dungeon(MainDungeon::DodongosCavern)) => {
                                "dc_text"
                            }
                            Some(DungeonRewardLocation::Dungeon(MainDungeon::JabuJabu)) => {
                                "jabu_text"
                            }
                            Some(DungeonRewardLocation::Dungeon(MainDungeon::ForestTemple)) => {
                                "forest_text"
                            }
                            Some(DungeonRewardLocation::Dungeon(MainDungeon::FireTemple)) => {
                                "fire_text"
                            }
                            Some(DungeonRewardLocation::Dungeon(MainDungeon::WaterTemple)) => {
                                "water_text"
                            }
                            Some(DungeonRewardLocation::Dungeon(MainDungeon::ShadowTemple)) => {
                                "shadow_text"
                            }
                            Some(DungeonRewardLocation::Dungeon(MainDungeon::SpiritTemple)) => {
                                "spirit_text"
                            }
                            Some(DungeonRewardLocation::LinksPocket) => "free_text",
                        }),
                        style: if location.is_some() {
                            LocationStyle::Normal
                        } else {
                            LocationStyle::Dimmed
                        },
                    },
                    // Show accessibility status: Checked if stone obtained
                    accessibility: if has_stone {
                        Some(AccessibilityStatus::Checked)
                    } else {
                        None
                    },
                    label: None,
                }
            }
        }
    }

    /// Handle a click action from a frontend that don't distinguish between left and right click.
    pub fn click(&self, state: &mut ModelState) {
        match self {
            Composite {
                active,
                toggle_left,
                toggle_right,
                ..
            }
            | Overlay {
                active,
                toggle_main: toggle_left,
                toggle_overlay: toggle_right,
                ..
            } => {
                let (left, _) = active(state);
                if left {
                    toggle_right(state)
                }
                toggle_left(state);
            }
            OptionalOverlay {
                toggle_main: toggle,
                ..
            }
            | Simple { toggle, .. } => toggle(state),
            CompositeKeys { boss, small } => {
                let (toggle_boss, get_small, set_small, max_small_vanilla, max_small_mq) = if let (
                    BossKey { toggle, .. },
                    TrackerCellKind::SmallKeys {
                        get,
                        set,
                        max_vanilla,
                        max_mq,
                        ..
                    },
                ) =
                    (boss.kind(), small.kind())
                {
                    (toggle, get, set, max_vanilla, max_mq)
                } else {
                    unimplemented!("CompositeKeys that aren't SmallKeys + BossKey")
                };
                let num_small = get_small(&state.ram.save.small_keys);
                if num_small == max_small_vanilla.max(max_small_mq) {
                    //TODO check MQ knowledge? Does plentiful go to +1?
                    set_small(&mut state.ram.save.small_keys, 0);
                    toggle_boss(&mut state.ram.save.dungeon_items);
                } else {
                    set_small(&mut state.ram.save.small_keys, num_small + 1);
                }
            }
            Count {
                get,
                set,
                max,
                step,
                ..
            } => {
                let current = get(state);
                set(
                    state,
                    if current == *max {
                        0
                    } else {
                        current.saturating_add(*step).min(*max)
                    },
                );
            }
            FortressMq => {
                if state
                    .knowledge
                    .string_settings
                    .get("gerudo_fortress")
                    .is_some_and(|fort| fort.iter().eq(iter::once("normal")))
                {
                    state.knowledge.string_settings.remove("gerudo_fortress");
                } else {
                    state
                        .knowledge
                        .string_settings
                        .insert("gerudo_fortress".to_string(), collect![format!("normal")]);
                }
            }
            GoBk => {
                state.knowledge.progression_mode = match state.knowledge.progression_mode {
                    ProgressionMode::Normal => ProgressionMode::Go,
                    ProgressionMode::Go => ProgressionMode::Bk,
                    ProgressionMode::Bk => ProgressionMode::Done,
                    ProgressionMode::Done => ProgressionMode::Normal,
                }
            }
            MagicLens => {
                if state.ram.save.magic == MagicCapacity::None {
                    state.ram.save.magic = MagicCapacity::Small;
                } else {
                    state.ram.save.magic = MagicCapacity::None;
                    state.ram.save.inv.lens = !state.ram.save.inv.lens;
                }
            }
            Medallion(med) => state.ram.save.quest_items.toggle(QuestItems::from(med)),
            MedallionLocation(med) => state
                .knowledge
                .dungeon_reward_locations
                .increment(DungeonReward::Medallion(*med)),
            MedallionWithLocation(med) => state
                .knowledge
                .dungeon_reward_locations
                .increment(DungeonReward::Medallion(*med)),
            Mq(dungeon) => {
                if state.knowledge.mq.get(dungeon) == Some(&Mq::Mq) {
                    state.knowledge.mq.remove(dungeon);
                } else {
                    state.knowledge.mq.insert(*dungeon, Mq::Mq);
                }
            }
            Sequence { increment, .. } => increment(state),
            TrackerCellKind::SmallKeys {
                get,
                set,
                max_vanilla,
                max_mq,
                ..
            } => {
                let num = get(&state.ram.save.small_keys);
                if num == *max_vanilla.max(max_mq) {
                    //TODO check MQ knowledge? Does plentiful go to +1?
                    set(&mut state.ram.save.small_keys, 0);
                } else {
                    set(&mut state.ram.save.small_keys, num + 1);
                }
            }
            TrackerCellKind::MmSmallKeys { get, set, max, .. } => {
                let mm_save = state.ram.mm_save.get_or_insert_with(Default::default);
                let num = get(&mm_save.small_keys);
                if num == *max {
                    set(&mut mm_save.small_keys, 0);
                } else {
                    set(&mut mm_save.small_keys, num + 1);
                }
            }
            Song {
                song: quest_item, ..
            } => state.ram.save.quest_items.toggle(*quest_item),
            Spells => {
                if state.ram.save.inv.dins_fire {
                    state.ram.save.inv.farores_wind = !state.ram.save.inv.farores_wind
                }
                state.ram.save.inv.dins_fire = !state.ram.save.inv.dins_fire;
            }
            Stone(stone) => state.ram.save.quest_items.toggle(QuestItems::from(stone)),
            StoneLocation(stone) => state
                .knowledge
                .dungeon_reward_locations
                .increment(DungeonReward::Stone(*stone)),
            StoneWithLocation(stone) => state
                .knowledge
                .dungeon_reward_locations
                .increment(DungeonReward::Stone(*stone)),
            FreeReward => {}
            OotMap { toggle, .. } => toggle(&mut state.ram.save.dungeon_items),
            OotCompass { toggle, .. } => toggle(&mut state.ram.save.dungeon_items),
            MmBossKey { toggle, .. } => {
                let mm_save = state.ram.mm_save.get_or_insert_with(Default::default);
                toggle(&mut mm_save.dungeon_items);
            }
            MmMap { toggle, .. } => {
                let mm_save = state.ram.mm_save.get_or_insert_with(Default::default);
                toggle(&mut mm_save.dungeon_items);
            }
            MmCompass { toggle, .. } => {
                let mm_save = state.ram.mm_save.get_or_insert_with(Default::default);
                toggle(&mut mm_save.dungeon_items);
            }
            BossKey { toggle, .. } => toggle(&mut state.ram.save.dungeon_items),
            BigPoeTriforce | SongCheck { .. } => unimplemented!(),
        }
    }

    #[cfg(feature = "iced")]
    /// Returns `true` if the menu should be opened.
    #[must_use]
    pub fn left_click(
        &self,
        can_change_state: bool,
        keyboard_modifiers: KeyboardModifiers,
        state: &mut ModelState,
    ) -> bool {
        //TODO shift-click support
        #[cfg(target_os = "macos")]
        if keyboard_modifiers.control() {
            return self.right_click(can_change_state, keyboard_modifiers, state);
        }
        if can_change_state {
            match self {
                Composite { toggle_left, .. }
                | Overlay {
                    toggle_main: toggle_left,
                    ..
                } => toggle_left(state),
                CompositeKeys { boss, .. } => {
                    if let BossKey { toggle, .. } = boss.kind() {
                        toggle(&mut state.ram.save.dungeon_items);
                    } else {
                        unimplemented!("CompositeKeys that aren't SmallKeys + BossKey")
                    }
                }
                Count {
                    get,
                    set,
                    max,
                    step,
                    ..
                } => {
                    let current = get(state);
                    set(
                        state,
                        if current == *max {
                            0
                        } else {
                            current
                                .saturating_add(
                                    step * if keyboard_modifiers.shift() && *max >= 10 {
                                        10
                                    } else {
                                        1
                                    },
                                )
                                .min(*max)
                        },
                    );
                }
                GoBk => {
                    state.knowledge.progression_mode = match state.knowledge.progression_mode {
                        ProgressionMode::Normal => ProgressionMode::Go,
                        ProgressionMode::Go => ProgressionMode::Normal,
                        ProgressionMode::Bk => ProgressionMode::Done,
                        ProgressionMode::Done => ProgressionMode::Bk,
                    }
                }
                MagicLens => {
                    state.ram.save.magic = match (keyboard_modifiers.shift(), state.ram.save.magic)
                    {
                        (true, MagicCapacity::Large) => MagicCapacity::Small,
                        (true, _) => MagicCapacity::Large,
                        (false, MagicCapacity::None) => MagicCapacity::Small,
                        (false, _) => MagicCapacity::None,
                    }
                }
                Spells => {
                    if keyboard_modifiers.shift() {
                        state.ram.save.inv.nayrus_love = !state.ram.save.inv.nayrus_love;
                    } else {
                        state.ram.save.inv.dins_fire = !state.ram.save.inv.dins_fire;
                    }
                }
                _ => self.click(state),
            }
        }
        false
    }

    #[cfg(feature = "iced")]
    /// Returns `true` if the menu should be opened.
    #[must_use]
    pub fn right_click(
        &self,
        can_change_state: bool,
        keyboard_modifiers: KeyboardModifiers,
        state: &mut ModelState,
    ) -> bool {
        //TODO shift-click support
        if let Medallion(_) = self {
            return true;
        }
        if can_change_state {
            match self {
                Composite { toggle_right, .. }
                | OptionalOverlay {
                    toggle_overlay: toggle_right,
                    ..
                }
                | Overlay {
                    toggle_overlay: toggle_right,
                    ..
                } => toggle_right(state),
                CompositeKeys { small, .. } => {
                    if let TrackerCellKind::SmallKeys {
                        get,
                        set,
                        max_vanilla,
                        max_mq,
                        ..
                    } = small.kind()
                    {
                        let num = get(&state.ram.save.small_keys);
                        if num == max_vanilla.max(max_mq) {
                            //TODO check MQ knowledge? Does plentiful go to +1?
                            set(&mut state.ram.save.small_keys, 0);
                        } else {
                            set(&mut state.ram.save.small_keys, num + 1);
                        }
                    } else {
                        unimplemented!("CompositeKeys that aren't SmallKeys + BossKey")
                    }
                }
                Count {
                    get,
                    set,
                    max,
                    step,
                    ..
                } => {
                    let current = get(state);
                    set(
                        state,
                        if current == 0 {
                            *max
                        } else {
                            current.saturating_sub(
                                step * if keyboard_modifiers.shift() && *max >= 10 {
                                    10
                                } else {
                                    1
                                },
                            )
                        },
                    );
                }
                GoBk => {
                    state.knowledge.progression_mode = match state.knowledge.progression_mode {
                        ProgressionMode::Normal => ProgressionMode::Bk,
                        ProgressionMode::Bk => ProgressionMode::Normal,
                        ProgressionMode::Go => ProgressionMode::Done,
                        ProgressionMode::Done => ProgressionMode::Go,
                    }
                }
                MagicLens => state.ram.save.inv.lens = !state.ram.save.inv.lens,
                Medallion(_) => unreachable!("already handled above"),
                MedallionLocation(med) => state
                    .knowledge
                    .dungeon_reward_locations
                    .decrement(DungeonReward::Medallion(*med)),
                MedallionWithLocation(med) => {
                    state.ram.save.quest_items.toggle(QuestItems::from(med))
                }
                Sequence { decrement, .. } => decrement(state),
                TrackerCellKind::SmallKeys {
                    get,
                    set,
                    max_vanilla,
                    max_mq,
                    ..
                } => {
                    let num = get(&state.ram.save.small_keys);
                    if num == 0 {
                        set(&mut state.ram.save.small_keys, *max_vanilla.max(max_mq));
                    //TODO check MQ knowledge? Does plentiful go to +1?
                    } else {
                        set(&mut state.ram.save.small_keys, num - 1);
                    }
                }
                TrackerCellKind::MmSmallKeys { get, set, max, .. } => {
                    let mm_save = state.ram.mm_save.get_or_insert_with(Default::default);
                    let num = get(&mm_save.small_keys);
                    if num == 0 {
                        set(&mut mm_save.small_keys, *max);
                    } else {
                        set(&mut mm_save.small_keys, num - 1);
                    }
                }
                Song { toggle_overlay, .. } => toggle_overlay(&mut state.ram.save.event_chk_inf),
                Spells => state.ram.save.inv.farores_wind = !state.ram.save.inv.farores_wind,
                StoneLocation(stone) => state
                    .knowledge
                    .dungeon_reward_locations
                    .decrement(DungeonReward::Stone(*stone)),
                StoneWithLocation(stone) => {
                    state.ram.save.quest_items.toggle(QuestItems::from(stone))
                }
                FreeReward | FortressMq | Mq(_) | Simple { .. } | Stone(_) => {}
                OotMap { .. }
                | OotCompass { .. }
                | MmBossKey { .. }
                | MmMap { .. }
                | MmCompass { .. } => {}
                BigPoeTriforce | BossKey { .. } | SongCheck { .. } => unimplemented!(),
            }
        }
        false
    }
}

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

cells! {
    GoMode: Simple {
        img: ImageInfo::extra("go_mode"),
        active: Box::new(|state| match state.knowledge.progression_mode {
            ProgressionMode::Go | ProgressionMode::Done => true,
            ProgressionMode::Bk | ProgressionMode::Normal => false,
        }),
        toggle: Box::new(|state| {
            let new_mode = match state.knowledge.progression_mode {
                ProgressionMode::Done => ProgressionMode::Done, // only the racetime integration may toggle .done for now
                ProgressionMode::Go => ProgressionMode::Normal,
                ProgressionMode::Bk | ProgressionMode::Normal => ProgressionMode::Go,
            };
            state.knowledge.progression_mode = new_mode;
        }),
    },
    GoBk: GoBk,
    LightMedallion: Medallion(Medallion::Light),
    ForestMedallion: Medallion(Medallion::Forest),
    FireMedallion: Medallion(Medallion::Fire),
    WaterMedallion: Medallion(Medallion::Water),
    ShadowMedallion: Medallion(Medallion::Shadow),
    SpiritMedallion: Medallion(Medallion::Spirit),
    LightMedallionLocation: MedallionLocation(Medallion::Light),
    ForestMedallionLocation: MedallionLocation(Medallion::Forest),
    FireMedallionLocation: MedallionLocation(Medallion::Fire),
    WaterMedallionLocation: MedallionLocation(Medallion::Water),
    ShadowMedallionLocation: MedallionLocation(Medallion::Shadow),
    SpiritMedallionLocation: MedallionLocation(Medallion::Spirit),
    LightMedallionWithLocation: MedallionWithLocation(Medallion::Light),
    ForestMedallionWithLocation: MedallionWithLocation(Medallion::Forest),
    FireMedallionWithLocation: MedallionWithLocation(Medallion::Fire),
    WaterMedallionWithLocation: MedallionWithLocation(Medallion::Water),
    ShadowMedallionWithLocation: MedallionWithLocation(Medallion::Shadow),
    SpiritMedallionWithLocation: MedallionWithLocation(Medallion::Spirit),
    AdultTrade: Sequence {
        idx: Box::new(|state| match state.ram.save.inv.adult_trade_item {
            AdultTradeItem::None => 0,
            AdultTradeItem::PocketEgg => 1,
            AdultTradeItem::PocketCucco => 2,
            AdultTradeItem::Cojiro => 3,
            AdultTradeItem::OddMushroom => 4,
            AdultTradeItem::OddPotion => 5,
            AdultTradeItem::PoachersSaw => 6,
            AdultTradeItem::BrokenSword => 7,
            AdultTradeItem::Prescription => 8,
            AdultTradeItem::EyeballFrog => 9,
            AdultTradeItem::Eyedrops => 10,
            AdultTradeItem::ClaimCheck => 11,
        }),
        img: Box::new(|state| match state.ram.save.inv.adult_trade_item {
            AdultTradeItem::None => (false, ImageInfo::new("blue_egg")),
            AdultTradeItem::PocketEgg | AdultTradeItem::PocketCucco => (true, ImageInfo::new("blue_egg")),
            AdultTradeItem::Cojiro => (true, ImageInfo::new("cojiro")),
            AdultTradeItem::OddMushroom => (true, ImageInfo::new("odd_mushroom")),
            AdultTradeItem::OddPotion => (true, ImageInfo::new("odd_poultice")),
            AdultTradeItem::PoachersSaw => (true, ImageInfo::new("poachers_saw")),
            AdultTradeItem::BrokenSword => (true, ImageInfo::new("broken_sword")),
            AdultTradeItem::Prescription => (true, ImageInfo::new("prescription")),
            AdultTradeItem::EyeballFrog => (true, ImageInfo::new("eyeball_frog")),
            AdultTradeItem::Eyedrops => (true, ImageInfo::new("eye_drops")),
            AdultTradeItem::ClaimCheck => (true, ImageInfo::new("claim_check")),
        }),
        increment: Box::new(|state| state.ram.save.inv.adult_trade_item = match state.ram.save.inv.adult_trade_item {
            AdultTradeItem::None => AdultTradeItem::PocketEgg,
            AdultTradeItem::PocketEgg => AdultTradeItem::PocketCucco,
            AdultTradeItem::PocketCucco => AdultTradeItem::Cojiro,
            AdultTradeItem::Cojiro => AdultTradeItem::OddMushroom,
            AdultTradeItem::OddMushroom => AdultTradeItem::OddPotion,
            AdultTradeItem::OddPotion => AdultTradeItem::PoachersSaw,
            AdultTradeItem::PoachersSaw => AdultTradeItem::BrokenSword,
            AdultTradeItem::BrokenSword => AdultTradeItem::Prescription,
            AdultTradeItem::Prescription => AdultTradeItem::EyeballFrog,
            AdultTradeItem::EyeballFrog => AdultTradeItem::Eyedrops,
            AdultTradeItem::Eyedrops => AdultTradeItem::ClaimCheck,
            AdultTradeItem::ClaimCheck => AdultTradeItem::None,
        }),
        decrement: Box::new(|state| state.ram.save.inv.adult_trade_item = match state.ram.save.inv.adult_trade_item {
            AdultTradeItem::None => AdultTradeItem::ClaimCheck,
            AdultTradeItem::PocketEgg => AdultTradeItem::None,
            AdultTradeItem::PocketCucco => AdultTradeItem::PocketEgg,
            AdultTradeItem::Cojiro => AdultTradeItem::PocketEgg,
            AdultTradeItem::OddMushroom => AdultTradeItem::Cojiro,
            AdultTradeItem::OddPotion => AdultTradeItem::OddMushroom,
            AdultTradeItem::PoachersSaw => AdultTradeItem::OddPotion,
            AdultTradeItem::BrokenSword => AdultTradeItem::PoachersSaw,
            AdultTradeItem::Prescription => AdultTradeItem::BrokenSword,
            AdultTradeItem::EyeballFrog => AdultTradeItem::Prescription,
            AdultTradeItem::Eyedrops => AdultTradeItem::EyeballFrog,
            AdultTradeItem::ClaimCheck => AdultTradeItem::Eyedrops,
        }),
    },
    AdultTradeNoChicken: Sequence {
        idx: Box::new(|state| match state.ram.save.inv.adult_trade_item {
            AdultTradeItem::None => 0,
            AdultTradeItem::PocketEgg | AdultTradeItem::PocketCucco => 1,
            AdultTradeItem::Cojiro => 2,
            AdultTradeItem::OddMushroom => 3,
            AdultTradeItem::OddPotion => 4,
            AdultTradeItem::PoachersSaw => 5,
            AdultTradeItem::BrokenSword => 6,
            AdultTradeItem::Prescription => 7,
            AdultTradeItem::EyeballFrog => 8,
            AdultTradeItem::Eyedrops => 9,
            AdultTradeItem::ClaimCheck => 10,
        }),
        img: Box::new(|state| match state.ram.save.inv.adult_trade_item {
            AdultTradeItem::None => (false, ImageInfo::new("blue_egg")),
            AdultTradeItem::PocketEgg | AdultTradeItem::PocketCucco => (true, ImageInfo::new("blue_egg")),
            AdultTradeItem::Cojiro => (true, ImageInfo::new("cojiro")),
            AdultTradeItem::OddMushroom => (true, ImageInfo::new("odd_mushroom")),
            AdultTradeItem::OddPotion => (true, ImageInfo::new("odd_poultice")),
            AdultTradeItem::PoachersSaw => (true, ImageInfo::new("poachers_saw")),
            AdultTradeItem::BrokenSword => (true, ImageInfo::new("broken_sword")),
            AdultTradeItem::Prescription => (true, ImageInfo::new("prescription")),
            AdultTradeItem::EyeballFrog => (true, ImageInfo::new("eyeball_frog")),
            AdultTradeItem::Eyedrops => (true, ImageInfo::new("eye_drops")),
            AdultTradeItem::ClaimCheck => (true, ImageInfo::new("claim_check")),
        }),
        increment: Box::new(|state| state.ram.save.inv.adult_trade_item = match state.ram.save.inv.adult_trade_item {
            AdultTradeItem::None => AdultTradeItem::PocketEgg,
            AdultTradeItem::PocketEgg | AdultTradeItem::PocketCucco => AdultTradeItem::Cojiro,
            AdultTradeItem::Cojiro => AdultTradeItem::OddMushroom,
            AdultTradeItem::OddMushroom => AdultTradeItem::OddPotion,
            AdultTradeItem::OddPotion => AdultTradeItem::PoachersSaw,
            AdultTradeItem::PoachersSaw => AdultTradeItem::BrokenSword,
            AdultTradeItem::BrokenSword => AdultTradeItem::Prescription,
            AdultTradeItem::Prescription => AdultTradeItem::EyeballFrog,
            AdultTradeItem::EyeballFrog => AdultTradeItem::Eyedrops,
            AdultTradeItem::Eyedrops => AdultTradeItem::ClaimCheck,
            AdultTradeItem::ClaimCheck => AdultTradeItem::None,
        }),
        decrement: Box::new(|state| state.ram.save.inv.adult_trade_item = match state.ram.save.inv.adult_trade_item {
            AdultTradeItem::None => AdultTradeItem::ClaimCheck,
            AdultTradeItem::PocketEgg | AdultTradeItem::PocketCucco => AdultTradeItem::None,
            AdultTradeItem::Cojiro => AdultTradeItem::PocketEgg,
            AdultTradeItem::OddMushroom => AdultTradeItem::Cojiro,
            AdultTradeItem::OddPotion => AdultTradeItem::OddMushroom,
            AdultTradeItem::PoachersSaw => AdultTradeItem::OddPotion,
            AdultTradeItem::BrokenSword => AdultTradeItem::PoachersSaw,
            AdultTradeItem::Prescription => AdultTradeItem::BrokenSword,
            AdultTradeItem::EyeballFrog => AdultTradeItem::Prescription,
            AdultTradeItem::Eyedrops => AdultTradeItem::EyeballFrog,
            AdultTradeItem::ClaimCheck => AdultTradeItem::Eyedrops,
        }),
    },
    Skulltula: Count {
        dimmed_img: ImageInfo::new("golden_skulltula"),
        img: ImageInfo::new("skulls"),
        get: Box::new(|state| state.ram.save.skull_tokens),
        set: Box::new(|state, value| state.ram.save.skull_tokens = value),
        max: 100,
        step: 1,
    },
    SkulltulaTens: Count {
        dimmed_img: ImageInfo::new("golden_skulltula"),
        img: ImageInfo::new("skulls"),
        get: Box::new(|state| state.ram.save.skull_tokens),
        set: Box::new(|state, value| state.ram.save.skull_tokens = value),
        max: 50,
        step: 10,
    },
    KokiriEmerald: Stone(Stone::KokiriEmerald),
    GoronRuby: Stone(Stone::GoronRuby),
    ZoraSapphire: Stone(Stone::ZoraSapphire),
    KokiriEmeraldLocation: StoneLocation(Stone::KokiriEmerald),
    GoronRubyLocation: StoneLocation(Stone::GoronRuby),
    ZoraSapphireLocation: StoneLocation(Stone::ZoraSapphire),
    KokiriEmeraldWithLocation: StoneWithLocation(Stone::KokiriEmerald),
    GoronRubyWithLocation: StoneWithLocation(Stone::GoronRuby),
    ZoraSapphireWithLocation: StoneWithLocation(Stone::ZoraSapphire),
    Bottle: OptionalOverlay {
        main_img: ImageInfo::new("bottle"),
        overlay_img: ImageInfo::new("letter"),
        active: Box::new(|state| (state.ram.save.inv.emptiable_bottles() > 0, state.ram.save.inv.has_rutos_letter())), //TODO also show Ruto's letter as active if it has been delivered or Open Fountain is known (https://github.com/fenhl/oottracker/issues/21)
        toggle_main: Box::new(|state| {
            let new_val = if state.ram.save.inv.emptiable_bottles() > 0 { 0 } else { 1 };
            state.ram.save.inv.set_emptiable_bottles(new_val);
        }),
        toggle_overlay: Box::new(|state| state.ram.save.inv.toggle_rutos_letter()),
    },
    NumBottles: Count {
        dimmed_img: ImageInfo::new("bottle"),
        img: ImageInfo::new("UNIMPLEMENTED"), //TODO make images for 1–4 bottles
        get: Box::new(|state| state.ram.save.inv.emptiable_bottles()),
        set: Box::new(|state, value| state.ram.save.inv.set_emptiable_bottles(value)),
        max: 4,
        step: 1,
    },
    RutosLetter: Simple {
        img: ImageInfo::new("UNIMPLEMENTED"),
        active: Box::new(|state| state.ram.save.inv.has_rutos_letter()), //TODO also show Ruto's letter as active if it has been delivered or Open Fountain is known (https://github.com/fenhl/oottracker/issues/21)
        toggle: Box::new(|state| state.ram.save.inv.toggle_rutos_letter()),
    },
    Scale: Sequence {
        idx: Box::new(|state| match state.ram.save.upgrades.scale() {
            Upgrades::SILVER_SCALE => 1,
            Upgrades::GOLD_SCALE => 2,
            _ => 0,
        }),
        img: Box::new(|state| match state.ram.save.upgrades.scale() {
            Upgrades::SILVER_SCALE => (true, ImageInfo::new("silver_scale")),
            Upgrades::GOLD_SCALE => (true, ImageInfo::new("gold_scale")),
            _ => (false, ImageInfo::new("silver_scale")),
        }),
        increment: Box::new(|state| {
            let new_val = match state.ram.save.upgrades.scale() {
                Upgrades::SILVER_SCALE => Upgrades::GOLD_SCALE,
                Upgrades::GOLD_SCALE => Upgrades::NONE,
                _ => Upgrades::SILVER_SCALE,
            };
            state.ram.save.upgrades.set_scale(new_val);
        }),
        decrement: Box::new(|state| {
            let new_val = match state.ram.save.upgrades.scale() {
                Upgrades::SILVER_SCALE => Upgrades::NONE,
                Upgrades::GOLD_SCALE => Upgrades::SILVER_SCALE,
                _ => Upgrades::GOLD_SCALE,
            };
            state.ram.save.upgrades.set_scale(new_val);
        }),
    },
    Slingshot: Simple {
        img: ImageInfo::new("slingshot"),
        active: Box::new(|state| state.ram.save.inv.slingshot),
        toggle: Box::new(|state| {
            state.ram.save.inv.slingshot = !state.ram.save.inv.slingshot;
            let new_bullet_bag = if state.ram.save.inv.slingshot { Upgrades::BULLET_BAG_30 } else { Upgrades::NONE };
            state.ram.save.upgrades.set_bullet_bag(new_bullet_bag);
        }),
    },
    BulletBag: Sequence {
        idx: Box::new(|state| match state.ram.save.upgrades.bullet_bag() {
            Upgrades::BULLET_BAG_30 => 1,
            Upgrades::BULLET_BAG_40 => 2,
            Upgrades::BULLET_BAG_50 => 3,
            _ => 0,
        }),
        img: Box::new(|state| (state.ram.save.inv.slingshot, ImageInfo::new("slingshot"))),
        increment: Box::new(|state| {
            let new_bullet_bag = match state.ram.save.upgrades.bullet_bag() {
                Upgrades::BULLET_BAG_30 => Upgrades::BULLET_BAG_40,
                Upgrades::BULLET_BAG_40 => Upgrades::BULLET_BAG_50,
                Upgrades::BULLET_BAG_50 => Upgrades::NONE,
                _ => Upgrades::BULLET_BAG_30,
            };
            state.ram.save.upgrades.set_bullet_bag(new_bullet_bag);
            state.ram.save.inv.slingshot = state.ram.save.upgrades.bullet_bag() != Upgrades::NONE;
        }),
        decrement: Box::new(|state| {
            let new_bullet_bag = match state.ram.save.upgrades.bullet_bag() {
                Upgrades::BULLET_BAG_30 => Upgrades::NONE,
                Upgrades::BULLET_BAG_40 => Upgrades::BULLET_BAG_30,
                Upgrades::BULLET_BAG_50 => Upgrades::BULLET_BAG_40,
                _ => Upgrades::BULLET_BAG_50,
            };
            state.ram.save.upgrades.set_bullet_bag(new_bullet_bag);
            state.ram.save.inv.slingshot = state.ram.save.upgrades.bullet_bag() != Upgrades::NONE;
        }),
    },
    Bombs: Overlay {
        main_img: ImageInfo::new("bomb_bag"),
        overlay_img: ImageInfo::new("bombchu"),
        active: Box::new(|state| (state.ram.save.upgrades.bomb_bag() != Upgrades::NONE, state.ram.save.inv.bombchus)),
        toggle_main: Box::new(|state| if state.ram.save.upgrades.bomb_bag() == Upgrades::NONE {
            state.ram.save.upgrades.set_bomb_bag(Upgrades::BOMB_BAG_20);
        } else {
            state.ram.save.upgrades.set_bomb_bag(Upgrades::NONE);
        }),
        toggle_overlay: Box::new(|state| state.ram.save.inv.bombchus = !state.ram.save.inv.bombchus),
    },
    BombBag: Sequence {
        idx: Box::new(|state| match state.ram.save.upgrades.bomb_bag() {
            Upgrades::BOMB_BAG_20 => 1,
            Upgrades::BOMB_BAG_30 => 2,
            Upgrades::BOMB_BAG_40 => 3,
            _ => 0,
        }),
        img: Box::new(|state| (state.ram.save.upgrades.bomb_bag() != Upgrades::NONE, ImageInfo::new("bomb_bag"))),
        increment: Box::new(|state| {
            let new_val = match state.ram.save.upgrades.bomb_bag() {
                Upgrades::BOMB_BAG_20 => Upgrades::BOMB_BAG_30,
                Upgrades::BOMB_BAG_30 => Upgrades::BOMB_BAG_40,
                Upgrades::BOMB_BAG_40 => Upgrades::NONE,
                _ => Upgrades::BOMB_BAG_20,
            };
            state.ram.save.upgrades.set_bomb_bag(new_val);
        }),
        decrement: Box::new(|state| {
            let new_val = match state.ram.save.upgrades.bomb_bag() {
                Upgrades::BOMB_BAG_20 => Upgrades::NONE,
                Upgrades::BOMB_BAG_30 => Upgrades::BOMB_BAG_20,
                Upgrades::BOMB_BAG_40 => Upgrades::BOMB_BAG_30,
                _ => Upgrades::BOMB_BAG_40,
            };
            state.ram.save.upgrades.set_bomb_bag(new_val);
        }),
    },
    Bombchus: Simple {
        img: ImageInfo::new("UNIMPLEMENTED"),
        active: Box::new(|state| state.ram.save.inv.bombchus),
        toggle: Box::new(|state| state.ram.save.inv.bombchus = !state.ram.save.inv.bombchus),
    },
    Boomerang: Simple {
        img: ImageInfo::new("boomerang"),
        active: Box::new(|state| state.ram.save.inv.boomerang),
        toggle: Box::new(|state| state.ram.save.inv.boomerang = !state.ram.save.inv.boomerang),
    },
    Strength: Sequence {
        idx: Box::new(|state| match state.ram.save.upgrades.strength() {
            Upgrades::GORON_BRACELET => 1,
            Upgrades::SILVER_GAUNTLETS => 2,
            Upgrades::GOLD_GAUNTLETS => 3,
            _ => 0,
        }),
        img: Box::new(|state| match state.ram.save.upgrades.strength() {
            Upgrades::GORON_BRACELET => (true, ImageInfo::new("goron_bracelet")),
            Upgrades::SILVER_GAUNTLETS => (true, ImageInfo::new("silver_gauntlets")),
            Upgrades::GOLD_GAUNTLETS => (true, ImageInfo::new("gold_gauntlets")),
            _ => (false, ImageInfo::new("goron_bracelet")),
        }),
        increment: Box::new(|state| {
            let new_val = match state.ram.save.upgrades.strength() {
                Upgrades::GORON_BRACELET => Upgrades::SILVER_GAUNTLETS,
                Upgrades::SILVER_GAUNTLETS => Upgrades::GOLD_GAUNTLETS,
                Upgrades::GOLD_GAUNTLETS => Upgrades::NONE,
                _ => Upgrades::GORON_BRACELET,
            };
            state.ram.save.upgrades.set_strength(new_val);
        }),
        decrement: Box::new(|state| {
            let new_val = match state.ram.save.upgrades.strength() {
                Upgrades::GORON_BRACELET => Upgrades::NONE,
                Upgrades::SILVER_GAUNTLETS => Upgrades::GORON_BRACELET,
                Upgrades::GOLD_GAUNTLETS => Upgrades::SILVER_GAUNTLETS,
                _ => Upgrades::GOLD_GAUNTLETS,
            };
            state.ram.save.upgrades.set_strength(new_val);
        }),
    },
    Magic: Simple {
        img: ImageInfo::new("magic"),
        active: Box::new(|state| state.ram.save.magic != MagicCapacity::None),
        toggle: Box::new(|state| if state.ram.save.magic == MagicCapacity::None {
            state.ram.save.magic = MagicCapacity::Small;
        } else {
            state.ram.save.magic = MagicCapacity::None;
        }),
    },
    MagicLens: MagicLens,
    MagicCapacity: Sequence {
        idx: Box::new(|state| match state.ram.save.magic {
            MagicCapacity::None => 0,
            MagicCapacity::Small => 1,
            MagicCapacity::Large => 2,
        }),
        img: Box::new(|state| (state.ram.save.magic != MagicCapacity::None, ImageInfo::new("magic"))),
        increment: Box::new(|state| state.ram.save.magic = match state.ram.save.magic {
            MagicCapacity::None => MagicCapacity::Small,
            MagicCapacity::Small => MagicCapacity::Large,
            MagicCapacity::Large => MagicCapacity::None,
        }),
        decrement: Box::new(|state| state.ram.save.magic = match state.ram.save.magic {
            MagicCapacity::None => MagicCapacity::Large,
            MagicCapacity::Small => MagicCapacity::None,
            MagicCapacity::Large => MagicCapacity::Small,
        }),
    },
    Lens: Simple {
        img: ImageInfo::new("lens"),
        active: Box::new(|state| state.ram.save.inv.lens),
        toggle: Box::new(|state| state.ram.save.inv.lens = !state.ram.save.inv.lens),
    },
    DinsFarores: Composite {
        left_img: ImageInfo::new("dins_fire"),
        right_img: ImageInfo::new("faores_wind"),
        both_img: ImageInfo::new("composite_magic"),
        active: Box::new(|state| (state.ram.save.inv.dins_fire, state.ram.save.inv.farores_wind)),
        toggle_left: Box::new(|state| state.ram.save.inv.dins_fire = !state.ram.save.inv.dins_fire),
        toggle_right: Box::new(|state| state.ram.save.inv.farores_wind = !state.ram.save.inv.farores_wind),
    },
    Spells: Spells,
    DinsFire: Simple {
        img: ImageInfo::new("dins_fire"),
        active: Box::new(|state| state.ram.save.inv.dins_fire),
        toggle: Box::new(|state| state.ram.save.inv.dins_fire = !state.ram.save.inv.dins_fire),
    },
    FaroresWind: Simple {
        img: ImageInfo::new("faores_wind"),
        active: Box::new(|state| state.ram.save.inv.farores_wind),
        toggle: Box::new(|state| state.ram.save.inv.farores_wind = !state.ram.save.inv.farores_wind),
    },
    NayrusLove: Simple {
        img: ImageInfo::extra("nayrus_love"),
        active: Box::new(|state| state.ram.save.inv.nayrus_love),
        toggle: Box::new(|state| state.ram.save.inv.nayrus_love = !state.ram.save.inv.nayrus_love),
    },
    Hookshot: Sequence {
        idx: Box::new(|state| match state.ram.save.inv.hookshot {
            Hookshot::None => 0,
            Hookshot::Hookshot => 1,
            Hookshot::Longshot => 2,
        }),
        img: Box::new(|state| match state.ram.save.inv.hookshot {
            Hookshot::None => (false, ImageInfo::new("hookshot")),
            Hookshot::Hookshot => (true, ImageInfo::new("hookshot_accessible")),
            Hookshot::Longshot => (true, ImageInfo::new("longshot_accessible")),
        }),
        increment: Box::new(|state| state.ram.save.inv.hookshot = match state.ram.save.inv.hookshot {
            Hookshot::None => Hookshot::Hookshot,
            Hookshot::Hookshot => Hookshot::Longshot,
            Hookshot::Longshot => Hookshot::None,
        }),
        decrement: Box::new(|state| state.ram.save.inv.hookshot = match state.ram.save.inv.hookshot {
            Hookshot::None => Hookshot::Longshot,
            Hookshot::Hookshot => Hookshot::None,
            Hookshot::Longshot => Hookshot::Hookshot,
        }),
    },
    Bow: OptionalOverlay {
        main_img: ImageInfo::new("bow"),
        overlay_img: ImageInfo::new("ice_arrows"),
        active: Box::new(|state| (state.ram.save.inv.bow, state.ram.save.inv.ice_arrows)),
        toggle_main: Box::new(|state| {
            state.ram.save.inv.bow = !state.ram.save.inv.bow;
            let new_quiver = if state.ram.save.inv.bow { Upgrades::QUIVER_30 } else { Upgrades::NONE };
            state.ram.save.upgrades.set_quiver(new_quiver);
        }),
        toggle_overlay: Box::new(|state| state.ram.save.inv.ice_arrows = !state.ram.save.inv.ice_arrows),
    },
    IceArrows: Simple {
        img: ImageInfo::new("ice_trap"),
        active: Box::new(|state| state.ram.save.inv.ice_arrows),
        toggle: Box::new(|state| state.ram.save.inv.ice_arrows = !state.ram.save.inv.ice_arrows),
    },
    Quiver: Sequence {
        idx: Box::new(|state| match state.ram.save.upgrades.quiver() {
            Upgrades::QUIVER_30 => 1,
            Upgrades::QUIVER_40 => 2,
            Upgrades::QUIVER_50 => 3,
            _ => 0,
        }),
        img: Box::new(|state| (state.ram.save.inv.bow, ImageInfo::new("bow"))),
        increment: Box::new(|state| {
            let new_quiver = match state.ram.save.upgrades.quiver() {
                Upgrades::QUIVER_30 => Upgrades::QUIVER_40,
                Upgrades::QUIVER_40 => Upgrades::QUIVER_50,
                Upgrades::QUIVER_50 => Upgrades::NONE,
                _ => Upgrades::QUIVER_30,
            };
            state.ram.save.upgrades.set_quiver(new_quiver);
            state.ram.save.inv.bow = state.ram.save.upgrades.quiver() != Upgrades::NONE;
        }),
        decrement: Box::new(|state| {
            let new_quiver = match state.ram.save.upgrades.quiver() {
                Upgrades::QUIVER_30 => Upgrades::NONE,
                Upgrades::QUIVER_40 => Upgrades::QUIVER_30,
                Upgrades::QUIVER_50 => Upgrades::QUIVER_40,
                _ => Upgrades::QUIVER_50,
            };
            state.ram.save.upgrades.set_quiver(new_quiver);
            state.ram.save.inv.bow = state.ram.save.upgrades.quiver() != Upgrades::NONE;
        }),
    },
    Arrows: Composite {
        left_img: ImageInfo::new("fire_arrows"),
        right_img: ImageInfo::new("light_arrows"),
        both_img: ImageInfo::new("composite_arrows"),
        active: Box::new(|state| (state.ram.save.inv.fire_arrows, state.ram.save.inv.light_arrows)),
        toggle_left: Box::new(|state| state.ram.save.inv.fire_arrows = !state.ram.save.inv.fire_arrows),
        toggle_right: Box::new(|state| state.ram.save.inv.light_arrows = !state.ram.save.inv.light_arrows),
    },
    FireArrows: Simple {
        img: ImageInfo::new("fire_arrows"),
        active: Box::new(|state| state.ram.save.inv.fire_arrows),
        toggle: Box::new(|state| state.ram.save.inv.fire_arrows = !state.ram.save.inv.fire_arrows),
    },
    LightArrows: Simple {
        img: ImageInfo::new("light_arrows"),
        active: Box::new(|state| state.ram.save.inv.light_arrows),
        toggle: Box::new(|state| state.ram.save.inv.light_arrows = !state.ram.save.inv.light_arrows),
    },
    Hammer: Simple {
        img: ImageInfo::new("hammer"),
        active: Box::new(|state| state.ram.save.inv.hammer),
        toggle: Box::new(|state| state.ram.save.inv.hammer = !state.ram.save.inv.hammer),
    },
    Boots: Composite {
        left_img: ImageInfo::new("iron_boots"),
        right_img: ImageInfo::new("hover_boots"),
        both_img: ImageInfo::new("composite_boots"),
        active: Box::new(|state| (state.ram.save.equipment.contains(Equipment::IRON_BOOTS), state.ram.save.equipment.contains(Equipment::HOVER_BOOTS))),
        toggle_left: Box::new(|state| state.ram.save.equipment.toggle(Equipment::IRON_BOOTS)),
        toggle_right: Box::new(|state| state.ram.save.equipment.toggle(Equipment::HOVER_BOOTS)),
    },
    IronBoots: Simple {
        img: ImageInfo::new("iron_boots"),
        active: Box::new(|state| state.ram.save.equipment.contains(Equipment::IRON_BOOTS)),
        toggle: Box::new(|state| state.ram.save.equipment.toggle(Equipment::IRON_BOOTS)),
    },
    HoverBoots: Simple {
        img: ImageInfo::new("hover_boots"),
        active: Box::new(|state| state.ram.save.equipment.contains(Equipment::HOVER_BOOTS)),
        toggle: Box::new(|state| state.ram.save.equipment.toggle(Equipment::HOVER_BOOTS)),
    },
    MirrorShield: Simple {
        img: ImageInfo::new("mirror_shield"),
        active: Box::new(|state| state.ram.save.equipment.contains(Equipment::MIRROR_SHIELD)),
        toggle: Box::new(|state| state.ram.save.equipment.toggle(Equipment::MIRROR_SHIELD)),
    },
    ChildTrade: Sequence {
        idx: Box::new(|state| match state.ram.save.inv.child_trade_item {
            ChildTradeItem::None => 0,
            ChildTradeItem::WeirdEgg => 1,
            ChildTradeItem::Chicken => 2,
            ChildTradeItem::ZeldasLetter | ChildTradeItem::GoronMask | ChildTradeItem::ZoraMask | ChildTradeItem::GerudoMask | ChildTradeItem::SoldOut => 3, //TODO for SOLD OUT, check trade quest progress
            ChildTradeItem::KeatonMask => 4,
            ChildTradeItem::SkullMask => 5,
            ChildTradeItem::SpookyMask => 6,
            ChildTradeItem::BunnyHood => 7,
            ChildTradeItem::MaskOfTruth => 8,
        }),
        img: Box::new(|state| match state.ram.save.inv.child_trade_item {
            ChildTradeItem::None => (false, ImageInfo::new("white_egg")),
            ChildTradeItem::WeirdEgg => (true, ImageInfo::new("white_egg")),
            ChildTradeItem::Chicken => (true, ImageInfo::new("white_chicken")),
            ChildTradeItem::ZeldasLetter | ChildTradeItem::GoronMask | ChildTradeItem::ZoraMask | ChildTradeItem::GerudoMask | ChildTradeItem::SoldOut => (true, ImageInfo::new("zelda_letter")), //TODO for SOLD OUT, check trade quest progress
            ChildTradeItem::KeatonMask => (true, ImageInfo::new("keaton_mask")),
            ChildTradeItem::SkullMask => (true, ImageInfo::new("skull_mask")),
            ChildTradeItem::SpookyMask => (true, ImageInfo::new("spooky_mask")),
            ChildTradeItem::BunnyHood => (true, ImageInfo::new("bunny_hood")),
            ChildTradeItem::MaskOfTruth => (true, ImageInfo::new("mask_of_truth")),
        }),
        increment: Box::new(|state| state.ram.save.inv.child_trade_item = match state.ram.save.inv.child_trade_item {
            ChildTradeItem::None => ChildTradeItem::WeirdEgg,
            ChildTradeItem::WeirdEgg => ChildTradeItem::Chicken,
            ChildTradeItem::Chicken => ChildTradeItem::ZeldasLetter,
            ChildTradeItem::ZeldasLetter | ChildTradeItem::GoronMask | ChildTradeItem::ZoraMask | ChildTradeItem::GerudoMask | ChildTradeItem::SoldOut => ChildTradeItem::KeatonMask, //TODO for SOLD OUT, check trade quest progress
            ChildTradeItem::KeatonMask => ChildTradeItem::SkullMask,
            ChildTradeItem::SkullMask => ChildTradeItem::SpookyMask,
            ChildTradeItem::SpookyMask => ChildTradeItem::BunnyHood,
            ChildTradeItem::BunnyHood => ChildTradeItem::MaskOfTruth,
            ChildTradeItem::MaskOfTruth => ChildTradeItem::None,
        }),
        decrement: Box::new(|state| state.ram.save.inv.child_trade_item = match state.ram.save.inv.child_trade_item {
            ChildTradeItem::None => ChildTradeItem::MaskOfTruth,
            ChildTradeItem::WeirdEgg => ChildTradeItem::None,
            ChildTradeItem::Chicken => ChildTradeItem::WeirdEgg,
            ChildTradeItem::ZeldasLetter | ChildTradeItem::GoronMask | ChildTradeItem::ZoraMask | ChildTradeItem::GerudoMask | ChildTradeItem::SoldOut => ChildTradeItem::Chicken, //TODO for SOLD OUT, check trade quest progress
            ChildTradeItem::KeatonMask => ChildTradeItem::ZeldasLetter,
            ChildTradeItem::SkullMask => ChildTradeItem::KeatonMask,
            ChildTradeItem::SpookyMask => ChildTradeItem::SkullMask,
            ChildTradeItem::BunnyHood => ChildTradeItem::SpookyMask,
            ChildTradeItem::MaskOfTruth => ChildTradeItem::BunnyHood,
        }),
    },
    ChildTradeNoChicken: Sequence {
        idx: Box::new(|state| match state.ram.save.inv.child_trade_item {
            ChildTradeItem::None => 0,
            ChildTradeItem::WeirdEgg | ChildTradeItem::Chicken => 1,
            ChildTradeItem::ZeldasLetter | ChildTradeItem::GoronMask | ChildTradeItem::ZoraMask | ChildTradeItem::GerudoMask | ChildTradeItem::SoldOut => 2, //TODO for SOLD OUT, check trade quest progress
            ChildTradeItem::KeatonMask => 3,
            ChildTradeItem::SkullMask => 4,
            ChildTradeItem::SpookyMask => 5,
            ChildTradeItem::BunnyHood => 6,
            ChildTradeItem::MaskOfTruth => 7,
        }),
        img: Box::new(|state| match state.ram.save.inv.child_trade_item {
            ChildTradeItem::None => (false, ImageInfo::new("white_egg")),
            ChildTradeItem::WeirdEgg | ChildTradeItem::Chicken => (true, ImageInfo::new("white_egg")),
            ChildTradeItem::ZeldasLetter | ChildTradeItem::GoronMask | ChildTradeItem::ZoraMask | ChildTradeItem::GerudoMask | ChildTradeItem::SoldOut => (true, ImageInfo::new("zelda_letter")), //TODO for SOLD OUT, check trade quest progress
            ChildTradeItem::KeatonMask => (true, ImageInfo::new("keaton_mask")),
            ChildTradeItem::SkullMask => (true, ImageInfo::new("skull_mask")),
            ChildTradeItem::SpookyMask => (true, ImageInfo::new("spooky_mask")),
            ChildTradeItem::BunnyHood => (true, ImageInfo::new("bunny_hood")),
            ChildTradeItem::MaskOfTruth => (true, ImageInfo::new("mask_of_truth")),
        }),
        increment: Box::new(|state| state.ram.save.inv.child_trade_item = match state.ram.save.inv.child_trade_item {
            ChildTradeItem::None => ChildTradeItem::WeirdEgg,
            ChildTradeItem::WeirdEgg | ChildTradeItem::Chicken => ChildTradeItem::ZeldasLetter,
            ChildTradeItem::ZeldasLetter | ChildTradeItem::GoronMask | ChildTradeItem::ZoraMask | ChildTradeItem::GerudoMask | ChildTradeItem::SoldOut => ChildTradeItem::KeatonMask, //TODO for SOLD OUT, check trade quest progress
            ChildTradeItem::KeatonMask => ChildTradeItem::SkullMask,
            ChildTradeItem::SkullMask => ChildTradeItem::SpookyMask,
            ChildTradeItem::SpookyMask => ChildTradeItem::BunnyHood,
            ChildTradeItem::BunnyHood => ChildTradeItem::MaskOfTruth,
            ChildTradeItem::MaskOfTruth => ChildTradeItem::None,
        }),
        decrement: Box::new(|state| state.ram.save.inv.child_trade_item = match state.ram.save.inv.child_trade_item {
            ChildTradeItem::None => ChildTradeItem::MaskOfTruth,
            ChildTradeItem::WeirdEgg | ChildTradeItem::Chicken => ChildTradeItem::None,
            ChildTradeItem::ZeldasLetter | ChildTradeItem::GoronMask | ChildTradeItem::ZoraMask | ChildTradeItem::GerudoMask | ChildTradeItem::SoldOut => ChildTradeItem::WeirdEgg, //TODO for SOLD OUT, check trade quest progress
            ChildTradeItem::KeatonMask => ChildTradeItem::ZeldasLetter,
            ChildTradeItem::SkullMask => ChildTradeItem::KeatonMask,
            ChildTradeItem::SpookyMask => ChildTradeItem::SkullMask,
            ChildTradeItem::BunnyHood => ChildTradeItem::SpookyMask,
            ChildTradeItem::MaskOfTruth => ChildTradeItem::BunnyHood,
        }),
    },
    ChildTradeSoldOut: Sequence {
        idx: Box::new(|state| match state.ram.save.inv.child_trade_item {
            ChildTradeItem::None => 0,
            ChildTradeItem::WeirdEgg => 1,
            ChildTradeItem::Chicken => 2,
            ChildTradeItem::ZeldasLetter | ChildTradeItem::GoronMask | ChildTradeItem::ZoraMask | ChildTradeItem::GerudoMask | ChildTradeItem::SoldOut => 3, //TODO for SOLD OUT, check trade quest progress
            //TODO Zelda's letter turned in => 4
            ChildTradeItem::KeatonMask => 5,
            //TODO Keaton mask sold => 6
            ChildTradeItem::SkullMask => 7,
            //TODO skull mask sold => 8
            ChildTradeItem::SpookyMask => 9,
            //TODO spooky mask sold => 10
            ChildTradeItem::BunnyHood => 11,
            //TODO bunny hood sold => 12
            ChildTradeItem::MaskOfTruth => 13,
        }),
        img: Box::new(|state| match state.ram.save.inv.child_trade_item {
            ChildTradeItem::None => (false, ImageInfo::new("white_egg")),
            ChildTradeItem::WeirdEgg => (true, ImageInfo::new("white_egg")),
            ChildTradeItem::Chicken => (true, ImageInfo::new("white_chicken")),
            ChildTradeItem::ZeldasLetter | ChildTradeItem::GoronMask | ChildTradeItem::ZoraMask | ChildTradeItem::GerudoMask | ChildTradeItem::SoldOut => (true, ImageInfo::new("zelda_letter")), //TODO for SOLD OUT, check trade quest progress
            //TODO Zelda's letter turned in => SOLD OUT
            ChildTradeItem::KeatonMask => (true, ImageInfo::new("keaton_mask")),
            //TODO Keaton mask sold => SOLD OUT
            ChildTradeItem::SkullMask => (true, ImageInfo::new("skull_mask")),
            //TODO skull mask sold => SOLD OUT
            ChildTradeItem::SpookyMask => (true, ImageInfo::new("spooky_mask")),
            //TODO spooky mask sold => SOLD OUT
            ChildTradeItem::BunnyHood => (true, ImageInfo::new("bunny_hood")),
            //TODO bunny hood sold => SOLD OUT
            ChildTradeItem::MaskOfTruth => (true, ImageInfo::new("mask_of_truth")),
        }),
        increment: Box::new(|state| state.ram.save.inv.child_trade_item = match state.ram.save.inv.child_trade_item {
            //TODO consider sold-out states
            ChildTradeItem::None => ChildTradeItem::WeirdEgg,
            ChildTradeItem::WeirdEgg => ChildTradeItem::Chicken,
            ChildTradeItem::Chicken => ChildTradeItem::ZeldasLetter,
            ChildTradeItem::ZeldasLetter | ChildTradeItem::GoronMask | ChildTradeItem::ZoraMask | ChildTradeItem::GerudoMask | ChildTradeItem::SoldOut => ChildTradeItem::KeatonMask, //TODO for SOLD OUT, check trade quest progress
            ChildTradeItem::KeatonMask => ChildTradeItem::SkullMask,
            ChildTradeItem::SkullMask => ChildTradeItem::SpookyMask,
            ChildTradeItem::SpookyMask => ChildTradeItem::BunnyHood,
            ChildTradeItem::BunnyHood => ChildTradeItem::MaskOfTruth,
            ChildTradeItem::MaskOfTruth => ChildTradeItem::None,
        }),
        decrement: Box::new(|state| state.ram.save.inv.child_trade_item = match state.ram.save.inv.child_trade_item {
            //TODO consider sold-out states
            ChildTradeItem::None => ChildTradeItem::MaskOfTruth,
            ChildTradeItem::WeirdEgg => ChildTradeItem::None,
            ChildTradeItem::Chicken => ChildTradeItem::WeirdEgg,
            ChildTradeItem::ZeldasLetter | ChildTradeItem::GoronMask | ChildTradeItem::ZoraMask | ChildTradeItem::GerudoMask | ChildTradeItem::SoldOut => ChildTradeItem::Chicken, //TODO for SOLD OUT, check trade quest progress
            ChildTradeItem::KeatonMask => ChildTradeItem::ZeldasLetter,
            ChildTradeItem::SkullMask => ChildTradeItem::KeatonMask,
            ChildTradeItem::SpookyMask => ChildTradeItem::SkullMask,
            ChildTradeItem::BunnyHood => ChildTradeItem::SpookyMask,
            ChildTradeItem::MaskOfTruth => ChildTradeItem::BunnyHood,
        }),
    },
    Ocarina: Overlay {
        main_img: ImageInfo::new("ocarina"),
        overlay_img: ImageInfo::new("scarecrow"),
        //TODO this has multiple issues:
        // * it leaks the info that the free scarecrow setting is active as soon as the scarecrow song has been set as child
        // * it doesn't display free scarecrow song known from settings input
        // see also https://github.com/fenhl/oottracker/issues/21
        active: Box::new(|state| (state.ram.save.inv.ocarina != Ocarina::None, state.ram.save.scarecrow_song_child && state.ram.save.event_chk_inf.9.contains(EventChkInf9::SCARECROW_SONG))),
        toggle_main: Box::new(|state| state.ram.save.inv.ocarina = match state.ram.save.inv.ocarina {
            Ocarina::None => Ocarina::FairyOcarina,
            Ocarina::FairyOcarina | Ocarina::OcarinaOfTime => Ocarina::None,
        }),
        toggle_overlay: Box::new(|state| if state.ram.save.scarecrow_song_child && state.ram.save.event_chk_inf.9.contains(EventChkInf9::SCARECROW_SONG) {
            state.ram.save.event_chk_inf.9.remove(EventChkInf9::SCARECROW_SONG);
        } else {
            state.ram.save.scarecrow_song_child = true;
            state.ram.save.event_chk_inf.9.insert(EventChkInf9::SCARECROW_SONG);
        }), //TODO make sure free scarecrow knowledge is toggled properly
    },
    Beans: Simple { //TODO overlay with number bought if auto-tracking is on & shuffle beans is off
        img: ImageInfo::new("beans"),
        active: Box::new(|state| state.ram.save.inv.beans),
        toggle: Box::new(|state| state.ram.save.inv.beans = !state.ram.save.inv.beans),
    },
    SwordCard: Composite {
        left_img: ImageInfo::new("kokiri_sword"),
        right_img: ImageInfo::new("gerudo_card"),
        both_img: ImageInfo::extra("composite_ksword_gcard"),
        active: Box::new(|state| (state.ram.save.equipment.contains(Equipment::KOKIRI_SWORD), state.ram.save.quest_items.contains(QuestItems::GERUDO_CARD))),
        toggle_left: Box::new(|state| state.ram.save.equipment.toggle(Equipment::KOKIRI_SWORD)),
        toggle_right: Box::new(|state| state.ram.save.quest_items.toggle(QuestItems::GERUDO_CARD)),
    },
    SwordShield: Overlay {
        main_img: ImageInfo::new("kokiri_sword"),
        overlay_img: ImageInfo::extra("deku_shield_badge"),
        active: Box::new(|state| (state.ram.save.equipment.contains(Equipment::KOKIRI_SWORD), state.ram.save.equipment.contains(Equipment::DEKU_SHIELD))),
        toggle_main: Box::new(|state| state.ram.save.equipment.toggle(Equipment::KOKIRI_SWORD)),
        toggle_overlay: Box::new(|state| state.ram.save.equipment.toggle(Equipment::DEKU_SHIELD)),
    },
    KokiriSword: Simple {
        img: ImageInfo::new("kokiri_sword"),
        active: Box::new(|state| state.ram.save.equipment.contains(Equipment::KOKIRI_SWORD)),
        toggle: Box::new(|state| state.ram.save.equipment.toggle(Equipment::KOKIRI_SWORD)),
    },
    Tunics: Composite {
        left_img: ImageInfo::new("goron_tunic"),
        right_img: ImageInfo::new("zora_tunic"),
        both_img: ImageInfo::new("composite_tunics"),
        active: Box::new(|state| (state.ram.save.equipment.contains(Equipment::GORON_TUNIC), state.ram.save.equipment.contains(Equipment::ZORA_TUNIC))),
        toggle_left: Box::new(|state| state.ram.save.equipment.toggle(Equipment::GORON_TUNIC)),
        toggle_right: Box::new(|state| state.ram.save.equipment.toggle(Equipment::ZORA_TUNIC)),
    },
    GoronTunic: Simple {
        img: ImageInfo::new("goron_tunic"),
        active: Box::new(|state| state.ram.save.equipment.contains(Equipment::GORON_TUNIC)),
        toggle: Box::new(|state| state.ram.save.equipment.toggle(Equipment::GORON_TUNIC)),
    },
    ZoraTunic: Simple {
        img: ImageInfo::new("zora_tunic"),
        active: Box::new(|state| state.ram.save.equipment.contains(Equipment::ZORA_TUNIC)),
        toggle: Box::new(|state| state.ram.save.equipment.toggle(Equipment::ZORA_TUNIC)),
    },
    Triforce: Count {
        dimmed_img: ImageInfo::new("triforce"),
        img: ImageInfo::new("force"),
        get: Box::new(|state| state.ram.save.triforce_pieces()),
        set: Box::new(|state, value| state.ram.save.set_triforce_pieces(value)),
        max: 100,
        step: 1,
    },
    BigPoeTriforce: BigPoeTriforce,
    TriforceOneAndFives: Sequence {
        idx: Box::new(|state| match state.ram.save.triforce_pieces() {
            0 => 0,
            1..=4 => 1,
            5..=9 => 2,
            10..=14 => 3,
            15..=19 => 4,
            20..=24 => 5,
            25..=29 => 6,
            30..=34 => 7,
            35..=39 => 8,
            40..=44 => 9,
            45..=49 => 10,
            50..=54 => 11,
            55..=59 => 12,
            _ => 13,
        }),
        img: Box::new(|state| (state.ram.save.triforce_pieces() > 0, ImageInfo::new("triforce"))), //TODO images from count?
        increment: Box::new(|state| {
            let new_val = match state.ram.save.triforce_pieces() {
                0 => 1,
                1..=4 => 5,
                5..=9 => 10,
                10..=14 => 15,
                15..=19 => 20,
                20..=24 => 25,
                25..=29 => 30,
                30..=34 => 35,
                35..=39 => 40,
                40..=44 => 45,
                45..=49 => 50,
                50..=54 => 55,
                55..=59 => 60,
                _ => 0,
            };
            state.ram.save.set_triforce_pieces(new_val);
        }),
        decrement: Box::new(|state| {
            let new_val = match state.ram.save.triforce_pieces() {
                0 => 60,
                1..=4 => 0,
                5..=9 => 1,
                10..=14 => 5,
                15..=19 => 10,
                20..=24 => 15,
                25..=29 => 20,
                30..=34 => 25,
                35..=39 => 30,
                40..=44 => 35,
                45..=49 => 40,
                50..=54 => 45,
                55..=59 => 50,
                _ => 55,
            };
            state.ram.save.set_triforce_pieces(new_val);
        }),
    },
    ZeldasLullaby: Song {
        song: QuestItems::ZELDAS_LULLABY,
        check: "Song from Impa",
        toggle_overlay: Box::new(|eci| eci.5.toggle(EventChkInf5::SONG_FROM_IMPA)),
    },
    ZeldasLullabyCheck: SongCheck {
        check: "Song from Impa",
        toggle_overlay: Box::new(|eci| eci.5.toggle(EventChkInf5::SONG_FROM_IMPA)),
    },
    EponasSong: Song {
        song: QuestItems::EPONAS_SONG,
        check: "Song from Malon",
        toggle_overlay: Box::new(|eci| eci.5.toggle(EventChkInf5::SONG_FROM_MALON)),
    },
    EponasSongCheck: SongCheck {
        check: "Song from Malon",
        toggle_overlay: Box::new(|eci| eci.5.toggle(EventChkInf5::SONG_FROM_MALON)),
    },
    SariasSong: Song {
        song: QuestItems::SARIAS_SONG,
        check: "Song from Saria",
        toggle_overlay: Box::new(|eci| eci.5.toggle(EventChkInf5::SONG_FROM_SARIA)),
    },
    SariasSongCheck: SongCheck {
        check: "Song from Saria",
        toggle_overlay: Box::new(|eci| eci.5.toggle(EventChkInf5::SONG_FROM_SARIA)),
    },
    SunsSong: Song {
        song: QuestItems::SUNS_SONG,
        check: "Song from Royal Familys Tomb",
        toggle_overlay: Box::new(|eci| eci.5.toggle(EventChkInf5::SONG_FROM_ROYAL_FAMILYS_TOMB)),
    },
    SunsSongCheck: SongCheck {
        check: "Song from Royal Familys Tomb",
        toggle_overlay: Box::new(|eci| eci.5.toggle(EventChkInf5::SONG_FROM_ROYAL_FAMILYS_TOMB)),
    },
    SongOfTime: Song {
        song: QuestItems::SONG_OF_TIME,
        check: "Song from Ocarina of Time",
        toggle_overlay: Box::new(|eci| eci.10.toggle(EventChkInf10::SONG_FROM_OCARINA_OF_TIME)),
    },
    SongOfTimeCheck: SongCheck {
        check: "Song from Ocarina of Time",
        toggle_overlay: Box::new(|eci| eci.10.toggle(EventChkInf10::SONG_FROM_OCARINA_OF_TIME)),
    },
    SongOfStorms: Song {
        song: QuestItems::SONG_OF_STORMS,
        check: "Song from Windmill",
        toggle_overlay: Box::new(|eci| eci.5.toggle(EventChkInf5::SONG_FROM_WINDMILL)),
    },
    SongOfStormsCheck: SongCheck {
        check: "Song from Windmill",
        toggle_overlay: Box::new(|eci| eci.5.toggle(EventChkInf5::SONG_FROM_WINDMILL)),
    },
    Minuet: Song {
        song: QuestItems::MINUET_OF_FOREST,
        check: "Sheik in Forest",
        toggle_overlay: Box::new(|eci| eci.5.toggle(EventChkInf5::SHEIK_IN_FOREST)),
    },
    MinuetCheck: SongCheck {
        check: "Sheik in Forest",
        toggle_overlay: Box::new(|eci| eci.5.toggle(EventChkInf5::SHEIK_IN_FOREST)),
    },
    Bolero: Song {
        song: QuestItems::BOLERO_OF_FIRE,
        check: "Sheik in Crater",
        toggle_overlay: Box::new(|eci| eci.5.toggle(EventChkInf5::SHEIK_IN_CRATER)),
    },
    BoleroCheck: SongCheck {
        check: "Sheik in Crater",
        toggle_overlay: Box::new(|eci| eci.5.toggle(EventChkInf5::SHEIK_IN_CRATER)),
    },
    Serenade: Song {
        song: QuestItems::SERENADE_OF_WATER,
        check: "Sheik in Ice Cavern",
        toggle_overlay: Box::new(|eci| eci.5.toggle(EventChkInf5::SHEIK_IN_ICE_CAVERN)),
    },
    SerenadeCheck: SongCheck {
        check: "Sheik in Ice Cavern",
        toggle_overlay: Box::new(|eci| eci.5.toggle(EventChkInf5::SHEIK_IN_ICE_CAVERN)),
    },
    Requiem: Song {
        song: QuestItems::REQUIEM_OF_SPIRIT,
        check: "Sheik at Colossus",
        toggle_overlay: Box::new(|eci| eci.10.toggle(EventChkInf10::SHEIK_AT_COLOSSUS)),
    },
    RequiemCheck: SongCheck {
        check: "Sheik at Colossus",
        toggle_overlay: Box::new(|eci| eci.10.toggle(EventChkInf10::SHEIK_AT_COLOSSUS)),
    },
    Nocturne: Song {
        song: QuestItems::NOCTURNE_OF_SHADOW,
        check: "Sheik in Kakariko",
        toggle_overlay: Box::new(|eci| eci.5.toggle(EventChkInf5::SHEIK_IN_KAKARIKO)),
    },
    NocturneCheck: SongCheck {
        check: "Sheik in Kakariko",
        toggle_overlay: Box::new(|eci| eci.5.toggle(EventChkInf5::SHEIK_IN_KAKARIKO)),
    },
    Prelude: Song {
        song: QuestItems::PRELUDE_OF_LIGHT,
        check: "Sheik at Temple",
        toggle_overlay: Box::new(|eci| eci.5.toggle(EventChkInf5::SHEIK_AT_TEMPLE)),
    },
    PreludeCheck: SongCheck {
        check: "Sheik at Temple",
        toggle_overlay: Box::new(|eci| eci.5.toggle(EventChkInf5::SHEIK_AT_TEMPLE)),
    },
    FreeReward: FreeReward,
    DekuMq: Mq(Dungeon::Main(MainDungeon::DekuTree)),
    DcMq: Mq(Dungeon::Main(MainDungeon::DodongosCavern)),
    JabuMq: Mq(Dungeon::Main(MainDungeon::JabuJabu)),
    ForestMq: Mq(Dungeon::Main(MainDungeon::ForestTemple)),
    ForestSmallKeys: TrackerCellKind::SmallKeys {
        get: Box::new(|keys| keys.forest_temple),
        set: Box::new(|keys, value| keys.forest_temple = value),
        max_vanilla: 5,
        max_mq: 6,
        label: "Frst",
    },
    ForestBossKey: BossKey {
        active: Box::new(|keys| keys.forest_temple.contains(DungeonItems::BOSS_KEY)),
        toggle: Box::new(|keys| keys.forest_temple.toggle(DungeonItems::BOSS_KEY)),
        label: "Frst",
    },
    ForestKeys: CompositeKeys {
        small: TrackerCellId::ForestSmallKeys,
        boss: TrackerCellId::ForestBossKey,
    },
    FireMq: Mq(Dungeon::Main(MainDungeon::FireTemple)),
    FireSmallKeys: TrackerCellKind::SmallKeys {
        get: Box::new(|keys| keys.fire_temple),
        set: Box::new(|keys, value| keys.fire_temple = value),
        max_vanilla: 8,
        max_mq: 5,
        label: "Fire",
    },
    FireBossKey: BossKey {
        active: Box::new(|keys| keys.fire_temple.contains(DungeonItems::BOSS_KEY)),
        toggle: Box::new(|keys| keys.fire_temple.toggle(DungeonItems::BOSS_KEY)),
        label: "Fire",
    },
    FireKeys: CompositeKeys {
        small: TrackerCellId::FireSmallKeys,
        boss: TrackerCellId::FireBossKey,
    },
    WaterMq: Mq(Dungeon::Main(MainDungeon::WaterTemple)),
    WaterSmallKeys: TrackerCellKind::SmallKeys {
        get: Box::new(|keys| keys.water_temple),
        set: Box::new(|keys, value| keys.water_temple = value),
        max_vanilla: 6,
        max_mq: 2,
        label: "Watr",
    },
    WaterBossKey: BossKey {
        active: Box::new(|keys| keys.water_temple.contains(DungeonItems::BOSS_KEY)),
        toggle: Box::new(|keys| keys.water_temple.toggle(DungeonItems::BOSS_KEY)),
        label: "Watr",
    },
    WaterKeys: CompositeKeys {
        small: TrackerCellId::WaterSmallKeys,
        boss: TrackerCellId::WaterBossKey,
    },
    ShadowMq: Mq(Dungeon::Main(MainDungeon::ShadowTemple)),
    ShadowSmallKeys: TrackerCellKind::SmallKeys {
        get: Box::new(|keys| keys.shadow_temple),
        set: Box::new(|keys, value| keys.shadow_temple = value),
        max_vanilla: 5,
        max_mq: 6,
        label: "Shdw",
    },
    ShadowBossKey: BossKey {
        active: Box::new(|keys| keys.shadow_temple.contains(DungeonItems::BOSS_KEY)),
        toggle: Box::new(|keys| keys.shadow_temple.toggle(DungeonItems::BOSS_KEY)),
        label: "Shdw",
    },
    ShadowKeys: CompositeKeys {
        small: TrackerCellId::ShadowSmallKeys,
        boss: TrackerCellId::ShadowBossKey,
    },
    SpiritMq: Mq(Dungeon::Main(MainDungeon::SpiritTemple)),
    SpiritSmallKeys: TrackerCellKind::SmallKeys {
        get: Box::new(|keys| keys.spirit_temple),
        set: Box::new(|keys, value| keys.spirit_temple = value),
        max_vanilla: 5,
        max_mq: 7,
        label: "Sprt",
    },
    SpiritBossKey: BossKey {
        active: Box::new(|keys| keys.spirit_temple.contains(DungeonItems::BOSS_KEY)),
        toggle: Box::new(|keys| keys.spirit_temple.toggle(DungeonItems::BOSS_KEY)),
        label: "Sprt",
    },
    SpiritKeys: CompositeKeys {
        small: TrackerCellId::SpiritSmallKeys,
        boss: TrackerCellId::SpiritBossKey,
    },
    IceMq: Mq(Dungeon::IceCavern),
    WellMq: Mq(Dungeon::BottomOfTheWell),
    WellSmallKeys: TrackerCellKind::SmallKeys {
        get: Box::new(|keys| keys.bottom_of_the_well),
        set: Box::new(|keys, value| keys.bottom_of_the_well = value),
        max_vanilla: 3,
        max_mq: 2,
        label: "Well",
    },
    FortressMq: FortressMq,
    FortressSmallKeys: TrackerCellKind::SmallKeys {
        get: Box::new(|keys| keys.thieves_hideout),
        set: Box::new(|keys, value| keys.thieves_hideout = value),
        max_vanilla: 4,
        max_mq: 4,
        label: "Fort",
    },
    GtgMq: Mq(Dungeon::GerudoTrainingGround),
    GtgSmallKeys: TrackerCellKind::SmallKeys {
        get: Box::new(|keys| keys.gerudo_training_ground),
        set: Box::new(|keys, value| keys.gerudo_training_ground = value),
        max_vanilla: 9,
        max_mq: 3,
        label: "GTG",
    },
    GanonMq: Mq(Dungeon::GanonsCastle),
    GanonSmallKeys: TrackerCellKind::SmallKeys {
        get: Box::new(|keys| keys.ganons_castle),
        set: Box::new(|keys, value| keys.ganons_castle = value),
        max_vanilla: 2,
        max_mq: 3,
        label: "Ganon",
    },
    GanonBossKey: BossKey {
        active: Box::new(|keys| keys.ganons_castle.contains(DungeonItems::BOSS_KEY)),
        toggle: Box::new(|keys| keys.ganons_castle.toggle(DungeonItems::BOSS_KEY)),
        label: "Ganon",
    },
    GanonKeys: CompositeKeys {
        small: TrackerCellId::GanonSmallKeys,
        boss: TrackerCellId::GanonBossKey,
    },

    // ============================================================================
    // OoT Dungeon Maps
    // ============================================================================
    DekuMap: OotMap {
        active: Box::new(|keys| keys.deku_tree.contains(DungeonItems::MAP)),
        toggle: Box::new(|keys| keys.deku_tree.toggle(DungeonItems::MAP)),
    },
    DcMap: OotMap {
        active: Box::new(|keys| keys.dodongos_cavern.contains(DungeonItems::MAP)),
        toggle: Box::new(|keys| keys.dodongos_cavern.toggle(DungeonItems::MAP)),
    },
    JabuMap: OotMap {
        active: Box::new(|keys| keys.jabu_jabu.contains(DungeonItems::MAP)),
        toggle: Box::new(|keys| keys.jabu_jabu.toggle(DungeonItems::MAP)),
    },
    ForestMap: OotMap {
        active: Box::new(|keys| keys.forest_temple.contains(DungeonItems::MAP)),
        toggle: Box::new(|keys| keys.forest_temple.toggle(DungeonItems::MAP)),
    },
    FireMap: OotMap {
        active: Box::new(|keys| keys.fire_temple.contains(DungeonItems::MAP)),
        toggle: Box::new(|keys| keys.fire_temple.toggle(DungeonItems::MAP)),
    },
    WaterMap: OotMap {
        active: Box::new(|keys| keys.water_temple.contains(DungeonItems::MAP)),
        toggle: Box::new(|keys| keys.water_temple.toggle(DungeonItems::MAP)),
    },
    ShadowMap: OotMap {
        active: Box::new(|keys| keys.shadow_temple.contains(DungeonItems::MAP)),
        toggle: Box::new(|keys| keys.shadow_temple.toggle(DungeonItems::MAP)),
    },
    SpiritMap: OotMap {
        active: Box::new(|keys| keys.spirit_temple.contains(DungeonItems::MAP)),
        toggle: Box::new(|keys| keys.spirit_temple.toggle(DungeonItems::MAP)),
    },
    WellMap: OotMap {
        active: Box::new(|keys| keys.bottom_of_the_well.contains(DungeonItems::MAP)),
        toggle: Box::new(|keys| keys.bottom_of_the_well.toggle(DungeonItems::MAP)),
    },
    IceMap: OotMap {
        active: Box::new(|keys| keys.ice_cavern.contains(DungeonItems::MAP)),
        toggle: Box::new(|keys| keys.ice_cavern.toggle(DungeonItems::MAP)),
    },
    GanonMap: OotMap {
        active: Box::new(|keys| keys.ganons_castle.contains(DungeonItems::MAP)),
        toggle: Box::new(|keys| keys.ganons_castle.toggle(DungeonItems::MAP)),
    },

    // ============================================================================
    // OoT Dungeon Compasses
    // ============================================================================
    DekuCompass: OotCompass {
        active: Box::new(|keys| keys.deku_tree.contains(DungeonItems::COMPASS)),
        toggle: Box::new(|keys| keys.deku_tree.toggle(DungeonItems::COMPASS)),
        loc: ImageInfo::new("deku_text"),
    },
    DcCompass: OotCompass {
        active: Box::new(|keys| keys.dodongos_cavern.contains(DungeonItems::COMPASS)),
        toggle: Box::new(|keys| keys.dodongos_cavern.toggle(DungeonItems::COMPASS)),
        loc: ImageInfo::new("dc_text"),
    },
    JabuCompass: OotCompass {
        active: Box::new(|keys| keys.jabu_jabu.contains(DungeonItems::COMPASS)),
        toggle: Box::new(|keys| keys.jabu_jabu.toggle(DungeonItems::COMPASS)),
        loc: ImageInfo::new("jabu_text"),
    },
    ForestCompass: OotCompass {
        active: Box::new(|keys| keys.forest_temple.contains(DungeonItems::COMPASS)),
        toggle: Box::new(|keys| keys.forest_temple.toggle(DungeonItems::COMPASS)),
        loc: ImageInfo::new("forest_text"),
    },
    FireCompass: OotCompass {
        active: Box::new(|keys| keys.fire_temple.contains(DungeonItems::COMPASS)),
        toggle: Box::new(|keys| keys.fire_temple.toggle(DungeonItems::COMPASS)),
        loc: ImageInfo::new("fire_text"),
    },
    WaterCompass: OotCompass {
        active: Box::new(|keys| keys.water_temple.contains(DungeonItems::COMPASS)),
        toggle: Box::new(|keys| keys.water_temple.toggle(DungeonItems::COMPASS)),
        loc: ImageInfo::new("water_text"),
    },
    ShadowCompass: OotCompass {
        active: Box::new(|keys| keys.shadow_temple.contains(DungeonItems::COMPASS)),
        toggle: Box::new(|keys| keys.shadow_temple.toggle(DungeonItems::COMPASS)),
        loc: ImageInfo::new("shadow_text"),
    },
    SpiritCompass: OotCompass {
        active: Box::new(|keys| keys.spirit_temple.contains(DungeonItems::COMPASS)),
        toggle: Box::new(|keys| keys.spirit_temple.toggle(DungeonItems::COMPASS)),
        loc: ImageInfo::new("spirit_text"),
    },
    WellCompass: OotCompass {
        active: Box::new(|keys| keys.bottom_of_the_well.contains(DungeonItems::COMPASS)),
        toggle: Box::new(|keys| keys.bottom_of_the_well.toggle(DungeonItems::COMPASS)),
        loc: ImageInfo::new("well_text"),
    },
    IceCompass: OotCompass {
        active: Box::new(|keys| keys.ice_cavern.contains(DungeonItems::COMPASS)),
        toggle: Box::new(|keys| keys.ice_cavern.toggle(DungeonItems::COMPASS)),
        loc: ImageInfo::new("ice_text"),
    },

    // ============================================================================
    // MM Dungeon Boss Keys
    // ============================================================================
    MmWoodfallBossKey: MmBossKey {
        active: Box::new(|keys| keys.woodfall.contains(crate::mm_save::MmDungeonItems::BOSS_KEY)),
        toggle: Box::new(|keys| keys.woodfall.toggle(crate::mm_save::MmDungeonItems::BOSS_KEY)),
        label: "WF",
    },
    MmSnowheadBossKey: MmBossKey {
        active: Box::new(|keys| keys.snowhead.contains(crate::mm_save::MmDungeonItems::BOSS_KEY)),
        toggle: Box::new(|keys| keys.snowhead.toggle(crate::mm_save::MmDungeonItems::BOSS_KEY)),
        label: "SH",
    },
    MmGreatBayBossKey: MmBossKey {
        active: Box::new(|keys| keys.great_bay.contains(crate::mm_save::MmDungeonItems::BOSS_KEY)),
        toggle: Box::new(|keys| keys.great_bay.toggle(crate::mm_save::MmDungeonItems::BOSS_KEY)),
        label: "GB",
    },
    MmStoneTowerBossKey: MmBossKey {
        active: Box::new(|keys| keys.stone_tower.contains(crate::mm_save::MmDungeonItems::BOSS_KEY)),
        toggle: Box::new(|keys| keys.stone_tower.toggle(crate::mm_save::MmDungeonItems::BOSS_KEY)),
        label: "ST",
    },

    // ============================================================================
    // MM Dungeon Maps
    // ============================================================================
    MmWoodfallMap: MmMap {
        active: Box::new(|keys| keys.woodfall.contains(crate::mm_save::MmDungeonItems::MAP)),
        toggle: Box::new(|keys| keys.woodfall.toggle(crate::mm_save::MmDungeonItems::MAP)),
    },
    MmSnowheadMap: MmMap {
        active: Box::new(|keys| keys.snowhead.contains(crate::mm_save::MmDungeonItems::MAP)),
        toggle: Box::new(|keys| keys.snowhead.toggle(crate::mm_save::MmDungeonItems::MAP)),
    },
    MmGreatBayMap: MmMap {
        active: Box::new(|keys| keys.great_bay.contains(crate::mm_save::MmDungeonItems::MAP)),
        toggle: Box::new(|keys| keys.great_bay.toggle(crate::mm_save::MmDungeonItems::MAP)),
    },
    MmStoneTowerMap: MmMap {
        active: Box::new(|keys| keys.stone_tower.contains(crate::mm_save::MmDungeonItems::MAP)),
        toggle: Box::new(|keys| keys.stone_tower.toggle(crate::mm_save::MmDungeonItems::MAP)),
    },

    // ============================================================================
    // MM Dungeon Compasses
    // ============================================================================
    MmWoodfallCompass: MmCompass {
        active: Box::new(|keys| keys.woodfall.contains(crate::mm_save::MmDungeonItems::COMPASS)),
        toggle: Box::new(|keys| keys.woodfall.toggle(crate::mm_save::MmDungeonItems::COMPASS)),
    },
    MmSnowheadCompass: MmCompass {
        active: Box::new(|keys| keys.snowhead.contains(crate::mm_save::MmDungeonItems::COMPASS)),
        toggle: Box::new(|keys| keys.snowhead.toggle(crate::mm_save::MmDungeonItems::COMPASS)),
    },
    MmGreatBayCompass: MmCompass {
        active: Box::new(|keys| keys.great_bay.contains(crate::mm_save::MmDungeonItems::COMPASS)),
        toggle: Box::new(|keys| keys.great_bay.toggle(crate::mm_save::MmDungeonItems::COMPASS)),
    },
    MmStoneTowerCompass: MmCompass {
        active: Box::new(|keys| keys.stone_tower.contains(crate::mm_save::MmDungeonItems::COMPASS)),
        toggle: Box::new(|keys| keys.stone_tower.toggle(crate::mm_save::MmDungeonItems::COMPASS)),
    },

    BiggoronSword: Simple {
        img: ImageInfo::new("UNIMPLEMENTED"),
        active: Box::new(|state| state.ram.save.biggoron_sword && state.ram.save.equipment.contains(Equipment::GIANTS_KNIFE)),
        toggle: Box::new(|state| if state.ram.save.biggoron_sword && state.ram.save.equipment.contains(Equipment::GIANTS_KNIFE) {
            state.ram.save.biggoron_sword = false;
            state.ram.save.equipment.remove(Equipment::GIANTS_KNIFE);
        } else {
            state.ram.save.biggoron_sword = true;
            state.ram.save.equipment.insert(Equipment::GIANTS_KNIFE);
        }),
    },
    WalletNoTycoon: Sequence {
        idx: Box::new(|state| match state.ram.save.upgrades.wallet() {
            Upgrades::ADULTS_WALLET => 1,
            Upgrades::GIANTS_WALLET | Upgrades::TYCOONS_WALLET => 2,
            _ => 0,
        }),
        img: Box::new(|state| (state.ram.save.upgrades.wallet() != Upgrades::NONE, ImageInfo::new("UNIMPLEMENTED"))),
        increment: Box::new(|state| {
            let new_val = match state.ram.save.upgrades.wallet() {
                Upgrades::ADULTS_WALLET => Upgrades::GIANTS_WALLET,
                Upgrades::GIANTS_WALLET | Upgrades::TYCOONS_WALLET => Upgrades::NONE,
                _ => Upgrades::ADULTS_WALLET,
            };
            state.ram.save.upgrades.set_wallet(new_val);
        }),
        decrement: Box::new(|state| {
            let new_val = match state.ram.save.upgrades.wallet() {
                Upgrades::ADULTS_WALLET => Upgrades::NONE,
                Upgrades::GIANTS_WALLET | Upgrades::TYCOONS_WALLET => Upgrades::ADULTS_WALLET,
                _ => Upgrades::GIANTS_WALLET,
            };
            state.ram.save.upgrades.set_wallet(new_val);
        }),
    },
    StoneOfAgony: Simple {
        img: ImageInfo::new("UNIMPLEMENTED"),
        active: Box::new(|state| state.ram.save.quest_items.contains(QuestItems::STONE_OF_AGONY)),
        toggle: Box::new(|state| state.ram.save.quest_items.toggle(QuestItems::STONE_OF_AGONY)),
    },
    Blank: Simple {
        img: ImageInfo::extra("blank"),
        active: Box::new(|_| false),
        toggle: Box::new(|_| ()),
    },

    // ============================================================================
    // MM Items - Transformation Masks
    // ============================================================================
    MmDekuMask: Simple {
        img: ImageInfo::mm("deku_mask"),
        active: Box::new(|state| state.ram.mm_save.as_ref().is_some_and(|mm| mm.has_deku_mask())),
        toggle: Box::new(|_| ()),
    },
    MmGoronMask: Simple {
        img: ImageInfo::mm("goron_mask"),
        active: Box::new(|state| state.ram.mm_save.as_ref().is_some_and(|mm| mm.has_goron_mask())),
        toggle: Box::new(|_| ()),
    },
    MmZoraMask: Simple {
        img: ImageInfo::mm("zora_mask"),
        active: Box::new(|state| state.ram.mm_save.as_ref().is_some_and(|mm| mm.has_zora_mask())),
        toggle: Box::new(|_| ()),
    },
    MmFierceDeityMask: Simple {
        img: ImageInfo::mm("fierce_deity_mask"),
        active: Box::new(|state| state.ram.mm_save.as_ref().is_some_and(|mm| mm.has_fierce_deity_mask())),
        toggle: Box::new(|_| ()),
    },

    // ============================================================================
    // MM Items - Collectible Masks (24 unique)
    // ============================================================================
    MmPostmanHat: Simple {
        img: ImageInfo::mm("postman_hat"),
        active: Box::new(|state| state.ram.mm_save.as_ref().is_some_and(|save| save.has_postman_hat())),
        toggle: Box::new(|_| ()),
    },
    MmAllNightMask: Simple {
        img: ImageInfo::mm("all_night_mask"),
        active: Box::new(|state| state.ram.mm_save.as_ref().is_some_and(|save| save.has_all_night_mask())),
        toggle: Box::new(|_| ()),
    },
    MmBlastMask: Simple {
        img: ImageInfo::mm("blast_mask"),
        active: Box::new(|state| state.ram.mm_save.as_ref().is_some_and(|save| save.has_blast_mask())),
        toggle: Box::new(|_| ()),
    },
    MmStoneMask: Simple {
        img: ImageInfo::mm("stone_mask"),
        active: Box::new(|state| state.ram.mm_save.as_ref().is_some_and(|save| save.has_stone_mask())),
        toggle: Box::new(|_| ()),
    },
    MmGreatFairyMask: Simple {
        img: ImageInfo::mm("great_fairy_mask"),
        active: Box::new(|state| state.ram.mm_save.as_ref().is_some_and(|save| save.has_great_fairy_mask())),
        toggle: Box::new(|_| ()),
    },
    MmKeatonMask: Simple {
        img: ImageInfo::mm("keaton_mask"),
        active: Box::new(|state| state.ram.mm_save.as_ref().is_some_and(|save| save.has_keaton_mask())),
        toggle: Box::new(|_| ()),
    },
    MmBremenMask: Simple {
        img: ImageInfo::mm("bremen_mask"),
        active: Box::new(|state| state.ram.mm_save.as_ref().is_some_and(|save| save.has_bremen_mask())),
        toggle: Box::new(|_| ()),
    },
    MmBunnyHood: Simple {
        img: ImageInfo::mm("bunny_hood"),
        active: Box::new(|state| state.ram.mm_save.as_ref().is_some_and(|save| save.has_bunny_hood())),
        toggle: Box::new(|_| ()),
    },
    MmDonGeroMask: Simple {
        img: ImageInfo::mm("don_gero_mask"),
        active: Box::new(|state| state.ram.mm_save.as_ref().is_some_and(|save| save.has_don_gero_mask())),
        toggle: Box::new(|_| ()),
    },
    MmMaskOfScents: Simple {
        img: ImageInfo::mm("mask_of_scents"),
        active: Box::new(|state| state.ram.mm_save.as_ref().is_some_and(|save| save.has_mask_of_scents())),
        toggle: Box::new(|_| ()),
    },
    MmRomaniMask: Simple {
        img: ImageInfo::mm("romani_mask"),
        active: Box::new(|state| state.ram.mm_save.as_ref().is_some_and(|save| save.has_romani_mask())),
        toggle: Box::new(|_| ()),
    },
    MmCircusLeaderMask: Simple {
        img: ImageInfo::mm("circus_leader_mask"),
        active: Box::new(|state| state.ram.mm_save.as_ref().is_some_and(|save| save.has_circus_leader_mask())),
        toggle: Box::new(|_| ()),
    },
    MmKafeiMask: Simple {
        img: ImageInfo::mm("kafei_mask"),
        active: Box::new(|state| state.ram.mm_save.as_ref().is_some_and(|save| save.has_kafei_mask())),
        toggle: Box::new(|_| ()),
    },
    MmCouplesMask: Simple {
        img: ImageInfo::mm("couples_mask"),
        active: Box::new(|state| state.ram.mm_save.as_ref().is_some_and(|save| save.has_couples_mask())),
        toggle: Box::new(|_| ()),
    },
    MmMaskOfTruth: Simple {
        img: ImageInfo::mm("mask_of_truth"),
        active: Box::new(|state| state.ram.mm_save.as_ref().is_some_and(|save| save.has_mask_of_truth())),
        toggle: Box::new(|_| ()),
    },
    MmKamaroMask: Simple {
        img: ImageInfo::mm("kamaro_mask"),
        active: Box::new(|state| state.ram.mm_save.as_ref().is_some_and(|save| save.has_kamaro_mask())),
        toggle: Box::new(|_| ()),
    },
    MmGibdoMask: Simple {
        img: ImageInfo::mm("gibdo_mask"),
        active: Box::new(|state| state.ram.mm_save.as_ref().is_some_and(|save| save.has_gibdo_mask())),
        toggle: Box::new(|_| ()),
    },
    MmGaroMask: Simple {
        img: ImageInfo::mm("garo_mask"),
        active: Box::new(|state| state.ram.mm_save.as_ref().is_some_and(|save| save.has_garo_mask())),
        toggle: Box::new(|_| ()),
    },
    MmCaptainHat: Simple {
        img: ImageInfo::mm("captain_hat"),
        active: Box::new(|state| state.ram.mm_save.as_ref().is_some_and(|save| save.has_captain_hat())),
        toggle: Box::new(|_| ()),
    },
    MmGiantMask: Simple {
        img: ImageInfo::mm("giant_mask"),
        active: Box::new(|state| state.ram.mm_save.as_ref().is_some_and(|save| save.has_giant_mask())),
        toggle: Box::new(|_| ()),
    },

    // ============================================================================
    // MM Items - Boss Remains
    // ============================================================================
    MmOdolwaRemains: Simple {
        img: ImageInfo::mm("odolwa_remains"),
        active: Box::new(|state| state.ram.mm_save.as_ref().is_some_and(|save| save.has_odolwa_remains())),
        toggle: Box::new(|_| ()),
    },
    MmGohtRemains: Simple {
        img: ImageInfo::mm("goht_remains"),
        active: Box::new(|state| state.ram.mm_save.as_ref().is_some_and(|save| save.has_goht_remains())),
        toggle: Box::new(|_| ()),
    },
    MmGyorgRemains: Simple {
        img: ImageInfo::mm("gyorg_remains"),
        active: Box::new(|state| state.ram.mm_save.as_ref().is_some_and(|save| save.has_gyorg_remains())),
        toggle: Box::new(|_| ()),
    },
    MmTwinmoldRemains: Simple {
        img: ImageInfo::mm("twinmold_remains"),
        active: Box::new(|state| state.ram.mm_save.as_ref().is_some_and(|save| save.has_twinmold_remains())),
        toggle: Box::new(|_| ()),
    },

    // ============================================================================
    // MM Items - Stray Fairies (per dungeon)
    // ============================================================================
    MmStrayFairyWoodfall: Count {
        dimmed_img: ImageInfo::mm("stray_fairy_woodfall"),
        img: ImageInfo::mm("stray_fairy_woodfall"),
        get: Box::new(|state| state.ram.mm_save.as_ref().map_or(0, |mm| mm.stray_fairies.woodfall)),
        set: Box::new(|_, _| ()),
        max: 15,
        step: 1,
    },
    MmStrayFairySnowhead: Count {
        dimmed_img: ImageInfo::mm("stray_fairy_snowhead"),
        img: ImageInfo::mm("stray_fairy_snowhead"),
        get: Box::new(|state| state.ram.mm_save.as_ref().map_or(0, |mm| mm.stray_fairies.snowhead)),
        set: Box::new(|_, _| ()),
        max: 15,
        step: 1,
    },
    MmStrayFairyGreatBay: Count {
        dimmed_img: ImageInfo::mm("stray_fairy_great_bay"),
        img: ImageInfo::mm("stray_fairy_great_bay"),
        get: Box::new(|state| state.ram.mm_save.as_ref().map_or(0, |mm| mm.stray_fairies.great_bay)),
        set: Box::new(|_, _| ()),
        max: 15,
        step: 1,
    },
    MmStrayFairyStoneTower: Count {
        dimmed_img: ImageInfo::mm("stray_fairy_stone_tower"),
        img: ImageInfo::mm("stray_fairy_stone_tower"),
        get: Box::new(|state| state.ram.mm_save.as_ref().map_or(0, |mm| mm.stray_fairies.stone_tower)),
        set: Box::new(|_, _| ()),
        max: 15,
        step: 1,
    },
    MmStrayFairyClockTown: Simple {
        img: ImageInfo::mm("stray_fairy_clock_town"),
        active: Box::new(|state| state.ram.mm_save.as_ref().is_some_and(|mm| mm.stray_fairies.clock_town > 0)),
        toggle: Box::new(|_| ()),
    },

    // ============================================================================
    // MM Items - Songs
    // ============================================================================
    MmSongOfTime: Simple {
        img: ImageInfo::mm("song_of_time"),
        active: Box::new(|state| state.ram.mm_save.as_ref().is_some_and(|mm| mm.has_song_of_time())),
        toggle: Box::new(|_| ()),
    },
    MmSongOfHealing: Simple {
        img: ImageInfo::mm("song_of_healing"),
        active: Box::new(|state| state.ram.mm_save.as_ref().is_some_and(|mm| mm.has_song_of_healing())),
        toggle: Box::new(|_| ()),
    },
    MmEponasSong: Simple {
        img: ImageInfo::mm("eponas_song"),
        active: Box::new(|state| state.ram.mm_save.as_ref().is_some_and(|mm| mm.has_eponas_song())),
        toggle: Box::new(|_| ()),
    },
    MmSongOfSoaring: Simple {
        img: ImageInfo::mm("song_of_soaring"),
        active: Box::new(|state| state.ram.mm_save.as_ref().is_some_and(|mm| mm.has_song_of_soaring())),
        toggle: Box::new(|_| ()),
    },
    MmSongOfStorms: Simple {
        img: ImageInfo::mm("song_of_storms"),
        active: Box::new(|state| state.ram.mm_save.as_ref().is_some_and(|mm| mm.has_song_of_storms())),
        toggle: Box::new(|_| ()),
    },
    MmSonataOfAwakening: Simple {
        img: ImageInfo::mm("sonata_of_awakening"),
        active: Box::new(|state| state.ram.mm_save.as_ref().is_some_and(|mm| mm.has_sonata_of_awakening())),
        toggle: Box::new(|_| ()),
    },
    MmGoronLullaby: Simple {
        img: ImageInfo::mm("goron_lullaby"),
        active: Box::new(|state| state.ram.mm_save.as_ref().is_some_and(|mm| mm.has_goron_lullaby())),
        toggle: Box::new(|_| ()),
    },
    MmNewWaveBossaNova: Simple {
        img: ImageInfo::mm("new_wave_bossa_nova"),
        active: Box::new(|state| state.ram.mm_save.as_ref().is_some_and(|mm| mm.has_new_wave_bossa_nova())),
        toggle: Box::new(|_| ()),
    },
    MmElegyOfEmptiness: Simple {
        img: ImageInfo::mm("elegy_of_emptiness"),
        active: Box::new(|state| state.ram.mm_save.as_ref().is_some_and(|mm| mm.has_elegy_of_emptiness())),
        toggle: Box::new(|_| ()),
    },
    MmOathToOrder: Simple {
        img: ImageInfo::mm("oath_to_order"),
        active: Box::new(|state| state.ram.mm_save.as_ref().is_some_and(|mm| mm.has_oath_to_order())),
        toggle: Box::new(|_| ()),
    },

    // ============================================================================
    // MM Items - Bomber's Notebook
    // ============================================================================
    MmBomberNotebook: Simple {
        img: ImageInfo::mm("bomber_notebook"),
        active: Box::new(|state| state.ram.mm_save.as_ref().is_some_and(|save| save.has_bombers_notebook())),
        toggle: Box::new(|_| ()),
    },

    // ============================================================================
    // MM Items - Equipment
    // ============================================================================
    MmOcarina: Simple {
        img: ImageInfo::mm("ocarina"),
        active: Box::new(|state| state.ram.mm_save.as_ref().is_some_and(|save| save.has_ocarina())),
        toggle: Box::new(|_| ()),
    },
    MmHerosBow: Simple {
        img: ImageInfo::mm("heros_bow"),
        active: Box::new(|state| state.ram.mm_save.as_ref().is_some_and(|save| save.has_heros_bow())),
        toggle: Box::new(|_| ()),
    },
    MmFireArrow: Simple {
        img: ImageInfo::mm("fire_arrow"),
        active: Box::new(|state| state.ram.mm_save.as_ref().is_some_and(|save| save.has_fire_arrow())),
        toggle: Box::new(|_| ()),
    },
    MmIceArrow: Simple {
        img: ImageInfo::mm("ice_arrow"),
        active: Box::new(|state| state.ram.mm_save.as_ref().is_some_and(|save| save.has_ice_arrow())),
        toggle: Box::new(|_| ()),
    },
    MmLightArrow: Simple {
        img: ImageInfo::mm("light_arrow"),
        active: Box::new(|state| state.ram.mm_save.as_ref().is_some_and(|save| save.has_light_arrow())),
        toggle: Box::new(|_| ()),
    },
    MmHookshot: Simple {
        img: ImageInfo::mm("hookshot"),
        active: Box::new(|state| state.ram.mm_save.as_ref().is_some_and(|save| save.has_hookshot())),
        toggle: Box::new(|_| ()),
    },
    MmBombs: Simple {
        img: ImageInfo::mm("bombs"),
        active: Box::new(|state| state.ram.mm_save.as_ref().is_some_and(|save| save.has_bombs())),
        toggle: Box::new(|_| ()),
    },
    MmBombchu: Simple {
        img: ImageInfo::mm("bombchu"),
        active: Box::new(|state| state.ram.mm_save.as_ref().is_some_and(|save| save.has_bombchu())),
        toggle: Box::new(|_| ()),
    },
    MmPowderKeg: Simple {
        img: ImageInfo::mm("powder_keg"),
        active: Box::new(|state| state.ram.mm_save.as_ref().is_some_and(|save| save.has_powder_keg())),
        toggle: Box::new(|_| ()),
    },
    MmLensOfTruth: Simple {
        img: ImageInfo::mm("lens_of_truth"),
        active: Box::new(|state| state.ram.mm_save.as_ref().is_some_and(|save| save.has_lens_of_truth())),
        toggle: Box::new(|_| ()),
    },
    MmPictographBox: Simple {
        img: ImageInfo::mm("pictograph_box"),
        active: Box::new(|state| state.ram.mm_save.as_ref().is_some_and(|save| save.has_pictograph_box())),
        toggle: Box::new(|_| ()),
    },
    MmGreatFairySword: Simple {
        img: ImageInfo::mm("great_fairy_sword"),
        active: Box::new(|state| state.ram.mm_save.as_ref().is_some_and(|save| save.has_great_fairy_sword())),
        toggle: Box::new(|_| ()),
    },
    MmMagicBean: Simple {
        img: ImageInfo::mm("magic_bean"),
        active: Box::new(|state| state.ram.mm_save.as_ref().is_some_and(|save| save.has_magic_bean())),
        toggle: Box::new(|_| ()),
    },

    // ============================================================================
    // MM Items - Swords
    // ============================================================================
    MmSword: Sequence {
        idx: Box::new(|state| {
            state.ram.mm_save.as_ref().map_or(0, |mm| match mm.sword {
                crate::mm_save::MmSword::None => 0,
                crate::mm_save::MmSword::KokiriSword => 1,
                crate::mm_save::MmSword::RazorSword => 2,
                crate::mm_save::MmSword::GildedSword => 3,
            })
        }),
        img: Box::new(|state| {
            state.ram.mm_save.as_ref().map_or((false, ImageInfo::mm("kokiri_sword")), |mm| match mm.sword {
                crate::mm_save::MmSword::None => (false, ImageInfo::mm("kokiri_sword")),
                crate::mm_save::MmSword::KokiriSword => (true, ImageInfo::mm("kokiri_sword")),
                crate::mm_save::MmSword::RazorSword => (true, ImageInfo::mm("razor_sword")),
                crate::mm_save::MmSword::GildedSword => (true, ImageInfo::mm("gilded_sword")),
            })
        }),
        increment: Box::new(|state| {
            if let Some(mm) = state.ram.mm_save.as_mut() {
                mm.sword = match mm.sword {
                    crate::mm_save::MmSword::None => crate::mm_save::MmSword::KokiriSword,
                    crate::mm_save::MmSword::KokiriSword => crate::mm_save::MmSword::RazorSword,
                    crate::mm_save::MmSword::RazorSword => crate::mm_save::MmSword::GildedSword,
                    crate::mm_save::MmSword::GildedSword => crate::mm_save::MmSword::None,
                };
            }
        }),
        decrement: Box::new(|state| {
            if let Some(mm) = state.ram.mm_save.as_mut() {
                mm.sword = match mm.sword {
                    crate::mm_save::MmSword::None => crate::mm_save::MmSword::GildedSword,
                    crate::mm_save::MmSword::KokiriSword => crate::mm_save::MmSword::None,
                    crate::mm_save::MmSword::RazorSword => crate::mm_save::MmSword::KokiriSword,
                    crate::mm_save::MmSword::GildedSword => crate::mm_save::MmSword::RazorSword,
                };
            }
        }),
    },

    // ============================================================================
    // MM Items - Shields
    // ============================================================================
    MmShield: Sequence {
        idx: Box::new(|state| {
            state.ram.mm_save.as_ref().map_or(0, |mm| match mm.shield {
                crate::mm_save::MmShield::None => 0,
                crate::mm_save::MmShield::HeroShield => 1,
                crate::mm_save::MmShield::MirrorShield => 2,
            })
        }),
        img: Box::new(|state| {
            state.ram.mm_save.as_ref().map_or((false, ImageInfo::mm("hero_shield")), |mm| match mm.shield {
                crate::mm_save::MmShield::None => (false, ImageInfo::mm("hero_shield")),
                crate::mm_save::MmShield::HeroShield => (true, ImageInfo::mm("hero_shield")),
                crate::mm_save::MmShield::MirrorShield => (true, ImageInfo::mm("mirror_shield")),
            })
        }),
        increment: Box::new(|state| {
            if let Some(mm) = state.ram.mm_save.as_mut() {
                mm.shield = match mm.shield {
                    crate::mm_save::MmShield::None => crate::mm_save::MmShield::HeroShield,
                    crate::mm_save::MmShield::HeroShield => crate::mm_save::MmShield::MirrorShield,
                    crate::mm_save::MmShield::MirrorShield => crate::mm_save::MmShield::None,
                };
            }
        }),
        decrement: Box::new(|state| {
            if let Some(mm) = state.ram.mm_save.as_mut() {
                mm.shield = match mm.shield {
                    crate::mm_save::MmShield::None => crate::mm_save::MmShield::MirrorShield,
                    crate::mm_save::MmShield::HeroShield => crate::mm_save::MmShield::None,
                    crate::mm_save::MmShield::MirrorShield => crate::mm_save::MmShield::HeroShield,
                };
            }
        }),
    },

    // ============================================================================
    // MM Items - Bottles
    // ============================================================================
    MmBottle: Count {
        dimmed_img: ImageInfo::mm("bottle"),
        img: ImageInfo::mm("bottle"),
        get: Box::new(|state| {
            state.ram.mm_save.as_ref().map_or(0, |mm| {
                mm.inventory.bottles.iter().filter(|&&b| b != crate::mm_save::MmBottle::None).count() as u8
            })
        }),
        set: Box::new(|state, value| {
            if let Some(mm) = state.ram.mm_save.as_mut() {
                // Set bottles to Empty up to the value, then None for the rest
                for (i, bottle) in mm.inventory.bottles.iter_mut().enumerate() {
                    *bottle = if (i as u8) < value {
                        // Preserve existing bottle content, or set to Empty if was None
                        if *bottle == crate::mm_save::MmBottle::None {
                            crate::mm_save::MmBottle::Empty
                        } else {
                            *bottle
                        }
                    } else {
                        crate::mm_save::MmBottle::None
                    };
                }
            }
        }),
        max: 6,
        step: 1,
    },

    // ============================================================================
    // MM Items - Wallet/Upgrades
    // ============================================================================
    MmWallet: Sequence {
        idx: Box::new(|state| {
            state.ram.mm_save.as_ref().map_or(0, |mm| {
                let wallet = mm.upgrades.wallet();
                if wallet == crate::mm_save::MmUpgrades::GIANTS_WALLET {
                    2
                } else if wallet == crate::mm_save::MmUpgrades::ADULTS_WALLET {
                    1
                } else {
                    0
                }
            })
        }),
        img: Box::new(|state| {
            state.ram.mm_save.as_ref().map_or((false, ImageInfo::mm("wallet")), |mm| {
                let wallet = mm.upgrades.wallet();
                if wallet == crate::mm_save::MmUpgrades::GIANTS_WALLET {
                    (true, ImageInfo::mm("giants_wallet"))
                } else if wallet == crate::mm_save::MmUpgrades::ADULTS_WALLET {
                    (true, ImageInfo::mm("adults_wallet"))
                } else {
                    (false, ImageInfo::mm("wallet"))
                }
            })
        }),
        increment: Box::new(|state| {
            if let Some(mm) = state.ram.mm_save.as_mut() {
                let new_val = {
                    let wallet = mm.upgrades.wallet();
                    if wallet == crate::mm_save::MmUpgrades::ADULTS_WALLET {
                        crate::mm_save::MmUpgrades::GIANTS_WALLET
                    } else if wallet == crate::mm_save::MmUpgrades::GIANTS_WALLET {
                        crate::mm_save::MmUpgrades::empty()
                    } else {
                        crate::mm_save::MmUpgrades::ADULTS_WALLET
                    }
                };
                mm.upgrades.set_wallet(new_val);
            }
        }),
        decrement: Box::new(|state| {
            if let Some(mm) = state.ram.mm_save.as_mut() {
                let new_val = {
                    let wallet = mm.upgrades.wallet();
                    if wallet == crate::mm_save::MmUpgrades::ADULTS_WALLET {
                        crate::mm_save::MmUpgrades::empty()
                    } else if wallet == crate::mm_save::MmUpgrades::GIANTS_WALLET {
                        crate::mm_save::MmUpgrades::ADULTS_WALLET
                    } else {
                        crate::mm_save::MmUpgrades::GIANTS_WALLET
                    }
                };
                mm.upgrades.set_wallet(new_val);
            }
        }),
    },
    MmMagic: Simple {
        img: ImageInfo::mm("magic"),
        active: Box::new(|state| state.ram.mm_save.as_ref().is_some_and(|save| save.has_magic())),
        toggle: Box::new(|_| ()),
    },
    MmDoubleDefense: Simple {
        img: ImageInfo::mm("double_defense"),
        active: Box::new(|state| {
            state.ram.mm_save.as_ref().is_some_and(|mm| mm.double_defense)
        }),
        toggle: Box::new(|state| {
            if let Some(mm) = state.ram.mm_save.as_mut() {
                mm.double_defense = !mm.double_defense;
            }
        }),
    },

    // ============================================================================
    // Heart Count Cells
    // ============================================================================

    // OoT heart containers count (3-20)
    OotHearts: Count {
        dimmed_img: ImageInfo::extra("heart_container"),
        img: ImageInfo::extra("heart_container"),
        get: Box::new(|state| state.ram.save.heart_containers()),
        set: Box::new(|state, value| {
            // Set health_capacity based on heart containers (each heart = 0x10)
            state.ram.save.health_capacity = (value as u16) * 0x10;
        }),
        max: 20,
        step: 1,
    },

    // OoT heart pieces count (0-3)
    OotHeartPieces: Count {
        dimmed_img: ImageInfo::extra("heart_piece"),
        img: ImageInfo::extra("heart_piece"),
        get: Box::new(|state| state.ram.save.heart_pieces),
        set: Box::new(|state, value| {
            state.ram.save.heart_pieces = value.min(3);
        }),
        max: 3,
        step: 1,
    },

    // MM heart containers count (3-20)
    MmHearts: Count {
        dimmed_img: ImageInfo::extra("heart_container"),
        img: ImageInfo::extra("heart_container"),
        get: Box::new(|state| {
            state.ram.mm_save.as_ref().map_or(0, |mm| mm.heart_containers())
        }),
        set: Box::new(|state, value| {
            if let Some(mm) = state.ram.mm_save.as_mut() {
                mm.health_capacity = (value as u16) * 0x10;
            }
        }),
        max: 20,
        step: 1,
    },

    // MM heart pieces count (0-3)
    MmHeartPieces: Count {
        dimmed_img: ImageInfo::extra("heart_piece"),
        img: ImageInfo::extra("heart_piece"),
        get: Box::new(|state| {
            state.ram.mm_save.as_ref().map_or(0, |mm| mm.quest_items.heart_pieces())
        }),
        set: Box::new(|_state, _value| {
            // Heart pieces in MM are stored in quest_items bitflags - complex to set
        }),
        max: 3,
        step: 1,
    },

    // ============================================================================
    // MM Items - Dungeon Keys
    // ============================================================================
    MmWoodfallSmallKeys: TrackerCellKind::MmSmallKeys {
        get: Box::new(|keys| keys.woodfall),
        set: Box::new(|keys, value| keys.woodfall = value),
        max: 1,
        label: "WF",
    },
    MmSnowheadSmallKeys: TrackerCellKind::MmSmallKeys {
        get: Box::new(|keys| keys.snowhead()),
        set: Box::new(|keys, value| keys.snowhead = value),
        max: 3,
        label: "SH",
    },
    MmGreatBaySmallKeys: TrackerCellKind::MmSmallKeys {
        get: Box::new(|keys| keys.great_bay()),
        set: Box::new(|keys, value| keys.great_bay = value),
        max: 1,
        label: "GB",
    },
    MmStoneTowerSmallKeys: TrackerCellKind::MmSmallKeys {
        get: Box::new(|keys| keys.stone_tower()),
        set: Box::new(|keys, value| keys.stone_tower = value),
        max: 4,
        label: "ST",
    },

    // ============================================================================
    // MM Items - Item Sharing Indicators (OoTMM combo rando)
    // ============================================================================
    MmSharedOcarina: Overlay {
        main_img: ImageInfo::mm("ocarina"),
        overlay_img: ImageInfo::extra("oot_badge"),
        active: Box::new(|state| (
            state.ram.mm_save.as_ref().is_some_and(|save| save.has_ocarina()),
            state.ram.save.inv.ocarina != Ocarina::None,
        )),
        toggle_main: Box::new(|_| ()),
        toggle_overlay: Box::new(|_| ()),
    },
    MmSharedHookshot: Overlay {
        main_img: ImageInfo::mm("hookshot"),
        overlay_img: ImageInfo::extra("oot_badge"),
        active: Box::new(|state| (
            state.ram.mm_save.as_ref().is_some_and(|save| save.has_hookshot()),
            state.ram.save.inv.hookshot != Hookshot::None,
        )),
        toggle_main: Box::new(|_| ()),
        toggle_overlay: Box::new(|_| ()),
    },
    MmSharedBow: Overlay {
        main_img: ImageInfo::mm("heros_bow"),
        overlay_img: ImageInfo::extra("oot_badge"),
        active: Box::new(|state| (
            state.ram.mm_save.as_ref().is_some_and(|save| save.has_heros_bow()),
            state.ram.save.inv.bow,
        )),
        toggle_main: Box::new(|_| ()),
        toggle_overlay: Box::new(|_| ()),
    },
    MmSharedBombs: Overlay {
        main_img: ImageInfo::mm("bombs"),
        overlay_img: ImageInfo::extra("oot_badge"),
        active: Box::new(|state| (
            state.ram.mm_save.as_ref().is_some_and(|save| save.has_bombs()),
            state.ram.save.upgrades.bomb_bag() != Upgrades::NONE,
        )),
        toggle_main: Box::new(|_| ()),
        toggle_overlay: Box::new(|_| ()),
    },
    MmSharedMagic: Overlay {
        main_img: ImageInfo::mm("magic"),
        overlay_img: ImageInfo::extra("oot_badge"),
        active: Box::new(|state| (
            state.ram.mm_save.as_ref().is_some_and(|save| save.has_magic()),
            state.ram.save.magic != MagicCapacity::None,
        )),
        toggle_main: Box::new(|_| ()),
        toggle_overlay: Box::new(|_| ()),
    },
    MmSharedLens: Overlay {
        main_img: ImageInfo::mm("lens_of_truth"),
        overlay_img: ImageInfo::extra("oot_badge"),
        active: Box::new(|state| (
            state.ram.mm_save.as_ref().is_some_and(|save| save.has_lens_of_truth()),
            state.ram.save.inv.lens,
        )),
        toggle_main: Box::new(|_| ()),
        toggle_overlay: Box::new(|_| ()),
    },
    MmSharedWallet: Overlay {
        main_img: ImageInfo::mm("wallet"),
        overlay_img: ImageInfo::extra("oot_badge"),
        active: Box::new(|state| (
            state.ram.mm_save.as_ref().is_some_and(|save| save.upgrades.wallet() != crate::mm_save::MmUpgrades::empty()),
            state.ram.save.upgrades.wallet() != Upgrades::NONE,
        )),
        toggle_main: Box::new(|_| ()),
        toggle_overlay: Box::new(|_| ()),
    },
}

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

            // ============================================================================
            // MM Layouts
            // ============================================================================
            Self::MmDefault => {
                // Default MM layout showing key items
                // Row 1: Boss Remains (4)
                // Row 2: Transformation Masks (4)
                // Row 3: Equipment (6)
                // Row 4: Songs (6)
                // Row 5: Stray Fairies (5)
                columns!(
                    6,
                    [
                        // Row 1: Boss Remains
                        MmOdolwaRemains,
                        MmGohtRemains,
                        MmGyorgRemains,
                        MmTwinmoldRemains,
                        MmBomberNotebook,
                        MmBottle,
                        // Row 2: Transformation Masks + Upgrades
                        MmDekuMask,
                        MmGoronMask,
                        MmZoraMask,
                        MmFierceDeityMask,
                        MmMagic,
                        MmDoubleDefense,
                        // Row 3: Equipment
                        MmOcarina,
                        MmHerosBow,
                        MmHookshot,
                        MmBombs,
                        MmPowderKeg,
                        MmLensOfTruth,
                        // Row 4: Songs (first 6)
                        MmSongOfTime,
                        MmSongOfHealing,
                        MmEponasSong,
                        MmSongOfSoaring,
                        MmSongOfStorms,
                        MmSonataOfAwakening,
                        // Row 5: More Songs + Fairies
                        MmGoronLullaby,
                        MmNewWaveBossaNova,
                        MmElegyOfEmptiness,
                        MmOathToOrder,
                        MmStrayFairyClockTown,
                        Blank,
                        // Row 6: Stray Fairies by dungeon
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
                // Grid layout for all masks
                // 4 Transformation + 20 Regular = 24 total
                // 6x4 grid
                columns!(
                    6,
                    [
                        // Row 1: Transformation Masks
                        MmDekuMask,
                        MmGoronMask,
                        MmZoraMask,
                        MmFierceDeityMask,
                        Blank,
                        Blank,
                        // Row 2: Regular Masks 1-6
                        MmPostmanHat,
                        MmAllNightMask,
                        MmBlastMask,
                        MmStoneMask,
                        MmGreatFairyMask,
                        MmKeatonMask,
                        // Row 3: Regular Masks 7-12
                        MmBremenMask,
                        MmBunnyHood,
                        MmDonGeroMask,
                        MmMaskOfScents,
                        MmRomaniMask,
                        MmCircusLeaderMask,
                        // Row 4: Regular Masks 13-18
                        MmKafeiMask,
                        MmCouplesMask,
                        MmMaskOfTruth,
                        MmKamaroMask,
                        MmGibdoMask,
                        MmGaroMask,
                        // Row 5: Regular Masks 19-20
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
                // Compact boss remains display
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
                // Stray fairy counters per dungeon
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
                // All MM songs in a grid
                columns!(
                    5,
                    [
                        // Row 1: Shared songs
                        MmSongOfTime,
                        MmSongOfHealing,
                        MmEponasSong,
                        MmSongOfSoaring,
                        MmSongOfStorms,
                        // Row 2: MM-specific songs
                        MmSonataOfAwakening,
                        MmGoronLullaby,
                        MmNewWaveBossaNova,
                        MmElegyOfEmptiness,
                        MmOathToOrder,
                    ]
                )
            }

            Self::MmEquipment => {
                // MM equipment and items
                columns!(
                    6,
                    [
                        // Row 1: Main equipment
                        MmOcarina,
                        MmHerosBow,
                        MmHookshot,
                        MmBombs,
                        MmBombchu,
                        MmPowderKeg,
                        // Row 2: More equipment
                        MmLensOfTruth,
                        MmPictographBox,
                        MmGreatFairySword,
                        MmMagicBean,
                        MmSword,
                        MmShield,
                        // Row 3: Arrows + Upgrades
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
                // OoT dungeon items: maps, compasses, boss keys, and small keys
                // 6 columns, organized by dungeon
                columns!(
                    6,
                    [
                        // Row 1: Child dungeon maps
                        DekuMap,
                        DcMap,
                        JabuMap,
                        ForestMap,
                        FireMap,
                        WaterMap,
                        // Row 2: Adult dungeon maps
                        ShadowMap,
                        SpiritMap,
                        WellMap,
                        IceMap,
                        GanonMap,
                        Blank,
                        // Row 3: Child dungeon compasses
                        DekuCompass,
                        DcCompass,
                        JabuCompass,
                        ForestCompass,
                        FireCompass,
                        WaterCompass,
                        // Row 4: Adult dungeon compasses
                        ShadowCompass,
                        SpiritCompass,
                        WellCompass,
                        IceCompass,
                        Blank,
                        Blank,
                        // Row 5: Boss Keys (dungeons that have them)
                        ForestBossKey,
                        FireBossKey,
                        WaterBossKey,
                        ShadowBossKey,
                        SpiritBossKey,
                        GanonBossKey,
                        // Row 6: Small Keys (main temples + Well)
                        ForestSmallKeys,
                        FireSmallKeys,
                        WaterSmallKeys,
                        ShadowSmallKeys,
                        SpiritSmallKeys,
                        WellSmallKeys,
                        // Row 7: Small Keys (other dungeons)
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
                // MM dungeon items: maps, compasses, small keys, boss keys
                // 4 columns (one per dungeon)
                columns!(
                    4,
                    [
                        // Row 1: Maps
                        MmWoodfallMap,
                        MmSnowheadMap,
                        MmGreatBayMap,
                        MmStoneTowerMap,
                        // Row 2: Compasses
                        MmWoodfallCompass,
                        MmSnowheadCompass,
                        MmGreatBayCompass,
                        MmStoneTowerCompass,
                        // Row 3: Small Keys
                        MmWoodfallSmallKeys,
                        MmSnowheadSmallKeys,
                        MmGreatBaySmallKeys,
                        MmStoneTowerSmallKeys,
                        // Row 4: Boss Keys
                        MmWoodfallBossKey,
                        MmSnowheadBossKey,
                        MmGreatBayBossKey,
                        MmStoneTowerBossKey,
                    ]
                )
            }

            Self::Combo => {
                // Combo layout for OoTMM randomizer
                // Shows important items from both OoT and MM in a unified view
                // Layout: 12 columns wide to accommodate both games' items
                //
                // Row 1: OoT Medallions (6) + MM Boss Remains (4) + OoT Stones (2 placeholder)
                // Row 2: OoT Stone locations/stones (3) + MM Transformation Masks (4) + Core shared items
                // Row 3: OoT Core equipment
                // Row 4: OoT Songs (6) + MM Songs (6)
                // Row 5: MM Equipment + OoT Trade/Progressive items
                // Row 6: Additional items from both games

                columns!(
                    12,
                    [
                        // Row 1: OoT Dungeon Rewards + MM Boss Remains
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
                        // Row 2: OoT Stone + MM Transformation Masks + Shared Equipment
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
                        // Row 3: OoT Core Equipment
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
                        // Row 4: OoT Songs + MM Songs
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
                        // Row 5: OoT Warp Songs + MM Dungeon Songs
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
                        // Row 6: MM Equipment + OoT Trade Items
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
                        // Row 7: OoT Remaining Items + MM Remaining Items
                        ChildTrade,
                        Ocarina,
                        Beans,
                        SwordCard,
                        Tunics,
                        AdultTradeNoChicken,
                        MmGreatFairySword,
                        MmPictographBox,
                        MmMagicBean,
                        // Row 8: MM Collectible Masks (1-12)
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
                        // Row 9: MM Collectible Masks (13-20) + Heart Count Display
                        MmKafeiMask,
                        MmCouplesMask,
                        MmMaskOfTruth,
                        MmKamaroMask,
                        MmGibdoMask,
                        MmGaroMask,
                        MmCaptainHat,
                        MmGiantMask,
                        // Heart count display: OoT hearts + pieces, MM hearts + pieces
                        OotHearts,
                        OotHeartPieces,
                        MmHearts,
                        MmHeartPieces,
                        // Row 10: OoT Dungeon Maps
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
                        // Row 11: OoT Dungeon Compasses
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
                        // Row 12: OoT Boss Keys
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
                        // Row 13: OoT Small Keys
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
                        // Row 14: MM Dungeon Items - Boss Keys + Maps
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
                        // Row 15: MM Dungeon Items - Small Keys + Stray Fairies
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
    ///
    /// Columns are computed from the maximum x-position of cells,
    /// assuming cells are placed on a 60px grid starting at x=5.
    pub fn column_count(&self) -> usize {
        let cells = self.cells();
        if cells.is_empty() {
            return 0;
        }
        // Cells are on a 60px grid, starting at x=5
        // Column index = (x - 5) / 60, so column_count = max_column_index + 1
        cells
            .iter()
            .map(|c| ((c.pos[0].saturating_sub(5)) / 60) as usize + 1)
            .max()
            .unwrap_or(0)
    }

    /// Returns the number of rows in this layout based on cell positions.
    ///
    /// Rows are counted as distinct y-position bands where cells are placed.
    pub fn row_count(&self) -> usize {
        let cells = self.cells();
        if cells.is_empty() {
            return 0;
        }
        // Count distinct y positions (rows may not be evenly spaced)
        let mut y_positions: Vec<u16> = cells.iter().map(|c| c.pos[1]).collect();
        y_positions.sort_unstable();
        y_positions.dedup();
        y_positions.len()
    }

    /// Returns the pixel dimensions (width, height) of this layout.
    ///
    /// The dimensions include a 5px padding on the right and bottom edges.
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
        // Add 5px padding on right and bottom to match left/top padding
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
            Self::Default { .. } => unimplemented!(), //TODO
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
            //TODO parse Default variant with custom fields
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
///
/// Used in the web app for more compact dungeon reward layouts on restreams.
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Protocol)]
pub enum CellStyle {
    Normal,
    Dimmed,
    LeftDimmed,
    RightDimmed,
}

#[cfg(feature = "rocket")]
impl CellStyle {
    fn css_class(&self) -> &'static str {
        match self {
            Self::Normal => "",
            Self::Dimmed => "dimmed",
            Self::LeftDimmed => "left-dimmed",
            Self::RightDimmed => "right-dimmed",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Protocol)]
pub enum CellOverlay {
    None,
    Count {
        count: u8,
        max: u8,
        count_img: ImageInfo,
    },
    Image(ImageInfo),
    Location {
        loc: ImageInfo,
        style: LocationStyle,
    },
    /// Count with maximum displayed as "count/max" (e.g., "2/4" for bottles)
    CountWithMax {
        count: u8,
        max: u8,
        count_img: ImageInfo,
    },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Protocol)]
pub enum LocationStyle {
    Normal,
    Dimmed,
    Mq,
}

#[cfg(feature = "rocket")]
impl LocationStyle {
    fn css_classes(&self) -> &'static str {
        match self {
            Self::Normal => "loc",
            Self::Dimmed => "loc dimmed",
            Self::Mq => "loc mq",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Protocol)]
pub struct CellRender {
    pub img: ImageInfo,
    pub style: CellStyle,
    pub overlay: CellOverlay,
    /// Optional accessibility status for the cell.
    /// When set, adds a visual border indicator showing if the location is accessible.
    pub accessibility: Option<AccessibilityStatus>,
    /// Optional text label for the cell (e.g., dungeon abbreviation for keys).
    pub label: Option<String>,
}

impl CellRender {
    /// Creates a new CellRender without accessibility information or label.
    pub fn new(img: ImageInfo, style: CellStyle, overlay: CellOverlay) -> Self {
        Self {
            img,
            style,
            overlay,
            accessibility: None,
            label: None,
        }
    }

    /// Sets the accessibility status for this cell.
    pub fn with_accessibility(mut self, status: AccessibilityStatus) -> Self {
        self.accessibility = Some(status);
        self
    }

    /// Sets the text label for this cell.
    pub fn with_label(mut self, label: impl Into<String>) -> Self {
        self.label = Some(label.into());
        self
    }

    /// Returns the combined CSS classes for style and accessibility.
    #[cfg(feature = "rocket")]
    fn combined_css_classes(&self) -> String {
        let style_class = self.style.css_class();
        match self.accessibility {
            Some(status) => {
                if style_class.is_empty() {
                    format!("accessibility-{}", status.css_class())
                } else {
                    format!("{} accessibility-{}", style_class, status.css_class())
                }
            }
            None => style_class.to_string(),
        }
    }
}

#[cfg(feature = "rocket")]
impl ToHtml for CellRender {
    fn to_html(&self) -> RawHtml<String> {
        html! {
            img(class = self.combined_css_classes(), src = format!("/static/img/{}.png", self.img.to_string('/', ImageDirContext::Normal)));
            @match self.overlay {
                CellOverlay::None => ;
                CellOverlay::Count { count, max: 0, .. } => span(class = "count") : count;
                CellOverlay::Count { count, max, .. } => span(class = "count") : format!("{} / {}", count, max);
                CellOverlay::Image(ref overlay) => img(src = format!("/static/img/{}.png", overlay.to_string('/', ImageDirContext::OverlayOnly)));
                CellOverlay::Location { ref loc, style } => img(class = style.css_classes(), src = format!("/static/img/{}.png", loc.to_string('/', ImageDirContext::Normal)));
                CellOverlay::CountWithMax { count, max, .. } => span(class = "count") : format!("{}/{}", count, max);
            }
            @if let Some(ref label) = self.label {
                span(class = "key-label") : label;
            }
        }
    }
}

fn default_med_order() -> ElementOrder {
    ElementOrder::LightShadowSpirit
}
fn default_warp_song_order() -> ElementOrder {
    ElementOrder::SpiritShadowLight
}

pub fn dirs() -> Result<ProjectDirs, Error> {
    ProjectDirs::from("net", "Fenhl", "OoT Tracker").ok_or(Error::MissingHomeDir)
}

pub enum ImageDirContext {
    Normal,
    Count(u8),
    Dimmed,
    OverlayOnly,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Protocol)]
pub enum ImageDir {
    Xopar,
    Extra,
    Mm,
}

impl ImageDir {
    pub fn to_string(&self, ctx: ImageDirContext) -> &'static str {
        match (self, ctx) {
            (ImageDir::Xopar, ImageDirContext::Normal) => "xopar-images",
            (ImageDir::Extra, ImageDirContext::Normal) => "extra-images",
            (ImageDir::Mm, ImageDirContext::Normal) => "mm-images",
            (ImageDir::Xopar, ImageDirContext::Count(_)) => "xopar-images-count",
            (ImageDir::Extra, ImageDirContext::Count(_)) => "extra-images-count",
            (ImageDir::Mm, ImageDirContext::Count(_)) => "mm-images-count",
            (ImageDir::Xopar, ImageDirContext::Dimmed) => "xopar-images-dimmed",
            (ImageDir::Extra, ImageDirContext::Dimmed) => "extra-images-dimmed",
            (ImageDir::Mm, ImageDirContext::Dimmed) => "mm-images-dimmed",
            (ImageDir::Xopar, ImageDirContext::OverlayOnly) => "xopar-overlays",
            (ImageDir::Extra, ImageDirContext::OverlayOnly) => "extra-overlays",
            (ImageDir::Mm, ImageDirContext::OverlayOnly) => "mm-overlays",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Protocol)]
pub struct ImageInfo {
    pub dir: ImageDir,
    pub name: Cow<'static, str>,
}

impl ImageInfo {
    pub fn new(name: impl Into<Cow<'static, str>>) -> ImageInfo {
        ImageInfo {
            dir: ImageDir::Xopar,
            name: name.into(),
        }
    }

    pub fn extra(name: impl Into<Cow<'static, str>>) -> ImageInfo {
        ImageInfo {
            dir: ImageDir::Extra,
            name: name.into(),
        }
    }

    pub fn mm(name: impl Into<Cow<'static, str>>) -> ImageInfo {
        ImageInfo {
            dir: ImageDir::Mm,
            name: name.into(),
        }
    }

    #[cfg(feature = "embed-images")]
    pub fn embedded<T: FromEmbeddedImage>(&self, ctx: ImageDirContext) -> T {
        match (self.dir, ctx) {
            (ImageDir::Xopar, ImageDirContext::Normal) => images::xopar_images(&self.name),
            (ImageDir::Extra, ImageDirContext::Normal) => images::extra_images(&self.name),
            // MM images fall back to extra images until MM assets are added
            (ImageDir::Mm, ImageDirContext::Normal) => images::extra_images(&self.name),
            (ImageDir::Xopar, ImageDirContext::Count(count)) => {
                images::xopar_images_count(&format!("{}_{}", self.name, count))
            }
            (ImageDir::Extra, ImageDirContext::Count(count)) => {
                images::extra_images_count(&format!("{}_{}", self.name, count))
            }
            (ImageDir::Mm, ImageDirContext::Count(count)) => {
                images::extra_images_count(&format!("{}_{}", self.name, count))
            }
            (ImageDir::Xopar, ImageDirContext::Dimmed) => images::xopar_images_dimmed(&self.name),
            (ImageDir::Extra, ImageDirContext::Dimmed) => images::extra_images_dimmed(&self.name),
            (ImageDir::Mm, ImageDirContext::Dimmed) => images::extra_images_dimmed(&self.name),
            (ImageDir::Xopar, ImageDirContext::OverlayOnly) => images::xopar_overlays(&self.name),
            (ImageDir::Extra, ImageDirContext::OverlayOnly) => images::extra_overlays(&self.name),
            (ImageDir::Mm, ImageDirContext::OverlayOnly) => images::extra_overlays(&self.name),
        }
    }

    pub fn to_string(&self, sep: char, ctx: ImageDirContext) -> String {
        format!("{}{}{}", self.dir.to_string(ctx), sep, self.name)
    }

    pub fn with_overlay(&self, overlay: &ImageInfo) -> OverlayImageInfo {
        OverlayImageInfo {
            dir: if self.dir == ImageDir::Xopar && overlay.dir == ImageDir::Xopar {
                ImageDir::Xopar
            } else {
                ImageDir::Extra
            },
            main: self.name.clone(),
            overlay: overlay.name.clone(),
        }
    }
}

pub struct OverlayImageInfo {
    dir: ImageDir,
    main: Cow<'static, str>,
    overlay: Cow<'static, str>,
}

impl OverlayImageInfo {
    #[cfg(feature = "embed-images")]
    pub fn embedded<T: FromEmbeddedImage>(&self, main_active: bool) -> T {
        (match (self.dir, main_active) {
            (ImageDir::Xopar, false) => images::xopar_images_overlay_dimmed,
            (ImageDir::Xopar, true) => images::xopar_images_overlay,
            (ImageDir::Extra | ImageDir::Mm, false) => images::extra_images_overlay_dimmed,
            (ImageDir::Extra | ImageDir::Mm, true) => images::extra_images_overlay,
        })(&format!("{}_{}", self.main, self.overlay))
    }

    pub fn to_string(&self, sep: char, main_active: bool) -> String {
        format!(
            "{}-images-overlay{}{}{}_{}",
            match self.dir {
                ImageDir::Xopar => "xopar",
                ImageDir::Extra | ImageDir::Mm => "extra",
            },
            if main_active { "" } else { "-dimmed" },
            sep,
            self.main,
            self.overlay,
        )
    }
}

pub trait FromEmbeddedImage {
    fn from_embedded_image(contents: &'static [u8]) -> Self;
}

#[cfg(feature = "iced")]
impl FromEmbeddedImage for iced::widget::Image {
    fn from_embedded_image(contents: &'static [u8]) -> iced::widget::Image {
        iced::widget::Image::new(iced::widget::image::Handle::from_memory(contents.to_vec()))
    }
}

impl FromEmbeddedImage for DynamicImage {
    fn from_embedded_image(contents: &'static [u8]) -> DynamicImage {
        image::load_from_memory(contents).expect("failed to load embedded DynamicImage")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ModelState;
    use ootr::model::Medallion;

    // ==========================================================================
    // AccessibilityStatus Tests
    // ==========================================================================

    #[test]
    fn test_accessibility_status_default() {
        let status = AccessibilityStatus::default();
        assert_eq!(status, AccessibilityStatus::Unknown);
    }

    #[test]
    fn test_accessibility_status_icon() {
        assert_eq!(AccessibilityStatus::Accessible.icon(), "✓");
        assert_eq!(AccessibilityStatus::Inaccessible.icon(), "✗");
        assert_eq!(AccessibilityStatus::Checked.icon(), "●");
        assert_eq!(AccessibilityStatus::Unknown.icon(), "?");
    }

    #[test]
    fn test_accessibility_status_description() {
        assert_eq!(AccessibilityStatus::Accessible.description(), "Accessible");
        assert_eq!(
            AccessibilityStatus::Inaccessible.description(),
            "Not yet accessible"
        );
        assert_eq!(
            AccessibilityStatus::Checked.description(),
            "Already checked"
        );
        assert_eq!(AccessibilityStatus::Unknown.description(), "Unknown");
    }

    #[test]
    fn test_accessibility_status_from_check_status() {
        use crate::checks::CheckStatus;

        assert_eq!(
            AccessibilityStatus::from(CheckStatus::Checked),
            AccessibilityStatus::Checked
        );
        assert_eq!(
            AccessibilityStatus::from(CheckStatus::Reachable),
            AccessibilityStatus::Accessible
        );
        assert_eq!(
            AccessibilityStatus::from(CheckStatus::NotYetReachable),
            AccessibilityStatus::Inaccessible
        );
    }

    // ==========================================================================
    // AccessibilitySummary Tests
    // ==========================================================================

    #[test]
    fn test_accessibility_summary_new() {
        let summary = AccessibilitySummary::new();
        assert_eq!(summary.accessible, 0);
        assert_eq!(summary.inaccessible, 0);
        assert_eq!(summary.checked, 0);
        assert_eq!(summary.unknown, 0);
    }

    #[test]
    fn test_accessibility_summary_default() {
        let summary = AccessibilitySummary::default();
        assert_eq!(summary.total(), 0);
    }

    #[test]
    fn test_accessibility_summary_add() {
        let mut summary = AccessibilitySummary::new();

        summary.add(AccessibilityStatus::Accessible);
        assert_eq!(summary.accessible, 1);
        assert_eq!(summary.total(), 1);

        summary.add(AccessibilityStatus::Inaccessible);
        assert_eq!(summary.inaccessible, 1);
        assert_eq!(summary.total(), 2);

        summary.add(AccessibilityStatus::Checked);
        assert_eq!(summary.checked, 1);
        assert_eq!(summary.total(), 3);

        summary.add(AccessibilityStatus::Unknown);
        assert_eq!(summary.unknown, 1);
        assert_eq!(summary.total(), 4);
    }

    #[test]
    fn test_accessibility_summary_total() {
        let mut summary = AccessibilitySummary::new();
        summary.accessible = 5;
        summary.inaccessible = 3;
        summary.checked = 2;
        summary.unknown = 1;
        assert_eq!(summary.total(), 11);
    }

    #[test]
    fn test_accessibility_summary_display() {
        let mut summary = AccessibilitySummary::new();
        summary.accessible = 5;
        summary.inaccessible = 3;
        summary.checked = 2;
        summary.unknown = 1;
        let display = format!("{}", summary);
        assert_eq!(display, "✓5 / ✗3 / ●2 / ?1");
    }

    // ==========================================================================
    // LocationAccessibility Tests
    // ==========================================================================

    #[test]
    fn test_location_accessibility_new() {
        let loc = LocationAccessibility::new("Test Location", AccessibilityStatus::Accessible);
        assert_eq!(loc.name, "Test Location");
        assert_eq!(loc.status, AccessibilityStatus::Accessible);
    }

    #[test]
    fn test_location_accessibility_new_with_string() {
        let name = String::from("Dynamic Name");
        let loc = LocationAccessibility::new(name, AccessibilityStatus::Checked);
        assert_eq!(loc.name, "Dynamic Name");
        assert_eq!(loc.status, AccessibilityStatus::Checked);
    }

    // ==========================================================================
    // LocationAccessibilityList Tests
    // ==========================================================================

    #[test]
    fn test_location_accessibility_list_new() {
        let list = LocationAccessibilityList::new();
        assert!(list.locations.is_empty());
        assert_eq!(list.summary.total(), 0);
    }

    #[test]
    fn test_location_accessibility_list_default() {
        let list = LocationAccessibilityList::default();
        assert!(list.locations.is_empty());
    }

    #[test]
    fn test_location_accessibility_list_add() {
        let mut list = LocationAccessibilityList::new();

        list.add("Location 1", AccessibilityStatus::Accessible);
        list.add("Location 2", AccessibilityStatus::Inaccessible);
        list.add("Location 3", AccessibilityStatus::Accessible);

        assert_eq!(list.locations.len(), 3);
        assert_eq!(list.summary.accessible, 2);
        assert_eq!(list.summary.inaccessible, 1);
    }

    #[test]
    fn test_location_accessibility_list_accessible_filter() {
        let mut list = LocationAccessibilityList::new();
        list.add("Accessible 1", AccessibilityStatus::Accessible);
        list.add("Inaccessible 1", AccessibilityStatus::Inaccessible);
        list.add("Accessible 2", AccessibilityStatus::Accessible);

        let accessible: Vec<_> = list.accessible().collect();
        assert_eq!(accessible.len(), 2);
        assert_eq!(accessible[0].name, "Accessible 1");
        assert_eq!(accessible[1].name, "Accessible 2");
    }

    #[test]
    fn test_location_accessibility_list_inaccessible_filter() {
        let mut list = LocationAccessibilityList::new();
        list.add("Accessible 1", AccessibilityStatus::Accessible);
        list.add("Inaccessible 1", AccessibilityStatus::Inaccessible);
        list.add("Inaccessible 2", AccessibilityStatus::Inaccessible);

        let inaccessible: Vec<_> = list.inaccessible().collect();
        assert_eq!(inaccessible.len(), 2);
        assert_eq!(inaccessible[0].name, "Inaccessible 1");
        assert_eq!(inaccessible[1].name, "Inaccessible 2");
    }

    // ==========================================================================
    // CellStyle Tests
    // ==========================================================================

    #[test]
    fn test_cell_style_equality() {
        assert_eq!(CellStyle::Normal, CellStyle::Normal);
        assert_eq!(CellStyle::Dimmed, CellStyle::Dimmed);
        assert_eq!(CellStyle::LeftDimmed, CellStyle::LeftDimmed);
        assert_eq!(CellStyle::RightDimmed, CellStyle::RightDimmed);
        assert_ne!(CellStyle::Normal, CellStyle::Dimmed);
    }

    #[test]
    fn test_cell_style_clone() {
        let style = CellStyle::LeftDimmed;
        let cloned = style;
        assert_eq!(style, cloned);
    }

    // ==========================================================================
    // CellOverlay Tests
    // ==========================================================================

    #[test]
    fn test_cell_overlay_none() {
        let overlay = CellOverlay::None;
        assert_eq!(overlay, CellOverlay::None);
    }

    #[test]
    fn test_cell_overlay_count() {
        let overlay = CellOverlay::Count {
            count: 5,
            max: 100,
            count_img: ImageInfo::new("skulls"),
        };
        if let CellOverlay::Count { count, max, .. } = overlay {
            assert_eq!(count, 5);
            assert_eq!(max, 100);
        } else {
            panic!("Expected Count variant");
        }
    }

    #[test]
    fn test_cell_overlay_image() {
        let overlay = CellOverlay::Image(ImageInfo::new("lens"));
        if let CellOverlay::Image(img) = &overlay {
            assert_eq!(img.name, "lens");
        } else {
            panic!("Expected Image variant");
        }
    }

    #[test]
    fn test_cell_overlay_location() {
        let overlay = CellOverlay::Location {
            loc: ImageInfo::new("forest_text"),
            style: LocationStyle::Normal,
        };
        if let CellOverlay::Location { style, .. } = overlay {
            assert_eq!(style, LocationStyle::Normal);
        } else {
            panic!("Expected Location variant");
        }
    }

    // ==========================================================================
    // LocationStyle Tests
    // ==========================================================================

    #[test]
    fn test_location_style_equality() {
        assert_eq!(LocationStyle::Normal, LocationStyle::Normal);
        assert_eq!(LocationStyle::Dimmed, LocationStyle::Dimmed);
        assert_eq!(LocationStyle::Mq, LocationStyle::Mq);
        assert_ne!(LocationStyle::Normal, LocationStyle::Mq);
    }

    // ==========================================================================
    // CellRender Tests
    // ==========================================================================

    #[test]
    fn test_cell_render_new() {
        let render = CellRender::new(ImageInfo::new("test"), CellStyle::Normal, CellOverlay::None);
        assert_eq!(render.style, CellStyle::Normal);
        assert_eq!(render.overlay, CellOverlay::None);
        assert!(render.accessibility.is_none());
    }

    #[test]
    fn test_cell_render_with_accessibility() {
        let render = CellRender::new(ImageInfo::new("test"), CellStyle::Normal, CellOverlay::None)
            .with_accessibility(AccessibilityStatus::Accessible);

        assert_eq!(render.accessibility, Some(AccessibilityStatus::Accessible));
    }

    #[test]
    fn test_cell_render_equality() {
        let render1 = CellRender::new(ImageInfo::new("test"), CellStyle::Normal, CellOverlay::None);
        let render2 = CellRender::new(ImageInfo::new("test"), CellStyle::Normal, CellOverlay::None);
        assert_eq!(render1, render2);
    }

    #[test]
    fn test_cell_render_inequality_style() {
        let render1 = CellRender::new(ImageInfo::new("test"), CellStyle::Normal, CellOverlay::None);
        let render2 = CellRender::new(ImageInfo::new("test"), CellStyle::Dimmed, CellOverlay::None);
        assert_ne!(render1, render2);
    }

    // ==========================================================================
    // ImageInfo Tests
    // ==========================================================================

    #[test]
    fn test_image_info_new() {
        let img = ImageInfo::new("sword");
        assert_eq!(img.dir, ImageDir::Xopar);
        assert_eq!(img.name, "sword");
    }

    #[test]
    fn test_image_info_extra() {
        let img = ImageInfo::extra("boss_key");
        assert_eq!(img.dir, ImageDir::Extra);
        assert_eq!(img.name, "boss_key");
    }

    #[test]
    fn test_image_info_mm() {
        let img = ImageInfo::mm("mask");
        assert_eq!(img.dir, ImageDir::Mm);
        assert_eq!(img.name, "mask");
    }

    #[test]
    fn test_image_info_equality() {
        let img1 = ImageInfo::new("sword");
        let img2 = ImageInfo::new("sword");
        assert_eq!(img1, img2);
    }

    #[test]
    fn test_image_info_inequality_name() {
        let img1 = ImageInfo::new("sword");
        let img2 = ImageInfo::new("shield");
        assert_ne!(img1, img2);
    }

    #[test]
    fn test_image_info_inequality_dir() {
        let img1 = ImageInfo::new("item");
        let img2 = ImageInfo::extra("item");
        assert_ne!(img1, img2);
    }

    // ==========================================================================
    // ImageDir Tests
    // ==========================================================================

    #[test]
    fn test_image_dir_to_string_normal() {
        assert_eq!(
            ImageDir::Xopar.to_string(ImageDirContext::Normal),
            "xopar-images"
        );
        assert_eq!(
            ImageDir::Extra.to_string(ImageDirContext::Normal),
            "extra-images"
        );
        assert_eq!(ImageDir::Mm.to_string(ImageDirContext::Normal), "mm-images");
    }

    #[test]
    fn test_image_dir_to_string_dimmed() {
        assert_eq!(
            ImageDir::Xopar.to_string(ImageDirContext::Dimmed),
            "xopar-images-dimmed"
        );
        assert_eq!(
            ImageDir::Extra.to_string(ImageDirContext::Dimmed),
            "extra-images-dimmed"
        );
        assert_eq!(
            ImageDir::Mm.to_string(ImageDirContext::Dimmed),
            "mm-images-dimmed"
        );
    }

    #[test]
    fn test_image_dir_to_string_count() {
        assert_eq!(
            ImageDir::Xopar.to_string(ImageDirContext::Count(5)),
            "xopar-images-count"
        );
        assert_eq!(
            ImageDir::Extra.to_string(ImageDirContext::Count(10)),
            "extra-images-count"
        );
    }

    #[test]
    fn test_image_dir_to_string_overlay_only() {
        assert_eq!(
            ImageDir::Xopar.to_string(ImageDirContext::OverlayOnly),
            "xopar-overlays"
        );
        assert_eq!(
            ImageDir::Extra.to_string(ImageDirContext::OverlayOnly),
            "extra-overlays"
        );
        assert_eq!(
            ImageDir::Mm.to_string(ImageDirContext::OverlayOnly),
            "mm-overlays"
        );
    }

    // ==========================================================================
    // ElementOrder Tests
    // ==========================================================================

    #[test]
    fn test_element_order_light_shadow_spirit() {
        let order = ElementOrder::LightShadowSpirit;
        let meds: Vec<_> = order.into_iter().collect();
        assert_eq!(meds.len(), 6);
        assert_eq!(meds[0], Medallion::Light);
        assert_eq!(meds[4], Medallion::Shadow);
        assert_eq!(meds[5], Medallion::Spirit);
    }

    #[test]
    fn test_element_order_light_spirit_shadow() {
        let order = ElementOrder::LightSpiritShadow;
        let meds: Vec<_> = order.into_iter().collect();
        assert_eq!(meds[0], Medallion::Light);
        assert_eq!(meds[4], Medallion::Spirit);
        assert_eq!(meds[5], Medallion::Shadow);
    }

    #[test]
    fn test_element_order_shadow_spirit_light() {
        let order = ElementOrder::ShadowSpiritLight;
        let meds: Vec<_> = order.into_iter().collect();
        assert_eq!(meds[0], Medallion::Forest);
        assert_eq!(meds[3], Medallion::Shadow);
        assert_eq!(meds[5], Medallion::Light);
    }

    #[test]
    fn test_element_order_spirit_shadow_light() {
        let order = ElementOrder::SpiritShadowLight;
        let meds: Vec<_> = order.into_iter().collect();
        assert_eq!(meds[0], Medallion::Forest);
        assert_eq!(meds[3], Medallion::Spirit);
        assert_eq!(meds[5], Medallion::Light);
    }

    #[test]
    fn test_element_order_display() {
        assert_eq!(
            format!("{}", ElementOrder::LightShadowSpirit),
            "Light first, Shadow before Spirit"
        );
        assert_eq!(
            format!("{}", ElementOrder::LightSpiritShadow),
            "Light first, Spirit before Shadow"
        );
        assert_eq!(
            format!("{}", ElementOrder::ShadowSpiritLight),
            "Shadow before Spirit, Light last"
        );
        assert_eq!(
            format!("{}", ElementOrder::SpiritShadowLight),
            "Spirit before Shadow, Light last"
        );
    }

    // ==========================================================================
    // LayoutPreference Tests
    // ==========================================================================

    #[test]
    fn test_layout_preference_default() {
        let pref = LayoutPreference::default();
        assert_eq!(pref, LayoutPreference::Oot);
    }

    #[test]
    fn test_layout_preference_display() {
        assert_eq!(format!("{}", LayoutPreference::Oot), "Ocarina of Time");
        assert_eq!(format!("{}", LayoutPreference::Mm), "Majora's Mask");
        assert_eq!(format!("{}", LayoutPreference::Combo), "Combo (OoT + MM)");
        assert_eq!(
            format!("{}", LayoutPreference::DungeonItems),
            "OoT Dungeon Items"
        );
        assert_eq!(
            format!("{}", LayoutPreference::MmDungeonItems),
            "MM Dungeon Items"
        );
        assert_eq!(
            format!("{}", LayoutPreference::MmStrayFairies),
            "MM Stray Fairies"
        );
    }

    // ==========================================================================
    // TrackerCellId Tests
    // ==========================================================================

    #[test]
    fn test_tracker_cell_id_from_medallion() {
        assert_eq!(
            TrackerCellId::from(Medallion::Light),
            TrackerCellId::LightMedallion
        );
        assert_eq!(
            TrackerCellId::from(Medallion::Forest),
            TrackerCellId::ForestMedallion
        );
        assert_eq!(
            TrackerCellId::from(Medallion::Fire),
            TrackerCellId::FireMedallion
        );
        assert_eq!(
            TrackerCellId::from(Medallion::Water),
            TrackerCellId::WaterMedallion
        );
        assert_eq!(
            TrackerCellId::from(Medallion::Shadow),
            TrackerCellId::ShadowMedallion
        );
        assert_eq!(
            TrackerCellId::from(Medallion::Spirit),
            TrackerCellId::SpiritMedallion
        );
    }

    #[test]
    fn test_tracker_cell_id_med_location() {
        assert_eq!(
            TrackerCellId::med_location(Medallion::Light),
            TrackerCellId::LightMedallionLocation
        );
        assert_eq!(
            TrackerCellId::med_location(Medallion::Forest),
            TrackerCellId::ForestMedallionLocation
        );
    }

    #[test]
    fn test_tracker_cell_id_warp_song() {
        assert_eq!(
            TrackerCellId::warp_song(Medallion::Light),
            TrackerCellId::Prelude
        );
        assert_eq!(
            TrackerCellId::warp_song(Medallion::Forest),
            TrackerCellId::Minuet
        );
        assert_eq!(
            TrackerCellId::warp_song(Medallion::Fire),
            TrackerCellId::Bolero
        );
        assert_eq!(
            TrackerCellId::warp_song(Medallion::Water),
            TrackerCellId::Serenade
        );
        assert_eq!(
            TrackerCellId::warp_song(Medallion::Shadow),
            TrackerCellId::Nocturne
        );
        assert_eq!(
            TrackerCellId::warp_song(Medallion::Spirit),
            TrackerCellId::Requiem
        );
    }

    // ==========================================================================
    // TrackerCellKind Render Tests - Cell Rendering Logic
    // ==========================================================================

    #[test]
    fn test_medallion_render_inactive() {
        let state = ModelState::default();
        let cell = TrackerCellId::ForestMedallion.kind();
        let render = cell.render(&state);

        assert_eq!(render.style, CellStyle::Dimmed);
        assert_eq!(render.overlay, CellOverlay::None);
        assert!(render.img.name.contains("forest"));
    }

    #[test]
    fn test_medallion_render_active() {
        let mut state = ModelState::default();
        state
            .ram
            .save
            .quest_items
            .toggle(QuestItems::FOREST_MEDALLION);

        let cell = TrackerCellId::ForestMedallion.kind();
        let render = cell.render(&state);

        assert_eq!(render.style, CellStyle::Normal);
    }

    #[test]
    fn test_stone_render_inactive() {
        let state = ModelState::default();
        let cell = TrackerCellId::KokiriEmerald.kind();
        let render = cell.render(&state);

        assert_eq!(render.style, CellStyle::Dimmed);
    }

    #[test]
    fn test_stone_render_active() {
        let mut state = ModelState::default();
        state
            .ram
            .save
            .quest_items
            .toggle(QuestItems::KOKIRI_EMERALD);

        let cell = TrackerCellId::KokiriEmerald.kind();
        let render = cell.render(&state);

        assert_eq!(render.style, CellStyle::Normal);
    }

    #[test]
    fn test_count_cell_render_zero() {
        let state = ModelState::default();
        let cell = TrackerCellId::Skulltula.kind();
        let render = cell.render(&state);

        assert_eq!(render.style, CellStyle::Dimmed);
        assert_eq!(render.overlay, CellOverlay::None);
    }

    #[test]
    fn test_count_cell_render_nonzero() {
        let mut state = ModelState::default();
        state.ram.save.skull_tokens = 25;

        let cell = TrackerCellId::Skulltula.kind();
        let render = cell.render(&state);

        assert_eq!(render.style, CellStyle::Normal);
        if let CellOverlay::CountWithMax { count, max, .. } = render.overlay {
            assert_eq!(count, 25);
            assert_eq!(max, 100);
        } else {
            panic!("Expected CountWithMax overlay");
        }
    }

    #[test]
    fn test_count_with_max_overlay_construction() {
        let overlay = CellOverlay::CountWithMax {
            count: 2,
            max: 4,
            count_img: ImageInfo::new("bottle"),
        };
        if let CellOverlay::CountWithMax { count, max, .. } = overlay {
            assert_eq!(count, 2);
            assert_eq!(max, 4);
        } else {
            panic!("Expected CountWithMax variant");
        }
    }

    #[test]
    fn test_num_bottles_cell_render_zero() {
        let state = ModelState::default();
        let cell = TrackerCellId::NumBottles.kind();
        let render = cell.render(&state);

        assert_eq!(render.style, CellStyle::Dimmed);
        assert_eq!(render.overlay, CellOverlay::None);
    }

    #[test]
    fn test_num_bottles_cell_render_nonzero() {
        let mut state = ModelState::default();
        state.ram.save.inv.set_emptiable_bottles(2);

        let cell = TrackerCellId::NumBottles.kind();
        let render = cell.render(&state);

        assert_eq!(render.style, CellStyle::Normal);
        if let CellOverlay::CountWithMax { count, max, .. } = render.overlay {
            assert_eq!(count, 2);
            assert_eq!(max, 4);
        } else {
            panic!("Expected CountWithMax overlay for NumBottles");
        }
    }

    #[test]
    fn test_num_bottles_cell_render_max() {
        let mut state = ModelState::default();
        state.ram.save.inv.set_emptiable_bottles(4);

        let cell = TrackerCellId::NumBottles.kind();
        let render = cell.render(&state);

        assert_eq!(render.style, CellStyle::Normal);
        if let CellOverlay::CountWithMax { count, max, .. } = render.overlay {
            assert_eq!(count, 4);
            assert_eq!(max, 4);
        } else {
            panic!("Expected CountWithMax overlay for NumBottles at max");
        }
    }

    #[test]
    fn test_go_mode_render_normal() {
        let state = ModelState::default();
        let cell = TrackerCellId::GoMode.kind();
        let render = cell.render(&state);

        // Default progression mode is Normal, so GoMode should be dimmed
        assert_eq!(render.style, CellStyle::Dimmed);
    }

    #[test]
    fn test_go_mode_render_active() {
        let mut state = ModelState::default();
        state.knowledge.progression_mode = ProgressionMode::Go;

        let cell = TrackerCellId::GoMode.kind();
        let render = cell.render(&state);

        assert_eq!(render.style, CellStyle::Normal);
    }

    #[test]
    fn test_magic_lens_render_no_magic() {
        let state = ModelState::default();
        let cell = TrackerCellId::MagicLens.kind();
        let render = cell.render(&state);

        assert_eq!(render.style, CellStyle::Dimmed);
        assert_eq!(render.overlay, CellOverlay::None);
    }

    #[test]
    fn test_magic_lens_render_with_magic() {
        let mut state = ModelState::default();
        state.ram.save.magic = MagicCapacity::Small;

        let cell = TrackerCellId::MagicLens.kind();
        let render = cell.render(&state);

        assert_eq!(render.style, CellStyle::Normal);
    }

    #[test]
    fn test_magic_lens_render_with_lens() {
        let mut state = ModelState::default();
        state.ram.save.magic = MagicCapacity::Small;
        state.ram.save.inv.lens = true;

        let cell = TrackerCellId::MagicLens.kind();
        let render = cell.render(&state);

        if let CellOverlay::Image(img) = &render.overlay {
            assert_eq!(img.name, "lens");
        } else {
            panic!("Expected Image overlay with lens");
        }
    }

    #[test]
    fn test_spells_render_inactive() {
        let state = ModelState::default();
        let cell = TrackerCellId::Spells.kind();
        let render = cell.render(&state);

        assert_eq!(render.style, CellStyle::Dimmed);
    }

    #[test]
    fn test_spells_render_dins_only() {
        let mut state = ModelState::default();
        state.ram.save.inv.dins_fire = true;

        let cell = TrackerCellId::Spells.kind();
        let render = cell.render(&state);

        // With only Din's Fire, left part is active
        assert_eq!(render.style, CellStyle::Normal);
    }

    // ==========================================================================
    // TrackerCellKind Click Tests - State Transitions
    // ==========================================================================

    #[test]
    fn test_medallion_click_toggles() {
        let mut state = ModelState::default();
        let cell = TrackerCellId::ForestMedallion.kind();

        // Should not have medallion initially
        assert!(!state
            .ram
            .save
            .quest_items
            .contains(QuestItems::FOREST_MEDALLION));

        // Click to toggle on
        cell.click(&mut state);
        assert!(state
            .ram
            .save
            .quest_items
            .contains(QuestItems::FOREST_MEDALLION));

        // Click to toggle off
        cell.click(&mut state);
        assert!(!state
            .ram
            .save
            .quest_items
            .contains(QuestItems::FOREST_MEDALLION));
    }

    #[test]
    fn test_stone_click_toggles() {
        let mut state = ModelState::default();
        let cell = TrackerCellId::KokiriEmerald.kind();

        assert!(!state
            .ram
            .save
            .quest_items
            .contains(QuestItems::KOKIRI_EMERALD));

        cell.click(&mut state);
        assert!(state
            .ram
            .save
            .quest_items
            .contains(QuestItems::KOKIRI_EMERALD));

        cell.click(&mut state);
        assert!(!state
            .ram
            .save
            .quest_items
            .contains(QuestItems::KOKIRI_EMERALD));
    }

    #[test]
    fn test_count_click_increments() {
        let mut state = ModelState::default();
        let cell = TrackerCellId::Skulltula.kind();

        assert_eq!(state.ram.save.skull_tokens, 0);

        cell.click(&mut state);
        assert_eq!(state.ram.save.skull_tokens, 1);

        cell.click(&mut state);
        assert_eq!(state.ram.save.skull_tokens, 2);
    }

    #[test]
    fn test_count_click_wraps_at_max() {
        let mut state = ModelState::default();
        state.ram.save.skull_tokens = 100;

        let cell = TrackerCellId::Skulltula.kind();
        cell.click(&mut state);

        assert_eq!(state.ram.save.skull_tokens, 0);
    }

    #[test]
    fn test_go_mode_click_toggles() {
        let mut state = ModelState::default();
        let cell = TrackerCellId::GoMode.kind();

        assert_eq!(state.knowledge.progression_mode, ProgressionMode::Normal);

        cell.click(&mut state);
        assert_eq!(state.knowledge.progression_mode, ProgressionMode::Go);

        cell.click(&mut state);
        assert_eq!(state.knowledge.progression_mode, ProgressionMode::Normal);
    }

    #[test]
    fn test_go_bk_click_cycles() {
        let mut state = ModelState::default();
        let cell = TrackerCellId::GoBk.kind();

        assert_eq!(state.knowledge.progression_mode, ProgressionMode::Normal);

        cell.click(&mut state);
        assert_eq!(state.knowledge.progression_mode, ProgressionMode::Go);

        cell.click(&mut state);
        assert_eq!(state.knowledge.progression_mode, ProgressionMode::Bk);

        cell.click(&mut state);
        assert_eq!(state.knowledge.progression_mode, ProgressionMode::Done);

        cell.click(&mut state);
        assert_eq!(state.knowledge.progression_mode, ProgressionMode::Normal);
    }

    #[test]
    fn test_magic_lens_click_cycles() {
        let mut state = ModelState::default();
        let cell = TrackerCellId::MagicLens.kind();

        // Start with no magic
        assert_eq!(state.ram.save.magic, MagicCapacity::None);
        assert!(!state.ram.save.inv.lens);

        // First click: gain small magic
        cell.click(&mut state);
        assert_eq!(state.ram.save.magic, MagicCapacity::Small);
        assert!(!state.ram.save.inv.lens);

        // Second click: lose magic, gain lens
        cell.click(&mut state);
        assert_eq!(state.ram.save.magic, MagicCapacity::None);
        assert!(state.ram.save.inv.lens);

        // Third click: gain magic again
        cell.click(&mut state);
        assert_eq!(state.ram.save.magic, MagicCapacity::Small);
        assert!(state.ram.save.inv.lens);
    }

    #[test]
    fn test_spells_click_cycles() {
        let mut state = ModelState::default();
        let cell = TrackerCellId::Spells.kind();

        // Start with nothing
        assert!(!state.ram.save.inv.dins_fire);
        assert!(!state.ram.save.inv.farores_wind);

        // First click: gain Din's Fire
        cell.click(&mut state);
        assert!(state.ram.save.inv.dins_fire);
        assert!(!state.ram.save.inv.farores_wind);

        // Second click: lose Din's, gain Farore's
        cell.click(&mut state);
        assert!(!state.ram.save.inv.dins_fire);
        assert!(state.ram.save.inv.farores_wind);

        // Third click: gain Din's
        cell.click(&mut state);
        assert!(state.ram.save.inv.dins_fire);
        assert!(state.ram.save.inv.farores_wind);
    }

    #[test]
    fn test_sequence_cell_increment() {
        let mut state = ModelState::default();
        let cell = TrackerCellId::AdultTrade.kind();

        // Start with no trade item
        assert_eq!(state.ram.save.inv.adult_trade_item, AdultTradeItem::None);

        // Click to increment
        cell.click(&mut state);
        assert_eq!(
            state.ram.save.inv.adult_trade_item,
            AdultTradeItem::PocketEgg
        );

        cell.click(&mut state);
        assert_eq!(
            state.ram.save.inv.adult_trade_item,
            AdultTradeItem::PocketCucco
        );
    }

    #[test]
    fn test_small_keys_click_increments() {
        let mut state = ModelState::default();
        let cell = TrackerCellId::ForestSmallKeys.kind();

        let initial = state.ram.save.small_keys.forest_temple;
        cell.click(&mut state);
        assert_eq!(state.ram.save.small_keys.forest_temple, initial + 1);
    }

    #[test]
    fn test_small_keys_click_wraps() {
        let mut state = ModelState::default();
        // Forest Temple has max 6 keys (MQ has 6, vanilla has 5)
        state.ram.save.small_keys.forest_temple = 6;

        let cell = TrackerCellId::ForestSmallKeys.kind();
        cell.click(&mut state);

        assert_eq!(state.ram.save.small_keys.forest_temple, 0);
    }

    // ==========================================================================
    // DungeonRewardLocationExt Tests
    // ==========================================================================

    #[test]
    fn test_dungeon_reward_location_increment() {
        use std::collections::HashMap;

        let mut locations: HashMap<DungeonReward, DungeonRewardLocation> = HashMap::new();
        let reward = DungeonReward::Medallion(Medallion::Forest);

        // Start with no location
        assert!(!locations.contains_key(&reward));

        // Increment to first dungeon
        locations.increment(reward);
        assert_eq!(
            locations.get(&reward),
            Some(&DungeonRewardLocation::Dungeon(MainDungeon::DekuTree))
        );

        // Continue incrementing
        locations.increment(reward);
        assert_eq!(
            locations.get(&reward),
            Some(&DungeonRewardLocation::Dungeon(MainDungeon::DodongosCavern))
        );
    }

    #[test]
    fn test_dungeon_reward_location_decrement() {
        use std::collections::HashMap;

        let mut locations: HashMap<DungeonReward, DungeonRewardLocation> = HashMap::new();
        let reward = DungeonReward::Stone(Stone::KokiriEmerald);

        // Start with no location, decrement wraps to LinksPocket
        locations.decrement(reward);
        assert_eq!(
            locations.get(&reward),
            Some(&DungeonRewardLocation::LinksPocket)
        );

        // Decrement again
        locations.decrement(reward);
        assert_eq!(
            locations.get(&reward),
            Some(&DungeonRewardLocation::Dungeon(MainDungeon::SpiritTemple))
        );
    }

    #[test]
    fn test_dungeon_reward_location_full_cycle() {
        use std::collections::HashMap;

        let mut locations: HashMap<DungeonReward, DungeonRewardLocation> = HashMap::new();
        let reward = DungeonReward::Medallion(Medallion::Light);

        // Cycle through all locations
        locations.increment(reward); // DekuTree
        locations.increment(reward); // DodongosCavern
        locations.increment(reward); // JabuJabu
        locations.increment(reward); // ForestTemple
        locations.increment(reward); // FireTemple
        locations.increment(reward); // WaterTemple
        locations.increment(reward); // ShadowTemple
        locations.increment(reward); // SpiritTemple
        locations.increment(reward); // LinksPocket
        assert_eq!(
            locations.get(&reward),
            Some(&DungeonRewardLocation::LinksPocket)
        );

        // One more increment removes the entry
        locations.increment(reward);
        assert!(!locations.contains_key(&reward));
    }

    // ==========================================================================
    // Config Tests
    // ==========================================================================

    #[test]
    fn test_config_default() {
        let config = Config::default();
        assert_eq!(config.med_order, ElementOrder::LightShadowSpirit);
        assert_eq!(config.warp_song_order, ElementOrder::SpiritShadowLight);
        assert_eq!(config.layout_preference, LayoutPreference::Oot);
        assert_eq!(config.version, VERSION);
    }

    // ==========================================================================
    // TrackerLayout Tests
    // ==========================================================================

    #[test]
    fn test_tracker_layout_default() {
        let layout = TrackerLayout::default();
        // Default layout should exist
        assert!(matches!(layout, TrackerLayout::Default { .. }));
    }

    #[test]
    fn test_tracker_layout_from_config() {
        let config = Config::default();
        let layout = TrackerLayout::from(&config);
        assert!(matches!(layout, TrackerLayout::Default { .. }));
    }

    #[test]
    fn test_tracker_layout_column_count_default() {
        let layout = TrackerLayout::default();
        // Default layout has 6 columns
        assert_eq!(layout.column_count(), 6);
    }

    #[test]
    fn test_tracker_layout_column_count_various() {
        // MultiworldExpanded uses 4 columns
        assert_eq!(TrackerLayout::MultiworldExpanded.column_count(), 4);
        // MultiworldCollapsed uses 10 columns
        assert_eq!(TrackerLayout::MultiworldCollapsed.column_count(), 10);
        // TriforcePieces uses 1 column
        assert_eq!(TrackerLayout::TriforcePieces.column_count(), 1);
        // Combo uses 12 columns
        assert_eq!(TrackerLayout::Combo.column_count(), 12);
    }

    #[test]
    fn test_tracker_layout_row_count_default() {
        let layout = TrackerLayout::default();
        // Default layout has multiple rows with varying positions
        let row_count = layout.row_count();
        assert!(row_count > 0, "Default layout should have rows");
    }

    #[test]
    fn test_tracker_layout_row_count_various() {
        // TriforcePieces has 1 row
        assert_eq!(TrackerLayout::TriforcePieces.row_count(), 1);
        // MmBossRemains has 1 row (4 cells in 4 columns)
        assert_eq!(TrackerLayout::MmBossRemains.row_count(), 1);
        // MmSongs has 2 rows (10 cells in 5 columns)
        assert_eq!(TrackerLayout::MmSongs.row_count(), 2);
    }

    #[test]
    fn test_tracker_layout_pixel_dimensions_default() {
        let layout = TrackerLayout::default();
        let (width, height) = layout.pixel_dimensions();
        // Default layout is 6 columns wide (6 * 60 = 360)
        assert_eq!(width, 360);
        // Height depends on number of rows and their positioning
        assert!(height > 0, "Default layout should have non-zero height");
    }

    #[test]
    fn test_tracker_layout_pixel_dimensions_various() {
        // TriforcePieces: 1 column (1 * 60 = 60), 1 row
        let (width, height) = TrackerLayout::TriforcePieces.pixel_dimensions();
        assert_eq!(width, 60);
        assert_eq!(height, 60);

        // MultiworldExpanded: 4 columns (4 * 60 = 240)
        let (width, _) = TrackerLayout::MultiworldExpanded.pixel_dimensions();
        assert_eq!(width, 240);

        // Combo: 12 columns (12 * 60 = 720)
        let (width, _) = TrackerLayout::Combo.pixel_dimensions();
        assert_eq!(width, 720);
    }

    #[test]
    fn test_tracker_layout_dimensions_consistency() {
        // Test that all layout variants have valid dimensions
        let layouts = [
            TrackerLayout::default(),
            TrackerLayout::MultiworldExpanded,
            TrackerLayout::MultiworldCollapsed,
            TrackerLayout::MultiworldEdit,
            TrackerLayout::RslLeft,
            TrackerLayout::RslRight,
            TrackerLayout::RslEdit,
            TrackerLayout::Rsl3Player,
            TrackerLayout::TsgMainWithRewardLocations,
            TrackerLayout::TsgMainWithRewardLocationsEdit,
            TrackerLayout::TriforcePieces,
            TrackerLayout::MmDefault,
            TrackerLayout::MmMasks,
            TrackerLayout::MmBossRemains,
            TrackerLayout::MmStrayFairies,
            TrackerLayout::MmSongs,
            TrackerLayout::MmEquipment,
            TrackerLayout::Combo,
        ];

        for layout in layouts {
            let cols = layout.column_count();
            let rows = layout.row_count();
            let (width, height) = layout.pixel_dimensions();

            assert!(cols > 0, "Layout should have at least one column");
            assert!(rows > 0, "Layout should have at least one row");
            assert!(width > 0, "Layout should have non-zero width");
            assert!(height > 0, "Layout should have non-zero height");

            // Width should be columns * 60 (grid spacing)
            assert_eq!(
                width,
                (cols * 60) as u32,
                "Width should match column count * 60"
            );
        }
    }

    // ==========================================================================
    // Cell Visibility Condition Tests
    // ==========================================================================

    #[test]
    fn test_composite_keys_visibility_both_inactive() {
        let state = ModelState::default();
        let cell = TrackerCellId::ForestKeys.kind();
        let render = cell.render(&state);

        // No boss key and no small keys should be dimmed
        assert_eq!(render.style, CellStyle::Dimmed);
    }

    #[test]
    fn test_composite_keys_visibility_small_keys_only() {
        let mut state = ModelState::default();
        state.ram.save.small_keys.forest_temple = 3;

        let cell = TrackerCellId::ForestKeys.kind();
        let render = cell.render(&state);

        // Small keys but no boss key: left dimmed (boss key side)
        assert_eq!(render.style, CellStyle::LeftDimmed);
    }

    #[test]
    fn test_composite_keys_visibility_boss_key_only() {
        let mut state = ModelState::default();
        state
            .ram
            .save
            .dungeon_items
            .forest_temple
            .insert(DungeonItems::BOSS_KEY);

        let cell = TrackerCellId::ForestKeys.kind();
        let render = cell.render(&state);

        // Boss key but no small keys: right dimmed (small key side)
        assert_eq!(render.style, CellStyle::RightDimmed);
    }

    #[test]
    fn test_composite_keys_visibility_both_active() {
        let mut state = ModelState::default();
        state.ram.save.small_keys.forest_temple = 3;
        state
            .ram
            .save
            .dungeon_items
            .forest_temple
            .insert(DungeonItems::BOSS_KEY);

        let cell = TrackerCellId::ForestKeys.kind();
        let render = cell.render(&state);

        // Both active: normal
        assert_eq!(render.style, CellStyle::Normal);
    }

    #[test]
    fn test_overlay_cell_visibility() {
        let mut state = ModelState::default();
        let cell = TrackerCellId::Hookshot.kind();

        // Without hookshot, dimmed
        let render = cell.render(&state);
        assert_eq!(render.style, CellStyle::Dimmed);

        // With hookshot, normal
        state.ram.save.inv.hookshot = Hookshot::Hookshot;
        let render = cell.render(&state);
        assert_eq!(render.style, CellStyle::Normal);
    }

    // ==========================================================================
    // Additional Active/Inactive State Tests
    // ==========================================================================

    #[test]
    fn test_all_medallions_inactive_by_default() {
        let state = ModelState::default();

        for med in [
            TrackerCellId::LightMedallion,
            TrackerCellId::ForestMedallion,
            TrackerCellId::FireMedallion,
            TrackerCellId::WaterMedallion,
            TrackerCellId::ShadowMedallion,
            TrackerCellId::SpiritMedallion,
        ] {
            let render = med.kind().render(&state);
            assert_eq!(
                render.style,
                CellStyle::Dimmed,
                "Medallion {:?} should be dimmed by default",
                med
            );
        }
    }

    #[test]
    fn test_all_stones_inactive_by_default() {
        let state = ModelState::default();

        for stone in [
            TrackerCellId::KokiriEmerald,
            TrackerCellId::GoronRuby,
            TrackerCellId::ZoraSapphire,
        ] {
            let render = stone.kind().render(&state);
            assert_eq!(
                render.style,
                CellStyle::Dimmed,
                "Stone {:?} should be dimmed by default",
                stone
            );
        }
    }

    #[test]
    fn test_song_render_inactive() {
        let state = ModelState::default();
        let cell = TrackerCellId::ZeldasLullaby.kind();
        let render = cell.render(&state);

        assert_eq!(render.style, CellStyle::Dimmed);
    }

    #[test]
    fn test_song_click_toggles() {
        let mut state = ModelState::default();
        let cell = TrackerCellId::ZeldasLullaby.kind();

        assert!(!state
            .ram
            .save
            .quest_items
            .contains(QuestItems::ZELDAS_LULLABY));

        cell.click(&mut state);
        assert!(state
            .ram
            .save
            .quest_items
            .contains(QuestItems::ZELDAS_LULLABY));

        cell.click(&mut state);
        assert!(!state
            .ram
            .save
            .quest_items
            .contains(QuestItems::ZELDAS_LULLABY));
    }

    #[test]
    fn test_warp_song_render_and_click() {
        let mut state = ModelState::default();
        let cell = TrackerCellId::Minuet.kind();

        // Initially dimmed
        let render = cell.render(&state);
        assert_eq!(render.style, CellStyle::Dimmed);

        // Click to activate
        cell.click(&mut state);
        let render = cell.render(&state);
        assert_eq!(render.style, CellStyle::Normal);
    }

    // ==========================================================================
    // MM Items Toggle Tests (Issue #515)
    // ==========================================================================

    #[test]
    fn test_mm_small_keys_click_creates_mm_save_when_none() {
        let mut state = ModelState::default();

        // Verify mm_save is initially None
        assert!(state.ram.mm_save.is_none());

        let cell = TrackerCellId::MmWoodfallSmallKeys.kind();

        // Click to increment small keys
        cell.click(&mut state);

        // mm_save should now be Some and key count should be 1
        assert!(state.ram.mm_save.is_some());
        assert_eq!(state.ram.mm_save.as_ref().unwrap().small_keys.woodfall, 1);
    }

    #[test]
    fn test_mm_boss_key_click_creates_mm_save_when_none() {
        let mut state = ModelState::default();

        // Verify mm_save is initially None
        assert!(state.ram.mm_save.is_none());

        let cell = TrackerCellId::MmWoodfallBossKey.kind();

        // Click to toggle boss key
        cell.click(&mut state);

        // mm_save should now be Some and boss key should be set
        assert!(state.ram.mm_save.is_some());
        assert!(state
            .ram
            .mm_save
            .as_ref()
            .unwrap()
            .dungeon_items
            .woodfall
            .contains(crate::mm_save::MmDungeonItems::BOSS_KEY));
    }

    #[test]
    fn test_mm_map_click_creates_mm_save_when_none() {
        let mut state = ModelState::default();

        // Verify mm_save is initially None
        assert!(state.ram.mm_save.is_none());

        let cell = TrackerCellId::MmWoodfallMap.kind();

        // Click to toggle map
        cell.click(&mut state);

        // mm_save should now be Some and map should be set
        assert!(state.ram.mm_save.is_some());
        assert!(state
            .ram
            .mm_save
            .as_ref()
            .unwrap()
            .dungeon_items
            .woodfall
            .contains(crate::mm_save::MmDungeonItems::MAP));
    }

    #[test]
    fn test_mm_compass_click_creates_mm_save_when_none() {
        let mut state = ModelState::default();

        // Verify mm_save is initially None
        assert!(state.ram.mm_save.is_none());

        let cell = TrackerCellId::MmWoodfallCompass.kind();

        // Click to toggle compass
        cell.click(&mut state);

        // mm_save should now be Some and compass should be set
        assert!(state.ram.mm_save.is_some());
        assert!(state
            .ram
            .mm_save
            .as_ref()
            .unwrap()
            .dungeon_items
            .woodfall
            .contains(crate::mm_save::MmDungeonItems::COMPASS));
    }

    // ==========================================================================
    // Accessibility Rendering Tests
    // ==========================================================================

    #[test]
    fn test_medallion_with_location_accessibility_none_when_not_possessed() {
        let state = ModelState::default();
        let cell = TrackerCellId::ForestMedallionWithLocation.kind();
        let render = cell.render(&state);

        // When medallion is not possessed, accessibility should be None
        assert!(
            render.accessibility.is_none(),
            "Accessibility should be None when medallion is not possessed"
        );
        assert_eq!(render.style, CellStyle::Dimmed);
    }

    #[test]
    fn test_medallion_with_location_accessibility_checked_when_possessed() {
        let mut state = ModelState::default();
        state
            .ram
            .save
            .quest_items
            .insert(QuestItems::FOREST_MEDALLION);

        let cell = TrackerCellId::ForestMedallionWithLocation.kind();
        let render = cell.render(&state);

        // When medallion is possessed, accessibility should be Checked
        assert_eq!(
            render.accessibility,
            Some(AccessibilityStatus::Checked),
            "Accessibility should be Checked when medallion is possessed"
        );
        assert_eq!(render.style, CellStyle::Normal);
    }

    #[test]
    fn test_stone_with_location_accessibility_none_when_not_possessed() {
        let state = ModelState::default();
        let cell = TrackerCellId::KokiriEmeraldWithLocation.kind();
        let render = cell.render(&state);

        // When stone is not possessed, accessibility should be None
        assert!(
            render.accessibility.is_none(),
            "Accessibility should be None when stone is not possessed"
        );
        assert_eq!(render.style, CellStyle::Dimmed);
    }

    #[test]
    fn test_stone_with_location_accessibility_checked_when_possessed() {
        let mut state = ModelState::default();
        state
            .ram
            .save
            .quest_items
            .insert(QuestItems::KOKIRI_EMERALD);

        let cell = TrackerCellId::KokiriEmeraldWithLocation.kind();
        let render = cell.render(&state);

        // When stone is possessed, accessibility should be Checked
        assert_eq!(
            render.accessibility,
            Some(AccessibilityStatus::Checked),
            "Accessibility should be Checked when stone is possessed"
        );
        assert_eq!(render.style, CellStyle::Normal);
    }

    #[test]
    fn test_song_accessibility_none_when_check_not_completed() {
        let state = ModelState::default();
        let cell = TrackerCellId::ZeldasLullaby.kind();
        let render = cell.render(&state);

        // When song check is not completed, accessibility should be None
        assert!(
            render.accessibility.is_none(),
            "Accessibility should be None when song check is not completed"
        );
    }

    #[test]
    fn test_song_check_accessibility_none_when_not_completed() {
        let state = ModelState::default();
        let cell = TrackerCellId::ZeldasLullabyCheck.kind();
        let render = cell.render(&state);

        // When check is not completed, accessibility should be None
        assert!(
            render.accessibility.is_none(),
            "Accessibility should be None when check is not completed"
        );
    }
}

#[cfg(feature = "embed-images")]
pub mod images {
    use super::FromEmbeddedImage;

    oottracker_derive::embed_images!("assets/img/extra-images");
    oottracker_derive::embed_images!("assets/img/extra-images-count");
    oottracker_derive::embed_images!("assets/img/extra-images-dimmed");
    oottracker_derive::embed_images!("assets/img/extra-images-overlay");
    oottracker_derive::embed_images!("assets/img/extra-images-overlay-dimmed");
    oottracker_derive::embed_images!("assets/img/extra-overlays");
    oottracker_derive::embed_images!("assets/img/xopar-images");
    oottracker_derive::embed_images!("assets/img/xopar-images-count");
    oottracker_derive::embed_images!("assets/img/xopar-images-dimmed");
    oottracker_derive::embed_images!("assets/img/xopar-images-overlay");
    oottracker_derive::embed_images!("assets/img/xopar-images-overlay-dimmed");
    oottracker_derive::embed_images!("assets/img/xopar-overlays");
    oottracker_derive::embed_image!("assets/icon.ico");
}
