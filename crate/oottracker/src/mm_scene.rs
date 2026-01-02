//! Majora's Mask scene flag definitions for tracking location checks.
//!
//! MM has 120 permanent scene flag slots (vs OoT's 101/124). Each scene slot
//! contains the following flag types:
//! - `chest`: Chest collection flags (u32)
//! - `switch0`: Switch flags bank 0 (u32)
//! - `switch1`: Switch flags bank 1 (u32)
//! - `cleared_room`: Room clear flags (u32)
//! - `collectible`: Collectible pickup flags (u32)
//! - `cleared_floors`: Floor clear flags (u32)
//! - `rooms`: Visited rooms flags (u32)
//!
//! MM also has cycle-based flags that reset when playing the Song of Time.
//! This module handles both permanent (saved) and cycle-based (temporary) flags.

use {
    bitflags::bitflags,
    byteorder::{BigEndian, ByteOrder},
    std::fmt,
};

/// Number of permanent scene flag slots in MM
pub const MM_NUM_SCENES: usize = 120;

/// Size of each scene flag entry in bytes (7 × 4 = 28 bytes)
pub const MM_SCENE_SIZE: usize = 0x1c;

/// Total size of permanent scene flag data
pub const MM_SCENE_FLAGS_SIZE: usize = MM_NUM_SCENES * MM_SCENE_SIZE;

// ============================================================================
// Scene Identification
// ============================================================================

/// MM scene identification wrapper
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MmScene(pub &'static str);

impl MmScene {
    /// Get scene from scene ID
    pub fn from_id(scene_id: u8) -> Option<MmScene> {
        Some(MmScene(match scene_id {
            // Main Dungeons
            0x1F => "Woodfall Temple",
            0x22 => "Snowhead Temple",
            0x1E => "Great Bay Temple",
            0x18 => "Stone Tower Temple",
            0x19 => "Stone Tower Temple Inverted",

            // Dungeon Boss Rooms
            0x1A => "Woodfall Temple Boss Room",
            0x24 => "Snowhead Temple Boss Room",
            0x4F => "Great Bay Temple Boss Room",
            0x36 => "Stone Tower Temple Boss Room",

            // Mini Dungeons
            0x1B => "Beneath the Well",
            0x11 => "Ancient Castle of Ikana",
            0x13 => "Ikana Canyon Secret Shrine",
            0x29 => "Pirates Fortress",
            0x2A => "Pirates Fortress Interior",
            0x07 => "Beneath the Graveyard",

            // Spider Houses
            0x27 => "Swamp Spider House",
            0x28 => "Oceanside Spider House",

            // Clock Town Areas
            0x6C => "South Clock Town",
            0x6D => "North Clock Town",
            0x6E => "East Clock Town",
            0x6F => "West Clock Town",
            0x70 => "Laundry Pool",
            0x08 => "Clock Tower Interior",

            // Clock Town Buildings
            0x4D => "Stock Pot Inn",
            0x4B => "Stock Pot Inn Reservation",
            0x51 => "Milk Bar",
            0x4E => "Mayor's Office",
            0x30 => "Post Office",
            0x42 => "Lottery Shop",
            0x4A => "Trading Post",
            0x32 => "Bomb Shop",
            0x33 => "Curiosity Shop",
            0x44 => "Honey and Darling",
            0x4C => "Treasure Chest Shop",
            0x52 => "Astral Observatory",
            0x26 => "Clock Town Great Fairy Fountain",

            // Termina Field
            0x54 => "Termina Field",
            0x0D => "Road to Southern Swamp",
            0x5E => "Milk Road",
            0x64 => "Path to Mountain Village",
            0x47 => "Road to Ikana",
            0x37 => "Great Bay Coast",

            // Southern Swamp
            0x55 => "Southern Swamp",
            0x56 => "Southern Swamp Poisoned",
            0x5C => "Swamp Tourist Center",
            0x59 => "Deku Palace",
            0x2C => "Deku Shrine",
            0x5D => "Deku Palace Throne Room",
            0x14 => "Woodfall",
            0x20 => "Swamp Great Fairy Fountain",

            // Mountain Village
            0x65 => "Mountain Village",
            0x5B => "Mountain Village Spring",
            0x48 => "Twin Islands",
            0x49 => "Twin Islands Spring",
            0x6A => "Goron Village",
            0x6B => "Goron Village Spring",
            0x2D => "Goron Shrine",
            0x50 => "Mountain Village Smithy",
            0x21 => "Snowhead",
            0x25 => "Mountain Great Fairy Fountain",

            // Great Bay
            0x38 => "Zora Cape",
            0x39 => "Zora Hall",
            0x58 => "Zora Hall Rooms",
            0x3E => "Pirates Fortress Exterior",
            0x3F => "Pinnacle Rock",
            0x2B => "Waterfall Rapids",
            0x60 => "Ocean Great Fairy Fountain",

            // Ikana
            0x46 => "Ikana Canyon",
            0x10 => "Ikana Graveyard",
            0x12 => "Ikana Castle",
            0x0F => "Stone Tower",
            0x15 => "Ikana Great Fairy Fountain",

            // Romani Ranch
            0x5F => "Romani Ranch",
            0x63 => "Cucco Shack",
            0x62 => "Doggy Racetrack",
            0x5A => "Gorman Track",

            // Moon (uses different scene IDs)
            0x67 => "Moon Deku Trial",
            0x68 => "Moon Goron Trial",
            0x69 => "Moon Zora Trial",
            0x66 => "Moon Link Trial",

            // Grottos and Misc
            0x17 => "Lens of Truth Cave",
            0x53 => "Termina Field Grotto",

            _ => return None,
        }))
    }

    /// Get the scene name
    pub fn name(&self) -> &'static str {
        self.0
    }
}

impl fmt::Display for MmScene {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

// ============================================================================
// Scene Flag Structures
// ============================================================================

/// Permanent scene flags for a single MM scene
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct MmSceneFlags {
    pub chest: u32,
    pub switch0: u32,
    pub switch1: u32,
    pub cleared_room: u32,
    pub collectible: u32,
    pub cleared_floors: u32,
    pub rooms: u32,
}

impl MmSceneFlags {
    /// Check if a specific chest flag is set
    pub fn chest_collected(&self, flag: u32) -> bool {
        (self.chest & flag) != 0
    }

