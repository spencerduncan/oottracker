use {
    crate::item::Item,
    async_proto::Protocol,
    enum_iterator::Sequence,
    quote_value::QuoteValue,
    serde::{Deserialize, Serialize},
    serde_plain::{derive_deserialize_from_fromstr, derive_serialize_from_display},
    std::{fmt, str::FromStr},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Protocol, QuoteValue)]
pub enum Dungeon {
    Main(MainDungeon),
    IceCavern,
    BottomOfTheWell,
    GerudoTrainingGround,
    GanonsCastle,
}

impl Dungeon {
    pub fn rando_name(&self) -> &'static str {
        match self {
            Self::Main(MainDungeon::DekuTree) => "Deku Tree",
            Self::Main(MainDungeon::DodongosCavern) => "Dodongos Cavern",
            Self::Main(MainDungeon::JabuJabu) => "Jabu Jabus Belly",
            Self::Main(MainDungeon::ForestTemple) => "Forest Temple",
            Self::Main(MainDungeon::FireTemple) => "Fire Temple",
            Self::Main(MainDungeon::WaterTemple) => "Water Temple",
            Self::Main(MainDungeon::ShadowTemple) => "Shadow Temple",
            Self::Main(MainDungeon::SpiritTemple) => "Spirit Temple",
            Self::IceCavern => "Ice Cavern",
            Self::BottomOfTheWell => "Bottom of the Well",
            Self::GerudoTrainingGround => "Gerudo Training Ground",
            Self::GanonsCastle => "Ganons Castle",
        }
    }
}

impl FromStr for Dungeon {
    type Err = ();

    fn from_str(s: &str) -> Result<Dungeon, ()> {
        MainDungeon::from_str(s).map(Dungeon::Main).or(match s {
            "Ice Cavern" => Ok(Dungeon::IceCavern),
            "Bottom of the Well" => Ok(Dungeon::BottomOfTheWell),
            "Gerudo Training Ground" | "Gerudo Training Grounds" => {
                Ok(Dungeon::GerudoTrainingGround)
            }
            "Ganon's Castle" | "Ganons Castle" => Ok(Dungeon::GanonsCastle),
            _ => Err(()),
        })
    }
}

impl fmt::Display for Dungeon {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Dungeon::Main(main) => main.fmt(f),
            Dungeon::IceCavern => write!(f, "Ice Cavern"),
            Dungeon::BottomOfTheWell => write!(f, "Bottom of the Well"),
            Dungeon::GerudoTrainingGround => write!(f, "Gerudo Training Ground"),
            Dungeon::GanonsCastle => write!(f, "Ganon's Castle"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Sequence, Protocol)]
pub enum DungeonReward {
    Medallion(Medallion),
    Stone(Stone),
}

impl FromStr for DungeonReward {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, ()> {
        Ok(match s.parse() {
            Ok(med) => Self::Medallion(med),
            Err(()) => Self::Stone(s.parse()?),
        })
    }
}

impl TryFrom<Item> for DungeonReward {
    type Error = ();

    fn try_from(item: Item) -> Result<Self, ()> {
        item.0.parse()
    }
}

impl fmt::Display for DungeonReward {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Medallion(med) => med.fmt(f),
            Self::Stone(stone) => stone.fmt(f),
        }
    }
}

derive_deserialize_from_fromstr!(DungeonReward, "dungeon reward");
derive_serialize_from_display!(DungeonReward);

impl From<DungeonReward> for Item {
    fn from(reward: DungeonReward) -> Self {
        Self(reward.to_string())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Protocol)]
pub enum DungeonRewardLocation {
    LinksPocket,
    Dungeon(MainDungeon),
}

impl DungeonRewardLocation {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::LinksPocket => "Links Pocket",
            Self::Dungeon(dungeon) => dungeon.reward_location(),
        }
    }
}

impl FromStr for DungeonRewardLocation {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, ()> {
        Ok(if s == "Links Pocket" {
            Self::LinksPocket
        } else {
            Self::Dungeon(MainDungeon::from_reward_location(s).ok_or(())?)
        })
    }
}

