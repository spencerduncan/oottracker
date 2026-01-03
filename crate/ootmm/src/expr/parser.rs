//! Expression parser.
//!
//! Implements a recursive descent parser for condition expressions.
//! Operator precedence (lowest to highest): || < && < !

use crate::expr::lexer::{LexError, Lexer};
use crate::expr::{Expr, Token};
use thiserror::Error;

/// Maximum recursion depth allowed when parsing expressions.
/// This prevents stack overflow from deeply nested expressions.
pub const MAX_EXPR_DEPTH: usize = 100;

#[derive(Debug, Error)]
pub enum ParseError {
    #[error("expression too deeply nested (max depth: {0})")]
    TooDeep(usize),
    #[error("unexpected token: {0:?}")]
    UnexpectedToken(Token),
    #[error("unexpected end of input")]
    UnexpectedEof,
    #[error("expected {expected}, found {found:?}")]
    Expected { expected: String, found: Token },
    #[error("lexer error: {0}")]
    LexError(#[from] LexError),
}

pub struct Parser<'a> {
    lexer: Lexer<'a>,
    current: Token,
}

impl<'a> Parser<'a> {
    /// Create a new parser for the given input string.
    pub fn new(input: &'a str) -> Result<Self, ParseError> {
        let mut lexer = Lexer::new(input);
        let current = lexer.next_token()?;
        Ok(Self { lexer, current })
    }

    /// Parse the entire expression.
    pub fn parse(&mut self) -> Result<Expr, ParseError> {
        let expr = self.parse_or(0)?;

        // Ensure we've consumed all input
        if self.current != Token::Eof {
            return Err(ParseError::UnexpectedToken(self.current.clone()));
        }

        Ok(expr)
    }

    /// Advance to the next token.
    fn advance(&mut self) -> Result<(), ParseError> {
        self.current = self.lexer.next_token()?;
        Ok(())
    }

    /// Expect a specific token and advance.
    fn expect(&mut self, expected: Token) -> Result<(), ParseError> {
        if self.current == expected {
            self.advance()?;
            Ok(())
        } else {
            Err(ParseError::Expected {
                expected: format!("{:?}", expected),
                found: self.current.clone(),
            })
        }
    }

    /// Parse OR expressions (lowest precedence).
    /// or_expr = and_expr ("||" and_expr)*
    fn parse_or(&mut self, depth: usize) -> Result<Expr, ParseError> {
        if depth > MAX_EXPR_DEPTH {
            return Err(ParseError::TooDeep(MAX_EXPR_DEPTH));
        }

        let mut left = self.parse_and(depth)?;

        while self.current == Token::Or {
            self.advance()?;
            let right = self.parse_and(depth)?;
            left = Expr::or(left, right);
        }

        Ok(left)
    }

    /// Parse AND expressions.
    /// and_expr = unary_expr ("&&" unary_expr)*
    fn parse_and(&mut self, depth: usize) -> Result<Expr, ParseError> {
        let mut left = self.parse_unary(depth)?;

        while self.current == Token::And {
            self.advance()?;
            let right = self.parse_unary(depth)?;
            left = Expr::and(left, right);
        }

        Ok(left)
    }

    /// Parse unary expressions (NOT).
    /// unary_expr = "!" unary_expr | primary
    fn parse_unary(&mut self, depth: usize) -> Result<Expr, ParseError> {
        if depth > MAX_EXPR_DEPTH {
            return Err(ParseError::TooDeep(MAX_EXPR_DEPTH));
        }

        if self.current == Token::Not {
            self.advance()?;
            let expr = self.parse_unary(depth + 1)?;
            return Ok(Expr::not(expr));
        }

        self.parse_primary(depth)
    }

