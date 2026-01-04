//! MM-specific evaluation context for reading game state from N64 RAM.
//!
//! This module provides [`MmEvalContext`] which implements the [`EvalContext`] trait
//! by reading item possession and game state directly from N64 RAM addresses.
//!
//! # MM Save Data Layout
//!
//! The MM save data starts at base address `0x1EF670` in RDRAM. Items, equipment,
//! masks, songs, and upgrades are stored at various offsets from this base.
//!
//! # Key Differences from OoT
//!
//! - Different base address (0x1EF670 vs 0x11A5D0)
//! - Masks instead of trade items (24 collectible masks)
//! - Transformation masks (Deku, Goron, Zora, Fierce Deity)
//! - Different song set (Song of Healing, Song of Soaring, Elegy, etc.)
//! - Boss remains instead of medallions
//! - Three-day time cycle
//!
//! # Example
//!
//! ```ignore
//! use ootmm::expr::{MmEvalContext, MmRamReader, Evaluator};
//!
//! // Create a RAM reader (implementation depends on your emulator interface)
//! let ram_reader = MyRamReader::new();
//!
//! // Create the evaluation context
//! let ctx = MmEvalContext::new(&ram_reader);
//!
//! // Evaluate logic expressions against the current game state
//! let evaluator = Evaluator::new(&ctx);
//! let can_enter = evaluator.eval_str("has(MASK_DEKU) && has(OCARINA)")?;
//! ```

use crate::expr::EvalContext;
use std::collections::{HashMap, HashSet};

/// Base address of MM save data in N64 RDRAM.
pub const MM_SAVE_BASE: u32 = 0x1EF670;

/// Trait for reading bytes from N64 RAM.
///
/// Implement this trait to provide access to emulator memory.
pub trait MmRamReader {
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

/// RAM offsets relative to save base for MM inventory items.
/// Based on MM save data structure.
#[allow(dead_code)]
mod offsets {
    // Inventory slots (0x00-0x17)
    pub const OCARINA: u16 = 0x00;
    pub const BOW: u16 = 0x01;
    pub const FIRE_ARROWS: u16 = 0x02;
    pub const ICE_ARROWS: u16 = 0x03;
    pub const LIGHT_ARROWS: u16 = 0x04;
    pub const QUEST_ITEM_1: u16 = 0x05; // Trade sequence slot 1
    pub const BOMBS: u16 = 0x06;
    pub const BOMBCHU: u16 = 0x07;
    pub const DEKU_STICKS: u16 = 0x08;
    pub const DEKU_NUTS: u16 = 0x09;
    pub const MAGIC_BEANS: u16 = 0x0A;
    pub const QUEST_ITEM_2: u16 = 0x0B; // Trade sequence slot 2
    pub const POWDER_KEG: u16 = 0x0C;
    pub const PICTOGRAPH_BOX: u16 = 0x0D;
    pub const LENS_OF_TRUTH: u16 = 0x0E;
    pub const HOOKSHOT: u16 = 0x0F;
    pub const GREAT_FAIRY_SWORD: u16 = 0x10;
    pub const QUEST_ITEM_3: u16 = 0x11; // Trade sequence slot 3
    pub const BOTTLE_1: u16 = 0x12;
    pub const BOTTLE_2: u16 = 0x13;
    pub const BOTTLE_3: u16 = 0x14;
    pub const BOTTLE_4: u16 = 0x15;
    pub const BOTTLE_5: u16 = 0x16;
    pub const BOTTLE_6: u16 = 0x17;

    // Mask inventory (0x18-0x2F) - 24 mask slots
    pub const MASKS_START: u16 = 0x18;
    pub const MASK_POSTMAN: u16 = 0x18;
    pub const MASK_ALL_NIGHT: u16 = 0x19;
    pub const MASK_BLAST: u16 = 0x1A;
    pub const MASK_STONE: u16 = 0x1B;
    pub const MASK_GREAT_FAIRY: u16 = 0x1C;
    pub const MASK_DEKU: u16 = 0x1D;
    pub const MASK_KEATON: u16 = 0x1E;
    pub const MASK_BREMEN: u16 = 0x1F;
    pub const MASK_BUNNY: u16 = 0x20;
    pub const MASK_DON_GERO: u16 = 0x21;
    pub const MASK_SCENTS: u16 = 0x22;
    pub const MASK_GORON: u16 = 0x23;
    pub const MASK_ROMANI: u16 = 0x24;
    pub const MASK_CIRCUS_LEADER: u16 = 0x25;
    pub const MASK_KAFEI: u16 = 0x26;
    pub const MASK_COUPLE: u16 = 0x27;
    pub const MASK_TRUTH: u16 = 0x28;
    pub const MASK_ZORA: u16 = 0x29;
    pub const MASK_KAMARO: u16 = 0x2A;
    pub const MASK_GIBDO: u16 = 0x2B;
    pub const MASK_GARO: u16 = 0x2C;
    pub const MASK_CAPTAIN: u16 = 0x2D;
    pub const MASK_GIANT: u16 = 0x2E;
    pub const MASK_FIERCE_DEITY: u16 = 0x2F;

    // Ammo counts
    pub const AMMO_BOW: u16 = 0x70;
    pub const AMMO_BOMBS: u16 = 0x72;
    pub const AMMO_BOMBCHU: u16 = 0x73;
    pub const AMMO_STICKS: u16 = 0x74;
    pub const AMMO_NUTS: u16 = 0x75;
    pub const AMMO_BEANS: u16 = 0x76;
    pub const AMMO_KEG: u16 = 0x78;

    // Equipment flags
    pub const EQUIPMENT: u16 = 0x7C; // 2 bytes for swords/shields

    // Upgrades (quiver, bomb bag, wallet, etc.)
    pub const UPGRADES: u16 = 0x80; // 4 bytes

    // Quest status (songs, remains, owl statues)
    pub const QUEST_STATUS: u16 = 0x84; // 4 bytes

    // Quest items (key items like deeds, letters, etc.)
    pub const QUEST_ITEMS: u16 = 0x88; // 4 bytes

    // Dungeon items (maps, compasses, boss keys)
    pub const DUNGEON_ITEMS: u16 = 0x8C;

    // Small key counts per dungeon
    pub const SMALL_KEYS: u16 = 0xA0;

    // Rupees
    pub const RUPEES: u16 = 0x34;

    // Magic meter
    pub const MAGIC_SIZE: u16 = 0x3A; // 0 = none, 1 = normal, 2 = double
    pub const MAGIC_AMOUNT: u16 = 0x3B;

    // Heart containers
    pub const HEARTS: u16 = 0x2E; // Max hearts (in 16ths)

    // Current time (in-game clock)
    pub const TIME: u16 = 0x0C; // Current time of day
    pub const DAY: u16 = 0x10; // Current day (1-3)

    // Current form (transformation mask equipped)
    pub const CURRENT_FORM: u16 = 0x20; // 0=Link, 1=Deku, 2=Zora, 3=Goron, 4=Fierce Deity

