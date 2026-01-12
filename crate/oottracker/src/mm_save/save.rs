//! Main MmSave structure and parsing logic.

use byteorder::{BigEndian, ByteOrder as _};

use crate::mm_save::{
    constants::{mm_item_ids, MM_PERM_SCENE_COUNT, MM_PERM_SCENE_SIZE, MM_SIZE},
    dungeon_progress::{MmAllDungeonItems, MmDungeonItems, MmSmallKeys, MmStrayFairies},
    inventory::{MmBottle, MmInventory},
    masks::{MmMasks, MmMasksHigh, MmMasksLow, MmTransformationMasks},
    offsets::ootmm_offsets,
    quest_items::MmQuestItems,
    scene_flags::{MmCycleSceneFlags, MmPermanentSceneFlags},
    types::{MmDecodeError, MmMagicCapacity, MmShield, MmSword, PlayerForm},
    upgrades::MmUpgrades,
};

/// Complete MM save state
#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct MmSave {
    // Player state
    pub player_form: PlayerForm,
    pub health_capacity: u16,
    pub health: u16,
    pub magic: MmMagicCapacity,
    pub double_defense: bool,
    pub rupees: u16,

    // Equipment
    pub sword: MmSword,
    pub shield: MmShield,

    // Items
    pub inventory: MmInventory,
    pub masks: MmMasks,
    pub upgrades: MmUpgrades,
    pub quest_items: MmQuestItems,

    // Dungeon progress
    pub dungeon_items: MmAllDungeonItems,
    pub small_keys: MmSmallKeys,
    pub stray_fairies: MmStrayFairies,

    // Skulltula tokens
    pub skull_tokens_swamp: u16,
    pub skull_tokens_ocean: u16,

    // Scene flags
    pub permanent_scene_flags: Vec<MmPermanentSceneFlags>,
    pub cycle_scene_flags: Vec<MmCycleSceneFlags>,

    // Time state
    pub day: u32,
    pub time: u16,
    pub is_night: bool,
}

