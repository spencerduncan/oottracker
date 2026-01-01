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
    /// Look up an MmItem by its string name.
    ///
    /// Supports both PascalCase variant names (e.g., "DekuMask") and
    /// snake_case names (e.g., "deku_mask").
    #[must_use]
    pub fn by_name(name: &str) -> Option<Self> {
        let item = match name {
            // Masks - Transformation
            "DekuMask" | "deku_mask" => Self::DekuMask,
            "GoronMask" | "goron_mask" => Self::GoronMask,
            "ZoraMask" | "zora_mask" => Self::ZoraMask,
            "FierceDeityMask" | "fierce_deity_mask" => Self::FierceDeityMask,
            // Masks - Regular
            "PostmanHat" | "postman_hat" => Self::PostmanHat,
            "AllNightMask" | "all_night_mask" => Self::AllNightMask,
            "BlastMask" | "blast_mask" => Self::BlastMask,
            "StoneMask" | "stone_mask" => Self::StoneMask,
            "GreatFairyMask" | "great_fairy_mask" => Self::GreatFairyMask,
            "KeatonMask" | "keaton_mask" => Self::KeatonMask,
            "BremenMask" | "bremen_mask" => Self::BremenMask,
            "BunnyHood" | "bunny_hood" => Self::BunnyHood,
            "DonGeroMask" | "don_gero_mask" => Self::DonGeroMask,
            "MaskOfScents" | "mask_of_scents" => Self::MaskOfScents,
            "RomaniMask" | "romani_mask" => Self::RomaniMask,
            "CircusLeaderMask" | "circus_leader_mask" => Self::CircusLeaderMask,
            "KafeiMask" | "kafei_mask" => Self::KafeiMask,
            "CouplesMask" | "couples_mask" => Self::CouplesMask,
            "MaskOfTruth" | "mask_of_truth" => Self::MaskOfTruth,
            "KamaroMask" | "kamaro_mask" => Self::KamaroMask,
            "GibdoMask" | "gibdo_mask" => Self::GibdoMask,
            "GaroMask" | "garo_mask" => Self::GaroMask,
            "CaptainHat" | "captain_hat" => Self::CaptainHat,
            "GiantMask" | "giant_mask" => Self::GiantMask,
            // Swords
            "KokiriSword" | "kokiri_sword" => Self::KokiriSword,
            "RazorSword" | "razor_sword" => Self::RazorSword,
            "GildedSword" | "gilded_sword" => Self::GildedSword,
            "GreatFairySword" | "great_fairy_sword" => Self::GreatFairySword,
            // Shields
            "HeroShield" | "hero_shield" => Self::HeroShield,
            "MirrorShield" | "mirror_shield" => Self::MirrorShield,
            // Equipment Items
            "HerosBow" | "heros_bow" => Self::HerosBow,
            "FireArrow" | "fire_arrow" => Self::FireArrow,
            "IceArrow" | "ice_arrow" => Self::IceArrow,
            "LightArrow" | "light_arrow" => Self::LightArrow,
            "Bomb" | "bomb" => Self::Bomb,
            "Bombchu" | "bombchu" => Self::Bombchu,
            "DekuStick" | "deku_stick" => Self::DekuStick,
            "DekuNut" | "deku_nut" => Self::DekuNut,
            "MagicBean" | "magic_bean" => Self::MagicBean,
            "PowderKeg" | "powder_keg" => Self::PowderKeg,
            "Hookshot" | "hookshot" => Self::Hookshot,
            "LensOfTruth" | "lens_of_truth" => Self::LensOfTruth,
            "PictographBox" | "pictograph_box" => Self::PictographBox,
            "OcarinaOfTime" | "ocarina_of_time" => Self::OcarinaOfTime,
            // Bottles
            "Bottle" | "bottle" => Self::Bottle,
            "BottleRedPotion" | "bottle_red_potion" => Self::BottleRedPotion,
            "BottleGreenPotion" | "bottle_green_potion" => Self::BottleGreenPotion,
            "BottleBluePotion" | "bottle_blue_potion" => Self::BottleBluePotion,
            "BottleFairy" | "bottle_fairy" => Self::BottleFairy,
            "BottleDekuPrincess" | "bottle_deku_princess" => Self::BottleDekuPrincess,
            "BottleFish" | "bottle_fish" => Self::BottleFish,
            "BottleBugs" | "bottle_bugs" => Self::BottleBugs,
            "BottlePoe" | "bottle_poe" => Self::BottlePoe,
            "BottleBigPoe" | "bottle_big_poe" => Self::BottleBigPoe,
            "BottleHotSpringWater" | "bottle_hot_spring_water" => Self::BottleHotSpringWater,
            "BottleZoraEgg" | "bottle_zora_egg" => Self::BottleZoraEgg,
            "BottleMushroom" | "bottle_mushroom" => Self::BottleMushroom,
            "BottleGoldDust" | "bottle_gold_dust" => Self::BottleGoldDust,
            "BottleMilk" | "bottle_milk" => Self::BottleMilk,
            "BottleHalfMilk" | "bottle_half_milk" => Self::BottleHalfMilk,
            "BottleChateau" | "bottle_chateau" => Self::BottleChateau,
            "BottleSeaHorse" | "bottle_sea_horse" => Self::BottleSeaHorse,
            // Songs
            "SongOfTime" | "song_of_time" => Self::SongOfTime,
            "SongOfHealing" | "song_of_healing" => Self::SongOfHealing,
            "EponasSong" | "eponas_song" => Self::EponasSong,
            "SongOfSoaring" | "song_of_soaring" => Self::SongOfSoaring,
            "SongOfStorms" | "song_of_storms" => Self::SongOfStorms,
            "SonataOfAwakening" | "sonata_of_awakening" => Self::SonataOfAwakening,
            "GoronLullaby" | "goron_lullaby" => Self::GoronLullaby,
            "NewWaveBossaNova" | "new_wave_bossa_nova" => Self::NewWaveBossaNova,
            "ElegyOfEmptiness" | "elegy_of_emptiness" => Self::ElegyOfEmptiness,
            "OathToOrder" | "oath_to_order" => Self::OathToOrder,
            // Upgrades
            "AdultWallet" | "adult_wallet" => Self::AdultWallet,
            "GiantWallet" | "giant_wallet" => Self::GiantWallet,
            "Quiver30" | "quiver_30" => Self::Quiver30,
            "Quiver40" | "quiver_40" => Self::Quiver40,
            "Quiver50" | "quiver_50" => Self::Quiver50,
            "BombBag20" | "bomb_bag_20" => Self::BombBag20,
            "BombBag30" | "bomb_bag_30" => Self::BombBag30,
            "BombBag40" | "bomb_bag_40" => Self::BombBag40,
            "MagicMeter" | "magic_meter" => Self::MagicMeter,
            "DoubleMagic" | "double_magic" => Self::DoubleMagic,
            "DoubleDefense" | "double_defense" => Self::DoubleDefense,
            // Quest Items
            "MoonsTear" | "moons_tear" => Self::MoonsTear,
            "LandTitleDeed" | "land_title_deed" => Self::LandTitleDeed,
            "SwampTitleDeed" | "swamp_title_deed" => Self::SwampTitleDeed,
            "MountainTitleDeed" | "mountain_title_deed" => Self::MountainTitleDeed,
            "OceanTitleDeed" | "ocean_title_deed" => Self::OceanTitleDeed,
            "RoomKey" | "room_key" => Self::RoomKey,
            "LetterToKafei" | "letter_to_kafei" => Self::LetterToKafei,
            "PendantOfMemories" | "pendant_of_memories" => Self::PendantOfMemories,
            "LetterToMama" | "letter_to_mama" => Self::LetterToMama,
            "SpecialDeliveryToMama" | "special_delivery_to_mama" => Self::SpecialDeliveryToMama,
            // Boss Remains
            "OdolwaRemains" | "odolwa_remains" => Self::OdolwaRemains,
            "GohtRemains" | "goht_remains" => Self::GohtRemains,
            "GyorgRemains" | "gyorg_remains" => Self::GyorgRemains,
            "TwinmoldRemains" | "twinmold_remains" => Self::TwinmoldRemains,
            // Dungeon Items
            "SmallKey" | "small_key" => Self::SmallKey,
            "BossKey" | "boss_key" => Self::BossKey,
            "Map" | "map" => Self::Map,
            "Compass" | "compass" => Self::Compass,
            "StrayFairy" | "stray_fairy" => Self::StrayFairy,
            // Dungeon-Specific Keys
            "SmallKeyWoodfallTemple" | "small_key_woodfall_temple" => Self::SmallKeyWoodfallTemple,
            "SmallKeySnowheadTemple" | "small_key_snowhead_temple" => Self::SmallKeySnowheadTemple,
            "SmallKeyGreatBayTemple" | "small_key_great_bay_temple" => Self::SmallKeyGreatBayTemple,
            "SmallKeyStoneTowerTemple" | "small_key_stone_tower_temple" => {
                Self::SmallKeyStoneTowerTemple
            }
            "BossKeyWoodfallTemple" | "boss_key_woodfall_temple" => Self::BossKeyWoodfallTemple,
            "BossKeySnowheadTemple" | "boss_key_snowhead_temple" => Self::BossKeySnowheadTemple,
            "BossKeyGreatBayTemple" | "boss_key_great_bay_temple" => Self::BossKeyGreatBayTemple,
            "BossKeyStoneTowerTemple" | "boss_key_stone_tower_temple" => {
                Self::BossKeyStoneTowerTemple
            }
            // Stray Fairies per dungeon
            "StrayFairyWoodfall" | "stray_fairy_woodfall" => Self::StrayFairyWoodfall,
            "StrayFairySnowhead" | "stray_fairy_snowhead" => Self::StrayFairySnowhead,
            "StrayFairyGreatBay" | "stray_fairy_great_bay" => Self::StrayFairyGreatBay,
            "StrayFairyStoneTower" | "stray_fairy_stone_tower" => Self::StrayFairyStoneTower,
            "StrayFairyClockTown" | "stray_fairy_clock_town" => Self::StrayFairyClockTown,
            // Collectibles
            "HeartContainer" | "heart_container" => Self::HeartContainer,
            "PieceOfHeart" | "piece_of_heart" => Self::PieceOfHeart,
            "GreenRupee" | "green_rupee" => Self::GreenRupee,
            "BlueRupee" | "blue_rupee" => Self::BlueRupee,
            "RedRupee" | "red_rupee" => Self::RedRupee,
            "PurpleRupee" | "purple_rupee" => Self::PurpleRupee,
            "SilverRupee" | "silver_rupee" => Self::SilverRupee,
            "GoldRupee" | "gold_rupee" => Self::GoldRupee,
            // Notebook Events
            "BomberNotebook" | "bomber_notebook" => Self::BomberNotebook,
            // Special
            "GiantsWallet" | "giants_wallet" => Self::GiantsWallet,
            "OceanTitleDeedTraded" | "ocean_title_deed_traded" => Self::OceanTitleDeedTraded,
            _ => return None,
        };
        Some(item)
    }

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

    #[test]
    fn test_by_name_pascal_case() {
        assert_eq!(MmItem::by_name("DekuMask"), Some(MmItem::DekuMask));
        assert_eq!(MmItem::by_name("Hookshot"), Some(MmItem::Hookshot));
        assert_eq!(
            MmItem::by_name("OdolwaRemains"),
            Some(MmItem::OdolwaRemains)
        );
        assert_eq!(
            MmItem::by_name("SmallKeyWoodfallTemple"),
            Some(MmItem::SmallKeyWoodfallTemple)
        );
    }

    #[test]
    fn test_by_name_snake_case() {
        assert_eq!(MmItem::by_name("deku_mask"), Some(MmItem::DekuMask));
        assert_eq!(MmItem::by_name("hookshot"), Some(MmItem::Hookshot));
        assert_eq!(
            MmItem::by_name("odolwa_remains"),
            Some(MmItem::OdolwaRemains)
        );
        assert_eq!(
            MmItem::by_name("small_key_woodfall_temple"),
            Some(MmItem::SmallKeyWoodfallTemple)
        );
    }

    #[test]
    fn test_by_name_not_found() {
        assert_eq!(MmItem::by_name("NotAnItem"), None);
        assert_eq!(MmItem::by_name(""), None);
        assert_eq!(MmItem::by_name("invalid_item"), None);
    }
}
