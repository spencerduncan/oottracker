//! OoT-specific evaluation context for reading game state from N64 RAM.
//!
//! This module provides [`OotEvalContext`] which implements the [`EvalContext`] trait
//! by reading item possession and game state directly from N64 RAM addresses.
//!
//! # OoT Save Data Layout
//!
//! The OoT save data starts at base address `0x11A5D0` in RDRAM. Items, equipment,
//! songs, upgrades, and quest items are stored at various offsets from this base.
//!
//! # Example
//!
//! ```ignore
//! use ootmm::expr::{OotEvalContext, OotRamReader, Evaluator};
//!
//! // Create a RAM reader (implementation depends on your emulator interface)
//! let ram_reader = MyRamReader::new();
//!
//! // Create the evaluation context
//! let ctx = OotEvalContext::new(&ram_reader);
//!
//! // Evaluate logic expressions against the current game state
//! let evaluator = Evaluator::new(&ctx);
//! let can_enter = evaluator.eval_str("has(HOOKSHOT) && is_adult")?;
//! ```

use crate::expr::EvalContext;
use crate::settings::RandomizerSettings;
use std::collections::{HashMap, HashSet};

/// Base address of OoT save data in N64 RDRAM.
pub const OOT_SAVE_BASE: u32 = 0x11A5D0;

/// Trait for reading bytes from N64 RAM.
///
/// Implement this trait to provide access to emulator memory.
pub trait OotRamReader {
    /// Read a single byte from the given address.
    fn read_u8(&self, addr: u32) -> u8;

    /// Read a 16-bit big-endian value from the given address.
    fn read_u16(&self, addr: u32) -> u16 {
        let hi = self.read_u8(addr) as u16;
        let lo = self.read_u8(addr + 1) as u16;
        (hi << 8) | lo
    }

    /// Read a 32-bit big-endian value from the given address.
    fn read_u32(&self, addr: u32) -> u32 {
        let b0 = self.read_u8(addr) as u32;
        let b1 = self.read_u8(addr + 1) as u32;
        let b2 = self.read_u8(addr + 2) as u32;
        let b3 = self.read_u8(addr + 3) as u32;
        (b0 << 24) | (b1 << 16) | (b2 << 8) | b3
    }
}

/// Describes how to read an item's state from RAM.
///
/// This enum is provided for future extensibility and documentation purposes.
/// Currently, item checks are implemented directly in the `get_item_check` method.
#[derive(Debug, Clone, Copy)]
#[allow(dead_code)]
pub enum ItemCheck {
    /// Check if a byte at offset is non-zero.
    ByteNonZero(u16),
    /// Check if byte at offset equals or exceeds a specific value.
    ByteAtLeast(u16, u8),
    /// Check a specific bit in a byte at offset.
    BitFlag(u16, u8),
    /// Check progressive item level (strength, scale, etc.).
    /// Offset, minimum level (1-based).
    Progressive(u16, u8),
    /// Check a specific bit in 32-bit flags at offset.
    BitFlag32(u16, u8),
    /// Check if byte count is at least N.
    CountAtLeast(u16),
}

/// RAM offsets relative to save base for OoT inventory items.
/// Based on OoT save data structure.
#[allow(dead_code)]
mod offsets {
    // Inventory slots (0x00-0x17 are C-button items)
    pub const DEKU_STICKS: u16 = 0x00;
    pub const DEKU_NUTS: u16 = 0x01;
    pub const BOMBS: u16 = 0x02;
    pub const BOW: u16 = 0x03;
    pub const FIRE_ARROWS: u16 = 0x04;
    pub const DINS_FIRE: u16 = 0x05;
    pub const SLINGSHOT: u16 = 0x06;
    pub const OCARINA: u16 = 0x07;
    pub const BOMBCHU: u16 = 0x08;
    pub const HOOKSHOT: u16 = 0x09; // Hookshot/Longshot slot
    pub const ICE_ARROWS: u16 = 0x0A;
    pub const FARORES_WIND: u16 = 0x0B;
    pub const BOOMERANG: u16 = 0x0C;
    pub const LENS_OF_TRUTH: u16 = 0x0D;
    pub const MAGIC_BEANS: u16 = 0x0E;
    pub const MEGATON_HAMMER: u16 = 0x0F;
    pub const LIGHT_ARROWS: u16 = 0x10;
    pub const NAYRUS_LOVE: u16 = 0x11;
    pub const BOTTLE_1: u16 = 0x12;
    pub const BOTTLE_2: u16 = 0x13;
    pub const BOTTLE_3: u16 = 0x14;
    pub const BOTTLE_4: u16 = 0x15;
    pub const ADULT_TRADE: u16 = 0x16;
    pub const CHILD_TRADE: u16 = 0x17;

    // Ammo counts (0x8C-0x9B)
    pub const AMMO_STICKS: u16 = 0x8C;
    pub const AMMO_NUTS: u16 = 0x8D;
    pub const AMMO_BOMBS: u16 = 0x8E;
    pub const AMMO_ARROWS: u16 = 0x8F;
    pub const AMMO_SLINGSHOT: u16 = 0x92;
    pub const AMMO_BOMBCHU: u16 = 0x94;
    pub const AMMO_BEANS: u16 = 0x95;

    // Equipment flags (bitmask)
    pub const EQUIPMENT: u16 = 0x9C; // 2 bytes for swords/shields/tunics/boots

    // Upgrades (strength, scale, wallet, etc.)
    pub const UPGRADES: u16 = 0xA0; // 4 bytes of upgrade flags

    // Quest status (songs, medallions, stones)
    pub const QUEST_STATUS: u16 = 0xA4; // 4 bytes

    // Dungeon items (maps, compasses, boss keys)
    pub const DUNGEON_ITEMS: u16 = 0xA8; // Multiple bytes

    // Small key counts per dungeon
    pub const SMALL_KEYS: u16 = 0xBC; // Array of key counts

    // Gold Skulltula token count
    pub const GS_TOKENS: u16 = 0xD0; // 2 bytes

    // Magic meter
    pub const MAGIC_SIZE: u16 = 0x32; // 0 = none, 1 = normal, 2 = double
    pub const MAGIC_AMOUNT: u16 = 0x33;

    // Heart containers
    pub const HEARTS: u16 = 0x2E; // Max hearts (in 16ths)

    // Link's age (0 = adult, 1 = child)
    pub const AGE: u16 = 0x04; // In save context, not save data

    // Gerudo card flag (in quest status bits)
    // Quest items like Gerudo Card, Stone of Agony stored in quest status
}

/// Item IDs for inventory slots.
#[allow(dead_code)]
mod item_ids {
    pub const HOOKSHOT: u8 = 0x0A;
    pub const LONGSHOT: u8 = 0x0B;
    pub const OCARINA_FAIRY: u8 = 0x07;
    pub const OCARINA_TIME: u8 = 0x08;
    pub const BOTTLE_EMPTY: u8 = 0x14;
    pub const BOTTLE_FISH: u8 = 0x1F;
    pub const BOTTLE_LETTER: u8 = 0x1B;
}

