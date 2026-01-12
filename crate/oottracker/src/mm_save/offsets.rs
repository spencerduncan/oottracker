//! Memory offsets within MM SaveContext for OoTMM combo ROMs.
//!
//! This tracker only supports OoTMM Combomizer mode. Vanilla MM support
//! has been removed.

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
    /// Masks are in the same array as items, starting at index 24 (0x18)
    /// OoTMM masks are at items[24-47], so INVENTORY + 24 = 0x86
    /// Reference: OoTMM ITS_MM_MASK_POSTMAN = 0x18 (24 decimal)
    pub const MASKS: usize = 0x0086;
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
    /// Swamp skulltula count (u16)
    ///
    /// Offset calculation from MmSaveContext start:
    /// - MmSaveInfo starts at 0x24 within MmSave (ASSERT_OFFSET verified in OoTMM save.h)
    /// - skullCountSwamp is at 0xEC0 within MmSaveInfo (after permanentSceneFlags[120])
    /// - Absolute offset: 0x24 + 0xEC0 = 0x0EE4
    ///
    /// Note: permanentSceneFlags[120] is 120 × 0x1C = 0xD20 bytes, and skull counts
    /// come after this array plus additional fields (dekuPlaygroundHighScores, pictoFlags, etc.)
    ///
    /// Reference: https://github.com/OoTMM/OoTMM/blob/master/packages/core/include/combo/mm/save.h
    pub const SKULL_SWAMP: usize = 0x0EE4;
    /// Ocean skulltula count (u16)
    ///
    /// Offset calculation: MmSaveInfo(0x24) + skullCountOcean(0xEC2) = 0x0EE6
    pub const SKULL_OCEAN: usize = 0x0EE6;
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
