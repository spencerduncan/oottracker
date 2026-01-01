//! Expression parser for OoTMM condition logic.
//!
//! This module handles parsing and evaluation of condition expressions like:
//! - `has(HOOKSHOT) && is_adult`
//! - `event(MIDO_MOVED) || setting(skip_child_zelda)`
//! - `between(DAY1_AM_6_00, DAY1_PM_6_00)`

mod ast;
mod eval;
mod lexer;
mod parser;
mod token;

pub mod builtins;

pub use ast::Expr;
pub use eval::{eval, EvalContext, EvalError};
pub use lexer::{LexError, Lexer};
pub use parser::{parse, ParseError, Parser};
pub use token::Token;
