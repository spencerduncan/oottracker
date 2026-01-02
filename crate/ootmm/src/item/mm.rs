//! Majora's Mask items.

use crate::item::ItemCategory;

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

    /// Returns the category of this item.
    #[must_use]
    pub const fn category(&self) -> ItemCategory {
        match self {
            // Transformation Masks
            Self::DekuMask | Self::GoronMask | Self::ZoraMask | Self::FierceDeityMask => {
                ItemCategory::TransformationMask
            }

            // Regular Masks
            Self::PostmanHat
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
            | Self::GiantMask => ItemCategory::Mask,

            // Swords
            Self::KokiriSword | Self::RazorSword | Self::GildedSword | Self::GreatFairySword => {
                ItemCategory::Sword
            }

            // Shields
            Self::HeroShield | Self::MirrorShield => ItemCategory::Shield,

            // Equipment
            Self::HerosBow
            | Self::Bomb
            | Self::Bombchu
            | Self::DekuStick
            | Self::DekuNut
            | Self::MagicBean
            | Self::PowderKeg
            | Self::Hookshot
            | Self::LensOfTruth
            | Self::PictographBox => ItemCategory::Equipment,

            // Arrows (Equipment)
            Self::FireArrow | Self::IceArrow | Self::LightArrow => ItemCategory::Equipment,

            // Ocarina
            Self::OcarinaOfTime => ItemCategory::Ocarina,

            // Bottles and contents
            Self::Bottle
            | Self::BottleRedPotion
            | Self::BottleGreenPotion
            | Self::BottleBluePotion
            | Self::BottleFairy
            | Self::BottleDekuPrincess
            | Self::BottleFish
            | Self::BottleBugs
            | Self::BottlePoe
            | Self::BottleBigPoe
            | Self::BottleHotSpringWater
            | Self::BottleZoraEgg
            | Self::BottleMushroom
            | Self::BottleGoldDust
            | Self::BottleMilk
            | Self::BottleHalfMilk
            | Self::BottleChateau
            | Self::BottleSeaHorse => ItemCategory::Bottle,

            // Songs
            Self::SongOfTime
            | Self::SongOfHealing
            | Self::EponasSong
            | Self::SongOfSoaring
            | Self::SongOfStorms
            | Self::SonataOfAwakening
            | Self::GoronLullaby
            | Self::NewWaveBossaNova
            | Self::ElegyOfEmptiness
            | Self::OathToOrder => ItemCategory::Song,

            // Upgrades
            Self::AdultWallet
            | Self::GiantWallet
            | Self::GiantsWallet
            | Self::Quiver30
            | Self::Quiver40
            | Self::Quiver50
            | Self::BombBag20
            | Self::BombBag30
            | Self::BombBag40
            | Self::MagicMeter
            | Self::DoubleMagic
            | Self::DoubleDefense => ItemCategory::Upgrade,

            // Quest Items (Title Deeds, Letters, etc.)
            Self::MoonsTear
            | Self::LandTitleDeed
            | Self::SwampTitleDeed
            | Self::MountainTitleDeed
            | Self::OceanTitleDeed
            | Self::OceanTitleDeedTraded
            | Self::RoomKey
            | Self::LetterToKafei
            | Self::PendantOfMemories
            | Self::LetterToMama
            | Self::SpecialDeliveryToMama => ItemCategory::Trade,

            // Boss Remains (Quest Items)
            Self::OdolwaRemains
            | Self::GohtRemains
            | Self::GyorgRemains
            | Self::TwinmoldRemains => ItemCategory::QuestItem,

            // Generic Dungeon Items
            Self::Map | Self::Compass => ItemCategory::DungeonItem,

            // Small Keys
            Self::SmallKey
            | Self::SmallKeyWoodfallTemple
            | Self::SmallKeySnowheadTemple
            | Self::SmallKeyGreatBayTemple
            | Self::SmallKeyStoneTowerTemple => ItemCategory::SmallKey,

            // Boss Keys
            Self::BossKey
            | Self::BossKeyWoodfallTemple
            | Self::BossKeySnowheadTemple
            | Self::BossKeyGreatBayTemple
            | Self::BossKeyStoneTowerTemple => ItemCategory::BossKey,

            // Stray Fairies (Tokens)
            Self::StrayFairy
            | Self::StrayFairyWoodfall
            | Self::StrayFairySnowhead
            | Self::StrayFairyGreatBay
            | Self::StrayFairyStoneTower
            | Self::StrayFairyClockTown => ItemCategory::Token,

            // Collectibles/Consumables
            Self::HeartContainer
            | Self::PieceOfHeart
            | Self::GreenRupee
            | Self::BlueRupee
            | Self::RedRupee
            | Self::PurpleRupee
            | Self::SilverRupee
            | Self::GoldRupee => ItemCategory::Consumable,

            // Special
            Self::BomberNotebook => ItemCategory::Special,
        }
    }

    /// Returns true if this item can stack (be collected multiple times).
    #[must_use]
    pub const fn is_stackable(&self) -> bool {
        matches!(
            self,
            Self::Bomb
                | Self::DekuStick
                | Self::DekuNut
                | Self::Bottle
                | Self::SmallKey
                | Self::SmallKeyWoodfallTemple
                | Self::SmallKeySnowheadTemple
                | Self::SmallKeyGreatBayTemple
                | Self::SmallKeyStoneTowerTemple
                | Self::StrayFairy
                | Self::StrayFairyWoodfall
                | Self::StrayFairySnowhead
                | Self::StrayFairyGreatBay
                | Self::StrayFairyStoneTower
                | Self::StrayFairyClockTown
                | Self::HeartContainer
                | Self::PieceOfHeart
                | Self::GreenRupee
                | Self::BlueRupee
                | Self::RedRupee
                | Self::PurpleRupee
                | Self::SilverRupee
                | Self::GoldRupee
        )
    }

    /// Returns the maximum count for this item.
    /// Returns 1 for non-stackable items.
    #[must_use]
    pub const fn max_count(&self) -> u32 {
        match self {
            // Bombs max at 40 with biggest bomb bag
            Self::Bomb => 40,
            // Deku Sticks max at 30 (no upgrade in MM)
            Self::DekuStick => 30,
            // Deku Nuts max at 40 (no upgrade in MM)
            Self::DekuNut => 40,
            // 6 bottles in MM
            Self::Bottle => 6,
            // Dungeon-specific small key counts
            Self::SmallKeyWoodfallTemple => 1,
            Self::SmallKeySnowheadTemple => 3,
            Self::SmallKeyGreatBayTemple => 1,
            Self::SmallKeyStoneTowerTemple => 4,
            Self::SmallKey => 99, // Generic key, no specific limit
            // Stray Fairies per dungeon
            Self::StrayFairyWoodfall => 15,
            Self::StrayFairySnowhead => 15,
            Self::StrayFairyGreatBay => 15,
            Self::StrayFairyStoneTower => 15,
            Self::StrayFairyClockTown => 1, // Only 1 in Clock Town
            Self::StrayFairy => 99,         // Generic
            // Heart pieces
            Self::PieceOfHeart => 52, // MM has 52 pieces
            // Heart containers
            Self::HeartContainer => 4, // 4 from dungeons
            // Rupees (effectively unlimited in inventory, but track collected)
            Self::GreenRupee
            | Self::BlueRupee
            | Self::RedRupee
            | Self::PurpleRupee
            | Self::SilverRupee
            | Self::GoldRupee => 999,
            // All other items are single-obtain
            _ => 1,
        }
    }
}

