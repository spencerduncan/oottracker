//! Game detection for OoT, MM, and OoTMM combined ROMs.
//!
//! This module provides functionality to detect which game type is running
//! (standalone OoT, standalone MM, or OoTMM combo) and which game is currently
//! active in an OoTMM combo ROM.
//!
//! # OoTMM Architecture
//!
//! OoTMM creates a single combined ROM that merges both games:
//! - MM is placed at 0x2000000 offset within the ROM
//! - Players switch between games at specific locations (Happy Mask Shop <-> Clock Tower)
//! - The game auto-saves when switching between OoT and MM
//! - A single save file is shared between both games
//!
//! # Detection Strategy
//!
//! Game detection uses multiple strategies:
//! 1. **Combo context addresses**: OoTMM has specific memory addresses that indicate
//!    which game is active (OoT: 0x80006584, MM: 0x80098280)
//! 2. **Scene ID ranges**: OoT uses scene IDs 0x00-0x65, MM uses 0x00-0x70+ but with
//!    different semantic meanings
//! 3. **ROM signature detection**: The ROM header can identify game type

use {
    byteorder::{BigEndian, ByteOrder as _},
    serde::{Deserialize, Serialize},
    std::fmt,
};

// ============================================================================
// Constants
// ============================================================================

/// OoTMM combo context address for OoT game mode detection
/// When this address contains a non-zero value, OoT is the active game
pub const OOTMM_OOT_CONTEXT_ADDR: u32 = 0x80006584;

/// OoTMM combo context address for MM game mode detection
/// When this address contains a non-zero value, MM is the active game
pub const OOTMM_MM_CONTEXT_ADDR: u32 = 0x80098280;

/// Maximum scene ID for OoT (inclusive)
/// OoT scenes range from 0x00 to 0x65
pub const OOT_MAX_SCENE_ID: u8 = 0x65;

/// Maximum scene ID for MM (inclusive)
/// MM scenes range from 0x00 to approximately 0x70
pub const MM_MAX_SCENE_ID: u8 = 0x70;

/// N64 RDRAM base address offset
/// Memory addresses starting with 0x80 are RDRAM addresses
pub const RDRAM_BASE: u32 = 0x80000000;

/// RAM size (8 MB)
pub const RAM_SIZE: usize = 0x80_0000;

// ============================================================================
// Scene ID Constants
// ============================================================================

/// OoT-specific scene IDs that uniquely identify OoT
pub mod oot_scenes {
    /// Deku Tree (first dungeon)
    pub const DEKU_TREE: u8 = 0x00;
    /// Dodongo's Cavern
    pub const DODONGOS_CAVERN: u8 = 0x01;
    /// Jabu Jabu's Belly
    pub const JABU_JABUS_BELLY: u8 = 0x02;
    /// Forest Temple
    pub const FOREST_TEMPLE: u8 = 0x03;
    /// Fire Temple
    pub const FIRE_TEMPLE: u8 = 0x04;
    /// Water Temple
    pub const WATER_TEMPLE: u8 = 0x05;
    /// Spirit Temple
    pub const SPIRIT_TEMPLE: u8 = 0x06;
    /// Shadow Temple
    pub const SHADOW_TEMPLE: u8 = 0x07;
    /// Bottom of the Well
    pub const BOTTOM_OF_THE_WELL: u8 = 0x08;
    /// Ice Cavern
    pub const ICE_CAVERN: u8 = 0x09;
    /// Ganon's Castle Tower
    pub const GANONS_CASTLE_TOWER: u8 = 0x0A;
    /// Gerudo Training Ground
    pub const GERUDO_TRAINING_GROUND: u8 = 0x0B;
    /// Thieves' Hideout (Gerudo Fortress interior)
    pub const THIEVES_HIDEOUT: u8 = 0x0C;
    /// Ganon's Castle
    pub const GANONS_CASTLE: u8 = 0x0D;
    /// Hyrule Field
    pub const HYRULE_FIELD: u8 = 0x51;
    /// Kakariko Village
    pub const KAKARIKO_VILLAGE: u8 = 0x52;
    /// Graveyard
    pub const GRAVEYARD: u8 = 0x53;
    /// Zora's River
    pub const ZORAS_RIVER: u8 = 0x54;
    /// Kokiri Forest
    pub const KOKIRI_FOREST: u8 = 0x55;
    /// Lake Hylia
    pub const LAKE_HYLIA: u8 = 0x57;
    /// Zora's Domain
    pub const ZORAS_DOMAIN: u8 = 0x58;
    /// Zora's Fountain
    pub const ZORAS_FOUNTAIN: u8 = 0x59;
    /// Gerudo Valley
    pub const GERUDO_VALLEY: u8 = 0x5A;
    /// Lost Woods
    pub const LOST_WOODS: u8 = 0x5B;
    /// Desert Colossus
    pub const DESERT_COLOSSUS: u8 = 0x5C;
    /// Gerudo Fortress
    pub const GERUDO_FORTRESS: u8 = 0x5D;
    /// Haunted Wasteland
    pub const HAUNTED_WASTELAND: u8 = 0x5E;
    /// Death Mountain Trail
    pub const DEATH_MOUNTAIN: u8 = 0x60;
    /// Death Mountain Crater
    pub const DEATH_MOUNTAIN_CRATER: u8 = 0x61;
    /// Goron City
    pub const GORON_CITY: u8 = 0x62;
    /// Temple of Time (exterior)
    pub const TEMPLE_OF_TIME_EXTERIOR: u8 = 0x43;
    /// Happy Mask Shop (OoTMM transition point to MM)
    pub const HAPPY_MASK_SHOP: u8 = 0x2D;
}

