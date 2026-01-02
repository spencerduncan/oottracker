//! Item types for OoT and MM.

pub mod mm;
pub mod oot;
mod serde_impl;

pub use mm::MmItem;
pub use oot::OotItem;

/// Trait for converting items to and from their string representations.
///
/// This trait provides a consistent interface for serialization and deserialization
/// of item names. The string representation uses SCREAMING_SNAKE_CASE format.
pub trait ItemName: Sized {
    /// Returns the string representation of this item in SCREAMING_SNAKE_CASE format.
    ///
    /// # Example
    /// ```
    /// use ootmm::item::ItemName;
    /// use ootmm::OotItem;
    ///
    /// assert_eq!(OotItem::MasterSword.to_str(), "MASTER_SWORD");
    /// ```
    fn to_str(&self) -> &'static str;

    /// Parses an item from its string representation.
    ///
    /// Accepts SCREAMING_SNAKE_CASE format (e.g., "MASTER_SWORD").
    /// Returns `None` if the string doesn't match any known item.
    ///
    /// # Example
    /// ```
    /// use ootmm::item::ItemName;
    /// use ootmm::OotItem;
    ///
    /// assert_eq!(OotItem::from_str("MASTER_SWORD"), Some(OotItem::MasterSword));
    /// assert_eq!(OotItem::from_str("NOT_AN_ITEM"), None);
    /// ```
    fn from_str(s: &str) -> Option<Self>;
}

/// The game an item originates from.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Game {
    /// Ocarina of Time
    OcarinaOfTime,
    /// Majora's Mask
    MajorasMask,
}

/// Category of an item for classification purposes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ItemCategory {
    /// Swords (Kokiri Sword, Master Sword, etc.)
    Sword,
    /// Shields (Deku Shield, Hylian Shield, etc.)
    Shield,
    /// Tunics (OoT only - Kokiri, Goron, Zora Tunics)
    Tunic,
    /// Boots (Kokiri, Iron, Hover Boots)
    Boots,
    /// Regular masks (non-transformation)
    Mask,
    /// Transformation masks (MM - Deku, Goron, Zora, Fierce Deity)
    TransformationMask,
    /// Equipment items (Bow, Hookshot, etc.)
    Equipment,
    /// Magic spells (Din's Fire, etc.)
    Magic,
    /// Songs
    Song,
    /// Ocarinas
    Ocarina,
    /// Generic dungeon items (Map, Compass)
    DungeonItem,
    /// Small keys (both generic and dungeon-specific)
    SmallKey,
    /// Boss keys (both generic and dungeon-specific)
    BossKey,
    /// Capacity/ability upgrades
    Upgrade,
    /// Consumable items (rupees, hearts, ammo)
    Consumable,
    /// Quest items (medallions, stones, remains, etc.)
    QuestItem,
    /// Collectible tokens (Gold Skulltulas, Stray Fairies)
    Token,
    /// Bottles and bottle contents
    Bottle,
    /// Trade sequence items
    Trade,
    /// Special/unique items
    Special,
}

/// Combined item enum for both games.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Item {
    /// An item from Ocarina of Time.
    Oot(OotItem),
    /// An item from Majora's Mask.
    Mm(MmItem),
}

impl Item {
    /// Look up an Item by its string name.
    ///
    /// Tries OoT items first, then MM items. Supports both PascalCase
    /// variant names (e.g., "MasterSword") and snake_case names (e.g., "master_sword").
    ///
    /// Note: Items with identical names in both games (e.g., "Hookshot") will
    /// return the OoT variant. Use `OotItem::by_name` or `MmItem::by_name`
    /// directly if you need a specific game's item.
    #[must_use]
    pub fn by_name(name: &str) -> Option<Item> {
        OotItem::by_name(name)
            .map(Item::Oot)
            .or_else(|| MmItem::by_name(name).map(Item::Mm))
    }

    /// Returns the game this item originates from.
    #[must_use]
    pub const fn game(&self) -> Game {
        match self {
            Item::Oot(_) => Game::OcarinaOfTime,
            Item::Mm(_) => Game::MajorasMask,
        }
    }

    /// Returns the category of this item.
    #[must_use]
    pub const fn category(&self) -> ItemCategory {
        match self {
            Item::Oot(item) => item.category(),
            Item::Mm(item) => item.category(),
        }
    }

    /// Returns true if this is a progressive item that can be collected multiple times
    /// and contributes to item progression (e.g., heart pieces, skulltulas).
    #[must_use]
    pub const fn is_progressive(&self) -> bool {
        match self {
            Item::Oot(item) => item.is_progressive(),
            Item::Mm(item) => item.is_progressive(),
        }
    }

