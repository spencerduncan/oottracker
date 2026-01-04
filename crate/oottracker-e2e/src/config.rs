//! ROM configuration helpers for E2E testing.
//!
//! This module provides utilities for configuring ROMs and emulator settings
//! for E2E tests, including ROM validation, version detection, and
//! configuration file generation.

use std::{
    collections::HashMap,
    fs,
    path::{Path, PathBuf},
};

/// Supported OoT ROM versions.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum OotVersion {
    /// US NTSC version 1.0
    NtscU10,
    /// US NTSC version 1.1
    NtscU11,
    /// US NTSC version 1.2
    NtscU12,
    /// US NTSC GameCube (Master Quest)
    NtscUGc,
    /// PAL version 1.0
    Pal10,
    /// PAL version 1.1
    Pal11,
    /// Japanese version 1.0
    NtscJ10,
    /// Japanese version 1.1
    NtscJ11,
    /// Japanese version 1.2
    NtscJ12,
}

impl OotVersion {
    /// Returns the ROM CRC for this version.
    pub fn crc(&self) -> (u32, u32) {
        match self {
            OotVersion::NtscU10 => (0xEC7011B7, 0x7616D72B),
            OotVersion::NtscU11 => (0xD43DA81F, 0x021E1E19),
            OotVersion::NtscU12 => (0x693BA2AE, 0xB7F14E9F),
            OotVersion::NtscUGc => (0xF3DD35BA, 0x4152E075),
            OotVersion::Pal10 => (0xB044B569, 0x373C1985),
            OotVersion::Pal11 => (0xB2055FBD, 0x0BAB4E0C),
            OotVersion::NtscJ10 => (0xEC7011B7, 0x7616D72B),
            OotVersion::NtscJ11 => (0xD43DA81F, 0x021E1E19),
            OotVersion::NtscJ12 => (0x693BA2AE, 0xB7F14E9F),
        }
    }

    /// Returns a human-readable name for this version.
    pub fn name(&self) -> &'static str {
        match self {
            OotVersion::NtscU10 => "OoT NTSC-U 1.0",
            OotVersion::NtscU11 => "OoT NTSC-U 1.1",
            OotVersion::NtscU12 => "OoT NTSC-U 1.2",
            OotVersion::NtscUGc => "OoT NTSC-U GC",
            OotVersion::Pal10 => "OoT PAL 1.0",
            OotVersion::Pal11 => "OoT PAL 1.1",
            OotVersion::NtscJ10 => "OoT NTSC-J 1.0",
            OotVersion::NtscJ11 => "OoT NTSC-J 1.1",
            OotVersion::NtscJ12 => "OoT NTSC-J 1.2",
        }
    }

    /// Returns the save context address for this version.
    pub fn save_context_addr(&self) -> u32 {
        // Save context address is the same for all versions
        0x11A5D0
    }
}

/// Supported MM ROM versions.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MmVersion {
    /// US NTSC version
    NtscU,
    /// PAL version
    Pal,
    /// Japanese version
    NtscJ,
    /// Japanese Collector's Edition (GameCube)
    NtscJGc,
}

impl MmVersion {
    /// Returns the ROM CRC for this version.
    pub fn crc(&self) -> (u32, u32) {
        match self {
            MmVersion::NtscU => (0x5354631C, 0x03A2DEF0),
            MmVersion::Pal => (0x6F5E1D83, 0xCE4A2E51),
            MmVersion::NtscJ => (0xE97955F6, 0xD4E6A4B4),
            MmVersion::NtscJGc => (0xB428D8A7, 0x5BC0EB7A),
        }
    }

    /// Returns a human-readable name for this version.
    pub fn name(&self) -> &'static str {
        match self {
            MmVersion::NtscU => "MM NTSC-U",
            MmVersion::Pal => "MM PAL",
            MmVersion::NtscJ => "MM NTSC-J",
            MmVersion::NtscJGc => "MM NTSC-J GC",
        }
    }

    /// Returns the save context address for this version.
    pub fn save_context_addr(&self) -> u32 {
        // MM save context address
        0x1EF670
    }
}

/// ROM type (OoT, MM, or Combo).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RomType {
    /// Ocarina of Time
    Oot(OotVersion),
    /// Majora's Mask
    Mm(MmVersion),
    /// OoTMM Combo ROM
    Combo,
    /// Unknown ROM
    Unknown,
}

/// ROM information extracted from the ROM header.
#[derive(Debug, Clone)]
pub struct RomInfo {
    /// The detected ROM type.
    pub rom_type: RomType,
    /// ROM title from header (max 20 chars).
    pub title: String,
    /// Game code from header (4 chars).
    pub game_code: String,
    /// CRC values from header.
    pub crc: (u32, u32),
    /// File size in bytes.
    pub size: u64,
    /// File path.
    pub path: PathBuf,
}

