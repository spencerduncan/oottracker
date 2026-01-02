//! Ocarina of Time items.

use crate::item::ItemCategory;
use serde::{Deserialize, Serialize};

/// OoT item enum - all trackable items from Ocarina of Time.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
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
    /// Look up an OotItem by its string name.
    ///
    /// Supports both PascalCase variant names (e.g., "MasterSword") and
    /// snake_case names (e.g., "master_sword").
    #[must_use]
    pub fn by_name(name: &str) -> Option<Self> {
        // Try direct match first (PascalCase)
        let item = match name {
            // Swords
            "KokiriSword" | "kokiri_sword" => Self::KokiriSword,
            "MasterSword" | "master_sword" => Self::MasterSword,
            "BiggoronSword" | "biggoron_sword" => Self::BiggoronSword,
            "GiantKnife" | "giant_knife" => Self::GiantKnife,
            // Shields
            "DekuShield" | "deku_shield" => Self::DekuShield,
            "HylianShield" | "hylian_shield" => Self::HylianShield,
            "MirrorShield" | "mirror_shield" => Self::MirrorShield,
            // Tunics
            "KokiriTunic" | "kokiri_tunic" => Self::KokiriTunic,
            "GoronTunic" | "goron_tunic" => Self::GoronTunic,
            "ZoraTunic" | "zora_tunic" => Self::ZoraTunic,
            // Boots
            "KokiriBoots" | "kokiri_boots" => Self::KokiriBoots,
            "IronBoots" | "iron_boots" => Self::IronBoots,
            "HoverBoots" | "hover_boots" => Self::HoverBoots,
            // Equipment Items
            "DekuStick" | "deku_stick" => Self::DekuStick,
            "DekuNut" | "deku_nut" => Self::DekuNut,
            "Bomb" | "bomb" => Self::Bomb,
            "Bow" | "bow" => Self::Bow,
            "FireArrow" | "fire_arrow" => Self::FireArrow,
            "IceArrow" | "ice_arrow" => Self::IceArrow,
            "LightArrow" | "light_arrow" => Self::LightArrow,
            "DinsFire" | "dins_fire" => Self::DinsFire,
            "FaroresWind" | "farores_wind" => Self::FaroresWind,
            "NayrusLove" | "nayrus_love" => Self::NayrusLove,
            "Slingshot" | "slingshot" => Self::Slingshot,
            "Boomerang" | "boomerang" => Self::Boomerang,
            "Hookshot" | "hookshot" => Self::Hookshot,
            "Longshot" | "longshot" => Self::Longshot,
            "LensOfTruth" | "lens_of_truth" => Self::LensOfTruth,
            "MegatonHammer" | "megaton_hammer" => Self::MegatonHammer,
            "OcarinaOfTime" | "ocarina_of_time" => Self::OcarinaOfTime,
            // C-Button Items
            "Bottle" | "bottle" => Self::Bottle,
            "BottleRedPotion" | "bottle_red_potion" => Self::BottleRedPotion,
            "BottleGreenPotion" | "bottle_green_potion" => Self::BottleGreenPotion,
            "BottleBluePotion" | "bottle_blue_potion" => Self::BottleBluePotion,
            "BottleFairy" | "bottle_fairy" => Self::BottleFairy,
            "BottleFish" | "bottle_fish" => Self::BottleFish,
            "BottleBlueFire" | "bottle_blue_fire" => Self::BottleBlueFire,
            "BottleBugs" | "bottle_bugs" => Self::BottleBugs,
            "BottlePoe" | "bottle_poe" => Self::BottlePoe,
            "BottleBigPoe" | "bottle_big_poe" => Self::BottleBigPoe,
            "BottleMilk" | "bottle_milk" => Self::BottleMilk,
            "BottleHalfMilk" | "bottle_half_milk" => Self::BottleHalfMilk,
            "BottleRutosLetter" | "bottle_rutos_letter" => Self::BottleRutosLetter,
            // Adult Trade Sequence
            "PocketEgg" | "pocket_egg" => Self::PocketEgg,
            "PocketCucco" | "pocket_cucco" => Self::PocketCucco,
            "Cojiro" | "cojiro" => Self::Cojiro,
            "OddMushroom" | "odd_mushroom" => Self::OddMushroom,
            "OddPotion" | "odd_potion" => Self::OddPotion,
            "PoachersSaw" | "poachers_saw" => Self::PoachersSaw,
            "BrokenSword" | "broken_sword" => Self::BrokenSword,
            "Prescription" | "prescription" => Self::Prescription,
            "EyeballFrog" | "eyeball_frog" => Self::EyeballFrog,
            "Eyedrops" | "eyedrops" => Self::Eyedrops,
            "ClaimCheck" | "claim_check" => Self::ClaimCheck,
            // Child Trade Sequence
            "WeirdEgg" | "weird_egg" => Self::WeirdEgg,
            "Chicken" | "chicken" => Self::Chicken,
            "ZeldasLetter" | "zeldas_letter" => Self::ZeldasLetter,
            "SkullMask" | "skull_mask" => Self::SkullMask,
            "SpookyMask" | "spooky_mask" => Self::SpookyMask,
            "KeatonMask" | "keaton_mask" => Self::KeatonMask,
            "BunnyHood" | "bunny_hood" => Self::BunnyHood,
            "GoronMask" | "goron_mask" => Self::GoronMask,
            "ZoraMask" | "zora_mask" => Self::ZoraMask,
            "GerudoMask" | "gerudo_mask" => Self::GerudoMask,
            "MaskOfTruth" | "mask_of_truth" => Self::MaskOfTruth,
            // Songs
            "ZeldasLullaby" | "zeldas_lullaby" => Self::ZeldasLullaby,
            "EponasSong" | "eponas_song" => Self::EponasSong,
            "SariasSong" | "sarias_song" => Self::SariasSong,
            "SunsSong" | "suns_song" => Self::SunsSong,
            "SongOfTime" | "song_of_time" => Self::SongOfTime,
            "SongOfStorms" | "song_of_storms" => Self::SongOfStorms,
            "MinuetOfForest" | "minuet_of_forest" => Self::MinuetOfForest,
            "BoleroOfFire" | "bolero_of_fire" => Self::BoleroOfFire,
            "SerenadeOfWater" | "serenade_of_water" => Self::SerenadeOfWater,
            "NocturneOfShadow" | "nocturne_of_shadow" => Self::NocturneOfShadow,
            "RequiemOfSpirit" | "requiem_of_spirit" => Self::RequiemOfSpirit,
            "PreludeOfLight" | "prelude_of_light" => Self::PreludeOfLight,
            "ScarecrowSong" | "scarecrow_song" => Self::ScarecrowSong,
            // Upgrades
            "GoronBracelet" | "goron_bracelet" => Self::GoronBracelet,
            "SilverGauntlets" | "silver_gauntlets" => Self::SilverGauntlets,
            "GoldenGauntlets" | "golden_gauntlets" => Self::GoldenGauntlets,
            "SilverScale" | "silver_scale" => Self::SilverScale,
            "GoldenScale" | "golden_scale" => Self::GoldenScale,
            "ChildWallet" | "child_wallet" => Self::ChildWallet,
            "AdultWallet" | "adult_wallet" => Self::AdultWallet,
            "GiantWallet" | "giant_wallet" => Self::GiantWallet,
            "DekuStickCapacity20" | "deku_stick_capacity_20" => Self::DekuStickCapacity20,
            "DekuStickCapacity30" | "deku_stick_capacity_30" => Self::DekuStickCapacity30,
            "DekuNutCapacity30" | "deku_nut_capacity_30" => Self::DekuNutCapacity30,
            "DekuNutCapacity40" | "deku_nut_capacity_40" => Self::DekuNutCapacity40,
            "BulletBag30" | "bullet_bag_30" => Self::BulletBag30,
            "BulletBag40" | "bullet_bag_40" => Self::BulletBag40,
            "BulletBag50" | "bullet_bag_50" => Self::BulletBag50,
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
            "KokiriEmerald" | "kokiri_emerald" => Self::KokiriEmerald,
            "GoronRuby" | "goron_ruby" => Self::GoronRuby,
            "ZoraSapphire" | "zora_sapphire" => Self::ZoraSapphire,
            "ForestMedallion" | "forest_medallion" => Self::ForestMedallion,
            "FireMedallion" | "fire_medallion" => Self::FireMedallion,
            "WaterMedallion" | "water_medallion" => Self::WaterMedallion,
            "ShadowMedallion" | "shadow_medallion" => Self::ShadowMedallion,
            "SpiritMedallion" | "spirit_medallion" => Self::SpiritMedallion,
            "LightMedallion" | "light_medallion" => Self::LightMedallion,
            "StoneOfAgony" | "stone_of_agony" => Self::StoneOfAgony,
            "GerudoCard" | "gerudo_card" => Self::GerudoCard,
            // Dungeon Items
            "SmallKey" | "small_key" => Self::SmallKey,
            "BossKey" | "boss_key" => Self::BossKey,
            "Map" | "map" => Self::Map,
            "Compass" | "compass" => Self::Compass,
            // Dungeon-Specific Keys
            "SmallKeyForestTemple" | "small_key_forest_temple" => Self::SmallKeyForestTemple,
            "SmallKeyFireTemple" | "small_key_fire_temple" => Self::SmallKeyFireTemple,
            "SmallKeyWaterTemple" | "small_key_water_temple" => Self::SmallKeyWaterTemple,
            "SmallKeyShadowTemple" | "small_key_shadow_temple" => Self::SmallKeyShadowTemple,
            "SmallKeySpiritTemple" | "small_key_spirit_temple" => Self::SmallKeySpiritTemple,
            "SmallKeyBottomOfTheWell" | "small_key_bottom_of_the_well" => {
                Self::SmallKeyBottomOfTheWell
            }
            "SmallKeyGerudoFortress" | "small_key_gerudo_fortress" => Self::SmallKeyGerudoFortress,
            "SmallKeyGerudoTrainingGround" | "small_key_gerudo_training_ground" => {
                Self::SmallKeyGerudoTrainingGround
            }
            "SmallKeyGanonsCastle" | "small_key_ganons_castle" => Self::SmallKeyGanonsCastle,
            "BossKeyForestTemple" | "boss_key_forest_temple" => Self::BossKeyForestTemple,
            "BossKeyFireTemple" | "boss_key_fire_temple" => Self::BossKeyFireTemple,
            "BossKeyWaterTemple" | "boss_key_water_temple" => Self::BossKeyWaterTemple,
            "BossKeyShadowTemple" | "boss_key_shadow_temple" => Self::BossKeyShadowTemple,
            "BossKeySpiritTemple" | "boss_key_spirit_temple" => Self::BossKeySpiritTemple,
            "BossKeyGanonsCastle" | "boss_key_ganons_castle" => Self::BossKeyGanonsCastle,
            // Collectibles
            "HeartContainer" | "heart_container" => Self::HeartContainer,
            "PieceOfHeart" | "piece_of_heart" => Self::PieceOfHeart,
            "GoldSkulltula" | "gold_skulltula" => Self::GoldSkulltula,
            "SmallMagicJar" | "small_magic_jar" => Self::SmallMagicJar,
            "LargeMagicJar" | "large_magic_jar" => Self::LargeMagicJar,
            "RecoveryHeart" | "recovery_heart" => Self::RecoveryHeart,
            "GreenRupee" | "green_rupee" => Self::GreenRupee,
            "BlueRupee" | "blue_rupee" => Self::BlueRupee,
            "RedRupee" | "red_rupee" => Self::RedRupee,
            "PurpleRupee" | "purple_rupee" => Self::PurpleRupee,
            "GoldRupee" | "gold_rupee" => Self::GoldRupee,
            // Special
            "Triforce" | "triforce" => Self::Triforce,
            "TriforceOfCourage" | "triforce_of_courage" => Self::TriforceOfCourage,
            "GanonBossKey" | "ganon_boss_key" => Self::GanonBossKey,
            _ => return None,
        };
        Some(item)
    }

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

    /// Returns the category of this item.
    #[must_use]
    pub const fn category(&self) -> ItemCategory {
        match self {
            // Swords
            Self::KokiriSword | Self::MasterSword | Self::BiggoronSword | Self::GiantKnife => {
                ItemCategory::Sword
            }

            // Shields
            Self::DekuShield | Self::HylianShield | Self::MirrorShield => ItemCategory::Shield,

            // Tunics
            Self::KokiriTunic | Self::GoronTunic | Self::ZoraTunic => ItemCategory::Tunic,

            // Boots
            Self::KokiriBoots | Self::IronBoots | Self::HoverBoots => ItemCategory::Boots,

            // Equipment
            Self::DekuStick
            | Self::DekuNut
            | Self::Bomb
            | Self::Bow
            | Self::Slingshot
            | Self::Boomerang
            | Self::Hookshot
            | Self::Longshot
            | Self::LensOfTruth
            | Self::MegatonHammer => ItemCategory::Equipment,

            // Arrows (Equipment)
            Self::FireArrow | Self::IceArrow | Self::LightArrow => ItemCategory::Equipment,

            // Magic spells
            Self::DinsFire | Self::FaroresWind | Self::NayrusLove => ItemCategory::Magic,

            // Ocarina
            Self::OcarinaOfTime => ItemCategory::Ocarina,

            // Bottles and contents
            Self::Bottle
            | Self::BottleRedPotion
            | Self::BottleGreenPotion
            | Self::BottleBluePotion
            | Self::BottleFairy
            | Self::BottleFish
            | Self::BottleBlueFire
            | Self::BottleBugs
            | Self::BottlePoe
            | Self::BottleBigPoe
            | Self::BottleMilk
            | Self::BottleHalfMilk
            | Self::BottleRutosLetter => ItemCategory::Bottle,

            // Adult Trade Sequence
            Self::PocketEgg
            | Self::PocketCucco
            | Self::Cojiro
            | Self::OddMushroom
            | Self::OddPotion
            | Self::PoachersSaw
            | Self::BrokenSword
            | Self::Prescription
            | Self::EyeballFrog
            | Self::Eyedrops
            | Self::ClaimCheck => ItemCategory::Trade,

            // Child Trade Sequence
            Self::WeirdEgg | Self::Chicken | Self::ZeldasLetter => ItemCategory::Trade,

            // Child Masks (trade-related in OoT)
            Self::SkullMask
            | Self::SpookyMask
            | Self::KeatonMask
            | Self::BunnyHood
            | Self::GoronMask
            | Self::ZoraMask
            | Self::GerudoMask
            | Self::MaskOfTruth => ItemCategory::Mask,

            // Songs
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
            | Self::ScarecrowSong => ItemCategory::Song,

            // Upgrades
            Self::GoronBracelet
            | Self::SilverGauntlets
            | Self::GoldenGauntlets
            | Self::SilverScale
            | Self::GoldenScale
            | Self::ChildWallet
            | Self::AdultWallet
            | Self::GiantWallet
            | Self::DekuStickCapacity20
            | Self::DekuStickCapacity30
            | Self::DekuNutCapacity30
            | Self::DekuNutCapacity40
            | Self::BulletBag30
            | Self::BulletBag40
            | Self::BulletBag50
            | Self::Quiver30
            | Self::Quiver40
            | Self::Quiver50
            | Self::BombBag20
            | Self::BombBag30
            | Self::BombBag40
            | Self::MagicMeter
            | Self::DoubleMagic
            | Self::DoubleDefense => ItemCategory::Upgrade,

            // Quest Items (Spiritual Stones and Medallions)
            Self::KokiriEmerald
            | Self::GoronRuby
            | Self::ZoraSapphire
            | Self::ForestMedallion
            | Self::FireMedallion
            | Self::WaterMedallion
            | Self::ShadowMedallion
            | Self::SpiritMedallion
            | Self::LightMedallion
            | Self::StoneOfAgony
            | Self::GerudoCard => ItemCategory::QuestItem,

            // Generic Dungeon Items
            Self::Map | Self::Compass => ItemCategory::DungeonItem,

            // Small Keys
            Self::SmallKey
            | Self::SmallKeyForestTemple
            | Self::SmallKeyFireTemple
            | Self::SmallKeyWaterTemple
            | Self::SmallKeyShadowTemple
            | Self::SmallKeySpiritTemple
            | Self::SmallKeyBottomOfTheWell
            | Self::SmallKeyGerudoFortress
            | Self::SmallKeyGerudoTrainingGround
            | Self::SmallKeyGanonsCastle => ItemCategory::SmallKey,

            // Boss Keys
            Self::BossKey
            | Self::BossKeyForestTemple
            | Self::BossKeyFireTemple
            | Self::BossKeyWaterTemple
            | Self::BossKeyShadowTemple
            | Self::BossKeySpiritTemple
            | Self::BossKeyGanonsCastle
            | Self::GanonBossKey => ItemCategory::BossKey,

            // Collectibles/Consumables
            Self::HeartContainer
            | Self::PieceOfHeart
            | Self::SmallMagicJar
            | Self::LargeMagicJar
            | Self::RecoveryHeart
            | Self::GreenRupee
            | Self::BlueRupee
            | Self::RedRupee
            | Self::PurpleRupee
            | Self::GoldRupee => ItemCategory::Consumable,

            // Tokens
            Self::GoldSkulltula => ItemCategory::Token,

            // Special
            Self::Triforce | Self::TriforceOfCourage => ItemCategory::Special,
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
                | Self::SmallKeyForestTemple
                | Self::SmallKeyFireTemple
                | Self::SmallKeyWaterTemple
                | Self::SmallKeyShadowTemple
                | Self::SmallKeySpiritTemple
                | Self::SmallKeyBottomOfTheWell
                | Self::SmallKeyGerudoFortress
                | Self::SmallKeyGerudoTrainingGround
                | Self::SmallKeyGanonsCastle
                | Self::HeartContainer
                | Self::PieceOfHeart
                | Self::GoldSkulltula
                | Self::GreenRupee
                | Self::BlueRupee
                | Self::RedRupee
                | Self::PurpleRupee
                | Self::GoldRupee
                | Self::SmallMagicJar
                | Self::LargeMagicJar
                | Self::RecoveryHeart
        )
    }

    /// Returns the maximum count for this item.
    /// Returns 1 for non-stackable items.
    #[must_use]
    pub const fn max_count(&self) -> u32 {
        match self {
            // Bombs max at 40 with biggest bomb bag
            Self::Bomb => 40,
            // Deku Sticks max at 30 with upgrade
            Self::DekuStick => 30,
            // Deku Nuts max at 40 with upgrade
            Self::DekuNut => 40,
            // 4 bottles total
            Self::Bottle => 4,
            // Dungeon-specific small key counts
            Self::SmallKeyForestTemple => 5,
            Self::SmallKeyFireTemple => 8,
            Self::SmallKeyWaterTemple => 6,
            Self::SmallKeyShadowTemple => 5,
            Self::SmallKeySpiritTemple => 5,
            Self::SmallKeyBottomOfTheWell => 3,
            Self::SmallKeyGerudoFortress => 4,
            Self::SmallKeyGerudoTrainingGround => 9,
            Self::SmallKeyGanonsCastle => 2,
            Self::SmallKey => 99, // Generic key, no specific limit
            // Heart pieces
            Self::PieceOfHeart => 36, // OoT has 36 pieces
            // Heart containers
            Self::HeartContainer => 8, // 8 from dungeons
            // Gold Skulltulas
            Self::GoldSkulltula => 100,
            // Rupees (effectively unlimited in inventory, but track collected)
            Self::GreenRupee
            | Self::BlueRupee
            | Self::RedRupee
            | Self::PurpleRupee
            | Self::GoldRupee => 999,
            // Magic/Recovery (consumables, high limit)
            Self::SmallMagicJar | Self::LargeMagicJar | Self::RecoveryHeart => 999,
            // All other items are single-obtain
            _ => 1,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::OotItem;
    use std::collections::HashSet;

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

    #[test]
    fn test_by_name_pascal_case() {
        assert_eq!(OotItem::by_name("MasterSword"), Some(OotItem::MasterSword));
        assert_eq!(OotItem::by_name("Hookshot"), Some(OotItem::Hookshot));
        assert_eq!(
            OotItem::by_name("ForestMedallion"),
            Some(OotItem::ForestMedallion)
        );
        assert_eq!(
            OotItem::by_name("SmallKeyFireTemple"),
            Some(OotItem::SmallKeyFireTemple)
        );
    }

    #[test]
    fn test_by_name_snake_case() {
        assert_eq!(OotItem::by_name("master_sword"), Some(OotItem::MasterSword));
        assert_eq!(OotItem::by_name("hookshot"), Some(OotItem::Hookshot));
        assert_eq!(
            OotItem::by_name("forest_medallion"),
            Some(OotItem::ForestMedallion)
        );
        assert_eq!(
            OotItem::by_name("small_key_fire_temple"),
            Some(OotItem::SmallKeyFireTemple)
        );
    }

    #[test]
    fn test_by_name_not_found() {
        assert_eq!(OotItem::by_name("NotAnItem"), None);
        assert_eq!(OotItem::by_name(""), None);
        assert_eq!(OotItem::by_name("invalid_item"), None);
    }

    // Additional comprehensive tests

    #[test]
    fn test_by_name_all_swords() {
        assert_eq!(OotItem::by_name("KokiriSword"), Some(OotItem::KokiriSword));
        assert_eq!(OotItem::by_name("kokiri_sword"), Some(OotItem::KokiriSword));
        assert_eq!(OotItem::by_name("MasterSword"), Some(OotItem::MasterSword));
        assert_eq!(
            OotItem::by_name("BiggoronSword"),
            Some(OotItem::BiggoronSword)
        );
        assert_eq!(OotItem::by_name("GiantKnife"), Some(OotItem::GiantKnife));
    }

    #[test]
    fn test_by_name_all_shields() {
        assert_eq!(OotItem::by_name("DekuShield"), Some(OotItem::DekuShield));
        assert_eq!(OotItem::by_name("deku_shield"), Some(OotItem::DekuShield));
        assert_eq!(
            OotItem::by_name("HylianShield"),
            Some(OotItem::HylianShield)
        );
        assert_eq!(
            OotItem::by_name("MirrorShield"),
            Some(OotItem::MirrorShield)
        );
    }

    #[test]
    fn test_by_name_all_tunics() {
        assert_eq!(OotItem::by_name("KokiriTunic"), Some(OotItem::KokiriTunic));
        assert_eq!(OotItem::by_name("GoronTunic"), Some(OotItem::GoronTunic));
        assert_eq!(OotItem::by_name("ZoraTunic"), Some(OotItem::ZoraTunic));
        assert_eq!(OotItem::by_name("zora_tunic"), Some(OotItem::ZoraTunic));
    }

    #[test]
    fn test_by_name_all_boots() {
        assert_eq!(OotItem::by_name("KokiriBoots"), Some(OotItem::KokiriBoots));
        assert_eq!(OotItem::by_name("IronBoots"), Some(OotItem::IronBoots));
        assert_eq!(OotItem::by_name("HoverBoots"), Some(OotItem::HoverBoots));
        assert_eq!(OotItem::by_name("iron_boots"), Some(OotItem::IronBoots));
    }

    #[test]
    fn test_by_name_all_arrows() {
        assert_eq!(OotItem::by_name("FireArrow"), Some(OotItem::FireArrow));
        assert_eq!(OotItem::by_name("IceArrow"), Some(OotItem::IceArrow));
        assert_eq!(OotItem::by_name("LightArrow"), Some(OotItem::LightArrow));
        assert_eq!(OotItem::by_name("light_arrow"), Some(OotItem::LightArrow));
    }

    #[test]
    fn test_by_name_all_spells() {
        assert_eq!(OotItem::by_name("DinsFire"), Some(OotItem::DinsFire));
        assert_eq!(OotItem::by_name("dins_fire"), Some(OotItem::DinsFire));
        assert_eq!(OotItem::by_name("FaroresWind"), Some(OotItem::FaroresWind));
        assert_eq!(OotItem::by_name("NayrusLove"), Some(OotItem::NayrusLove));
    }

    #[test]
    fn test_by_name_all_bottles() {
        assert_eq!(OotItem::by_name("Bottle"), Some(OotItem::Bottle));
        assert_eq!(
            OotItem::by_name("BottleRedPotion"),
            Some(OotItem::BottleRedPotion)
        );
        assert_eq!(
            OotItem::by_name("BottleGreenPotion"),
            Some(OotItem::BottleGreenPotion)
        );
        assert_eq!(
            OotItem::by_name("BottleBluePotion"),
            Some(OotItem::BottleBluePotion)
        );
        assert_eq!(OotItem::by_name("BottleFairy"), Some(OotItem::BottleFairy));
        assert_eq!(
            OotItem::by_name("bottle_blue_fire"),
            Some(OotItem::BottleBlueFire)
        );
    }

    #[test]
    fn test_by_name_adult_trade_sequence() {
        assert_eq!(OotItem::by_name("PocketEgg"), Some(OotItem::PocketEgg));
        assert_eq!(OotItem::by_name("pocket_cucco"), Some(OotItem::PocketCucco));
        assert_eq!(OotItem::by_name("Cojiro"), Some(OotItem::Cojiro));
        assert_eq!(OotItem::by_name("OddMushroom"), Some(OotItem::OddMushroom));
        assert_eq!(OotItem::by_name("ClaimCheck"), Some(OotItem::ClaimCheck));
    }

    #[test]
    fn test_by_name_child_trade_sequence() {
        assert_eq!(OotItem::by_name("WeirdEgg"), Some(OotItem::WeirdEgg));
        assert_eq!(OotItem::by_name("Chicken"), Some(OotItem::Chicken));
        assert_eq!(
            OotItem::by_name("ZeldasLetter"),
            Some(OotItem::ZeldasLetter)
        );
        assert_eq!(OotItem::by_name("SkullMask"), Some(OotItem::SkullMask));
        assert_eq!(OotItem::by_name("MaskOfTruth"), Some(OotItem::MaskOfTruth));
    }

    #[test]
    fn test_by_name_all_ocarina_songs() {
        assert_eq!(
            OotItem::by_name("ZeldasLullaby"),
            Some(OotItem::ZeldasLullaby)
        );
        assert_eq!(OotItem::by_name("EponasSong"), Some(OotItem::EponasSong));
        assert_eq!(OotItem::by_name("SariasSong"), Some(OotItem::SariasSong));
        assert_eq!(OotItem::by_name("SunsSong"), Some(OotItem::SunsSong));
        assert_eq!(OotItem::by_name("SongOfTime"), Some(OotItem::SongOfTime));
        assert_eq!(
            OotItem::by_name("SongOfStorms"),
            Some(OotItem::SongOfStorms)
        );
    }

    #[test]
    fn test_by_name_all_warp_songs() {
        assert_eq!(
            OotItem::by_name("MinuetOfForest"),
            Some(OotItem::MinuetOfForest)
        );
        assert_eq!(
            OotItem::by_name("BoleroOfFire"),
            Some(OotItem::BoleroOfFire)
        );
        assert_eq!(
            OotItem::by_name("SerenadeOfWater"),
            Some(OotItem::SerenadeOfWater)
        );
        assert_eq!(
            OotItem::by_name("NocturneOfShadow"),
            Some(OotItem::NocturneOfShadow)
        );
        assert_eq!(
            OotItem::by_name("RequiemOfSpirit"),
            Some(OotItem::RequiemOfSpirit)
        );
        assert_eq!(
            OotItem::by_name("PreludeOfLight"),
            Some(OotItem::PreludeOfLight)
        );
    }

    #[test]
    fn test_by_name_spiritual_stones_and_medallions() {
        assert_eq!(
            OotItem::by_name("KokiriEmerald"),
            Some(OotItem::KokiriEmerald)
        );
        assert_eq!(OotItem::by_name("GoronRuby"), Some(OotItem::GoronRuby));
        assert_eq!(
            OotItem::by_name("ZoraSapphire"),
            Some(OotItem::ZoraSapphire)
        );
        assert_eq!(
            OotItem::by_name("ForestMedallion"),
            Some(OotItem::ForestMedallion)
        );
        assert_eq!(
            OotItem::by_name("LightMedallion"),
            Some(OotItem::LightMedallion)
        );
    }

    #[test]
    fn test_by_name_edge_cases() {
        // Whitespace should not match
        assert_eq!(OotItem::by_name(" MasterSword"), None);
        assert_eq!(OotItem::by_name("MasterSword "), None);
        // Mixed case should not match
        assert_eq!(OotItem::by_name("mastersword"), None);
        assert_eq!(OotItem::by_name("MASTERSWORD"), None);
        // Similar but incorrect names
        assert_eq!(OotItem::by_name("Master_Sword"), None);
        assert_eq!(OotItem::by_name("master-sword"), None);
    }

    #[test]
    fn test_progressive_items_complete() {
        // All progressive items
        assert!(OotItem::Bomb.is_progressive());
        assert!(OotItem::DekuStick.is_progressive());
        assert!(OotItem::DekuNut.is_progressive());
        assert!(OotItem::Bottle.is_progressive());
        assert!(OotItem::SmallKey.is_progressive());
        assert!(OotItem::HeartContainer.is_progressive());
        assert!(OotItem::PieceOfHeart.is_progressive());
        assert!(OotItem::GoldSkulltula.is_progressive());

        // Non-progressive items that might seem progressive
        assert!(!OotItem::BottleRedPotion.is_progressive());
        assert!(!OotItem::SmallKeyFireTemple.is_progressive());
    }

    #[test]
    fn test_dungeon_items_complete() {
        // Generic dungeon items
        assert!(OotItem::SmallKey.is_dungeon_item());
        assert!(OotItem::BossKey.is_dungeon_item());
        assert!(OotItem::Map.is_dungeon_item());
        assert!(OotItem::Compass.is_dungeon_item());

        // All dungeon-specific small keys
        assert!(OotItem::SmallKeyForestTemple.is_dungeon_item());
        assert!(OotItem::SmallKeyFireTemple.is_dungeon_item());
        assert!(OotItem::SmallKeyWaterTemple.is_dungeon_item());
        assert!(OotItem::SmallKeyShadowTemple.is_dungeon_item());
        assert!(OotItem::SmallKeySpiritTemple.is_dungeon_item());
        assert!(OotItem::SmallKeyBottomOfTheWell.is_dungeon_item());
        assert!(OotItem::SmallKeyGerudoFortress.is_dungeon_item());
        assert!(OotItem::SmallKeyGerudoTrainingGround.is_dungeon_item());
        assert!(OotItem::SmallKeyGanonsCastle.is_dungeon_item());

        // All dungeon-specific boss keys
        assert!(OotItem::BossKeyForestTemple.is_dungeon_item());
        assert!(OotItem::BossKeyFireTemple.is_dungeon_item());
        assert!(OotItem::BossKeyWaterTemple.is_dungeon_item());
        assert!(OotItem::BossKeyShadowTemple.is_dungeon_item());
        assert!(OotItem::BossKeySpiritTemple.is_dungeon_item());
        assert!(OotItem::BossKeyGanonsCastle.is_dungeon_item());

        // Non-dungeon items
        assert!(!OotItem::ForestMedallion.is_dungeon_item());
        assert!(!OotItem::GanonBossKey.is_dungeon_item());
    }

    #[test]
    fn test_songs_complete() {
        // All songs should return true
        assert!(OotItem::ZeldasLullaby.is_song());
        assert!(OotItem::EponasSong.is_song());
        assert!(OotItem::SariasSong.is_song());
        assert!(OotItem::SunsSong.is_song());
        assert!(OotItem::SongOfTime.is_song());
        assert!(OotItem::SongOfStorms.is_song());
        assert!(OotItem::MinuetOfForest.is_song());
        assert!(OotItem::BoleroOfFire.is_song());
        assert!(OotItem::SerenadeOfWater.is_song());
        assert!(OotItem::NocturneOfShadow.is_song());
        assert!(OotItem::RequiemOfSpirit.is_song());
        assert!(OotItem::PreludeOfLight.is_song());
        assert!(OotItem::ScarecrowSong.is_song());

        // Non-songs
        assert!(!OotItem::OcarinaOfTime.is_song());
        assert!(!OotItem::Hookshot.is_song());
    }

    #[test]
    fn test_clone_trait() {
        let item = OotItem::MasterSword;
        #[allow(clippy::clone_on_copy)]
        let cloned = item.clone();
        assert_eq!(item, cloned);
    }

    #[test]
    fn test_copy_trait() {
        let item = OotItem::Hookshot;
        let copied = item;
        // Original still usable (Copy semantics)
        assert_eq!(item, copied);
    }

    #[test]
    fn test_debug_trait() {
        let item = OotItem::Boomerang;
        let debug_str = format!("{:?}", item);
        assert_eq!(debug_str, "Boomerang");
    }

    #[test]
    fn test_hash_trait() {
        let mut set = HashSet::new();
        set.insert(OotItem::MasterSword);
        set.insert(OotItem::Hookshot);
        set.insert(OotItem::MasterSword); // Duplicate

        assert_eq!(set.len(), 2);
        assert!(set.contains(&OotItem::MasterSword));
        assert!(set.contains(&OotItem::Hookshot));
    }

    #[test]
    fn test_eq_trait() {
        assert_eq!(OotItem::Bow, OotItem::Bow);
        assert_ne!(OotItem::Bow, OotItem::Bomb);
    }

    #[test]
    fn test_by_name_upgrades() {
        assert_eq!(
            OotItem::by_name("GoronBracelet"),
            Some(OotItem::GoronBracelet)
        );
        assert_eq!(
            OotItem::by_name("SilverGauntlets"),
            Some(OotItem::SilverGauntlets)
        );
        assert_eq!(
            OotItem::by_name("GoldenGauntlets"),
            Some(OotItem::GoldenGauntlets)
        );
        assert_eq!(OotItem::by_name("SilverScale"), Some(OotItem::SilverScale));
        assert_eq!(OotItem::by_name("GoldenScale"), Some(OotItem::GoldenScale));
        assert_eq!(OotItem::by_name("MagicMeter"), Some(OotItem::MagicMeter));
        assert_eq!(OotItem::by_name("DoubleMagic"), Some(OotItem::DoubleMagic));
    }

    #[test]
    fn test_by_name_capacity_upgrades() {
        assert_eq!(
            OotItem::by_name("DekuStickCapacity20"),
            Some(OotItem::DekuStickCapacity20)
        );
        assert_eq!(
            OotItem::by_name("deku_stick_capacity_30"),
            Some(OotItem::DekuStickCapacity30)
        );
        assert_eq!(OotItem::by_name("BulletBag30"), Some(OotItem::BulletBag30));
        assert_eq!(OotItem::by_name("Quiver40"), Some(OotItem::Quiver40));
        assert_eq!(OotItem::by_name("BombBag40"), Some(OotItem::BombBag40));
    }

    #[test]
    fn test_by_name_rupees() {
        assert_eq!(OotItem::by_name("GreenRupee"), Some(OotItem::GreenRupee));
        assert_eq!(OotItem::by_name("BlueRupee"), Some(OotItem::BlueRupee));
        assert_eq!(OotItem::by_name("RedRupee"), Some(OotItem::RedRupee));
        assert_eq!(OotItem::by_name("PurpleRupee"), Some(OotItem::PurpleRupee));
        assert_eq!(OotItem::by_name("GoldRupee"), Some(OotItem::GoldRupee));
    }

    #[test]
    fn test_by_name_special_items() {
        assert_eq!(OotItem::by_name("Triforce"), Some(OotItem::Triforce));
        assert_eq!(
            OotItem::by_name("TriforceOfCourage"),
            Some(OotItem::TriforceOfCourage)
        );
        assert_eq!(
            OotItem::by_name("GanonBossKey"),
            Some(OotItem::GanonBossKey)
        );
        assert_eq!(
            OotItem::by_name("StoneOfAgony"),
            Some(OotItem::StoneOfAgony)
        );
        assert_eq!(OotItem::by_name("GerudoCard"), Some(OotItem::GerudoCard));
    }
}
