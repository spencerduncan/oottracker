//! Dungeon type definitions for OoT and MM randomizer settings.
//!
//! This module defines the dungeon identifiers used for open dungeon settings
//! and Master Quest dungeon selection.

use serde::{Deserialize, Serialize};

/// OoT dungeon identifiers for `openDungeonsOot` setting.
///
/// These correspond to the values used in logic expressions like
/// `setting(openDungeonsOot, DC)`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum OotDungeon {
    /// Dodongo's Cavern (DC)
    #[serde(rename = "DC")]
    DodongosCavern,
    /// Bottom of the Well (BotW)
    #[serde(rename = "BotW")]
    BottomOfTheWell,
    /// Jabu-Jabu's Belly (JJ)
    #[serde(rename = "JJ")]
    JabuJabu,
    /// Shadow Temple
    Shadow,
    /// Water Temple
    Water,
    /// Fire Temple accessible as child
    #[serde(rename = "fireChild")]
    FireChild,
    /// Well accessible as adult
    #[serde(rename = "wellAdult")]
    WellAdult,
}

impl OotDungeon {
    /// Returns the string identifier used in logic expressions.
    #[must_use]
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::DodongosCavern => "DC",
            Self::BottomOfTheWell => "BotW",
            Self::JabuJabu => "JJ",
            Self::Shadow => "Shadow",
            Self::Water => "Water",
            Self::FireChild => "fireChild",
            Self::WellAdult => "wellAdult",
        }
    }

    /// Parses a logic string identifier into an OotDungeon.
    #[must_use]
    pub fn parse(s: &str) -> Option<Self> {
        match s {
            "DC" => Some(Self::DodongosCavern),
            "BotW" => Some(Self::BottomOfTheWell),
            "JJ" => Some(Self::JabuJabu),
            "Shadow" => Some(Self::Shadow),
            "Water" => Some(Self::Water),
            "fireChild" => Some(Self::FireChild),
            "wellAdult" => Some(Self::WellAdult),
            _ => None,
        }
    }
}

/// MM dungeon identifiers for `openDungeonsMm` setting.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum MmDungeon {
    /// Stone Tower Temple (ST)
    #[serde(rename = "ST")]
    StoneTower,
    /// Woodfall Temple (WF)
    #[serde(rename = "WF")]
    Woodfall,
}

impl MmDungeon {
    /// Returns the string identifier used in logic expressions.
    #[must_use]
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::StoneTower => "ST",
            Self::Woodfall => "WF",
        }
    }

    /// Parses a logic string identifier into an MmDungeon.
    #[must_use]
    pub fn parse(s: &str) -> Option<Self> {
        match s {
            "ST" => Some(Self::StoneTower),
            "WF" => Some(Self::Woodfall),
            _ => None,
        }
    }
}

/// OoT dungeons that can be set to Master Quest.
///
/// These correspond to all dungeons in OoT that have Master Quest variants
/// with different layouts, puzzles, and check locations.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MqDungeon {
    /// Deku Tree
    DekuTree,
    /// Dodongo's Cavern
    DodongosCavern,
    /// Jabu-Jabu's Belly
    JabuJabu,
    /// Forest Temple
    ForestTemple,
    /// Fire Temple
    FireTemple,
    /// Water Temple
    WaterTemple,
    /// Spirit Temple
    SpiritTemple,
    /// Shadow Temple
    ShadowTemple,
    /// Bottom of the Well
    BottomOfTheWell,
    /// Ice Cavern
    IceCavern,
    /// Gerudo Training Ground
    GerudoTrainingGround,
    /// Ganon's Castle
    GanonsCastle,
}