/// Errors that can occur during ROM configuration.
#[derive(Debug, thiserror::Error)]
pub enum ConfigError {
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),

    #[error("ROM file not found: {0}")]
    RomNotFound(PathBuf),

    #[error("Invalid ROM header")]
    InvalidHeader,

    #[error("ROM too small: {size} bytes (minimum 64 bytes required)")]
    RomTooSmall { size: u64 },

    #[error("Unsupported ROM type")]
    UnsupportedRom,

    #[error("Configuration directory not found: {0}")]
    ConfigDirNotFound(PathBuf),
}

/// Result type for config operations.
pub type Result<T> = std::result::Result<T, ConfigError>;

/// Validates a ROM file and returns information about it.
pub fn validate_rom(path: &Path) -> Result<RomInfo> {
    if !path.exists() {
        return Err(ConfigError::RomNotFound(path.to_path_buf()));
    }

    let metadata = fs::metadata(path)?;
    let size = metadata.len();

    if size < 64 {
        return Err(ConfigError::RomTooSmall { size });
    }

    // Read ROM header
    let data = fs::read(path)?;

    // Check for byte-swapped formats and handle them
    let header = if data[0] == 0x80 {
        // Big-endian (native N64 format)
        data[..64].to_vec()
    } else if data[0] == 0x37 {
        // Byte-swapped
        let mut swapped = vec![0u8; 64];
        for i in 0..32 {
            swapped[i * 2] = data[i * 2 + 1];
            swapped[i * 2 + 1] = data[i * 2];
        }
        swapped
    } else if data[0] == 0x40 {
        // Little-endian
        let mut swapped = vec![0u8; 64];
        for i in 0..16 {
            swapped[i * 4] = data[i * 4 + 3];
            swapped[i * 4 + 1] = data[i * 4 + 2];
            swapped[i * 4 + 2] = data[i * 4 + 1];
            swapped[i * 4 + 3] = data[i * 4];
        }
        swapped
    } else {
        return Err(ConfigError::InvalidHeader);
    };

    // Extract CRC values (offsets 0x10 and 0x14)
    let crc1 = u32::from_be_bytes([header[0x10], header[0x11], header[0x12], header[0x13]]);
    let crc2 = u32::from_be_bytes([header[0x14], header[0x15], header[0x16], header[0x17]]);

    // Extract title (offset 0x20, 20 bytes)
    let title = String::from_utf8_lossy(&header[0x20..0x34])
        .trim_end_matches('\0')
        .trim()
        .to_string();

    // Extract game code (offset 0x3B, 4 bytes)
    let game_code = String::from_utf8_lossy(&header[0x3B..0x3F])
        .trim_end_matches('\0')
        .to_string();

    // Detect ROM type based on CRC
    let rom_type = detect_rom_type((crc1, crc2), &title, &game_code);

    Ok(RomInfo {
        rom_type,
        title,
        game_code,
        crc: (crc1, crc2),
        size,
        path: path.to_path_buf(),
    })
}

/// Detects the ROM type based on CRC and header info.
fn detect_rom_type(crc: (u32, u32), title: &str, _game_code: &str) -> RomType {
    // Check for known OoT versions
    for version in [
        OotVersion::NtscU10,
        OotVersion::NtscU11,
        OotVersion::NtscU12,
        OotVersion::NtscUGc,
        OotVersion::Pal10,
        OotVersion::Pal11,
        OotVersion::NtscJ10,
        OotVersion::NtscJ11,
        OotVersion::NtscJ12,
    ] {
        if version.crc() == crc {
            return RomType::Oot(version);
        }
    }

    // Check for known MM versions
    for version in [
        MmVersion::NtscU,
        MmVersion::Pal,
        MmVersion::NtscJ,
        MmVersion::NtscJGc,
    ] {
        if version.crc() == crc {
            return RomType::Mm(version);
        }
    }

    // Check for combo ROM based on title
    if title.contains("OoTMM") || title.contains("COMBO") {
        return RomType::Combo;
    }

    // Fallback to title-based detection
    if title.contains("ZELDA") {
        if title.contains("MASK") || title.contains("MAJORA") {
            return RomType::Mm(MmVersion::NtscU); // Default to US
        } else if title.contains("OCARINA") || title.contains("TIME") {
            return RomType::Oot(OotVersion::NtscU12); // Default to US 1.2
        }
    }

    RomType::Unknown
}

/// Project64-EM configuration settings.
#[derive(Debug, Clone)]
pub struct Pj64EmConfig {
    /// Path to the configuration directory.
    pub config_dir: PathBuf,
    /// ROM-specific settings.
    pub rom_settings: HashMap<String, RomSettings>,
}