    /// Returns true if this item can stack (be collected multiple times).
    #[must_use]
    pub const fn is_stackable(&self) -> bool {
        match self {
            Item::Oot(item) => item.is_stackable(),
            Item::Mm(item) => item.is_stackable(),
        }
    }

    /// Returns the maximum count for this item.
    /// Returns 1 for non-stackable items.
    #[must_use]
    pub const fn max_count(&self) -> u32 {
        match self {
            Item::Oot(item) => item.max_count(),
            Item::Mm(item) => item.max_count(),
        }
    }
}

impl From<OotItem> for Item {
    fn from(item: OotItem) -> Self {
        Item::Oot(item)
    }
}

impl From<MmItem> for Item {
    fn from(item: MmItem) -> Self {
        Item::Mm(item)
    }
}

impl ItemName for Item {
    fn to_str(&self) -> &'static str {
        match self {
            Item::Oot(item) => {
                // Return OOT-prefixed string
                // Since we need a &'static str, we use a static lookup
                oot_item_to_prefixed_str(item)
            }
            Item::Mm(item) => {
                // Return MM-prefixed string
                mm_item_to_prefixed_str(item)
            }
        }
    }

    fn from_str(s: &str) -> Option<Self> {
        // Try OOT_ prefix first
        if let Some(rest) = s.strip_prefix("OOT_") {
            return OotItem::from_str(rest).map(Item::Oot);
        }
        // Try MM_ prefix
        if let Some(rest) = s.strip_prefix("MM_") {
            return MmItem::from_str(rest).map(Item::Mm);
        }
        None
    }
}

