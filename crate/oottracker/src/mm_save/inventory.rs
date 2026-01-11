//! Inventory items and bottles.

use crate::mm_save::constants::mm_item_ids;

// ============================================================================
// Bottle Contents
// ============================================================================

/// Bottle contents
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum MmBottle {
    #[default]
    None,
    Empty,
    RedPotion,
    GreenPotion,
    BluePotion,
    Fairy,
    DekuPrincess,
    Milk,
    MilkHalf,
    Fish,
    Bug,
    BlueFire,
    Poe,
    BigPoe,
    Water,
    HotSpringWater,
    ZoraEgg,
    GoldDust,
    MagicalMushroom,
    SeaHorse,
    ChateauRomani,
    MysteryMilk,
    MysteryMilkSpoiled,
}

impl TryFrom<u8> for MmBottle {
    type Error = u8;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        use mm_item_ids::*;
        match value {
            NONE => Ok(MmBottle::None),
            BOTTLE_EMPTY => Ok(MmBottle::Empty),
            BOTTLE_RED_POTION => Ok(MmBottle::RedPotion),
            BOTTLE_GREEN_POTION => Ok(MmBottle::GreenPotion),
            BOTTLE_BLUE_POTION => Ok(MmBottle::BluePotion),
            BOTTLE_FAIRY => Ok(MmBottle::Fairy),
            BOTTLE_DEKU_PRINCESS => Ok(MmBottle::DekuPrincess),
            BOTTLE_MILK => Ok(MmBottle::Milk),
            BOTTLE_MILK_HALF => Ok(MmBottle::MilkHalf),
            BOTTLE_FISH => Ok(MmBottle::Fish),
            BOTTLE_BUG => Ok(MmBottle::Bug),
            BOTTLE_BLUE_FIRE => Ok(MmBottle::BlueFire),
            BOTTLE_POE => Ok(MmBottle::Poe),
            BOTTLE_BIG_POE => Ok(MmBottle::BigPoe),
            BOTTLE_WATER => Ok(MmBottle::Water),
            BOTTLE_HOT_SPRING_WATER => Ok(MmBottle::HotSpringWater),
            BOTTLE_ZORA_EGG => Ok(MmBottle::ZoraEgg),
            BOTTLE_GOLD_DUST => Ok(MmBottle::GoldDust),
            BOTTLE_MUSHROOM => Ok(MmBottle::MagicalMushroom),
            BOTTLE_SEAHORSE => Ok(MmBottle::SeaHorse),
            BOTTLE_CHATEAU_ROMANI => Ok(MmBottle::ChateauRomani),
            BOTTLE_MYSTERY_MILK => Ok(MmBottle::MysteryMilk),
            BOTTLE_MYSTERY_MILK_SPOILED => Ok(MmBottle::MysteryMilkSpoiled),
            _ => Err(value),
        }
    }
}

impl From<MmBottle> for u8 {
    fn from(bottle: MmBottle) -> u8 {
        use mm_item_ids::*;
        match bottle {
            MmBottle::None => NONE,
            MmBottle::Empty => BOTTLE_EMPTY,
            MmBottle::RedPotion => BOTTLE_RED_POTION,
            MmBottle::GreenPotion => BOTTLE_GREEN_POTION,
            MmBottle::BluePotion => BOTTLE_BLUE_POTION,
            MmBottle::Fairy => BOTTLE_FAIRY,
            MmBottle::DekuPrincess => BOTTLE_DEKU_PRINCESS,
            MmBottle::Milk => BOTTLE_MILK,
            MmBottle::MilkHalf => BOTTLE_MILK_HALF,
            MmBottle::Fish => BOTTLE_FISH,
            MmBottle::Bug => BOTTLE_BUG,
            MmBottle::BlueFire => BOTTLE_BLUE_FIRE,
            MmBottle::Poe => BOTTLE_POE,
            MmBottle::BigPoe => BOTTLE_BIG_POE,
            MmBottle::Water => BOTTLE_WATER,
            MmBottle::HotSpringWater => BOTTLE_HOT_SPRING_WATER,
            MmBottle::ZoraEgg => BOTTLE_ZORA_EGG,
            MmBottle::GoldDust => BOTTLE_GOLD_DUST,
            MmBottle::MagicalMushroom => BOTTLE_MUSHROOM,
            MmBottle::SeaHorse => BOTTLE_SEAHORSE,
            MmBottle::ChateauRomani => BOTTLE_CHATEAU_ROMANI,
            MmBottle::MysteryMilk => BOTTLE_MYSTERY_MILK,
            MmBottle::MysteryMilkSpoiled => BOTTLE_MYSTERY_MILK_SPOILED,
        }
    }
}

// ============================================================================
// Inventory
// ============================================================================

/// MM inventory items (non-mask C-button items)
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct MmInventory {
    // Usable items
    pub ocarina: bool,
    pub bow: bool,
    pub fire_arrows: bool,
    pub ice_arrows: bool,
    pub light_arrows: bool,
    pub bombs: bool,
    pub bombchus: bool,
    pub deku_sticks: bool,
    pub deku_nuts: bool,
    pub magic_beans: bool,
    pub powder_keg: bool,
    pub pictograph_box: bool,
    pub lens: bool,
    pub hookshot: bool,
    pub great_fairy_sword: bool,

    // Bottles
    pub bottles: [MmBottle; 6],
}