/// Settings for a specific ROM.
#[derive(Debug, Clone, Default)]
pub struct RomSettings {
    /// Enable Lua script auto-load.
    pub lua_autoload: bool,
    /// Path to Lua script to load.
    pub lua_script: Option<PathBuf>,
    /// Enable expanded memory (for combo ROMs).
    pub expanded_memory: bool,
    /// Custom save directory.
    pub save_directory: Option<PathBuf>,
    /// Custom save state directory.
    pub state_directory: Option<PathBuf>,
}

impl Pj64EmConfig {
    /// Creates a new configuration with default settings.
    pub fn new(config_dir: PathBuf) -> Self {
        Self {
            config_dir,
            rom_settings: HashMap::new(),
        }
    }

    /// Adds or updates settings for a ROM.
    pub fn set_rom_settings(&mut self, rom_id: String, settings: RomSettings) {
        self.rom_settings.insert(rom_id, settings);
    }

    /// Gets settings for a ROM.
    pub fn get_rom_settings(&self, rom_id: &str) -> Option<&RomSettings> {
        self.rom_settings.get(rom_id)
    }

    /// Creates default E2E test settings for a ROM.
    pub fn create_e2e_settings(
        &mut self,
        rom_info: &RomInfo,
        lua_script: PathBuf,
        save_dir: PathBuf,
    ) {
        let settings = RomSettings {
            lua_autoload: true,
            lua_script: Some(lua_script),
            expanded_memory: matches!(rom_info.rom_type, RomType::Combo),
            save_directory: Some(save_dir.clone()),
            state_directory: Some(save_dir),
        };

        // Use CRC as ROM ID
        let rom_id = format!("{:08X}-{:08X}", rom_info.crc.0, rom_info.crc.1);
        self.set_rom_settings(rom_id, settings);
    }
}

/// Environment configuration for E2E tests.
#[derive(Debug, Clone)]
pub struct TestEnvironment {
    /// Wine prefix directory.
    pub wine_prefix: PathBuf,
    /// Project64-EM installation directory.
    pub pj64_dir: PathBuf,
    /// ROM directory.
    pub rom_dir: PathBuf,
    /// Test data directory (fixtures, save states).
    pub test_data_dir: PathBuf,
    /// Temporary directory for test artifacts.
    pub temp_dir: PathBuf,
}

impl TestEnvironment {
    /// Creates a new test environment configuration.
    pub fn new(base_dir: PathBuf) -> Self {
        Self {
            wine_prefix: base_dir.join(".wine-oottracker"),
            pj64_dir: base_dir.join("pj64-em"),
            rom_dir: base_dir.join("roms"),
            test_data_dir: base_dir.join("test-data"),
            temp_dir: base_dir.join("temp"),
        }
    }

    /// Creates a test environment from environment variables.
    pub fn from_env() -> Option<Self> {
        let wine_prefix = std::env::var("OOTTRACKER_WINE_PREFIX")
            .map(PathBuf::from)
            .ok()?;
        let pj64_dir = std::env::var("OOTTRACKER_PJ64_DIR")
            .map(PathBuf::from)
            .ok()?;

        Some(Self {
            wine_prefix,
            pj64_dir: pj64_dir.clone(),
            rom_dir: std::env::var("OOTTRACKER_ROM_DIR")
                .map(PathBuf::from)
                .unwrap_or_else(|_| pj64_dir.join("roms")),
            test_data_dir: std::env::var("OOTTRACKER_TEST_DATA")
                .map(PathBuf::from)
                .unwrap_or_else(|_| PathBuf::from("test-data")),
            temp_dir: std::env::temp_dir().join("oottracker-e2e"),
        })
    }

    /// Returns the path to the Project64-EM executable.
    pub fn pj64_exe(&self) -> PathBuf {
        self.pj64_dir.join("Project64.exe")
    }

    /// Returns the path to the Lua script directory.
    pub fn lua_dir(&self) -> PathBuf {
        self.pj64_dir.join("Scripts")
    }

    /// Creates the necessary directories for testing.
    pub fn setup_directories(&self) -> Result<()> {
        fs::create_dir_all(&self.wine_prefix)?;
        fs::create_dir_all(&self.rom_dir)?;
        fs::create_dir_all(&self.test_data_dir)?;
        fs::create_dir_all(&self.temp_dir)?;
        fs::create_dir_all(self.lua_dir())?;
        Ok(())
    }

    /// Cleans up temporary test artifacts.
    pub fn cleanup(&self) -> Result<()> {
        if self.temp_dir.exists() {
            fs::remove_dir_all(&self.temp_dir)?;
        }
        Ok(())
    }

    /// Validates that all required paths exist.
    pub fn validate(&self) -> Result<()> {
        if !self.wine_prefix.exists() {
            return Err(ConfigError::ConfigDirNotFound(self.wine_prefix.clone()));
        }
        if !self.pj64_exe().exists() {
            return Err(ConfigError::ConfigDirNotFound(self.pj64_exe()));
        }
        Ok(())
    }
}