impl super::ItemName for MmItem {
    fn to_str(&self) -> &'static str {
        match self {
            // Masks - Transformation
            Self::DekuMask => "DEKU_MASK",
            Self::GoronMask => "GORON_MASK",
            Self::ZoraMask => "ZORA_MASK",
            Self::FierceDeityMask => "FIERCE_DEITY_MASK",
            // Masks - Regular
            Self::PostmanHat => "POSTMAN_HAT",
            Self::AllNightMask => "ALL_NIGHT_MASK",
            Self::BlastMask => "BLAST_MASK",
            Self::StoneMask => "STONE_MASK",
            Self::GreatFairyMask => "GREAT_FAIRY_MASK",
            Self::KeatonMask => "KEATON_MASK",
            Self::BremenMask => "BREMEN_MASK",
            Self::BunnyHood => "BUNNY_HOOD",
            Self::DonGeroMask => "DON_GERO_MASK",
            Self::MaskOfScents => "MASK_OF_SCENTS",
            Self::RomaniMask => "ROMANI_MASK",
            Self::CircusLeaderMask => "CIRCUS_LEADER_MASK",
            Self::KafeiMask => "KAFEI_MASK",
            Self::CouplesMask => "COUPLES_MASK",
            Self::MaskOfTruth => "MASK_OF_TRUTH",
            Self::KamaroMask => "KAMARO_MASK",
            Self::GibdoMask => "GIBDO_MASK",
            Self::GaroMask => "GARO_MASK",
            Self::CaptainHat => "CAPTAIN_HAT",
            Self::GiantMask => "GIANT_MASK",
            // Swords
            Self::KokiriSword => "KOKIRI_SWORD",
            Self::RazorSword => "RAZOR_SWORD",
            Self::GildedSword => "GILDED_SWORD",
            Self::GreatFairySword => "GREAT_FAIRY_SWORD",
            // Shields
            Self::HeroShield => "HERO_SHIELD",
            Self::MirrorShield => "MIRROR_SHIELD",
            // Equipment Items
            Self::HerosBow => "HEROS_BOW",
            Self::FireArrow => "FIRE_ARROW",
            Self::IceArrow => "ICE_ARROW",
            Self::LightArrow => "LIGHT_ARROW",
            Self::Bomb => "BOMB",
            Self::Bombchu => "BOMBCHU",
            Self::DekuStick => "DEKU_STICK",
            Self::DekuNut => "DEKU_NUT",
            Self::MagicBean => "MAGIC_BEAN",
            Self::PowderKeg => "POWDER_KEG",
            Self::Hookshot => "HOOKSHOT",
            Self::LensOfTruth => "LENS_OF_TRUTH",
            Self::PictographBox => "PICTOGRAPH_BOX",
            Self::OcarinaOfTime => "OCARINA_OF_TIME",
            // Bottles
            Self::Bottle => "BOTTLE",
            Self::BottleRedPotion => "BOTTLE_RED_POTION",
            Self::BottleGreenPotion => "BOTTLE_GREEN_POTION",
            Self::BottleBluePotion => "BOTTLE_BLUE_POTION",
            Self::BottleFairy => "BOTTLE_FAIRY",
            Self::BottleDekuPrincess => "BOTTLE_DEKU_PRINCESS",
            Self::BottleFish => "BOTTLE_FISH",
            Self::BottleBugs => "BOTTLE_BUGS",
            Self::BottlePoe => "BOTTLE_POE",
            Self::BottleBigPoe => "BOTTLE_BIG_POE",
            Self::BottleHotSpringWater => "BOTTLE_HOT_SPRING_WATER",
            Self::BottleZoraEgg => "BOTTLE_ZORA_EGG",
            Self::BottleMushroom => "BOTTLE_MUSHROOM",
            Self::BottleGoldDust => "BOTTLE_GOLD_DUST",
            Self::BottleMilk => "BOTTLE_MILK",
            Self::BottleHalfMilk => "BOTTLE_HALF_MILK",
            Self::BottleChateau => "BOTTLE_CHATEAU",
            Self::BottleSeaHorse => "BOTTLE_SEA_HORSE",
            // Songs
            Self::SongOfTime => "SONG_OF_TIME",
            Self::SongOfHealing => "SONG_OF_HEALING",
            Self::EponasSong => "EPONAS_SONG",
            Self::SongOfSoaring => "SONG_OF_SOARING",
            Self::SongOfStorms => "SONG_OF_STORMS",
            Self::SonataOfAwakening => "SONATA_OF_AWAKENING",
            Self::GoronLullaby => "GORON_LULLABY",
            Self::NewWaveBossaNova => "NEW_WAVE_BOSSA_NOVA",
            Self::ElegyOfEmptiness => "ELEGY_OF_EMPTINESS",
            Self::OathToOrder => "OATH_TO_ORDER",
            // Upgrades
            Self::AdultWallet => "ADULT_WALLET",
            Self::GiantWallet => "GIANT_WALLET",
            Self::Quiver30 => "QUIVER_30",
            Self::Quiver40 => "QUIVER_40",
            Self::Quiver50 => "QUIVER_50",
            Self::BombBag20 => "BOMB_BAG_20",
            Self::BombBag30 => "BOMB_BAG_30",
            Self::BombBag40 => "BOMB_BAG_40",
            Self::MagicMeter => "MAGIC_METER",
            Self::DoubleMagic => "DOUBLE_MAGIC",
            Self::DoubleDefense => "DOUBLE_DEFENSE",
            // Quest Items
            Self::MoonsTear => "MOONS_TEAR",
            Self::LandTitleDeed => "LAND_TITLE_DEED",
            Self::SwampTitleDeed => "SWAMP_TITLE_DEED",
            Self::MountainTitleDeed => "MOUNTAIN_TITLE_DEED",
            Self::OceanTitleDeed => "OCEAN_TITLE_DEED",
            Self::RoomKey => "ROOM_KEY",
            Self::LetterToKafei => "LETTER_TO_KAFEI",
            Self::PendantOfMemories => "PENDANT_OF_MEMORIES",
            Self::LetterToMama => "LETTER_TO_MAMA",
            Self::SpecialDeliveryToMama => "SPECIAL_DELIVERY_TO_MAMA",
            // Boss Remains
            Self::OdolwaRemains => "ODOLWA_REMAINS",
            Self::GohtRemains => "GOHT_REMAINS",
            Self::GyorgRemains => "GYORG_REMAINS",
            Self::TwinmoldRemains => "TWINMOLD_REMAINS",
            // Dungeon Items
            Self::SmallKey => "SMALL_KEY",
            Self::BossKey => "BOSS_KEY",
            Self::Map => "MAP",
            Self::Compass => "COMPASS",
            Self::StrayFairy => "STRAY_FAIRY",
            // Dungeon-Specific Keys
            Self::SmallKeyWoodfallTemple => "SMALL_KEY_WOODFALL_TEMPLE",
            Self::SmallKeySnowheadTemple => "SMALL_KEY_SNOWHEAD_TEMPLE",
            Self::SmallKeyGreatBayTemple => "SMALL_KEY_GREAT_BAY_TEMPLE",
            Self::SmallKeyStoneTowerTemple => "SMALL_KEY_STONE_TOWER_TEMPLE",
            Self::BossKeyWoodfallTemple => "BOSS_KEY_WOODFALL_TEMPLE",
            Self::BossKeySnowheadTemple => "BOSS_KEY_SNOWHEAD_TEMPLE",
            Self::BossKeyGreatBayTemple => "BOSS_KEY_GREAT_BAY_TEMPLE",
            Self::BossKeyStoneTowerTemple => "BOSS_KEY_STONE_TOWER_TEMPLE",
            // Stray Fairies per dungeon
            Self::StrayFairyWoodfall => "STRAY_FAIRY_WOODFALL",
            Self::StrayFairySnowhead => "STRAY_FAIRY_SNOWHEAD",
            Self::StrayFairyGreatBay => "STRAY_FAIRY_GREAT_BAY",
            Self::StrayFairyStoneTower => "STRAY_FAIRY_STONE_TOWER",
            Self::StrayFairyClockTown => "STRAY_FAIRY_CLOCK_TOWN",
            // Collectibles
            Self::HeartContainer => "HEART_CONTAINER",
            Self::PieceOfHeart => "PIECE_OF_HEART",
            Self::GreenRupee => "GREEN_RUPEE",
            Self::BlueRupee => "BLUE_RUPEE",
            Self::RedRupee => "RED_RUPEE",
            Self::PurpleRupee => "PURPLE_RUPEE",
            Self::SilverRupee => "SILVER_RUPEE",
            Self::GoldRupee => "GOLD_RUPEE",
            // Notebook Events
            Self::BomberNotebook => "BOMBER_NOTEBOOK",
            // Special
            Self::GiantsWallet => "GIANTS_WALLET",
            Self::OceanTitleDeedTraded => "OCEAN_TITLE_DEED_TRADED",
        }
    }

    fn from_str(s: &str) -> Option<Self> {
        let item = match s {
            // Masks - Transformation
            "DEKU_MASK" => Self::DekuMask,
            "GORON_MASK" => Self::GoronMask,
            "ZORA_MASK" => Self::ZoraMask,
            "FIERCE_DEITY_MASK" => Self::FierceDeityMask,
            // Masks - Regular
            "POSTMAN_HAT" => Self::PostmanHat,
            "ALL_NIGHT_MASK" => Self::AllNightMask,
            "BLAST_MASK" => Self::BlastMask,
            "STONE_MASK" => Self::StoneMask,
            "GREAT_FAIRY_MASK" => Self::GreatFairyMask,
            "KEATON_MASK" => Self::KeatonMask,
            "BREMEN_MASK" => Self::BremenMask,
            "BUNNY_HOOD" => Self::BunnyHood,
            "DON_GERO_MASK" => Self::DonGeroMask,
            "MASK_OF_SCENTS" => Self::MaskOfScents,
            "ROMANI_MASK" => Self::RomaniMask,
            "CIRCUS_LEADER_MASK" => Self::CircusLeaderMask,
            "KAFEI_MASK" => Self::KafeiMask,
            "COUPLES_MASK" => Self::CouplesMask,
            "MASK_OF_TRUTH" => Self::MaskOfTruth,
            "KAMARO_MASK" => Self::KamaroMask,
            "GIBDO_MASK" => Self::GibdoMask,
            "GARO_MASK" => Self::GaroMask,
            "CAPTAIN_HAT" => Self::CaptainHat,
            "GIANT_MASK" => Self::GiantMask,
            // Swords
            "KOKIRI_SWORD" => Self::KokiriSword,
            "RAZOR_SWORD" => Self::RazorSword,
            "GILDED_SWORD" => Self::GildedSword,
            "GREAT_FAIRY_SWORD" => Self::GreatFairySword,
            // Shields
            "HERO_SHIELD" => Self::HeroShield,
            "MIRROR_SHIELD" => Self::MirrorShield,
            // Equipment Items
            "HEROS_BOW" => Self::HerosBow,
            "FIRE_ARROW" => Self::FireArrow,
            "ICE_ARROW" => Self::IceArrow,
            "LIGHT_ARROW" => Self::LightArrow,
            "BOMB" => Self::Bomb,
            "BOMBCHU" => Self::Bombchu,
            "DEKU_STICK" => Self::DekuStick,
            "DEKU_NUT" => Self::DekuNut,
            "MAGIC_BEAN" => Self::MagicBean,
            "POWDER_KEG" => Self::PowderKeg,
            "HOOKSHOT" => Self::Hookshot,
            "LENS_OF_TRUTH" => Self::LensOfTruth,
            "PICTOGRAPH_BOX" => Self::PictographBox,
            "OCARINA_OF_TIME" => Self::OcarinaOfTime,
            // Bottles
            "BOTTLE" => Self::Bottle,
            "BOTTLE_RED_POTION" => Self::BottleRedPotion,
            "BOTTLE_GREEN_POTION" => Self::BottleGreenPotion,
            "BOTTLE_BLUE_POTION" => Self::BottleBluePotion,
            "BOTTLE_FAIRY" => Self::BottleFairy,
            "BOTTLE_DEKU_PRINCESS" => Self::BottleDekuPrincess,
            "BOTTLE_FISH" => Self::BottleFish,
            "BOTTLE_BUGS" => Self::BottleBugs,
            "BOTTLE_POE" => Self::BottlePoe,
            "BOTTLE_BIG_POE" => Self::BottleBigPoe,
            "BOTTLE_HOT_SPRING_WATER" => Self::BottleHotSpringWater,
            "BOTTLE_ZORA_EGG" => Self::BottleZoraEgg,
            "BOTTLE_MUSHROOM" => Self::BottleMushroom,
            "BOTTLE_GOLD_DUST" => Self::BottleGoldDust,
            "BOTTLE_MILK" => Self::BottleMilk,
            "BOTTLE_HALF_MILK" => Self::BottleHalfMilk,
            "BOTTLE_CHATEAU" => Self::BottleChateau,
            "BOTTLE_SEA_HORSE" => Self::BottleSeaHorse,
            // Songs
            "SONG_OF_TIME" => Self::SongOfTime,
            "SONG_OF_HEALING" => Self::SongOfHealing,
            "EPONAS_SONG" => Self::EponasSong,
            "SONG_OF_SOARING" => Self::SongOfSoaring,
            "SONG_OF_STORMS" => Self::SongOfStorms,
            "SONATA_OF_AWAKENING" => Self::SonataOfAwakening,
            "GORON_LULLABY" => Self::GoronLullaby,
            "NEW_WAVE_BOSSA_NOVA" => Self::NewWaveBossaNova,
            "ELEGY_OF_EMPTINESS" => Self::ElegyOfEmptiness,
            "OATH_TO_ORDER" => Self::OathToOrder,
            // Upgrades
            "ADULT_WALLET" => Self::AdultWallet,
            "GIANT_WALLET" => Self::GiantWallet,
            "QUIVER_30" => Self::Quiver30,
            "QUIVER_40" => Self::Quiver40,
            "QUIVER_50" => Self::Quiver50,
            "BOMB_BAG_20" => Self::BombBag20,
            "BOMB_BAG_30" => Self::BombBag30,
            "BOMB_BAG_40" => Self::BombBag40,
            "MAGIC_METER" => Self::MagicMeter,
            "DOUBLE_MAGIC" => Self::DoubleMagic,
            "DOUBLE_DEFENSE" => Self::DoubleDefense,
            // Quest Items
            "MOONS_TEAR" => Self::MoonsTear,
            "LAND_TITLE_DEED" => Self::LandTitleDeed,
            "SWAMP_TITLE_DEED" => Self::SwampTitleDeed,
            "MOUNTAIN_TITLE_DEED" => Self::MountainTitleDeed,
            "OCEAN_TITLE_DEED" => Self::OceanTitleDeed,
            "ROOM_KEY" => Self::RoomKey,
            "LETTER_TO_KAFEI" => Self::LetterToKafei,
            "PENDANT_OF_MEMORIES" => Self::PendantOfMemories,
            "LETTER_TO_MAMA" => Self::LetterToMama,
            "SPECIAL_DELIVERY_TO_MAMA" => Self::SpecialDeliveryToMama,
            // Boss Remains
            "ODOLWA_REMAINS" => Self::OdolwaRemains,
            "GOHT_REMAINS" => Self::GohtRemains,
            "GYORG_REMAINS" => Self::GyorgRemains,
            "TWINMOLD_REMAINS" => Self::TwinmoldRemains,
            // Dungeon Items
            "SMALL_KEY" => Self::SmallKey,
            "BOSS_KEY" => Self::BossKey,
            "MAP" => Self::Map,
            "COMPASS" => Self::Compass,
            "STRAY_FAIRY" => Self::StrayFairy,
            // Dungeon-Specific Keys
            "SMALL_KEY_WOODFALL_TEMPLE" => Self::SmallKeyWoodfallTemple,
            "SMALL_KEY_SNOWHEAD_TEMPLE" => Self::SmallKeySnowheadTemple,
            "SMALL_KEY_GREAT_BAY_TEMPLE" => Self::SmallKeyGreatBayTemple,
            "SMALL_KEY_STONE_TOWER_TEMPLE" => Self::SmallKeyStoneTowerTemple,
            "BOSS_KEY_WOODFALL_TEMPLE" => Self::BossKeyWoodfallTemple,
            "BOSS_KEY_SNOWHEAD_TEMPLE" => Self::BossKeySnowheadTemple,
            "BOSS_KEY_GREAT_BAY_TEMPLE" => Self::BossKeyGreatBayTemple,
            "BOSS_KEY_STONE_TOWER_TEMPLE" => Self::BossKeyStoneTowerTemple,
            // Stray Fairies per dungeon
            "STRAY_FAIRY_WOODFALL" => Self::StrayFairyWoodfall,
            "STRAY_FAIRY_SNOWHEAD" => Self::StrayFairySnowhead,
            "STRAY_FAIRY_GREAT_BAY" => Self::StrayFairyGreatBay,
            "STRAY_FAIRY_STONE_TOWER" => Self::StrayFairyStoneTower,
            "STRAY_FAIRY_CLOCK_TOWN" => Self::StrayFairyClockTown,
            // Collectibles
            "HEART_CONTAINER" => Self::HeartContainer,
            "PIECE_OF_HEART" => Self::PieceOfHeart,
            "GREEN_RUPEE" => Self::GreenRupee,
            "BLUE_RUPEE" => Self::BlueRupee,
            "RED_RUPEE" => Self::RedRupee,
            "PURPLE_RUPEE" => Self::PurpleRupee,
            "SILVER_RUPEE" => Self::SilverRupee,
            "GOLD_RUPEE" => Self::GoldRupee,
            // Notebook Events
            "BOMBER_NOTEBOOK" => Self::BomberNotebook,
            // Special
            "GIANTS_WALLET" => Self::GiantsWallet,
            "OCEAN_TITLE_DEED_TRADED" => Self::OceanTitleDeedTraded,
            _ => return None,
        };
        Some(item)
    }
}

