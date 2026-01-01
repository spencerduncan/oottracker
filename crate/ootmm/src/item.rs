//! Item definitions for OoT and MM.

pub mod mm;
pub mod oot;

/// Combined item enum for both games.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Item {
    Oot(oot::OotItem),
    Mm(mm::MmItem),
}
