//! Extended flags (xflags) and custom save data structures.
//!
//! This module provides Rust implementations of the OoTMM combo randomizer's
//! extended flag system and custom save structures.
//!
//! ## Submodules
//!
//! - [`mapping`]: Location ID to xflag bit position mappings
//!
//! Reference: OoTMM source
//! - packages/core/include/combo/xflags.h - Xflag struct definition
//! - packages/core/include/combo/xflags_data.h - XFLAGS_COUNT_OOT, XFLAGS_COUNT_MM
//! - packages/core/include/combo/oot/save.h - OotCustomSave
//! - packages/core/include/combo/mm/save.h - MmCustomSave
//! - packages/core/include/combo/save.h - SharedCustomSave

pub mod mapping;

use serde::{Deserialize, Serialize};

/// Number of xflags for Ocarina of Time (0x2e9 = 745)
pub const XFLAGS_COUNT_OOT: usize = 0x2e9;

/// Number of xflags for Majora's Mask (0x34a = 842)
pub const XFLAGS_COUNT_MM: usize = 0x34a;

/// Size of xflags byte array for OOT: ceil(745 / 8) = 94 bytes
/// Note: OoTMM source shows xflags[93], but 745 bits requires 94 bytes.
pub const XFLAGS_BYTES_OOT: usize = XFLAGS_COUNT_OOT.div_ceil(8);

/// Size of xflags byte array for MM: ceil(842 / 8) = 106 bytes
pub const XFLAGS_BYTES_MM: usize = XFLAGS_COUNT_MM.div_ceil(8);

/// Custom serde module for OOT xflags array (94 bytes)
mod serde_xflags_oot {
    use super::XFLAGS_BYTES_OOT;
    use serde::{Deserialize, Deserializer, Serialize, Serializer};

    pub fn serialize<S>(data: &[u8; XFLAGS_BYTES_OOT], serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        data.as_slice().serialize(serializer)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<[u8; XFLAGS_BYTES_OOT], D::Error>
    where
        D: Deserializer<'de>,
    {
        let vec: Vec<u8> = Vec::deserialize(deserializer)?;
        let mut arr = [0u8; XFLAGS_BYTES_OOT];
        let len = vec.len().min(XFLAGS_BYTES_OOT);
        arr[..len].copy_from_slice(&vec[..len]);
        Ok(arr)
    }
}

/// Custom serde module for MM xflags array (106 bytes)
mod serde_xflags_mm {
    use super::XFLAGS_BYTES_MM;
    use serde::{Deserialize, Deserializer, Serialize, Serializer};

    pub fn serialize<S>(data: &[u8; XFLAGS_BYTES_MM], serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        data.as_slice().serialize(serializer)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<[u8; XFLAGS_BYTES_MM], D::Error>
    where
        D: Deserializer<'de>,
    {
        let vec: Vec<u8> = Vec::deserialize(deserializer)?;
        let mut arr = [0u8; XFLAGS_BYTES_MM];
        let len = vec.len().min(XFLAGS_BYTES_MM);
        arr[..len].copy_from_slice(&vec[..len]);
        Ok(arr)
    }
}

/// Extended flag identifier.
///
/// Xflags are used by OoTMM to track randomized collectibles and other
/// items that need persistent tracking beyond the vanilla game's flags.
///
/// Each xflag uniquely identifies a location in the game world using:
/// - scene_id: The scene/area ID
/// - setup_id: The room setup/configuration
/// - room_id: The room within the scene
/// - slice_id: A subdivision for grouping items
/// - id: The specific item/actor within the slice
///
/// Reference: OoTMM packages/core/include/combo/xflags.h
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Xflag {
    pub scene_id: u8,
    pub setup_id: u8,
    pub room_id: u8,
    pub slice_id: u8,
    pub id: u8,
}

impl Xflag {
    /// Creates a new Xflag with the given identifiers.
    pub const fn new(scene_id: u8, setup_id: u8, room_id: u8, slice_id: u8, id: u8) -> Self {
        Self {
            scene_id,
            setup_id,
            room_id,
            slice_id,
            id,
        }
    }

    /// Creates an Xflag from a 5-byte array.
    ///
    /// The bytes are interpreted as: [scene_id, setup_id, room_id, slice_id, id]
    pub const fn from_bytes(bytes: [u8; 5]) -> Self {
        Self {
            scene_id: bytes[0],
            setup_id: bytes[1],
            room_id: bytes[2],
            slice_id: bytes[3],
            id: bytes[4],
        }
    }

