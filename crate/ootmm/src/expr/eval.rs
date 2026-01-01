//! Expression evaluator.
//! TODO: Implement (Issue #16)

use crate::expr::Expr;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum EvalError {
    #[error("evaluation error: {0}")]
    Error(String),
}

/// Context for expression evaluation.
pub trait EvalContext {
    fn has_item(&self, item: &str, count: u32) -> bool;
    fn event(&self, name: &str) -> bool;
    fn setting(&self, name: &str) -> Option<bool>;
    fn trick(&self, name: &str) -> bool;
    fn is_adult(&self) -> bool;
    fn is_child(&self) -> bool;
}

/// Evaluate an expression against a context.
pub fn eval(_expr: &Expr, _ctx: &impl EvalContext) -> Result<bool, EvalError> {
    todo!("Implement evaluator")
}
