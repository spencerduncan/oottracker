//! Save state fixtures for E2E testing.
//!
//! This module provides simulated save state data representing various game states
//! for testing the tracker's event detection and state parsing.
//!
//! # OoT Save Context Structure
//!
//! The save context in OoT starts at address `0x11A5D0` and contains:
//! - Bytes 0x00-0x1B: Save header
//! - Bytes 0x1C-0x21: "ZELDAZ" magic number (identifies valid save)
//! - Bytes 0x22-0x2B: Player name
//! - Bytes 0x2C-0x3D: Death counter, equipment, items
//! - Bytes 0x74+: Inventory and equipment data
//! - Bytes 0xED4+: Event flags and quest status
//! - Bytes 0x135C-0x135F: Game mode (0 = gameplay)

use std::collections::HashMap;

/// OoT save context magic number "ZELDAZ".
pub const ZELDAZ_MAGIC: [u8; 6] = [0x5A, 0x45, 0x4C, 0x44, 0x41, 0x5A];

/// Offset of ZELDAZ magic within save context.
pub const ZELDAZ_OFFSET: usize = 0x1C;

/// Offset of game mode within save context.
pub const GAME_MODE_OFFSET: usize = 0x135C;

/// Size of the game mode field.
pub const GAME_MODE_SIZE: usize = 4;

/// Inventory offset within save context.
pub const INVENTORY_OFFSET: usize = 0x74;

/// Equipment offset within save context.
pub const EQUIPMENT_OFFSET: usize = 0x9C;

/// Quest status offset within save context.
pub const QUEST_STATUS_OFFSET: usize = 0xA4;

/// Event flags offset within save context.
pub const EVENT_FLAGS_OFFSET: usize = 0xED4;

/// Item slot indices in the inventory.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(usize)]
pub enum ItemSlot {
    DekuSticks = 0,
    DekuNuts = 1,
    Bombs = 2,
    FairyBow = 3,
    FireArrows = 4,
    DinsFire = 5,
    Slingshot = 6,
    FairyOcarina = 7,
    Bombchus = 8,
    Hookshot = 9,
    IceArrows = 10,
    FaroresWind = 11,
    Boomerang = 12,
    LensOfTruth = 13,
    MagicBeans = 14,
    MegatonHammer = 15,
    LightArrows = 16,
    NayrusLove = 17,
    AdultBottle1 = 18,
    AdultBottle2 = 19,
    AdultBottle3 = 20,
    AdultBottle4 = 21,
    ChildTrade = 22,
    AdultTrade = 23,
}

/// Item IDs for OoT items.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum ItemId {
    None = 0xFF,
    DekuStick = 0x00,
    DekuNut = 0x01,
    Bomb = 0x02,
    FairyBow = 0x03,
    FireArrow = 0x04,
    DinsFire = 0x05,
    FairySlingshot = 0x06,
    FairyOcarina = 0x07,
    OcarinaOfTime = 0x08,
    Bombchu = 0x09,
    Hookshot = 0x0A,
    Longshot = 0x0B,
    IceArrow = 0x0C,
    FaroresWind = 0x0D,
    Boomerang = 0x0E,
    LensOfTruth = 0x0F,
    MagicBean = 0x10,
    MegatonHammer = 0x11,
    LightArrow = 0x12,
    NayrusLove = 0x13,
    EmptyBottle = 0x14,
    RedPotion = 0x15,
    GreenPotion = 0x16,
    BluePotion = 0x17,
    BottledFairy = 0x18,
    Fish = 0x19,
    MilkBottle = 0x1A,
    Letter = 0x1B,
    BlueFire = 0x1C,
    Bug = 0x1D,
    BigPoe = 0x1E,
    HalfMilk = 0x1F,
    Poe = 0x20,
    WeirdEgg = 0x21,
    Chicken = 0x22,
    ZeldasLetter = 0x23,
    KeatonMask = 0x24,
    SkullMask = 0x25,
    SpookyMask = 0x26,
    BunnyHood = 0x27,
    GoronMask = 0x28,
    ZoraMask = 0x29,
    GerudoMask = 0x2A,
    MaskOfTruth = 0x2B,
    SoldOut = 0x2C,
    PocketEgg = 0x2D,
    PocketCucco = 0x2E,
    Cojiro = 0x2F,
    OddMushroom = 0x30,
    OddPotion = 0x31,
    PoachersSaw = 0x32,
    BrokenGoronSword = 0x33,
    Prescription = 0x34,
    EyeballFrog = 0x35,
    EyeDrops = 0x36,
    ClaimCheck = 0x37,
    KokiriSword = 0x3B,
    MasterSword = 0x3C,
    BiggoronSword = 0x3D,
    DekuShield = 0x3E,
    HylianShield = 0x3F,
    MirrorShield = 0x40,
    KokiriTunic = 0x41,
    GoronTunic = 0x42,
    ZoraTunic = 0x43,
    KokiriBoots = 0x44,
    IronBoots = 0x45,
    HoverBoots = 0x46,
}