    /// Converts the Xflag to a 5-byte array.
    pub const fn to_bytes(self) -> [u8; 5] {
        [
            self.scene_id,
            self.setup_id,
            self.room_id,
            self.slice_id,
            self.id,
        ]
    }
}

/// Trait for types that store xflags as a bit array.
pub trait XflagsStorage {
    /// Returns a reference to the underlying xflags byte array.
    fn xflags(&self) -> &[u8];

    /// Returns a mutable reference to the underlying xflags byte array.
    fn xflags_mut(&mut self) -> &mut [u8];

    /// Returns the maximum number of xflags this storage can hold.
    fn xflags_count(&self) -> usize;

    /// Checks if the xflag at the given index is set.
    ///
    /// Returns `false` if the index is out of bounds.
    fn is_xflag_set(&self, index: usize) -> bool {
        if index >= self.xflags_count() {
            return false;
        }
        let byte_index = index / 8;
        let bit_index = index % 8;
        if byte_index >= self.xflags().len() {
            return false;
        }
        (self.xflags()[byte_index] & (1 << bit_index)) != 0
    }

    /// Sets the xflag at the given index.
    ///
    /// Does nothing if the index is out of bounds.
    fn set_xflag(&mut self, index: usize) {
        if index >= self.xflags_count() {
            return;
        }
        let byte_index = index / 8;
        let bit_index = index % 8;
        if byte_index >= self.xflags().len() {
            return;
        }
        self.xflags_mut()[byte_index] |= 1 << bit_index;
    }

    /// Clears the xflag at the given index.
    ///
    /// Does nothing if the index is out of bounds.
    fn clear_xflag(&mut self, index: usize) {
        if index >= self.xflags_count() {
            return;
        }
        let byte_index = index / 8;
        let bit_index = index % 8;
        if byte_index >= self.xflags().len() {
            return;
        }
        self.xflags_mut()[byte_index] &= !(1 << bit_index);
    }

    /// Returns the number of xflags that are currently set.
    fn count_set_xflags(&self) -> usize {
        self.xflags().iter().map(|b| b.count_ones() as usize).sum()
    }
}

/// Ocarina of Time custom save data.
///
/// This structure holds OOT-specific extended tracking data used by
/// the OoTMM combo randomizer.
///
/// Reference: OoTMM packages/core/include/combo/oot/save.h lines 459-470
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OotCustomSave {
    /// Extended flags for tracking randomized items in OOT.
    /// Each bit represents a collected/checked location.
    #[serde(with = "serde_xflags_oot")]
    pub xflags: [u8; XFLAGS_BYTES_OOT],
}

impl Default for OotCustomSave {
    fn default() -> Self {
        Self {
            xflags: [0u8; XFLAGS_BYTES_OOT],
        }
    }
}

impl XflagsStorage for OotCustomSave {
    fn xflags(&self) -> &[u8] {
        &self.xflags
    }

    fn xflags_mut(&mut self) -> &mut [u8] {
        &mut self.xflags
    }

    fn xflags_count(&self) -> usize {
        XFLAGS_COUNT_OOT
    }
}

impl OotCustomSave {
    /// Creates a new OotCustomSave with all xflags cleared.
    pub fn new() -> Self {
        Self::default()
    }

    /// Creates an OotCustomSave from a byte slice.
    ///
    /// If the slice is shorter than required, remaining bytes are zero-filled.
    /// If longer, extra bytes are ignored.
    pub fn from_bytes(bytes: &[u8]) -> Self {
        let mut xflags = [0u8; XFLAGS_BYTES_OOT];
        let len = bytes.len().min(XFLAGS_BYTES_OOT);
        xflags[..len].copy_from_slice(&bytes[..len]);
        Self { xflags }
    }

    /// Returns the xflags data as a byte slice.
    pub fn as_bytes(&self) -> &[u8] {
        &self.xflags
    }
}

/// Majora's Mask custom save data.
///
/// This structure holds MM-specific extended tracking data used by
/// the OoTMM combo randomizer.
///
/// Reference: OoTMM packages/core/include/combo/mm/save.h lines 563-573
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MmCustomSave {
    /// Extended flags for tracking randomized items in MM.
    /// Each bit represents a collected/checked location.
    #[serde(with = "serde_xflags_mm")]
    pub xflags: [u8; XFLAGS_BYTES_MM],
}

impl Default for MmCustomSave {
    fn default() -> Self {
        Self {
            xflags: [0u8; XFLAGS_BYTES_MM],
        }
    }
}

impl XflagsStorage for MmCustomSave {
    fn xflags(&self) -> &[u8] {
        &self.xflags
    }