    /// Parse primary expressions (highest precedence).
    /// primary = "true" | "false" | number | string | ident | call | "(" expr ")"
    fn parse_primary(&mut self, depth: usize) -> Result<Expr, ParseError> {
        match self.current.clone() {
            Token::True => {
                self.advance()?;
                Ok(Expr::Bool(true))
            }
            Token::False => {
                self.advance()?;
                Ok(Expr::Bool(false))
            }
            Token::Number(n) => {
                self.advance()?;
                Ok(Expr::Number(n))
            }
            Token::String(s) => {
                self.advance()?;
                Ok(Expr::String(s))
            }
            Token::Ident(name) => {
                self.advance()?;
                // Check if this is a function call
                if self.current == Token::LParen {
                    self.parse_call(name, depth)
                } else {
                    Ok(Expr::Ident(name))
                }
            }
            Token::LParen => {
                self.advance()?;
                let expr = self.parse_or(depth + 1)?;
                self.expect(Token::RParen)?;
                Ok(expr)
            }
            Token::Eof => Err(ParseError::UnexpectedEof),
            token => Err(ParseError::UnexpectedToken(token)),
        }
    }

    /// Parse a function call: name(arg1, arg2, ...)
    fn parse_call(&mut self, name: String, depth: usize) -> Result<Expr, ParseError> {
        self.expect(Token::LParen)?;

        let mut args = Vec::new();

        // Handle empty argument list
        if self.current != Token::RParen {
            // Parse first argument
            args.push(self.parse_or(depth + 1)?);

            // Parse remaining arguments
            while self.current == Token::Comma {
                self.advance()?;
                args.push(self.parse_or(depth + 1)?);
            }
        }

        self.expect(Token::RParen)?;

        Ok(Expr::call(name, args))
    }
}

/// Parse an expression string into an AST.
pub fn parse(input: &str) -> Result<Expr, ParseError> {
    let mut parser = Parser::new(input)?;
    parser.parse()
}

#[cfg(test)]
mod tests {
    use super::*;

    // --- Simple literals ---

    #[test]
    fn test_parse_true() {
        let expr = parse("true").unwrap();
        assert_eq!(expr, Expr::Bool(true));
    }

    #[test]
    fn test_parse_false() {
        let expr = parse("false").unwrap();
        assert_eq!(expr, Expr::Bool(false));
    }

    #[test]
    fn test_parse_number() {
        let expr = parse("42").unwrap();
        assert_eq!(expr, Expr::Number(42));
    }

    #[test]
    fn test_parse_string() {
        let expr = parse("\"hello\"").unwrap();
        assert_eq!(expr, Expr::String("hello".into()));
    }

    #[test]
    fn test_parse_identifier() {
        let expr = parse("is_adult").unwrap();
        assert_eq!(expr, Expr::Ident("is_adult".into()));
    }

    // --- Binary operators ---

    #[test]
    fn test_parse_and() {
        let expr = parse("a && b").unwrap();
        assert_eq!(
            expr,
            Expr::and(Expr::Ident("a".into()), Expr::Ident("b".into()))
        );
    }

    #[test]
    fn test_parse_or() {
        let expr = parse("a || b").unwrap();
        assert_eq!(
            expr,
            Expr::or(Expr::Ident("a".into()), Expr::Ident("b".into()))
        );
    }

    #[test]
    fn test_parse_multiple_and() {
        let expr = parse("a && b && c").unwrap();
        // Left-associative: (a && b) && c
        assert_eq!(
            expr,
            Expr::and(
                Expr::and(Expr::Ident("a".into()), Expr::Ident("b".into())),
                Expr::Ident("c".into())
            )
        );
    }

    #[test]
    fn test_parse_multiple_or() {
        let expr = parse("a || b || c").unwrap();
        // Left-associative: (a || b) || c
        assert_eq!(
            expr,
            Expr::or(
                Expr::or(Expr::Ident("a".into()), Expr::Ident("b".into())),
                Expr::Ident("c".into())
            )
        );
    }

    // --- Unary operator ---

    #[test]
    fn test_parse_not() {
        let expr = parse("!a").unwrap();
        assert_eq!(expr, Expr::not(Expr::Ident("a".into())));
    }

