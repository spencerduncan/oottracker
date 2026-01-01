//! Expression parser.
//! TODO: Implement (Issue #12)

use crate::expr::Expr;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ParseError {
    #[error("parse error: {0}")]
    Error(String),
}

pub struct Parser;

/// Parse an expression string into an AST.
pub fn parse(_input: &str) -> Result<Expr, ParseError> {
    todo!("Implement parser")
}