    /// Check if a specific collectible flag is set
    pub fn collectible_obtained(&self, flag: u32) -> bool {
        (self.collectible & flag) != 0
    }

    /// Check if a specific switch flag is set (bank 0)
    pub fn switch0_set(&self, flag: u32) -> bool {
        (self.switch0 & flag) != 0
    }

    /// Check if a specific switch flag is set (bank 1)
    pub fn switch1_set(&self, flag: u32) -> bool {
        (self.switch1 & flag) != 0
    }

    /// Check if a room has been cleared
    pub fn room_cleared(&self, flag: u32) -> bool {
        (self.cleared_room & flag) != 0
    }
}

impl TryFrom<&[u8]> for MmSceneFlags {
    type Error = ();

    fn try_from(data: &[u8]) -> Result<Self, Self::Error> {
        if data.len() < MM_SCENE_SIZE {
            return Err(());
        }
        Ok(MmSceneFlags {
            chest: BigEndian::read_u32(&data[0x00..0x04]),
            switch0: BigEndian::read_u32(&data[0x04..0x08]),
            switch1: BigEndian::read_u32(&data[0x08..0x0c]),
            cleared_room: BigEndian::read_u32(&data[0x0c..0x10]),
            collectible: BigEndian::read_u32(&data[0x10..0x14]),
            cleared_floors: BigEndian::read_u32(&data[0x14..0x18]),
            rooms: BigEndian::read_u32(&data[0x18..0x1c]),
        })
    }
}

impl From<MmSceneFlags> for Vec<u8> {
    fn from(flags: MmSceneFlags) -> Vec<u8> {
        let mut buf = vec![0u8; MM_SCENE_SIZE];
        BigEndian::write_u32(&mut buf[0x00..0x04], flags.chest);
        BigEndian::write_u32(&mut buf[0x04..0x08], flags.switch0);
        BigEndian::write_u32(&mut buf[0x08..0x0c], flags.switch1);
        BigEndian::write_u32(&mut buf[0x0c..0x10], flags.cleared_room);
        BigEndian::write_u32(&mut buf[0x10..0x14], flags.collectible);
        BigEndian::write_u32(&mut buf[0x14..0x18], flags.cleared_floors);
        BigEndian::write_u32(&mut buf[0x18..0x1c], flags.rooms);
        buf
    }
}

/// Cycle-based scene flags that reset on Song of Time
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct MmCycleFlags {
    pub chest: u32,
    pub switch0: u32,
    pub switch1: u32,
    pub cleared_room: u32,
    pub collectible: u32,
}

impl MmCycleFlags {
    /// Check if a cycle-based chest has been collected this cycle
    pub fn chest_collected(&self, flag: u32) -> bool {
        (self.chest & flag) != 0
    }

    /// Check if a cycle-based collectible has been obtained this cycle
    pub fn collectible_obtained(&self, flag: u32) -> bool {
        (self.collectible & flag) != 0
    }
}

impl TryFrom<&[u8]> for MmCycleFlags {
    type Error = ();

    fn try_from(data: &[u8]) -> Result<Self, Self::Error> {
        if data.len() < 20 {
            return Err(());
        }
        Ok(MmCycleFlags {
            chest: BigEndian::read_u32(&data[0x00..0x04]),
            switch0: BigEndian::read_u32(&data[0x04..0x08]),
            switch1: BigEndian::read_u32(&data[0x08..0x0c]),
            cleared_room: BigEndian::read_u32(&data[0x0c..0x10]),
            collectible: BigEndian::read_u32(&data[0x10..0x14]),
        })
    }
}

impl From<MmCycleFlags> for Vec<u8> {
    fn from(flags: MmCycleFlags) -> Vec<u8> {
        let mut buf = vec![0u8; 20];
        BigEndian::write_u32(&mut buf[0x00..0x04], flags.chest);
        BigEndian::write_u32(&mut buf[0x04..0x08], flags.switch0);
        BigEndian::write_u32(&mut buf[0x08..0x0c], flags.switch1);
        BigEndian::write_u32(&mut buf[0x0c..0x10], flags.cleared_room);
        BigEndian::write_u32(&mut buf[0x10..0x14], flags.collectible);
        buf
    }
}

// ============================================================================
// All Permanent Scene Flags Container
// ============================================================================

/// Container for all MM permanent scene flags (120 scenes)
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MmAllSceneFlags {
    scenes: [MmSceneFlags; MM_NUM_SCENES],
}

impl Default for MmAllSceneFlags {
    fn default() -> Self {
        Self {
            scenes: [MmSceneFlags::default(); MM_NUM_SCENES],
        }
    }
}

impl MmAllSceneFlags {
    /// Create new empty scene flags
    pub fn new() -> Self {
        Self::default()
    }

    /// Get scene flags by scene ID
    pub fn get(&self, scene_id: u8) -> Option<&MmSceneFlags> {
        if (scene_id as usize) < MM_NUM_SCENES {
            Some(&self.scenes[scene_id as usize])
        } else {
            None
        }
    }

    /// Get mutable scene flags by scene ID
    pub fn get_mut(&mut self, scene_id: u8) -> Option<&mut MmSceneFlags> {
        if (scene_id as usize) < MM_NUM_SCENES {
            Some(&mut self.scenes[scene_id as usize])
        } else {
            None
        }
    }