/// Equipment bits in the equipment field.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Equipment {
    pub kokiri_sword: bool,
    pub master_sword: bool,
    pub biggoron_sword: bool,
    pub deku_shield: bool,
    pub hylian_shield: bool,
    pub mirror_shield: bool,
    pub kokiri_tunic: bool,
    pub goron_tunic: bool,
    pub zora_tunic: bool,
    pub kokiri_boots: bool,
    pub iron_boots: bool,
    pub hover_boots: bool,
}

impl Default for Equipment {
    fn default() -> Self {
        Self {
            kokiri_sword: false,
            master_sword: false,
            biggoron_sword: false,
            deku_shield: false,
            hylian_shield: false,
            mirror_shield: false,
            kokiri_tunic: true, // Link starts with Kokiri Tunic
            goron_tunic: false,
            zora_tunic: false,
            kokiri_boots: true, // Link starts with Kokiri Boots
            iron_boots: false,
            hover_boots: false,
        }
    }
}

impl Equipment {
    /// Converts equipment to the save context byte format.
    pub fn to_bytes(&self) -> [u8; 2] {
        let mut sword_shield: u8 = 0;
        let mut tunic_boots: u8 = 0;

        // Sword bits (bits 0-3 of sword_shield)
        if self.kokiri_sword {
            sword_shield |= 0x01;
        }
        if self.master_sword {
            sword_shield |= 0x02;
        }
        if self.biggoron_sword {
            sword_shield |= 0x04;
        }

        // Shield bits (bits 4-6 of sword_shield)
        if self.deku_shield {
            sword_shield |= 0x10;
        }
        if self.hylian_shield {
            sword_shield |= 0x20;
        }
        if self.mirror_shield {
            sword_shield |= 0x40;
        }

        // Tunic bits (bits 0-2 of tunic_boots)
        if self.kokiri_tunic {
            tunic_boots |= 0x01;
        }
        if self.goron_tunic {
            tunic_boots |= 0x02;
        }
        if self.zora_tunic {
            tunic_boots |= 0x04;
        }

        // Boots bits (bits 4-6 of tunic_boots)
        if self.kokiri_boots {
            tunic_boots |= 0x10;
        }
        if self.iron_boots {
            tunic_boots |= 0x20;
        }
        if self.hover_boots {
            tunic_boots |= 0x40;
        }

        [sword_shield, tunic_boots]
    }
}

/// Quest items and dungeon rewards.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct QuestStatus {
    pub kokiri_emerald: bool,
    pub goron_ruby: bool,
    pub zora_sapphire: bool,
    pub forest_medallion: bool,
    pub fire_medallion: bool,
    pub water_medallion: bool,
    pub spirit_medallion: bool,
    pub shadow_medallion: bool,
    pub light_medallion: bool,
    pub stone_of_agony: bool,
    pub gerudos_card: bool,
    pub gold_skulltulas: u8,
    pub heart_pieces: u8,
}