/// Bit positions in quest status flags.
mod quest_bits {
    // Songs (in quest status byte 0)
    pub const SONG_LULLABY: u8 = 12;
    pub const SONG_EPONA: u8 = 13;
    pub const SONG_SARIA: u8 = 14;
    pub const SONG_SUN: u8 = 15;
    pub const SONG_TIME: u8 = 16;
    pub const SONG_STORMS: u8 = 17;
    pub const SONG_MINUET: u8 = 18;
    pub const SONG_BOLERO: u8 = 19;
    pub const SONG_SERENADE: u8 = 20;
    pub const SONG_REQUIEM: u8 = 21;
    pub const SONG_NOCTURNE: u8 = 22;
    pub const SONG_PRELUDE: u8 = 23;

    // Medallions and stones (in quest status)
    pub const MEDALLION_FOREST: u8 = 0;
    pub const MEDALLION_FIRE: u8 = 1;
    pub const MEDALLION_WATER: u8 = 2;
    pub const MEDALLION_SPIRIT: u8 = 3;
    pub const MEDALLION_SHADOW: u8 = 4;
    pub const MEDALLION_LIGHT: u8 = 5;
    pub const STONE_KOKIRI: u8 = 18 + 8; // Emerald
    pub const STONE_GORON: u8 = 19 + 8; // Ruby
    pub const STONE_ZORA: u8 = 20 + 8; // Sapphire

    // Quest items
    pub const STONE_OF_AGONY: u8 = 21 + 8;
    pub const GERUDO_CARD: u8 = 22 + 8;
}

/// Equipment bit positions.
mod equipment_bits {
    // Swords (bits 0-3 in equipment[0])
    pub const KOKIRI_SWORD: u8 = 0;
    pub const MASTER_SWORD: u8 = 1;
    pub const BIGGORON_SWORD: u8 = 2;

    // Shields (bits 4-6)
    pub const DEKU_SHIELD: u8 = 4;
    pub const HYLIAN_SHIELD: u8 = 5;
    pub const MIRROR_SHIELD: u8 = 6;

    // Tunics (bits 8-10)
    pub const KOKIRI_TUNIC: u8 = 8;
    pub const GORON_TUNIC: u8 = 9;
    pub const ZORA_TUNIC: u8 = 10;

    // Boots (bits 12-14)
    pub const KOKIRI_BOOTS: u8 = 12;
    pub const IRON_BOOTS: u8 = 13;
    pub const HOVER_BOOTS: u8 = 14;
}

/// Upgrade field positions (in upgrades u32).
#[allow(dead_code)]
mod upgrade_bits {
    pub const QUIVER_SHIFT: u8 = 0;
    pub const BOMB_BAG_SHIFT: u8 = 3;
    pub const STRENGTH_SHIFT: u8 = 6;
    pub const SCALE_SHIFT: u8 = 9;
    pub const WALLET_SHIFT: u8 = 12;
    pub const BULLET_BAG_SHIFT: u8 = 14;
    pub const STICK_CAPACITY_SHIFT: u8 = 17;
    pub const NUT_CAPACITY_SHIFT: u8 = 20;
}

/// Child trade item IDs.
#[allow(dead_code)]
mod child_trade_ids {
    pub const WEIRD_EGG: u8 = 0x21;
    pub const CHICKEN: u8 = 0x22;
    pub const ZELDA_LETTER: u8 = 0x23;
    pub const MASK_KEATON: u8 = 0x24;
    pub const MASK_SKULL: u8 = 0x25;
    pub const MASK_SPOOKY: u8 = 0x26;
    pub const MASK_BUNNY: u8 = 0x27;
    pub const MASK_GORON: u8 = 0x28;
    pub const MASK_ZORA: u8 = 0x29;
    pub const MASK_GERUDO: u8 = 0x2A;
    pub const MASK_TRUTH: u8 = 0x2B;
    pub const SOLD_OUT: u8 = 0x2C;
}

/// Adult trade item IDs.
mod adult_trade_ids {
    pub const POCKET_EGG: u8 = 0x2D;
    pub const POCKET_CUCCO: u8 = 0x2E;
    pub const COJIRO: u8 = 0x2F;
    pub const ODD_MUSHROOM: u8 = 0x30;
    pub const ODD_POTION: u8 = 0x31;
    pub const POACHERS_SAW: u8 = 0x32;
    pub const BROKEN_SWORD: u8 = 0x33;
    pub const PRESCRIPTION: u8 = 0x34;
    pub const EYEBALL_FROG: u8 = 0x35;
    pub const EYEDROPS: u8 = 0x36;
    pub const CLAIM_CHECK: u8 = 0x37;
}

/// OoT evaluation context that reads game state from N64 RAM.
///
/// This context maps item names used in logic expressions to their
/// corresponding RAM addresses and provides the [`EvalContext`] interface
/// for expression evaluation.
pub struct OotEvalContext<'a, R: OotRamReader> {
    /// RAM reader for accessing emulator memory.
    reader: &'a R,
    /// Base address of save data.
    save_base: u32,
    /// Enabled tricks (configured by user).
    tricks: HashSet<String>,
    /// Legacy settings (configured by user, for backward compatibility).
    settings: HashMap<String, bool>,
    /// Events (tracked separately, not in RAM).
    events: HashSet<String>,
    /// Address for Link's age (may differ from save data).
    age_addr: u32,
    /// Randomizer settings for logic evaluation.
    randomizer_settings: Option<&'a RandomizerSettings>,
}

impl<'a, R: OotRamReader> OotEvalContext<'a, R> {
    /// Creates a new OoT evaluation context.
    ///
    /// # Arguments
    /// * `reader` - RAM reader for accessing emulator memory
    ///
    /// Uses default save base address (`0x11A5D0`).
    pub fn new(reader: &'a R) -> Self {
        Self {
            reader,
            save_base: OOT_SAVE_BASE,
            tricks: HashSet::new(),
            settings: HashMap::new(),
            events: HashSet::new(),
            age_addr: 0x11A5D0 + 0x04, // Default age address
            randomizer_settings: None,
        }
    }

    /// Creates a new OoT evaluation context with custom save base address.
    ///
    /// # Arguments
    /// * `reader` - RAM reader for accessing emulator memory
    /// * `save_base` - Custom save data base address
    pub fn with_save_base(reader: &'a R, save_base: u32) -> Self {
        Self {
            reader,
            save_base,
            tricks: HashSet::new(),
            settings: HashMap::new(),
            events: HashSet::new(),
            age_addr: save_base + 0x04,
            randomizer_settings: None,
        }
    }

    /// Sets the randomizer settings for logic evaluation.
    ///
    /// When set, `setting()` and `trick()` calls will first check the
    /// randomizer settings before falling back to legacy settings/tricks.
    pub fn set_randomizer_settings(&mut self, settings: &'a RandomizerSettings) {
        self.randomizer_settings = Some(settings);
    }

    /// Returns the randomizer settings if set.
    #[must_use]
    pub fn randomizer_settings(&self) -> Option<&RandomizerSettings> {
        self.randomizer_settings
    }

    /// Sets the address for reading Link's age.
    pub fn set_age_addr(&mut self, addr: u32) {
        self.age_addr = addr;
    }

    /// Adds a trick to the enabled tricks set.
    pub fn add_trick(&mut self, trick: &str) {
        self.tricks.insert(trick.to_string());
    }

    /// Removes a trick from the enabled tricks set.
    pub fn remove_trick(&mut self, trick: &str) {
        self.tricks.remove(trick);
    }

    /// Sets a setting value.
    pub fn set_setting(&mut self, name: &str, value: bool) {
        self.settings.insert(name.to_string(), value);
    }