/// MM-specific scene IDs
pub mod mm_scenes {
    /// Mayor's Residence
    pub const MAYORS_RESIDENCE: u8 = 0x00;
    /// Majora's Lair
    pub const MAJORAS_LAIR: u8 = 0x01;
    /// Beneath the Graveyard (Dampe's House)
    pub const BENEATH_GRAVEYARD: u8 = 0x02;
    /// Woodfall Temple
    pub const WOODFALL_TEMPLE: u8 = 0x07;
    /// Snowhead Temple
    pub const SNOWHEAD_TEMPLE: u8 = 0x1B;
    /// Great Bay Temple
    pub const GREAT_BAY_TEMPLE: u8 = 0x37;
    /// Stone Tower Temple
    pub const STONE_TOWER_TEMPLE: u8 = 0x12;
    /// Stone Tower Temple (inverted)
    pub const STONE_TOWER_TEMPLE_INVERTED: u8 = 0x13;
    /// Clock Town South
    pub const CLOCK_TOWN_SOUTH: u8 = 0x6E;
    /// Clock Town North
    pub const CLOCK_TOWN_NORTH: u8 = 0x6F;
    /// Clock Town East
    pub const CLOCK_TOWN_EAST: u8 = 0x70;
    /// Clock Town West
    pub const CLOCK_TOWN_WEST: u8 = 0x71;
    /// Clock Tower (OoTMM transition point to OoT)
    pub const CLOCK_TOWER: u8 = 0x6C;
    /// Termina Field
    pub const TERMINA_FIELD: u8 = 0x54;
    /// Romani Ranch
    pub const ROMANI_RANCH: u8 = 0x35;
    /// Southern Swamp
    pub const SOUTHERN_SWAMP: u8 = 0x55;
    /// Mountain Village
    pub const MOUNTAIN_VILLAGE: u8 = 0x5A;
    /// Great Bay Coast
    pub const GREAT_BAY_COAST: u8 = 0x57;
    /// Ikana Canyon
    pub const IKANA_CANYON: u8 = 0x5B;
}

// ============================================================================
// Enums
// ============================================================================

/// The type of game/ROM being tracked.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum GameType {
    /// Standalone Ocarina of Time ROM
    StandaloneOoT,
    /// Standalone Majora's Mask ROM
    StandaloneMM,
    /// OoTMM combined ROM (both games in one)
    OoTMMCombo,
}

impl GameType {
    /// Returns true if this is a standalone game (not OoTMM combo)
    pub fn is_standalone(&self) -> bool {
        matches!(self, Self::StandaloneOoT | Self::StandaloneMM)
    }

    /// Returns true if this is the OoTMM combo ROM
    pub fn is_combo(&self) -> bool {
        matches!(self, Self::OoTMMCombo)
    }

    /// Returns true if this game type includes OoT gameplay
    pub fn has_oot(&self) -> bool {
        matches!(self, Self::StandaloneOoT | Self::OoTMMCombo)
    }

    /// Returns true if this game type includes MM gameplay
    pub fn has_mm(&self) -> bool {
        matches!(self, Self::StandaloneMM | Self::OoTMMCombo)
    }
}

impl fmt::Display for GameType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::StandaloneOoT => write!(f, "Ocarina of Time"),
            Self::StandaloneMM => write!(f, "Majora's Mask"),
            Self::OoTMMCombo => write!(f, "OoTMM Combo"),
        }
    }
}

impl Default for GameType {
    fn default() -> Self {
        Self::StandaloneOoT
    }
}

/// The currently active game in an OoTMM combo ROM.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ActiveGame {
    /// Ocarina of Time is currently active
    OcarinaOfTime,
    /// Majora's Mask is currently active
    MajorasMask,
}

impl ActiveGame {
    /// Returns true if OoT is currently active
    pub fn is_oot(&self) -> bool {
        matches!(self, Self::OcarinaOfTime)
    }

    /// Returns true if MM is currently active
    pub fn is_mm(&self) -> bool {
        matches!(self, Self::MajorasMask)
    }
}

impl fmt::Display for ActiveGame {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::OcarinaOfTime => write!(f, "OoT"),
            Self::MajorasMask => write!(f, "MM"),
        }
    }
}

impl Default for ActiveGame {
    fn default() -> Self {
        Self::OcarinaOfTime
    }
}

/// Game transition state for OoTMM.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum TransitionState {
    /// No transition in progress, game is stable
    Stable,
    /// Transitioning from OoT to MM
    OotToMm,
    /// Transitioning from MM to OoT
    MmToOot,
}

impl TransitionState {
    /// Returns true if a game transition is in progress
    pub fn is_transitioning(&self) -> bool {
        !matches!(self, Self::Stable)
    }