impl QuestStatus {
    /// Converts quest status to the save context byte format.
    pub fn to_bytes(&self) -> [u8; 4] {
        let mut bytes = [0u8; 4];

        // Spiritual stones (byte 0, bits 0-2)
        if self.kokiri_emerald {
            bytes[0] |= 0x04;
        }
        if self.goron_ruby {
            bytes[0] |= 0x02;
        }
        if self.zora_sapphire {
            bytes[0] |= 0x01;
        }

        // Medallions (byte 0, bits 3-7 and byte 1, bit 0)
        if self.forest_medallion {
            bytes[0] |= 0x08;
        }
        if self.fire_medallion {
            bytes[0] |= 0x10;
        }
        if self.water_medallion {
            bytes[0] |= 0x20;
        }
        if self.spirit_medallion {
            bytes[0] |= 0x40;
        }
        if self.shadow_medallion {
            bytes[0] |= 0x80;
        }
        if self.light_medallion {
            bytes[1] |= 0x01;
        }

        // Stone of Agony and Gerudo's Card (byte 1, bits 1-2)
        if self.stone_of_agony {
            bytes[1] |= 0x02;
        }
        if self.gerudos_card {
            bytes[1] |= 0x04;
        }

        // Gold skulltula count (byte 2) and heart pieces (byte 3)
        bytes[2] = self.gold_skulltulas;
        bytes[3] = self.heart_pieces;

        bytes
    }
}

/// Boss defeat flags.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct BossDefeats {
    pub queen_gohma: bool,
    pub king_dodongo: bool,
    pub barinade: bool,
    pub phantom_ganon: bool,
    pub volvagia: bool,
    pub morpha: bool,
    pub bongo_bongo: bool,
    pub twinrova: bool,
}

/// Game state fixture representing a specific point in the game.
#[derive(Debug, Clone)]
pub struct GameStateFixture {
    /// Unique identifier for this fixture.
    pub id: &'static str,
    /// Human-readable description.
    pub description: &'static str,
    /// Player's inventory items.
    pub inventory: HashMap<ItemSlot, ItemId>,
    /// Equipment owned.
    pub equipment: Equipment,
    /// Quest status.
    pub quest_status: QuestStatus,
    /// Boss defeats.
    pub boss_defeats: BossDefeats,
    /// Whether the player is adult (false = child).
    pub is_adult: bool,
    /// Current health (in quarter hearts).
    pub health: u16,
    /// Maximum health (in quarter hearts).
    pub max_health: u16,
    /// Current magic (0-48 for single bar, 0-96 for double).
    pub magic: u8,
    /// Whether player has double magic.
    pub double_magic: bool,
    /// Rupee count.
    pub rupees: u16,
}

impl Default for GameStateFixture {
    fn default() -> Self {
        Self {
            id: "empty",
            description: "Empty game state",
            inventory: HashMap::new(),
            equipment: Equipment::default(),
            quest_status: QuestStatus::default(),
            boss_defeats: BossDefeats::default(),
            is_adult: false,
            health: 12, // 3 hearts
            max_health: 12,
            magic: 0,
            double_magic: false,
            rupees: 0,
        }
    }
}

impl GameStateFixture {
    /// Creates a new game state fixture.
    pub fn new(id: &'static str, description: &'static str) -> Self {
        Self {
            id,
            description,
            ..Default::default()
        }
    }

    /// Adds an item to the inventory.
    pub fn with_item(mut self, slot: ItemSlot, item: ItemId) -> Self {
        self.inventory.insert(slot, item);
        self
    }

    /// Sets the equipment.
    pub fn with_equipment(mut self, equipment: Equipment) -> Self {
        self.equipment = equipment;
        self
    }

    /// Sets the quest status.
    pub fn with_quest_status(mut self, quest_status: QuestStatus) -> Self {
        self.quest_status = quest_status;
        self
    }

    /// Sets boss defeats.
    pub fn with_boss_defeats(mut self, boss_defeats: BossDefeats) -> Self {
        self.boss_defeats = boss_defeats;
        self
    }

    /// Sets the player as adult.
    pub fn as_adult(mut self) -> Self {
        self.is_adult = true;
        self
    }