    /// Adds an event to the triggered events set.
    pub fn add_event(&mut self, event: &str) {
        self.events.insert(event.to_string());
    }

    /// Removes an event from the triggered events set.
    pub fn remove_event(&mut self, event: &str) {
        self.events.remove(event);
    }

    /// Read a byte from save data at the given offset.
    fn read_save_u8(&self, offset: u16) -> u8 {
        self.reader.read_u8(self.save_base + offset as u32)
    }

    /// Read a 16-bit value from save data at the given offset.
    fn read_save_u16(&self, offset: u16) -> u16 {
        self.reader.read_u16(self.save_base + offset as u32)
    }

    /// Read a 32-bit value from save data at the given offset.
    fn read_save_u32(&self, offset: u16) -> u32 {
        self.reader.read_u32(self.save_base + offset as u32)
    }

    /// Check if Link is currently Adult.
    fn check_is_adult(&self) -> bool {
        // Age: 0 = adult, 1 = child
        self.reader.read_u8(self.age_addr) == 0
    }

    /// Check if an inventory slot contains a specific item or any item.
    fn has_inventory_item(&self, offset: u16, expected: Option<u8>) -> bool {
        let slot = self.read_save_u8(offset);
        match expected {
            Some(id) => slot == id,
            None => slot != 0xFF, // 0xFF = empty slot
        }
    }

    /// Check if a bit is set in the equipment flags.
    fn has_equipment_bit(&self, bit: u8) -> bool {
        let equipment = self.read_save_u16(offsets::EQUIPMENT);
        (equipment & (1 << bit)) != 0
    }

    /// Check if a bit is set in the quest status flags.
    fn has_quest_bit(&self, bit: u8) -> bool {
        let quest = self.read_save_u32(offsets::QUEST_STATUS);
        (quest & (1 << bit)) != 0
    }

    /// Get an upgrade level (strength, scale, etc.).
    fn get_upgrade_level(&self, shift: u8, mask: u8) -> u8 {
        let upgrades = self.read_save_u32(offsets::UPGRADES);
        ((upgrades >> shift) & mask as u32) as u8
    }

    /// Get the hookshot level: 0 = none, 1 = hookshot, 2 = longshot.
    fn get_hookshot_level(&self) -> u8 {
        let slot = self.read_save_u8(offsets::HOOKSHOT);
        match slot {
            item_ids::HOOKSHOT => 1,
            item_ids::LONGSHOT => 2,
            _ => 0,
        }
    }

    /// Get the ocarina level: 0 = none, 1 = fairy, 2 = time.
    fn get_ocarina_level(&self) -> u8 {
        let slot = self.read_save_u8(offsets::OCARINA);
        match slot {
            item_ids::OCARINA_FAIRY => 1,
            item_ids::OCARINA_TIME => 2,
            _ => 0,
        }
    }

    /// Get the number of bottles.
    fn get_bottle_count(&self) -> u32 {
        let mut count = 0;
        for offset in [
            offsets::BOTTLE_1,
            offsets::BOTTLE_2,
            offsets::BOTTLE_3,
            offsets::BOTTLE_4,
        ] {
            let slot = self.read_save_u8(offset);
            // Any bottle content (0x14-0x1F typically)
            if (0x14..=0x1F).contains(&slot) {
                count += 1;
            }
        }
        count
    }

    /// Get the Gold Skulltula token count.
    fn get_gs_token_count(&self) -> u32 {
        self.read_save_u16(offsets::GS_TOKENS) as u32
    }

    /// Check child trade item.
    fn has_child_trade_item(&self, item_id: u8) -> bool {
        self.read_save_u8(offsets::CHILD_TRADE) == item_id
    }

    /// Check adult trade item.
    fn has_adult_trade_item(&self, item_id: u8) -> bool {
        self.read_save_u8(offsets::ADULT_TRADE) == item_id
    }