    fn xflags_mut(&mut self) -> &mut [u8] {
        &mut self.xflags
    }

    fn xflags_count(&self) -> usize {
        XFLAGS_COUNT_MM
    }
}

impl MmCustomSave {
    /// Creates a new MmCustomSave with all xflags cleared.
    pub fn new() -> Self {
        Self::default()
    }

    /// Creates a MmCustomSave from a byte slice.
    ///
    /// If the slice is shorter than required, remaining bytes are zero-filled.
    /// If longer, extra bytes are ignored.
    pub fn from_bytes(bytes: &[u8]) -> Self {
        let mut xflags = [0u8; XFLAGS_BYTES_MM];
        let len = bytes.len().min(XFLAGS_BYTES_MM);
        xflags[..len].copy_from_slice(&bytes[..len]);
        Self { xflags }
    }

    /// Returns the xflags data as a byte slice.
    pub fn as_bytes(&self) -> &[u8] {
        &self.xflags
    }
}

/// Shared custom save data for the OoTMM combo randomizer.
///
/// This structure contains the custom save data for both OOT and MM,
/// providing a unified interface for tracking progress across both games.
///
/// Reference: OoTMM packages/core/include/combo/save.h
#[derive(Debug, Default, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SharedCustomSave {
    /// Ocarina of Time custom save data.
    pub oot: OotCustomSave,
    /// Majora's Mask custom save data.
    pub mm: MmCustomSave,
}

impl SharedCustomSave {
    /// Creates a new SharedCustomSave with all xflags cleared.
    pub fn new() -> Self {
        Self::default()
    }

    /// Checks if an OOT xflag is set.
    pub fn is_oot_xflag_set(&self, index: usize) -> bool {
        self.oot.is_xflag_set(index)
    }

    /// Sets an OOT xflag.
    pub fn set_oot_xflag(&mut self, index: usize) {
        self.oot.set_xflag(index);
    }

    /// Clears an OOT xflag.
    pub fn clear_oot_xflag(&mut self, index: usize) {
        self.oot.clear_xflag(index);
    }

    /// Checks if an MM xflag is set.
    pub fn is_mm_xflag_set(&self, index: usize) -> bool {
        self.mm.is_xflag_set(index)
    }

    /// Sets an MM xflag.
    pub fn set_mm_xflag(&mut self, index: usize) {
        self.mm.set_xflag(index);
    }

    /// Clears an MM xflag.
    pub fn clear_mm_xflag(&mut self, index: usize) {
        self.mm.clear_xflag(index);
    }

    /// Returns the total number of set xflags across both games.
    pub fn total_set_xflags(&self) -> usize {
        self.oot.count_set_xflags() + self.mm.count_set_xflags()
    }

    /// Creates a SharedCustomSave from raw bytes.
    ///
    /// Expects OOT xflags (94 bytes) followed by MM xflags (106 bytes) = 200 bytes total.
    /// If fewer bytes are provided, remaining xflags are zero-filled.
    pub fn from_bytes(bytes: &[u8]) -> Self {
        let oot = OotCustomSave::from_bytes(&bytes[..bytes.len().min(XFLAGS_BYTES_OOT)]);
        let mm = if bytes.len() > XFLAGS_BYTES_OOT {
            MmCustomSave::from_bytes(&bytes[XFLAGS_BYTES_OOT..])
        } else {
            MmCustomSave::default()
        };
        Self { oot, mm }
    }

    /// Returns the combined xflags data as a byte vector.
    ///
    /// Returns OOT xflags (94 bytes) followed by MM xflags (106 bytes) = 200 bytes.
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::with_capacity(XFLAGS_BYTES_OOT + XFLAGS_BYTES_MM);
        bytes.extend_from_slice(&self.oot.xflags);
        bytes.extend_from_slice(&self.mm.xflags);
        bytes
    }
}

// Protocol implementation for SharedCustomSave
// Serializes as OOT xflags (94 bytes) + MM xflags (106 bytes) = 200 bytes total
impl async_proto::Protocol for SharedCustomSave {
    fn read<'a, R: tokio::io::AsyncRead + Unpin + Send + 'a>(
        stream: &'a mut R,
    ) -> std::pin::Pin<
        Box<dyn std::future::Future<Output = Result<Self, async_proto::ReadError>> + Send + 'a>,
    > {
        Box::pin(async move {
            use tokio::io::AsyncReadExt;
            const TOTAL_SIZE: usize = XFLAGS_BYTES_OOT + XFLAGS_BYTES_MM;
            let mut buf = [0u8; TOTAL_SIZE];
            stream.read_exact(&mut buf).await?;
            Ok(Self::from_bytes(&buf))
        })
    }