    /// Check a location using scene flags
    pub fn checked(&self, location: &str) -> Option<bool> {
        match location {
            // Woodfall Temple Chests (Scene 0x1F)
            "Woodfall Temple Map Chest" => {
                Some(self.scenes[0x1F].chest_collected(WoodfallTempleChests::MAP.bits()))
            }
            "Woodfall Temple Compass Chest" => {
                Some(self.scenes[0x1F].chest_collected(WoodfallTempleChests::COMPASS.bits()))
            }
            "Woodfall Temple Boss Key Chest" => {
                Some(self.scenes[0x1F].chest_collected(WoodfallTempleChests::BOSS_KEY.bits()))
            }
            "Woodfall Temple Small Key Chest" => {
                Some(self.scenes[0x1F].chest_collected(WoodfallTempleChests::SMALL_KEY.bits()))
            }
            "Woodfall Temple Heros Bow Chest" => {
                Some(self.scenes[0x1F].chest_collected(WoodfallTempleChests::HEROS_BOW.bits()))
            }

            // Snowhead Temple Chests (Scene 0x22)
            "Snowhead Temple Map Chest" => {
                Some(self.scenes[0x22].chest_collected(SnowheadTempleChests::MAP.bits()))
            }
            "Snowhead Temple Compass Chest" => {
                Some(self.scenes[0x22].chest_collected(SnowheadTempleChests::COMPASS.bits()))
            }
            "Snowhead Temple Boss Key Chest" => {
                Some(self.scenes[0x22].chest_collected(SnowheadTempleChests::BOSS_KEY.bits()))
            }
            "Snowhead Temple Fire Arrow Chest" => {
                Some(self.scenes[0x22].chest_collected(SnowheadTempleChests::FIRE_ARROWS.bits()))
            }

            // Great Bay Temple Chests (Scene 0x1E)
            "Great Bay Temple Map Chest" => {
                Some(self.scenes[0x1E].chest_collected(GreatBayTempleChests::MAP.bits()))
            }
            "Great Bay Temple Compass Chest" => {
                Some(self.scenes[0x1E].chest_collected(GreatBayTempleChests::COMPASS.bits()))
            }
            "Great Bay Temple Boss Key Chest" => {
                Some(self.scenes[0x1E].chest_collected(GreatBayTempleChests::BOSS_KEY.bits()))
            }
            "Great Bay Temple Ice Arrow Chest" => {
                Some(self.scenes[0x1E].chest_collected(GreatBayTempleChests::ICE_ARROWS.bits()))
            }
            "Great Bay Temple Hookshot Chest" => {
                Some(self.scenes[0x1E].chest_collected(GreatBayTempleChests::HOOKSHOT.bits()))
            }

            // Stone Tower Temple Chests (Scene 0x18)
            "Stone Tower Temple Map Chest" => {
                Some(self.scenes[0x18].chest_collected(StoneTowerTempleChests::MAP.bits()))
            }
            "Stone Tower Temple Compass Chest" => {
                Some(self.scenes[0x18].chest_collected(StoneTowerTempleChests::COMPASS.bits()))
            }
            "Stone Tower Temple Boss Key Chest" => {
                Some(self.scenes[0x18].chest_collected(StoneTowerTempleChests::BOSS_KEY.bits()))
            }
            "Stone Tower Temple Light Arrow Chest" => {
                Some(self.scenes[0x18].chest_collected(StoneTowerTempleChests::LIGHT_ARROWS.bits()))
            }
            "Stone Tower Temple Giants Mask Chest" => {
                Some(self.scenes[0x18].chest_collected(StoneTowerTempleChests::GIANTS_MASK.bits()))
            }

            // Swamp Spider House (Scene 0x27)
            "Swamp Skulltula House Mask of Truth" => Some(
                self.scenes[0x27].collectible_obtained(SwampSpiderHouseCollectibles::REWARD.bits()),
            ),

            // Oceanside Spider House (Scene 0x28)
            "Oceanside Skulltula House Giants Wallet" => Some(
                self.scenes[0x28]
                    .collectible_obtained(OceansideSpiderHouseCollectibles::REWARD.bits()),
            ),

            // Clock Town Chests
            "East Clock Town Chest" => {
                Some(self.scenes[0x6E].chest_collected(EastClockTownChests::ROOFTOP.bits()))
            }
            "North Clock Town Tree Chest" => {
                Some(self.scenes[0x6D].chest_collected(NorthClockTownChests::TREE.bits()))
            }

            // Moon Trial Chests
            "Moon Deku Trial Heart Piece" => {
                Some(self.scenes[0x67].chest_collected(MoonDekuTrialChests::HEART_PIECE.bits()))
            }
            "Moon Goron Trial Heart Piece" => {
                Some(self.scenes[0x68].chest_collected(MoonGoronTrialChests::HEART_PIECE.bits()))
            }
            "Moon Zora Trial Heart Piece" => {
                Some(self.scenes[0x69].chest_collected(MoonZoraTrialChests::HEART_PIECE.bits()))
            }
            "Moon Link Trial Heart Piece" => {
                Some(self.scenes[0x66].chest_collected(MoonLinkTrialChests::HEART_PIECE.bits()))
            }

            // Great Fairy Rewards (collectibles)
            "Clock Town Great Fairy Reward" => Some(
                self.scenes[0x26]
                    .collectible_obtained(ClockTownGreatFairyCollectibles::REWARD.bits()),
            ),
            "Woodfall Great Fairy Reward" => Some(
                self.scenes[0x20]
                    .collectible_obtained(WoodfallGreatFairyCollectibles::REWARD.bits()),
            ),
            "Snowhead Great Fairy Reward" => Some(
                self.scenes[0x25]
                    .collectible_obtained(SnowheadGreatFairyCollectibles::REWARD.bits()),
            ),
            "Great Bay Great Fairy Reward" => Some(
                self.scenes[0x60]
                    .collectible_obtained(GreatBayGreatFairyCollectibles::REWARD.bits()),
            ),
            "Ikana Great Fairy Reward" => Some(
                self.scenes[0x15].collectible_obtained(IkanaGreatFairyCollectibles::REWARD.bits()),
            ),

            // Pirates Fortress
            "Pirates Fortress Hookshot Chest" => Some(
                self.scenes[0x2A].chest_collected(PiratesFortressInteriorChests::HOOKSHOT.bits()),
            ),

            // Beneath the Well
            "Beneath the Well Mirror Shield Chest" => {
                Some(self.scenes[0x1B].chest_collected(BeneathTheWellChests::MIRROR_SHIELD.bits()))
            }

            // Ancient Castle of Ikana
            "Ancient Castle of Ikana Powder Keg Chest" => Some(
                self.scenes[0x11].chest_collected(AncientCastleOfIkanaChests::POWDER_KEG.bits()),
            ),

            _ => None,
        }
    }
}

impl TryFrom<Vec<u8>> for MmAllSceneFlags {
    type Error = Vec<u8>;

