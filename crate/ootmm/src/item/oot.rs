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
}
