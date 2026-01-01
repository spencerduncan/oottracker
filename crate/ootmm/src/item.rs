//! Item definitions for OoT and MM.

pub mod oot;
pub mod mm;

/// Combined item enum for both games.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Item {
    Oot(oot::OotItem),
    Mm(mm::MmItem),
}