    /// Map an item name to a RAM check.
    ///
    /// Returns `Some((check, count))` where check is how to verify the item
    /// and count is the number to check for (default 1 for boolean items).
    fn get_item_check(&self, item: &str) -> Option<(bool, u32)> {
        let item_upper = item.to_uppercase();
        let item_str = item_upper.as_str();

        // Handle most items
        let result = match item_str {
            // Equipment items (inventory slots)
            "HOOKSHOT" => self.get_hookshot_level() >= 1,
            "LONGSHOT" => self.get_hookshot_level() >= 2,
            "BOW" => self.has_inventory_item(offsets::BOW, None),
            "BOMB_BAG" | "BOMBS" => {
                let level = self.get_upgrade_level(upgrade_bits::BOMB_BAG_SHIFT, 0x07);
                level >= 1
            }
            "BOMBCHU" => self.has_inventory_item(offsets::BOMBCHU, None),
            "BOOMERANG" => self.has_inventory_item(offsets::BOOMERANG, None),
            "SLINGSHOT" => self.has_inventory_item(offsets::SLINGSHOT, None),
            "OCARINA" | "OCARINA_FAIRY" => self.get_ocarina_level() >= 1,
            "OCARINA_TIME" | "OCARINA_OF_TIME" => self.get_ocarina_level() >= 2,
            "LENS_OF_TRUTH" | "LENS" => self.has_inventory_item(offsets::LENS_OF_TRUTH, None),
            "MEGATON_HAMMER" | "HAMMER" => self.has_inventory_item(offsets::MEGATON_HAMMER, None),
            "DEKU_STICK" | "DEKU_STICKS" | "STICKS" => {
                self.has_inventory_item(offsets::DEKU_STICKS, None)
            }
            "DEKU_NUT" | "DEKU_NUTS" | "NUTS" => self.has_inventory_item(offsets::DEKU_NUTS, None),
            "MAGIC_BEAN" | "MAGIC_BEANS" | "BEANS" => {
                self.has_inventory_item(offsets::MAGIC_BEANS, None)
            }

            // Magic arrows
            "FIRE_ARROW" | "FIRE_ARROWS" => self.has_inventory_item(offsets::FIRE_ARROWS, None),
            "ICE_ARROW" | "ICE_ARROWS" => self.has_inventory_item(offsets::ICE_ARROWS, None),
            "LIGHT_ARROW" | "LIGHT_ARROWS" => self.has_inventory_item(offsets::LIGHT_ARROWS, None),

            // Magic spells
            "DINS_FIRE" | "DIN" => self.has_inventory_item(offsets::DINS_FIRE, None),
            "FARORES_WIND" | "FARORE" => self.has_inventory_item(offsets::FARORES_WIND, None),
            "NAYRUS_LOVE" | "NAYRU" => self.has_inventory_item(offsets::NAYRUS_LOVE, None),

            // Bottles
            "BOTTLE" => return Some((self.get_bottle_count() >= 1, 1)),

            // Equipment (swords, shields, tunics, boots)
            "KOKIRI_SWORD" | "SWORD_KOKIRI" => self.has_equipment_bit(equipment_bits::KOKIRI_SWORD),
            "MASTER_SWORD" | "SWORD_MASTER" => self.has_equipment_bit(equipment_bits::MASTER_SWORD),
            "BIGGORON_SWORD" | "SWORD_BIGGORON" | "GIANTS_KNIFE" => {
                self.has_equipment_bit(equipment_bits::BIGGORON_SWORD)
            }

            "DEKU_SHIELD" | "SHIELD_DEKU" => self.has_equipment_bit(equipment_bits::DEKU_SHIELD),
            "HYLIAN_SHIELD" | "SHIELD_HYLIAN" => {
                self.has_equipment_bit(equipment_bits::HYLIAN_SHIELD)
            }
            "MIRROR_SHIELD" | "SHIELD_MIRROR" => {
                self.has_equipment_bit(equipment_bits::MIRROR_SHIELD)
            }

            "KOKIRI_TUNIC" | "TUNIC_KOKIRI" => self.has_equipment_bit(equipment_bits::KOKIRI_TUNIC),
            "GORON_TUNIC" | "TUNIC_GORON" => self.has_equipment_bit(equipment_bits::GORON_TUNIC),
            "ZORA_TUNIC" | "TUNIC_ZORA" => self.has_equipment_bit(equipment_bits::ZORA_TUNIC),

            "KOKIRI_BOOTS" | "BOOTS_KOKIRI" => self.has_equipment_bit(equipment_bits::KOKIRI_BOOTS),
            "IRON_BOOTS" | "BOOTS_IRON" => self.has_equipment_bit(equipment_bits::IRON_BOOTS),
            "HOVER_BOOTS" | "BOOTS_HOVER" => self.has_equipment_bit(equipment_bits::HOVER_BOOTS),

            // Progressive upgrades
            "STRENGTH" | "GORON_BRACELET" => {
                let level = self.get_upgrade_level(upgrade_bits::STRENGTH_SHIFT, 0x07);
                level >= 1
            }
            "SILVER_GAUNTLETS" => {
                let level = self.get_upgrade_level(upgrade_bits::STRENGTH_SHIFT, 0x07);
                level >= 2
            }
            "GOLDEN_GAUNTLETS" | "GOLD_GAUNTLETS" => {
                let level = self.get_upgrade_level(upgrade_bits::STRENGTH_SHIFT, 0x07);
                level >= 3
            }

            "SCALE" | "SILVER_SCALE" => {
                let level = self.get_upgrade_level(upgrade_bits::SCALE_SHIFT, 0x07);
                level >= 1
            }
            "GOLDEN_SCALE" | "GOLD_SCALE" => {
                let level = self.get_upgrade_level(upgrade_bits::SCALE_SHIFT, 0x07);
                level >= 2
            }

            "WALLET" | "ADULT_WALLET" => {
                let level = self.get_upgrade_level(upgrade_bits::WALLET_SHIFT, 0x03);
                level >= 1
            }
            "GIANT_WALLET" => {
                let level = self.get_upgrade_level(upgrade_bits::WALLET_SHIFT, 0x03);
                level >= 2
            }

            "MAGIC" | "MAGIC_METER" | "MAGIC_UPGRADE" => {
                self.read_save_u8(offsets::MAGIC_SIZE) >= 1
            }
            "DOUBLE_MAGIC" => self.read_save_u8(offsets::MAGIC_SIZE) >= 2,

            "QUIVER" => {
                let level = self.get_upgrade_level(upgrade_bits::QUIVER_SHIFT, 0x07);
                level >= 1
            }

            "BULLET_BAG" => {
                let level = self.get_upgrade_level(upgrade_bits::BULLET_BAG_SHIFT, 0x07);
                level >= 1
            }

            // Songs
            "ZELDAS_LULLABY" | "ZELDA_SONG" | "LULLABY" => {
                self.has_quest_bit(quest_bits::SONG_LULLABY)
            }
            "EPONAS_SONG" | "EPONA_SONG" | "EPONA" => self.has_quest_bit(quest_bits::SONG_EPONA),
            "SARIAS_SONG" | "SARIA_SONG" | "SARIA" => self.has_quest_bit(quest_bits::SONG_SARIA),
            "SUNS_SONG" | "SUN_SONG" => self.has_quest_bit(quest_bits::SONG_SUN),
            "SONG_OF_TIME" | "TIME_SONG" => self.has_quest_bit(quest_bits::SONG_TIME),
            "SONG_OF_STORMS" | "STORMS_SONG" | "STORMS" => {
                self.has_quest_bit(quest_bits::SONG_STORMS)
            }
            "MINUET_OF_FOREST" | "MINUET" => self.has_quest_bit(quest_bits::SONG_MINUET),
            "BOLERO_OF_FIRE" | "BOLERO" => self.has_quest_bit(quest_bits::SONG_BOLERO),
            "SERENADE_OF_WATER" | "SERENADE" => self.has_quest_bit(quest_bits::SONG_SERENADE),
            "REQUIEM_OF_SPIRIT" | "REQUIEM" => self.has_quest_bit(quest_bits::SONG_REQUIEM),
            "NOCTURNE_OF_SHADOW" | "NOCTURNE" => self.has_quest_bit(quest_bits::SONG_NOCTURNE),
            "PRELUDE_OF_LIGHT" | "PRELUDE" => self.has_quest_bit(quest_bits::SONG_PRELUDE),

            // Spiritual stones
            "KOKIRI_EMERALD" | "EMERALD" | "STONE_KOKIRI" => {
                self.has_quest_bit(quest_bits::STONE_KOKIRI)
            }
            "GORON_RUBY" | "RUBY" | "STONE_GORON" => self.has_quest_bit(quest_bits::STONE_GORON),
            "ZORA_SAPPHIRE" | "SAPPHIRE" | "STONE_ZORA" => {
                self.has_quest_bit(quest_bits::STONE_ZORA)
            }

            // Medallions
            "FOREST_MEDALLION" | "MEDALLION_FOREST" => {
                self.has_quest_bit(quest_bits::MEDALLION_FOREST)
            }
            "FIRE_MEDALLION" | "MEDALLION_FIRE" => self.has_quest_bit(quest_bits::MEDALLION_FIRE),
            "WATER_MEDALLION" | "MEDALLION_WATER" => {
                self.has_quest_bit(quest_bits::MEDALLION_WATER)
            }
            "SPIRIT_MEDALLION" | "MEDALLION_SPIRIT" => {
                self.has_quest_bit(quest_bits::MEDALLION_SPIRIT)
            }
            "SHADOW_MEDALLION" | "MEDALLION_SHADOW" => {
                self.has_quest_bit(quest_bits::MEDALLION_SHADOW)
            }
            "LIGHT_MEDALLION" | "MEDALLION_LIGHT" => {
                self.has_quest_bit(quest_bits::MEDALLION_LIGHT)
            }

            // Quest items
            "GERUDO_CARD" | "GERUDO_MEMBERSHIP_CARD" => self.has_quest_bit(quest_bits::GERUDO_CARD),
            "STONE_OF_AGONY" | "AGONY" => self.has_quest_bit(quest_bits::STONE_OF_AGONY),

            // Child trade items
            "WEIRD_EGG" => self.has_child_trade_item(child_trade_ids::WEIRD_EGG),
            "CHICKEN" => self.has_child_trade_item(child_trade_ids::CHICKEN),
            "ZELDA_LETTER" | "ZELDAS_LETTER" => {
                self.has_child_trade_item(child_trade_ids::ZELDA_LETTER)
            }
            "MASK_KEATON" | "KEATON_MASK" => {
                self.has_child_trade_item(child_trade_ids::MASK_KEATON)
            }
            "MASK_SKULL" | "SKULL_MASK" => self.has_child_trade_item(child_trade_ids::MASK_SKULL),
            "MASK_SPOOKY" | "SPOOKY_MASK" => {
                self.has_child_trade_item(child_trade_ids::MASK_SPOOKY)
            }
            "MASK_BUNNY" | "BUNNY_HOOD" => self.has_child_trade_item(child_trade_ids::MASK_BUNNY),
            "MASK_GORON" | "GORON_MASK" => self.has_child_trade_item(child_trade_ids::MASK_GORON),
            "MASK_ZORA" | "ZORA_MASK" => self.has_child_trade_item(child_trade_ids::MASK_ZORA),
            "MASK_GERUDO" | "GERUDO_MASK" => {
                self.has_child_trade_item(child_trade_ids::MASK_GERUDO)
            }
            "MASK_TRUTH" | "MASK_OF_TRUTH" => {
                self.has_child_trade_item(child_trade_ids::MASK_TRUTH)
            }

            // Adult trade items
            "POCKET_EGG" => self.has_adult_trade_item(adult_trade_ids::POCKET_EGG),
            "POCKET_CUCCO" => self.has_adult_trade_item(adult_trade_ids::POCKET_CUCCO),
            "COJIRO" => self.has_adult_trade_item(adult_trade_ids::COJIRO),
            "ODD_MUSHROOM" => self.has_adult_trade_item(adult_trade_ids::ODD_MUSHROOM),
            "ODD_POTION" => self.has_adult_trade_item(adult_trade_ids::ODD_POTION),
            "POACHERS_SAW" => self.has_adult_trade_item(adult_trade_ids::POACHERS_SAW),
            "BROKEN_SWORD" | "BROKEN_GORON_SWORD" => {
                self.has_adult_trade_item(adult_trade_ids::BROKEN_SWORD)
            }
            "PRESCRIPTION" => self.has_adult_trade_item(adult_trade_ids::PRESCRIPTION),
            "EYEBALL_FROG" => self.has_adult_trade_item(adult_trade_ids::EYEBALL_FROG),
            "EYE_DROPS" | "EYEDROPS" => self.has_adult_trade_item(adult_trade_ids::EYEDROPS),
            "CLAIM_CHECK" => self.has_adult_trade_item(adult_trade_ids::CLAIM_CHECK),

            // Gold Skulltula tokens - special handling for count
            "GS_TOKEN" | "GOLD_SKULLTULA" | "SKULLTULA_TOKEN" => {
                return Some((true, self.get_gs_token_count()))
            }

            _ => return None,
        };

        Some((result, 1))
    }
}