    fn try_from(data: Vec<u8>) -> Result<Self, Self::Error> {
        if data.len() < MM_SCENE_FLAGS_SIZE {
            return Err(data);
        }
        let mut scenes = [MmSceneFlags::default(); MM_NUM_SCENES];
        for (i, scene) in scenes.iter_mut().enumerate() {
            let start = i * MM_SCENE_SIZE;
            let end = start + MM_SCENE_SIZE;
            *scene = MmSceneFlags::try_from(&data[start..end]).map_err(|()| data.clone())?;
        }
        Ok(MmAllSceneFlags { scenes })
    }
}

impl From<&MmAllSceneFlags> for Vec<u8> {
    fn from(flags: &MmAllSceneFlags) -> Vec<u8> {
        let mut buf = Vec::with_capacity(MM_SCENE_FLAGS_SIZE);
        for scene in &flags.scenes {
            buf.extend(Vec::<u8>::from(*scene));
        }
        buf
    }
}

// ============================================================================
// Chest Flag Definitions
// ============================================================================

bitflags! {
    /// Woodfall Temple chest flags (Scene 0x1F)
    #[derive(Default)]
    pub struct WoodfallTempleChests: u32 {
        const MAP = 0x0000_0001;
        const COMPASS = 0x0000_0002;
        const BOSS_KEY = 0x0000_0004;
        const SMALL_KEY = 0x0000_0008;
        const HEROS_BOW = 0x0000_0010;
        const STRAY_FAIRY_CHEST_1 = 0x0000_0020;
        const STRAY_FAIRY_CHEST_2 = 0x0000_0040;
        const STRAY_FAIRY_CHEST_3 = 0x0000_0080;
    }
}

bitflags! {
    /// Snowhead Temple chest flags (Scene 0x22)
    #[derive(Default)]
    pub struct SnowheadTempleChests: u32 {
        const MAP = 0x0000_0001;
        const COMPASS = 0x0000_0002;
        const BOSS_KEY = 0x0000_0004;
        const FIRE_ARROWS = 0x0000_0008;
        const SMALL_KEY_1 = 0x0000_0010;
        const SMALL_KEY_2 = 0x0000_0020;
        const SMALL_KEY_3 = 0x0000_0040;
        const STRAY_FAIRY_CHEST_1 = 0x0000_0080;
        const STRAY_FAIRY_CHEST_2 = 0x0000_0100;
    }
}

bitflags! {
    /// Great Bay Temple chest flags (Scene 0x1E)
    #[derive(Default)]
    pub struct GreatBayTempleChests: u32 {
        const MAP = 0x0000_0001;
        const COMPASS = 0x0000_0002;
        const BOSS_KEY = 0x0000_0004;
        const ICE_ARROWS = 0x0000_0008;
        const HOOKSHOT = 0x0000_0010;
        const SMALL_KEY = 0x0000_0020;
        const STRAY_FAIRY_CHEST_1 = 0x0000_0040;
        const STRAY_FAIRY_CHEST_2 = 0x0000_0080;
    }
}

bitflags! {
    /// Stone Tower Temple chest flags (Scene 0x18)
    #[derive(Default)]
    pub struct StoneTowerTempleChests: u32 {
        const MAP = 0x0000_0001;
        const COMPASS = 0x0000_0002;
        const BOSS_KEY = 0x0000_0004;
        const LIGHT_ARROWS = 0x0000_0008;
        const GIANTS_MASK = 0x0000_0010;
        const SMALL_KEY_1 = 0x0000_0020;
        const SMALL_KEY_2 = 0x0000_0040;
        const SMALL_KEY_3 = 0x0000_0080;
        const SMALL_KEY_4 = 0x0000_0100;
        const STRAY_FAIRY_CHEST_1 = 0x0000_0200;
    }
}

bitflags! {
    /// Stone Tower Temple Inverted chest flags (Scene 0x19)
    #[derive(Default)]
    pub struct StoneTowerTempleInvertedChests: u32 {
        const STRAY_FAIRY_CHEST_1 = 0x0000_0001;
        const STRAY_FAIRY_CHEST_2 = 0x0000_0002;
        const BOSS_ROOM_CHEST = 0x0000_0004;
    }
}

bitflags! {
    /// East Clock Town chest flags (Scene 0x6E)
    #[derive(Default)]
    pub struct EastClockTownChests: u32 {
        const ROOFTOP = 0x0000_0001;
    }
}

bitflags! {
    /// North Clock Town chest flags (Scene 0x6D)
    #[derive(Default)]
    pub struct NorthClockTownChests: u32 {
        const TREE = 0x0000_0001;
    }
}

bitflags! {
    /// Treasure Chest Shop chest flags (Scene 0x4C)
    #[derive(Default)]
    pub struct TreasureChestShopChests: u32 {
        const HEART_PIECE = 0x0000_0001;
    }
}

bitflags! {
    /// Moon Deku Trial chest flags (Scene 0x67)
    #[derive(Default)]
    pub struct MoonDekuTrialChests: u32 {
        const HEART_PIECE = 0x0000_0001;
    }
}

bitflags! {
    /// Moon Goron Trial chest flags (Scene 0x68)
    #[derive(Default)]
    pub struct MoonGoronTrialChests: u32 {
        const HEART_PIECE = 0x0000_0001;
    }
}

bitflags! {
    /// Moon Zora Trial chest flags (Scene 0x69)
    #[derive(Default)]
    pub struct MoonZoraTrialChests: u32 {
        const HEART_PIECE = 0x0000_0001;
    }
}

bitflags! {
    /// Moon Link Trial chest flags (Scene 0x66)
    #[derive(Default)]
    pub struct MoonLinkTrialChests: u32 {
        const HEART_PIECE = 0x0000_0001;
        const GOSSIP_STONE_CHEST = 0x0000_0002;
    }
}

bitflags! {
    /// Pirates Fortress Interior chest flags (Scene 0x2A)
    #[derive(Default)]
    pub struct PiratesFortressInteriorChests: u32 {
        const HOOKSHOT = 0x0000_0001;
        const SILVER_RUPEE_1 = 0x0000_0002;
        const SILVER_RUPEE_2 = 0x0000_0004;
        const SILVER_RUPEE_3 = 0x0000_0008;
    }
}

bitflags! {
    /// Beneath the Well chest flags (Scene 0x1B)
    #[derive(Default)]
    pub struct BeneathTheWellChests: u32 {
        const MIRROR_SHIELD = 0x0000_0001;
        const COMPASS = 0x0000_0002;
        const MAP = 0x0000_0004;
    }
}

bitflags! {
    /// Ancient Castle of Ikana chest flags (Scene 0x11)
    #[derive(Default)]
    pub struct AncientCastleOfIkanaChests: u32 {
        const POWDER_KEG = 0x0000_0001;
        const COMPASS = 0x0000_0002;
        const MAP = 0x0000_0004;
    }
}

bitflags! {
    /// Ikana Canyon Secret Shrine chest flags (Scene 0x13)
    #[derive(Default)]
    pub struct SecretShrineChests: u32 {
        const HEART_PIECE = 0x0000_0001;
        const LIGHT_ARROWS = 0x0000_0002;
    }
}

// ============================================================================
// Collectible Flag Definitions
// ============================================================================

bitflags! {
    /// Swamp Spider House collectible flags (Scene 0x27)
    #[derive(Default)]
    pub struct SwampSpiderHouseCollectibles: u32 {
        const REWARD = 0x0000_0001;
        const SKULLTULA_1 = 0x0000_0002;
        const SKULLTULA_2 = 0x0000_0004;
        const SKULLTULA_3 = 0x0000_0008;
        // ... more skulltulas up to 30
    }
}

bitflags! {
    /// Oceanside Spider House collectible flags (Scene 0x28)
    #[derive(Default)]
    pub struct OceansideSpiderHouseCollectibles: u32 {
        const REWARD = 0x0000_0001;
        const SKULLTULA_1 = 0x0000_0002;
        const SKULLTULA_2 = 0x0000_0004;
        const SKULLTULA_3 = 0x0000_0008;
        // ... more skulltulas up to 30
    }
}

bitflags! {
    /// Clock Town Great Fairy collectible flags (Scene 0x26)
    #[derive(Default)]
    pub struct ClockTownGreatFairyCollectibles: u32 {
        const REWARD = 0x0000_0001;
        const STRAY_FAIRY_COLLECTED = 0x0000_0002;
    }
}

bitflags! {
    /// Woodfall Great Fairy collectible flags (Scene 0x20)
    #[derive(Default)]
    pub struct WoodfallGreatFairyCollectibles: u32 {
        const REWARD = 0x0000_0001;
    }
}

bitflags! {
    /// Snowhead Great Fairy collectible flags (Scene 0x25)
    #[derive(Default)]
    pub struct SnowheadGreatFairyCollectibles: u32 {
        const REWARD = 0x0000_0001;
    }
}

bitflags! {
    /// Great Bay Great Fairy collectible flags (Scene 0x60)
    #[derive(Default)]
    pub struct GreatBayGreatFairyCollectibles: u32 {
        const REWARD = 0x0000_0001;
    }
}

bitflags! {
    /// Ikana Great Fairy collectible flags (Scene 0x15)
    #[derive(Default)]
    pub struct IkanaGreatFairyCollectibles: u32 {
        const REWARD = 0x0000_0001;
    }
}

bitflags! {
    /// Termina Field collectible flags (Scene 0x54)
    #[derive(Default)]
    pub struct TerminaFieldCollectibles: u32 {
        const BIO_BABA_GROTTO_HP = 0x0000_0001;
        const GOSSIP_STONES_HP = 0x0000_0002;
        const UNDERWATER_CHEST = 0x0000_0004;
        const GRASS_GROTTO_CHEST = 0x0000_0008;
        const BUSINESS_SCRUB = 0x0000_0010;
        const PEAHAT_GROTTO = 0x0000_0020;
    }
}

bitflags! {
    /// Southern Swamp collectible flags (Scene 0x55)
    #[derive(Default)]
    pub struct SouthernSwampCollectibles: u32 {
        const PICTOGRAPH_CONTEST = 0x0000_0001;
        const BOAT_ARCHERY = 0x0000_0002;
        const SWAMP_SHOOTING_GALLERY = 0x0000_0004;
    }
}

bitflags! {
    /// Mountain Village collectible flags (Scene 0x65)
    #[derive(Default)]
    pub struct MountainVillageCollectibles: u32 {
        const DON_GERO_HP = 0x0000_0001;
        const SPRING_WATER_HP = 0x0000_0002;
    }
}

bitflags! {
    /// Great Bay Coast collectible flags (Scene 0x37)
    #[derive(Default)]
    pub struct GreatBayCoastCollectibles: u32 {
        const MARINE_LAB_HP = 0x0000_0001;
        const FISHERMAN_HP = 0x0000_0002;
        const PIRATE_PHOTO = 0x0000_0004;
        const SEAHORSE = 0x0000_0008;
    }
}

bitflags! {
    /// Ikana Canyon collectible flags (Scene 0x46)
    #[derive(Default)]
    pub struct IkanaCanyonCollectibles: u32 {
        const PAMELAS_FATHER = 0x0000_0001;
        const COMPOSERS_HP = 0x0000_0002;
        const SECRET_SHRINE_HP = 0x0000_0004;
    }
}

bitflags! {
    /// Romani Ranch collectible flags (Scene 0x5F)
    #[derive(Default)]
    pub struct RomaniRanchCollectibles: u32 {
        const ROMANI_MASK = 0x0000_0001;
        const EPONAS_SONG = 0x0000_0002;
        const CREMIA_REWARD = 0x0000_0004;
        const DOG_RACE_HP = 0x0000_0008;
    }
}

// ============================================================================
// Flag Type Classification
// ============================================================================

/// Indicates whether a flag is permanent or resets with the cycle
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FlagPersistence {
    /// Flag persists across Song of Time resets
    Permanent,
    /// Flag resets when playing Song of Time
    CycleBased,
}

/// Information about a specific location's flag
#[derive(Debug, Clone)]
pub struct MmLocationFlag {
    pub scene_id: u8,
    pub flag_type: MmFlagType,
    pub flag_value: u32,
    pub persistence: FlagPersistence,
    pub location_name: &'static str,
}

/// Type of flag for a location
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MmFlagType {
    Chest,
    Collectible,
    Switch0,
    Switch1,
    ClearedRoom,
}

impl MmLocationFlag {
    /// Create a new permanent chest location
    pub const fn permanent_chest(
        scene_id: u8,
        flag_value: u32,
        location_name: &'static str,
    ) -> Self {
        Self {
            scene_id,
            flag_type: MmFlagType::Chest,
            flag_value,
            persistence: FlagPersistence::Permanent,
            location_name,
        }
    }