// Helper functions to get prefixed static strings for Item serialization.
// These use const string concatenation at compile time is not possible,
// so we use match statements with all variants.
fn oot_item_to_prefixed_str(item: &OotItem) -> &'static str {
    match item {
        OotItem::KokiriSword => "OOT_KOKIRI_SWORD",
        OotItem::MasterSword => "OOT_MASTER_SWORD",
        OotItem::BiggoronSword => "OOT_BIGGORON_SWORD",
        OotItem::GiantKnife => "OOT_GIANT_KNIFE",
        OotItem::DekuShield => "OOT_DEKU_SHIELD",
        OotItem::HylianShield => "OOT_HYLIAN_SHIELD",
        OotItem::MirrorShield => "OOT_MIRROR_SHIELD",
        OotItem::KokiriTunic => "OOT_KOKIRI_TUNIC",
        OotItem::GoronTunic => "OOT_GORON_TUNIC",
        OotItem::ZoraTunic => "OOT_ZORA_TUNIC",
        OotItem::KokiriBoots => "OOT_KOKIRI_BOOTS",
        OotItem::IronBoots => "OOT_IRON_BOOTS",
        OotItem::HoverBoots => "OOT_HOVER_BOOTS",
        OotItem::DekuStick => "OOT_DEKU_STICK",
        OotItem::DekuNut => "OOT_DEKU_NUT",
        OotItem::Bomb => "OOT_BOMB",
        OotItem::Bow => "OOT_BOW",
        OotItem::FireArrow => "OOT_FIRE_ARROW",
        OotItem::IceArrow => "OOT_ICE_ARROW",
        OotItem::LightArrow => "OOT_LIGHT_ARROW",
        OotItem::DinsFire => "OOT_DINS_FIRE",
        OotItem::FaroresWind => "OOT_FARORES_WIND",
        OotItem::NayrusLove => "OOT_NAYRUS_LOVE",
        OotItem::Slingshot => "OOT_SLINGSHOT",
        OotItem::Boomerang => "OOT_BOOMERANG",
        OotItem::Hookshot => "OOT_HOOKSHOT",
        OotItem::Longshot => "OOT_LONGSHOT",
        OotItem::LensOfTruth => "OOT_LENS_OF_TRUTH",
        OotItem::MegatonHammer => "OOT_MEGATON_HAMMER",
        OotItem::OcarinaOfTime => "OOT_OCARINA_OF_TIME",
        OotItem::Bottle => "OOT_BOTTLE",
        OotItem::BottleRedPotion => "OOT_BOTTLE_RED_POTION",
        OotItem::BottleGreenPotion => "OOT_BOTTLE_GREEN_POTION",
        OotItem::BottleBluePotion => "OOT_BOTTLE_BLUE_POTION",
        OotItem::BottleFairy => "OOT_BOTTLE_FAIRY",
        OotItem::BottleFish => "OOT_BOTTLE_FISH",
        OotItem::BottleBlueFire => "OOT_BOTTLE_BLUE_FIRE",
        OotItem::BottleBugs => "OOT_BOTTLE_BUGS",
        OotItem::BottlePoe => "OOT_BOTTLE_POE",
        OotItem::BottleBigPoe => "OOT_BOTTLE_BIG_POE",
        OotItem::BottleMilk => "OOT_BOTTLE_MILK",
        OotItem::BottleHalfMilk => "OOT_BOTTLE_HALF_MILK",
        OotItem::BottleRutosLetter => "OOT_BOTTLE_RUTOS_LETTER",
        OotItem::PocketEgg => "OOT_POCKET_EGG",
        OotItem::PocketCucco => "OOT_POCKET_CUCCO",
        OotItem::Cojiro => "OOT_COJIRO",
        OotItem::OddMushroom => "OOT_ODD_MUSHROOM",
        OotItem::OddPotion => "OOT_ODD_POTION",
        OotItem::PoachersSaw => "OOT_POACHERS_SAW",
        OotItem::BrokenSword => "OOT_BROKEN_SWORD",
        OotItem::Prescription => "OOT_PRESCRIPTION",
        OotItem::EyeballFrog => "OOT_EYEBALL_FROG",
        OotItem::Eyedrops => "OOT_EYEDROPS",
        OotItem::ClaimCheck => "OOT_CLAIM_CHECK",
        OotItem::WeirdEgg => "OOT_WEIRD_EGG",
        OotItem::Chicken => "OOT_CHICKEN",
        OotItem::ZeldasLetter => "OOT_ZELDAS_LETTER",
        OotItem::SkullMask => "OOT_SKULL_MASK",
        OotItem::SpookyMask => "OOT_SPOOKY_MASK",
        OotItem::KeatonMask => "OOT_KEATON_MASK",
        OotItem::BunnyHood => "OOT_BUNNY_HOOD",
        OotItem::GoronMask => "OOT_GORON_MASK",
        OotItem::ZoraMask => "OOT_ZORA_MASK",
        OotItem::GerudoMask => "OOT_GERUDO_MASK",
        OotItem::MaskOfTruth => "OOT_MASK_OF_TRUTH",
        OotItem::ZeldasLullaby => "OOT_ZELDAS_LULLABY",
        OotItem::EponasSong => "OOT_EPONAS_SONG",
        OotItem::SariasSong => "OOT_SARIAS_SONG",
        OotItem::SunsSong => "OOT_SUNS_SONG",
        OotItem::SongOfTime => "OOT_SONG_OF_TIME",
        OotItem::SongOfStorms => "OOT_SONG_OF_STORMS",
        OotItem::MinuetOfForest => "OOT_MINUET_OF_FOREST",
        OotItem::BoleroOfFire => "OOT_BOLERO_OF_FIRE",
        OotItem::SerenadeOfWater => "OOT_SERENADE_OF_WATER",
        OotItem::NocturneOfShadow => "OOT_NOCTURNE_OF_SHADOW",
        OotItem::RequiemOfSpirit => "OOT_REQUIEM_OF_SPIRIT",
        OotItem::PreludeOfLight => "OOT_PRELUDE_OF_LIGHT",
        OotItem::ScarecrowSong => "OOT_SCARECROW_SONG",
        OotItem::GoronBracelet => "OOT_GORON_BRACELET",
        OotItem::SilverGauntlets => "OOT_SILVER_GAUNTLETS",
        OotItem::GoldenGauntlets => "OOT_GOLDEN_GAUNTLETS",
        OotItem::SilverScale => "OOT_SILVER_SCALE",
        OotItem::GoldenScale => "OOT_GOLDEN_SCALE",
        OotItem::ChildWallet => "OOT_CHILD_WALLET",
        OotItem::AdultWallet => "OOT_ADULT_WALLET",
        OotItem::GiantWallet => "OOT_GIANT_WALLET",
        OotItem::DekuStickCapacity20 => "OOT_DEKU_STICK_CAPACITY_20",
        OotItem::DekuStickCapacity30 => "OOT_DEKU_STICK_CAPACITY_30",
        OotItem::DekuNutCapacity30 => "OOT_DEKU_NUT_CAPACITY_30",
        OotItem::DekuNutCapacity40 => "OOT_DEKU_NUT_CAPACITY_40",
        OotItem::BulletBag30 => "OOT_BULLET_BAG_30",
        OotItem::BulletBag40 => "OOT_BULLET_BAG_40",
        OotItem::BulletBag50 => "OOT_BULLET_BAG_50",
        OotItem::Quiver30 => "OOT_QUIVER_30",
        OotItem::Quiver40 => "OOT_QUIVER_40",
        OotItem::Quiver50 => "OOT_QUIVER_50",
        OotItem::BombBag20 => "OOT_BOMB_BAG_20",
        OotItem::BombBag30 => "OOT_BOMB_BAG_30",
        OotItem::BombBag40 => "OOT_BOMB_BAG_40",
        OotItem::MagicMeter => "OOT_MAGIC_METER",
        OotItem::DoubleMagic => "OOT_DOUBLE_MAGIC",
        OotItem::DoubleDefense => "OOT_DOUBLE_DEFENSE",
        OotItem::KokiriEmerald => "OOT_KOKIRI_EMERALD",
        OotItem::GoronRuby => "OOT_GORON_RUBY",
        OotItem::ZoraSapphire => "OOT_ZORA_SAPPHIRE",
        OotItem::ForestMedallion => "OOT_FOREST_MEDALLION",
        OotItem::FireMedallion => "OOT_FIRE_MEDALLION",
        OotItem::WaterMedallion => "OOT_WATER_MEDALLION",
        OotItem::ShadowMedallion => "OOT_SHADOW_MEDALLION",
        OotItem::SpiritMedallion => "OOT_SPIRIT_MEDALLION",
        OotItem::LightMedallion => "OOT_LIGHT_MEDALLION",
        OotItem::StoneOfAgony => "OOT_STONE_OF_AGONY",
        OotItem::GerudoCard => "OOT_GERUDO_CARD",
        OotItem::SmallKey => "OOT_SMALL_KEY",
        OotItem::BossKey => "OOT_BOSS_KEY",
        OotItem::Map => "OOT_MAP",
        OotItem::Compass => "OOT_COMPASS",
        OotItem::SmallKeyForestTemple => "OOT_SMALL_KEY_FOREST_TEMPLE",
        OotItem::SmallKeyFireTemple => "OOT_SMALL_KEY_FIRE_TEMPLE",
        OotItem::SmallKeyWaterTemple => "OOT_SMALL_KEY_WATER_TEMPLE",
        OotItem::SmallKeyShadowTemple => "OOT_SMALL_KEY_SHADOW_TEMPLE",
        OotItem::SmallKeySpiritTemple => "OOT_SMALL_KEY_SPIRIT_TEMPLE",
        OotItem::SmallKeyBottomOfTheWell => "OOT_SMALL_KEY_BOTTOM_OF_THE_WELL",
        OotItem::SmallKeyGerudoFortress => "OOT_SMALL_KEY_GERUDO_FORTRESS",
        OotItem::SmallKeyGerudoTrainingGround => "OOT_SMALL_KEY_GERUDO_TRAINING_GROUND",
        OotItem::SmallKeyGanonsCastle => "OOT_SMALL_KEY_GANONS_CASTLE",
        OotItem::BossKeyForestTemple => "OOT_BOSS_KEY_FOREST_TEMPLE",
        OotItem::BossKeyFireTemple => "OOT_BOSS_KEY_FIRE_TEMPLE",
        OotItem::BossKeyWaterTemple => "OOT_BOSS_KEY_WATER_TEMPLE",
        OotItem::BossKeyShadowTemple => "OOT_BOSS_KEY_SHADOW_TEMPLE",
        OotItem::BossKeySpiritTemple => "OOT_BOSS_KEY_SPIRIT_TEMPLE",
        OotItem::BossKeyGanonsCastle => "OOT_BOSS_KEY_GANONS_CASTLE",
        OotItem::HeartContainer => "OOT_HEART_CONTAINER",
        OotItem::PieceOfHeart => "OOT_PIECE_OF_HEART",
        OotItem::GoldSkulltula => "OOT_GOLD_SKULLTULA",
        OotItem::SmallMagicJar => "OOT_SMALL_MAGIC_JAR",
        OotItem::LargeMagicJar => "OOT_LARGE_MAGIC_JAR",
        OotItem::RecoveryHeart => "OOT_RECOVERY_HEART",
        OotItem::GreenRupee => "OOT_GREEN_RUPEE",
        OotItem::BlueRupee => "OOT_BLUE_RUPEE",
        OotItem::RedRupee => "OOT_RED_RUPEE",
        OotItem::PurpleRupee => "OOT_PURPLE_RUPEE",
        OotItem::GoldRupee => "OOT_GOLD_RUPEE",
        OotItem::Triforce => "OOT_TRIFORCE",
        OotItem::TriforceOfCourage => "OOT_TRIFORCE_OF_COURAGE",
        OotItem::GanonBossKey => "OOT_GANON_BOSS_KEY",
    }
}