    /// Returns the source game of the transition, if any
    pub fn source_game(&self) -> Option<ActiveGame> {
        match self {
            Self::Stable => None,
            Self::OotToMm => Some(ActiveGame::OcarinaOfTime),
            Self::MmToOot => Some(ActiveGame::MajorasMask),
        }
    }

    /// Returns the destination game of the transition, if any
    pub fn destination_game(&self) -> Option<ActiveGame> {
        match self {
            Self::Stable => None,
            Self::OotToMm => Some(ActiveGame::MajorasMask),
            Self::MmToOot => Some(ActiveGame::OcarinaOfTime),
        }
    }
}

impl Default for TransitionState {
    fn default() -> Self {
        Self::Stable
    }
}

// ============================================================================
// Detection Result
// ============================================================================

/// Result of game detection from memory.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct GameDetectionResult {
    /// The detected game type
    pub game_type: GameType,
    /// The currently active game (only meaningful for OoTMMCombo)
    pub active_game: ActiveGame,
    /// Current transition state
    pub transition_state: TransitionState,
    /// Current scene ID
    pub scene_id: u8,
    /// Confidence level of the detection (0.0 - 1.0)
    pub confidence: f32,
}

impl GameDetectionResult {
    /// Create a new detection result for standalone OoT
    pub fn standalone_oot(scene_id: u8) -> Self {
        Self {
            game_type: GameType::StandaloneOoT,
            active_game: ActiveGame::OcarinaOfTime,
            transition_state: TransitionState::Stable,
            scene_id,
            confidence: 1.0,
        }
    }

    /// Create a new detection result for standalone MM
    pub fn standalone_mm(scene_id: u8) -> Self {
        Self {
            game_type: GameType::StandaloneMM,
            active_game: ActiveGame::MajorasMask,
            transition_state: TransitionState::Stable,
            scene_id,
            confidence: 1.0,
        }
    }

    /// Create a new detection result for OoTMM combo
    pub fn combo(active_game: ActiveGame, scene_id: u8, transition_state: TransitionState) -> Self {
        Self {
            game_type: GameType::OoTMMCombo,
            active_game,
            transition_state,
            scene_id,
            confidence: 1.0,
        }
    }

    /// Create a detection result with a specific confidence level
    pub fn with_confidence(mut self, confidence: f32) -> Self {
        self.confidence = confidence.clamp(0.0, 1.0);
        self
    }

    /// Returns true if the player is currently in OoT
    pub fn is_in_oot(&self) -> bool {
        self.active_game.is_oot()
    }

    /// Returns true if the player is currently in MM
    pub fn is_in_mm(&self) -> bool {
        self.active_game.is_mm()
    }
}

impl Default for GameDetectionResult {
    fn default() -> Self {
        Self::standalone_oot(0)
    }
}

// ============================================================================
// Game Detector
// ============================================================================

/// Handles game detection and transition tracking for OoT, MM, and OoTMM.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameDetector {
    /// The known game type (set during initialization or ROM detection)
    game_type: GameType,
    /// The last detected active game
    last_active_game: ActiveGame,
    /// The last known scene ID
    last_scene_id: u8,
    /// Current transition state
    transition_state: TransitionState,
    /// Number of frames since last game change
    frames_since_change: u32,
}

impl GameDetector {
    /// Create a new game detector with unknown game type
    pub fn new() -> Self {
        Self {
            game_type: GameType::StandaloneOoT,
            last_active_game: ActiveGame::OcarinaOfTime,
            last_scene_id: 0,
            transition_state: TransitionState::Stable,
            frames_since_change: 0,
        }
    }

    /// Create a new game detector with a known game type
    pub fn with_game_type(game_type: GameType) -> Self {
        let active_game = match game_type {
            GameType::StandaloneOoT | GameType::OoTMMCombo => ActiveGame::OcarinaOfTime,
            GameType::StandaloneMM => ActiveGame::MajorasMask,
        };
        Self {
            game_type,
            last_active_game: active_game,
            last_scene_id: 0,
            transition_state: TransitionState::Stable,
            frames_since_change: 0,
        }
    }

    /// Set the game type
    pub fn set_game_type(&mut self, game_type: GameType) {
        self.game_type = game_type;
        if game_type == GameType::StandaloneMM {
            self.last_active_game = ActiveGame::MajorasMask;
        }
    }

    /// Get the current game type
    pub fn game_type(&self) -> GameType {
        self.game_type
    }

    /// Get the last detected active game
    pub fn active_game(&self) -> ActiveGame {
        self.last_active_game
    }

    /// Get the current transition state
    pub fn transition_state(&self) -> TransitionState {
        self.transition_state
    }

    /// Detect the active game from RAM data.
    ///
    /// For OoTMM combo ROMs, this checks the combo context addresses.
    /// For standalone games, it returns the known game type.
    pub fn detect_from_ram(&mut self, ram: &[u8]) -> GameDetectionResult {
        if ram.len() < RAM_SIZE {
            return GameDetectionResult::default().with_confidence(0.0);
        }

        match self.game_type {
            GameType::StandaloneOoT => self.detect_standalone_oot(ram),
            GameType::StandaloneMM => self.detect_standalone_mm(ram),
            GameType::OoTMMCombo => self.detect_combo(ram),
        }
    }