    /// Sets the player health.
    pub fn with_health(mut self, current: u16, max: u16) -> Self {
        self.health = current;
        self.max_health = max;
        self
    }

    /// Generates a simulated save context buffer for this game state.
    ///
    /// This returns a byte array that mimics the structure of an OoT save context,
    /// suitable for testing tracker parsing logic.
    pub fn to_save_context(&self) -> Vec<u8> {
        // Create a buffer large enough for the save context areas we care about
        let mut buffer = vec![0u8; 0x1400];

        // Write ZELDAZ magic
        buffer[ZELDAZ_OFFSET..ZELDAZ_OFFSET + 6].copy_from_slice(&ZELDAZ_MAGIC);

        // Write game mode (0 = gameplay)
        buffer[GAME_MODE_OFFSET..GAME_MODE_OFFSET + GAME_MODE_SIZE].copy_from_slice(&[0, 0, 0, 0]);

        // Initialize all inventory slots to empty (0xFF)
        for i in 0..24 {
            buffer[INVENTORY_OFFSET + i] = ItemId::None as u8;
        }
        // Write inventory items
        for (&slot, &item) in &self.inventory {
            buffer[INVENTORY_OFFSET + slot as usize] = item as u8;
        }

        // Write equipment
        let equip_bytes = self.equipment.to_bytes();
        buffer[EQUIPMENT_OFFSET] = equip_bytes[0];
        buffer[EQUIPMENT_OFFSET + 1] = equip_bytes[1];

        // Write quest status
        let quest_bytes = self.quest_status.to_bytes();
        buffer[QUEST_STATUS_OFFSET..QUEST_STATUS_OFFSET + 4].copy_from_slice(&quest_bytes);

        // Write health data (at offset 0x30)
        buffer[0x30] = (self.health >> 8) as u8;
        buffer[0x31] = self.health as u8;
        buffer[0x2E] = (self.max_health >> 8) as u8;
        buffer[0x2F] = self.max_health as u8;

        // Write magic data (at offset 0x32)
        buffer[0x32] = self.magic;
        buffer[0x33] = if self.double_magic { 0x01 } else { 0x00 };

        // Write rupees (at offset 0x34)
        buffer[0x34] = (self.rupees >> 8) as u8;
        buffer[0x35] = self.rupees as u8;

        buffer
    }
}

// ============================================================================
// Pre-defined Fixtures
// ============================================================================

/// Creates a fixture for a new game (just started, in Kokiri Forest).
pub fn new_game() -> GameStateFixture {
    GameStateFixture::new("new_game", "Fresh game start in Kokiri Forest")
}

/// Creates a fixture for after getting the Kokiri Sword and Deku Shield.
pub fn kokiri_equipped() -> GameStateFixture {
    GameStateFixture::new(
        "kokiri_equipped",
        "After getting Kokiri Sword and Deku Shield",
    )
    .with_equipment(Equipment {
        kokiri_sword: true,
        deku_shield: true,
        ..Default::default()
    })
}

/// Creates a fixture for after defeating Queen Gohma.
pub fn deku_tree_complete() -> GameStateFixture {
    GameStateFixture::new("deku_tree_complete", "After defeating Queen Gohma")
        .with_equipment(Equipment {
            kokiri_sword: true,
            deku_shield: true,
            ..Default::default()
        })
        .with_item(ItemSlot::Slingshot, ItemId::FairySlingshot)
        .with_quest_status(QuestStatus {
            kokiri_emerald: true,
            ..Default::default()
        })
        .with_boss_defeats(BossDefeats {
            queen_gohma: true,
            ..Default::default()
        })
        .with_health(16, 16) // 4 hearts after heart container
}

/// Creates a fixture for after leaving Kokiri Forest (has Ocarina).
pub fn left_kokiri_forest() -> GameStateFixture {
    deku_tree_complete().with_item(ItemSlot::FairyOcarina, ItemId::FairyOcarina)
}

