//! Image handling for the tracker UI.
//!
//! This module provides types for referencing and loading tracker images
//! from various image directories (xopar, extra, mm).

use {async_proto::Protocol, std::borrow::Cow};

/// Context for image directory selection.
pub enum ImageDirContext {
    Normal,
    Count(u8),
    Dimmed,
    OverlayOnly,
}

/// Image directory categories.
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

/// Information about a tracker image.
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
            (ImageDir::Xopar, ImageDirContext::Normal) => embedded_images::xopar_images(&self.name),
            (ImageDir::Extra, ImageDirContext::Normal) => embedded_images::extra_images(&self.name),
            // MM images fall back to extra images until MM assets are added
            (ImageDir::Mm, ImageDirContext::Normal) => embedded_images::extra_images(&self.name),
            (ImageDir::Xopar, ImageDirContext::Count(count)) => {
                embedded_images::xopar_images_count(&format!("{}_{}", self.name, count))
            }
            (ImageDir::Extra, ImageDirContext::Count(count)) => {
                embedded_images::extra_images_count(&format!("{}_{}", self.name, count))
            }
            (ImageDir::Mm, ImageDirContext::Count(count)) => {
                embedded_images::extra_images_count(&format!("{}_{}", self.name, count))
            }
            (ImageDir::Xopar, ImageDirContext::Dimmed) => {
                embedded_images::xopar_images_dimmed(&self.name)
            }
            (ImageDir::Extra, ImageDirContext::Dimmed) => {
                embedded_images::extra_images_dimmed(&self.name)
            }
            (ImageDir::Mm, ImageDirContext::Dimmed) => {
                embedded_images::extra_images_dimmed(&self.name)
            }
            (ImageDir::Xopar, ImageDirContext::OverlayOnly) => {
                embedded_images::xopar_overlays(&self.name)
            }
            (ImageDir::Extra, ImageDirContext::OverlayOnly) => {
                embedded_images::extra_overlays(&self.name)
            }
            (ImageDir::Mm, ImageDirContext::OverlayOnly) => {
                embedded_images::extra_overlays(&self.name)
            }
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

/// Image info for overlay combinations.
pub struct OverlayImageInfo {
    dir: ImageDir,
    main: Cow<'static, str>,
    overlay: Cow<'static, str>,
}

impl OverlayImageInfo {
    #[cfg(feature = "embed-images")]
    pub fn embedded<T: FromEmbeddedImage>(&self, main_active: bool) -> T {
        (match (self.dir, main_active) {
            (ImageDir::Xopar, false) => embedded_images::xopar_images_overlay_dimmed,
            (ImageDir::Xopar, true) => embedded_images::xopar_images_overlay,
            (ImageDir::Extra | ImageDir::Mm, false) => embedded_images::extra_images_overlay_dimmed,
            (ImageDir::Extra | ImageDir::Mm, true) => embedded_images::extra_images_overlay,
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

/// Trait for types that can be constructed from embedded image data.
pub trait FromEmbeddedImage {
    fn from_embedded_image(contents: &'static [u8]) -> Self;
}

#[cfg(feature = "iced")]
impl FromEmbeddedImage for iced::widget::Image {
    fn from_embedded_image(contents: &'static [u8]) -> iced::widget::Image {
        iced::widget::Image::new(iced::widget::image::Handle::from_memory(contents.to_vec()))
    }
}

impl FromEmbeddedImage for image::DynamicImage {
    fn from_embedded_image(contents: &'static [u8]) -> image::DynamicImage {
        image::load_from_memory(contents).expect("failed to load embedded image")
    }
}

/// Embedded images module (when feature is enabled).
#[cfg(feature = "embed-images")]
pub mod embedded_images {
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