fn mm_item_to_prefixed_str(item: &MmItem) -> &'static str {
    match item {
        MmItem::DekuMask => "MM_DEKU_MASK",
        MmItem::GoronMask => "MM_GORON_MASK",
        MmItem::ZoraMask => "MM_ZORA_MASK",
        MmItem::FierceDeityMask => "MM_FIERCE_DEITY_MASK",
        MmItem::PostmanHat => "MM_POSTMAN_HAT",
        MmItem::AllNightMask => "MM_ALL_NIGHT_MASK",
        MmItem::BlastMask => "MM_BLAST_MASK",
        MmItem::StoneMask => "MM_STONE_MASK",
        MmItem::GreatFairyMask => "MM_GREAT_FAIRY_MASK",
        MmItem::KeatonMask => "MM_KEATON_MASK",
        MmItem::BremenMask => "MM_BREMEN_MASK",
        MmItem::BunnyHood => "MM_BUNNY_HOOD",
        MmItem::DonGeroMask => "MM_DON_GERO_MASK",
        MmItem::MaskOfScents => "MM_MASK_OF_SCENTS",
        MmItem::RomaniMask => "MM_ROMANI_MASK",
        MmItem::CircusLeaderMask => "MM_CIRCUS_LEADER_MASK",
        MmItem::KafeiMask => "MM_KAFEI_MASK",
        MmItem::CouplesMask => "MM_COUPLES_MASK",
        MmItem::MaskOfTruth => "MM_MASK_OF_TRUTH",
        MmItem::KamaroMask => "MM_KAMARO_MASK",
        MmItem::GibdoMask => "MM_GIBDO_MASK",
        MmItem::GaroMask => "MM_GARO_MASK",
        MmItem::CaptainHat => "MM_CAPTAIN_HAT",
        MmItem::GiantMask => "MM_GIANT_MASK",
        MmItem::KokiriSword => "MM_KOKIRI_SWORD",
        MmItem::RazorSword => "MM_RAZOR_SWORD",
        MmItem::GildedSword => "MM_GILDED_SWORD",
        MmItem::GreatFairySword => "MM_GREAT_FAIRY_SWORD",
        MmItem::HeroShield => "MM_HERO_SHIELD",
        MmItem::MirrorShield => "MM_MIRROR_SHIELD",
        MmItem::HerosBow => "MM_HEROS_BOW",
        MmItem::FireArrow => "MM_FIRE_ARROW",
        MmItem::IceArrow => "MM_ICE_ARROW",
        MmItem::LightArrow => "MM_LIGHT_ARROW",
        MmItem::Bomb => "MM_BOMB",
        MmItem::Bombchu => "MM_BOMBCHU",
        MmItem::DekuStick => "MM_DEKU_STICK",
        MmItem::DekuNut => "MM_DEKU_NUT",
        MmItem::MagicBean => "MM_MAGIC_BEAN",
        MmItem::PowderKeg => "MM_POWDER_KEG",
        MmItem::Hookshot => "MM_HOOKSHOT",
        MmItem::LensOfTruth => "MM_LENS_OF_TRUTH",
        MmItem::PictographBox => "MM_PICTOGRAPH_BOX",
        MmItem::OcarinaOfTime => "MM_OCARINA_OF_TIME",
        MmItem::Bottle => "MM_BOTTLE",
        MmItem::BottleRedPotion => "MM_BOTTLE_RED_POTION",
        MmItem::BottleGreenPotion => "MM_BOTTLE_GREEN_POTION",
        MmItem::BottleBluePotion => "MM_BOTTLE_BLUE_POTION",
        MmItem::BottleFairy => "MM_BOTTLE_FAIRY",
        MmItem::BottleDekuPrincess => "MM_BOTTLE_DEKU_PRINCESS",
        MmItem::BottleFish => "MM_BOTTLE_FISH",
        MmItem::BottleBugs => "MM_BOTTLE_BUGS",
        MmItem::BottlePoe => "MM_BOTTLE_POE",
        MmItem::BottleBigPoe => "MM_BOTTLE_BIG_POE",
        MmItem::BottleHotSpringWater => "MM_BOTTLE_HOT_SPRING_WATER",
        MmItem::BottleZoraEgg => "MM_BOTTLE_ZORA_EGG",
        MmItem::BottleMushroom => "MM_BOTTLE_MUSHROOM",
        MmItem::BottleGoldDust => "MM_BOTTLE_GOLD_DUST",
        MmItem::BottleMilk => "MM_BOTTLE_MILK",
        MmItem::BottleHalfMilk => "MM_BOTTLE_HALF_MILK",
        MmItem::BottleChateau => "MM_BOTTLE_CHATEAU",
        MmItem::BottleSeaHorse => "MM_BOTTLE_SEA_HORSE",
        MmItem::SongOfTime => "MM_SONG_OF_TIME",
        MmItem::SongOfHealing => "MM_SONG_OF_HEALING",
        MmItem::EponasSong => "MM_EPONAS_SONG",
        MmItem::SongOfSoaring => "MM_SONG_OF_SOARING",
        MmItem::SongOfStorms => "MM_SONG_OF_STORMS",
        MmItem::SonataOfAwakening => "MM_SONATA_OF_AWAKENING",
        MmItem::GoronLullaby => "MM_GORON_LULLABY",
        MmItem::NewWaveBossaNova => "MM_NEW_WAVE_BOSSA_NOVA",
        MmItem::ElegyOfEmptiness => "MM_ELEGY_OF_EMPTINESS",
        MmItem::OathToOrder => "MM_OATH_TO_ORDER",
        MmItem::AdultWallet => "MM_ADULT_WALLET",
        MmItem::GiantWallet => "MM_GIANT_WALLET",
        MmItem::Quiver30 => "MM_QUIVER_30",
        MmItem::Quiver40 => "MM_QUIVER_40",
        MmItem::Quiver50 => "MM_QUIVER_50",
        MmItem::BombBag20 => "MM_BOMB_BAG_20",
        MmItem::BombBag30 => "MM_BOMB_BAG_30",
        MmItem::BombBag40 => "MM_BOMB_BAG_40",
        MmItem::MagicMeter => "MM_MAGIC_METER",
        MmItem::DoubleMagic => "MM_DOUBLE_MAGIC",
        MmItem::DoubleDefense => "MM_DOUBLE_DEFENSE",
        MmItem::MoonsTear => "MM_MOONS_TEAR",
        MmItem::LandTitleDeed => "MM_LAND_TITLE_DEED",
        MmItem::SwampTitleDeed => "MM_SWAMP_TITLE_DEED",
        MmItem::MountainTitleDeed => "MM_MOUNTAIN_TITLE_DEED",
        MmItem::OceanTitleDeed => "MM_OCEAN_TITLE_DEED",
        MmItem::RoomKey => "MM_ROOM_KEY",
        MmItem::LetterToKafei => "MM_LETTER_TO_KAFEI",
        MmItem::PendantOfMemories => "MM_PENDANT_OF_MEMORIES",
        MmItem::LetterToMama => "MM_LETTER_TO_MAMA",
        MmItem::SpecialDeliveryToMama => "MM_SPECIAL_DELIVERY_TO_MAMA",
        MmItem::OdolwaRemains => "MM_ODOLWA_REMAINS",
        MmItem::GohtRemains => "MM_GOHT_REMAINS",
        MmItem::GyorgRemains => "MM_GYORG_REMAINS",
        MmItem::TwinmoldRemains => "MM_TWINMOLD_REMAINS",
        MmItem::SmallKey => "MM_SMALL_KEY",
        MmItem::BossKey => "MM_BOSS_KEY",
        MmItem::Map => "MM_MAP",
        MmItem::Compass => "MM_COMPASS",
        MmItem::StrayFairy => "MM_STRAY_FAIRY",
        MmItem::SmallKeyWoodfallTemple => "MM_SMALL_KEY_WOODFALL_TEMPLE",
        MmItem::SmallKeySnowheadTemple => "MM_SMALL_KEY_SNOWHEAD_TEMPLE",
        MmItem::SmallKeyGreatBayTemple => "MM_SMALL_KEY_GREAT_BAY_TEMPLE",
        MmItem::SmallKeyStoneTowerTemple => "MM_SMALL_KEY_STONE_TOWER_TEMPLE",
        MmItem::BossKeyWoodfallTemple => "MM_BOSS_KEY_WOODFALL_TEMPLE",
        MmItem::BossKeySnowheadTemple => "MM_BOSS_KEY_SNOWHEAD_TEMPLE",
        MmItem::BossKeyGreatBayTemple => "MM_BOSS_KEY_GREAT_BAY_TEMPLE",
        MmItem::BossKeyStoneTowerTemple => "MM_BOSS_KEY_STONE_TOWER_TEMPLE",
        MmItem::StrayFairyWoodfall => "MM_STRAY_FAIRY_WOODFALL",
        MmItem::StrayFairySnowhead => "MM_STRAY_FAIRY_SNOWHEAD",
        MmItem::StrayFairyGreatBay => "MM_STRAY_FAIRY_GREAT_BAY",
        MmItem::StrayFairyStoneTower => "MM_STRAY_FAIRY_STONE_TOWER",
        MmItem::StrayFairyClockTown => "MM_STRAY_FAIRY_CLOCK_TOWN",
        MmItem::HeartContainer => "MM_HEART_CONTAINER",
        MmItem::PieceOfHeart => "MM_PIECE_OF_HEART",
        MmItem::GreenRupee => "MM_GREEN_RUPEE",
        MmItem::BlueRupee => "MM_BLUE_RUPEE",
        MmItem::RedRupee => "MM_RED_RUPEE",
        MmItem::PurpleRupee => "MM_PURPLE_RUPEE",
        MmItem::SilverRupee => "MM_SILVER_RUPEE",
        MmItem::GoldRupee => "MM_GOLD_RUPEE",
        MmItem::BomberNotebook => "MM_BOMBER_NOTEBOOK",
        MmItem::GiantsWallet => "MM_GIANTS_WALLET",
        MmItem::OceanTitleDeedTraded => "MM_OCEAN_TITLE_DEED_TRADED",
    }
}