impl fmt::Display for DungeonRewardLocation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.as_str().fmt(f)
    }
}

derive_deserialize_from_fromstr!(DungeonRewardLocation, "dungeon reward location");
derive_serialize_from_display!(DungeonRewardLocation);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Sequence, Protocol, QuoteValue)]
pub enum MainDungeon {
    DekuTree,
    DodongosCavern,
    JabuJabu,
    ForestTemple,
    FireTemple,
    WaterTemple,
    ShadowTemple,
    SpiritTemple,
}

impl MainDungeon {
    pub fn from_reward_location(loc: &str) -> Option<Self> {
        match loc {
            "Queen Gohma" => Some(Self::DekuTree),
            "King Dodongo" => Some(Self::DodongosCavern),
            "Barinade" => Some(Self::JabuJabu),
            "Phantom Ganon" => Some(Self::ForestTemple),
            "Volvagia" => Some(Self::FireTemple),
            "Morpha" => Some(Self::WaterTemple),
            "Bongo Bongo" => Some(Self::ShadowTemple),
            "Twinrova" => Some(Self::SpiritTemple),
            _ => None,
        }
    }

    pub fn reward_location(&self) -> &'static str {
        match self {
            Self::DekuTree => "Queen Gohma",
            Self::DodongosCavern => "King Dodongo",
            Self::JabuJabu => "Barinade",
            Self::ForestTemple => "Phantom Ganon",
            Self::FireTemple => "Volvagia",
            Self::WaterTemple => "Morpha",
            Self::ShadowTemple => "Bongo Bongo",
            Self::SpiritTemple => "Twinrova",
        }
    }
}

impl FromStr for MainDungeon {
    type Err = ();

    fn from_str(s: &str) -> Result<MainDungeon, ()> {
        match s {
            "Deku Tree" => Ok(MainDungeon::DekuTree),
            "Dodongo's Cavern" | "Dodongos Cavern" => Ok(MainDungeon::DodongosCavern),
            "Jabu-Jabu" | "Jabu Jabus Belly" => Ok(MainDungeon::JabuJabu),
            "Forest Temple" => Ok(MainDungeon::ForestTemple),
            "Fire Temple" => Ok(MainDungeon::FireTemple),
            "Water Temple" => Ok(MainDungeon::WaterTemple),
            "Shadow Temple" => Ok(MainDungeon::ShadowTemple),
            "Spirit Temple" => Ok(MainDungeon::SpiritTemple),
            _ => Err(()),
        }
    }
}

impl fmt::Display for MainDungeon {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MainDungeon::DekuTree => write!(f, "Deku Tree"),
            MainDungeon::DodongosCavern => write!(f, "Dodongo's Cavern"),
            MainDungeon::JabuJabu => write!(f, "Jabu-Jabu"),
            MainDungeon::ForestTemple => write!(f, "Forest Temple"),
            MainDungeon::FireTemple => write!(f, "Fire Temple"),
            MainDungeon::WaterTemple => write!(f, "Water Temple"),
            MainDungeon::ShadowTemple => write!(f, "Shadow Temple"),
            MainDungeon::SpiritTemple => write!(f, "Spirit Temple"),
        }
    }
}

derive_deserialize_from_fromstr!(MainDungeon, "main dungeon");
derive_serialize_from_display!(MainDungeon);

#[derive(
    Debug, Clone, Copy, PartialEq, Eq, Hash, Sequence, Protocol, Deserialize, Serialize, QuoteValue,
)]
pub enum Medallion {
    Light,
    Forest,
    Fire,
    Water,
    Shadow,
    Spirit,
}

impl Medallion {
    /// Returns the medallion's element, e.g. `"Light"` for the Light Medallion.
    pub fn element(&self) -> &'static str {
        match self {
            Medallion::Light => "Light",
            Medallion::Forest => "Forest",
            Medallion::Fire => "Fire",
            Medallion::Water => "Water",
            Medallion::Shadow => "Shadow",
            Medallion::Spirit => "Spirit",
        }
    }
}