    #[test]
    fn test_parse_double_not() {
        let expr = parse("!!a").unwrap();
        assert_eq!(expr, Expr::not(Expr::not(Expr::Ident("a".into()))));
    }

    // --- Function calls ---

    #[test]
    fn test_parse_function_no_args() {
        let expr = parse("func()").unwrap();
        assert_eq!(expr, Expr::call("func", vec![]));
    }

    #[test]
    fn test_parse_function_one_arg() {
        let expr = parse("has(Hookshot)").unwrap();
        assert_eq!(
            expr,
            Expr::call("has", vec![Expr::Ident("Hookshot".into())])
        );
    }

    #[test]
    fn test_parse_function_multiple_args() {
        let expr = parse("between(1, 2, 3)").unwrap();
        assert_eq!(
            expr,
            Expr::call(
                "between",
                vec![Expr::Number(1), Expr::Number(2), Expr::Number(3)]
            )
        );
    }

    #[test]
    fn test_parse_function_with_expr_arg() {
        let expr = parse("func(a && b)").unwrap();
        assert_eq!(
            expr,
            Expr::call(
                "func",
                vec![Expr::and(Expr::Ident("a".into()), Expr::Ident("b".into()))]
            )
        );
    }

    // --- Nested expressions ---

    #[test]
    fn test_parse_parentheses() {
        let expr = parse("(a)").unwrap();
        assert_eq!(expr, Expr::Ident("a".into()));
    }

    #[test]
    fn test_parse_nested_parentheses() {
        let expr = parse("((a))").unwrap();
        assert_eq!(expr, Expr::Ident("a".into()));
    }

    #[test]
    fn test_parse_complex_nested() {
        let expr = parse("(a && b) || (c && d)").unwrap();
        assert_eq!(
            expr,
            Expr::or(
                Expr::and(Expr::Ident("a".into()), Expr::Ident("b".into())),
                Expr::and(Expr::Ident("c".into()), Expr::Ident("d".into()))
            )
        );
    }

    // --- Operator precedence ---

    #[test]
    fn test_precedence_and_over_or() {
        // a || b && c should parse as a || (b && c)
        let expr = parse("a || b && c").unwrap();
        assert_eq!(
            expr,
            Expr::or(
                Expr::Ident("a".into()),
                Expr::and(Expr::Ident("b".into()), Expr::Ident("c".into()))
            )
        );
    }

    #[test]
    fn test_precedence_not_highest() {
        // !a && b should parse as (!a) && b
        let expr = parse("!a && b").unwrap();
        assert_eq!(
            expr,
            Expr::and(Expr::not(Expr::Ident("a".into())), Expr::Ident("b".into()))
        );
    }

    #[test]
    fn test_precedence_not_or() {
        // !a || b should parse as (!a) || b
        let expr = parse("!a || b").unwrap();
        assert_eq!(
            expr,
            Expr::or(Expr::not(Expr::Ident("a".into())), Expr::Ident("b".into()))
        );
    }

    #[test]
    fn test_precedence_complex() {
        // a || b && !c should parse as a || (b && (!c))
        let expr = parse("a || b && !c").unwrap();
        assert_eq!(
            expr,
            Expr::or(
                Expr::Ident("a".into()),
                Expr::and(Expr::Ident("b".into()), Expr::not(Expr::Ident("c".into())))
            )
        );
    }

    #[test]
    fn test_parentheses_override_precedence() {
        // (a || b) && c should group as written
        let expr = parse("(a || b) && c").unwrap();
        assert_eq!(
            expr,
            Expr::and(
                Expr::or(Expr::Ident("a".into()), Expr::Ident("b".into())),
                Expr::Ident("c".into())
            )
        );
    }

    // --- Real-world examples ---

