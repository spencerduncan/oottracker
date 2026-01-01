//! Token types for the expression lexer.
//! TODO: Implement full token handling (Issue #10)

/// Token types produced by the lexer.
#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    // Literals
    True,
    False,
    Number(i64),
    String(String),
    Ident(String),

    // Operators
    And, // &&
    Or,  // ||
    Not, // !

    // Delimiters
    LParen,
    RParen,
    Comma,

    // End of input
    Eof,
}