impl MmSave {
    /// Converts Majora's Mask save data into an `MmSave`.
    ///
    /// This parses OoTMM combo ROM save data format.
    /// Reference: https://github.com/OoTMM/OoTMM/blob/master/packages/core/include/combo/mm/save.h
    ///
    /// # Arguments
    /// * `save_data` - Raw save data bytes (must be exactly MM_SIZE bytes)
    ///
    /// # Errors
    /// Returns `MmDecodeError` if the data is invalid or wrong size.
    pub fn from_save_data(save_data: &[u8]) -> Result<MmSave, MmDecodeError> {
        use ootmm_offsets::*;

        // Helper macro to get a single byte at offset
        macro_rules! get_u8 {
            ($offset:expr) => {{
                *save_data
                    .get($offset)
                    .ok_or(MmDecodeError::Index($offset as u16))?
            }};
        }

        // Helper macro to get a u16 at offset (big endian)
        macro_rules! get_u16 {
            ($offset:expr) => {{
                let slice =
                    save_data
                        .get($offset..$offset + 2)
                        .ok_or(MmDecodeError::IndexRange {
                            start: $offset as u16,
                            end: ($offset + 2) as u16,
                        })?;
                BigEndian::read_u16(slice)
            }};
        }

        // Helper macro to get a u32 at offset (big endian)
        macro_rules! get_u32 {
            ($offset:expr) => {{
                let slice =
                    save_data
                        .get($offset..$offset + 4)
                        .ok_or(MmDecodeError::IndexRange {
                            start: $offset as u16,
                            end: ($offset + 4) as u16,
                        })?;
                BigEndian::read_u32(slice)
            }};
        }

        // Validate size
        if save_data.len() < MM_SIZE {
            return Err(MmDecodeError::Size(save_data.len()));
        }

        // Parse player form (same offset as vanilla)
        let player_form = PlayerForm::try_from(get_u8!(PLAYER_FORM)).unwrap_or(PlayerForm::Human);

        // Parse health (different offset in OoTMM)
        let health_capacity = get_u16!(HEALTH_CAPACITY);
        let health = get_u16!(HEALTH);

        // Parse magic
        let magic =
            MmMagicCapacity::try_from(get_u8!(MAGIC_LEVEL)).unwrap_or(MmMagicCapacity::None);

        // Parse double defense
        let double_defense = get_u8!(DOUBLE_DEFENSE) != 0;

        // Parse rupees
        let rupees = get_u16!(RUPEES);

        // Parse sword and shield from equipment u16 bitfield (OoTMM format)
        // On BIG-ENDIAN (N64), bitfields are packed from MSB to LSB:
        // bits 15-12: boots, bits 11-8: tunic, bits 7-4: shield, bits 3-0: sword
        let equipment = get_u16!(SWORD_SHIELD);
        let sword = MmSword::try_from((equipment & 0x0F) as u8).unwrap_or(MmSword::None);
        let shield = MmShield::try_from(((equipment >> 4) & 0x0F) as u8).unwrap_or(MmShield::None);

        // Parse inventory (OoTMM uses different offset)
        let inventory = Self::parse_inventory(save_data)?;

        // Parse masks (OoTMM has masks in same array as items, starting at index 24)
        let masks = Self::parse_masks(save_data)?;

        // Parse quest items (OoTMM uses same bit layout as vanilla MM)
        let quest_items = MmQuestItems::from_ootmm_bits(get_u32!(QUEST_ITEMS));

        // Parse upgrades (different offset in OoTMM)
        let upgrades = MmUpgrades::from_bits_truncate(get_u32!(UPGRADES));

        // Parse dungeon items (OoTMM has 10 dungeons, we use first 4)
        let dungeon_items = MmAllDungeonItems {
            woodfall: MmDungeonItems::from_bits_truncate(get_u8!(DUNGEON_ITEMS)),
            snowhead: MmDungeonItems::from_bits_truncate(get_u8!(DUNGEON_ITEMS + 1)),
            great_bay: MmDungeonItems::from_bits_truncate(get_u8!(DUNGEON_ITEMS + 2)),
            stone_tower: MmDungeonItems::from_bits_truncate(get_u8!(DUNGEON_ITEMS + 3)),
        };

        // Parse small keys (OoTMM has 9 dungeons, handle 0xFF as 0)
        let parse_key = |offset: usize| -> u8 {
            let val = save_data.get(offset).copied().unwrap_or(0xFF);
            if val == 0xFF {
                0
            } else {
                val
            }
        };
        let small_keys = MmSmallKeys {
            woodfall: parse_key(SMALL_KEYS),
            snowhead: parse_key(SMALL_KEYS + 1),
            great_bay: parse_key(SMALL_KEYS + 2),
            stone_tower: parse_key(SMALL_KEYS + 3),
        };

        // Parse stray fairies (OoTMM has 10 areas, we use first 5)
        let stray_fairies = MmStrayFairies {
            clock_town: get_u8!(STRAY_FAIRIES),
            woodfall: get_u8!(STRAY_FAIRIES + 1),
            snowhead: get_u8!(STRAY_FAIRIES + 2),
            great_bay: get_u8!(STRAY_FAIRIES + 3),
            stone_tower: get_u8!(STRAY_FAIRIES + 4),
        };

        // Parse skulltula tokens
        let skull_tokens_swamp = get_u16!(SKULL_SWAMP);
        let skull_tokens_ocean = get_u16!(SKULL_OCEAN);

        // Parse permanent scene flags (same structure, potentially same offset)
        let permanent_scene_flags = Self::parse_permanent_scene_flags(save_data)?;

        // Parse cycle scene flags
        let cycle_scene_flags = Self::parse_cycle_scene_flags(save_data)?;

        // Parse time state (different offsets in OoTMM)
        let day = get_u32!(DAY);
        let time = get_u16!(TIME);
        let is_night = get_u32!(IS_NIGHT) != 0; // Note: IS_NIGHT is s32 in OoTMM

        Ok(MmSave {
            player_form,
            health_capacity,
            health,
            magic,
            double_defense,
            rupees,
            sword,
            shield,
            inventory,
            masks,
            upgrades,
            quest_items,
            dungeon_items,
            small_keys,
            stray_fairies,
            skull_tokens_swamp,
            skull_tokens_ocean,
            permanent_scene_flags,
            cycle_scene_flags,
            day,
            time,
            is_night,
        })
    }