    #[test]
    fn test_parse_real_expression() {
        let expr = parse("is_child && has(HOOKSHOT)").unwrap();
        assert_eq!(
            expr,
            Expr::and(
                Expr::Ident("is_child".into()),
                Expr::call("has", vec![Expr::Ident("HOOKSHOT".into())])
            )
        );
    }

    #[test]
    fn test_parse_complex_real_expression() {
        let expr = parse("event(MIDO_MOVED) || setting(skip_child_zelda)").unwrap();
        assert_eq!(
            expr,
            Expr::or(
                Expr::call("event", vec![Expr::Ident("MIDO_MOVED".into())]),
                Expr::call("setting", vec![Expr::Ident("skip_child_zelda".into())])
            )
        );
    }

    // --- Error cases ---

    #[test]
    fn test_parse_empty_input() {
        let result = parse("");
        assert!(matches!(result, Err(ParseError::UnexpectedEof)));
    }

    #[test]
    fn test_parse_unexpected_token() {
        let result = parse("&&");
        assert!(matches!(result, Err(ParseError::UnexpectedToken(_))));
    }

    #[test]
    fn test_parse_unclosed_paren() {
        let result = parse("(a");
        assert!(matches!(result, Err(ParseError::Expected { .. })));
    }

    #[test]
    fn test_parse_extra_tokens() {
        let result = parse("a b");
        assert!(matches!(result, Err(ParseError::UnexpectedToken(_))));
    }

    #[test]
    fn test_parse_missing_operand() {
        let result = parse("a &&");
        assert!(matches!(result, Err(ParseError::UnexpectedEof)));
    }

    // ===== Additional parser error handling tests =====

    #[test]
    fn test_parse_missing_operand_or() {
        let result = parse("a ||");
        assert!(matches!(result, Err(ParseError::UnexpectedEof)));
    }

    #[test]
    fn test_parse_not_without_operand() {
        let result = parse("!");
        assert!(matches!(result, Err(ParseError::UnexpectedEof)));
    }

    #[test]
    fn test_parse_unclosed_function_call() {
        let result = parse("func(a, b");
        assert!(matches!(result, Err(ParseError::Expected { .. })));
    }

    #[test]
    fn test_parse_missing_comma_in_args() {
        let result = parse("func(a b)");
        assert!(matches!(result, Err(ParseError::Expected { .. })));
    }

    #[test]
    fn test_parse_extra_closing_paren() {
        let result = parse("(a))");
        assert!(matches!(result, Err(ParseError::UnexpectedToken(_))));
    }

    #[test]
    fn test_parse_empty_parens_as_expr() {
        let result = parse("()");
        assert!(matches!(result, Err(ParseError::UnexpectedToken(_))));
    }

    #[test]
    fn test_parse_double_operator_and() {
        let result = parse("a && && b");
        assert!(matches!(result, Err(ParseError::UnexpectedToken(_))));
    }

    #[test]
    fn test_parse_double_operator_or() {
        let result = parse("a || || b");
        assert!(matches!(result, Err(ParseError::UnexpectedToken(_))));
    }

    #[test]
    fn test_parse_trailing_comma_in_func() {
        let result = parse("func(a,)");
        assert!(matches!(result, Err(ParseError::UnexpectedToken(_))));
    }

    #[test]
    fn test_parse_leading_comma_in_func() {
        let result = parse("func(,a)");
        assert!(matches!(result, Err(ParseError::UnexpectedToken(_))));
    }

    #[test]
    fn test_parse_standalone_comma() {
        let result = parse(",");
        assert!(matches!(result, Err(ParseError::UnexpectedToken(_))));
    }

    #[test]
    fn test_parse_operator_at_start() {
        let result = parse("|| a");
        assert!(matches!(result, Err(ParseError::UnexpectedToken(_))));
    }

    #[test]
    fn test_parse_deeply_nested_unclosed() {
        let result = parse("((((a");
        assert!(matches!(result, Err(ParseError::Expected { .. })));
    }