impl<R: OotRamReader> EvalContext for OotEvalContext<'_, R> {
    fn has_item(&self, item: &str, count: u32) -> bool {
        match self.get_item_check(item) {
            Some((has, item_count)) => {
                if item.to_uppercase().contains("GS_TOKEN")
                    || item.to_uppercase().contains("SKULLTULA")
                {
                    // For tokens, compare the count
                    item_count >= count
                } else {
                    // For boolean items, just check presence
                    has && count <= 1
                }
            }
            None => false,
        }
    }

    fn event(&self, name: &str) -> bool {
        self.events.contains(name)
    }

    fn setting(&self, name: &str) -> Option<bool> {
        // First check randomizer settings if available
        if let Some(rs) = self.randomizer_settings {
            if let Some(value) = rs.get_bool_setting(name) {
                return Some(value);
            }
        }
        // Fall back to legacy settings
        self.settings.get(name).copied()
    }

    fn setting_value(&self, name: &str, value: &str) -> bool {
        // Delegate to randomizer settings if available
        if let Some(rs) = self.randomizer_settings {
            return rs.check_setting_value(name, value);
        }
        false
    }

    fn trick(&self, name: &str) -> bool {
        // Check local tricks first
        if self.tricks.contains(name) {
            return true;
        }
        // Then check randomizer settings tricks if available
        if let Some(rs) = self.randomizer_settings {
            return rs.has_trick(name);
        }
        false
    }

    fn is_adult(&self) -> bool {
        self.check_is_adult()
    }

    fn is_child(&self) -> bool {
        !self.check_is_adult()
    }

    fn mm_time(&self) -> u32 {
        // OoT doesn't have MM time system, return 0
        0
    }
}

/// Builder for [`OotEvalContext`].
///
/// Provides a fluent API for constructing OoT evaluation contexts with
/// pre-configured tricks, settings, and events.
pub struct OotEvalContextBuilder<'a, R: OotRamReader> {
    ctx: OotEvalContext<'a, R>,
}

impl<'a, R: OotRamReader> OotEvalContextBuilder<'a, R> {
    /// Creates a new builder with the given RAM reader.
    pub fn new(reader: &'a R) -> Self {
        Self {
            ctx: OotEvalContext::new(reader),
        }
    }

    /// Sets a custom save base address.
    #[must_use]
    pub fn with_save_base(mut self, base: u32) -> Self {
        self.ctx.save_base = base;
        self.ctx.age_addr = base + 0x04;
        self
    }

    /// Sets a custom age address.
    #[must_use]
    pub fn with_age_addr(mut self, addr: u32) -> Self {
        self.ctx.age_addr = addr;
        self
    }

    /// Adds a trick.
    #[must_use]
    pub fn with_trick(mut self, trick: &str) -> Self {
        self.ctx.add_trick(trick);
        self
    }

    /// Sets a legacy setting.
    #[must_use]
    pub fn with_setting(mut self, name: &str, value: bool) -> Self {
        self.ctx.set_setting(name, value);
        self
    }

    /// Sets the randomizer settings.
    #[must_use]
    pub fn with_randomizer_settings(mut self, settings: &'a RandomizerSettings) -> Self {
        self.ctx.set_randomizer_settings(settings);
        self
    }

    /// Adds an event.
    #[must_use]
    pub fn with_event(mut self, event: &str) -> Self {
        self.ctx.add_event(event);
        self
    }

    /// Builds the [`OotEvalContext`].
    #[must_use]
    pub fn build(self) -> OotEvalContext<'a, R> {
        self.ctx
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Mock RAM reader for testing.
    struct MockRam {
        data: HashMap<u32, u8>,
    }

    impl MockRam {
        fn new() -> Self {
            Self {
                data: HashMap::new(),
            }
        }

        fn set(&mut self, addr: u32, value: u8) -> &mut Self {
            self.data.insert(addr, value);
            self
        }

        fn set_u16(&mut self, addr: u32, value: u16) -> &mut Self {
            self.data.insert(addr, (value >> 8) as u8);
            self.data.insert(addr + 1, (value & 0xFF) as u8);
            self
        }

        fn set_u32(&mut self, addr: u32, value: u32) -> &mut Self {
            self.data.insert(addr, (value >> 24) as u8);
            self.data.insert(addr + 1, ((value >> 16) & 0xFF) as u8);
            self.data.insert(addr + 2, ((value >> 8) & 0xFF) as u8);
            self.data.insert(addr + 3, (value & 0xFF) as u8);
            self
        }
    }

    impl OotRamReader for MockRam {
        fn read_u8(&self, addr: u32) -> u8 {
            self.data.get(&addr).copied().unwrap_or(0xFF)
        }
    }

    const BASE: u32 = OOT_SAVE_BASE;

    // --- Basic tests ---