#[cfg(test)]
mod tests {
    use super::MmItem;
    use crate::item::ItemName;
    use std::collections::HashSet;

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

    // Additional comprehensive tests

    #[test]
    fn test_by_name_all_transformation_masks() {
        assert_eq!(MmItem::by_name("DekuMask"), Some(MmItem::DekuMask));
        assert_eq!(MmItem::by_name("deku_mask"), Some(MmItem::DekuMask));
        assert_eq!(MmItem::by_name("GoronMask"), Some(MmItem::GoronMask));
        assert_eq!(MmItem::by_name("ZoraMask"), Some(MmItem::ZoraMask));
        assert_eq!(
            MmItem::by_name("FierceDeityMask"),
            Some(MmItem::FierceDeityMask)
        );
    }

    #[test]
    fn test_by_name_regular_masks() {
        assert_eq!(MmItem::by_name("PostmanHat"), Some(MmItem::PostmanHat));
        assert_eq!(MmItem::by_name("AllNightMask"), Some(MmItem::AllNightMask));
        assert_eq!(MmItem::by_name("BlastMask"), Some(MmItem::BlastMask));
        assert_eq!(MmItem::by_name("StoneMask"), Some(MmItem::StoneMask));
        assert_eq!(
            MmItem::by_name("GreatFairyMask"),
            Some(MmItem::GreatFairyMask)
        );
        assert_eq!(MmItem::by_name("BremenMask"), Some(MmItem::BremenMask));
        assert_eq!(MmItem::by_name("bunny_hood"), Some(MmItem::BunnyHood));
        assert_eq!(MmItem::by_name("GiantMask"), Some(MmItem::GiantMask));
    }