    // Spin attack upgrade
    pub const SPIN_UPGRADE: u16 = 0x38; // Great Spin Attack learned
}

/// Item IDs for inventory slots.
#[allow(dead_code)]
mod item_ids {
    // Ocarina
    pub const OCARINA_FAIRY: u8 = 0x00;

    // Bow
    pub const BOW: u8 = 0x01;

    // Arrows
    pub const FIRE_ARROWS: u8 = 0x02;
    pub const ICE_ARROWS: u8 = 0x03;
    pub const LIGHT_ARROWS: u8 = 0x04;

    // Bombs
    pub const BOMBS: u8 = 0x06;
    pub const BOMBCHU: u8 = 0x07;

    // Basic items
    pub const DEKU_STICKS: u8 = 0x08;
    pub const DEKU_NUTS: u8 = 0x09;
    pub const MAGIC_BEANS: u8 = 0x0A;

    // Special items
    pub const POWDER_KEG: u8 = 0x0C;
    pub const PICTOGRAPH_BOX: u8 = 0x0D;
    pub const LENS_OF_TRUTH: u8 = 0x0E;
    pub const HOOKSHOT: u8 = 0x0F;
    pub const GREAT_FAIRY_SWORD: u8 = 0x10;

    // Bottles (0x12-0x2A range for bottle contents)
    pub const BOTTLE_EMPTY: u8 = 0x12;

    // Mask item IDs (when in mask inventory)
    pub const MASK_POSTMAN: u8 = 0x3E;
    pub const MASK_ALL_NIGHT: u8 = 0x3F;
    pub const MASK_BLAST: u8 = 0x40;
    pub const MASK_STONE: u8 = 0x41;
    pub const MASK_GREAT_FAIRY: u8 = 0x42;
    pub const MASK_DEKU: u8 = 0x43;
    pub const MASK_KEATON: u8 = 0x44;
    pub const MASK_BREMEN: u8 = 0x45;
    pub const MASK_BUNNY: u8 = 0x46;
    pub const MASK_DON_GERO: u8 = 0x47;
    pub const MASK_SCENTS: u8 = 0x48;
    pub const MASK_GORON: u8 = 0x49;
    pub const MASK_ROMANI: u8 = 0x4A;
    pub const MASK_CIRCUS_LEADER: u8 = 0x4B;
    pub const MASK_KAFEI: u8 = 0x4C;
    pub const MASK_COUPLE: u8 = 0x4D;
    pub const MASK_TRUTH: u8 = 0x4E;
    pub const MASK_ZORA: u8 = 0x4F;
    pub const MASK_KAMARO: u8 = 0x50;
    pub const MASK_GIBDO: u8 = 0x51;
    pub const MASK_GARO: u8 = 0x52;
    pub const MASK_CAPTAIN: u8 = 0x53;
    pub const MASK_GIANT: u8 = 0x54;
    pub const MASK_FIERCE_DEITY: u8 = 0x55;

    // Quest items (trade sequence / key items)
    pub const MOON_TEAR: u8 = 0x28;
    pub const DEED_LAND: u8 = 0x29;
    pub const DEED_SWAMP: u8 = 0x2A;
    pub const DEED_MOUNTAIN: u8 = 0x2B;
    pub const DEED_OCEAN: u8 = 0x2C;
    pub const ROOM_KEY: u8 = 0x2D;
    pub const LETTER_TO_MAMA: u8 = 0x2E;
    pub const LETTER_TO_KAFEI: u8 = 0x2F;
    pub const PENDANT_OF_MEMORIES: u8 = 0x30;
}

/// Bit positions in quest status flags.
#[allow(dead_code)]
mod quest_bits {
    // Songs (bits in quest status)
    pub const SONG_SONATA: u8 = 0; // Sonata of Awakening
    pub const SONG_LULLABY: u8 = 1; // Goron Lullaby
    pub const SONG_BOSSA_NOVA: u8 = 2; // New Wave Bossa Nova
    pub const SONG_ELEGY: u8 = 3; // Elegy of Emptiness
    pub const SONG_OATH: u8 = 4; // Oath to Order
    pub const SONG_HEALING: u8 = 5; // Song of Healing
    pub const SONG_SOARING: u8 = 6; // Song of Soaring
    pub const SONG_EPONA: u8 = 7; // Epona's Song
    pub const SONG_SUN: u8 = 8; // Sun's Song (if applicable)
    pub const SONG_STORMS: u8 = 9; // Song of Storms
    pub const SONG_TIME: u8 = 10; // Song of Time
    pub const SONG_SCARECROW: u8 = 11; // Scarecrow's Song

    // Boss remains
    pub const REMAINS_ODOLWA: u8 = 16;
    pub const REMAINS_GOHT: u8 = 17;
    pub const REMAINS_GYORG: u8 = 18;
    pub const REMAINS_TWINMOLD: u8 = 19;

    // Owl statues activated (bits)
    pub const OWL_CLOCK_TOWN: u8 = 20;
    pub const OWL_MILK_ROAD: u8 = 21;
    pub const OWL_SOUTHERN_SWAMP: u8 = 22;
    pub const OWL_WOODFALL: u8 = 23;
    pub const OWL_MOUNTAIN_VILLAGE: u8 = 24;
    pub const OWL_SNOWHEAD: u8 = 25;
    pub const OWL_ZORA_CAPE: u8 = 26;
    pub const OWL_GREAT_BAY: u8 = 27;
    pub const OWL_IKANA_CANYON: u8 = 28;
    pub const OWL_STONE_TOWER: u8 = 29;
}

/// Equipment bit positions.
mod equipment_bits {
    // Swords (bits 0-2)
    pub const KOKIRI_SWORD: u8 = 0;
    pub const RAZOR_SWORD: u8 = 1;
    pub const GILDED_SWORD: u8 = 2;

    // Shields (bits 4-5)
    pub const HERO_SHIELD: u8 = 4;
    pub const MIRROR_SHIELD: u8 = 5;
}

/// Upgrade field positions (in upgrades u32).
#[allow(dead_code)]
mod upgrade_bits {
    pub const QUIVER_SHIFT: u8 = 0;
    pub const BOMB_BAG_SHIFT: u8 = 3;
    pub const WALLET_SHIFT: u8 = 6;
    pub const STICK_CAPACITY_SHIFT: u8 = 9;
    pub const NUT_CAPACITY_SHIFT: u8 = 12;
}

/// Quest item bit positions.
#[allow(dead_code)]
mod quest_item_bits {
    pub const BOMBERS_NOTEBOOK: u8 = 0;
    pub const SPIN_UPGRADE: u8 = 1; // Great Spin Attack
}

/// MM evaluation context that reads game state from N64 RAM.
///
/// This context maps item names used in logic expressions to their
/// corresponding RAM addresses and provides the [`EvalContext`] interface
/// for expression evaluation.
pub struct MmEvalContext<'a, R: MmRamReader> {
    /// RAM reader for accessing emulator memory.
    reader: &'a R,
    /// Base address of save data.
    save_base: u32,
    /// Enabled tricks (configured by user).
    tricks: HashSet<String>,
    /// Settings (configured by user).
    settings: HashMap<String, bool>,
    /// Events (tracked separately, not in RAM).
    events: HashSet<String>,
}