    #[test]
    fn test_new_context() {
        let ram = MockRam::new();
        let ctx = OotEvalContext::new(&ram);
        assert_eq!(ctx.save_base, OOT_SAVE_BASE);
    }

    #[test]
    fn test_custom_save_base() {
        let ram = MockRam::new();
        let ctx = OotEvalContext::with_save_base(&ram, 0x200000);
        assert_eq!(ctx.save_base, 0x200000);
    }

    // --- Inventory tests ---

    #[test]
    fn test_has_hookshot() {
        let mut ram = MockRam::new();
        ram.set(BASE + offsets::HOOKSHOT as u32, item_ids::HOOKSHOT);

        let ctx = OotEvalContext::new(&ram);
        assert!(ctx.has_item("HOOKSHOT", 1));
        assert!(!ctx.has_item("LONGSHOT", 1));
    }

    #[test]
    fn test_has_longshot() {
        let mut ram = MockRam::new();
        ram.set(BASE + offsets::HOOKSHOT as u32, item_ids::LONGSHOT);

        let ctx = OotEvalContext::new(&ram);
        assert!(ctx.has_item("HOOKSHOT", 1)); // Longshot includes hookshot
        assert!(ctx.has_item("LONGSHOT", 1));
    }

    #[test]
    fn test_has_bow() {
        let mut ram = MockRam::new();
        ram.set(BASE + offsets::BOW as u32, 0x03); // Bow item ID

        let ctx = OotEvalContext::new(&ram);
        assert!(ctx.has_item("BOW", 1));
    }

    #[test]
    fn test_has_boomerang() {
        let mut ram = MockRam::new();
        ram.set(BASE + offsets::BOOMERANG as u32, 0x0E); // Boomerang item ID

        let ctx = OotEvalContext::new(&ram);
        assert!(ctx.has_item("BOOMERANG", 1));
    }

    #[test]
    fn test_has_hammer() {
        let mut ram = MockRam::new();
        ram.set(BASE + offsets::MEGATON_HAMMER as u32, 0x11); // Hammer item ID

        let ctx = OotEvalContext::new(&ram);
        assert!(ctx.has_item("HAMMER", 1));
        assert!(ctx.has_item("MEGATON_HAMMER", 1));
    }

    #[test]
    fn test_missing_item() {
        let ram = MockRam::new(); // All slots empty (0xFF)
        let ctx = OotEvalContext::new(&ram);
        assert!(!ctx.has_item("HOOKSHOT", 1));
        assert!(!ctx.has_item("BOW", 1));
    }

    // --- Equipment tests ---

    #[test]
    fn test_has_kokiri_sword() {
        let mut ram = MockRam::new();
        ram.set_u16(
            BASE + offsets::EQUIPMENT as u32,
            1 << equipment_bits::KOKIRI_SWORD,
        );

        let ctx = OotEvalContext::new(&ram);
        assert!(ctx.has_item("KOKIRI_SWORD", 1));
    }

    #[test]
    fn test_has_iron_boots() {
        let mut ram = MockRam::new();
        ram.set_u16(
            BASE + offsets::EQUIPMENT as u32,
            1 << equipment_bits::IRON_BOOTS,
        );

        let ctx = OotEvalContext::new(&ram);
        assert!(ctx.has_item("IRON_BOOTS", 1));
    }

    #[test]
    fn test_has_hover_boots() {
        let mut ram = MockRam::new();
        ram.set_u16(
            BASE + offsets::EQUIPMENT as u32,
            1 << equipment_bits::HOVER_BOOTS,
        );

        let ctx = OotEvalContext::new(&ram);
        assert!(ctx.has_item("HOVER_BOOTS", 1));
    }

    // --- Upgrade tests ---

    #[test]
    fn test_has_strength() {
        let mut ram = MockRam::new();
        // Strength level 1 (Goron Bracelet)
        ram.set_u32(
            BASE + offsets::UPGRADES as u32,
            1 << upgrade_bits::STRENGTH_SHIFT,
        );

        let ctx = OotEvalContext::new(&ram);
        assert!(ctx.has_item("STRENGTH", 1));
        assert!(ctx.has_item("GORON_BRACELET", 1));
        assert!(!ctx.has_item("SILVER_GAUNTLETS", 1));
    }

    #[test]
    fn test_has_silver_gauntlets() {
        let mut ram = MockRam::new();
        // Strength level 2
        ram.set_u32(
            BASE + offsets::UPGRADES as u32,
            2 << upgrade_bits::STRENGTH_SHIFT,
        );

        let ctx = OotEvalContext::new(&ram);
        assert!(ctx.has_item("STRENGTH", 1));
        assert!(ctx.has_item("SILVER_GAUNTLETS", 1));
        assert!(!ctx.has_item("GOLDEN_GAUNTLETS", 1));
    }

    #[test]
    fn test_has_golden_gauntlets() {
        let mut ram = MockRam::new();
        // Strength level 3
        ram.set_u32(
            BASE + offsets::UPGRADES as u32,
            3 << upgrade_bits::STRENGTH_SHIFT,
        );

        let ctx = OotEvalContext::new(&ram);
        assert!(ctx.has_item("STRENGTH", 1));
        assert!(ctx.has_item("SILVER_GAUNTLETS", 1));
        assert!(ctx.has_item("GOLDEN_GAUNTLETS", 1));
    }

    #[test]
    fn test_has_scale() {
        let mut ram = MockRam::new();
        ram.set_u32(
            BASE + offsets::UPGRADES as u32,
            1 << upgrade_bits::SCALE_SHIFT,
        );

        let ctx = OotEvalContext::new(&ram);
        assert!(ctx.has_item("SCALE", 1));
        assert!(!ctx.has_item("GOLDEN_SCALE", 1));
    }

    #[test]
    fn test_has_magic() {
        let mut ram = MockRam::new();
        ram.set(BASE + offsets::MAGIC_SIZE as u32, 1);

        let ctx = OotEvalContext::new(&ram);
        assert!(ctx.has_item("MAGIC", 1));
        assert!(!ctx.has_item("DOUBLE_MAGIC", 1));
    }

    #[test]
    fn test_has_double_magic() {
        let mut ram = MockRam::new();
        ram.set(BASE + offsets::MAGIC_SIZE as u32, 2);

        let ctx = OotEvalContext::new(&ram);
        assert!(ctx.has_item("MAGIC", 1));
        assert!(ctx.has_item("DOUBLE_MAGIC", 1));
    }

    // --- Song tests ---

    #[test]
    fn test_has_zeldas_lullaby() {
        let mut ram = MockRam::new();
        ram.set_u32(
            BASE + offsets::QUEST_STATUS as u32,
            1 << quest_bits::SONG_LULLABY,
        );

        let ctx = OotEvalContext::new(&ram);
        assert!(ctx.has_item("ZELDAS_LULLABY", 1));
        assert!(ctx.has_item("ZELDA_SONG", 1));
    }

    #[test]
    fn test_has_eponas_song() {
        let mut ram = MockRam::new();
        ram.set_u32(
            BASE + offsets::QUEST_STATUS as u32,
            1 << quest_bits::SONG_EPONA,
        );

        let ctx = OotEvalContext::new(&ram);
        assert!(ctx.has_item("EPONAS_SONG", 1));
        assert!(ctx.has_item("EPONA_SONG", 1));
    }

