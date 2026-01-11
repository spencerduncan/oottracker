//! Memory offsets within MM SaveContext for vanilla and OoTMM ROMs.

/// ROM type detection for MM save parsing
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum MmRomType {
    /// Standard vanilla Majora's Mask ROM
    #[default]
    Vanilla,
    /// OoTMM combo randomizer ROM
    OoTMM,
}

impl MmRomType {
    /// Get the ROM type from the `OOTTRACKER_MM_ROM_TYPE` environment variable.
    ///
    /// Set to "ootmm" or "OoTMM" for OoTMM combo ROM support.
    /// Defaults to `Vanilla` if not set or unrecognized.
    pub fn from_env() -> Self {
        match std::env::var("OOTTRACKER_MM_ROM_TYPE")
            .unwrap_or_default()
            .to_lowercase()
            .as_str()
        {
            "ootmm" | "combo" => MmRomType::OoTMM,
            _ => MmRomType::Vanilla,
        }
    }

    /// Determine if this is a combo ROM type.
    ///
    /// When `is_combo` is true, returns `OoTMM`, otherwise returns `Vanilla`.
    /// This is used when the game type is detected automatically rather than
    /// relying on environment variables.
    pub fn from_combo_flag(is_combo: bool) -> Self {
        if is_combo {
            MmRomType::OoTMM
        } else {
            MmRomType::Vanilla
        }
    }
}

/// Memory offsets within MM SaveContext - vanilla MM structure
pub mod vanilla_offsets {
    /// Player form (u8)
    pub const PLAYER_FORM: usize = 0x0020;
    /// Health capacity (u16, in 16ths of a heart)
    pub const HEALTH_CAPACITY: usize = 0x002C;
    /// Current health (u16)
    pub const HEALTH: usize = 0x002E;
    /// Magic capacity level (u8: 0=none, 1=single, 2=double)
    pub const MAGIC_LEVEL: usize = 0x0032;
    /// Rupees (u16)
    pub const RUPEES: usize = 0x0034;
    /// Sword and shield equipment bits (u8)
    pub const SWORD_SHIELD: usize = 0x0044;
    /// Double defense (u8: non-zero = has it)
    pub const DOUBLE_DEFENSE: usize = 0x003B;
    /// Inventory items array start (24 slots, each u8)
    pub const INVENTORY: usize = 0x0070;
    /// Mask inventory start (24 slots, each u8)
    pub const MASKS: usize = 0x0088;
    /// Quest items flags (u32)
    pub const QUEST_ITEMS: usize = 0x00A4;
    /// Dungeon items (4 dungeons × 1 byte each)
    pub const DUNGEON_ITEMS: usize = 0x00A8;
    /// Small keys for each dungeon (array starts here)
    pub const SMALL_KEYS: usize = 0x00BC;
    /// Upgrades (u32)
    pub const UPGRADES: usize = 0x00A0;
    /// Stray fairy counts (5 areas × 1 byte each): Clock Town, Woodfall, Snowhead, Great Bay, Stone Tower
    pub const STRAY_FAIRIES: usize = 0x00D0;
    /// Swamp skulltula count (u16)
    pub const SKULL_SWAMP: usize = 0x00D8;
    /// Ocean skulltula count (u16)
    pub const SKULL_OCEAN: usize = 0x00DA;
    /// Permanent scene flags start
    /// Decomp: SaveInfo at 0x24 + permanentSceneFlags at 0xD4 = 0xF8
    /// Reference: https://github.com/zeldaret/mm/blob/main/include/z64save.h
    pub const PERM_SCENE_FLAGS: usize = 0x00F8;
    /// Cycle scene flags start (after perm flags)
    /// Decomp: Cycle flags follow perm flags after additional save data
    pub const CYCLE_SCENE_FLAGS: usize = 0x0DF8;
    /// Current day (u32)
    pub const DAY: usize = 0x0048;
    /// Current time (u16)
    pub const TIME: usize = 0x004C;
    /// Is night flag (derived from time, but sometimes stored)
    pub const IS_NIGHT: usize = 0x0050;

    // Size constants for arrays
    pub const MASK_SLOTS: usize = 24;
    #[allow(dead_code)] // Documented for reference
    pub const INVENTORY_SLOTS: usize = 24;
}