impl FromStr for Medallion {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, ()> {
        Ok(match s {
            "Light Medallion" => Self::Light,
            "Forest Medallion" => Self::Forest,
            "Fire Medallion" => Self::Fire,
            "Water Medallion" => Self::Water,
            "Shadow Medallion" => Self::Shadow,
            "Spirit Medallion" => Self::Spirit,
            _ => return Err(()),
        })
    }
}

impl TryFrom<Item> for Medallion {
    type Error = ();

    fn try_from(item: Item) -> Result<Self, ()> {
        item.0.parse()
    }
}

impl fmt::Display for Medallion {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} Medallion", self.element())
    }
}

impl From<Medallion> for Item {
    fn from(med: Medallion) -> Self {
        Self(med.to_string())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Sequence, Protocol)]
pub enum Stone {
    KokiriEmerald,
    GoronRuby,
    ZoraSapphire,
}

impl FromStr for Stone {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, ()> {
        Ok(match s {
            "Kokiri Emerald" => Self::KokiriEmerald,
            "Goron Ruby" => Self::GoronRuby,
            "Zora Sapphire" => Self::ZoraSapphire,
            _ => return Err(()),
        })
    }
}

impl TryFrom<Item> for Stone {
    type Error = ();

    fn try_from(item: Item) -> Result<Self, ()> {
        item.0.parse()
    }
}

impl fmt::Display for Stone {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::KokiriEmerald => write!(f, "Kokiri Emerald"),
            Self::GoronRuby => write!(f, "Goron Ruby"),
            Self::ZoraSapphire => write!(f, "Zora Sapphire"),
        }
    }
}

impl From<Stone> for Item {
    fn from(stone: Stone) -> Self {
        Self(stone.to_string())
    }
}

#[derive(Debug, Clone, Copy, QuoteValue)]
pub enum TimeRange {
    /// 06:00–18:00.
    ///
    /// Playing Sun's Song during `Night` sets the time to 12:00.
    Day,
    /// 18:00–06:00.
    ///
    /// Playing Sun's Song during `Day` sets the time to 00:00.
    Night,
    /// The time of day when Dampé's Heart-Pounding Gravedigging Tour is available: 18:00–21:00, a subset of `Night`.
    ///
    /// Going to outside Ganon's Castle sets the time to 18:01.
    Dampe,
}

#[cfg(test)]
mod tests {
    use super::*;

    // MainDungeon tests
    #[test]
    fn test_main_dungeon_from_str_valid() {
        assert_eq!(
            MainDungeon::from_str("Deku Tree"),
            Ok(MainDungeon::DekuTree)
        );
        assert_eq!(
            MainDungeon::from_str("Forest Temple"),
            Ok(MainDungeon::ForestTemple)
        );
        assert_eq!(
            MainDungeon::from_str("Fire Temple"),
            Ok(MainDungeon::FireTemple)
        );
        assert_eq!(
            MainDungeon::from_str("Water Temple"),
            Ok(MainDungeon::WaterTemple)
        );
        assert_eq!(
            MainDungeon::from_str("Shadow Temple"),
            Ok(MainDungeon::ShadowTemple)
        );
        assert_eq!(
            MainDungeon::from_str("Spirit Temple"),
            Ok(MainDungeon::SpiritTemple)
        );
    }

    #[test]
    fn test_main_dungeon_from_str_alternate_spellings() {
        // Dodongo's Cavern with apostrophe and without
        assert_eq!(
            MainDungeon::from_str("Dodongo's Cavern"),
            Ok(MainDungeon::DodongosCavern)
        );
        assert_eq!(
            MainDungeon::from_str("Dodongos Cavern"),
            Ok(MainDungeon::DodongosCavern)
        );
        // Jabu-Jabu variations
        assert_eq!(
            MainDungeon::from_str("Jabu-Jabu"),
            Ok(MainDungeon::JabuJabu)
        );
        assert_eq!(
            MainDungeon::from_str("Jabu Jabus Belly"),
            Ok(MainDungeon::JabuJabu)
        );
    }

