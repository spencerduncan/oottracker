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

    // MainDungeon exhaustive roundtrip test
    #[test]
    fn test_main_dungeon_display_from_str_roundtrip() {
        // Test that Display -> FromStr roundtrips for all MainDungeon variants
        for dungeon in enum_iterator::all::<MainDungeon>() {
            let display_str = dungeon.to_string();
            let parsed = MainDungeon::from_str(&display_str);
            assert_eq!(
                parsed,
                Ok(dungeon),
                "Roundtrip failed for {:?}: '{}' did not parse back",
                dungeon,
                display_str
            );
        }
    }

    // Medallion TryFrom<Item> tests
    #[test]
    fn test_medallion_try_from_item_valid() {
        for medallion in enum_iterator::all::<Medallion>() {
            let item: Item = medallion.into();
            let result = Medallion::try_from(item);
            assert_eq!(
                result,
                Ok(medallion),
                "TryFrom<Item> failed for {:?}",
                medallion
            );
        }
    }

    #[test]
    fn test_medallion_try_from_item_invalid() {
        let invalid_items = [
            Item("Invalid Item".to_string()),
            Item("Light".to_string()), // Missing "Medallion"
            Item("".to_string()),
            Item("Kokiri Emerald".to_string()), // A stone, not a medallion
        ];

        for item in &invalid_items {
            let result = Medallion::try_from(item.clone());
            assert!(
                result.is_err(),
                "Expected error for item '{}', got {:?}",
                item.name(),
                result
            );
        }
    }

    // Stone TryFrom<Item> tests
    #[test]
    fn test_stone_try_from_item_valid() {
        for stone in enum_iterator::all::<Stone>() {
            let item: Item = stone.into();
            let result = Stone::try_from(item);
            assert_eq!(result, Ok(stone), "TryFrom<Item> failed for {:?}", stone);
        }
    }

    #[test]
    fn test_stone_try_from_item_invalid() {
        let invalid_items = [
            Item("Invalid Item".to_string()),
            Item("Kokiri".to_string()), // Partial name
            Item("".to_string()),
            Item("Light Medallion".to_string()), // A medallion, not a stone
        ];

        for item in &invalid_items {
            let result = Stone::try_from(item.clone());
            assert!(
                result.is_err(),
                "Expected error for item '{}', got {:?}",
                item.name(),
                result
            );
        }
    }

    // DungeonReward TryFrom<Item> tests
    #[test]
    fn test_dungeon_reward_try_from_item_valid() {
        for reward in enum_iterator::all::<DungeonReward>() {
            let item: Item = reward.into();
            let result = DungeonReward::try_from(item);
            assert_eq!(result, Ok(reward), "TryFrom<Item> failed for {:?}", reward);
        }
    }

    #[test]
    fn test_dungeon_reward_try_from_item_invalid() {
        let invalid_items = [
            Item("Invalid Item".to_string()),
            Item("Light".to_string()),
            Item("".to_string()),
            Item("Hookshot".to_string()),
        ];

        for item in &invalid_items {
            let result = DungeonReward::try_from(item.clone());
            assert!(
                result.is_err(),
                "Expected error for item '{}', got {:?}",
                item.name(),
                result
            );
        }
    }

    // DungeonReward roundtrip tests
    #[test]
    fn test_dungeon_reward_display_from_str_roundtrip() {
        for reward in enum_iterator::all::<DungeonReward>() {
            let display_str = reward.to_string();
            let parsed = DungeonReward::from_str(&display_str);
            assert_eq!(
                parsed,
                Ok(reward),
                "Roundtrip failed for {:?}: '{}' did not parse back",
                reward,
                display_str
            );
        }
    }

    // Item conversion roundtrip tests
    #[test]
    fn test_medallion_to_item_roundtrip() {
        for medallion in enum_iterator::all::<Medallion>() {
            let item: Item = medallion.into();
            let back: Result<Medallion, ()> = Medallion::try_from(item.clone());
            assert_eq!(
                back,
                Ok(medallion),
                "Item roundtrip failed for {:?} -> {:?}",
                medallion,
                item
            );
        }
    }

    #[test]
    fn test_stone_to_item_roundtrip() {
        for stone in enum_iterator::all::<Stone>() {
            let item: Item = stone.into();
            let back: Result<Stone, ()> = Stone::try_from(item.clone());
            assert_eq!(
                back,
                Ok(stone),
                "Item roundtrip failed for {:?} -> {:?}",
                stone,
                item
            );
        }
    }

    #[test]
    fn test_dungeon_reward_to_item_roundtrip() {
        for reward in enum_iterator::all::<DungeonReward>() {
            let item: Item = reward.into();
            let back: Result<DungeonReward, ()> = DungeonReward::try_from(item.clone());
            assert_eq!(
                back,
                Ok(reward),
                "Item roundtrip failed for {:?} -> {:?}",
                reward,
                item
            );
        }
    }

    // Dungeon exhaustive tests
    #[test]
    fn test_dungeon_from_str_invalid() {
        assert_eq!(Dungeon::from_str("Invalid Dungeon"), Err(()));
        assert_eq!(Dungeon::from_str(""), Err(()));
        assert_eq!(Dungeon::from_str("Temple"), Err(()));
    }

    // DungeonRewardLocation exhaustive roundtrip
    #[test]
    fn test_dungeon_reward_location_roundtrip() {
        // Test Links Pocket
        let links_pocket = DungeonRewardLocation::LinksPocket;
        let parsed = DungeonRewardLocation::from_str(links_pocket.as_str());
        assert_eq!(parsed, Ok(links_pocket));

        // Test all dungeon locations
        for dungeon in enum_iterator::all::<MainDungeon>() {
            let location = DungeonRewardLocation::Dungeon(dungeon);
            let as_str = location.as_str();
            let parsed = DungeonRewardLocation::from_str(as_str);
            assert_eq!(
                parsed,
                Ok(location),
                "Roundtrip failed for {:?}: '{}' did not parse back",
                location,
                as_str
            );
        }
    }

    // Medallion exhaustive roundtrip
    #[test]
    fn test_medallion_display_from_str_roundtrip() {
        for medallion in enum_iterator::all::<Medallion>() {
            let display_str = medallion.to_string();
            let parsed = Medallion::from_str(&display_str);
            assert_eq!(
                parsed,
                Ok(medallion),
                "Roundtrip failed for {:?}: '{}' did not parse back",
                medallion,
                display_str
            );
        }
    }

    // Stone exhaustive roundtrip
    #[test]
    fn test_stone_display_from_str_roundtrip() {
        for stone in enum_iterator::all::<Stone>() {
            let display_str = stone.to_string();
            let parsed = Stone::from_str(&display_str);
            assert_eq!(
                parsed,
                Ok(stone),
                "Roundtrip failed for {:?}: '{}' did not parse back",
                stone,
                display_str
            );
        }
    }

    // ===== SEMANTIC EQUIVALENCE ROUNDTRIP TESTS =====
    //
    // These tests verify that roundtripped model types preserve semantic properties,
    // not just structural equality. The types should behave identically after
    // Display -> FromStr conversion.

    /// Verifies that a Medallion preserves its Item conversion after roundtrip.
    fn verify_medallion_semantic_roundtrip(medallion: Medallion) {
        let display_str = medallion.to_string();
        let parsed = Medallion::from_str(&display_str).unwrap();

        // Structural check
        assert_eq!(medallion, parsed);

        // Semantic checks - verify Item conversion is preserved
        let original_item: Item = medallion.into();
        let roundtrip_item: Item = parsed.into();
        assert_eq!(
            original_item.name(),
            roundtrip_item.name(),
            "Item name mismatch after roundtrip for {:?}",
            medallion
        );

        // Verify conversion back to Medallion works
        let back_from_original = Medallion::try_from(original_item.clone());
        let back_from_roundtrip = Medallion::try_from(roundtrip_item.clone());
        assert_eq!(
            back_from_original, back_from_roundtrip,
            "Medallion conversion mismatch for {:?}",
            medallion
        );
    }

    /// Verifies that a Stone preserves its Item conversion after roundtrip.
    fn verify_stone_semantic_roundtrip(stone: Stone) {
        let display_str = stone.to_string();
        let parsed = Stone::from_str(&display_str).unwrap();

        // Structural check
        assert_eq!(stone, parsed);

        // Semantic checks - verify Item conversion is preserved
        let original_item: Item = stone.into();
        let roundtrip_item: Item = parsed.into();
        assert_eq!(
            original_item.name(),
            roundtrip_item.name(),
            "Item name mismatch after roundtrip for {:?}",
            stone
        );

        // Verify conversion back to Stone works
        let back_from_original = Stone::try_from(original_item.clone());
        let back_from_roundtrip = Stone::try_from(roundtrip_item.clone());
        assert_eq!(
            back_from_original, back_from_roundtrip,
            "Stone conversion mismatch for {:?}",
            stone
        );
    }

    /// Verifies that a DungeonReward preserves its Item conversion after roundtrip.
    fn verify_dungeon_reward_semantic_roundtrip(reward: DungeonReward) {
        let display_str = reward.to_string();
        let parsed = DungeonReward::from_str(&display_str).unwrap();

        // Structural check
        assert_eq!(reward, parsed);

        // Semantic checks - verify Item conversion is preserved
        let original_item: Item = reward.into();
        let roundtrip_item: Item = parsed.into();
        assert_eq!(
            original_item.name(),
            roundtrip_item.name(),
            "Item name mismatch after roundtrip for {:?}",
            reward
        );

        // Verify conversion back to DungeonReward works
        let back_from_original = DungeonReward::try_from(original_item.clone());
        let back_from_roundtrip = DungeonReward::try_from(roundtrip_item.clone());
        assert_eq!(
            back_from_original, back_from_roundtrip,
            "DungeonReward conversion mismatch for {:?}",
            reward
        );
    }

    #[test]
    fn test_medallion_semantic_roundtrip_all() {
        for medallion in enum_iterator::all::<Medallion>() {
            verify_medallion_semantic_roundtrip(medallion);
        }
    }

    #[test]
    fn test_stone_semantic_roundtrip_all() {
        for stone in enum_iterator::all::<Stone>() {
            verify_stone_semantic_roundtrip(stone);
        }
    }

    #[test]
    fn test_dungeon_reward_semantic_roundtrip_all() {
        for reward in enum_iterator::all::<DungeonReward>() {
            verify_dungeon_reward_semantic_roundtrip(reward);
        }
    }

    #[test]
    fn test_main_dungeon_reward_location_semantic_roundtrip() {
        // MainDungeon -> DungeonRewardLocation -> string -> back
        for dungeon in enum_iterator::all::<MainDungeon>() {
            let location = DungeonRewardLocation::Dungeon(dungeon);
            let location_str = location.as_str();
            let parsed = DungeonRewardLocation::from_str(location_str).unwrap();

            assert_eq!(location, parsed);

            // Verify the dungeon can still be extracted correctly
            match parsed {
                DungeonRewardLocation::Dungeon(d) => assert_eq!(dungeon, d),
                DungeonRewardLocation::LinksPocket => {
                    panic!("Expected Dungeon, got LinksPocket")
                }
            }
        }
    }

    // ===== EDGE CASES AND NORMALIZATION TESTS =====

    #[test]
    fn test_dungeon_reward_location_links_pocket_semantic() {
        let location = DungeonRewardLocation::LinksPocket;
        let location_str = location.as_str();
        let parsed = DungeonRewardLocation::from_str(location_str).unwrap();

        assert_eq!(location, parsed);

        // Verify it's recognized as LinksPocket, not a dungeon
        assert!(matches!(parsed, DungeonRewardLocation::LinksPocket));
    }

    #[test]
    fn test_main_dungeon_display_string_stability() {
        // Verify that display strings are stable across multiple roundtrips
        for dungeon in enum_iterator::all::<MainDungeon>() {
            let str1 = dungeon.to_string();
            let parsed1 = MainDungeon::from_str(&str1).unwrap();
            let str2 = parsed1.to_string();
            let parsed2 = MainDungeon::from_str(&str2).unwrap();
            let str3 = parsed2.to_string();

            // All strings should be identical
            assert_eq!(
                str1, str2,
                "First roundtrip changed display for {:?}",
                dungeon
            );
            assert_eq!(
                str2, str3,
                "Second roundtrip changed display for {:?}",
                dungeon
            );
        }
    }

    #[test]
    fn test_dungeon_reward_double_roundtrip() {
        // Verify semantic equivalence after multiple roundtrips
        for reward in enum_iterator::all::<DungeonReward>() {
            let str1 = reward.to_string();
            let rt1 = DungeonReward::from_str(&str1).unwrap();
            let str2 = rt1.to_string();
            let rt2 = DungeonReward::from_str(&str2).unwrap();

            // Item conversions should be identical at all stages
            let item_original: Item = reward.into();
            let item_rt1: Item = rt1.into();
            let item_rt2: Item = rt2.into();

            assert_eq!(item_original.name(), item_rt1.name());
            assert_eq!(item_original.name(), item_rt2.name());
        }
    }

    #[test]
    fn test_medallion_distinguishable_after_roundtrip() {
        // Ensure all medallions remain distinguishable after roundtrip
        let roundtripped: Vec<_> = enum_iterator::all::<Medallion>()
            .map(|m| {
                let s = m.to_string();
                Medallion::from_str(&s).unwrap()
            })
            .collect();

        // All should be unique
        for (i, m1) in roundtripped.iter().enumerate() {
            for (j, m2) in roundtripped.iter().enumerate() {
                if i != j {
                    assert_ne!(m1, m2, "Medallions at {} and {} should be different", i, j);
                }
            }
        }
    }

    #[test]
    fn test_stone_distinguishable_after_roundtrip() {
        // Ensure all stones remain distinguishable after roundtrip
        let roundtripped: Vec<_> = enum_iterator::all::<Stone>()
            .map(|s| {
                let str = s.to_string();
                Stone::from_str(&str).unwrap()
            })
            .collect();

        // All should be unique
        for (i, s1) in roundtripped.iter().enumerate() {
            for (j, s2) in roundtripped.iter().enumerate() {
                if i != j {
                    assert_ne!(s1, s2, "Stones at {} and {} should be different", i, j);
                }
            }
        }
    }

    #[test]
    fn test_dungeon_reward_type_preservation() {
        // Verify that Medallion/Stone type is preserved through DungeonReward roundtrip
        for medallion in enum_iterator::all::<Medallion>() {
            let reward = DungeonReward::Medallion(medallion);
            let str = reward.to_string();
            let parsed = DungeonReward::from_str(&str).unwrap();

            match parsed {
                DungeonReward::Medallion(m) => assert_eq!(medallion, m),
                DungeonReward::Stone(_) => {
                    panic!("Expected Medallion, got Stone after roundtrip")
                }
            }
        }

        for stone in enum_iterator::all::<Stone>() {
            let reward = DungeonReward::Stone(stone);
            let str = reward.to_string();
            let parsed = DungeonReward::from_str(&str).unwrap();

            match parsed {
                DungeonReward::Stone(s) => assert_eq!(stone, s),
                DungeonReward::Medallion(_) => {
                    panic!("Expected Stone, got Medallion after roundtrip")
                }
            }
        }
    }
}