    /// Create a new permanent collectible location
    pub const fn permanent_collectible(
        scene_id: u8,
        flag_value: u32,
        location_name: &'static str,
    ) -> Self {
        Self {
            scene_id,
            flag_type: MmFlagType::Collectible,
            flag_value,
            persistence: FlagPersistence::Permanent,
            location_name,
        }
    }

    /// Create a new cycle-based chest location
    pub const fn cycle_chest(scene_id: u8, flag_value: u32, location_name: &'static str) -> Self {
        Self {
            scene_id,
            flag_type: MmFlagType::Chest,
            flag_value,
            persistence: FlagPersistence::CycleBased,
            location_name,
        }
    }

    /// Create a new cycle-based collectible location
    pub const fn cycle_collectible(
        scene_id: u8,
        flag_value: u32,
        location_name: &'static str,
    ) -> Self {
        Self {
            scene_id,
            flag_type: MmFlagType::Collectible,
            flag_value,
            persistence: FlagPersistence::CycleBased,
            location_name,
        }
    }
}

// ============================================================================
// Location Registry
// ============================================================================

/// Registry of all trackable MM locations with their flag information
pub static MM_LOCATIONS: &[MmLocationFlag] = &[
    // Woodfall Temple
    MmLocationFlag::permanent_chest(0x1F, 0x0000_0001, "Woodfall Temple Map Chest"),
    MmLocationFlag::permanent_chest(0x1F, 0x0000_0002, "Woodfall Temple Compass Chest"),
    MmLocationFlag::permanent_chest(0x1F, 0x0000_0004, "Woodfall Temple Boss Key Chest"),
    MmLocationFlag::permanent_chest(0x1F, 0x0000_0008, "Woodfall Temple Small Key Chest"),
    MmLocationFlag::permanent_chest(0x1F, 0x0000_0010, "Woodfall Temple Heros Bow Chest"),
    // Snowhead Temple
    MmLocationFlag::permanent_chest(0x22, 0x0000_0001, "Snowhead Temple Map Chest"),
    MmLocationFlag::permanent_chest(0x22, 0x0000_0002, "Snowhead Temple Compass Chest"),
    MmLocationFlag::permanent_chest(0x22, 0x0000_0004, "Snowhead Temple Boss Key Chest"),
    MmLocationFlag::permanent_chest(0x22, 0x0000_0008, "Snowhead Temple Fire Arrow Chest"),
    // Great Bay Temple
    MmLocationFlag::permanent_chest(0x1E, 0x0000_0001, "Great Bay Temple Map Chest"),
    MmLocationFlag::permanent_chest(0x1E, 0x0000_0002, "Great Bay Temple Compass Chest"),
    MmLocationFlag::permanent_chest(0x1E, 0x0000_0004, "Great Bay Temple Boss Key Chest"),
    MmLocationFlag::permanent_chest(0x1E, 0x0000_0008, "Great Bay Temple Ice Arrow Chest"),
    MmLocationFlag::permanent_chest(0x1E, 0x0000_0010, "Great Bay Temple Hookshot Chest"),
    // Stone Tower Temple
    MmLocationFlag::permanent_chest(0x18, 0x0000_0001, "Stone Tower Temple Map Chest"),
    MmLocationFlag::permanent_chest(0x18, 0x0000_0002, "Stone Tower Temple Compass Chest"),
    MmLocationFlag::permanent_chest(0x18, 0x0000_0004, "Stone Tower Temple Boss Key Chest"),
    MmLocationFlag::permanent_chest(0x18, 0x0000_0008, "Stone Tower Temple Light Arrow Chest"),
    MmLocationFlag::permanent_chest(0x18, 0x0000_0010, "Stone Tower Temple Giants Mask Chest"),
    // Clock Town
    MmLocationFlag::permanent_chest(0x6E, 0x0000_0001, "East Clock Town Chest"),
    MmLocationFlag::permanent_chest(0x6D, 0x0000_0001, "North Clock Town Tree Chest"),
    // Moon Trials
    MmLocationFlag::permanent_chest(0x67, 0x0000_0001, "Moon Deku Trial Heart Piece"),
    MmLocationFlag::permanent_chest(0x68, 0x0000_0001, "Moon Goron Trial Heart Piece"),
    MmLocationFlag::permanent_chest(0x69, 0x0000_0001, "Moon Zora Trial Heart Piece"),
    MmLocationFlag::permanent_chest(0x66, 0x0000_0001, "Moon Link Trial Heart Piece"),
    // Great Fairy Rewards
    MmLocationFlag::permanent_collectible(0x26, 0x0000_0001, "Clock Town Great Fairy Reward"),
    MmLocationFlag::permanent_collectible(0x20, 0x0000_0001, "Woodfall Great Fairy Reward"),
    MmLocationFlag::permanent_collectible(0x25, 0x0000_0001, "Snowhead Great Fairy Reward"),
    MmLocationFlag::permanent_collectible(0x60, 0x0000_0001, "Great Bay Great Fairy Reward"),
    MmLocationFlag::permanent_collectible(0x15, 0x0000_0001, "Ikana Great Fairy Reward"),
    // Spider Houses
    MmLocationFlag::permanent_collectible(0x27, 0x0000_0001, "Swamp Skulltula House Mask of Truth"),
    MmLocationFlag::permanent_collectible(
        0x28,
        0x0000_0001,
        "Oceanside Skulltula House Giants Wallet",
    ),
    // Mini Dungeons
    MmLocationFlag::permanent_chest(0x1B, 0x0000_0001, "Beneath the Well Mirror Shield Chest"),
    MmLocationFlag::permanent_chest(
        0x11,
        0x0000_0001,
        "Ancient Castle of Ikana Powder Keg Chest",
    ),
    MmLocationFlag::permanent_chest(0x2A, 0x0000_0001, "Pirates Fortress Hookshot Chest"),
];

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_scene_from_id() {
        // Test valid scene IDs
        assert_eq!(
            MmScene::from_id(0x1F).map(|s| s.name()),
            Some("Woodfall Temple")
        );
        assert_eq!(
            MmScene::from_id(0x22).map(|s| s.name()),
            Some("Snowhead Temple")
        );
        assert_eq!(
            MmScene::from_id(0x1E).map(|s| s.name()),
            Some("Great Bay Temple")
        );
        assert_eq!(
            MmScene::from_id(0x18).map(|s| s.name()),
            Some("Stone Tower Temple")
        );
        assert_eq!(
            MmScene::from_id(0x6C).map(|s| s.name()),
            Some("South Clock Town")
        );
        assert_eq!(
            MmScene::from_id(0x27).map(|s| s.name()),
            Some("Swamp Spider House")
        );

        // Test invalid scene ID
        assert!(MmScene::from_id(0xFF).is_none());
    }