    #[test]
    fn test_by_name_all_swords() {
        assert_eq!(MmItem::by_name("KokiriSword"), Some(MmItem::KokiriSword));
        assert_eq!(MmItem::by_name("RazorSword"), Some(MmItem::RazorSword));
        assert_eq!(MmItem::by_name("GildedSword"), Some(MmItem::GildedSword));
        assert_eq!(
            MmItem::by_name("GreatFairySword"),
            Some(MmItem::GreatFairySword)
        );
        assert_eq!(
            MmItem::by_name("great_fairy_sword"),
            Some(MmItem::GreatFairySword)
        );
    }

    #[test]
    fn test_by_name_all_shields() {
        assert_eq!(MmItem::by_name("HeroShield"), Some(MmItem::HeroShield));
        assert_eq!(MmItem::by_name("hero_shield"), Some(MmItem::HeroShield));
        assert_eq!(MmItem::by_name("MirrorShield"), Some(MmItem::MirrorShield));
    }

    #[test]
    fn test_by_name_equipment_items() {
        assert_eq!(MmItem::by_name("HerosBow"), Some(MmItem::HerosBow));
        assert_eq!(MmItem::by_name("heros_bow"), Some(MmItem::HerosBow));
        assert_eq!(MmItem::by_name("Bombchu"), Some(MmItem::Bombchu));
        assert_eq!(MmItem::by_name("MagicBean"), Some(MmItem::MagicBean));
        assert_eq!(MmItem::by_name("PowderKeg"), Some(MmItem::PowderKeg));
        assert_eq!(
            MmItem::by_name("PictographBox"),
            Some(MmItem::PictographBox)
        );
    }

