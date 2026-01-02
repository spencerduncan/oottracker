//! Mock traits and implementations for testing.
//!
//! This module provides mockable traits for external dependencies
//! and their mock implementations to facilitate unit testing.

use {
    crate::save::Save,
    std::{collections::HashMap, fmt},
};

/// Error type for mock operations.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MockError {
    /// No data available at the requested offset.
    NoData,
    /// The requested size exceeds available data.
    InsufficientData { available: usize, requested: usize },
    /// Custom error message.
    Custom(String),
}

impl fmt::Display for MockError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MockError::NoData => write!(f, "no data available at the requested offset"),
            MockError::InsufficientData {
                available,
                requested,
            } => write!(
                f,
                "insufficient data: requested {} bytes but only {} available",
                requested, available
            ),
            MockError::Custom(msg) => write!(f, "{}", msg),
        }
    }
}

impl std::error::Error for MockError {}

/// Trait for reading RAM data from an emulator or similar source.
pub trait RamReader {
    /// Reads RAM data at the specified offset.
    ///
    /// # Arguments
    /// * `offset` - The memory offset to read from
    /// * `size` - The number of bytes to read
    ///
    /// # Returns
    /// A vector containing the requested bytes, or an error if the read fails.
    fn read_ram(&self, offset: u32, size: usize) -> Result<Vec<u8>, MockError>;
}

/// Trait for reading save data.
pub trait SaveReader {
    /// Reads the current save state.
    ///
    /// # Returns
    /// The current Save state, or an error if reading fails.
    fn read_save(&self) -> Result<Save, MockError>;
}

/// Mock implementation of `RamReader` for testing.
///
/// Stores RAM data in a HashMap, keyed by offset.
#[derive(Default, Clone)]
pub struct MockRamReader {
    /// Data storage, keyed by offset address.
    pub data: HashMap<u32, Vec<u8>>,
}

impl MockRamReader {
    /// Creates a new empty MockRamReader.
    pub fn new() -> Self {
        Self::default()
    }

    /// Creates a MockRamReader with pre-populated data.
    pub fn with_data(data: HashMap<u32, Vec<u8>>) -> Self {
        Self { data }
    }

    /// Inserts data at the specified offset.
    pub fn insert(&mut self, offset: u32, data: Vec<u8>) -> &mut Self {
        self.data.insert(offset, data);
        self
    }
}

impl RamReader for MockRamReader {
    fn read_ram(&self, offset: u32, size: usize) -> Result<Vec<u8>, MockError> {
        self.data
            .get(&offset)
            .map(|d| {
                if d.len() < size {
                    Err(MockError::InsufficientData {
                        available: d.len(),
                        requested: size,
                    })
                } else {
                    Ok(d[..size].to_vec())
                }
            })
            .unwrap_or(Err(MockError::NoData))
    }
}

/// Mock implementation of `SaveReader` for testing.
#[derive(Default, Clone)]
pub struct MockSaveReader {
    /// The save data to return.
    pub save: Option<Save>,
    /// Optional error to return instead of save data.
    pub error: Option<MockError>,
}

impl MockSaveReader {
    /// Creates a new MockSaveReader with default save data.
    pub fn new() -> Self {
        Self::default()
    }

    /// Creates a MockSaveReader with the specified save data.
    pub fn with_save(save: Save) -> Self {
        Self {
            save: Some(save),
            error: None,
        }
    }

    /// Creates a MockSaveReader that returns an error.
    pub fn with_error(error: MockError) -> Self {
        Self {
            save: None,
            error: Some(error),
        }
    }

    /// Sets the save data to return.
    pub fn set_save(&mut self, save: Save) -> &mut Self {
        self.save = Some(save);
        self.error = None;
        self
    }

    /// Sets an error to return.
    pub fn set_error(&mut self, error: MockError) -> &mut Self {
        self.error = Some(error);
        self
    }
}

impl SaveReader for MockSaveReader {
    fn read_save(&self) -> Result<Save, MockError> {
        if let Some(ref error) = self.error {
            return Err(error.clone());
        }
        self.save.ok_or(MockError::NoData)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mock_ram_reader_read_exact() {
        let mut reader = MockRamReader::new();
        reader.insert(0x1000, vec![0x01, 0x02, 0x03, 0x04]);

        let result = reader.read_ram(0x1000, 4);
        assert_eq!(result, Ok(vec![0x01, 0x02, 0x03, 0x04]));
    }

    #[test]
    fn test_mock_ram_reader_read_partial() {
        let mut reader = MockRamReader::new();
        reader.insert(0x1000, vec![0x01, 0x02, 0x03, 0x04]);

        let result = reader.read_ram(0x1000, 2);
        assert_eq!(result, Ok(vec![0x01, 0x02]));
    }

    #[test]
    fn test_mock_ram_reader_no_data() {
        let reader = MockRamReader::new();

        let result = reader.read_ram(0x1000, 4);
        assert_eq!(result, Err(MockError::NoData));
    }

    #[test]
    fn test_mock_ram_reader_insufficient_data() {
        let mut reader = MockRamReader::new();
        reader.insert(0x1000, vec![0x01, 0x02]);

        let result = reader.read_ram(0x1000, 4);
        assert_eq!(
            result,
            Err(MockError::InsufficientData {
                available: 2,
                requested: 4
            })
        );
    }

    #[test]
    fn test_mock_save_reader_with_save() {
        let save = Save::default();
        let reader = MockSaveReader::with_save(save);

        let result = reader.read_save();
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), save);
    }

    #[test]
    fn test_mock_save_reader_with_error() {
        let reader = MockSaveReader::with_error(MockError::Custom("test error".to_string()));

        let result = reader.read_save();
        assert_eq!(result, Err(MockError::Custom("test error".to_string())));
    }

    #[test]
    fn test_mock_save_reader_no_data() {
        let reader = MockSaveReader::new();

        let result = reader.read_save();
        assert_eq!(result, Err(MockError::NoData));
    }

    #[test]
    fn test_mock_error_display() {
        assert_eq!(
            format!("{}", MockError::NoData),
            "no data available at the requested offset"
        );
        assert_eq!(
            format!(
                "{}",
                MockError::InsufficientData {
                    available: 2,
                    requested: 4
                }
            ),
            "insufficient data: requested 4 bytes but only 2 available"
        );
        assert_eq!(
            format!("{}", MockError::Custom("custom".to_string())),
            "custom"
        );
    }
}