/// Memory offsets within MM SaveContext - OoTMM combo ROM structure
///
/// OoTMM reorganizes the save structure significantly:
/// - Items and masks are combined into a single 48-slot array
/// - SaveInfo starts at offset 0x24 within MmSave
/// - All inventory-related offsets are shifted
///
/// Reference: https://github.com/OoTMM/OoTMM/blob/master/packages/core/include/combo/mm/save.h
pub mod ootmm_offsets {
    /// Player form (u8) - same as vanilla
    pub const PLAYER_FORM: usize = 0x0020;
    /// Health capacity (u16) - SaveInfo(0x24) + playerData(0x00) + healthCapacity(0x10)
    pub const HEALTH_CAPACITY: usize = 0x0034;
    /// Current health (u16)
    pub const HEALTH: usize = 0x0036;
    /// Magic capacity level (u8)
    pub const MAGIC_LEVEL: usize = 0x0038;
    /// Current magic (u8)
    #[allow(dead_code)] // Documented for reference
    pub const MAGIC: usize = 0x0039;
    /// Rupees (u16)
    pub const RUPEES: usize = 0x003A;
    /// Double defense (u8)
    pub const DOUBLE_DEFENSE: usize = 0x0042;
    /// Sword and shield equipment bits - in itemEquips at 0x4c + 0x20
    pub const SWORD_SHIELD: usize = 0x006C;
    /// Combined inventory array (48 slots: items 0-23, masks 24-47)
    /// SaveInfo(0x24) + inventory(0x4a) + items(0x00)
    pub const INVENTORY: usize = 0x006E;
    /// Masks are in the same array as items, starting at index 32 (0x20)
    /// OoTMM masks are at items[32-55], so INVENTORY + 32 = 0x8E
    /// Note: Not index 24 - the mask slots start at 32 in OoTMM
    pub const MASKS: usize = 0x008E;
    /// Ammo array (24 slots)
    #[allow(dead_code)] // Documented for reference
    pub const AMMO: usize = 0x009E;
    /// Upgrades (u32)
    pub const UPGRADES: usize = 0x00B6;
    /// Quest items flags (u32)
    pub const QUEST_ITEMS: usize = 0x00BA;
    /// Dungeon items (10 dungeons × 1 byte each)
    pub const DUNGEON_ITEMS: usize = 0x00BE;
    /// Small keys for each dungeon (9 dungeons)
    pub const SMALL_KEYS: usize = 0x00C8;
    /// Defense hearts (u8)
    #[allow(dead_code)] // Documented for reference
    pub const DEFENSE_HEARTS: usize = 0x00D1;
    /// Stray fairy counts (10 areas × 1 byte each)
    pub const STRAY_FAIRIES: usize = 0x00D2;
    /// Swamp skulltula count - need to verify this offset
    pub const SKULL_SWAMP: usize = 0x00DC;
    /// Ocean skulltula count - need to verify this offset
    pub const SKULL_OCEAN: usize = 0x00DE;
    /// Permanent scene flags start
    /// OoTMM: SaveInfo(0x24) + after inventory = 0xE4 from info start
    /// Calculation: MmSavePlayerData(0x28) + MmItemEquips(0x14) + MmInventory(0xA8) = 0xE4
    /// Absolute: 0x24 + 0xE4 = 0x108
    /// Reference: https://github.com/OoTMM/OoTMM/blob/master/packages/core/include/combo/mm/save.h
    pub const PERM_SCENE_FLAGS: usize = 0x0108;
    /// Cycle scene flags start
    /// OoTMM places cycle flags at 0x3F68 within MmSaveContext
    /// Reference: https://github.com/OoTMM/OoTMM/blob/master/packages/core/include/combo/mm/save.h
    pub const CYCLE_SCENE_FLAGS: usize = 0x3F68;
    /// Current day (u32)
    pub const DAY: usize = 0x0018;
    /// Days elapsed (u32)
    #[allow(dead_code)] // Documented for reference
    pub const DAYS_ELAPSED: usize = 0x001C;
    /// Current time (u16)
    pub const TIME: usize = 0x000C;
    /// Is night flag (s32)
    pub const IS_NIGHT: usize = 0x0010;

    // Size constants for arrays - OoTMM uses combined array
    pub const MASK_SLOTS: usize = 24;
    #[allow(dead_code)] // Documented for reference
    pub const INVENTORY_SLOTS: usize = 24;
    #[allow(dead_code)] // Documented for reference
    pub const COMBINED_INVENTORY_SLOTS: usize = 48;
}
