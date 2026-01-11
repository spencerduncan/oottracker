//! Configuration types for the tracker UI.
//!
//! This module provides configuration persistence for user preferences
//! such as medallion order, warp song order, and layout preferences.

use {
    async_proto::Protocol,
    derivative::Derivative,
    directories::ProjectDirs,
    ootr::model::Medallion,
    serde::{Deserialize, Serialize},
    std::{fmt, io, path::PathBuf, sync::Arc, vec},
    tokio::{
        fs::{self, File},
        io::AsyncReadExt as _,
    },
    wheel::FromArc,
};

const VERSION: u8 = 0;

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

fn default_med_order() -> ElementOrder {
    ElementOrder::LightShadowSpirit
}

fn default_warp_song_order() -> ElementOrder {
    ElementOrder::SpiritShadowLight
}

pub fn dirs() -> Result<ProjectDirs, Error> {
    ProjectDirs::from("net", "Fenhl", "OoT Tracker").ok_or(Error::MissingHomeDir)
}

/// Re-export of VERSION for tests
pub const CONFIG_VERSION: u8 = VERSION;
