//! Majora's Mask items.

/// MM item enum - all trackable items from Majora's Mask.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(missing_docs)]
pub enum MmItem {
    // Masks - Transformation
    DekuMask,
    GoronMask,
    ZoraMask,
    FierceDeityMask,

    // Masks - Regular
    PostmanHat,
    AllNightMask,
    BlastMask,
    StoneMask,
    GreatFairyMask,
    KeatonMask,
    BremenMask,
    BunnyHood,
    DonGeroMask,
    MaskOfScents,
    RomaniMask,
    CircusLeaderMask,
    KafeiMask,
    CouplesMask,
    MaskOfTruth,
    KamaroMask,
    GibdoMask,
    GaroMask,
    CaptainHat,
    GiantMask,

    // Swords
    KokiriSword,
    RazorSword,
    GildedSword,
    GreatFairySword,

    // Shields
    HeroShield,
    MirrorShield,

    // Equipment Items
    HerosBow,
    FireArrow,
    IceArrow,
    LightArrow,
    Bomb,
    Bombchu,
    DekuStick,
    DekuNut,
    MagicBean,
    PowderKeg,
    Hookshot,
    LensOfTruth,
    PictographBox,
    OcarinaOfTime,

    // Bottles
    Bottle,
    BottleRedPotion,
    BottleGreenPotion,
    BottleBluePotion,
    BottleFairy,
    BottleDekuPrincess,
    BottleFish,
    BottleBugs,
    BottlePoe,
    BottleBigPoe,
    BottleHotSpringWater,
    BottleZoraEgg,
    BottleMushroom,
    BottleGoldDust,
    BottleMilk,
    BottleHalfMilk,
    BottleChateau,
    BottleSeaHorse,

    // Songs
    SongOfTime,
    SongOfHealing,
    EponasSong,
    SongOfSoaring,
    SongOfStorms,
    SonataOfAwakening,
    GoronLullaby,
    NewWaveBossaNova,
    ElegyOfEmptiness,
    OathToOrder,

    // Upgrades
    AdultWallet,
    GiantWallet,
    Quiver30,
    Quiver40,
    Quiver50,
    BombBag20,
    BombBag30,
    BombBag40,
    MagicMeter,
    DoubleMagic,
    DoubleDefense,

    // Quest Items
    MoonsTear,
    LandTitleDeed,
    SwampTitleDeed,
    MountainTitleDeed,
    OceanTitleDeed,
    RoomKey,
    LetterToKafei,
    PendantOfMemories,
    LetterToMama,
    SpecialDeliveryToMama,

    // Boss Remains
    OdolwaRemains,
    GohtRemains,
    GyorgRemains,
    TwinmoldRemains,

    // Dungeon Items
    SmallKey,
    BossKey,
    Map,
    Compass,
    StrayFairy,

    // Dungeon-Specific Keys
    SmallKeyWoodfallTemple,
    SmallKeySnowheadTemple,
    SmallKeyGreatBayTemple,
    SmallKeyStoneTowerTemple,
    BossKeyWoodfallTemple,
    BossKeySnowheadTemple,
    BossKeyGreatBayTemple,
    BossKeyStoneTowerTemple,

    // Stray Fairies per dungeon
    StrayFairyWoodfall,
    StrayFairySnowhead,
    StrayFairyGreatBay,
    StrayFairyStoneTower,
    StrayFairyClockTown,

    // Collectibles
    HeartContainer,
    PieceOfHeart,
    GreenRupee,
    BlueRupee,
    RedRupee,
    PurpleRupee,
    SilverRupee,
    GoldRupee,

    // Notebook Events (for tracking)
    BomberNotebook,

    // Special
    GiantsWallet,
    OceanTitleDeedTraded,
}

impl MmItem {
    /// Returns true if this is a transformation mask.
    #[must_use]
    pub const fn is_transformation_mask(&self) -> bool {
        matches!(
            self,
            Self::DekuMask | Self::GoronMask | Self::ZoraMask | Self::FierceDeityMask
        )
    }