    #[test]
    fn test_has_sarias_song() {
        let mut ram = MockRam::new();
        ram.set_u32(
            BASE + offsets::QUEST_STATUS as u32,
            1 << quest_bits::SONG_SARIA,
        );

        let ctx = OotEvalContext::new(&ram);
        assert!(ctx.has_item("SARIAS_SONG", 1));
        assert!(ctx.has_item("SARIA_SONG", 1));
    }

    // --- Medallion tests ---

    #[test]
    fn test_has_forest_medallion() {
        let mut ram = MockRam::new();
        ram.set_u32(
            BASE + offsets::QUEST_STATUS as u32,
            1 << quest_bits::MEDALLION_FOREST,
        );

        let ctx = OotEvalContext::new(&ram);
        assert!(ctx.has_item("FOREST_MEDALLION", 1));
        assert!(ctx.has_item("MEDALLION_FOREST", 1));
    }

    #[test]
    fn test_has_multiple_medallions() {
        let mut ram = MockRam::new();
        let flags = (1 << quest_bits::MEDALLION_FOREST)
            | (1 << quest_bits::MEDALLION_FIRE)
            | (1 << quest_bits::MEDALLION_WATER);
        ram.set_u32(BASE + offsets::QUEST_STATUS as u32, flags);

        let ctx = OotEvalContext::new(&ram);
        assert!(ctx.has_item("FOREST_MEDALLION", 1));
        assert!(ctx.has_item("FIRE_MEDALLION", 1));
        assert!(ctx.has_item("WATER_MEDALLION", 1));
        assert!(!ctx.has_item("SPIRIT_MEDALLION", 1));
    }

    // --- Quest item tests ---

    #[test]
    fn test_has_gerudo_card() {
        let mut ram = MockRam::new();
        ram.set_u32(
            BASE + offsets::QUEST_STATUS as u32,
            1 << quest_bits::GERUDO_CARD,
        );

        let ctx = OotEvalContext::new(&ram);
        assert!(ctx.has_item("GERUDO_CARD", 1));
    }

    // --- Trade item tests ---

    #[test]
    fn test_has_zelda_letter() {
        let mut ram = MockRam::new();
        ram.set(
            BASE + offsets::CHILD_TRADE as u32,
            child_trade_ids::ZELDA_LETTER,
        );

        let ctx = OotEvalContext::new(&ram);
        assert!(ctx.has_item("ZELDA_LETTER", 1));
    }

    #[test]
    fn test_has_chicken() {
        let mut ram = MockRam::new();
        ram.set(BASE + offsets::CHILD_TRADE as u32, child_trade_ids::CHICKEN);

        let ctx = OotEvalContext::new(&ram);
        assert!(ctx.has_item("CHICKEN", 1));
    }

    #[test]
    fn test_has_skull_mask() {
        let mut ram = MockRam::new();
        ram.set(
            BASE + offsets::CHILD_TRADE as u32,
            child_trade_ids::MASK_SKULL,
        );

        let ctx = OotEvalContext::new(&ram);
        assert!(ctx.has_item("MASK_SKULL", 1));
        assert!(ctx.has_item("SKULL_MASK", 1));
    }

    // --- GS Token tests ---

    #[test]
    fn test_has_gs_tokens() {
        let mut ram = MockRam::new();
        ram.set_u16(BASE + offsets::GS_TOKENS as u32, 50);

        let ctx = OotEvalContext::new(&ram);
        assert!(ctx.has_item("GS_TOKEN", 10));
        assert!(ctx.has_item("GS_TOKEN", 50));
        assert!(!ctx.has_item("GS_TOKEN", 51));
    }

    // --- Age tests ---

    #[test]
    fn test_is_adult() {
        let mut ram = MockRam::new();
        ram.set(BASE + 0x04, 0); // 0 = adult

        let ctx = OotEvalContext::new(&ram);
        assert!(ctx.is_adult());
        assert!(!ctx.is_child());
    }

    #[test]
    fn test_is_child() {
        let mut ram = MockRam::new();
        ram.set(BASE + 0x04, 1); // 1 = child

        let ctx = OotEvalContext::new(&ram);
        assert!(ctx.is_child());
        assert!(!ctx.is_adult());
    }

    // --- Trick/Setting/Event tests ---

    #[test]
    fn test_tricks() {
        let ram = MockRam::new();
        let mut ctx = OotEvalContext::new(&ram);

        assert!(!ctx.trick("hover_boost"));
        ctx.add_trick("hover_boost");
        assert!(ctx.trick("hover_boost"));
        ctx.remove_trick("hover_boost");
        assert!(!ctx.trick("hover_boost"));
    }

    #[test]
    fn test_settings() {
        let ram = MockRam::new();
        let mut ctx = OotEvalContext::new(&ram);

        assert_eq!(ctx.setting("shuffle_songs"), None);
        ctx.set_setting("shuffle_songs", true);
        assert_eq!(ctx.setting("shuffle_songs"), Some(true));
    }

    #[test]
    fn test_events() {
        let ram = MockRam::new();
        let mut ctx = OotEvalContext::new(&ram);

        assert!(!ctx.event("MIDO_MOVED"));
        ctx.add_event("MIDO_MOVED");
        assert!(ctx.event("MIDO_MOVED"));
        ctx.remove_event("MIDO_MOVED");
        assert!(!ctx.event("MIDO_MOVED"));
    }

    // --- Builder tests ---

    #[test]
    fn test_builder() {
        let ram = MockRam::new();
        let ctx = OotEvalContextBuilder::new(&ram)
            .with_trick("hover_boost")
            .with_setting("shuffle_songs", true)
            .with_event("MIDO_MOVED")
            .build();

        assert!(ctx.trick("hover_boost"));
        assert_eq!(ctx.setting("shuffle_songs"), Some(true));
        assert!(ctx.event("MIDO_MOVED"));
    }

    #[test]
    fn test_builder_with_custom_base() {
        let ram = MockRam::new();
        let ctx = OotEvalContextBuilder::new(&ram)
            .with_save_base(0x200000)
            .build();

        assert_eq!(ctx.save_base, 0x200000);
    }

    // --- Case insensitivity tests ---

    #[test]
    fn test_case_insensitive_items() {
        let mut ram = MockRam::new();
        ram.set(BASE + offsets::HOOKSHOT as u32, item_ids::HOOKSHOT);

        let ctx = OotEvalContext::new(&ram);
        assert!(ctx.has_item("HOOKSHOT", 1));
        assert!(ctx.has_item("hookshot", 1));
        assert!(ctx.has_item("Hookshot", 1));
    }

    // --- Unknown item tests ---

    #[test]
    fn test_unknown_item() {
        let ram = MockRam::new();
        let ctx = OotEvalContext::new(&ram);
        assert!(!ctx.has_item("UNKNOWN_ITEM", 1));
    }

    // --- MM time test ---

    #[test]
    fn test_mm_time_returns_zero() {
        let ram = MockRam::new();
        let ctx = OotEvalContext::new(&ram);
        assert_eq!(ctx.mm_time(), 0);
    }

    // --- Ocarina tests ---

    #[test]
    fn test_has_fairy_ocarina() {
        let mut ram = MockRam::new();
        ram.set(BASE + offsets::OCARINA as u32, item_ids::OCARINA_FAIRY);

        let ctx = OotEvalContext::new(&ram);
        assert!(ctx.has_item("OCARINA", 1));
        assert!(ctx.has_item("OCARINA_FAIRY", 1));
        assert!(!ctx.has_item("OCARINA_TIME", 1));
    }