    /// Parse inventory items from save data
    fn parse_inventory(save_data: &[u8]) -> Result<MmInventory, MmDecodeError> {
        use mm_item_ids::*;
        use ootmm_offsets::*;

        let get_item =
            |offset: usize| -> u8 { save_data.get(INVENTORY + offset).copied().unwrap_or(NONE) };

        // Check presence of items by their inventory slot contents
        let ocarina = get_item(0) == OCARINA;
        let bow = get_item(1) == BOW;
        let fire_arrows = get_item(2) == FIRE_ARROW;
        let ice_arrows = get_item(3) == ICE_ARROW;
        let light_arrows = get_item(4) == LIGHT_ARROW;
        let bombs = get_item(6) == BOMB;
        let bombchus = get_item(7) == BOMBCHU;
        let deku_sticks = get_item(8) == DEKU_STICK;
        let deku_nuts = get_item(9) == DEKU_NUT;
        let magic_beans = get_item(10) == MAGIC_BEAN;
        let powder_keg = get_item(12) == POWDER_KEG;
        let pictograph_box = get_item(13) == PICTOGRAPH_BOX;
        let lens = get_item(14) == LENS;
        let hookshot = get_item(15) == HOOKSHOT;
        let great_fairy_sword = get_item(16) == GREAT_FAIRY_SWORD;

        // Parse bottles (slots 18-23 in inventory)
        let parse_bottle = |slot: usize| -> MmBottle {
            let val = get_item(slot);
            MmBottle::try_from(val).unwrap_or(MmBottle::None)
        };

        let bottles = [
            parse_bottle(18),
            parse_bottle(19),
            parse_bottle(20),
            parse_bottle(21),
            parse_bottle(22),
            parse_bottle(23),
        ];

        Ok(MmInventory {
            ocarina,
            bow,
            fire_arrows,
            ice_arrows,
            light_arrows,
            bombs,
            bombchus,
            deku_sticks,
            deku_nuts,
            magic_beans,
            powder_keg,
            pictograph_box,
            lens,
            hookshot,
            great_fairy_sword,
            bottles,
        })
    }

    /// Parse masks from the mask inventory slots
    ///
    /// Masks are stored in the combined inventory array starting at the MASKS offset.
    fn parse_masks(save_data: &[u8]) -> Result<MmMasks, MmDecodeError> {
        use mm_item_ids::*;
        use ootmm_offsets::*;

        let mut transformation = MmTransformationMasks::empty();
        let mut masks_low = MmMasksLow::empty();
        let mut masks_high = MmMasksHigh::empty();

        // Scan all 24 mask slots
        for i in 0..MASK_SLOTS {
            let mask_id = save_data.get(MASKS + i).copied().unwrap_or(NONE);
            match mask_id {
                MASK_DEKU => transformation.insert(MmTransformationMasks::DEKU),
                MASK_GORON => transformation.insert(MmTransformationMasks::GORON),
                MASK_ZORA => transformation.insert(MmTransformationMasks::ZORA),
                MASK_FIERCE_DEITY => transformation.insert(MmTransformationMasks::FIERCE_DEITY),
                MASK_POSTMAN => masks_low.insert(MmMasksLow::POSTMAN),
                MASK_ALL_NIGHT => masks_low.insert(MmMasksLow::ALL_NIGHT),
                MASK_BLAST => masks_low.insert(MmMasksLow::BLAST),
                MASK_STONE => masks_low.insert(MmMasksLow::STONE),
                MASK_GREAT_FAIRY => masks_low.insert(MmMasksLow::GREAT_FAIRY),
                MASK_KEATON => masks_low.insert(MmMasksLow::KEATON),
                MASK_BREMEN => masks_low.insert(MmMasksLow::BREMEN),
                MASK_BUNNY => masks_low.insert(MmMasksLow::BUNNY),
                MASK_DON_GERO => masks_low.insert(MmMasksLow::DON_GERO),
                MASK_SCENTS => masks_low.insert(MmMasksLow::SCENTS),
                MASK_ROMANI => masks_low.insert(MmMasksLow::ROMANI),
                MASK_CIRCUS_LEADER => masks_low.insert(MmMasksLow::CIRCUS_LEADER),
                MASK_KAFEI => masks_low.insert(MmMasksLow::KAFEI),
                MASK_COUPLES => masks_low.insert(MmMasksLow::COUPLES),
                MASK_TRUTH => masks_low.insert(MmMasksLow::TRUTH),
                MASK_KAMARO => masks_low.insert(MmMasksLow::KAMARO),
                MASK_GIBDO => masks_high.insert(MmMasksHigh::GIBDO),
                MASK_GARO => masks_high.insert(MmMasksHigh::GARO),
                MASK_CAPTAIN => masks_high.insert(MmMasksHigh::CAPTAIN),
                MASK_GIANT => masks_high.insert(MmMasksHigh::GIANT),
                _ => {}
            }
        }

        Ok(MmMasks {
            transformation,
            masks_low,
            masks_high,
        })
    }