#[cfg(test)]
mod tests {
    use super::{Game, Item, ItemCategory, ItemName, MmItem, OotItem};

    #[test]
    fn test_from_oot_item() {
        let item: Item = OotItem::MasterSword.into();
        assert_eq!(item, Item::Oot(OotItem::MasterSword));
    }

    #[test]
    fn test_from_mm_item() {
        let item: Item = MmItem::DekuMask.into();
        assert_eq!(item, Item::Mm(MmItem::DekuMask));
    }

    #[test]
    fn test_by_name_oot_specific() {
        // Items unique to OoT
        assert_eq!(
            Item::by_name("MasterSword"),
            Some(Item::Oot(OotItem::MasterSword))
        );
        assert_eq!(
            Item::by_name("master_sword"),
            Some(Item::Oot(OotItem::MasterSword))
        );
        assert_eq!(
            Item::by_name("Boomerang"),
            Some(Item::Oot(OotItem::Boomerang))
        );
    }

    #[test]
    fn test_by_name_mm_specific() {
        // Items unique to MM
        assert_eq!(Item::by_name("DekuMask"), Some(Item::Mm(MmItem::DekuMask)));
        assert_eq!(Item::by_name("deku_mask"), Some(Item::Mm(MmItem::DekuMask)));
        assert_eq!(
            Item::by_name("OdolwaRemains"),
            Some(Item::Mm(MmItem::OdolwaRemains))
        );
    }