    #[test]
    fn test_parse_mismatched_parens() {
        let result = parse("(a && b))");
        assert!(matches!(result, Err(ParseError::UnexpectedToken(_))));
    }

    // ===== Additional parser success cases =====

    #[test]
    fn test_parse_triple_not() {
        let expr = parse("!!!a").unwrap();
        assert_eq!(
            expr,
            Expr::not(Expr::not(Expr::not(Expr::Ident("a".into()))))
        );
    }

    #[test]
    fn test_parse_not_in_parentheses() {
        let expr = parse("!(a)").unwrap();
        assert_eq!(expr, Expr::not(Expr::Ident("a".into())));
    }

    #[test]
    fn test_parse_function_with_nested_function_arg() {
        let expr = parse("outer(inner(x))").unwrap();
        assert_eq!(
            expr,
            Expr::call(
                "outer",
                vec![Expr::call("inner", vec![Expr::Ident("x".into())])]
            )
        );
    }

    #[test]
    fn test_parse_function_with_multiple_nested_args() {
        let expr = parse("f(a(x), b(y))").unwrap();
        assert_eq!(
            expr,
            Expr::call(
                "f",
                vec![
                    Expr::call("a", vec![Expr::Ident("x".into())]),
                    Expr::call("b", vec![Expr::Ident("y".into())])
                ]
            )
        );
    }

    #[test]
    fn test_parse_function_with_logical_expr_arg() {
        let expr = parse("check(a || b && c)").unwrap();
        assert_eq!(
            expr,
            Expr::call(
                "check",
                vec![Expr::or(
                    Expr::Ident("a".into()),
                    Expr::and(Expr::Ident("b".into()), Expr::Ident("c".into()))
                )]
            )
        );
    }

    #[test]
    fn test_parse_mixed_literals_in_call() {
        let expr = parse("func(true, 42, \"hello\", x)").unwrap();
        assert_eq!(
            expr,
            Expr::call(
                "func",
                vec![
                    Expr::Bool(true),
                    Expr::Number(42),
                    Expr::String("hello".into()),
                    Expr::Ident("x".into())
                ]
            )
        );
    }

    #[test]
    fn test_parse_complex_precedence_chain() {
        // a || b && c || d && e should parse as (a || (b && c)) || (d && e)
        let expr = parse("a || b && c || d && e").unwrap();
        assert_eq!(
            expr,
            Expr::or(
                Expr::or(
                    Expr::Ident("a".into()),
                    Expr::and(Expr::Ident("b".into()), Expr::Ident("c".into()))
                ),
                Expr::and(Expr::Ident("d".into()), Expr::Ident("e".into()))
            )
        );
    }

    #[test]
    fn test_parse_not_with_function_call() {
        let expr = parse("!has(ITEM)").unwrap();
        assert_eq!(
            expr,
            Expr::not(Expr::call("has", vec![Expr::Ident("ITEM".into())]))
        );
    }

    #[test]
    fn test_parse_number_zero() {
        let expr = parse("0").unwrap();
        assert_eq!(expr, Expr::Number(0));
    }

    #[test]
    fn test_parse_empty_string() {
        let expr = parse("\"\"").unwrap();
        assert_eq!(expr, Expr::String("".into()));
    }

    #[test]
    fn test_parse_string_with_spaces() {
        let expr = parse("\"hello world\"").unwrap();
        assert_eq!(expr, Expr::String("hello world".into()));
    }

    #[test]
    fn test_parse_deeply_nested_parens() {
        let expr = parse("((((a))))").unwrap();
        assert_eq!(expr, Expr::Ident("a".into()));
    }

    #[test]
    fn test_parse_four_way_and() {
        let expr = parse("a && b && c && d").unwrap();
        // Left-associative: ((a && b) && c) && d
        assert_eq!(
            expr,
            Expr::and(
                Expr::and(
                    Expr::and(Expr::Ident("a".into()), Expr::Ident("b".into())),
                    Expr::Ident("c".into())
                ),
                Expr::Ident("d".into())
            )
        );
    }

