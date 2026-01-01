//! Unified error types for the ootmm crate.

use thiserror::Error;

/// Top-level error type for ootmm operations.
#[derive(Debug, Error)]
pub enum Error {
    /// Error parsing an expression.
    #[error("parse error: {0}")]
    Parse(#[from] crate::expr::ParseError),

    /// Error evaluating an expression.
    #[error("evaluation error: {0}")]
    Eval(#[from] crate::expr::EvalError),

    /// IO error.
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    /// Item not found in the item database.
    #[error("item not found: {0}")]
    ItemNotFound(String),

    /// Region not found in the world graph.
    #[error("region not found: {0}")]
    RegionNotFound(String),

    /// Invalid logic expression encountered.
    #[error("invalid logic expression: {0}")]
    InvalidLogicExpression(String),

    /// YAML parsing error.
    #[error("YAML parse error: {0}")]
    YamlParse(#[from] serde_yaml::Error),

    /// Error loading world data.
    #[error("world load error: {message}")]
    WorldLoad {
        /// Description of the world loading failure.
        message: String,
        /// Optional underlying cause.
        #[source]
        source: Option<Box<dyn std::error::Error + Send + Sync>>,
    },
}

/// A specialized Result type for ootmm operations.
pub type Result<T> = std::result::Result<T, Error>;

impl Error {
    /// Creates an `ItemNotFound` error.
    pub fn item_not_found(item: impl Into<String>) -> Self {
        Self::ItemNotFound(item.into())
    }

    /// Creates a `RegionNotFound` error.
    pub fn region_not_found(region: impl Into<String>) -> Self {
        Self::RegionNotFound(region.into())
    }

    /// Creates an `InvalidLogicExpression` error.
    pub fn invalid_logic(expr: impl Into<String>) -> Self {
        Self::InvalidLogicExpression(expr.into())
    }

    /// Creates a `WorldLoad` error with just a message.
    pub fn world_load(message: impl Into<String>) -> Self {
        Self::WorldLoad {
            message: message.into(),
            source: None,
        }
    }

    /// Creates a `WorldLoad` error with a message and source error.
    pub fn world_load_with_source(
        message: impl Into<String>,
        source: impl std::error::Error + Send + Sync + 'static,
    ) -> Self {
        Self::WorldLoad {
            message: message.into(),
            source: Some(Box::new(source)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::error::Error as StdError;

    #[test]
    fn test_item_not_found_error() {
        let err = Error::item_not_found("HOOKSHOT");
        assert_eq!(err.to_string(), "item not found: HOOKSHOT");
    }

    #[test]
    fn test_region_not_found_error() {
        let err = Error::region_not_found("Kokiri Forest");
        assert_eq!(err.to_string(), "region not found: Kokiri Forest");
    }

    #[test]
    fn test_invalid_logic_error() {
        let err = Error::invalid_logic("has(HOOKSHOT) &&");
        assert_eq!(
            err.to_string(),
            "invalid logic expression: has(HOOKSHOT) &&"
        );
    }

    #[test]
    fn test_world_load_error() {
        let err = Error::world_load("failed to load dungeon data");
        assert_eq!(
            err.to_string(),
            "world load error: failed to load dungeon data"
        );
    }

    #[test]
    fn test_world_load_with_source() {
        let io_err = std::io::Error::new(std::io::ErrorKind::NotFound, "file not found");
        let err = Error::world_load_with_source("failed to read world file", io_err);
        assert_eq!(
            err.to_string(),
            "world load error: failed to read world file"
        );
        assert!(StdError::source(&err).is_some());
    }

    #[test]
    fn test_io_error_conversion() {
        let io_err = std::io::Error::new(std::io::ErrorKind::NotFound, "not found");
        let err: Error = io_err.into();
        assert!(matches!(err, Error::Io(_)));
    }
}
