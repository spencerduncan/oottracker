//! MM Flag Type definitions.
//!
//! This module defines the types of flags used to track location checks
//! in MM save data.

/// Types of flags used to track location checks in MM save data.
///
/// Each location in the game is tracked by one of these flag types,
/// stored in specific memory regions within the save context.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MmFlagType {
    /// Chest opened flags (scene flags offset 0x00).
    /// Each bit represents a chest in the scene.
    Chest,

    /// Switch/trigger flags bank 0 (scene flags offset 0x04).
    /// Includes crystal switches, floor switches, etc.
    Switch0,

    /// Switch/trigger flags bank 1 (scene flags offset 0x08).
    /// Additional switch flags for complex scenes.
    Switch1,

    /// Room clear flags (scene flags offset 0x0C).
    /// Set when all enemies in a room are defeated.
    ClearedRoom,

    /// Collectible item flags (scene flags offset 0x10).
    /// Freestanding items, rupees, hearts, etc.
    Collectible,

    /// Gold Skulltula flags for spider houses.
    /// Swamp Spider House and Oceanside Spider House tokens.
    GoldSkulltula,

    /// Event flags for global game events.
    /// Tracks major story events and NPC interactions.
    EventInf,

    /// Week event flags that persist across cycles.
    /// Tracks events that should survive Song of Time.
    WeekEventReg,

    /// Item get flags.
    /// Tracks specific item acquisitions.
    ItemGetInf,

    /// Shop item flags.
    /// Tracks purchased shop items.
    Shop,

    /// Scrub/merchant purchase flags.
    /// Business scrubs and other merchants.
    Scrub,

    /// Great Fairy reward flags.
    /// Tracks fairy fountain upgrades received.
    GreatFairy,

    /// Boss defeated flags.
    /// Boss remains collected after defeating bosses.
    Boss,

    /// Song learned flags.
    /// Stored in quest items bitfield.
    Song,

    /// Cow/milk flags.
    /// Playing Epona's Song to cows.
    Cow,

    /// Stray fairy flags.
    /// Tracks stray fairies collected in dungeons.
    StrayFairy,

    /// Owl statue flags.
    /// Tracks activated owl statues.
    OwlStatue,

    /// Moon's Tear related flags.
    MoonsTear,

    /// Gossip stone hint flags.
    /// Hints from gossip stones (if shuffled).
    GossipStone,

    /// Extended flags (xflags) from OoTMM.
    /// Used for actor-based collectibles like pots, grass, crates, etc.
    /// The flag_bit field contains the xflag bit position.
    Xflag,
}

impl MmFlagType {
    /// Returns the byte offset within scene flags for scene-based flag types.
    ///
    /// Returns `None` for global flag types that aren't stored per-scene.
    #[must_use]
    pub const fn scene_offset(&self) -> Option<usize> {
        match self {
            MmFlagType::Chest => Some(0x00),
            MmFlagType::Switch0 => Some(0x04),
            MmFlagType::Switch1 => Some(0x08),
            MmFlagType::ClearedRoom => Some(0x0C),
            MmFlagType::Collectible => Some(0x10),
            // These are global, not per-scene
            MmFlagType::GoldSkulltula
            | MmFlagType::EventInf
            | MmFlagType::WeekEventReg
            | MmFlagType::ItemGetInf
            | MmFlagType::Shop
            | MmFlagType::Scrub
            | MmFlagType::GreatFairy
            | MmFlagType::Boss
            | MmFlagType::Song
            | MmFlagType::Cow
            | MmFlagType::StrayFairy
            | MmFlagType::OwlStatue
            | MmFlagType::MoonsTear
            | MmFlagType::GossipStone
            | MmFlagType::Xflag => None,
        }
    }

    /// Returns whether this flag type is stored per-scene.
    #[must_use]
    pub const fn is_scene_based(&self) -> bool {
        self.scene_offset().is_some()
    }
}