    #[test]
    fn test_by_name_all_arrows() {
        assert_eq!(MmItem::by_name("FireArrow"), Some(MmItem::FireArrow));
        assert_eq!(MmItem::by_name("IceArrow"), Some(MmItem::IceArrow));
        assert_eq!(MmItem::by_name("LightArrow"), Some(MmItem::LightArrow));
        assert_eq!(MmItem::by_name("ice_arrow"), Some(MmItem::IceArrow));
    }

    #[test]
    fn test_by_name_all_bottles() {
        assert_eq!(MmItem::by_name("Bottle"), Some(MmItem::Bottle));
        assert_eq!(
            MmItem::by_name("BottleRedPotion"),
            Some(MmItem::BottleRedPotion)
        );
        assert_eq!(
            MmItem::by_name("BottleDekuPrincess"),
            Some(MmItem::BottleDekuPrincess)
        );
        assert_eq!(
            MmItem::by_name("BottleHotSpringWater"),
            Some(MmItem::BottleHotSpringWater)
        );
        assert_eq!(
            MmItem::by_name("BottleZoraEgg"),
            Some(MmItem::BottleZoraEgg)
        );
        assert_eq!(
            MmItem::by_name("bottle_chateau"),
            Some(MmItem::BottleChateau)
        );
        assert_eq!(
            MmItem::by_name("BottleSeaHorse"),
            Some(MmItem::BottleSeaHorse)
        );
    }