    #[test]
    fn test_has_ocarina_of_time() {
        let mut ram = MockRam::new();
        ram.set(BASE + offsets::OCARINA as u32, item_ids::OCARINA_TIME);

        let ctx = OotEvalContext::new(&ram);
        assert!(ctx.has_item("OCARINA", 1));
        assert!(ctx.has_item("OCARINA_TIME", 1));
    }

    // --- Bomb bag tests ---

    #[test]
    fn test_has_bomb_bag() {
        let mut ram = MockRam::new();
        // Bomb bag level 1
        ram.set_u32(
            BASE + offsets::UPGRADES as u32,
            1 << upgrade_bits::BOMB_BAG_SHIFT,
        );

        let ctx = OotEvalContext::new(&ram);
        assert!(ctx.has_item("BOMB_BAG", 1));
        assert!(ctx.has_item("BOMBS", 1));
    }

    // --- Bottle tests ---

    #[test]
    fn test_has_bottle() {
        let mut ram = MockRam::new();
        ram.set(BASE + offsets::BOTTLE_1 as u32, 0x14); // Empty bottle

        let ctx = OotEvalContext::new(&ram);
        assert!(ctx.has_item("BOTTLE", 1));
    }

    #[test]
    fn test_multiple_bottles() {
        let mut ram = MockRam::new();
        ram.set(BASE + offsets::BOTTLE_1 as u32, 0x14);
        ram.set(BASE + offsets::BOTTLE_2 as u32, 0x15);
        ram.set(BASE + offsets::BOTTLE_3 as u32, 0x16);

        let ctx = OotEvalContext::new(&ram);
        assert!(ctx.has_item("BOTTLE", 1));
        // Bottle count check would need special handling
    }

    // --- Magic spell tests ---

    #[test]
    fn test_has_dins_fire() {
        let mut ram = MockRam::new();
        ram.set(BASE + offsets::DINS_FIRE as u32, 0x05);

        let ctx = OotEvalContext::new(&ram);
        assert!(ctx.has_item("DINS_FIRE", 1));
        assert!(ctx.has_item("DIN", 1));
    }

    #[test]
    fn test_has_farores_wind() {
        let mut ram = MockRam::new();
        ram.set(BASE + offsets::FARORES_WIND as u32, 0x0D);

        let ctx = OotEvalContext::new(&ram);
        assert!(ctx.has_item("FARORES_WIND", 1));
        assert!(ctx.has_item("FARORE", 1));
    }

    #[test]
    fn test_has_nayrus_love() {
        let mut ram = MockRam::new();
        ram.set(BASE + offsets::NAYRUS_LOVE as u32, 0x13);

        let ctx = OotEvalContext::new(&ram);
        assert!(ctx.has_item("NAYRUS_LOVE", 1));
        assert!(ctx.has_item("NAYRU", 1));
    }

    // --- RandomizerSettings integration tests ---

    #[test]
    fn test_setting_with_randomizer_settings_bool() {
        use crate::settings::RandomizerSettings;

        let ram = MockRam::new();
        let mut rs = RandomizerSettings::new();
        rs.ageless_boots = true;
        rs.er_moon = false;

        let ctx = OotEvalContextBuilder::new(&ram)
            .with_randomizer_settings(&rs)
            .build();

        assert_eq!(ctx.setting("agelessBoots"), Some(true));
        assert_eq!(ctx.setting("erMoon"), Some(false));
        assert_eq!(ctx.setting("unknownSetting"), None);
    }

    #[test]
    fn test_setting_value_with_randomizer_settings() {
        use crate::settings::{OotDungeon, RandomizerSettings};

        let ram = MockRam::new();
        let mut rs = RandomizerSettings::new();
        rs.open_dungeons_oot.insert(OotDungeon::DodongosCavern);
        rs.open_dungeons_oot.insert(OotDungeon::Shadow);

        let ctx = OotEvalContextBuilder::new(&ram)
            .with_randomizer_settings(&rs)
            .build();

        assert!(ctx.setting_value("openDungeonsOot", "DC"));
        assert!(ctx.setting_value("openDungeonsOot", "Shadow"));
        assert!(!ctx.setting_value("openDungeonsOot", "Water"));
        assert!(!ctx.setting_value("unknownSetting", "value"));
    }

    #[test]
    fn test_setting_value_with_enum_settings() {
        use crate::settings::{DekuTreeState, GanonBossKeyMode, RandomizerSettings};

        let ram = MockRam::new();
        let mut rs = RandomizerSettings::new();
        rs.deku_tree = DekuTreeState::Open;
        rs.ganon_boss_key = GanonBossKeyMode::Removed;

        let ctx = OotEvalContextBuilder::new(&ram)
            .with_randomizer_settings(&rs)
            .build();

        assert!(ctx.setting_value("dekuTree", "open"));
        assert!(!ctx.setting_value("dekuTree", "closed"));
        assert!(ctx.setting_value("ganonBossKey", "removed"));
        assert!(!ctx.setting_value("ganonBossKey", "vanilla"));
    }

    #[test]
    fn test_trick_with_randomizer_settings() {
        use crate::settings::RandomizerSettings;

        let ram = MockRam::new();
        let mut rs = RandomizerSettings::new();
        rs.enable_trick("OOT_LENS_WASTELAND");
        rs.enable_trick("OOT_HOVER_BOOST");

        let ctx = OotEvalContextBuilder::new(&ram)
            .with_randomizer_settings(&rs)
            .build();

        assert!(ctx.trick("OOT_LENS_WASTELAND"));
        assert!(ctx.trick("OOT_HOVER_BOOST"));
        assert!(!ctx.trick("OOT_SOME_OTHER_TRICK"));
    }

    #[test]
    fn test_trick_combined_sources() {
        use crate::settings::RandomizerSettings;

        let ram = MockRam::new();
        let mut rs = RandomizerSettings::new();
        rs.enable_trick("from_settings");

        let ctx = OotEvalContextBuilder::new(&ram)
            .with_trick("from_local")
            .with_randomizer_settings(&rs)
            .build();

        // Should find tricks from both sources
        assert!(ctx.trick("from_local"));
        assert!(ctx.trick("from_settings"));
        assert!(!ctx.trick("not_enabled"));
    }

    #[test]
    fn test_setting_fallback_to_legacy() {
        use crate::settings::RandomizerSettings;

        let ram = MockRam::new();
        let rs = RandomizerSettings::new();

        let ctx = OotEvalContextBuilder::new(&ram)
            .with_setting("legacySetting", true)
            .with_randomizer_settings(&rs)
            .build();

        // Legacy setting should work when not in RandomizerSettings
        assert_eq!(ctx.setting("legacySetting"), Some(true));
        // RandomizerSettings booleans should also work
        assert_eq!(ctx.setting("agelessBoots"), Some(false));
    }

    #[test]
    fn test_builder_with_randomizer_settings() {
        use crate::settings::RandomizerSettings;

        let ram = MockRam::new();
        let rs = RandomizerSettings::new();

        let ctx = OotEvalContextBuilder::new(&ram)
            .with_randomizer_settings(&rs)
            .build();

        assert!(ctx.randomizer_settings().is_some());
    }
}
