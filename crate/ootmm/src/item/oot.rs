//! Ocarina of Time items.

/// OoT item enum - all trackable items from Ocarina of Time.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(missing_docs)]
pub enum OotItem {
    // Swords
    KokiriSword,
    MasterSword,
    BiggoronSword,
    GiantKnife,

    // Shields
    DekuShield,
    HylianShield,
    MirrorShield,

    // Tunics
    KokiriTunic,
    GoronTunic,
    ZoraTunic,

    // Boots
    KokiriBoots,
    IronBoots,
    HoverBoots,

    // Equipment Items
    DekuStick,
    DekuNut,
    Bomb,
    Bow,
    FireArrow,
    IceArrow,
    LightArrow,
    DinsFire,
    FaroresWind,
    NayrusLove,
    Slingshot,
    Boomerang,
    Hookshot,
    Longshot,
    LensOfTruth,
    MegatonHammer,
    OcarinaOfTime,

    // C-Button Items
    Bottle,
    BottleRedPotion,
    BottleGreenPotion,
    BottleBluePotion,
    BottleFairy,
    BottleFish,
    BottleBlueFire,
    BottleBugs,
    BottlePoe,
    BottleBigPoe,
    BottleMilk,
    BottleHalfMilk,
    BottleRutosLetter,

    // Adult Trade Sequence
    PocketEgg,
    PocketCucco,
    Cojiro,
    OddMushroom,
    OddPotion,
    PoachersSaw,
    BrokenSword,
    Prescription,
    EyeballFrog,
    Eyedrops,
    ClaimCheck,

    // Child Trade Sequence
    WeirdEgg,
    Chicken,
    ZeldasLetter,
    SkullMask,
    SpookyMask,
    KeatonMask,
    BunnyHood,
    GoronMask,
    ZoraMask,
    GerudoMask,
    MaskOfTruth,

    // Songs
    ZeldasLullaby,
    EponasSong,
    SariasSong,
    SunsSong,
    SongOfTime,
    SongOfStorms,
    MinuetOfForest,
    BoleroOfFire,
    SerenadeOfWater,
    NocturneOfShadow,
    RequiemOfSpirit,
    PreludeOfLight,
    ScarecrowSong,

    // Upgrades
    GoronBracelet,
    SilverGauntlets,
    GoldenGauntlets,
    SilverScale,
    GoldenScale,
    ChildWallet,
    AdultWallet,
    GiantWallet,
    DekuStickCapacity20,
    DekuStickCapacity30,
    DekuNutCapacity30,
    DekuNutCapacity40,
    BulletBag30,
    BulletBag40,
    BulletBag50,
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
    KokiriEmerald,
    GoronRuby,
    ZoraSapphire,
    ForestMedallion,
    FireMedallion,
    WaterMedallion,
    ShadowMedallion,
    SpiritMedallion,
    LightMedallion,
    StoneOfAgony,
    GerudoCard,

    // Dungeon Items
    SmallKey,
    BossKey,
    Map,
    Compass,

    // Dungeon-Specific Keys (for tracking)
    SmallKeyForestTemple,
    SmallKeyFireTemple,
    SmallKeyWaterTemple,
    SmallKeyShadowTemple,
    SmallKeySpiritTemple,
    SmallKeyBottomOfTheWell,
    SmallKeyGerudoFortress,
    SmallKeyGerudoTrainingGround,
    SmallKeyGanonsCastle,
    BossKeyForestTemple,
    BossKeyFireTemple,
    BossKeyWaterTemple,
    BossKeyShadowTemple,
    BossKeySpiritTemple,
    BossKeyGanonsCastle,

    // Collectibles
    HeartContainer,
    PieceOfHeart,
    GoldSkulltula,
    SmallMagicJar,
    LargeMagicJar,
    RecoveryHeart,
    GreenRupee,
    BlueRupee,
    RedRupee,
    PurpleRupee,
    GoldRupee,

    // Special
    Triforce,
    TriforceOfCourage,
    GanonBossKey,
}

impl OotItem {
    /// Returns true if this is a progressive item that can be collected multiple times.
    #[must_use]
    pub const fn is_progressive(&self) -> bool {
        matches!(
            self,
            Self::Bomb
                | Self::DekuStick
                | Self::DekuNut
                | Self::Bottle
                | Self::SmallKey
                | Self::HeartContainer
                | Self::PieceOfHeart
                | Self::GoldSkulltula
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
                | Self::SmallKeyForestTemple
                | Self::SmallKeyFireTemple
                | Self::SmallKeyWaterTemple
                | Self::SmallKeyShadowTemple
                | Self::SmallKeySpiritTemple
                | Self::SmallKeyBottomOfTheWell
                | Self::SmallKeyGerudoFortress
                | Self::SmallKeyGerudoTrainingGround
                | Self::SmallKeyGanonsCastle
                | Self::BossKeyForestTemple
                | Self::BossKeyFireTemple
                | Self::BossKeyWaterTemple
                | Self::BossKeyShadowTemple
                | Self::BossKeySpiritTemple
                | Self::BossKeyGanonsCastle
        )
    }

    /// Returns true if this is a song.
    #[must_use]
    pub const fn is_song(&self) -> bool {
        matches!(
            self,
            Self::ZeldasLullaby
                | Self::EponasSong
                | Self::SariasSong
                | Self::SunsSong
                | Self::SongOfTime
                | Self::SongOfStorms
                | Self::MinuetOfForest
                | Self::BoleroOfFire
                | Self::SerenadeOfWater
                | Self::NocturneOfShadow
                | Self::RequiemOfSpirit
                | Self::PreludeOfLight
                | Self::ScarecrowSong
        )
    }
}

#[cfg(test)]
mod tests {
    use super::OotItem;

    #[test]
    fn test_progressive_items() {
        assert!(OotItem::Bomb.is_progressive());
        assert!(OotItem::HeartContainer.is_progressive());
        assert!(!OotItem::MasterSword.is_progressive());
        assert!(!OotItem::Hookshot.is_progressive());
    }

    #[test]
    fn test_dungeon_items() {
        assert!(OotItem::SmallKey.is_dungeon_item());
        assert!(OotItem::BossKeyFireTemple.is_dungeon_item());
        assert!(!OotItem::Hookshot.is_dungeon_item());
    }

    #[test]
    fn test_songs() {
        assert!(OotItem::ZeldasLullaby.is_song());
        assert!(OotItem::BoleroOfFire.is_song());
        assert!(!OotItem::Hookshot.is_song());
    }
}