    #[test]
    fn test_by_name_all_songs() {
        assert_eq!(MmItem::by_name("SongOfTime"), Some(MmItem::SongOfTime));
        assert_eq!(
            MmItem::by_name("SongOfHealing"),
            Some(MmItem::SongOfHealing)
        );
        assert_eq!(MmItem::by_name("EponasSong"), Some(MmItem::EponasSong));
        assert_eq!(
            MmItem::by_name("SongOfSoaring"),
            Some(MmItem::SongOfSoaring)
        );
        assert_eq!(MmItem::by_name("SongOfStorms"), Some(MmItem::SongOfStorms));
        assert_eq!(
            MmItem::by_name("SonataOfAwakening"),
            Some(MmItem::SonataOfAwakening)
        );
        assert_eq!(MmItem::by_name("GoronLullaby"), Some(MmItem::GoronLullaby));
        assert_eq!(
            MmItem::by_name("NewWaveBossaNova"),
            Some(MmItem::NewWaveBossaNova)
        );
        assert_eq!(
            MmItem::by_name("ElegyOfEmptiness"),
            Some(MmItem::ElegyOfEmptiness)
        );
        assert_eq!(MmItem::by_name("OathToOrder"), Some(MmItem::OathToOrder));
    }

    #[test]
    fn test_by_name_quest_items() {
        assert_eq!(MmItem::by_name("MoonsTear"), Some(MmItem::MoonsTear));
        assert_eq!(
            MmItem::by_name("LandTitleDeed"),
            Some(MmItem::LandTitleDeed)
        );
        assert_eq!(
            MmItem::by_name("SwampTitleDeed"),
            Some(MmItem::SwampTitleDeed)
        );
        assert_eq!(MmItem::by_name("RoomKey"), Some(MmItem::RoomKey));
        assert_eq!(
            MmItem::by_name("LetterToKafei"),
            Some(MmItem::LetterToKafei)
        );
        assert_eq!(
            MmItem::by_name("PendantOfMemories"),
            Some(MmItem::PendantOfMemories)
        );
    }

    #[test]
    fn test_by_name_all_boss_remains() {
        assert_eq!(
            MmItem::by_name("OdolwaRemains"),
            Some(MmItem::OdolwaRemains)
        );
        assert_eq!(MmItem::by_name("GohtRemains"), Some(MmItem::GohtRemains));
        assert_eq!(MmItem::by_name("GyorgRemains"), Some(MmItem::GyorgRemains));
        assert_eq!(
            MmItem::by_name("TwinmoldRemains"),
            Some(MmItem::TwinmoldRemains)
        );
        assert_eq!(
            MmItem::by_name("twinmold_remains"),
            Some(MmItem::TwinmoldRemains)
        );
    }

    #[test]
    fn test_by_name_stray_fairies() {
        assert_eq!(MmItem::by_name("StrayFairy"), Some(MmItem::StrayFairy));
        assert_eq!(
            MmItem::by_name("StrayFairyWoodfall"),
            Some(MmItem::StrayFairyWoodfall)
        );
        assert_eq!(
            MmItem::by_name("StrayFairySnowhead"),
            Some(MmItem::StrayFairySnowhead)
        );
        assert_eq!(
            MmItem::by_name("StrayFairyGreatBay"),
            Some(MmItem::StrayFairyGreatBay)
        );
        assert_eq!(
            MmItem::by_name("StrayFairyStoneTower"),
            Some(MmItem::StrayFairyStoneTower)
        );
        assert_eq!(
            MmItem::by_name("stray_fairy_clock_town"),
            Some(MmItem::StrayFairyClockTown)
        );
    }

    #[test]
    fn test_by_name_edge_cases() {
        // Whitespace should not match
        assert_eq!(MmItem::by_name(" DekuMask"), None);
        assert_eq!(MmItem::by_name("DekuMask "), None);
        // Mixed case should not match
        assert_eq!(MmItem::by_name("dekumask"), None);
        assert_eq!(MmItem::by_name("DEKUMASK"), None);
        // Similar but incorrect names
        assert_eq!(MmItem::by_name("Deku_Mask"), None);
        assert_eq!(MmItem::by_name("deku-mask"), None);
    }