    /// Detect active game for standalone OoT
    fn detect_standalone_oot(&mut self, ram: &[u8]) -> GameDetectionResult {
        // Read scene ID from OoT RAM offset
        let scene_id = ram.get(0x1c8545).copied().unwrap_or(0);
        self.last_scene_id = scene_id;
        self.frames_since_change += 1;

        GameDetectionResult::standalone_oot(scene_id)
    }

    /// Detect active game for standalone MM
    fn detect_standalone_mm(&mut self, ram: &[u8]) -> GameDetectionResult {
        // Read scene ID from MM RAM offset (different from OoT)
        // MM scene ID location varies but is typically around 0x1EF674 in the save context
        let scene_id = ram.get(0x1ef674).copied().unwrap_or(0);
        self.last_scene_id = scene_id;
        self.frames_since_change += 1;

        GameDetectionResult::standalone_mm(scene_id)
    }

    /// Detect active game for OoTMM combo ROM
    fn detect_combo(&mut self, ram: &[u8]) -> GameDetectionResult {
        // Convert RDRAM addresses to RAM buffer offsets
        let oot_context_offset = (OOTMM_OOT_CONTEXT_ADDR - RDRAM_BASE) as usize;
        let mm_context_offset = (OOTMM_MM_CONTEXT_ADDR - RDRAM_BASE) as usize;

        // Read the combo context values
        let oot_context_value = if oot_context_offset + 4 <= ram.len() {
            BigEndian::read_u32(&ram[oot_context_offset..oot_context_offset + 4])
        } else {
            0
        };

        let mm_context_value = if mm_context_offset + 4 <= ram.len() {
            BigEndian::read_u32(&ram[mm_context_offset..mm_context_offset + 4])
        } else {
            0
        };

        // Determine active game based on context values
        let new_active_game = if oot_context_value != 0 && mm_context_value == 0 {
            ActiveGame::OcarinaOfTime
        } else if mm_context_value != 0 && oot_context_value == 0 {
            ActiveGame::MajorasMask
        } else {
            // Fallback to scene-based detection
            self.detect_active_game_by_scene(ram)
        };

        // Detect transitions
        let transition_state = self.detect_transition(new_active_game);

        // Update state
        let old_active_game = self.last_active_game;
        self.last_active_game = new_active_game;
        self.transition_state = transition_state;

        if new_active_game != old_active_game {
            self.frames_since_change = 0;
        } else {
            self.frames_since_change += 1;
        }

        // Read scene ID based on active game
        let scene_id = match new_active_game {
            ActiveGame::OcarinaOfTime => ram.get(0x1c8545).copied().unwrap_or(0),
            ActiveGame::MajorasMask => ram.get(0x1ef674).copied().unwrap_or(0),
        };
        self.last_scene_id = scene_id;

        let confidence = if oot_context_value != 0 || mm_context_value != 0 {
            1.0
        } else {
            0.7 // Lower confidence for scene-based detection
        };

        GameDetectionResult::combo(new_active_game, scene_id, transition_state)
            .with_confidence(confidence)
    }

    /// Detect active game using scene ID ranges (fallback method)
    fn detect_active_game_by_scene(&self, ram: &[u8]) -> ActiveGame {
        // Try to read scene ID from OoT location
        let oot_scene_id = ram.get(0x1c8545).copied().unwrap_or(0xFF);

        // Check if we're at OoTMM transition scenes
        if oot_scene_id == oot_scenes::HAPPY_MASK_SHOP {
            // At Happy Mask Shop, we might be transitioning to MM
            return ActiveGame::OcarinaOfTime;
        }

        // Check if scene ID is valid for OoT
        if is_valid_oot_scene(oot_scene_id) {
            return ActiveGame::OcarinaOfTime;
        }

        // Default to last known state
        self.last_active_game
    }

    /// Detect transition state based on scene changes
    fn detect_transition(&self, new_active_game: ActiveGame) -> TransitionState {
        if new_active_game == self.last_active_game {
            // Check for transition scenes
            if new_active_game == ActiveGame::OcarinaOfTime
                && self.last_scene_id == oot_scenes::HAPPY_MASK_SHOP
            {
                // At Happy Mask Shop, might be about to transition
                return TransitionState::OotToMm;
            }
            if new_active_game == ActiveGame::MajorasMask
                && self.last_scene_id == mm_scenes::CLOCK_TOWER
            {
                // At Clock Tower, might be about to transition
                return TransitionState::MmToOot;
            }
            return TransitionState::Stable;
        }

        // Active game changed
        match (self.last_active_game, new_active_game) {
            (ActiveGame::OcarinaOfTime, ActiveGame::MajorasMask) => TransitionState::OotToMm,
            (ActiveGame::MajorasMask, ActiveGame::OcarinaOfTime) => TransitionState::MmToOot,
            _ => TransitionState::Stable,
        }
    }

    /// Check if a scene ID represents a transition point between games
    pub fn is_transition_scene(&self, scene_id: u8, active_game: ActiveGame) -> bool {
        match active_game {
            ActiveGame::OcarinaOfTime => scene_id == oot_scenes::HAPPY_MASK_SHOP,
            ActiveGame::MajorasMask => scene_id == mm_scenes::CLOCK_TOWER,
        }
    }
}