    /// Parse permanent scene flags (120 slots)
    fn parse_permanent_scene_flags(
        save_data: &[u8],
    ) -> Result<Vec<MmPermanentSceneFlags>, MmDecodeError> {
        use ootmm_offsets::PERM_SCENE_FLAGS;

        let mut flags = Vec::with_capacity(MM_PERM_SCENE_COUNT);

        for i in 0..MM_PERM_SCENE_COUNT {
            let base = PERM_SCENE_FLAGS + (i * MM_PERM_SCENE_SIZE);

            let get_u32_at = |offset: usize| -> u32 {
                save_data
                    .get(base + offset..base + offset + 4)
                    .map(BigEndian::read_u32)
                    .unwrap_or(0)
            };

            flags.push(MmPermanentSceneFlags {
                chest: get_u32_at(0x00),
                switch0: get_u32_at(0x04),
                switch1: get_u32_at(0x08),
                cleared_room: get_u32_at(0x0c),
                collectible: get_u32_at(0x10),
                cleared_floors: get_u32_at(0x14),
                rooms: get_u32_at(0x18),
            });
        }

        Ok(flags)
    }

    /// Parse cycle scene flags (reset on Song of Time)
    fn parse_cycle_scene_flags(save_data: &[u8]) -> Result<Vec<MmCycleSceneFlags>, MmDecodeError> {
        use ootmm_offsets::CYCLE_SCENE_FLAGS;

        const CYCLE_SCENE_SIZE: usize = 0x14;
        const CYCLE_SCENE_COUNT: usize = 120;

        let mut flags = Vec::with_capacity(CYCLE_SCENE_COUNT);

        for i in 0..CYCLE_SCENE_COUNT {
            let base = CYCLE_SCENE_FLAGS + (i * CYCLE_SCENE_SIZE);

            let get_u32_at = |offset: usize| -> u32 {
                save_data
                    .get(base + offset..base + offset + 4)
                    .map(BigEndian::read_u32)
                    .unwrap_or(0)
            };

            flags.push(MmCycleSceneFlags {
                chest: get_u32_at(0x00),
                switch0: get_u32_at(0x04),
                switch1: get_u32_at(0x08),
                cleared_room: get_u32_at(0x0c),
                collectible: get_u32_at(0x10),
            });
        }

        Ok(flags)
    }

    /// Returns the number of heart containers (full hearts).
    /// MM starts with 3 hearts, max is 20.
    pub fn heart_containers(&self) -> u8 {
        // health_capacity is in 16ths of a heart (0x10 per heart)
        (self.health_capacity / 0x10) as u8
    }
}
