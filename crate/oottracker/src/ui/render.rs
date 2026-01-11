//! Cell rendering types for the tracker UI.
//!
//! This module provides types for describing how tracker cells should be rendered,
//! including styles, overlays, and accessibility indicators.

#[cfg(feature = "rocket")]
use {
    super::images::ImageDirContext,
    rocket::response::content::RawHtml,
    rocket_util::{html, ToHtml},
};
use {
    super::{accessibility::AccessibilityStatus, images::ImageInfo},
    async_proto::Protocol,
};

/// Visual style for a cell.
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

/// Overlay type for a cell.
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

/// Style for location overlay text.
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

/// Complete render specification for a cell.
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