impl Default for GameDetector {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// Helper Functions
// ============================================================================

/// Check if a scene ID is valid for OoT
pub fn is_valid_oot_scene(scene_id: u8) -> bool {
    // OoT has scenes in ranges 0x00-0x65 with some gaps
    // Major valid ranges:
    // 0x00-0x0F: Dungeons and related areas
    // 0x10-0x1F: More interior areas
    // 0x20-0x4F: Interior locations (houses, shops, etc.)
    // 0x50-0x65: Overworld areas
    scene_id <= OOT_MAX_SCENE_ID
}

/// Check if a scene ID is valid for MM
pub fn is_valid_mm_scene(scene_id: u8) -> bool {
    // MM has scenes in ranges 0x00-0x70+
    scene_id <= MM_MAX_SCENE_ID
}

/// Check if a scene ID is a dungeon in OoT
pub fn is_oot_dungeon_scene(scene_id: u8) -> bool {
    matches!(
        scene_id,
        oot_scenes::DEKU_TREE
            | oot_scenes::DODONGOS_CAVERN
            | oot_scenes::JABU_JABUS_BELLY
            | oot_scenes::FOREST_TEMPLE
            | oot_scenes::FIRE_TEMPLE
            | oot_scenes::WATER_TEMPLE
            | oot_scenes::SPIRIT_TEMPLE
            | oot_scenes::SHADOW_TEMPLE
            | oot_scenes::BOTTOM_OF_THE_WELL
            | oot_scenes::ICE_CAVERN
            | oot_scenes::GANONS_CASTLE_TOWER
            | oot_scenes::GERUDO_TRAINING_GROUND
            | oot_scenes::GANONS_CASTLE
    )
}

/// Check if a scene ID is a dungeon in MM
pub fn is_mm_dungeon_scene(scene_id: u8) -> bool {
    matches!(
        scene_id,
        mm_scenes::WOODFALL_TEMPLE
            | mm_scenes::SNOWHEAD_TEMPLE
            | mm_scenes::GREAT_BAY_TEMPLE
            | mm_scenes::STONE_TOWER_TEMPLE
            | mm_scenes::STONE_TOWER_TEMPLE_INVERTED
            | mm_scenes::MAJORAS_LAIR
    )
}

/// Detect game type from ROM header data
pub fn detect_game_type_from_rom(rom_header: &[u8]) -> Option<GameType> {
    if rom_header.len() < 0x40 {
        return None;
    }

    // Read game ID from ROM header at offset 0x3B-0x3E
    let game_id = &rom_header[0x3B..0x3F];

    match game_id {
        // NTSC-U OoT: "CZLE"
        [b'C', b'Z', b'L', b'E'] => Some(GameType::StandaloneOoT),
        // NTSC-J OoT: "CZLJ"
        [b'C', b'Z', b'L', b'J'] => Some(GameType::StandaloneOoT),
        // PAL OoT: "CZLP"
        [b'C', b'Z', b'L', b'P'] => Some(GameType::StandaloneOoT),
        // NTSC-U MM: "NZSE"
        [b'N', b'Z', b'S', b'E'] => Some(GameType::StandaloneMM),
        // NTSC-J MM: "NZSJ"
        [b'N', b'Z', b'S', b'J'] => Some(GameType::StandaloneMM),
        // PAL MM: "NZSP"
        [b'N', b'Z', b'S', b'P'] => Some(GameType::StandaloneMM),
        // OoTMM uses a custom game ID that starts with specific patterns
        // This is a placeholder - actual OoTMM detection may need ROM hash comparison
        _ => {
            // Check for OoTMM specific signatures
            // OoTMM ROMs might have specific header modifications
            if is_ootmm_rom_header(rom_header) {
                Some(GameType::OoTMMCombo)
            } else {
                None
            }
        }
    }
}

/// Check if ROM header indicates an OoTMM combo ROM
fn is_ootmm_rom_header(rom_header: &[u8]) -> bool {
    if rom_header.len() < 0x40 {
        return false;
    }

    // OoTMM ROMs have specific characteristics:
    // 1. Modified game name in header
    // 2. Larger ROM size indicator
    // 3. Specific hash signatures

    // Check for "OOTMM" or similar in the ROM name area (offset 0x20-0x34)
    let rom_name = &rom_header[0x20..0x34];
    if rom_name.windows(5).any(|w| w == b"OOTMM") {
        return true;
    }

    // Check for modified game code patterns that OoTMM uses
    // This is a heuristic and may need refinement
    false
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_game_type_properties() {
        assert!(GameType::StandaloneOoT.is_standalone());
        assert!(GameType::StandaloneMM.is_standalone());
        assert!(!GameType::OoTMMCombo.is_standalone());

        assert!(!GameType::StandaloneOoT.is_combo());
        assert!(!GameType::StandaloneMM.is_combo());
        assert!(GameType::OoTMMCombo.is_combo());

        assert!(GameType::StandaloneOoT.has_oot());
        assert!(!GameType::StandaloneOoT.has_mm());
        assert!(!GameType::StandaloneMM.has_oot());
        assert!(GameType::StandaloneMM.has_mm());
        assert!(GameType::OoTMMCombo.has_oot());
        assert!(GameType::OoTMMCombo.has_mm());
    }

    #[test]
    fn test_active_game_properties() {
        assert!(ActiveGame::OcarinaOfTime.is_oot());
        assert!(!ActiveGame::OcarinaOfTime.is_mm());
        assert!(!ActiveGame::MajorasMask.is_oot());
        assert!(ActiveGame::MajorasMask.is_mm());
    }

    #[test]
    fn test_transition_state_properties() {
        assert!(!TransitionState::Stable.is_transitioning());
        assert!(TransitionState::OotToMm.is_transitioning());
        assert!(TransitionState::MmToOot.is_transitioning());

        assert_eq!(TransitionState::Stable.source_game(), None);
        assert_eq!(
            TransitionState::OotToMm.source_game(),
            Some(ActiveGame::OcarinaOfTime)
        );
        assert_eq!(
            TransitionState::MmToOot.source_game(),
            Some(ActiveGame::MajorasMask)
        );

        assert_eq!(TransitionState::Stable.destination_game(), None);
        assert_eq!(
            TransitionState::OotToMm.destination_game(),
            Some(ActiveGame::MajorasMask)
        );
        assert_eq!(
            TransitionState::MmToOot.destination_game(),
            Some(ActiveGame::OcarinaOfTime)
        );
    }

    #[test]
    fn test_game_detection_result_constructors() {
        let oot_result = GameDetectionResult::standalone_oot(0x55);
        assert_eq!(oot_result.game_type, GameType::StandaloneOoT);
        assert_eq!(oot_result.active_game, ActiveGame::OcarinaOfTime);
        assert_eq!(oot_result.scene_id, 0x55);
        assert_eq!(oot_result.confidence, 1.0);

        let mm_result = GameDetectionResult::standalone_mm(0x6E);
        assert_eq!(mm_result.game_type, GameType::StandaloneMM);
        assert_eq!(mm_result.active_game, ActiveGame::MajorasMask);
        assert_eq!(mm_result.scene_id, 0x6E);

        let combo_result =
            GameDetectionResult::combo(ActiveGame::OcarinaOfTime, 0x51, TransitionState::Stable);
        assert_eq!(combo_result.game_type, GameType::OoTMMCombo);
        assert!(combo_result.is_in_oot());
        assert!(!combo_result.is_in_mm());
    }

    #[test]
    fn test_game_detector_initialization() {
        let detector = GameDetector::new();
        assert_eq!(detector.game_type(), GameType::StandaloneOoT);
        assert_eq!(detector.active_game(), ActiveGame::OcarinaOfTime);
        assert_eq!(detector.transition_state(), TransitionState::Stable);

        let mm_detector = GameDetector::with_game_type(GameType::StandaloneMM);
        assert_eq!(mm_detector.game_type(), GameType::StandaloneMM);
        assert_eq!(mm_detector.active_game(), ActiveGame::MajorasMask);

        let combo_detector = GameDetector::with_game_type(GameType::OoTMMCombo);
        assert_eq!(combo_detector.game_type(), GameType::OoTMMCombo);
        assert_eq!(combo_detector.active_game(), ActiveGame::OcarinaOfTime);
    }

    #[test]
    fn test_game_detector_set_game_type() {
        let mut detector = GameDetector::new();

        detector.set_game_type(GameType::StandaloneMM);
        assert_eq!(detector.game_type(), GameType::StandaloneMM);
        assert_eq!(detector.active_game(), ActiveGame::MajorasMask);

        detector.set_game_type(GameType::OoTMMCombo);
        assert_eq!(detector.game_type(), GameType::OoTMMCombo);
        // Active game should remain MM since we didn't reset it
        assert_eq!(detector.active_game(), ActiveGame::MajorasMask);
    }

    #[test]
    fn test_is_valid_oot_scene() {
        assert!(is_valid_oot_scene(oot_scenes::KOKIRI_FOREST));
        assert!(is_valid_oot_scene(oot_scenes::HYRULE_FIELD));
        assert!(is_valid_oot_scene(oot_scenes::DEKU_TREE));
        assert!(is_valid_oot_scene(0x65));
        assert!(!is_valid_oot_scene(0x66));
        assert!(!is_valid_oot_scene(0xFF));
    }

    #[test]
    fn test_is_valid_mm_scene() {
        assert!(is_valid_mm_scene(mm_scenes::CLOCK_TOWN_SOUTH));
        assert!(is_valid_mm_scene(mm_scenes::TERMINA_FIELD));
        assert!(is_valid_mm_scene(mm_scenes::WOODFALL_TEMPLE));
        assert!(is_valid_mm_scene(0x70));
        assert!(!is_valid_mm_scene(0x71));
    }

    #[test]
    fn test_is_oot_dungeon_scene() {
        assert!(is_oot_dungeon_scene(oot_scenes::DEKU_TREE));
        assert!(is_oot_dungeon_scene(oot_scenes::FOREST_TEMPLE));
        assert!(is_oot_dungeon_scene(oot_scenes::GANONS_CASTLE));
        assert!(!is_oot_dungeon_scene(oot_scenes::KOKIRI_FOREST));
        assert!(!is_oot_dungeon_scene(oot_scenes::HYRULE_FIELD));
    }

    #[test]
    fn test_is_mm_dungeon_scene() {
        assert!(is_mm_dungeon_scene(mm_scenes::WOODFALL_TEMPLE));
        assert!(is_mm_dungeon_scene(mm_scenes::SNOWHEAD_TEMPLE));
        assert!(is_mm_dungeon_scene(mm_scenes::STONE_TOWER_TEMPLE_INVERTED));
        assert!(!is_mm_dungeon_scene(mm_scenes::CLOCK_TOWN_SOUTH));
        assert!(!is_mm_dungeon_scene(mm_scenes::TERMINA_FIELD));
    }

    #[test]
    fn test_transition_scene_detection() {
        let detector = GameDetector::with_game_type(GameType::OoTMMCombo);

        assert!(
            detector.is_transition_scene(oot_scenes::HAPPY_MASK_SHOP, ActiveGame::OcarinaOfTime)
        );
        assert!(!detector.is_transition_scene(oot_scenes::KOKIRI_FOREST, ActiveGame::OcarinaOfTime));

        assert!(detector.is_transition_scene(mm_scenes::CLOCK_TOWER, ActiveGame::MajorasMask));
        assert!(!detector.is_transition_scene(mm_scenes::CLOCK_TOWN_SOUTH, ActiveGame::MajorasMask));
    }

    #[test]
    fn test_detect_standalone_oot() {
        let mut detector = GameDetector::with_game_type(GameType::StandaloneOoT);

        // Create fake RAM with scene ID at OoT offset
        let mut ram = vec![0u8; RAM_SIZE];
        ram[0x1c8545] = oot_scenes::KOKIRI_FOREST;

        let result = detector.detect_from_ram(&ram);
        assert_eq!(result.game_type, GameType::StandaloneOoT);
        assert_eq!(result.active_game, ActiveGame::OcarinaOfTime);
        assert_eq!(result.scene_id, oot_scenes::KOKIRI_FOREST);
        assert_eq!(result.transition_state, TransitionState::Stable);
    }

    #[test]
    fn test_detect_standalone_mm() {
        let mut detector = GameDetector::with_game_type(GameType::StandaloneMM);

        // Create fake RAM with scene ID at MM offset
        let mut ram = vec![0u8; RAM_SIZE];
        ram[0x1ef674] = mm_scenes::CLOCK_TOWN_SOUTH;

        let result = detector.detect_from_ram(&ram);
        assert_eq!(result.game_type, GameType::StandaloneMM);
        assert_eq!(result.active_game, ActiveGame::MajorasMask);
        assert_eq!(result.scene_id, mm_scenes::CLOCK_TOWN_SOUTH);
    }

    #[test]
    fn test_detect_combo_oot_active() {
        let mut detector = GameDetector::with_game_type(GameType::OoTMMCombo);

        // Create fake RAM with OoT context active
        let mut ram = vec![0u8; RAM_SIZE];

        // Set OoT context to non-zero
        let oot_offset = (OOTMM_OOT_CONTEXT_ADDR - RDRAM_BASE) as usize;
        ram[oot_offset..oot_offset + 4].copy_from_slice(&[0x00, 0x01, 0x00, 0x00]);

        // Set MM context to zero (already zero from vec initialization)

        // Set scene ID
        ram[0x1c8545] = oot_scenes::HYRULE_FIELD;

        let result = detector.detect_from_ram(&ram);
        assert_eq!(result.game_type, GameType::OoTMMCombo);
        assert_eq!(result.active_game, ActiveGame::OcarinaOfTime);
        assert_eq!(result.scene_id, oot_scenes::HYRULE_FIELD);
        assert_eq!(result.confidence, 1.0);
    }

    #[test]
    fn test_detect_combo_mm_active() {
        let mut detector = GameDetector::with_game_type(GameType::OoTMMCombo);

        // Create fake RAM with MM context active
        let mut ram = vec![0u8; RAM_SIZE];

        // Set MM context to non-zero
        let mm_offset = (OOTMM_MM_CONTEXT_ADDR - RDRAM_BASE) as usize;
        ram[mm_offset..mm_offset + 4].copy_from_slice(&[0x00, 0x01, 0x00, 0x00]);

        // OoT context is zero (default)

        // Set scene ID at MM location
        ram[0x1ef674] = mm_scenes::CLOCK_TOWN_SOUTH;

        let result = detector.detect_from_ram(&ram);
        assert_eq!(result.game_type, GameType::OoTMMCombo);
        assert_eq!(result.active_game, ActiveGame::MajorasMask);
        assert_eq!(result.scene_id, mm_scenes::CLOCK_TOWN_SOUTH);
    }

    #[test]
    fn test_detect_game_transition() {
        let mut detector = GameDetector::with_game_type(GameType::OoTMMCombo);

        // Start with OoT active
        let mut ram = vec![0u8; RAM_SIZE];
        let oot_offset = (OOTMM_OOT_CONTEXT_ADDR - RDRAM_BASE) as usize;
        ram[oot_offset..oot_offset + 4].copy_from_slice(&[0x00, 0x01, 0x00, 0x00]);
        ram[0x1c8545] = oot_scenes::HYRULE_FIELD;

        let result1 = detector.detect_from_ram(&ram);
        assert_eq!(result1.active_game, ActiveGame::OcarinaOfTime);
        assert_eq!(result1.transition_state, TransitionState::Stable);

        // Now switch to MM
        ram[oot_offset..oot_offset + 4].copy_from_slice(&[0x00, 0x00, 0x00, 0x00]);
        let mm_offset = (OOTMM_MM_CONTEXT_ADDR - RDRAM_BASE) as usize;
        ram[mm_offset..mm_offset + 4].copy_from_slice(&[0x00, 0x01, 0x00, 0x00]);
        ram[0x1ef674] = mm_scenes::CLOCK_TOWN_SOUTH;

        let result2 = detector.detect_from_ram(&ram);
        assert_eq!(result2.active_game, ActiveGame::MajorasMask);
        assert_eq!(result2.transition_state, TransitionState::OotToMm);
    }

    #[test]
    fn test_detect_with_insufficient_ram() {
        let mut detector = GameDetector::new();

        // Test with RAM that's too small
        let small_ram = vec![0u8; 100];
        let result = detector.detect_from_ram(&small_ram);
        assert_eq!(result.confidence, 0.0);
    }

    #[test]
    fn test_game_type_display() {
        assert_eq!(format!("{}", GameType::StandaloneOoT), "Ocarina of Time");
        assert_eq!(format!("{}", GameType::StandaloneMM), "Majora's Mask");
        assert_eq!(format!("{}", GameType::OoTMMCombo), "OoTMM Combo");
    }

    #[test]
    fn test_active_game_display() {
        assert_eq!(format!("{}", ActiveGame::OcarinaOfTime), "OoT");
        assert_eq!(format!("{}", ActiveGame::MajorasMask), "MM");
    }

    #[test]
    fn test_confidence_clamping() {
        let result = GameDetectionResult::standalone_oot(0).with_confidence(1.5);
        assert_eq!(result.confidence, 1.0);

        let result = GameDetectionResult::standalone_oot(0).with_confidence(-0.5);
        assert_eq!(result.confidence, 0.0);

        let result = GameDetectionResult::standalone_oot(0).with_confidence(0.75);
        assert_eq!(result.confidence, 0.75);
    }

    #[test]
    fn test_detect_game_type_from_rom_oot() {
        // NTSC-U OoT header
        let mut rom_header = vec![0u8; 0x40];
        rom_header[0x3B..0x3F].copy_from_slice(b"CZLE");

        let result = detect_game_type_from_rom(&rom_header);
        assert_eq!(result, Some(GameType::StandaloneOoT));

        // NTSC-J OoT header
        rom_header[0x3B..0x3F].copy_from_slice(b"CZLJ");
        let result = detect_game_type_from_rom(&rom_header);
        assert_eq!(result, Some(GameType::StandaloneOoT));
    }

    #[test]
    fn test_detect_game_type_from_rom_mm() {
        // NTSC-U MM header
        let mut rom_header = vec![0u8; 0x40];
        rom_header[0x3B..0x3F].copy_from_slice(b"NZSE");

        let result = detect_game_type_from_rom(&rom_header);
        assert_eq!(result, Some(GameType::StandaloneMM));

        // PAL MM header
        rom_header[0x3B..0x3F].copy_from_slice(b"NZSP");
        let result = detect_game_type_from_rom(&rom_header);
        assert_eq!(result, Some(GameType::StandaloneMM));
    }

    #[test]
    fn test_detect_game_type_from_rom_unknown() {
        // Unknown game ID
        let mut rom_header = vec![0u8; 0x40];
        rom_header[0x3B..0x3F].copy_from_slice(b"XXXX");

        let result = detect_game_type_from_rom(&rom_header);
        assert_eq!(result, None);
    }

    #[test]
    fn test_detect_game_type_from_rom_too_short() {
        let rom_header = vec![0u8; 0x20];
        let result = detect_game_type_from_rom(&rom_header);
        assert_eq!(result, None);
    }

    #[test]
    fn test_defaults() {
        assert_eq!(GameType::default(), GameType::StandaloneOoT);
        assert_eq!(ActiveGame::default(), ActiveGame::OcarinaOfTime);
        assert_eq!(TransitionState::default(), TransitionState::Stable);
        assert_eq!(
            GameDetectionResult::default().game_type,
            GameType::StandaloneOoT
        );
    }

    #[test]
    fn test_serialization_roundtrip() {
        let result = GameDetectionResult::combo(
            ActiveGame::MajorasMask,
            mm_scenes::CLOCK_TOWN_SOUTH,
            TransitionState::OotToMm,
        )
        .with_confidence(0.9);

        let json = serde_json::to_string(&result).unwrap();
        let deserialized: GameDetectionResult = serde_json::from_str(&json).unwrap();

        assert_eq!(result.game_type, deserialized.game_type);
        assert_eq!(result.active_game, deserialized.active_game);
        assert_eq!(result.scene_id, deserialized.scene_id);
        assert_eq!(result.transition_state, deserialized.transition_state);
        assert!((result.confidence - deserialized.confidence).abs() < f32::EPSILON);
    }
}
