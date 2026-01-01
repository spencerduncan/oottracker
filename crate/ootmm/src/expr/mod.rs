//! Expression parser for OoTMM condition logic.
//!
//! This module handles parsing and evaluation of condition expressions like:
//! - `has(HOOKSHOT) && is_adult`
//! - `event(MIDO_MOVED) || setting(skip_child_zelda)`
//! - `between(DAY1_AM_6_00, DAY1_PM_6_00)`

mod token;
mod lexer;
mod ast;
mod parser;
mod eval;

pub mod builtins;

pub use token::Token;
pub use lexer::{Lexer, LexError};
pub use ast::Expr;
pub use parser::{Parser, ParseError, parse};
pub use eval::{EvalContext, EvalError, eval};