/// Creates a fixture for after defeating King Dodongo.
pub fn dodongos_cavern_complete() -> GameStateFixture {
    left_kokiri_forest()
        .with_item(ItemSlot::Bombs, ItemId::Bomb)
        .with_quest_status(QuestStatus {
            kokiri_emerald: true,
            goron_ruby: true,
            ..Default::default()
        })
        .with_boss_defeats(BossDefeats {
            queen_gohma: true,
            king_dodongo: true,
            ..Default::default()
        })
        .with_health(20, 20) // 5 hearts
}

/// Creates a fixture for after defeating Barinade.
pub fn jabu_jabus_belly_complete() -> GameStateFixture {
    dodongos_cavern_complete()
        .with_item(ItemSlot::Boomerang, ItemId::Boomerang)
        .with_quest_status(QuestStatus {
            kokiri_emerald: true,
            goron_ruby: true,
            zora_sapphire: true,
            ..Default::default()
        })
        .with_boss_defeats(BossDefeats {
            queen_gohma: true,
            king_dodongo: true,
            barinade: true,
            ..Default::default()
        })
        .with_health(24, 24) // 6 hearts
}

/// Creates a fixture for child with all three spiritual stones.
pub fn child_complete() -> GameStateFixture {
    jabu_jabus_belly_complete().with_item(ItemSlot::FairyOcarina, ItemId::OcarinaOfTime)
}

/// Creates a fixture for adult Link just after pulling the Master Sword.
pub fn adult_start() -> GameStateFixture {
    child_complete()
        .as_adult()
        .with_equipment(Equipment {
            kokiri_sword: true,
            master_sword: true,
            deku_shield: true,
            hylian_shield: true,
            ..Default::default()
        })
        .with_quest_status(QuestStatus {
            kokiri_emerald: true,
            goron_ruby: true,
            zora_sapphire: true,
            light_medallion: true,
            ..Default::default()
        })
}

/// Creates a fixture for after defeating Phantom Ganon.
pub fn forest_temple_complete() -> GameStateFixture {
    adult_start()
        .with_item(ItemSlot::FairyBow, ItemId::FairyBow)
        .with_item(ItemSlot::Hookshot, ItemId::Hookshot)
        .with_quest_status(QuestStatus {
            kokiri_emerald: true,
            goron_ruby: true,
            zora_sapphire: true,
            light_medallion: true,
            forest_medallion: true,
            ..Default::default()
        })
        .with_boss_defeats(BossDefeats {
            queen_gohma: true,
            king_dodongo: true,
            barinade: true,
            phantom_ganon: true,
            ..Default::default()
        })
        .with_health(28, 28) // 7 hearts
}

/// Creates a fixture for after defeating Volvagia.
pub fn fire_temple_complete() -> GameStateFixture {
    forest_temple_complete()
        .with_item(ItemSlot::MegatonHammer, ItemId::MegatonHammer)
        .with_equipment(Equipment {
            kokiri_sword: true,
            master_sword: true,
            deku_shield: true,
            hylian_shield: true,
            goron_tunic: true,
            ..Default::default()
        })
        .with_quest_status(QuestStatus {
            kokiri_emerald: true,
            goron_ruby: true,
            zora_sapphire: true,
            light_medallion: true,
            forest_medallion: true,
            fire_medallion: true,
            ..Default::default()
        })
        .with_boss_defeats(BossDefeats {
            queen_gohma: true,
            king_dodongo: true,
            barinade: true,
            phantom_ganon: true,
            volvagia: true,
            ..Default::default()
        })
        .with_health(32, 32) // 8 hearts
}