    #[test]
    fn test_scene_flags_default() {
        let flags = MmSceneFlags::default();
        assert_eq!(flags.chest, 0);
        assert_eq!(flags.switch0, 0);
        assert_eq!(flags.switch1, 0);
        assert_eq!(flags.cleared_room, 0);
        assert_eq!(flags.collectible, 0);
        assert_eq!(flags.cleared_floors, 0);
        assert_eq!(flags.rooms, 0);
    }

    #[test]
    fn test_scene_flags_from_bytes() {
        let mut data = [0u8; MM_SCENE_SIZE];
        // Set chest flags
        BigEndian::write_u32(&mut data[0x00..0x04], 0x0000_001F);
        // Set switch0 flags
        BigEndian::write_u32(&mut data[0x04..0x08], 0x0000_00FF);
        // Set collectible flags
        BigEndian::write_u32(&mut data[0x10..0x14], 0x0000_0007);

        let flags = MmSceneFlags::try_from(&data[..]).unwrap();
        assert_eq!(flags.chest, 0x0000_001F);
        assert_eq!(flags.switch0, 0x0000_00FF);
        assert_eq!(flags.collectible, 0x0000_0007);
    }

    #[test]
    fn test_scene_flags_to_bytes() {
        let flags = MmSceneFlags {
            chest: 0x0000_001F,
            switch0: 0x0000_00FF,
            switch1: 0x0000_0000,
            cleared_room: 0x0000_0000,
            collectible: 0x0000_0007,
            cleared_floors: 0x0000_0000,
            rooms: 0x0000_0001,
        };

        let data: Vec<u8> = flags.into();
        assert_eq!(data.len(), MM_SCENE_SIZE);
        assert_eq!(BigEndian::read_u32(&data[0x00..0x04]), 0x0000_001F);
        assert_eq!(BigEndian::read_u32(&data[0x04..0x08]), 0x0000_00FF);
        assert_eq!(BigEndian::read_u32(&data[0x10..0x14]), 0x0000_0007);
        assert_eq!(BigEndian::read_u32(&data[0x18..0x1c]), 0x0000_0001);
    }