    #[test]
    fn test_by_name_shared_returns_oot() {
        // Items that exist in both games should return OoT variant first
        assert_eq!(
            Item::by_name("Hookshot"),
            Some(Item::Oot(OotItem::Hookshot))
        );
        assert_eq!(Item::by_name("Bomb"), Some(Item::Oot(OotItem::Bomb)));
    }

    #[test]
    fn test_by_name_not_found() {
        assert_eq!(Item::by_name("NotAnItem"), None);
        assert_eq!(Item::by_name(""), None);
    }

    #[test]
    fn test_game() {
        assert_eq!(Item::Oot(OotItem::MasterSword).game(), Game::OcarinaOfTime);
        assert_eq!(Item::Mm(MmItem::DekuMask).game(), Game::MajorasMask);
    }

    #[test]
    fn test_category_swords() {
        assert_eq!(
            Item::Oot(OotItem::MasterSword).category(),
            ItemCategory::Sword
        );
        assert_eq!(
            Item::Mm(MmItem::GildedSword).category(),
            ItemCategory::Sword
        );
    }

    #[test]
    fn test_category_masks() {
        // OoT masks are regular masks (trade-related)
        assert_eq!(Item::Oot(OotItem::BunnyHood).category(), ItemCategory::Mask);
        // MM transformation masks
        assert_eq!(
            Item::Mm(MmItem::DekuMask).category(),
            ItemCategory::TransformationMask
        );
        // MM regular masks
        assert_eq!(Item::Mm(MmItem::BunnyHood).category(), ItemCategory::Mask);
    }