impl MqDungeon {
    /// Returns the string identifier used in logic expressions and settings.
    #[must_use]
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::DekuTree => "deku_tree",
            Self::DodongosCavern => "dodongos_cavern",
            Self::JabuJabu => "jabu_jabu",
            Self::ForestTemple => "forest_temple",
            Self::FireTemple => "fire_temple",
            Self::WaterTemple => "water_temple",
            Self::SpiritTemple => "spirit_temple",
            Self::ShadowTemple => "shadow_temple",
            Self::BottomOfTheWell => "bottom_of_the_well",
            Self::IceCavern => "ice_cavern",
            Self::GerudoTrainingGround => "gerudo_training_ground",
            Self::GanonsCastle => "ganons_castle",
        }
    }

    /// Returns the location ID prefix for this dungeon in its vanilla variant.
    ///
    /// Vanilla locations use the `oot_<dungeon>_` prefix.
    #[must_use]
    pub const fn vanilla_location_prefix(&self) -> &'static str {
        match self {
            Self::DekuTree => "oot_deku_tree_",
            Self::DodongosCavern => "oot_dodongo_cavern_",
            Self::JabuJabu => "oot_jabu_jabu_",
            Self::ForestTemple => "oot_forest_temple_",
            Self::FireTemple => "oot_fire_temple_",
            Self::WaterTemple => "oot_water_temple_",
            Self::SpiritTemple => "oot_spirit_temple_",
            Self::ShadowTemple => "oot_shadow_temple_",
            Self::BottomOfTheWell => "oot_bottom_of_the_well_",
            Self::IceCavern => "oot_ice_cavern_",
            Self::GerudoTrainingGround => "oot_gerudo_training_",
            Self::GanonsCastle => "oot_ganon_castle_",
        }
    }

    /// Returns the location ID prefix for this dungeon in its MQ variant.
    ///
    /// MQ locations use the `mq_oot_mq_<dungeon>_` prefix for checks,
    /// or `mq_oot_<dungeon>_` for regions.
    #[must_use]
    pub const fn mq_location_prefix(&self) -> &'static str {
        match self {
            Self::DekuTree => "mq_oot_mq_deku_tree_",
            Self::DodongosCavern => "mq_oot_mq_dodongo_cavern_",
            Self::JabuJabu => "mq_oot_mq_jabu_jabu_",
            Self::ForestTemple => "mq_oot_mq_forest_temple_",
            Self::FireTemple => "mq_oot_mq_fire_temple_",
            Self::WaterTemple => "mq_oot_mq_water_temple_",
            Self::SpiritTemple => "mq_oot_mq_spirit_temple_",
            Self::ShadowTemple => "mq_oot_mq_shadow_temple_",
            Self::BottomOfTheWell => "mq_oot_mq_bottom_of_the_well_",
            Self::IceCavern => "mq_oot_mq_ice_cavern_",
            Self::GerudoTrainingGround => "mq_oot_mq_gerudo_training_",
            Self::GanonsCastle => "mq_oot_mq_ganon_castle_",
        }
    }

    /// Parses a string identifier into an MqDungeon.
    #[must_use]
    pub fn parse(s: &str) -> Option<Self> {
        match s {
            "deku_tree" | "DekuTree" => Some(Self::DekuTree),
            "dodongos_cavern" | "DodongosCavern" => Some(Self::DodongosCavern),
            "jabu_jabu" | "JabuJabu" => Some(Self::JabuJabu),
            "forest_temple" | "ForestTemple" => Some(Self::ForestTemple),
            "fire_temple" | "FireTemple" => Some(Self::FireTemple),
            "water_temple" | "WaterTemple" => Some(Self::WaterTemple),
            "spirit_temple" | "SpiritTemple" => Some(Self::SpiritTemple),
            "shadow_temple" | "ShadowTemple" => Some(Self::ShadowTemple),
            "bottom_of_the_well" | "BottomOfTheWell" => Some(Self::BottomOfTheWell),
            "ice_cavern" | "IceCavern" => Some(Self::IceCavern),
            "gerudo_training_ground" | "GerudoTrainingGround" => Some(Self::GerudoTrainingGround),
            "ganons_castle" | "GanonsCastle" => Some(Self::GanonsCastle),
            _ => None,
        }
    }

    /// Returns all MQ dungeon variants.
    #[must_use]
    pub const fn all() -> &'static [MqDungeon] {
        &[
            Self::DekuTree,
            Self::DodongosCavern,
            Self::JabuJabu,
            Self::ForestTemple,
            Self::FireTemple,
            Self::WaterTemple,
            Self::SpiritTemple,
            Self::ShadowTemple,
            Self::BottomOfTheWell,
            Self::IceCavern,
            Self::GerudoTrainingGround,
            Self::GanonsCastle,
        ]
    }

    /// Attempts to determine which dungeon a location ID belongs to.
    ///
    /// Returns `None` if the location is not in a dungeon that has MQ variants.
    #[must_use]
    pub fn from_location_id(location_id: &str) -> Option<Self> {
        // Check MQ locations first (they have the mq_ prefix)
        if location_id.starts_with("mq_oot_") {
            // MQ dungeon locations
            if location_id.contains("deku_tree") {
                return Some(Self::DekuTree);
            }
            if location_id.contains("dodongo") {
                return Some(Self::DodongosCavern);
            }
            if location_id.contains("jabu") {
                return Some(Self::JabuJabu);
            }
            if location_id.contains("forest_temple") {
                return Some(Self::ForestTemple);
            }
            if location_id.contains("fire_temple") {
                return Some(Self::FireTemple);
            }
            if location_id.contains("water_temple") {
                return Some(Self::WaterTemple);
            }
            if location_id.contains("spirit_temple") {
                return Some(Self::SpiritTemple);
            }
            if location_id.contains("shadow_temple") {
                return Some(Self::ShadowTemple);
            }
            if location_id.contains("bottom_of_the_well") {
                return Some(Self::BottomOfTheWell);
            }
            if location_id.contains("ice_cavern") {
                return Some(Self::IceCavern);
            }
            if location_id.contains("gerudo_training") {
                return Some(Self::GerudoTrainingGround);
            }
            if location_id.contains("ganon_castle") {
                return Some(Self::GanonsCastle);
            }
            return None;
        }

        // Check vanilla OoT dungeon locations
        if location_id.starts_with("oot_deku_tree_") {
            return Some(Self::DekuTree);
        }
        if location_id.starts_with("oot_dodongo_cavern_") || location_id.starts_with("oot_dodongo_")
        {
            return Some(Self::DodongosCavern);
        }
        if location_id.starts_with("oot_jabu_jabu_") {
            return Some(Self::JabuJabu);
        }
        if location_id.starts_with("oot_forest_temple_") {
            return Some(Self::ForestTemple);
        }
        if location_id.starts_with("oot_fire_temple_") {
            return Some(Self::FireTemple);
        }
        if location_id.starts_with("oot_water_temple_") {
            return Some(Self::WaterTemple);
        }
        if location_id.starts_with("oot_spirit_temple_") {
            return Some(Self::SpiritTemple);
        }
        if location_id.starts_with("oot_shadow_temple_") {
            return Some(Self::ShadowTemple);
        }
        if location_id.starts_with("oot_bottom_of_the_well_") {
            return Some(Self::BottomOfTheWell);
        }
        if location_id.starts_with("oot_ice_cavern_") {
            return Some(Self::IceCavern);
        }
        if location_id.starts_with("oot_gerudo_training_") {
            return Some(Self::GerudoTrainingGround);
        }
        if location_id.starts_with("oot_ganon_castle_") {
            return Some(Self::GanonsCastle);
        }

        None
    }
}