/// Creates a fixture for after defeating Morpha.
pub fn water_temple_complete() -> GameStateFixture {
    fire_temple_complete()
        .with_item(ItemSlot::Hookshot, ItemId::Longshot)
        .with_equipment(Equipment {
            kokiri_sword: true,
            master_sword: true,
            deku_shield: true,
            hylian_shield: true,
            goron_tunic: true,
            zora_tunic: true,
            iron_boots: true,
            ..Default::default()
        })
        .with_quest_status(QuestStatus {
            kokiri_emerald: true,
            goron_ruby: true,
            zora_sapphire: true,
            light_medallion: true,
            forest_medallion: true,
            fire_medallion: true,
            water_medallion: true,
            ..Default::default()
        })
        .with_boss_defeats(BossDefeats {
            queen_gohma: true,
            king_dodongo: true,
            barinade: true,
            phantom_ganon: true,
            volvagia: true,
            morpha: true,
            ..Default::default()
        })
        .with_health(36, 36) // 9 hearts
}

/// Creates a fixture for after defeating Bongo Bongo.
pub fn shadow_temple_complete() -> GameStateFixture {
    water_temple_complete()
        .with_item(ItemSlot::LensOfTruth, ItemId::LensOfTruth)
        .with_equipment(Equipment {
            kokiri_sword: true,
            master_sword: true,
            deku_shield: true,
            hylian_shield: true,
            goron_tunic: true,
            zora_tunic: true,
            iron_boots: true,
            hover_boots: true,
            ..Default::default()
        })
        .with_quest_status(QuestStatus {
            kokiri_emerald: true,
            goron_ruby: true,
            zora_sapphire: true,
            light_medallion: true,
            forest_medallion: true,
            fire_medallion: true,
            water_medallion: true,
            shadow_medallion: true,
            ..Default::default()
        })
        .with_boss_defeats(BossDefeats {
            queen_gohma: true,
            king_dodongo: true,
            barinade: true,
            phantom_ganon: true,
            volvagia: true,
            morpha: true,
            bongo_bongo: true,
            ..Default::default()
        })
        .with_health(40, 40) // 10 hearts
}

/// Creates a fixture for after defeating Twinrova.
pub fn spirit_temple_complete() -> GameStateFixture {
    shadow_temple_complete()
        .with_equipment(Equipment {
            kokiri_sword: true,
            master_sword: true,
            deku_shield: true,
            hylian_shield: true,
            mirror_shield: true,
            goron_tunic: true,
            zora_tunic: true,
            iron_boots: true,
            hover_boots: true,
            ..Default::default()
        })
        .with_quest_status(QuestStatus {
            kokiri_emerald: true,
            goron_ruby: true,
            zora_sapphire: true,
            light_medallion: true,
            forest_medallion: true,
            fire_medallion: true,
            water_medallion: true,
            shadow_medallion: true,
            spirit_medallion: true,
            ..Default::default()
        })
        .with_boss_defeats(BossDefeats {
            queen_gohma: true,
            king_dodongo: true,
            barinade: true,
            phantom_ganon: true,
            volvagia: true,
            morpha: true,
            bongo_bongo: true,
            twinrova: true,
        })
        .with_health(44, 44) // 11 hearts
}

/// Creates a fixture for a fully equipped save (all items, ready for Ganon).
pub fn ganon_ready() -> GameStateFixture {
    spirit_temple_complete()
        .with_item(ItemSlot::LightArrows, ItemId::LightArrow)
        .with_item(ItemSlot::FireArrows, ItemId::FireArrow)
        .with_item(ItemSlot::IceArrows, ItemId::IceArrow)
        .with_item(ItemSlot::DinsFire, ItemId::DinsFire)
        .with_item(ItemSlot::FaroresWind, ItemId::FaroresWind)
        .with_item(ItemSlot::NayrusLove, ItemId::NayrusLove)
        .with_equipment(Equipment {
            kokiri_sword: true,
            master_sword: true,
            biggoron_sword: true,
            deku_shield: true,
            hylian_shield: true,
            mirror_shield: true,
            kokiri_tunic: true,
            goron_tunic: true,
            zora_tunic: true,
            kokiri_boots: true,
            iron_boots: true,
            hover_boots: true,
        })
        .with_health(80, 80) // 20 hearts (max)
}