    #[test]
    fn test_chest_collected() {
        let flags = MmSceneFlags {
            chest: WoodfallTempleChests::MAP.bits() | WoodfallTempleChests::COMPASS.bits(),
            ..Default::default()
        };

        assert!(flags.chest_collected(WoodfallTempleChests::MAP.bits()));
        assert!(flags.chest_collected(WoodfallTempleChests::COMPASS.bits()));
        assert!(!flags.chest_collected(WoodfallTempleChests::BOSS_KEY.bits()));
    }

    #[test]
    fn test_collectible_obtained() {
        let flags = MmSceneFlags {
            collectible: SwampSpiderHouseCollectibles::REWARD.bits(),
            ..Default::default()
        };

        assert!(flags.collectible_obtained(SwampSpiderHouseCollectibles::REWARD.bits()));
        assert!(!flags.collectible_obtained(SwampSpiderHouseCollectibles::SKULLTULA_1.bits()));
    }

    #[test]
    fn test_all_scene_flags_default() {
        let all_flags = MmAllSceneFlags::default();
        for scene_id in 0..MM_NUM_SCENES as u8 {
            let scene = all_flags.get(scene_id).unwrap();
            assert_eq!(*scene, MmSceneFlags::default());
        }
    }

    #[test]
    fn test_all_scene_flags_get_mut() {
        let mut all_flags = MmAllSceneFlags::default();

        // Modify Woodfall Temple flags
        if let Some(woodfall) = all_flags.get_mut(0x1F) {
            woodfall.chest = WoodfallTempleChests::MAP.bits();
        }

        // Verify modification
        assert_eq!(
            all_flags.get(0x1F).unwrap().chest,
            WoodfallTempleChests::MAP.bits()
        );
    }