    /// Returns true if this is any mask.
    #[must_use]
    pub const fn is_mask(&self) -> bool {
        matches!(
            self,
            Self::DekuMask
                | Self::GoronMask
                | Self::ZoraMask
                | Self::FierceDeityMask
                | Self::PostmanHat
                | Self::AllNightMask
                | Self::BlastMask
                | Self::StoneMask
                | Self::GreatFairyMask
                | Self::KeatonMask
                | Self::BremenMask
                | Self::BunnyHood
                | Self::DonGeroMask
                | Self::MaskOfScents
                | Self::RomaniMask
                | Self::CircusLeaderMask
                | Self::KafeiMask
                | Self::CouplesMask
                | Self::MaskOfTruth
                | Self::KamaroMask
                | Self::GibdoMask
                | Self::GaroMask
                | Self::CaptainHat
                | Self::GiantMask
        )
    }

    /// Returns true if this is a progressive item.
    #[must_use]
    pub const fn is_progressive(&self) -> bool {
        matches!(
            self,
            Self::Bomb
                | Self::DekuStick
                | Self::DekuNut
                | Self::Bottle
                | Self::SmallKey
                | Self::StrayFairy
                | Self::HeartContainer
                | Self::PieceOfHeart
        )
    }

    /// Returns true if this is a boss remain.
    #[must_use]
    pub const fn is_boss_remain(&self) -> bool {
        matches!(
            self,
            Self::OdolwaRemains | Self::GohtRemains | Self::GyorgRemains | Self::TwinmoldRemains
        )
    }

    /// Returns true if this is a song.
    #[must_use]
    pub const fn is_song(&self) -> bool {
        matches!(
            self,
            Self::SongOfTime
                | Self::SongOfHealing
                | Self::EponasSong
                | Self::SongOfSoaring
                | Self::SongOfStorms
                | Self::SonataOfAwakening
                | Self::GoronLullaby
                | Self::NewWaveBossaNova
                | Self::ElegyOfEmptiness
                | Self::OathToOrder
        )
    }

    /// Returns true if this is a dungeon item.
    #[must_use]
    pub const fn is_dungeon_item(&self) -> bool {
        matches!(
            self,
            Self::SmallKey
                | Self::BossKey
                | Self::Map
                | Self::Compass
                | Self::StrayFairy
                | Self::SmallKeyWoodfallTemple
                | Self::SmallKeySnowheadTemple
                | Self::SmallKeyGreatBayTemple
                | Self::SmallKeyStoneTowerTemple
                | Self::BossKeyWoodfallTemple
                | Self::BossKeySnowheadTemple
                | Self::BossKeyGreatBayTemple
                | Self::BossKeyStoneTowerTemple
        )
    }
}

#[cfg(test)]
mod tests {
    use super::MmItem;

    #[test]
    fn test_transformation_masks() {
        assert!(MmItem::DekuMask.is_transformation_mask());
        assert!(MmItem::GoronMask.is_transformation_mask());
        assert!(MmItem::ZoraMask.is_transformation_mask());
        assert!(MmItem::FierceDeityMask.is_transformation_mask());
        assert!(!MmItem::BunnyHood.is_transformation_mask());
    }

    #[test]
    fn test_all_masks() {
        assert!(MmItem::DekuMask.is_mask());
        assert!(MmItem::BunnyHood.is_mask());
        assert!(MmItem::KeatonMask.is_mask());
        assert!(!MmItem::Hookshot.is_mask());
    }

    #[test]
    fn test_boss_remains() {
        assert!(MmItem::OdolwaRemains.is_boss_remain());
        assert!(MmItem::TwinmoldRemains.is_boss_remain());
        assert!(!MmItem::Hookshot.is_boss_remain());
    }

    #[test]
    fn test_songs() {
        assert!(MmItem::SongOfHealing.is_song());
        assert!(MmItem::OathToOrder.is_song());
        assert!(!MmItem::Hookshot.is_song());
    }

    #[test]
    fn test_progressive() {
        assert!(MmItem::Bomb.is_progressive());
        assert!(MmItem::StrayFairy.is_progressive());
        assert!(!MmItem::Hookshot.is_progressive());
    }
}
