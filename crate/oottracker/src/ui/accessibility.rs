//! Accessibility status types for location/check tracking.
//!
//! This module provides types for representing and displaying whether
//! game locations are accessible, inaccessible, checked, or unknown.

use {
    async_proto::Protocol,
    serde::{Deserialize, Serialize},
    std::fmt,
};
#[cfg(feature = "rocket")]
use {
    rocket::response::content::RawHtml,
    rocket_util::{html, ToHtml},
};

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
