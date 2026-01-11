//! UI module for the OoT Tracker.
//!
//! This module provides the tracker UI components including:
//! - Cell types and rendering
//! - Layout configurations
//! - Configuration persistence
//! - Accessibility status tracking
//! - Image handling

#![allow(unused_qualifications)] // oottracker::ui::TrackerCellKind::SmallKeys vs oottracker::save::SmallKeys

pub mod accessibility;
pub mod config;
pub mod images;
pub mod render;

// Re-export commonly used types
pub use accessibility::{
    AccessibilityStatus, AccessibilitySummary, LocationAccessibility, LocationAccessibilityList,
};
pub use config::{dirs, Config, ElementOrder, Error, LayoutPreference, CONFIG_VERSION};
pub use images::{FromEmbeddedImage, ImageDir, ImageDirContext, ImageInfo, OverlayImageInfo};
pub use render::{CellOverlay, CellRender, CellStyle, LocationStyle};

// Re-export embedded images when available
#[cfg(feature = "embed-images")]
pub use images::embedded_images;

// Include the cells and layout code (these are large and tightly coupled)
mod cells;
mod layout;

pub use cells::{
    DungeonRewardLocationExt, MmSmallKeysSetter, SmallKeysSetter, StateImageGetter,
    StatePairChecker, StateU8Getter, StateU8Setter, TrackerCellId, TrackerCellKind,
};
pub use layout::{CellLayout, DoubleTrackerLayout, TrackerLayout};

#[cfg(test)]
mod tests;