    #[test]
    fn test_transformation_masks_complete() {
        // All transformation masks
        assert!(MmItem::DekuMask.is_transformation_mask());
        assert!(MmItem::GoronMask.is_transformation_mask());
        assert!(MmItem::ZoraMask.is_transformation_mask());
        assert!(MmItem::FierceDeityMask.is_transformation_mask());

        // Non-transformation masks
        assert!(!MmItem::KeatonMask.is_transformation_mask());
        assert!(!MmItem::GiantMask.is_transformation_mask());
    }

    #[test]
    fn test_is_mask_complete() {
        // Transformation masks
        assert!(MmItem::DekuMask.is_mask());
        assert!(MmItem::GoronMask.is_mask());
        assert!(MmItem::ZoraMask.is_mask());
        assert!(MmItem::FierceDeityMask.is_mask());

        // Regular masks
        assert!(MmItem::PostmanHat.is_mask());
        assert!(MmItem::AllNightMask.is_mask());
        assert!(MmItem::BlastMask.is_mask());
        assert!(MmItem::StoneMask.is_mask());
        assert!(MmItem::GreatFairyMask.is_mask());
        assert!(MmItem::KeatonMask.is_mask());
        assert!(MmItem::BremenMask.is_mask());
        assert!(MmItem::BunnyHood.is_mask());
        assert!(MmItem::DonGeroMask.is_mask());
        assert!(MmItem::MaskOfScents.is_mask());
        assert!(MmItem::RomaniMask.is_mask());
        assert!(MmItem::CircusLeaderMask.is_mask());
        assert!(MmItem::KafeiMask.is_mask());
        assert!(MmItem::CouplesMask.is_mask());
        assert!(MmItem::MaskOfTruth.is_mask());
        assert!(MmItem::KamaroMask.is_mask());
        assert!(MmItem::GibdoMask.is_mask());
        assert!(MmItem::GaroMask.is_mask());
        assert!(MmItem::CaptainHat.is_mask());
        assert!(MmItem::GiantMask.is_mask());

        // Non-masks
        assert!(!MmItem::Hookshot.is_mask());
        assert!(!MmItem::HerosBow.is_mask());
    }

    #[test]
    fn test_boss_remains_complete() {
        // All boss remains
        assert!(MmItem::OdolwaRemains.is_boss_remain());
        assert!(MmItem::GohtRemains.is_boss_remain());
        assert!(MmItem::GyorgRemains.is_boss_remain());
        assert!(MmItem::TwinmoldRemains.is_boss_remain());

        // Non-boss remains
        assert!(!MmItem::Hookshot.is_boss_remain());
        assert!(!MmItem::DekuMask.is_boss_remain());
    }

    #[test]
    fn test_songs_complete() {
        // All songs
        assert!(MmItem::SongOfTime.is_song());
        assert!(MmItem::SongOfHealing.is_song());
        assert!(MmItem::EponasSong.is_song());
        assert!(MmItem::SongOfSoaring.is_song());
        assert!(MmItem::SongOfStorms.is_song());
        assert!(MmItem::SonataOfAwakening.is_song());
        assert!(MmItem::GoronLullaby.is_song());
        assert!(MmItem::NewWaveBossaNova.is_song());
        assert!(MmItem::ElegyOfEmptiness.is_song());
        assert!(MmItem::OathToOrder.is_song());

        // Non-songs
        assert!(!MmItem::OcarinaOfTime.is_song());
        assert!(!MmItem::Hookshot.is_song());
    }

    #[test]
    fn test_progressive_items_complete() {
        // All progressive items
        assert!(MmItem::Bomb.is_progressive());
        assert!(MmItem::DekuStick.is_progressive());
        assert!(MmItem::DekuNut.is_progressive());
        assert!(MmItem::Bottle.is_progressive());
        assert!(MmItem::SmallKey.is_progressive());
        assert!(MmItem::StrayFairy.is_progressive());
        assert!(MmItem::HeartContainer.is_progressive());
        assert!(MmItem::PieceOfHeart.is_progressive());

        // Non-progressive items
        assert!(!MmItem::BottleRedPotion.is_progressive());
        assert!(!MmItem::SmallKeyWoodfallTemple.is_progressive());
    }

    #[test]
    fn test_dungeon_items_complete() {
        // Generic dungeon items
        assert!(MmItem::SmallKey.is_dungeon_item());
        assert!(MmItem::BossKey.is_dungeon_item());
        assert!(MmItem::Map.is_dungeon_item());
        assert!(MmItem::Compass.is_dungeon_item());
        assert!(MmItem::StrayFairy.is_dungeon_item());

        // All dungeon-specific small keys
        assert!(MmItem::SmallKeyWoodfallTemple.is_dungeon_item());
        assert!(MmItem::SmallKeySnowheadTemple.is_dungeon_item());
        assert!(MmItem::SmallKeyGreatBayTemple.is_dungeon_item());
        assert!(MmItem::SmallKeyStoneTowerTemple.is_dungeon_item());

        // All dungeon-specific boss keys
        assert!(MmItem::BossKeyWoodfallTemple.is_dungeon_item());
        assert!(MmItem::BossKeySnowheadTemple.is_dungeon_item());
        assert!(MmItem::BossKeyGreatBayTemple.is_dungeon_item());
        assert!(MmItem::BossKeyStoneTowerTemple.is_dungeon_item());

        // Non-dungeon items
        assert!(!MmItem::OdolwaRemains.is_dungeon_item());
        assert!(!MmItem::StrayFairyWoodfall.is_dungeon_item());
    }