    #[test]
    fn test_category_songs() {
        assert_eq!(
            Item::Oot(OotItem::ZeldasLullaby).category(),
            ItemCategory::Song
        );
        assert_eq!(
            Item::Mm(MmItem::SongOfHealing).category(),
            ItemCategory::Song
        );
    }

    #[test]
    fn test_category_keys() {
        assert_eq!(
            Item::Oot(OotItem::SmallKeyFireTemple).category(),
            ItemCategory::SmallKey
        );
        assert_eq!(
            Item::Oot(OotItem::BossKeyFireTemple).category(),
            ItemCategory::BossKey
        );
        assert_eq!(
            Item::Mm(MmItem::SmallKeyWoodfallTemple).category(),
            ItemCategory::SmallKey
        );
    }

    #[test]
    fn test_is_progressive() {
        assert!(Item::Oot(OotItem::PieceOfHeart).is_progressive());
        assert!(Item::Oot(OotItem::GoldSkulltula).is_progressive());
        assert!(Item::Mm(MmItem::StrayFairy).is_progressive());
        assert!(!Item::Oot(OotItem::MasterSword).is_progressive());
        assert!(!Item::Mm(MmItem::DekuMask).is_progressive());
    }

    #[test]
    fn test_is_stackable() {
        assert!(Item::Oot(OotItem::SmallKeyFireTemple).is_stackable());
        assert!(Item::Oot(OotItem::GoldSkulltula).is_stackable());
        assert!(Item::Mm(MmItem::StrayFairyWoodfall).is_stackable());
        assert!(!Item::Oot(OotItem::MasterSword).is_stackable());
        assert!(!Item::Mm(MmItem::Hookshot).is_stackable());
    }

