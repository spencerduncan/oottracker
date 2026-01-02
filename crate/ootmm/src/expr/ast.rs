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

    // ===== Number edge cases =====

    #[test]
    fn test_number_edge_cases() {
        assert_eq!(Expr::Number(i64::MIN).to_string(), "-9223372036854775808");
        assert_eq!(Expr::Number(i64::MAX).to_string(), "9223372036854775807");
        assert_eq!(Expr::Number(-999999999).to_string(), "-999999999");
    }

    #[test]
    fn test_equality_number_edge_cases() {
        assert_eq!(Expr::Number(i64::MIN), Expr::Number(i64::MIN));
        assert_eq!(Expr::Number(i64::MAX), Expr::Number(i64::MAX));
        assert_ne!(Expr::Number(i64::MIN), Expr::Number(i64::MAX));
    }

    // ===== String edge cases =====

    #[test]
    fn test_display_string_special_characters() {
        // Note: Display doesn't escape internal quotes (raw display)
        assert_eq!(
            Expr::String("hello\nworld".into()).to_string(),
            "\"hello\nworld\""
        );
        assert_eq!(
            Expr::String("tab\there".into()).to_string(),
            "\"tab\there\""
        );
        assert_eq!(
            Expr::String("back\\slash".into()).to_string(),
            "\"back\\slash\""
        );
    }

    #[test]
    fn test_display_string_with_internal_quotes() {
        assert_eq!(
            Expr::String("say \"hi\"".into()).to_string(),
            "\"say \"hi\"\""
        );
    }

    #[test]
    fn test_string_whitespace_only() {
        assert_eq!(Expr::String("   ".into()).to_string(), "\"   \"");
        assert_eq!(Expr::String("\t\n".into()).to_string(), "\"\t\n\"");
    }

    #[test]
    fn test_equality_string_whitespace() {
        assert_eq!(Expr::String(" ".into()), Expr::String(" ".into()));
        assert_ne!(Expr::String(" ".into()), Expr::String("  ".into()));
        assert_ne!(Expr::String("".into()), Expr::String(" ".into()));
    }

    // ===== Ident edge cases =====

    #[test]
    fn test_display_ident_with_numbers() {
        assert_eq!(Expr::Ident("var1".into()).to_string(), "var1");
        assert_eq!(Expr::Ident("item_42".into()).to_string(), "item_42");
        assert_eq!(Expr::Ident("_private".into()).to_string(), "_private");
    }

    #[test]
    fn test_display_ident_empty() {
        // Edge case: empty identifier (unusual but valid at AST level)
        assert_eq!(Expr::Ident("".into()).to_string(), "");
    }

    #[test]
    fn test_equality_ident_case_sensitive() {
        assert_ne!(Expr::Ident("Foo".into()), Expr::Ident("foo".into()));
        assert_ne!(Expr::Ident("FOO".into()), Expr::Ident("foo".into()));
    }

    // ===== Debug trait tests =====

    #[test]
    fn test_debug_bool() {
        let expr = Expr::Bool(true);
        let debug_str = format!("{:?}", expr);
        assert!(debug_str.contains("Bool"));
        assert!(debug_str.contains("true"));
    }

    #[test]
    fn test_debug_number() {
        let expr = Expr::Number(42);
        let debug_str = format!("{:?}", expr);
        assert!(debug_str.contains("Number"));
        assert!(debug_str.contains("42"));
    }

    #[test]
    fn test_debug_string() {
        let expr = Expr::String("test".into());
        let debug_str = format!("{:?}", expr);
        assert!(debug_str.contains("String"));
        assert!(debug_str.contains("test"));
    }

    #[test]
    fn test_debug_ident() {
        let expr = Expr::Ident("my_var".into());
        let debug_str = format!("{:?}", expr);
        assert!(debug_str.contains("Ident"));
        assert!(debug_str.contains("my_var"));
    }

    #[test]
    fn test_debug_and() {
        let expr = Expr::and(Expr::Bool(true), Expr::Bool(false));
        let debug_str = format!("{:?}", expr);
        assert!(debug_str.contains("And"));
    }

    #[test]
    fn test_debug_or() {
        let expr = Expr::or(Expr::Bool(true), Expr::Bool(false));
        let debug_str = format!("{:?}", expr);
        assert!(debug_str.contains("Or"));
    }

    #[test]
    fn test_debug_not() {
        let expr = Expr::not(Expr::Bool(true));
        let debug_str = format!("{:?}", expr);
        assert!(debug_str.contains("Not"));
    }

    #[test]
    fn test_debug_call() {
        let expr = Expr::call("func", vec![Expr::Number(1)]);
        let debug_str = format!("{:?}", expr);
        assert!(debug_str.contains("Call"));
        assert!(debug_str.contains("func"));
    }

    // ===== Clone independence tests =====

    #[test]
    fn test_clone_string_independence() {
        let original = Expr::String("hello".into());
        let cloned = original.clone();
        // Verify they are equal
        assert_eq!(original, cloned);
        // Verify they have the same display
        assert_eq!(original.to_string(), cloned.to_string());
    }

    #[test]
    fn test_clone_and_independence() {
        let original = Expr::and(Expr::Bool(true), Expr::Ident("x".into()));
        let cloned = original.clone();
        assert_eq!(original, cloned);
        assert_eq!(original.to_string(), cloned.to_string());
    }

    #[test]
    fn test_clone_or_independence() {
        let original = Expr::or(Expr::Number(1), Expr::Number(2));
        let cloned = original.clone();
        assert_eq!(original, cloned);
        assert_eq!(original.to_string(), cloned.to_string());
    }

    #[test]
    fn test_clone_not_independence() {
        let original = Expr::not(Expr::Ident("flag".into()));
        let cloned = original.clone();
        assert_eq!(original, cloned);
        assert_eq!(original.to_string(), cloned.to_string());
    }

    #[test]
    fn test_clone_call_independence() {
        let original = Expr::call("func", vec![Expr::Number(1), Expr::Number(2)]);
        let cloned = original.clone();
        assert_eq!(original, cloned);
        assert_eq!(original.to_string(), cloned.to_string());
    }

    // ===== Call equality edge cases =====

    #[test]
    fn test_equality_call_different_arg_counts() {
        let expr1 = Expr::call("func", vec![Expr::Number(1)]);
        let expr2 = Expr::call("func", vec![Expr::Number(1), Expr::Number(2)]);
        assert_ne!(expr1, expr2);
    }

    #[test]
    fn test_equality_call_empty_vs_nonempty() {
        let expr1 = Expr::call("func", vec![]);
        let expr2 = Expr::call("func", vec![Expr::Bool(true)]);
        assert_ne!(expr1, expr2);
    }

    #[test]
    fn test_equality_call_same_args_different_order() {
        let expr1 = Expr::call("func", vec![Expr::Number(1), Expr::Number(2)]);
        let expr2 = Expr::call("func", vec![Expr::Number(2), Expr::Number(1)]);
        assert_ne!(expr1, expr2);
    }

    // ===== Mixed complex nesting tests =====

    #[test]
    fn test_all_variants_combined() {
        // Build: (has("SWORD") && count(RUPEES, 50)) || !is_adult
        let expr = Expr::or(
            Expr::and(
                Expr::call("has", vec![Expr::String("SWORD".into())]),
                Expr::call(
                    "count",
                    vec![Expr::Ident("RUPEES".into()), Expr::Number(50)],
                ),
            ),
            Expr::not(Expr::Ident("is_adult".into())),
        );
        assert_eq!(
            expr.to_string(),
            "((has(\"SWORD\") && count(RUPEES, 50)) || !is_adult)"
        );
    }

    #[test]
    fn test_not_of_and() {
        let expr = Expr::not(Expr::and(Expr::Bool(true), Expr::Bool(false)));
        assert_eq!(expr.to_string(), "!(true && false)");
    }

    #[test]
    fn test_not_of_or() {
        let expr = Expr::not(Expr::or(Expr::Bool(true), Expr::Bool(false)));
        assert_eq!(expr.to_string(), "!(true || false)");
    }

    #[test]
    fn test_not_of_call() {
        let expr = Expr::not(Expr::call("has", vec![Expr::Ident("ITEM".into())]));
        assert_eq!(expr.to_string(), "!has(ITEM)");
    }

    #[test]
    fn test_call_with_logical_expression_args() {
        let expr = Expr::call(
            "check",
            vec![
                Expr::and(Expr::Bool(true), Expr::Bool(false)),
                Expr::or(Expr::Ident("a".into()), Expr::Ident("b".into())),
            ],
        );
        assert_eq!(expr.to_string(), "check((true && false), (a || b))");
    }

    #[test]
    fn test_and_with_calls_on_both_sides() {
        let expr = Expr::and(
            Expr::call("has", vec![Expr::Ident("A".into())]),
            Expr::call("has", vec![Expr::Ident("B".into())]),
        );
        assert_eq!(expr.to_string(), "(has(A) && has(B))");
    }

    #[test]
    fn test_or_with_not_on_both_sides() {
        let expr = Expr::or(
            Expr::not(Expr::Ident("a".into())),
            Expr::not(Expr::Ident("b".into())),
        );
        assert_eq!(expr.to_string(), "(!a || !b)");
    }

    // ===== Very deep nesting stress tests =====

    #[test]
    fn test_very_deep_and_chain() {
        // Build a chain of 10 ANDs
        let mut expr = Expr::Ident("v0".into());
        for i in 1..10 {
            expr = Expr::and(expr, Expr::Ident(format!("v{}", i)));
        }
        let display = expr.to_string();
        assert!(display.contains("v0"));
        assert!(display.contains("v9"));
        assert!(display.contains("&&"));
    }

    #[test]
    fn test_very_deep_or_chain() {
        // Build a chain of 10 ORs
        let mut expr = Expr::Ident("v0".into());
        for i in 1..10 {
            expr = Expr::or(expr, Expr::Ident(format!("v{}", i)));
        }
        let display = expr.to_string();
        assert!(display.contains("v0"));
        assert!(display.contains("v9"));
        assert!(display.contains("||"));
    }

    #[test]
    fn test_very_deep_not_chain() {
        // Build 10 levels of NOT
        let mut expr = Expr::Bool(true);
        for _ in 0..10 {
            expr = Expr::not(expr);
        }
        assert_eq!(expr.to_string(), "!!!!!!!!!!true");
    }

    #[test]
    fn test_very_deep_call_chain() {
        // Build nested calls: f(f(f(f(f(x)))))
        let mut expr = Expr::Ident("x".into());
        for _ in 0..5 {
            expr = Expr::call("f", vec![expr]);
        }
        assert_eq!(expr.to_string(), "f(f(f(f(f(x)))))");
    }

    #[test]
    fn test_alternating_and_or() {
        // (((a && b) || c) && d) || e
        let expr = Expr::or(
            Expr::and(
                Expr::or(
                    Expr::and(Expr::Ident("a".into()), Expr::Ident("b".into())),
                    Expr::Ident("c".into()),
                ),
                Expr::Ident("d".into()),
            ),
            Expr::Ident("e".into()),
        );
        assert_eq!(expr.to_string(), "((((a && b) || c) && d) || e)");
    }

    // ===== Display consistency tests =====

    #[test]
    fn test_display_consistency_after_clone() {
        let expressions = vec![
            Expr::Bool(true),
            Expr::Number(42),
            Expr::String("test".into()),
            Expr::Ident("foo".into()),
            Expr::and(Expr::Bool(true), Expr::Bool(false)),
            Expr::or(Expr::Bool(true), Expr::Bool(false)),
            Expr::not(Expr::Bool(true)),
            Expr::call("func", vec![Expr::Number(1), Expr::Number(2)]),
        ];

        for expr in expressions {
            let cloned = expr.clone();
            assert_eq!(
                expr.to_string(),
                cloned.to_string(),
                "Display should be identical for {:?}",
                expr
            );
        }
    }

    #[test]
    fn test_equality_reflexive() {
        // Every expression should equal itself
        let expressions = vec![
            Expr::Bool(true),
            Expr::Bool(false),
            Expr::Number(0),
            Expr::Number(i64::MIN),
            Expr::String("".into()),
            Expr::String("test".into()),
            Expr::Ident("x".into()),
            Expr::and(Expr::Bool(true), Expr::Bool(false)),
            Expr::or(Expr::Bool(true), Expr::Bool(false)),
            Expr::not(Expr::Bool(true)),
            Expr::call("f", vec![]),
            Expr::call("f", vec![Expr::Number(1)]),
        ];

        for expr in expressions {
            assert_eq!(expr, expr.clone(), "Expression should equal its clone");
        }
    }

    #[test]
    fn test_equality_symmetric() {
        let a = Expr::and(Expr::Bool(true), Expr::Ident("x".into()));
        let b = Expr::and(Expr::Bool(true), Expr::Ident("x".into()));
        assert_eq!(a, b);
        assert_eq!(b, a);
    }

    // ===== Call with mixed argument types =====

    #[test]
    fn test_call_all_arg_types() {
        let expr = Expr::call(
            "mixed",
            vec![
                Expr::Bool(true),
                Expr::Number(42),
                Expr::String("hello".into()),
                Expr::Ident("var".into()),
            ],
        );
        assert_eq!(expr.to_string(), "mixed(true, 42, \"hello\", var)");
    }

    #[test]
    fn test_call_with_nested_expressions_as_args() {
        let expr = Expr::call(
            "complex",
            vec![
                Expr::and(Expr::Bool(true), Expr::Bool(false)),
                Expr::not(Expr::Ident("x".into())),
                Expr::call("inner", vec![Expr::Number(1)]),
            ],
        );
        assert_eq!(expr.to_string(), "complex((true && false), !x, inner(1))");
    }

    // ===== Structural tests =====

    #[test]
    fn test_and_is_not_commutative_structurally() {
        let expr1 = Expr::and(Expr::Ident("a".into()), Expr::Ident("b".into()));
        let expr2 = Expr::and(Expr::Ident("b".into()), Expr::Ident("a".into()));
        // Structurally different (AST comparison, not logical equivalence)
        assert_ne!(expr1, expr2);
    }

    #[test]
    fn test_or_is_not_commutative_structurally() {
        let expr1 = Expr::or(Expr::Ident("a".into()), Expr::Ident("b".into()));
        let expr2 = Expr::or(Expr::Ident("b".into()), Expr::Ident("a".into()));
        // Structurally different
        assert_ne!(expr1, expr2);
    }

    #[test]
    fn test_deeply_nested_equality() {
        let expr1 = Expr::and(
            Expr::or(
                Expr::not(Expr::Bool(true)),
                Expr::call("f", vec![Expr::Number(1)]),
            ),
            Expr::Ident("x".into()),
        );
        let expr2 = Expr::and(
            Expr::or(
                Expr::not(Expr::Bool(true)),
                Expr::call("f", vec![Expr::Number(1)]),
            ),
            Expr::Ident("x".into()),
        );
        assert_eq!(expr1, expr2);
    }

    #[test]
    fn test_deeply_nested_inequality() {
        let expr1 = Expr::and(
            Expr::or(
                Expr::not(Expr::Bool(true)),
                Expr::call("f", vec![Expr::Number(1)]),
            ),
            Expr::Ident("x".into()),
        );
        let expr2 = Expr::and(
            Expr::or(
                Expr::not(Expr::Bool(false)), // Changed from true to false
                Expr::call("f", vec![Expr::Number(1)]),
            ),
            Expr::Ident("x".into()),
        );
        assert_ne!(expr1, expr2);
    }
}