impl<'a, R: MmRamReader> MmEvalContext<'a, R> {
    /// Creates a new MM evaluation context.
    ///
    /// # Arguments
    /// * `reader` - RAM reader for accessing emulator memory
    ///
    /// Uses default save base address (`0x1EF670`).
    pub fn new(reader: &'a R) -> Self {
        Self {
            reader,
            save_base: MM_SAVE_BASE,
            tricks: HashSet::new(),
            settings: HashMap::new(),
            events: HashSet::new(),
        }
    }

    /// Creates a new MM evaluation context with custom save base address.
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
        }
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

    /// Check if an inventory slot contains a specific item or any item.
    fn has_inventory_item(&self, offset: u16, expected: Option<u8>) -> bool {
        let slot = self.read_save_u8(offset);
        match expected {
            Some(id) => slot == id,
            None => slot != 0xFF, // 0xFF = empty slot
        }
    }

    /// Check if a mask slot contains the expected mask.
    fn has_mask(&self, offset: u16, expected_id: u8) -> bool {
        let slot = self.read_save_u8(offset);
        slot == expected_id
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

    /// Check if a bit is set in the quest item flags.
    fn has_quest_item_bit(&self, bit: u8) -> bool {
        let items = self.read_save_u32(offsets::QUEST_ITEMS);
        (items & (1 << bit)) != 0
    }

    /// Get an upgrade level (quiver, bomb bag, etc.).
    fn get_upgrade_level(&self, shift: u8, mask: u8) -> u8 {
        let upgrades = self.read_save_u32(offsets::UPGRADES);
        ((upgrades >> shift) & mask as u32) as u8
    }

    /// Get the number of bottles.
    fn get_bottle_count(&self) -> u32 {
        let mut count = 0;
        for offset in [
            offsets::BOTTLE_1,
            offsets::BOTTLE_2,
            offsets::BOTTLE_3,
            offsets::BOTTLE_4,
            offsets::BOTTLE_5,
            offsets::BOTTLE_6,
        ] {
            let slot = self.read_save_u8(offset);
            // Any bottle content (0x12+ typically)
            if slot != 0xFF {
                count += 1;
            }
        }
        count
    }

    /// Get the current MM time as minutes since Day 1 6:00 AM.
    fn get_mm_time(&self) -> u32 {
        let day = self.read_save_u8(offsets::DAY) as u32;
        let time = self.read_save_u16(offsets::TIME) as u32;

        // Time is stored as a 16-bit value representing time of day
        // 0x0000 = 6:00 AM, wraps at midnight
        // Each day is 1440 minutes in our representation
        let minutes_in_day = (time as u64 * 1440 / 0x10000) as u32;

        // Calculate total minutes: (day - 1) * 1440 + minutes
        let day_offset = day.saturating_sub(1) * 1440;
        day_offset + minutes_in_day
    }

    /// Check if a quest item slot contains a specific item.
    fn has_quest_item_in_slot(&self, slot_offset: u16, item_id: u8) -> bool {
        self.read_save_u8(slot_offset) == item_id
    }

    /// Map an item name to a RAM check.
    ///
    /// Returns `Some((has_item, count))` where has_item indicates possession
    /// and count is the quantity (1 for boolean items).
    fn get_item_check(&self, item: &str) -> Option<(bool, u32)> {
        let item_upper = item.to_uppercase();
        let item_str = item_upper.as_str();

        let result = match item_str {
            // Transformation masks (most important for MM)
            "MASK_DEKU" => self.has_mask(offsets::MASK_DEKU, item_ids::MASK_DEKU),
            "MASK_GORON" => self.has_mask(offsets::MASK_GORON, item_ids::MASK_GORON),
            "MASK_ZORA" => self.has_mask(offsets::MASK_ZORA, item_ids::MASK_ZORA),
            "MASK_FIERCE_DEITY" => {
                self.has_mask(offsets::MASK_FIERCE_DEITY, item_ids::MASK_FIERCE_DEITY)
            }

            // Other masks
            "MASK_POSTMAN" => self.has_mask(offsets::MASK_POSTMAN, item_ids::MASK_POSTMAN),
            "MASK_ALL_NIGHT" => self.has_mask(offsets::MASK_ALL_NIGHT, item_ids::MASK_ALL_NIGHT),
            "MASK_BLAST" => self.has_mask(offsets::MASK_BLAST, item_ids::MASK_BLAST),
            "MASK_STONE" => self.has_mask(offsets::MASK_STONE, item_ids::MASK_STONE),
            "MASK_GREAT_FAIRY" => {
                self.has_mask(offsets::MASK_GREAT_FAIRY, item_ids::MASK_GREAT_FAIRY)
            }
            "MASK_KEATON" => self.has_mask(offsets::MASK_KEATON, item_ids::MASK_KEATON),
            "MASK_BREMEN" => self.has_mask(offsets::MASK_BREMEN, item_ids::MASK_BREMEN),
            "MASK_BUNNY" => self.has_mask(offsets::MASK_BUNNY, item_ids::MASK_BUNNY),
            "MASK_DON_GERO" => self.has_mask(offsets::MASK_DON_GERO, item_ids::MASK_DON_GERO),
            "MASK_SCENTS" => self.has_mask(offsets::MASK_SCENTS, item_ids::MASK_SCENTS),
            "MASK_ROMANI" => self.has_mask(offsets::MASK_ROMANI, item_ids::MASK_ROMANI),
            "MASK_CIRCUS_LEADER" | "MASK_TROUPE_LEADER" => {
                self.has_mask(offsets::MASK_CIRCUS_LEADER, item_ids::MASK_CIRCUS_LEADER)
            }
            "MASK_KAFEI" => self.has_mask(offsets::MASK_KAFEI, item_ids::MASK_KAFEI),
            "MASK_COUPLE" | "MASK_COUPLES" => {
                self.has_mask(offsets::MASK_COUPLE, item_ids::MASK_COUPLE)
            }
            "MASK_TRUTH" | "MASK_OF_TRUTH" => {
                self.has_mask(offsets::MASK_TRUTH, item_ids::MASK_TRUTH)
            }
            "MASK_KAMARO" => self.has_mask(offsets::MASK_KAMARO, item_ids::MASK_KAMARO),
            "MASK_GIBDO" => self.has_mask(offsets::MASK_GIBDO, item_ids::MASK_GIBDO),
            "MASK_GARO" | "MASK_GARO_MASTER" => {
                self.has_mask(offsets::MASK_GARO, item_ids::MASK_GARO)
            }
            "MASK_CAPTAIN" | "MASK_CAPTAINS_HAT" => {
                self.has_mask(offsets::MASK_CAPTAIN, item_ids::MASK_CAPTAIN)
            }
            "MASK_GIANT" | "MASK_GIANTS" => {
                self.has_mask(offsets::MASK_GIANT, item_ids::MASK_GIANT)
            }

            // Inventory items
            "OCARINA" | "OCARINA_FAIRY" => self.has_inventory_item(offsets::OCARINA, None),
            "BOW" => self.has_inventory_item(offsets::BOW, None),
            "FIRE_ARROW" | "FIRE_ARROWS" => self.has_inventory_item(offsets::FIRE_ARROWS, None),
            "ICE_ARROW" | "ICE_ARROWS" => self.has_inventory_item(offsets::ICE_ARROWS, None),
            "LIGHT_ARROW" | "LIGHT_ARROWS" => self.has_inventory_item(offsets::LIGHT_ARROWS, None),
            "BOMB_BAG" | "BOMBS" => {
                let level = self.get_upgrade_level(upgrade_bits::BOMB_BAG_SHIFT, 0x07);
                level >= 1
            }
            "BOMBCHU" => self.has_inventory_item(offsets::BOMBCHU, None),
            "DEKU_STICK" | "DEKU_STICKS" | "STICKS" => {
                self.has_inventory_item(offsets::DEKU_STICKS, None)
            }
            "DEKU_NUT" | "DEKU_NUTS" | "NUTS" => self.has_inventory_item(offsets::DEKU_NUTS, None),
            "MAGIC_BEAN" | "MAGIC_BEANS" | "BEANS" => {
                self.has_inventory_item(offsets::MAGIC_BEANS, None)
            }
            "POWDER_KEG" => self.has_inventory_item(offsets::POWDER_KEG, None),
            "PICTOGRAPH_BOX" | "PICTOBOX" => self.has_inventory_item(offsets::PICTOGRAPH_BOX, None),
            "LENS_OF_TRUTH" | "LENS" => self.has_inventory_item(offsets::LENS_OF_TRUTH, None),
            "HOOKSHOT" => self.has_inventory_item(offsets::HOOKSHOT, None),
            "GREAT_FAIRY_SWORD" | "GFS" => {
                self.has_inventory_item(offsets::GREAT_FAIRY_SWORD, None)
            }

            // Bottles
            "BOTTLE" => return Some((self.get_bottle_count() >= 1, 1)),

            // Equipment (swords, shields)
            "KOKIRI_SWORD" | "SWORD_KOKIRI" => self.has_equipment_bit(equipment_bits::KOKIRI_SWORD),
            "RAZOR_SWORD" | "SWORD_RAZOR" => self.has_equipment_bit(equipment_bits::RAZOR_SWORD),
            "GILDED_SWORD" | "SWORD_GILDED" => self.has_equipment_bit(equipment_bits::GILDED_SWORD),
            "HERO_SHIELD" | "SHIELD_HERO" => self.has_equipment_bit(equipment_bits::HERO_SHIELD),
            "MIRROR_SHIELD" | "SHIELD_MIRROR" => {
                self.has_equipment_bit(equipment_bits::MIRROR_SHIELD)
            }

            // Upgrades
            "WALLET" | "ADULT_WALLET" => {
                let level = self.get_upgrade_level(upgrade_bits::WALLET_SHIFT, 0x03);
                level >= 1
            }
            "GIANT_WALLET" => {
                let level = self.get_upgrade_level(upgrade_bits::WALLET_SHIFT, 0x03);
                level >= 2
            }
            "QUIVER" => {
                let level = self.get_upgrade_level(upgrade_bits::QUIVER_SHIFT, 0x07);
                level >= 1
            }
            "MAGIC" | "MAGIC_METER" | "MAGIC_UPGRADE" | "SHARED_MAGIC_UPGRADE" => {
                self.read_save_u8(offsets::MAGIC_SIZE) >= 1
            }
            "DOUBLE_MAGIC" => self.read_save_u8(offsets::MAGIC_SIZE) >= 2,

            // Spin upgrade (Great Spin Attack)
            "SPIN_UPGRADE" | "GREAT_SPIN" => self.has_quest_item_bit(quest_item_bits::SPIN_UPGRADE),

            // Boss remains
            "REMAINS_ODOLWA" | "ODOLWA_REMAINS" => self.has_quest_bit(quest_bits::REMAINS_ODOLWA),
            "REMAINS_GOHT" | "GOHT_REMAINS" => self.has_quest_bit(quest_bits::REMAINS_GOHT),
            "REMAINS_GYORG" | "GYORG_REMAINS" => self.has_quest_bit(quest_bits::REMAINS_GYORG),
            "REMAINS_TWINMOLD" | "TWINMOLD_REMAINS" => {
                self.has_quest_bit(quest_bits::REMAINS_TWINMOLD)
            }

            // Songs
            "SONG_HEALING" | "SONG_OF_HEALING" => self.has_quest_bit(quest_bits::SONG_HEALING),
            "SONG_SOARING" | "SONG_OF_SOARING" => self.has_quest_bit(quest_bits::SONG_SOARING),
            "SONG_SONATA" | "SONATA" | "SONATA_OF_AWAKENING" => {
                self.has_quest_bit(quest_bits::SONG_SONATA)
            }
            "SONG_LULLABY" | "GORON_LULLABY" => self.has_quest_bit(quest_bits::SONG_LULLABY),
            "SONG_BOSSA_NOVA" | "BOSSA_NOVA" | "NEW_WAVE_BOSSA_NOVA" => {
                self.has_quest_bit(quest_bits::SONG_BOSSA_NOVA)
            }
            "SONG_ELEGY" | "ELEGY" | "ELEGY_OF_EMPTINESS" => {
                self.has_quest_bit(quest_bits::SONG_ELEGY)
            }
            "SONG_OATH" | "OATH" | "OATH_TO_ORDER" => self.has_quest_bit(quest_bits::SONG_OATH),
            "EPONAS_SONG" | "EPONA_SONG" | "EPONA" => self.has_quest_bit(quest_bits::SONG_EPONA),
            "SONG_OF_TIME" | "TIME_SONG" => self.has_quest_bit(quest_bits::SONG_TIME),
            "SONG_OF_STORMS" | "STORMS_SONG" | "STORMS" => {
                self.has_quest_bit(quest_bits::SONG_STORMS)
            }
            "SCARECROW_SONG" | "SONG_SCARECROW" => self.has_quest_bit(quest_bits::SONG_SCARECROW),

            // Owl statues
            "OWL_CLOCK_TOWN" => self.has_quest_bit(quest_bits::OWL_CLOCK_TOWN),
            "OWL_MILK_ROAD" => self.has_quest_bit(quest_bits::OWL_MILK_ROAD),
            "OWL_SOUTHERN_SWAMP" | "OWL_SWAMP" => {
                self.has_quest_bit(quest_bits::OWL_SOUTHERN_SWAMP)
            }
            "OWL_WOODFALL" => self.has_quest_bit(quest_bits::OWL_WOODFALL),
            "OWL_MOUNTAIN_VILLAGE" => self.has_quest_bit(quest_bits::OWL_MOUNTAIN_VILLAGE),
            "OWL_SNOWHEAD" => self.has_quest_bit(quest_bits::OWL_SNOWHEAD),
            "OWL_ZORA_CAPE" | "OWL_ZORA" => self.has_quest_bit(quest_bits::OWL_ZORA_CAPE),
            "OWL_GREAT_BAY" => self.has_quest_bit(quest_bits::OWL_GREAT_BAY),
            "OWL_IKANA_CANYON" | "OWL_IKANA" => self.has_quest_bit(quest_bits::OWL_IKANA_CANYON),
            "OWL_STONE_TOWER" => self.has_quest_bit(quest_bits::OWL_STONE_TOWER),

            // Quest items / Trade sequence
            "MOON_TEAR" => {
                self.has_quest_item_in_slot(offsets::QUEST_ITEM_1, item_ids::MOON_TEAR)
                    || self.has_quest_item_in_slot(offsets::QUEST_ITEM_2, item_ids::MOON_TEAR)
                    || self.has_quest_item_in_slot(offsets::QUEST_ITEM_3, item_ids::MOON_TEAR)
            }
            "DEED_LAND" | "LAND_TITLE_DEED" => {
                self.has_quest_item_in_slot(offsets::QUEST_ITEM_1, item_ids::DEED_LAND)
                    || self.has_quest_item_in_slot(offsets::QUEST_ITEM_2, item_ids::DEED_LAND)
                    || self.has_quest_item_in_slot(offsets::QUEST_ITEM_3, item_ids::DEED_LAND)
            }
            "DEED_SWAMP" | "SWAMP_TITLE_DEED" => {
                self.has_quest_item_in_slot(offsets::QUEST_ITEM_1, item_ids::DEED_SWAMP)
                    || self.has_quest_item_in_slot(offsets::QUEST_ITEM_2, item_ids::DEED_SWAMP)
                    || self.has_quest_item_in_slot(offsets::QUEST_ITEM_3, item_ids::DEED_SWAMP)
            }
            "DEED_MOUNTAIN" | "MOUNTAIN_TITLE_DEED" => {
                self.has_quest_item_in_slot(offsets::QUEST_ITEM_1, item_ids::DEED_MOUNTAIN)
                    || self.has_quest_item_in_slot(offsets::QUEST_ITEM_2, item_ids::DEED_MOUNTAIN)
                    || self.has_quest_item_in_slot(offsets::QUEST_ITEM_3, item_ids::DEED_MOUNTAIN)
            }
            "DEED_OCEAN" | "OCEAN_TITLE_DEED" => {
                self.has_quest_item_in_slot(offsets::QUEST_ITEM_1, item_ids::DEED_OCEAN)
                    || self.has_quest_item_in_slot(offsets::QUEST_ITEM_2, item_ids::DEED_OCEAN)
                    || self.has_quest_item_in_slot(offsets::QUEST_ITEM_3, item_ids::DEED_OCEAN)
            }
            "ROOM_KEY" => {
                self.has_quest_item_in_slot(offsets::QUEST_ITEM_1, item_ids::ROOM_KEY)
                    || self.has_quest_item_in_slot(offsets::QUEST_ITEM_2, item_ids::ROOM_KEY)
                    || self.has_quest_item_in_slot(offsets::QUEST_ITEM_3, item_ids::ROOM_KEY)
            }
            "LETTER_TO_MAMA" | "SPECIAL_DELIVERY" => {
                self.has_quest_item_in_slot(offsets::QUEST_ITEM_1, item_ids::LETTER_TO_MAMA)
                    || self.has_quest_item_in_slot(offsets::QUEST_ITEM_2, item_ids::LETTER_TO_MAMA)
                    || self.has_quest_item_in_slot(offsets::QUEST_ITEM_3, item_ids::LETTER_TO_MAMA)
            }
            "LETTER_TO_KAFEI" | "PRIORITY_MAIL" => {
                self.has_quest_item_in_slot(offsets::QUEST_ITEM_1, item_ids::LETTER_TO_KAFEI)
                    || self.has_quest_item_in_slot(offsets::QUEST_ITEM_2, item_ids::LETTER_TO_KAFEI)
                    || self.has_quest_item_in_slot(offsets::QUEST_ITEM_3, item_ids::LETTER_TO_KAFEI)
            }
            "PENDANT_OF_MEMORIES" | "PENDANT" => {
                self.has_quest_item_in_slot(offsets::QUEST_ITEM_1, item_ids::PENDANT_OF_MEMORIES)
                    || self.has_quest_item_in_slot(
                        offsets::QUEST_ITEM_2,
                        item_ids::PENDANT_OF_MEMORIES,
                    )
                    || self.has_quest_item_in_slot(
                        offsets::QUEST_ITEM_3,
                        item_ids::PENDANT_OF_MEMORIES,
                    )
            }

            _ => return None,
        };

        Some((result, 1))
    }
}

