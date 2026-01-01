//! Unified error types for the ootmm crate.
//! TODO: Implement (Issue #30)

use thiserror::Error;

/// Top-level error type for ootmm operations.
#[derive(Debug, Error)]
pub enum Error {
    #[error("parse error: {0}")]
    Parse(#[from] crate::expr::ParseError),

    #[error("evaluation error: {0}")]
    Eval(#[from] crate::expr::EvalError),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
}
