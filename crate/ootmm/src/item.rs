//! Item types for OoT and MM.

pub mod mm;
pub mod oot;

pub use mm::MmItem;
pub use oot::OotItem;

/// Combined item enum for both games.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Item {
    Oot(OotItem),
    Mm(MmItem),
}
