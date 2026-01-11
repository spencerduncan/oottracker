//! MM save data constants including item IDs and memory addresses.

/// MM SaveContext base address in N64 memory
pub const MM_ADDR: u32 = 0x801ef670;
/// MM SaveContext size in bytes
pub const MM_SIZE: usize = 0x48d0;
/// Number of permanent scene flag slots in MM
pub const MM_PERM_SCENE_COUNT: usize = 120;
/// Size of each permanent scene flag entry in bytes
pub const MM_PERM_SCENE_SIZE: usize = 0x1c;

/// MM inventory item IDs
pub mod mm_item_ids {
    pub const OCARINA: u8 = 0x00;
    pub const BOW: u8 = 0x01;
    pub const FIRE_ARROW: u8 = 0x02;
    pub const ICE_ARROW: u8 = 0x03;
    pub const LIGHT_ARROW: u8 = 0x04;
    pub const QUEST_1: u8 = 0x05; // unused slot
    pub const BOMB: u8 = 0x06;
    pub const BOMBCHU: u8 = 0x07;
    pub const DEKU_STICK: u8 = 0x08;
    pub const DEKU_NUT: u8 = 0x09;
    pub const MAGIC_BEAN: u8 = 0x0A;
    pub const QUEST_2: u8 = 0x0B; // unused slot
    pub const POWDER_KEG: u8 = 0x0C;
    pub const PICTOGRAPH_BOX: u8 = 0x0D;
    pub const LENS: u8 = 0x0E;
    pub const HOOKSHOT: u8 = 0x0F;
    pub const GREAT_FAIRY_SWORD: u8 = 0x10;
    pub const QUEST_3: u8 = 0x11; // unused slot
                                  // Bottles start at 0x12
    pub const BOTTLE_EMPTY: u8 = 0x12;
    pub const BOTTLE_RED_POTION: u8 = 0x13;
    pub const BOTTLE_GREEN_POTION: u8 = 0x14;
    pub const BOTTLE_BLUE_POTION: u8 = 0x15;
    pub const BOTTLE_FAIRY: u8 = 0x16;
    pub const BOTTLE_DEKU_PRINCESS: u8 = 0x17;
    pub const BOTTLE_MILK: u8 = 0x18;
    pub const BOTTLE_MILK_HALF: u8 = 0x19;
    pub const BOTTLE_FISH: u8 = 0x1A;
    pub const BOTTLE_BUG: u8 = 0x1B;
    pub const BOTTLE_BLUE_FIRE: u8 = 0x1C;
    pub const BOTTLE_POE: u8 = 0x1D;
    pub const BOTTLE_BIG_POE: u8 = 0x1E;
    pub const BOTTLE_WATER: u8 = 0x1F;
    pub const BOTTLE_HOT_SPRING_WATER: u8 = 0x20;
    pub const BOTTLE_ZORA_EGG: u8 = 0x21;
    pub const BOTTLE_GOLD_DUST: u8 = 0x22;
    pub const BOTTLE_MUSHROOM: u8 = 0x23;
    pub const BOTTLE_SEAHORSE: u8 = 0x24;
    pub const BOTTLE_CHATEAU_ROMANI: u8 = 0x25;
    pub const BOTTLE_MYSTERY_MILK: u8 = 0x26;
    pub const BOTTLE_MYSTERY_MILK_SPOILED: u8 = 0x27;
    // Masks start at 0x32 - IDs match zeldaret/mm decomp project
    // https://github.com/zeldaret/mm/blob/main/include/z64item.h
    pub const MASK_DEKU: u8 = 0x32;
    pub const MASK_GORON: u8 = 0x33;
    pub const MASK_ZORA: u8 = 0x34;
    pub const MASK_FIERCE_DEITY: u8 = 0x35;
    pub const MASK_TRUTH: u8 = 0x36;
    pub const MASK_KAFEI: u8 = 0x37;
    pub const MASK_ALL_NIGHT: u8 = 0x38;
    pub const MASK_BUNNY: u8 = 0x39;
    pub const MASK_KEATON: u8 = 0x3A;
    pub const MASK_GARO: u8 = 0x3B;
    pub const MASK_ROMANI: u8 = 0x3C;
    pub const MASK_CIRCUS_LEADER: u8 = 0x3D;
    pub const MASK_POSTMAN: u8 = 0x3E;
    pub const MASK_COUPLES: u8 = 0x3F;
    pub const MASK_GREAT_FAIRY: u8 = 0x40;
    pub const MASK_GIBDO: u8 = 0x41;
    pub const MASK_DON_GERO: u8 = 0x42;
    pub const MASK_KAMARO: u8 = 0x43;
    pub const MASK_CAPTAIN: u8 = 0x44;
    pub const MASK_STONE: u8 = 0x45;
    pub const MASK_BREMEN: u8 = 0x46;
    pub const MASK_BLAST: u8 = 0x47;
    pub const MASK_SCENTS: u8 = 0x48;
    pub const MASK_GIANT: u8 = 0x49;
    pub const NONE: u8 = 0xFF;
}