impl<R: MmRamReader> EvalContext for MmEvalContext<'_, R> {
    fn has_item(&self, item: &str, count: u32) -> bool {
        match self.get_item_check(item) {
            Some((has, item_count)) => {
                if item.to_uppercase().contains("BOTTLE") {
                    // For bottles, compare the count
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
        self.settings.get(name).copied()
    }

    fn trick(&self, name: &str) -> bool {
        self.tricks.contains(name)
    }

    fn is_adult(&self) -> bool {
        // In MM, Link is always "adult" age (no child form)
        // Transformations are handled via masks
        true
    }

    fn is_child(&self) -> bool {
        // In MM, Link is never "child" in the OoT sense
        false
    }

    fn mm_time(&self) -> u32 {
        self.get_mm_time()
    }
}

/// Builder for [`MmEvalContext`].
///
/// Provides a fluent API for constructing MM evaluation contexts with
/// pre-configured tricks, settings, and events.
pub struct MmEvalContextBuilder<'a, R: MmRamReader> {
    ctx: MmEvalContext<'a, R>,
}

impl<'a, R: MmRamReader> MmEvalContextBuilder<'a, R> {
    /// Creates a new builder with the given RAM reader.
    pub fn new(reader: &'a R) -> Self {
        Self {
            ctx: MmEvalContext::new(reader),
        }
    }

    /// Sets a custom save base address.
    #[must_use]
    pub fn with_save_base(mut self, base: u32) -> Self {
        self.ctx.save_base = base;
        self
    }

    /// Adds a trick.
    #[must_use]
    pub fn with_trick(mut self, trick: &str) -> Self {
        self.ctx.add_trick(trick);
        self
    }

    /// Sets a setting.
    #[must_use]
    pub fn with_setting(mut self, name: &str, value: bool) -> Self {
        self.ctx.set_setting(name, value);
        self
    }

    /// Adds an event.
    #[must_use]
    pub fn with_event(mut self, event: &str) -> Self {
        self.ctx.add_event(event);
        self
    }

    /// Builds the [`MmEvalContext`].
    #[must_use]
    pub fn build(self) -> MmEvalContext<'a, R> {
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

    impl MmRamReader for MockRam {
        fn read_u8(&self, addr: u32) -> u8 {
            self.data.get(&addr).copied().unwrap_or(0xFF)
        }
    }

    const BASE: u32 = MM_SAVE_BASE;

    // --- Basic tests ---

    #[test]
    fn test_new_context() {
        let ram = MockRam::new();
        let ctx = MmEvalContext::new(&ram);
        assert_eq!(ctx.save_base, MM_SAVE_BASE);
    }

    #[test]
    fn test_custom_save_base() {
        let ram = MockRam::new();
        let ctx = MmEvalContext::with_save_base(&ram, 0x200000);
        assert_eq!(ctx.save_base, 0x200000);
    }

    // --- Transformation mask tests ---

    #[test]
    fn test_has_mask_deku() {
        let mut ram = MockRam::new();
        ram.set(BASE + offsets::MASK_DEKU as u32, item_ids::MASK_DEKU);

        let ctx = MmEvalContext::new(&ram);
        assert!(ctx.has_item("MASK_DEKU", 1));
    }

    #[test]
    fn test_has_mask_goron() {
        let mut ram = MockRam::new();
        ram.set(BASE + offsets::MASK_GORON as u32, item_ids::MASK_GORON);

        let ctx = MmEvalContext::new(&ram);
        assert!(ctx.has_item("MASK_GORON", 1));
    }

    #[test]
    fn test_has_mask_zora() {
        let mut ram = MockRam::new();
        ram.set(BASE + offsets::MASK_ZORA as u32, item_ids::MASK_ZORA);

        let ctx = MmEvalContext::new(&ram);
        assert!(ctx.has_item("MASK_ZORA", 1));
    }

    #[test]
    fn test_has_mask_fierce_deity() {
        let mut ram = MockRam::new();
        ram.set(
            BASE + offsets::MASK_FIERCE_DEITY as u32,
            item_ids::MASK_FIERCE_DEITY,
        );

        let ctx = MmEvalContext::new(&ram);
        assert!(ctx.has_item("MASK_FIERCE_DEITY", 1));
    }

    // --- Other mask tests ---

    #[test]
    fn test_has_mask_captain() {
        let mut ram = MockRam::new();
        ram.set(BASE + offsets::MASK_CAPTAIN as u32, item_ids::MASK_CAPTAIN);

        let ctx = MmEvalContext::new(&ram);
        assert!(ctx.has_item("MASK_CAPTAIN", 1));
    }

    #[test]
    fn test_has_mask_great_fairy() {
        let mut ram = MockRam::new();
        ram.set(
            BASE + offsets::MASK_GREAT_FAIRY as u32,
            item_ids::MASK_GREAT_FAIRY,
        );

        let ctx = MmEvalContext::new(&ram);
        assert!(ctx.has_item("MASK_GREAT_FAIRY", 1));
    }

    #[test]
    fn test_has_mask_giant() {
        let mut ram = MockRam::new();
        ram.set(BASE + offsets::MASK_GIANT as u32, item_ids::MASK_GIANT);

        let ctx = MmEvalContext::new(&ram);
        assert!(ctx.has_item("MASK_GIANT", 1));
    }

    #[test]
    fn test_has_mask_don_gero() {
        let mut ram = MockRam::new();
        ram.set(
            BASE + offsets::MASK_DON_GERO as u32,
            item_ids::MASK_DON_GERO,
        );

        let ctx = MmEvalContext::new(&ram);
        assert!(ctx.has_item("MASK_DON_GERO", 1));
    }

    #[test]
    fn test_has_mask_scents() {
        let mut ram = MockRam::new();
        ram.set(BASE + offsets::MASK_SCENTS as u32, item_ids::MASK_SCENTS);

        let ctx = MmEvalContext::new(&ram);
        assert!(ctx.has_item("MASK_SCENTS", 1));
    }

    #[test]
    fn test_has_mask_kafei() {
        let mut ram = MockRam::new();
        ram.set(BASE + offsets::MASK_KAFEI as u32, item_ids::MASK_KAFEI);

        let ctx = MmEvalContext::new(&ram);
        assert!(ctx.has_item("MASK_KAFEI", 1));
    }

    #[test]
    fn test_has_mask_couple() {
        let mut ram = MockRam::new();
        ram.set(BASE + offsets::MASK_COUPLE as u32, item_ids::MASK_COUPLE);

        let ctx = MmEvalContext::new(&ram);
        assert!(ctx.has_item("MASK_COUPLE", 1));
    }

    #[test]
    fn test_has_mask_all_night() {
        let mut ram = MockRam::new();
        ram.set(
            BASE + offsets::MASK_ALL_NIGHT as u32,
            item_ids::MASK_ALL_NIGHT,
        );

        let ctx = MmEvalContext::new(&ram);
        assert!(ctx.has_item("MASK_ALL_NIGHT", 1));
    }

    #[test]
    fn test_has_mask_bremen() {
        let mut ram = MockRam::new();
        ram.set(BASE + offsets::MASK_BREMEN as u32, item_ids::MASK_BREMEN);

        let ctx = MmEvalContext::new(&ram);
        assert!(ctx.has_item("MASK_BREMEN", 1));
    }

    #[test]
    fn test_has_mask_romani() {
        let mut ram = MockRam::new();
        ram.set(BASE + offsets::MASK_ROMANI as u32, item_ids::MASK_ROMANI);

        let ctx = MmEvalContext::new(&ram);
        assert!(ctx.has_item("MASK_ROMANI", 1));
    }

    #[test]
    fn test_has_mask_postman() {
        let mut ram = MockRam::new();
        ram.set(BASE + offsets::MASK_POSTMAN as u32, item_ids::MASK_POSTMAN);

        let ctx = MmEvalContext::new(&ram);
        assert!(ctx.has_item("MASK_POSTMAN", 1));
    }

    #[test]
    fn test_has_mask_garo() {
        let mut ram = MockRam::new();
        ram.set(BASE + offsets::MASK_GARO as u32, item_ids::MASK_GARO);

        let ctx = MmEvalContext::new(&ram);
        assert!(ctx.has_item("MASK_GARO", 1));
    }

    #[test]
    fn test_has_mask_gibdo() {
        let mut ram = MockRam::new();
        ram.set(BASE + offsets::MASK_GIBDO as u32, item_ids::MASK_GIBDO);

        let ctx = MmEvalContext::new(&ram);
        assert!(ctx.has_item("MASK_GIBDO", 1));
    }

    #[test]
    fn test_has_mask_kamaro() {
        let mut ram = MockRam::new();
        ram.set(BASE + offsets::MASK_KAMARO as u32, item_ids::MASK_KAMARO);

        let ctx = MmEvalContext::new(&ram);
        assert!(ctx.has_item("MASK_KAMARO", 1));
    }

    // --- Inventory item tests ---

    #[test]
    fn test_has_hookshot() {
        let mut ram = MockRam::new();
        ram.set(BASE + offsets::HOOKSHOT as u32, item_ids::HOOKSHOT);

        let ctx = MmEvalContext::new(&ram);
        assert!(ctx.has_item("HOOKSHOT", 1));
    }

    #[test]
    fn test_has_bow() {
        let mut ram = MockRam::new();
        ram.set(BASE + offsets::BOW as u32, item_ids::BOW);

        let ctx = MmEvalContext::new(&ram);
        assert!(ctx.has_item("BOW", 1));
    }

    #[test]
    fn test_has_pictograph_box() {
        let mut ram = MockRam::new();
        ram.set(
            BASE + offsets::PICTOGRAPH_BOX as u32,
            item_ids::PICTOGRAPH_BOX,
        );

        let ctx = MmEvalContext::new(&ram);
        assert!(ctx.has_item("PICTOGRAPH_BOX", 1));
    }

    #[test]
    fn test_has_great_fairy_sword() {
        let mut ram = MockRam::new();
        ram.set(
            BASE + offsets::GREAT_FAIRY_SWORD as u32,
            item_ids::GREAT_FAIRY_SWORD,
        );

        let ctx = MmEvalContext::new(&ram);
        assert!(ctx.has_item("GREAT_FAIRY_SWORD", 1));
    }

    #[test]
    fn test_has_powder_keg() {
        let mut ram = MockRam::new();
        ram.set(BASE + offsets::POWDER_KEG as u32, item_ids::POWDER_KEG);

        let ctx = MmEvalContext::new(&ram);
        assert!(ctx.has_item("POWDER_KEG", 1));
    }

    #[test]
    fn test_missing_item() {
        let ram = MockRam::new(); // All slots empty (0xFF)
        let ctx = MmEvalContext::new(&ram);
        assert!(!ctx.has_item("HOOKSHOT", 1));
        assert!(!ctx.has_item("MASK_DEKU", 1));
    }

    // --- Boss remains tests ---

    #[test]
    fn test_has_remains_odolwa() {
        let mut ram = MockRam::new();
        ram.set_u32(
            BASE + offsets::QUEST_STATUS as u32,
            1 << quest_bits::REMAINS_ODOLWA,
        );

        let ctx = MmEvalContext::new(&ram);
        assert!(ctx.has_item("REMAINS_ODOLWA", 1));
    }

    #[test]
    fn test_has_remains_goht() {
        let mut ram = MockRam::new();
        ram.set_u32(
            BASE + offsets::QUEST_STATUS as u32,
            1 << quest_bits::REMAINS_GOHT,
        );

        let ctx = MmEvalContext::new(&ram);
        assert!(ctx.has_item("REMAINS_GOHT", 1));
    }

    #[test]
    fn test_has_remains_gyorg() {
        let mut ram = MockRam::new();
        ram.set_u32(
            BASE + offsets::QUEST_STATUS as u32,
            1 << quest_bits::REMAINS_GYORG,
        );

        let ctx = MmEvalContext::new(&ram);
        assert!(ctx.has_item("REMAINS_GYORG", 1));
    }

    #[test]
    fn test_has_remains_twinmold() {
        let mut ram = MockRam::new();
        ram.set_u32(
            BASE + offsets::QUEST_STATUS as u32,
            1 << quest_bits::REMAINS_TWINMOLD,
        );

        let ctx = MmEvalContext::new(&ram);
        assert!(ctx.has_item("REMAINS_TWINMOLD", 1));
    }

    // --- Owl statue tests ---

    #[test]
    fn test_has_owl_clock_town() {
        let mut ram = MockRam::new();
        ram.set_u32(
            BASE + offsets::QUEST_STATUS as u32,
            1 << quest_bits::OWL_CLOCK_TOWN,
        );

        let ctx = MmEvalContext::new(&ram);
        assert!(ctx.has_item("OWL_CLOCK_TOWN", 1));
    }

    #[test]
    fn test_has_owl_mountain_village() {
        let mut ram = MockRam::new();
        ram.set_u32(
            BASE + offsets::QUEST_STATUS as u32,
            1 << quest_bits::OWL_MOUNTAIN_VILLAGE,
        );

        let ctx = MmEvalContext::new(&ram);
        assert!(ctx.has_item("OWL_MOUNTAIN_VILLAGE", 1));
    }

    #[test]
    fn test_has_owl_ikana_canyon() {
        let mut ram = MockRam::new();
        ram.set_u32(
            BASE + offsets::QUEST_STATUS as u32,
            1 << quest_bits::OWL_IKANA_CANYON,
        );

        let ctx = MmEvalContext::new(&ram);
        assert!(ctx.has_item("OWL_IKANA_CANYON", 1));
    }

    // --- Magic upgrade tests ---

    #[test]
    fn test_has_magic() {
        let mut ram = MockRam::new();
        ram.set(BASE + offsets::MAGIC_SIZE as u32, 1);

        let ctx = MmEvalContext::new(&ram);
        assert!(ctx.has_item("MAGIC_UPGRADE", 1));
        assert!(ctx.has_item("SHARED_MAGIC_UPGRADE", 1));
        assert!(!ctx.has_item("DOUBLE_MAGIC", 1));
    }

    #[test]
    fn test_has_double_magic() {
        let mut ram = MockRam::new();
        ram.set(BASE + offsets::MAGIC_SIZE as u32, 2);

        let ctx = MmEvalContext::new(&ram);
        assert!(ctx.has_item("MAGIC_UPGRADE", 1));
        assert!(ctx.has_item("DOUBLE_MAGIC", 1));
    }

    // --- Spin upgrade tests ---

    #[test]
    fn test_has_spin_upgrade() {
        let mut ram = MockRam::new();
        ram.set_u32(
            BASE + offsets::QUEST_ITEMS as u32,
            1 << quest_item_bits::SPIN_UPGRADE,
        );

        let ctx = MmEvalContext::new(&ram);
        assert!(ctx.has_item("SPIN_UPGRADE", 1));
    }

    // --- Quest item tests ---

    #[test]
    fn test_has_moon_tear() {
        let mut ram = MockRam::new();
        ram.set(BASE + offsets::QUEST_ITEM_1 as u32, item_ids::MOON_TEAR);

        let ctx = MmEvalContext::new(&ram);
        assert!(ctx.has_item("MOON_TEAR", 1));
    }

    #[test]
    fn test_has_deed_land() {
        let mut ram = MockRam::new();
        ram.set(BASE + offsets::QUEST_ITEM_2 as u32, item_ids::DEED_LAND);

        let ctx = MmEvalContext::new(&ram);
        assert!(ctx.has_item("DEED_LAND", 1));
    }

    #[test]
    fn test_has_deed_swamp() {
        let mut ram = MockRam::new();
        ram.set(BASE + offsets::QUEST_ITEM_1 as u32, item_ids::DEED_SWAMP);

        let ctx = MmEvalContext::new(&ram);
        assert!(ctx.has_item("DEED_SWAMP", 1));
    }

    #[test]
    fn test_has_deed_mountain() {
        let mut ram = MockRam::new();
        ram.set(BASE + offsets::QUEST_ITEM_3 as u32, item_ids::DEED_MOUNTAIN);

        let ctx = MmEvalContext::new(&ram);
        assert!(ctx.has_item("DEED_MOUNTAIN", 1));
    }

    #[test]
    fn test_has_deed_ocean() {
        let mut ram = MockRam::new();
        ram.set(BASE + offsets::QUEST_ITEM_1 as u32, item_ids::DEED_OCEAN);

        let ctx = MmEvalContext::new(&ram);
        assert!(ctx.has_item("DEED_OCEAN", 1));
    }

    #[test]
    fn test_has_room_key() {
        let mut ram = MockRam::new();
        ram.set(BASE + offsets::QUEST_ITEM_2 as u32, item_ids::ROOM_KEY);

        let ctx = MmEvalContext::new(&ram);
        assert!(ctx.has_item("ROOM_KEY", 1));
    }

    #[test]
    fn test_has_letter_to_kafei() {
        let mut ram = MockRam::new();
        ram.set(
            BASE + offsets::QUEST_ITEM_1 as u32,
            item_ids::LETTER_TO_KAFEI,
        );

        let ctx = MmEvalContext::new(&ram);
        assert!(ctx.has_item("LETTER_TO_KAFEI", 1));
    }

    #[test]
    fn test_has_letter_to_mama() {
        let mut ram = MockRam::new();
        ram.set(
            BASE + offsets::QUEST_ITEM_3 as u32,
            item_ids::LETTER_TO_MAMA,
        );

        let ctx = MmEvalContext::new(&ram);
        assert!(ctx.has_item("LETTER_TO_MAMA", 1));
    }

    #[test]
    fn test_has_pendant_of_memories() {
        let mut ram = MockRam::new();
        ram.set(
            BASE + offsets::QUEST_ITEM_2 as u32,
            item_ids::PENDANT_OF_MEMORIES,
        );

        let ctx = MmEvalContext::new(&ram);
        assert!(ctx.has_item("PENDANT_OF_MEMORIES", 1));
    }

    // --- Age tests (MM-specific behavior) ---

    #[test]
    fn test_is_always_adult_in_mm() {
        let ram = MockRam::new();
        let ctx = MmEvalContext::new(&ram);

        // In MM, Link is always considered "adult" (no child form)
        assert!(ctx.is_adult());
        assert!(!ctx.is_child());
    }

    // --- Trick/Setting/Event tests ---

    #[test]
    fn test_tricks() {
        let ram = MockRam::new();
        let mut ctx = MmEvalContext::new(&ram);

        assert!(!ctx.trick("goron_bomb_jump"));
        ctx.add_trick("goron_bomb_jump");
        assert!(ctx.trick("goron_bomb_jump"));
        ctx.remove_trick("goron_bomb_jump");
        assert!(!ctx.trick("goron_bomb_jump"));
    }

    #[test]
    fn test_settings() {
        let ram = MockRam::new();
        let mut ctx = MmEvalContext::new(&ram);

        assert_eq!(ctx.setting("shuffle_songs"), None);
        ctx.set_setting("shuffle_songs", true);
        assert_eq!(ctx.setting("shuffle_songs"), Some(true));
    }

    #[test]
    fn test_events() {
        let ram = MockRam::new();
        let mut ctx = MmEvalContext::new(&ram);

        assert!(!ctx.event("EPONA_RESCUED"));
        ctx.add_event("EPONA_RESCUED");
        assert!(ctx.event("EPONA_RESCUED"));
        ctx.remove_event("EPONA_RESCUED");
        assert!(!ctx.event("EPONA_RESCUED"));
    }

    // --- Builder tests ---

    #[test]
    fn test_builder() {
        let ram = MockRam::new();
        let ctx = MmEvalContextBuilder::new(&ram)
            .with_trick("goron_bomb_jump")
            .with_setting("shuffle_songs", true)
            .with_event("EPONA_RESCUED")
            .build();

        assert!(ctx.trick("goron_bomb_jump"));
        assert_eq!(ctx.setting("shuffle_songs"), Some(true));
        assert!(ctx.event("EPONA_RESCUED"));
    }

    #[test]
    fn test_builder_with_custom_base() {
        let ram = MockRam::new();
        let ctx = MmEvalContextBuilder::new(&ram)
            .with_save_base(0x200000)
            .build();

        assert_eq!(ctx.save_base, 0x200000);
    }

    // --- Case insensitivity tests ---

    #[test]
    fn test_case_insensitive_items() {
        let mut ram = MockRam::new();
        ram.set(BASE + offsets::HOOKSHOT as u32, item_ids::HOOKSHOT);

        let ctx = MmEvalContext::new(&ram);
        assert!(ctx.has_item("HOOKSHOT", 1));
        assert!(ctx.has_item("hookshot", 1));
        assert!(ctx.has_item("Hookshot", 1));
    }

    // --- Unknown item tests ---

    #[test]
    fn test_unknown_item() {
        let ram = MockRam::new();
        let ctx = MmEvalContext::new(&ram);
        assert!(!ctx.has_item("UNKNOWN_ITEM", 1));
    }

    // --- MM time tests ---

    #[test]
    fn test_mm_time_day_1() {
        let mut ram = MockRam::new();
        ram.set(BASE + offsets::DAY as u32, 1); // Day 1
        ram.set_u16(BASE + offsets::TIME as u32, 0x0000); // 6:00 AM

        let ctx = MmEvalContext::new(&ram);
        assert_eq!(ctx.mm_time(), 0); // Start of Day 1
    }

    #[test]
    fn test_mm_time_midday() {
        let mut ram = MockRam::new();
        ram.set(BASE + offsets::DAY as u32, 1);
        ram.set_u16(BASE + offsets::TIME as u32, 0x8000); // Noon (halfway through day)

        let ctx = MmEvalContext::new(&ram);
        assert_eq!(ctx.mm_time(), 720); // 12 hours into Day 1
    }

    #[test]
    fn test_mm_time_day_2() {
        let mut ram = MockRam::new();
        ram.set(BASE + offsets::DAY as u32, 2); // Day 2
        ram.set_u16(BASE + offsets::TIME as u32, 0x0000); // 6:00 AM

        let ctx = MmEvalContext::new(&ram);
        assert_eq!(ctx.mm_time(), 1440); // Start of Day 2
    }

    // --- Bottle tests ---

    #[test]
    fn test_has_bottle() {
        let mut ram = MockRam::new();
        ram.set(BASE + offsets::BOTTLE_1 as u32, 0x12); // Empty bottle

        let ctx = MmEvalContext::new(&ram);
        assert!(ctx.has_item("BOTTLE", 1));
    }

    #[test]
    fn test_multiple_bottles() {
        let mut ram = MockRam::new();
        ram.set(BASE + offsets::BOTTLE_1 as u32, 0x12);
        ram.set(BASE + offsets::BOTTLE_2 as u32, 0x13);
        ram.set(BASE + offsets::BOTTLE_3 as u32, 0x14);

        let ctx = MmEvalContext::new(&ram);
        assert!(ctx.has_item("BOTTLE", 1));
    }

    // --- Day/Night tests ---

    #[test]
    fn test_is_day() {
        let mut ram = MockRam::new();
        ram.set(BASE + offsets::DAY as u32, 1);
        ram.set_u16(BASE + offsets::TIME as u32, 0x0000); // 6:00 AM (daytime)

        let ctx = MmEvalContext::new(&ram);
        assert!(ctx.is_day());
        assert!(!ctx.is_night());
    }

    #[test]
    fn test_is_night() {
        let mut ram = MockRam::new();
        ram.set(BASE + offsets::DAY as u32, 1);
        ram.set_u16(BASE + offsets::TIME as u32, 0xC000); // 6:00 PM (nighttime)

        let ctx = MmEvalContext::new(&ram);
        assert!(ctx.is_night());
        assert!(!ctx.is_day());
    }
}
