//! Abstract Syntax Tree types for expressions.

use std::fmt;

/// Expression AST node.
#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    /// Boolean literal: true, false
    Bool(bool),
    /// Integer literal
    Number(i64),
    /// String literal
    String(String),
    /// Identifier (variable or function name)
    Ident(String),
    /// Logical AND: a && b
    And(Box<Expr>, Box<Expr>),
    /// Logical OR: a || b
    Or(Box<Expr>, Box<Expr>),
    /// Logical NOT: !a
    Not(Box<Expr>),
    /// Function call: name(arg1, arg2, ...)
    Call { name: String, args: Vec<Expr> },
}

impl Expr {
    /// Create a boxed AND expression
    pub fn and(left: Expr, right: Expr) -> Expr {
        Expr::And(Box::new(left), Box::new(right))
    }

    /// Create a boxed OR expression
    pub fn or(left: Expr, right: Expr) -> Expr {
        Expr::Or(Box::new(left), Box::new(right))
    }

    /// Create a boxed NOT expression
    #[allow(clippy::should_implement_trait)]
    pub fn not(expr: Expr) -> Expr {
        Expr::Not(Box::new(expr))
    }

    /// Create a function call
    pub fn call(name: impl Into<String>, args: Vec<Expr>) -> Expr {
        Expr::Call {
            name: name.into(),
            args,
        }
    }
}

impl fmt::Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Expr::Bool(b) => write!(f, "{}", b),
            Expr::Number(n) => write!(f, "{}", n),
            Expr::String(s) => write!(f, "\"{}\"", s),
            Expr::Ident(name) => write!(f, "{}", name),
            Expr::And(left, right) => write!(f, "({} && {})", left, right),
            Expr::Or(left, right) => write!(f, "({} || {})", left, right),
            Expr::Not(expr) => write!(f, "!{}", expr),
            Expr::Call { name, args } => {
                write!(f, "{}(", name)?;
                for (i, arg) in args.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}", arg)?;
                }
                write!(f, ")")
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_display_bool() {
        assert_eq!(Expr::Bool(true).to_string(), "true");
        assert_eq!(Expr::Bool(false).to_string(), "false");
    }

    #[test]
    fn test_display_and_or() {
        let expr = Expr::and(Expr::Bool(true), Expr::Bool(false));
        assert_eq!(expr.to_string(), "(true && false)");
    }

    #[test]
    fn test_display_call() {
        let expr = Expr::call("has", vec![Expr::Ident("HOOKSHOT".into())]);
        assert_eq!(expr.to_string(), "has(HOOKSHOT)");
    }

    #[test]
    fn test_helper_constructors() {
        let _ = Expr::and(Expr::Bool(true), Expr::Bool(false));
        let _ = Expr::or(Expr::Bool(true), Expr::Bool(false));
        let _ = Expr::not(Expr::Bool(true));
    }
}