    #[test]
    fn test_clone_trait() {
        let item = MmItem::DekuMask;
        #[allow(clippy::clone_on_copy)]
        let cloned = item.clone();
        assert_eq!(item, cloned);
    }

    #[test]
    fn test_copy_trait() {
        let item = MmItem::Hookshot;
        let copied = item;
        // Original still usable (Copy semantics)
        assert_eq!(item, copied);
    }

    #[test]
    fn test_debug_trait() {
        let item = MmItem::FierceDeityMask;
        let debug_str = format!("{:?}", item);
        assert_eq!(debug_str, "FierceDeityMask");
    }

    #[test]
    fn test_hash_trait() {
        let mut set = HashSet::new();
        set.insert(MmItem::DekuMask);
        set.insert(MmItem::Hookshot);
        set.insert(MmItem::DekuMask); // Duplicate

        assert_eq!(set.len(), 2);
        assert!(set.contains(&MmItem::DekuMask));
        assert!(set.contains(&MmItem::Hookshot));
    }

    #[test]
    fn test_eq_trait() {
        assert_eq!(MmItem::HerosBow, MmItem::HerosBow);
        assert_ne!(MmItem::HerosBow, MmItem::Bomb);
    }

    #[test]
    fn test_by_name_upgrades() {
        assert_eq!(MmItem::by_name("AdultWallet"), Some(MmItem::AdultWallet));
        assert_eq!(MmItem::by_name("GiantWallet"), Some(MmItem::GiantWallet));
        assert_eq!(MmItem::by_name("MagicMeter"), Some(MmItem::MagicMeter));
        assert_eq!(MmItem::by_name("DoubleMagic"), Some(MmItem::DoubleMagic));
        assert_eq!(
            MmItem::by_name("DoubleDefense"),
            Some(MmItem::DoubleDefense)
        );
    }

    #[test]
    fn test_by_name_capacity_upgrades() {
        assert_eq!(MmItem::by_name("Quiver30"), Some(MmItem::Quiver30));
        assert_eq!(MmItem::by_name("quiver_40"), Some(MmItem::Quiver40));
        assert_eq!(MmItem::by_name("Quiver50"), Some(MmItem::Quiver50));
        assert_eq!(MmItem::by_name("BombBag20"), Some(MmItem::BombBag20));
        assert_eq!(MmItem::by_name("bomb_bag_30"), Some(MmItem::BombBag30));
        assert_eq!(MmItem::by_name("BombBag40"), Some(MmItem::BombBag40));
    }

    #[test]
    fn test_by_name_rupees() {
        assert_eq!(MmItem::by_name("GreenRupee"), Some(MmItem::GreenRupee));
        assert_eq!(MmItem::by_name("BlueRupee"), Some(MmItem::BlueRupee));
        assert_eq!(MmItem::by_name("RedRupee"), Some(MmItem::RedRupee));
        assert_eq!(MmItem::by_name("PurpleRupee"), Some(MmItem::PurpleRupee));
        assert_eq!(MmItem::by_name("SilverRupee"), Some(MmItem::SilverRupee));
        assert_eq!(MmItem::by_name("GoldRupee"), Some(MmItem::GoldRupee));
    }

    #[test]
    fn test_by_name_special_items() {
        assert_eq!(
            MmItem::by_name("BomberNotebook"),
            Some(MmItem::BomberNotebook)
        );
        assert_eq!(MmItem::by_name("GiantsWallet"), Some(MmItem::GiantsWallet));
        assert_eq!(
            MmItem::by_name("OceanTitleDeedTraded"),
            Some(MmItem::OceanTitleDeedTraded)
        );
    }

    // ItemName trait tests

    #[test]
    fn test_item_name_to_str() {
        assert_eq!(MmItem::DekuMask.to_str(), "DEKU_MASK");
        assert_eq!(MmItem::Hookshot.to_str(), "HOOKSHOT");
        assert_eq!(MmItem::SongOfHealing.to_str(), "SONG_OF_HEALING");
        assert_eq!(
            MmItem::SmallKeyWoodfallTemple.to_str(),
            "SMALL_KEY_WOODFALL_TEMPLE"
        );
        assert_eq!(MmItem::OdolwaRemains.to_str(), "ODOLWA_REMAINS");
    }

    #[test]
    fn test_item_name_from_str() {
        assert_eq!(MmItem::from_str("DEKU_MASK"), Some(MmItem::DekuMask));
        assert_eq!(MmItem::from_str("HOOKSHOT"), Some(MmItem::Hookshot));
        assert_eq!(
            MmItem::from_str("SONG_OF_HEALING"),
            Some(MmItem::SongOfHealing)
        );
        assert_eq!(
            MmItem::from_str("SMALL_KEY_WOODFALL_TEMPLE"),
            Some(MmItem::SmallKeyWoodfallTemple)
        );
    }

    #[test]
    fn test_item_name_from_str_invalid() {
        assert_eq!(MmItem::from_str("INVALID_ITEM"), None);
        assert_eq!(MmItem::from_str(""), None);
        assert_eq!(MmItem::from_str("deku_mask"), None); // Wrong case
    }

    #[test]
    fn test_item_name_roundtrip() {
        // Test that to_str and from_str are inverses for all items
        let items = [
            MmItem::DekuMask,
            MmItem::Hookshot,
            MmItem::SongOfHealing,
            MmItem::SmallKeyWoodfallTemple,
            MmItem::OdolwaRemains,
            MmItem::StrayFairySnowhead,
            MmItem::FierceDeityMask,
        ];

        for item in items {
            let s = item.to_str();
            let parsed = MmItem::from_str(s);
            assert_eq!(parsed, Some(item), "Roundtrip failed for {:?}", item);
        }
    }
}
