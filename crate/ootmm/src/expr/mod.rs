//! Expression parser for OoTMM condition logic.
//!
//! This module handles parsing and evaluation of condition expressions like:
//! - `has(HOOKSHOT) && is_adult`
//! - `event(MIDO_MOVED) || setting(skip_child_zelda)`
//! - `between(DAY1_AM_6_00, DAY1_PM_6_00)`

mod ast;
mod context;
mod eval;
mod lexer;
mod mm_context;
mod oot_context;
mod parser;
mod token;

pub mod builtins;

pub use ast::Expr;
pub use context::{Age, GameContext, GameContextBuilder};
pub use eval::{eval, eval_str, EvalContext, EvalError, Evaluator};
pub use lexer::{LexError, Lexer};
pub use mm_context::{MmEvalContext, MmEvalContextBuilder, MmRamReader, MM_SAVE_BASE};
pub use oot_context::{OotEvalContext, OotEvalContextBuilder, OotRamReader, OOT_SAVE_BASE};
pub use parser::{parse, ParseError, Parser};
pub use token::Token;