    #[test]
    fn test_parse_four_way_or() {
        let expr = parse("a || b || c || d").unwrap();
        // Left-associative: ((a || b) || c) || d
        assert_eq!(
            expr,
            Expr::or(
                Expr::or(
                    Expr::or(Expr::Ident("a".into()), Expr::Ident("b".into())),
                    Expr::Ident("c".into())
                ),
                Expr::Ident("d".into())
            )
        );
    }

    #[test]
    fn test_parse_function_with_five_args() {
        let expr = parse("f(1, 2, 3, 4, 5)").unwrap();
        assert_eq!(
            expr,
            Expr::call(
                "f",
                vec![
                    Expr::Number(1),
                    Expr::Number(2),
                    Expr::Number(3),
                    Expr::Number(4),
                    Expr::Number(5)
                ]
            )
        );
    }

    #[test]
    fn test_parse_lex_error_propagation() {
        // Lexer error should be wrapped in ParseError::LexError
        let result = parse("a @ b");
        assert!(matches!(result, Err(ParseError::LexError(_))));
    }

    // --- Depth limit tests ---

    #[test]
    fn test_parse_depth_limit_nested_parens() {
        // Create an expression with depth > MAX_EXPR_DEPTH using nested parentheses
        let open_parens = "(".repeat(MAX_EXPR_DEPTH + 10);
        let close_parens = ")".repeat(MAX_EXPR_DEPTH + 10);
        let deep_expr = format!("{}a{}", open_parens, close_parens);

        let result = parse(&deep_expr);
        assert!(matches!(result, Err(ParseError::TooDeep(_))));
    }

    #[test]
    fn test_parse_depth_limit_nested_not() {
        // Create an expression with depth > MAX_EXPR_DEPTH using chained NOT operators
        let nots = "!".repeat(MAX_EXPR_DEPTH + 10);
        let deep_expr = format!("{}a", nots);

        let result = parse(&deep_expr);
        assert!(matches!(result, Err(ParseError::TooDeep(_))));
    }

    #[test]
    fn test_parse_depth_limit_nested_calls() {
        // Create deeply nested function calls: f(f(f(...f(a)...)))
        let mut expr = "a".to_string();
        for _ in 0..(MAX_EXPR_DEPTH + 10) {
            expr = format!("f({})", expr);
        }

        let result = parse(&expr);
        assert!(matches!(result, Err(ParseError::TooDeep(_))));
    }

    #[test]
    fn test_parse_depth_at_limit_succeeds() {
        // An expression at exactly MAX_EXPR_DEPTH should succeed
        // Using depth - 1 nested parens since depth starts at 0
        let open_parens = "(".repeat(MAX_EXPR_DEPTH);
        let close_parens = ")".repeat(MAX_EXPR_DEPTH);
        let expr = format!("{}a{}", open_parens, close_parens);

        let result = parse(&expr);
        assert!(result.is_ok());
    }

    #[test]
    fn test_parse_depth_limit_error_message() {
        let nots = "!".repeat(MAX_EXPR_DEPTH + 10);
        let deep_expr = format!("{}a", nots);

        let result = parse(&deep_expr);
        match result {
            Err(ParseError::TooDeep(depth)) => {
                assert_eq!(depth, MAX_EXPR_DEPTH);
            }
            _ => panic!("expected TooDeep error"),
        }
    }

    #[test]
    fn test_parse_depth_mixed_nesting() {
        // Mix parentheses and NOT operators to exceed depth
        let mut expr = "a".to_string();
        for i in 0..(MAX_EXPR_DEPTH + 10) {
            if i % 2 == 0 {
                expr = format!("({})", expr);
            } else {
                expr = format!("!{}", expr);
            }
        }

        let result = parse(&expr);
        assert!(matches!(result, Err(ParseError::TooDeep(_))));
    }
}