    fn write<'a, W: tokio::io::AsyncWrite + Unpin + Send + 'a>(
        &'a self,
        sink: &'a mut W,
    ) -> std::pin::Pin<
        Box<dyn std::future::Future<Output = Result<(), async_proto::WriteError>> + Send + 'a>,
    > {
        Box::pin(async move {
            use tokio::io::AsyncWriteExt;
            sink.write_all(&self.oot.xflags).await?;
            sink.write_all(&self.mm.xflags).await?;
            Ok(())
        })
    }

    fn read_sync(stream: &mut impl std::io::Read) -> Result<Self, async_proto::ReadError> {
        const TOTAL_SIZE: usize = XFLAGS_BYTES_OOT + XFLAGS_BYTES_MM;
        let mut buf = [0u8; TOTAL_SIZE];
        stream.read_exact(&mut buf)?;
        Ok(Self::from_bytes(&buf))
    }

    fn write_sync(&self, sink: &mut impl std::io::Write) -> Result<(), async_proto::WriteError> {
        sink.write_all(&self.oot.xflags)?;
        sink.write_all(&self.mm.xflags)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_xflag_creation() {
        let xflag = Xflag::new(1, 2, 3, 4, 5);
        assert_eq!(xflag.scene_id, 1);
        assert_eq!(xflag.setup_id, 2);
        assert_eq!(xflag.room_id, 3);
        assert_eq!(xflag.slice_id, 4);
        assert_eq!(xflag.id, 5);
    }

    #[test]
    fn test_xflag_from_bytes() {
        let bytes = [10, 20, 30, 40, 50];
        let xflag = Xflag::from_bytes(bytes);
        assert_eq!(xflag.scene_id, 10);
        assert_eq!(xflag.setup_id, 20);
        assert_eq!(xflag.room_id, 30);
        assert_eq!(xflag.slice_id, 40);
        assert_eq!(xflag.id, 50);
    }

    #[test]
    fn test_xflag_to_bytes() {
        let xflag = Xflag::new(1, 2, 3, 4, 5);
        let bytes = xflag.to_bytes();
        assert_eq!(bytes, [1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_xflag_roundtrip() {
        let original = Xflag::new(100, 50, 25, 12, 6);
        let bytes = original.to_bytes();
        let reconstructed = Xflag::from_bytes(bytes);
        assert_eq!(original, reconstructed);
    }

    #[test]
    fn test_oot_custom_save_default() {
        let save = OotCustomSave::default();
        assert_eq!(save.xflags.len(), XFLAGS_BYTES_OOT);
        assert!(save.xflags.iter().all(|&b| b == 0));
    }

    #[test]
    fn test_oot_custom_save_xflag_operations() {
        let mut save = OotCustomSave::new();

        // Test setting xflag 0
        assert!(!save.is_xflag_set(0));
        save.set_xflag(0);
        assert!(save.is_xflag_set(0));

        // Test setting xflag 7 (last bit of first byte)
        assert!(!save.is_xflag_set(7));
        save.set_xflag(7);
        assert!(save.is_xflag_set(7));

        // Test setting xflag 8 (first bit of second byte)
        assert!(!save.is_xflag_set(8));
        save.set_xflag(8);
        assert!(save.is_xflag_set(8));

        // Test clearing
        save.clear_xflag(0);
        assert!(!save.is_xflag_set(0));
        assert!(save.is_xflag_set(7)); // Others should still be set
        assert!(save.is_xflag_set(8));
    }

    #[test]
    fn test_oot_custom_save_out_of_bounds() {
        let mut save = OotCustomSave::new();

        // Out of bounds should not panic and return false
        assert!(!save.is_xflag_set(XFLAGS_COUNT_OOT));
        assert!(!save.is_xflag_set(XFLAGS_COUNT_OOT + 100));
        assert!(!save.is_xflag_set(usize::MAX));

        // Setting out of bounds should not panic
        save.set_xflag(XFLAGS_COUNT_OOT);
        save.set_xflag(usize::MAX);
    }

    #[test]
    fn test_oot_custom_save_count() {
        let mut save = OotCustomSave::new();
        assert_eq!(save.count_set_xflags(), 0);

        save.set_xflag(0);
        assert_eq!(save.count_set_xflags(), 1);

        save.set_xflag(100);
        assert_eq!(save.count_set_xflags(), 2);

        save.set_xflag(500);
        assert_eq!(save.count_set_xflags(), 3);
    }

    #[test]
    fn test_oot_custom_save_from_bytes() {
        let bytes = [0xFF, 0x00, 0xAA];
        let save = OotCustomSave::from_bytes(&bytes);
        assert_eq!(save.xflags[0], 0xFF);
        assert_eq!(save.xflags[1], 0x00);
        assert_eq!(save.xflags[2], 0xAA);
        // Remaining bytes should be zero
        assert!(save.xflags[3..].iter().all(|&b| b == 0));
    }

    #[test]
    fn test_mm_custom_save_default() {
        let save = MmCustomSave::default();
        assert_eq!(save.xflags.len(), XFLAGS_BYTES_MM);
        assert!(save.xflags.iter().all(|&b| b == 0));
    }

    #[test]
    fn test_mm_custom_save_xflag_operations() {
        let mut save = MmCustomSave::new();

        // Test setting various xflags
        save.set_xflag(0);
        save.set_xflag(100);
        save.set_xflag(500);
        save.set_xflag(XFLAGS_COUNT_MM - 1);

        assert!(save.is_xflag_set(0));
        assert!(save.is_xflag_set(100));
        assert!(save.is_xflag_set(500));
        assert!(save.is_xflag_set(XFLAGS_COUNT_MM - 1));
        assert!(!save.is_xflag_set(1));
    }

    #[test]
    fn test_shared_custom_save_default() {
        let save = SharedCustomSave::default();
        assert_eq!(save.oot.xflags.len(), XFLAGS_BYTES_OOT);
        assert_eq!(save.mm.xflags.len(), XFLAGS_BYTES_MM);
    }

    #[test]
    fn test_shared_custom_save_operations() {
        let mut save = SharedCustomSave::new();

        // Set OOT xflags
        save.set_oot_xflag(10);
        save.set_oot_xflag(20);

        // Set MM xflags
        save.set_mm_xflag(30);
        save.set_mm_xflag(40);
        save.set_mm_xflag(50);

        assert!(save.is_oot_xflag_set(10));
        assert!(save.is_oot_xflag_set(20));
        assert!(!save.is_oot_xflag_set(30));

        assert!(!save.is_mm_xflag_set(10));
        assert!(save.is_mm_xflag_set(30));
        assert!(save.is_mm_xflag_set(40));
        assert!(save.is_mm_xflag_set(50));

        assert_eq!(save.total_set_xflags(), 5);
    }

    #[test]
    fn test_shared_custom_save_clear() {
        let mut save = SharedCustomSave::new();

        save.set_oot_xflag(10);
        save.set_mm_xflag(20);

        save.clear_oot_xflag(10);
        save.clear_mm_xflag(20);

        assert!(!save.is_oot_xflag_set(10));
        assert!(!save.is_mm_xflag_set(20));
        assert_eq!(save.total_set_xflags(), 0);
    }

    #[test]
    fn test_constants() {
        // Verify constants match OoTMM source
        assert_eq!(XFLAGS_COUNT_OOT, 0x2e9); // 745
        assert_eq!(XFLAGS_COUNT_MM, 0x34a); // 842
                                            // Byte sizes are computed to hold all flags: (count + 7) / 8
        assert_eq!(XFLAGS_BYTES_OOT, 94); // ceil(745 / 8) = 94
        assert_eq!(XFLAGS_BYTES_MM, 106); // ceil(842 / 8) = 106
    }

    #[test]
    fn test_xflag_max_indices() {
        // Test that we can access all valid xflag indices
        let mut oot_save = OotCustomSave::new();
        let mut mm_save = MmCustomSave::new();

        // Set the last valid xflag for each game
        oot_save.set_xflag(XFLAGS_COUNT_OOT - 1);
        mm_save.set_xflag(XFLAGS_COUNT_MM - 1);

        assert!(oot_save.is_xflag_set(XFLAGS_COUNT_OOT - 1));
        assert!(mm_save.is_xflag_set(XFLAGS_COUNT_MM - 1));
    }

    #[test]
    fn test_serialization() {
        // Test that the structs can be serialized and deserialized
        let mut save = SharedCustomSave::new();
        save.set_oot_xflag(42);
        save.set_mm_xflag(123);

        let json = serde_json::to_string(&save).expect("Failed to serialize");
        let deserialized: SharedCustomSave =
            serde_json::from_str(&json).expect("Failed to deserialize");

        assert!(deserialized.is_oot_xflag_set(42));
        assert!(deserialized.is_mm_xflag_set(123));
        assert_eq!(save, deserialized);
    }
}
