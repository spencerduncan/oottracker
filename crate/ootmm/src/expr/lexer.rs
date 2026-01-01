//! Lexer for expression parsing.
//! TODO: Implement (Issue #10)

use thiserror::Error;

#[derive(Debug, Error)]
pub enum LexError {
    #[error("unexpected character: {0}")]
    UnexpectedChar(char),
}

pub struct Lexer;