/// Save state management for E2E tests.
pub struct SaveStateManager {
    /// Directory containing save states.
    state_dir: PathBuf,
}

impl SaveStateManager {
    /// Creates a new save state manager.
    pub fn new(state_dir: PathBuf) -> Self {
        Self { state_dir }
    }

    /// Returns the path to a save state file.
    pub fn state_path(&self, name: &str) -> PathBuf {
        self.state_dir.join(format!("{}.pj", name))
    }

    /// Lists available save states.
    pub fn list_states(&self) -> Result<Vec<String>> {
        let mut states = Vec::new();

        if !self.state_dir.exists() {
            return Ok(states);
        }

        for entry in fs::read_dir(&self.state_dir)? {
            let entry = entry?;
            let path = entry.path();

            if path.extension().is_some_and(|ext| ext == "pj") {
                if let Some(name) = path.file_stem() {
                    states.push(name.to_string_lossy().to_string());
                }
            }
        }

        states.sort();
        Ok(states)
    }

    /// Checks if a save state exists.
    pub fn state_exists(&self, name: &str) -> bool {
        self.state_path(name).exists()
    }

    /// Creates the state directory if it doesn't exist.
    pub fn ensure_dir(&self) -> Result<()> {
        fs::create_dir_all(&self.state_dir)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_oot_version_crc() {
        let v10 = OotVersion::NtscU10;
        assert_eq!(v10.crc(), (0xEC7011B7, 0x7616D72B));
    }

    #[test]
    fn test_oot_version_name() {
        assert_eq!(OotVersion::NtscU12.name(), "OoT NTSC-U 1.2");
        assert_eq!(OotVersion::Pal10.name(), "OoT PAL 1.0");
    }

    #[test]
    fn test_mm_version_crc() {
        let ntsc = MmVersion::NtscU;
        assert_eq!(ntsc.crc(), (0x5354631C, 0x03A2DEF0));
    }

    #[test]
    fn test_detect_rom_type_oot() {
        let crc = OotVersion::NtscU12.crc();
        let rom_type = detect_rom_type(crc, "THE LEGEND OF ZELDA", "CZLE");

        assert!(matches!(rom_type, RomType::Oot(OotVersion::NtscU12)));
    }

    #[test]
    fn test_detect_rom_type_mm() {
        let crc = MmVersion::NtscU.crc();
        let rom_type = detect_rom_type(crc, "ZELDA MAJORA'S MASK", "NZSE");

        assert!(matches!(rom_type, RomType::Mm(MmVersion::NtscU)));
    }

    #[test]
    fn test_detect_rom_type_combo() {
        let rom_type = detect_rom_type((0x12345678, 0x87654321), "OoTMM COMBO", "OOTM");
        assert!(matches!(rom_type, RomType::Combo));
    }

    #[test]
    fn test_detect_rom_type_unknown() {
        let rom_type = detect_rom_type((0x00000000, 0x00000000), "UNKNOWN GAME", "UNKN");
        assert!(matches!(rom_type, RomType::Unknown));
    }

    #[test]
    fn test_rom_settings_default() {
        let settings = RomSettings::default();
        assert!(!settings.lua_autoload);
        assert!(!settings.expanded_memory);
        assert!(settings.lua_script.is_none());
    }

    #[test]
    fn test_pj64em_config() {
        let mut config = Pj64EmConfig::new(PathBuf::from("/tmp/pj64"));

        let settings = RomSettings {
            lua_autoload: true,
            lua_script: Some(PathBuf::from("/tmp/script.lua")),
            expanded_memory: true,
            ..Default::default()
        };

        config.set_rom_settings("test-rom".to_string(), settings);

        let retrieved = config.get_rom_settings("test-rom").unwrap();
        assert!(retrieved.lua_autoload);
        assert!(retrieved.expanded_memory);
    }

    #[test]
    fn test_test_environment_paths() {
        let env = TestEnvironment::new(PathBuf::from("/home/test/oottracker"));

        assert_eq!(
            env.wine_prefix,
            PathBuf::from("/home/test/oottracker/.wine-oottracker")
        );
        assert_eq!(
            env.pj64_exe(),
            PathBuf::from("/home/test/oottracker/pj64-em/Project64.exe")
        );
        assert_eq!(
            env.lua_dir(),
            PathBuf::from("/home/test/oottracker/pj64-em/Scripts")
        );
    }

    #[test]
    fn test_save_state_manager() {
        let manager = SaveStateManager::new(PathBuf::from("/tmp/states"));

        assert_eq!(
            manager.state_path("deku_tree_complete"),
            PathBuf::from("/tmp/states/deku_tree_complete.pj")
        );
    }
}