/// Returns all pre-defined fixtures.
pub fn all_fixtures() -> Vec<GameStateFixture> {
    vec![
        new_game(),
        kokiri_equipped(),
        deku_tree_complete(),
        left_kokiri_forest(),
        dodongos_cavern_complete(),
        jabu_jabus_belly_complete(),
        child_complete(),
        adult_start(),
        forest_temple_complete(),
        fire_temple_complete(),
        water_temple_complete(),
        shadow_temple_complete(),
        spirit_temple_complete(),
        ganon_ready(),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_game_fixture() {
        let fixture = new_game();
        assert_eq!(fixture.id, "new_game");
        assert!(!fixture.is_adult);
        assert!(fixture.inventory.is_empty());
    }

    #[test]
    fn test_save_context_has_zeldaz_magic() {
        let fixture = new_game();
        let context = fixture.to_save_context();

        assert_eq!(
            &context[ZELDAZ_OFFSET..ZELDAZ_OFFSET + 6],
            &ZELDAZ_MAGIC,
            "Save context should contain ZELDAZ magic"
        );
    }

    #[test]
    fn test_save_context_game_mode_is_gameplay() {
        let fixture = new_game();
        let context = fixture.to_save_context();

        assert_eq!(
            &context[GAME_MODE_OFFSET..GAME_MODE_OFFSET + 4],
            &[0, 0, 0, 0],
            "Game mode should be 0 (gameplay)"
        );
    }

    #[test]
    fn test_equipment_to_bytes() {
        let equip = Equipment {
            kokiri_sword: true,
            deku_shield: true,
            ..Default::default()
        };

        let bytes = equip.to_bytes();
        assert_eq!(bytes[0] & 0x01, 0x01, "Kokiri Sword bit should be set");
        assert_eq!(bytes[0] & 0x10, 0x10, "Deku Shield bit should be set");
    }

    #[test]
    fn test_quest_status_to_bytes() {
        let quest = QuestStatus {
            kokiri_emerald: true,
            forest_medallion: true,
            ..Default::default()
        };

        let bytes = quest.to_bytes();
        assert_eq!(bytes[0] & 0x04, 0x04, "Kokiri Emerald bit should be set");
        assert_eq!(bytes[0] & 0x08, 0x08, "Forest Medallion bit should be set");
    }

    #[test]
    fn test_fixture_progression() {
        let fixtures = all_fixtures();

        // Verify fixtures progress through the game
        assert_eq!(fixtures[0].id, "new_game");
        assert!(!fixtures[0].quest_status.kokiri_emerald);

        // Deku Tree complete should have emerald
        let deku_complete = &fixtures[2];
        assert!(deku_complete.quest_status.kokiri_emerald);
        assert!(deku_complete.boss_defeats.queen_gohma);

        // Adult start should have all stones
        let adult = &fixtures[7];
        assert!(adult.is_adult);
        assert!(adult.quest_status.kokiri_emerald);
        assert!(adult.quest_status.goron_ruby);
        assert!(adult.quest_status.zora_sapphire);
    }

    #[test]
    fn test_ganon_ready_has_all_medallions() {
        let ganon = ganon_ready();

        assert!(ganon.quest_status.forest_medallion);
        assert!(ganon.quest_status.fire_medallion);
        assert!(ganon.quest_status.water_medallion);
        assert!(ganon.quest_status.shadow_medallion);
        assert!(ganon.quest_status.spirit_medallion);
        assert!(ganon.quest_status.light_medallion);
    }

    #[test]
    fn test_all_fixtures_generate_valid_contexts() {
        for fixture in all_fixtures() {
            let context = fixture.to_save_context();

            // All contexts should have ZELDAZ magic
            assert_eq!(
                &context[ZELDAZ_OFFSET..ZELDAZ_OFFSET + 6],
                &ZELDAZ_MAGIC,
                "Fixture '{}' should have valid ZELDAZ magic",
                fixture.id
            );

            // All contexts should be in gameplay mode
            assert_eq!(
                &context[GAME_MODE_OFFSET..GAME_MODE_OFFSET + 4],
                &[0, 0, 0, 0],
                "Fixture '{}' should be in gameplay mode",
                fixture.id
            );
        }
    }
}