    #[test]
    fn test_main_dungeon_from_str_invalid() {
        assert_eq!(MainDungeon::from_str("Invalid Dungeon"), Err(()));
        assert_eq!(MainDungeon::from_str(""), Err(()));
        assert_eq!(MainDungeon::from_str("Ice Cavern"), Err(()));
    }

    #[test]
    fn test_main_dungeon_display() {
        assert_eq!(MainDungeon::DekuTree.to_string(), "Deku Tree");
        assert_eq!(MainDungeon::DodongosCavern.to_string(), "Dodongo's Cavern");
        assert_eq!(MainDungeon::JabuJabu.to_string(), "Jabu-Jabu");
        assert_eq!(MainDungeon::ForestTemple.to_string(), "Forest Temple");
    }

    #[test]
    fn test_main_dungeon_reward_location_roundtrip() {
        for dungeon in enum_iterator::all::<MainDungeon>() {
            let boss = dungeon.reward_location();
            let from_boss = MainDungeon::from_reward_location(boss);
            assert_eq!(from_boss, Some(dungeon));
        }
    }

    #[test]
    fn test_main_dungeon_from_reward_location_invalid() {
        assert_eq!(MainDungeon::from_reward_location("Invalid Boss"), None);
        assert_eq!(MainDungeon::from_reward_location(""), None);
    }

    // Dungeon tests
    #[test]
    fn test_dungeon_from_str_main_dungeons() {
        assert_eq!(
            Dungeon::from_str("Deku Tree"),
            Ok(Dungeon::Main(MainDungeon::DekuTree))
        );
        assert_eq!(
            Dungeon::from_str("Forest Temple"),
            Ok(Dungeon::Main(MainDungeon::ForestTemple))
        );
    }

    #[test]
    fn test_dungeon_from_str_mini_dungeons() {
        assert_eq!(Dungeon::from_str("Ice Cavern"), Ok(Dungeon::IceCavern));
        assert_eq!(
            Dungeon::from_str("Bottom of the Well"),
            Ok(Dungeon::BottomOfTheWell)
        );
        // Test alternate spellings for Gerudo Training Ground
        assert_eq!(
            Dungeon::from_str("Gerudo Training Ground"),
            Ok(Dungeon::GerudoTrainingGround)
        );
        assert_eq!(
            Dungeon::from_str("Gerudo Training Grounds"),
            Ok(Dungeon::GerudoTrainingGround)
        );
        // Test alternate spellings for Ganon's Castle
        assert_eq!(
            Dungeon::from_str("Ganon's Castle"),
            Ok(Dungeon::GanonsCastle)
        );
        assert_eq!(
            Dungeon::from_str("Ganons Castle"),
            Ok(Dungeon::GanonsCastle)
        );
    }

    #[test]
    fn test_dungeon_display() {
        assert_eq!(Dungeon::IceCavern.to_string(), "Ice Cavern");
        assert_eq!(Dungeon::BottomOfTheWell.to_string(), "Bottom of the Well");
        assert_eq!(
            Dungeon::GerudoTrainingGround.to_string(),
            "Gerudo Training Ground"
        );
        assert_eq!(Dungeon::GanonsCastle.to_string(), "Ganon's Castle");
        assert_eq!(
            Dungeon::Main(MainDungeon::DekuTree).to_string(),
            "Deku Tree"
        );
    }

    #[test]
    fn test_dungeon_rando_name() {
        assert_eq!(Dungeon::IceCavern.rando_name(), "Ice Cavern");
        assert_eq!(Dungeon::BottomOfTheWell.rando_name(), "Bottom of the Well");
        assert_eq!(Dungeon::GanonsCastle.rando_name(), "Ganons Castle");
        assert_eq!(
            Dungeon::Main(MainDungeon::DekuTree).rando_name(),
            "Deku Tree"
        );
        assert_eq!(
            Dungeon::Main(MainDungeon::JabuJabu).rando_name(),
            "Jabu Jabus Belly"
        );
    }

