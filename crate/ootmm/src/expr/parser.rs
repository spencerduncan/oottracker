//! Expression parser.
//!
//! Implements a recursive descent parser for condition expressions.
//! Operator precedence (lowest to highest): || < && < !

use crate::expr::lexer::{LexError, Lexer};
use crate::expr::{Expr, Token};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ParseError {
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
        let expr = self.parse_or()?;

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
    fn parse_or(&mut self) -> Result<Expr, ParseError> {
        let mut left = self.parse_and()?;

        while self.current == Token::Or {
            self.advance()?;
            let right = self.parse_and()?;
            left = Expr::or(left, right);
        }

        Ok(left)
    }

    /// Parse AND expressions.
    /// and_expr = unary_expr ("&&" unary_expr)*
    fn parse_and(&mut self) -> Result<Expr, ParseError> {
        let mut left = self.parse_unary()?;

        while self.current == Token::And {
            self.advance()?;
            let right = self.parse_unary()?;
            left = Expr::and(left, right);
        }

        Ok(left)
    }

    /// Parse unary expressions (NOT).
    /// unary_expr = "!" unary_expr | primary
    fn parse_unary(&mut self) -> Result<Expr, ParseError> {
        if self.current == Token::Not {
            self.advance()?;
            let expr = self.parse_unary()?;
            return Ok(Expr::not(expr));
        }

        self.parse_primary()
    }

    /// Parse primary expressions (highest precedence).
    /// primary = "true" | "false" | number | string | ident | call | "(" expr ")"
    fn parse_primary(&mut self) -> Result<Expr, ParseError> {
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
                    self.parse_call(name)
                } else {
                    Ok(Expr::Ident(name))
                }
            }
            Token::LParen => {
                self.advance()?;
                let expr = self.parse_or()?;
                self.expect(Token::RParen)?;
                Ok(expr)
            }
            Token::Eof => Err(ParseError::UnexpectedEof),
            token => Err(ParseError::UnexpectedToken(token)),
        }
    }

    /// Parse a function call: name(arg1, arg2, ...)
    fn parse_call(&mut self, name: String) -> Result<Expr, ParseError> {
        self.expect(Token::LParen)?;

        let mut args = Vec::new();

        // Handle empty argument list
        if self.current != Token::RParen {
            // Parse first argument
            args.push(self.parse_or()?);

            // Parse remaining arguments
            while self.current == Token::Comma {
                self.advance()?;
                args.push(self.parse_or()?);
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
        assert_eq!(expr, Expr::call("has", vec![Expr::Ident("Hookshot".into())]));
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
            Expr::and(
                Expr::not(Expr::Ident("a".into())),
                Expr::Ident("b".into())
            )
        );
    }

    #[test]
    fn test_precedence_not_or() {
        // !a || b should parse as (!a) || b
        let expr = parse("!a || b").unwrap();
        assert_eq!(
            expr,
            Expr::or(
                Expr::not(Expr::Ident("a".into())),
                Expr::Ident("b".into())
            )
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
                Expr::and(
                    Expr::Ident("b".into()),
                    Expr::not(Expr::Ident("c".into()))
                )
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
}
