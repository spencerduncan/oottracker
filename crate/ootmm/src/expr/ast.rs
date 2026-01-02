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

    #[test]
    fn test_display_number() {
        assert_eq!(Expr::Number(0).to_string(), "0");
        assert_eq!(Expr::Number(42).to_string(), "42");
        assert_eq!(Expr::Number(-100).to_string(), "-100");
        assert_eq!(Expr::Number(i64::MAX).to_string(), i64::MAX.to_string());
    }

    #[test]
    fn test_display_string() {
        assert_eq!(Expr::String("".into()).to_string(), r#""""#);
        assert_eq!(Expr::String("hello".into()).to_string(), r#""hello""#);
        assert_eq!(
            Expr::String("world with spaces".into()).to_string(),
            r#""world with spaces""#
        );
    }

    #[test]
    fn test_display_ident() {
        assert_eq!(Expr::Ident("x".into()).to_string(), "x");
        assert_eq!(Expr::Ident("HOOKSHOT".into()).to_string(), "HOOKSHOT");
        assert_eq!(
            Expr::Ident("some_variable".into()).to_string(),
            "some_variable"
        );
    }

    #[test]
    fn test_display_or() {
        let expr = Expr::or(Expr::Bool(true), Expr::Bool(false));
        assert_eq!(expr.to_string(), "(true || false)");

        let expr = Expr::or(Expr::Ident("a".into()), Expr::Ident("b".into()));
        assert_eq!(expr.to_string(), "(a || b)");
    }

    #[test]
    fn test_display_not() {
        assert_eq!(Expr::not(Expr::Bool(true)).to_string(), "!true");
        assert_eq!(Expr::not(Expr::Bool(false)).to_string(), "!false");
        assert_eq!(Expr::not(Expr::Ident("x".into())).to_string(), "!x");
    }

    #[test]
    fn test_display_call_variants() {
        // Empty args
        let expr = Expr::call("func", vec![]);
        assert_eq!(expr.to_string(), "func()");

        // Multiple args
        let expr = Expr::call(
            "has_all",
            vec![
                Expr::Ident("SWORD".into()),
                Expr::Ident("SHIELD".into()),
                Expr::Number(3),
            ],
        );
        assert_eq!(expr.to_string(), "has_all(SWORD, SHIELD, 3)");

        // Nested call
        let inner = Expr::call("count", vec![Expr::Ident("KEYS".into())]);
        let outer = Expr::call("gte", vec![inner, Expr::Number(5)]);
        assert_eq!(outer.to_string(), "gte(count(KEYS), 5)");
    }

    #[test]
    fn test_complex_nested_expression() {
        // (has(HOOKSHOT) && !is_child) || can_reach(FOREST_TEMPLE)
        let has_hookshot = Expr::call("has", vec![Expr::Ident("HOOKSHOT".into())]);
        let is_child = Expr::Ident("is_child".into());
        let not_child = Expr::not(is_child);
        let left = Expr::and(has_hookshot, not_child);
        let can_reach = Expr::call("can_reach", vec![Expr::Ident("FOREST_TEMPLE".into())]);
        let full_expr = Expr::or(left, can_reach);

        assert_eq!(
            full_expr.to_string(),
            "((has(HOOKSHOT) && !is_child) || can_reach(FOREST_TEMPLE))"
        );
    }

    #[test]
    fn test_expr_clone() {
        let original = Expr::and(
            Expr::call("has", vec![Expr::Ident("BOW".into())]),
            Expr::Number(10),
        );
        let cloned = original.clone();

        assert_eq!(original, cloned);
        assert_eq!(original.to_string(), cloned.to_string());
    }

    #[test]
    fn test_expr_partial_eq() {
        // Same expressions
        assert_eq!(Expr::Bool(true), Expr::Bool(true));
        assert_eq!(Expr::Number(42), Expr::Number(42));
        assert_eq!(Expr::String("test".into()), Expr::String("test".into()));
        assert_eq!(Expr::Ident("x".into()), Expr::Ident("x".into()));

        // Different expressions
        assert_ne!(Expr::Bool(true), Expr::Bool(false));
        assert_ne!(Expr::Number(1), Expr::Number(2));
        assert_ne!(Expr::String("a".into()), Expr::String("b".into()));
        assert_ne!(Expr::Ident("x".into()), Expr::Ident("y".into()));

        // Different variants
        assert_ne!(Expr::Bool(true), Expr::Number(1));
        assert_ne!(Expr::String("1".into()), Expr::Number(1));

        // Complex expressions
        let expr1 = Expr::and(Expr::Bool(true), Expr::Bool(false));
        let expr2 = Expr::and(Expr::Bool(true), Expr::Bool(false));
        let expr3 = Expr::or(Expr::Bool(true), Expr::Bool(false));
        assert_eq!(expr1, expr2);
        assert_ne!(expr1, expr3);
    }

    #[test]
    fn test_expr_tree_traversal_depth() {
        // Build a deep expression tree: ((a && b) || (c && d))
        let a = Expr::Ident("a".into());
        let b = Expr::Ident("b".into());
        let c = Expr::Ident("c".into());
        let d = Expr::Ident("d".into());

        let ab = Expr::and(a, b);
        let cd = Expr::and(c, d);
        let root = Expr::or(ab, cd);

        // Verify structure through display
        assert_eq!(root.to_string(), "((a && b) || (c && d))");

        // Verify we can match on the structure
        if let Expr::Or(left, right) = &root {
            assert!(matches!(left.as_ref(), Expr::And(_, _)));
            assert!(matches!(right.as_ref(), Expr::And(_, _)));
        } else {
            panic!("Expected Or expression at root");
        }
    }

    #[test]
    fn test_call_struct_fields() {
        let expr = Expr::Call {
            name: "test_func".into(),
            args: vec![Expr::Number(1), Expr::Bool(true)],
        };

        if let Expr::Call { name, args } = expr {
            assert_eq!(name, "test_func");
            assert_eq!(args.len(), 2);
            assert_eq!(args[0], Expr::Number(1));
            assert_eq!(args[1], Expr::Bool(true));
        } else {
            panic!("Expected Call expression");
        }
    }
}