    // Medallion tests
    #[test]
    fn test_medallion_element() {
        assert_eq!(Medallion::Light.element(), "Light");
        assert_eq!(Medallion::Forest.element(), "Forest");
        assert_eq!(Medallion::Fire.element(), "Fire");
        assert_eq!(Medallion::Water.element(), "Water");
        assert_eq!(Medallion::Shadow.element(), "Shadow");
        assert_eq!(Medallion::Spirit.element(), "Spirit");
    }

    #[test]
    fn test_medallion_from_str_and_display() {
        let medallions = [
            (Medallion::Light, "Light Medallion"),
            (Medallion::Forest, "Forest Medallion"),
            (Medallion::Fire, "Fire Medallion"),
            (Medallion::Water, "Water Medallion"),
            (Medallion::Shadow, "Shadow Medallion"),
            (Medallion::Spirit, "Spirit Medallion"),
        ];

        for (medallion, name) in &medallions {
            assert_eq!(Medallion::from_str(name), Ok(*medallion));
            assert_eq!(medallion.to_string(), *name);
        }
    }

    #[test]
    fn test_medallion_from_str_invalid() {
        assert_eq!(Medallion::from_str("Invalid Medallion"), Err(()));
        assert_eq!(Medallion::from_str("Light"), Err(()));
        assert_eq!(Medallion::from_str(""), Err(()));
    }

    // Stone tests
    #[test]
    fn test_stone_from_str_and_display() {
        let stones = [
            (Stone::KokiriEmerald, "Kokiri Emerald"),
            (Stone::GoronRuby, "Goron Ruby"),
            (Stone::ZoraSapphire, "Zora Sapphire"),
        ];

        for (stone, name) in &stones {
            assert_eq!(Stone::from_str(name), Ok(*stone));
            assert_eq!(stone.to_string(), *name);
        }
    }

    #[test]
    fn test_stone_from_str_invalid() {
        assert_eq!(Stone::from_str("Invalid Stone"), Err(()));
        assert_eq!(Stone::from_str("Kokiri"), Err(()));
        assert_eq!(Stone::from_str(""), Err(()));
    }

    // DungeonReward tests
    #[test]
    fn test_dungeon_reward_from_str() {
        // Test medallion parsing
        assert_eq!(
            DungeonReward::from_str("Light Medallion"),
            Ok(DungeonReward::Medallion(Medallion::Light))
        );
        // Test stone parsing
        assert_eq!(
            DungeonReward::from_str("Kokiri Emerald"),
            Ok(DungeonReward::Stone(Stone::KokiriEmerald))
        );
    }

    #[test]
    fn test_dungeon_reward_display() {
        assert_eq!(
            DungeonReward::Medallion(Medallion::Forest).to_string(),
            "Forest Medallion"
        );
        assert_eq!(
            DungeonReward::Stone(Stone::GoronRuby).to_string(),
            "Goron Ruby"
        );
    }

    // DungeonRewardLocation tests
    #[test]
    fn test_dungeon_reward_location_as_str() {
        assert_eq!(DungeonRewardLocation::LinksPocket.as_str(), "Links Pocket");
        assert_eq!(
            DungeonRewardLocation::Dungeon(MainDungeon::DekuTree).as_str(),
            "Queen Gohma"
        );
    }

    #[test]
    fn test_dungeon_reward_location_from_str() {
        assert_eq!(
            DungeonRewardLocation::from_str("Links Pocket"),
            Ok(DungeonRewardLocation::LinksPocket)
        );
        assert_eq!(
            DungeonRewardLocation::from_str("Queen Gohma"),
            Ok(DungeonRewardLocation::Dungeon(MainDungeon::DekuTree))
        );
        assert_eq!(DungeonRewardLocation::from_str("Invalid"), Err(()));
    }

    #[test]
    fn test_dungeon_reward_location_display() {
        assert_eq!(
            DungeonRewardLocation::LinksPocket.to_string(),
            "Links Pocket"
        );
        assert_eq!(
            DungeonRewardLocation::Dungeon(MainDungeon::SpiritTemple).to_string(),
            "Twinrova"
        );
    }
}