    #[test]
    fn test_checked_woodfall_temple() {
        let mut all_flags = MmAllSceneFlags::default();

        // Initially unchecked
        assert_eq!(all_flags.checked("Woodfall Temple Map Chest"), Some(false));
        assert_eq!(
            all_flags.checked("Woodfall Temple Compass Chest"),
            Some(false)
        );

        // Set some flags
        if let Some(woodfall) = all_flags.get_mut(0x1F) {
            woodfall.chest =
                WoodfallTempleChests::MAP.bits() | WoodfallTempleChests::BOSS_KEY.bits();
        }

        // Now checked
        assert_eq!(all_flags.checked("Woodfall Temple Map Chest"), Some(true));
        assert_eq!(
            all_flags.checked("Woodfall Temple Compass Chest"),
            Some(false)
        );
        assert_eq!(
            all_flags.checked("Woodfall Temple Boss Key Chest"),
            Some(true)
        );
    }

    #[test]
    fn test_checked_unknown_location() {
        let all_flags = MmAllSceneFlags::default();
        assert_eq!(
            all_flags.checked("Unknown Location That Does Not Exist"),
            None
        );
    }

    #[test]
    fn test_cycle_flags_default() {
        let flags = MmCycleFlags::default();
        assert_eq!(flags.chest, 0);
        assert_eq!(flags.switch0, 0);
        assert_eq!(flags.switch1, 0);
        assert_eq!(flags.cleared_room, 0);
        assert_eq!(flags.collectible, 0);
    }

    #[test]
    fn test_cycle_flags_from_bytes() {
        let mut data = [0u8; 20];
        BigEndian::write_u32(&mut data[0x00..0x04], 0x0000_000F);
        BigEndian::write_u32(&mut data[0x10..0x14], 0x0000_0003);

        let flags = MmCycleFlags::try_from(&data[..]).unwrap();
        assert_eq!(flags.chest, 0x0000_000F);
        assert_eq!(flags.collectible, 0x0000_0003);
    }

    #[test]
    fn test_cycle_flags_to_bytes() {
        let flags = MmCycleFlags {
            chest: 0x0000_000F,
            switch0: 0,
            switch1: 0,
            cleared_room: 0,
            collectible: 0x0000_0003,
        };

        let data: Vec<u8> = flags.into();
        assert_eq!(data.len(), 20);
        assert_eq!(BigEndian::read_u32(&data[0x00..0x04]), 0x0000_000F);
        assert_eq!(BigEndian::read_u32(&data[0x10..0x14]), 0x0000_0003);
    }

    #[test]
    fn test_flag_persistence() {
        // Test permanent locations
        let permanent_loc = MmLocationFlag::permanent_chest(0x1F, 0x01, "Test Permanent");
        assert_eq!(permanent_loc.persistence, FlagPersistence::Permanent);

        // Test cycle-based locations
        let cycle_loc = MmLocationFlag::cycle_chest(0x1F, 0x01, "Test Cycle");
        assert_eq!(cycle_loc.persistence, FlagPersistence::CycleBased);
    }

    #[test]
    fn test_mm_locations_registry() {
        // Verify the static registry contains expected locations
        let woodfall_map = MM_LOCATIONS
            .iter()
            .find(|loc| loc.location_name == "Woodfall Temple Map Chest");

        assert!(woodfall_map.is_some());
        let loc = woodfall_map.unwrap();
        assert_eq!(loc.scene_id, 0x1F);
        assert_eq!(loc.flag_type, MmFlagType::Chest);
        assert_eq!(loc.persistence, FlagPersistence::Permanent);
    }

    #[test]
    fn test_all_scene_flags_roundtrip() {
        let mut original = MmAllSceneFlags::default();

        // Set some flags
        if let Some(woodfall) = original.get_mut(0x1F) {
            woodfall.chest = 0x0000_001F;
            woodfall.collectible = 0x0000_0007;
        }
        if let Some(snowhead) = original.get_mut(0x22) {
            snowhead.chest = 0x0000_000F;
        }

        // Convert to bytes and back
        let bytes: Vec<u8> = (&original).into();
        assert_eq!(bytes.len(), MM_SCENE_FLAGS_SIZE);

        let restored = MmAllSceneFlags::try_from(bytes).unwrap();
        assert_eq!(original, restored);
    }

    #[test]
    fn test_great_fairy_locations() {
        let mut all_flags = MmAllSceneFlags::default();

        // Initially unchecked
        assert_eq!(
            all_flags.checked("Clock Town Great Fairy Reward"),
            Some(false)
        );
        assert_eq!(
            all_flags.checked("Woodfall Great Fairy Reward"),
            Some(false)
        );

        // Set Clock Town Great Fairy as collected
        if let Some(scene) = all_flags.get_mut(0x26) {
            scene.collectible = ClockTownGreatFairyCollectibles::REWARD.bits();
        }

        // Now checked
        assert_eq!(
            all_flags.checked("Clock Town Great Fairy Reward"),
            Some(true)
        );
        assert_eq!(
            all_flags.checked("Woodfall Great Fairy Reward"),
            Some(false)
        );
    }

    #[test]
    fn test_spider_house_locations() {
        let mut all_flags = MmAllSceneFlags::default();

        // Set Swamp Spider House reward
        if let Some(scene) = all_flags.get_mut(0x27) {
            scene.collectible = SwampSpiderHouseCollectibles::REWARD.bits();
        }

        assert_eq!(
            all_flags.checked("Swamp Skulltula House Mask of Truth"),
            Some(true)
        );
        assert_eq!(
            all_flags.checked("Oceanside Skulltula House Giants Wallet"),
            Some(false)
        );
    }

    #[test]
    fn test_moon_trial_locations() {
        let mut all_flags = MmAllSceneFlags::default();

        // Set Deku Trial heart piece
        if let Some(scene) = all_flags.get_mut(0x67) {
            scene.chest = MoonDekuTrialChests::HEART_PIECE.bits();
        }

        assert_eq!(all_flags.checked("Moon Deku Trial Heart Piece"), Some(true));
        assert_eq!(
            all_flags.checked("Moon Goron Trial Heart Piece"),
            Some(false)
        );
        assert_eq!(
            all_flags.checked("Moon Zora Trial Heart Piece"),
            Some(false)
        );
        assert_eq!(
            all_flags.checked("Moon Link Trial Heart Piece"),
            Some(false)
        );
    }
}
