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

    // ===== Additional AST construction and Display tests =====

    #[test]
    fn test_display_number() {
        assert_eq!(Expr::Number(42).to_string(), "42");
        assert_eq!(Expr::Number(0).to_string(), "0");
        assert_eq!(Expr::Number(-1).to_string(), "-1");
        assert_eq!(Expr::Number(9999999).to_string(), "9999999");
    }

    #[test]
    fn test_display_string() {
        assert_eq!(Expr::String("hello".into()).to_string(), "\"hello\"");
        assert_eq!(Expr::String("".into()).to_string(), "\"\"");
        assert_eq!(
            Expr::String("with spaces".into()).to_string(),
            "\"with spaces\""
        );
    }

    #[test]
    fn test_display_ident() {
        assert_eq!(Expr::Ident("foo".into()).to_string(), "foo");
        assert_eq!(Expr::Ident("is_adult".into()).to_string(), "is_adult");
        assert_eq!(Expr::Ident("HOOKSHOT".into()).to_string(), "HOOKSHOT");
    }

    #[test]
    fn test_display_not() {
        let expr = Expr::not(Expr::Bool(true));
        assert_eq!(expr.to_string(), "!true");

        let expr = Expr::not(Expr::Ident("x".into()));
        assert_eq!(expr.to_string(), "!x");
    }

    #[test]
    fn test_display_double_not() {
        let expr = Expr::not(Expr::not(Expr::Bool(true)));
        assert_eq!(expr.to_string(), "!!true");
    }

    #[test]
    fn test_display_or() {
        let expr = Expr::or(Expr::Bool(true), Expr::Bool(false));
        assert_eq!(expr.to_string(), "(true || false)");
    }

    #[test]
    fn test_display_nested_and_or() {
        let expr = Expr::or(
            Expr::and(Expr::Ident("a".into()), Expr::Ident("b".into())),
            Expr::Ident("c".into()),
        );
        assert_eq!(expr.to_string(), "((a && b) || c)");
    }

    #[test]
    fn test_display_call_no_args() {
        let expr = Expr::call("func", vec![]);
        assert_eq!(expr.to_string(), "func()");
    }

    #[test]
    fn test_display_call_multiple_args() {
        let expr = Expr::call(
            "between",
            vec![Expr::Number(1), Expr::Number(2), Expr::Number(3)],
        );
        assert_eq!(expr.to_string(), "between(1, 2, 3)");
    }

    #[test]
    fn test_display_call_with_string_arg() {
        let expr = Expr::call("message", vec![Expr::String("hello world".into())]);
        assert_eq!(expr.to_string(), "message(\"hello world\")");
    }

    #[test]
    fn test_display_nested_call() {
        let expr = Expr::call(
            "outer",
            vec![Expr::call("inner", vec![Expr::Ident("x".into())])],
        );
        assert_eq!(expr.to_string(), "outer(inner(x))");
    }

    #[test]
    fn test_display_complex_expression() {
        // (has(SWORD) && is_adult) || !has(SLINGSHOT)
        let expr = Expr::or(
            Expr::and(
                Expr::call("has", vec![Expr::Ident("SWORD".into())]),
                Expr::Ident("is_adult".into()),
            ),
            Expr::not(Expr::call("has", vec![Expr::Ident("SLINGSHOT".into())])),
        );
        assert_eq!(
            expr.to_string(),
            "((has(SWORD) && is_adult) || !has(SLINGSHOT))"
        );
    }

    // ===== Equality tests =====

    #[test]
    fn test_equality_bool() {
        assert_eq!(Expr::Bool(true), Expr::Bool(true));
        assert_eq!(Expr::Bool(false), Expr::Bool(false));
        assert_ne!(Expr::Bool(true), Expr::Bool(false));
    }

    #[test]
    fn test_equality_number() {
        assert_eq!(Expr::Number(42), Expr::Number(42));
        assert_ne!(Expr::Number(42), Expr::Number(43));
        assert_ne!(Expr::Number(0), Expr::Number(1));
    }

    #[test]
    fn test_equality_string() {
        assert_eq!(Expr::String("hello".into()), Expr::String("hello".into()));
        assert_ne!(Expr::String("hello".into()), Expr::String("world".into()));
    }

    #[test]
    fn test_equality_ident() {
        assert_eq!(Expr::Ident("foo".into()), Expr::Ident("foo".into()));
        assert_ne!(Expr::Ident("foo".into()), Expr::Ident("bar".into()));
    }

    #[test]
    fn test_equality_and() {
        let expr1 = Expr::and(Expr::Bool(true), Expr::Bool(false));
        let expr2 = Expr::and(Expr::Bool(true), Expr::Bool(false));
        let expr3 = Expr::and(Expr::Bool(false), Expr::Bool(true));
        assert_eq!(expr1, expr2);
        assert_ne!(expr1, expr3);
    }

    #[test]
    fn test_equality_or() {
        let expr1 = Expr::or(Expr::Bool(true), Expr::Bool(false));
        let expr2 = Expr::or(Expr::Bool(true), Expr::Bool(false));
        let expr3 = Expr::or(Expr::Bool(false), Expr::Bool(true));
        assert_eq!(expr1, expr2);
        assert_ne!(expr1, expr3);
    }

    #[test]
    fn test_equality_not() {
        let expr1 = Expr::not(Expr::Bool(true));
        let expr2 = Expr::not(Expr::Bool(true));
        let expr3 = Expr::not(Expr::Bool(false));
        assert_eq!(expr1, expr2);
        assert_ne!(expr1, expr3);
    }

    #[test]
    fn test_equality_call() {
        let expr1 = Expr::call("func", vec![Expr::Number(1)]);
        let expr2 = Expr::call("func", vec![Expr::Number(1)]);
        let expr3 = Expr::call("func", vec![Expr::Number(2)]);
        let expr4 = Expr::call("other", vec![Expr::Number(1)]);
        assert_eq!(expr1, expr2);
        assert_ne!(expr1, expr3);
        assert_ne!(expr1, expr4);
    }

    #[test]
    fn test_inequality_different_variants() {
        assert_ne!(Expr::Bool(true), Expr::Number(1));
        assert_ne!(Expr::Number(0), Expr::Bool(false));
        assert_ne!(Expr::String("true".into()), Expr::Bool(true));
        assert_ne!(Expr::Ident("42".into()), Expr::Number(42));
    }

    // ===== Clone tests =====

    #[test]
    fn test_clone_bool() {
        let expr = Expr::Bool(true);
        let cloned = expr.clone();
        assert_eq!(expr, cloned);
    }

    #[test]
    fn test_clone_number() {
        let expr = Expr::Number(42);
        let cloned = expr.clone();
        assert_eq!(expr, cloned);
    }

    #[test]
    fn test_clone_string() {
        let expr = Expr::String("hello".into());
        let cloned = expr.clone();
        assert_eq!(expr, cloned);
    }

    #[test]
    fn test_clone_ident() {
        let expr = Expr::Ident("foo".into());
        let cloned = expr.clone();
        assert_eq!(expr, cloned);
    }

    #[test]
    fn test_clone_complex_expression() {
        let expr = Expr::or(
            Expr::and(
                Expr::call("has", vec![Expr::Ident("SWORD".into())]),
                Expr::not(Expr::Bool(false)),
            ),
            Expr::call("event", vec![Expr::String("BOSS".into())]),
        );
        let cloned = expr.clone();
        assert_eq!(expr, cloned);
    }

    // ===== Helper constructor tests =====

    #[test]
    fn test_and_constructor_creates_boxed() {
        let result = Expr::and(Expr::Bool(true), Expr::Bool(false));
        match result {
            Expr::And(left, right) => {
                assert_eq!(*left, Expr::Bool(true));
                assert_eq!(*right, Expr::Bool(false));
            }
            _ => panic!("Expected And variant"),
        }
    }

    #[test]
    fn test_or_constructor_creates_boxed() {
        let result = Expr::or(Expr::Bool(true), Expr::Bool(false));
        match result {
            Expr::Or(left, right) => {
                assert_eq!(*left, Expr::Bool(true));
                assert_eq!(*right, Expr::Bool(false));
            }
            _ => panic!("Expected Or variant"),
        }
    }

    #[test]
    fn test_not_constructor_creates_boxed() {
        let result = Expr::not(Expr::Bool(true));
        match result {
            Expr::Not(inner) => {
                assert_eq!(*inner, Expr::Bool(true));
            }
            _ => panic!("Expected Not variant"),
        }
    }

    #[test]
    fn test_call_constructor_with_string_name() {
        let result = Expr::call("my_func".to_string(), vec![Expr::Number(1)]);
        match result {
            Expr::Call { name, args } => {
                assert_eq!(name, "my_func");
                assert_eq!(args.len(), 1);
            }
            _ => panic!("Expected Call variant"),
        }
    }

    #[test]
    fn test_call_constructor_with_str_name() {
        let result = Expr::call("my_func", vec![]);
        match result {
            Expr::Call { name, args } => {
                assert_eq!(name, "my_func");
                assert_eq!(args.len(), 0);
            }
            _ => panic!("Expected Call variant"),
        }
    }

    // ===== Deep nesting tests =====

    #[test]
    fn test_deeply_nested_and() {
        // ((a && b) && c) && d
        let expr = Expr::and(
            Expr::and(
                Expr::and(Expr::Ident("a".into()), Expr::Ident("b".into())),
                Expr::Ident("c".into()),
            ),
            Expr::Ident("d".into()),
        );
        assert_eq!(expr.to_string(), "(((a && b) && c) && d)");
    }

    #[test]
    fn test_deeply_nested_or() {
        // ((a || b) || c) || d
        let expr = Expr::or(
            Expr::or(
                Expr::or(Expr::Ident("a".into()), Expr::Ident("b".into())),
                Expr::Ident("c".into()),
            ),
            Expr::Ident("d".into()),
        );
        assert_eq!(expr.to_string(), "(((a || b) || c) || d)");
    }

    #[test]
    fn test_deeply_nested_not() {
        let expr = Expr::not(Expr::not(Expr::not(Expr::not(Expr::Bool(true)))));
        assert_eq!(expr.to_string(), "!!!!true");
    }

    #[test]
    fn test_deeply_nested_call() {
        let expr = Expr::call(
            "a",
            vec![Expr::call(
                "b",
                vec![Expr::call("c", vec![Expr::Ident("x".into())])],
            )],
        );
        assert_eq!(expr.to_string(), "a(b(c(x)))");
    }
}