    #[test]
    fn test_max_count() {
        // OoT
        assert_eq!(Item::Oot(OotItem::SmallKeyFireTemple).max_count(), 8);
        assert_eq!(Item::Oot(OotItem::GoldSkulltula).max_count(), 100);
        assert_eq!(Item::Oot(OotItem::PieceOfHeart).max_count(), 36);
        assert_eq!(Item::Oot(OotItem::Bottle).max_count(), 4);
        assert_eq!(Item::Oot(OotItem::MasterSword).max_count(), 1);
        // MM
        assert_eq!(Item::Mm(MmItem::SmallKeySnowheadTemple).max_count(), 3);
        assert_eq!(Item::Mm(MmItem::StrayFairyWoodfall).max_count(), 15);
        assert_eq!(Item::Mm(MmItem::PieceOfHeart).max_count(), 52);
        assert_eq!(Item::Mm(MmItem::Bottle).max_count(), 6);
        assert_eq!(Item::Mm(MmItem::Hookshot).max_count(), 1);
    }

    // ItemName trait tests for combined Item type

    #[test]
    fn test_item_name_to_str() {
        assert_eq!(Item::Oot(OotItem::MasterSword).to_str(), "OOT_MASTER_SWORD");
        assert_eq!(Item::Oot(OotItem::Hookshot).to_str(), "OOT_HOOKSHOT");
        assert_eq!(Item::Mm(MmItem::DekuMask).to_str(), "MM_DEKU_MASK");
        assert_eq!(Item::Mm(MmItem::Hookshot).to_str(), "MM_HOOKSHOT");
    }

    #[test]
    fn test_item_name_from_str() {
        assert_eq!(
            Item::from_str("OOT_MASTER_SWORD"),
            Some(Item::Oot(OotItem::MasterSword))
        );
        assert_eq!(
            Item::from_str("OOT_HOOKSHOT"),
            Some(Item::Oot(OotItem::Hookshot))
        );
        assert_eq!(
            Item::from_str("MM_DEKU_MASK"),
            Some(Item::Mm(MmItem::DekuMask))
        );
        assert_eq!(
            Item::from_str("MM_HOOKSHOT"),
            Some(Item::Mm(MmItem::Hookshot))
        );
    }

    #[test]
    fn test_item_name_from_str_invalid() {
        assert_eq!(Item::from_str("INVALID_ITEM"), None);
        assert_eq!(Item::from_str(""), None);
        assert_eq!(Item::from_str("MASTER_SWORD"), None); // Missing prefix
        assert_eq!(Item::from_str("oot_master_sword"), None); // Wrong case
    }

    #[test]
    fn test_item_name_roundtrip() {
        let items = [
            Item::Oot(OotItem::MasterSword),
            Item::Oot(OotItem::Hookshot),
            Item::Oot(OotItem::ZeldasLullaby),
            Item::Mm(MmItem::DekuMask),
            Item::Mm(MmItem::Hookshot),
            Item::Mm(MmItem::OdolwaRemains),
        ];

        for item in items {
            let s = item.to_str();
            let parsed = Item::from_str(s);
            assert_eq!(parsed, Some(item), "Roundtrip failed for {:?}", item);
        }
    }
}
