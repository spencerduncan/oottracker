//! Lexer for expression parsing.

use crate::expr::Token;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum LexError {
    #[error("unexpected character '{0}' at position {1}")]
    UnexpectedChar(char, usize),
    #[error("unterminated string at position {0}")]
    UnterminatedString(usize),
}

pub struct Lexer<'a> {
    input: &'a str,
    pos: usize,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        Self { input, pos: 0 }
    }

    pub fn next_token(&mut self) -> Result<Token, LexError> {
        self.skip_whitespace();

        if self.pos >= self.input.len() {
            return Ok(Token::Eof);
        }

        let bytes = self.input.as_bytes();
        let ch = bytes[self.pos] as char;

        // Single-character tokens
        match ch {
            '(' => {
                self.pos += 1;
                return Ok(Token::LParen);
            }
            ')' => {
                self.pos += 1;
                return Ok(Token::RParen);
            }
            ',' => {
                self.pos += 1;
                return Ok(Token::Comma);
            }
            '!' => {
                self.pos += 1;
                return Ok(Token::Not);
            }
            _ => {}
        }

        // Two-character operators
        if ch == '&' && self.pos + 1 < self.input.len() && bytes[self.pos + 1] == b'&' {
            self.pos += 2;
            return Ok(Token::And);
        }
        if ch == '|' && self.pos + 1 < self.input.len() && bytes[self.pos + 1] == b'|' {
            self.pos += 2;
            return Ok(Token::Or);
        }

        // String literals
        if ch == '"' {
            return self.read_string();
        }

        // Numbers
        if ch.is_ascii_digit() {
            return Ok(self.read_number());
        }

        // Identifiers and keywords
        if ch.is_ascii_alphabetic() || ch == '_' {
            return Ok(self.read_identifier());
        }

        Err(LexError::UnexpectedChar(ch, self.pos))
    }

    fn skip_whitespace(&mut self) {
        while self.pos < self.input.len() && self.input.as_bytes()[self.pos].is_ascii_whitespace() {
            self.pos += 1;
        }
    }

    fn read_string(&mut self) -> Result<Token, LexError> {
        let start_pos = self.pos;
        self.pos += 1; // Skip opening quote

        let mut value = String::new();
        while self.pos < self.input.len() {
            let ch = self.input.as_bytes()[self.pos] as char;
            if ch == '"' {
                self.pos += 1; // Skip closing quote
                return Ok(Token::String(value));
            }
            value.push(ch);
            self.pos += 1;
        }

        Err(LexError::UnterminatedString(start_pos))
    }

    fn read_number(&mut self) -> Token {
        let start = self.pos;
        while self.pos < self.input.len() && self.input.as_bytes()[self.pos].is_ascii_digit() {
            self.pos += 1;
        }
        let num_str = &self.input[start..self.pos];
        Token::Number(num_str.parse().unwrap())
    }

    fn read_identifier(&mut self) -> Token {
        let start = self.pos;
        while self.pos < self.input.len() {
            let ch = self.input.as_bytes()[self.pos] as char;
            if ch.is_ascii_alphanumeric() || ch == '_' {
                self.pos += 1;
            } else {
                break;
            }
        }
        let ident = &self.input[start..self.pos];

        // Check for keywords
        match ident {
            "true" => Token::True,
            "false" => Token::False,
            _ => Token::Ident(ident.to_string()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_keywords() {
        let mut lexer = Lexer::new("true false");
        assert_eq!(lexer.next_token().unwrap(), Token::True);
        assert_eq!(lexer.next_token().unwrap(), Token::False);
    }

    #[test]
    fn test_expression() {
        let mut lexer = Lexer::new("has(ITEM)");
        assert_eq!(lexer.next_token().unwrap(), Token::Ident("has".into()));
        assert_eq!(lexer.next_token().unwrap(), Token::LParen);
        assert_eq!(lexer.next_token().unwrap(), Token::Ident("ITEM".into()));
        assert_eq!(lexer.next_token().unwrap(), Token::RParen);
    }

    #[test]
    fn test_operators() {
        let mut lexer = Lexer::new("&& || !");
        assert_eq!(lexer.next_token().unwrap(), Token::And);
        assert_eq!(lexer.next_token().unwrap(), Token::Or);
        assert_eq!(lexer.next_token().unwrap(), Token::Not);
    }

    #[test]
    fn test_numbers() {
        let mut lexer = Lexer::new("42 123");
        assert_eq!(lexer.next_token().unwrap(), Token::Number(42));
        assert_eq!(lexer.next_token().unwrap(), Token::Number(123));
    }

    #[test]
    fn test_strings() {
        let mut lexer = Lexer::new("\"hello\" \"world\"");
        assert_eq!(lexer.next_token().unwrap(), Token::String("hello".into()));
        assert_eq!(lexer.next_token().unwrap(), Token::String("world".into()));
    }

    #[test]
    fn test_complex_expression() {
        let mut lexer = Lexer::new("is_child && has(HOOKSHOT)");
        assert_eq!(lexer.next_token().unwrap(), Token::Ident("is_child".into()));
        assert_eq!(lexer.next_token().unwrap(), Token::And);
        assert_eq!(lexer.next_token().unwrap(), Token::Ident("has".into()));
        assert_eq!(lexer.next_token().unwrap(), Token::LParen);
        assert_eq!(lexer.next_token().unwrap(), Token::Ident("HOOKSHOT".into()));
        assert_eq!(lexer.next_token().unwrap(), Token::RParen);
        assert_eq!(lexer.next_token().unwrap(), Token::Eof);
    }

    #[test]
    fn test_comma() {
        let mut lexer = Lexer::new("func(a, b)");
        assert_eq!(lexer.next_token().unwrap(), Token::Ident("func".into()));
        assert_eq!(lexer.next_token().unwrap(), Token::LParen);
        assert_eq!(lexer.next_token().unwrap(), Token::Ident("a".into()));
        assert_eq!(lexer.next_token().unwrap(), Token::Comma);
        assert_eq!(lexer.next_token().unwrap(), Token::Ident("b".into()));
        assert_eq!(lexer.next_token().unwrap(), Token::RParen);
    }

    #[test]
    fn test_eof() {
        let mut lexer = Lexer::new("");
        assert_eq!(lexer.next_token().unwrap(), Token::Eof);
    }

    #[test]
    fn test_unterminated_string() {
        let mut lexer = Lexer::new("\"unterminated");
        assert!(matches!(
            lexer.next_token(),
            Err(LexError::UnterminatedString(_))
        ));
    }

    #[test]
    fn test_unexpected_char() {
        let mut lexer = Lexer::new("@");
        assert!(matches!(
            lexer.next_token(),
            Err(LexError::UnexpectedChar('@', 0))
        ));
    }

    // ===== Additional tests for comprehensive coverage =====

    #[test]
    fn test_whitespace_handling_tabs() {
        let mut lexer = Lexer::new("true\t\tfalse");
        assert_eq!(lexer.next_token().unwrap(), Token::True);
        assert_eq!(lexer.next_token().unwrap(), Token::False);
        assert_eq!(lexer.next_token().unwrap(), Token::Eof);
    }

    #[test]
    fn test_whitespace_handling_newlines() {
        let mut lexer = Lexer::new("true\n\nfalse");
        assert_eq!(lexer.next_token().unwrap(), Token::True);
        assert_eq!(lexer.next_token().unwrap(), Token::False);
        assert_eq!(lexer.next_token().unwrap(), Token::Eof);
    }

    #[test]
    fn test_whitespace_handling_mixed() {
        let mut lexer = Lexer::new("  true \t\n  false  ");
        assert_eq!(lexer.next_token().unwrap(), Token::True);
        assert_eq!(lexer.next_token().unwrap(), Token::False);
        assert_eq!(lexer.next_token().unwrap(), Token::Eof);
    }

    #[test]
    fn test_multiple_eof_calls() {
        let mut lexer = Lexer::new("x");
        assert_eq!(lexer.next_token().unwrap(), Token::Ident("x".into()));
        assert_eq!(lexer.next_token().unwrap(), Token::Eof);
        assert_eq!(lexer.next_token().unwrap(), Token::Eof);
        assert_eq!(lexer.next_token().unwrap(), Token::Eof);
    }

    #[test]
    fn test_single_lparen() {
        let mut lexer = Lexer::new("(");
        assert_eq!(lexer.next_token().unwrap(), Token::LParen);
        assert_eq!(lexer.next_token().unwrap(), Token::Eof);
    }

    #[test]
    fn test_single_rparen() {
        let mut lexer = Lexer::new(")");
        assert_eq!(lexer.next_token().unwrap(), Token::RParen);
        assert_eq!(lexer.next_token().unwrap(), Token::Eof);
    }

    #[test]
    fn test_identifier_with_underscore_prefix() {
        let mut lexer = Lexer::new("_private _internal__value");
        assert_eq!(lexer.next_token().unwrap(), Token::Ident("_private".into()));
        assert_eq!(
            lexer.next_token().unwrap(),
            Token::Ident("_internal__value".into())
        );
    }

    #[test]
    fn test_identifier_with_numbers() {
        let mut lexer = Lexer::new("item1 var2name test_123_abc");
        assert_eq!(lexer.next_token().unwrap(), Token::Ident("item1".into()));
        assert_eq!(lexer.next_token().unwrap(), Token::Ident("var2name".into()));
        assert_eq!(
            lexer.next_token().unwrap(),
            Token::Ident("test_123_abc".into())
        );
    }

    #[test]
    fn test_keywords_are_case_sensitive() {
        let mut lexer = Lexer::new("True FALSE True123");
        assert_eq!(lexer.next_token().unwrap(), Token::Ident("True".into()));
        assert_eq!(lexer.next_token().unwrap(), Token::Ident("FALSE".into()));
        assert_eq!(lexer.next_token().unwrap(), Token::Ident("True123".into()));
    }

    #[test]
    fn test_number_zero() {
        let mut lexer = Lexer::new("0");
        assert_eq!(lexer.next_token().unwrap(), Token::Number(0));
    }

    #[test]
    fn test_number_single_digit() {
        let mut lexer = Lexer::new("7");
        assert_eq!(lexer.next_token().unwrap(), Token::Number(7));
    }

    #[test]
    fn test_number_large() {
        let mut lexer = Lexer::new("9999999999");
        assert_eq!(lexer.next_token().unwrap(), Token::Number(9999999999));
    }

    #[test]
    fn test_empty_string_literal() {
        let mut lexer = Lexer::new("\"\"");
        assert_eq!(lexer.next_token().unwrap(), Token::String("".into()));
    }

    #[test]
    fn test_string_with_spaces() {
        let mut lexer = Lexer::new("\"hello world\"");
        assert_eq!(
            lexer.next_token().unwrap(),
            Token::String("hello world".into())
        );
    }

    #[test]
    fn test_string_with_special_chars() {
        let mut lexer = Lexer::new("\"a && b || !c\"");
        assert_eq!(
            lexer.next_token().unwrap(),
            Token::String("a && b || !c".into())
        );
    }

    #[test]
    fn test_unexpected_char_single_ampersand() {
        let mut lexer = Lexer::new("&");
        assert!(matches!(
            lexer.next_token(),
            Err(LexError::UnexpectedChar('&', 0))
        ));
    }

    #[test]
    fn test_unexpected_char_single_pipe() {
        let mut lexer = Lexer::new("|");
        assert!(matches!(
            lexer.next_token(),
            Err(LexError::UnexpectedChar('|', 0))
        ));
    }

    #[test]
    fn test_nested_parentheses() {
        let mut lexer = Lexer::new("((()))");
        assert_eq!(lexer.next_token().unwrap(), Token::LParen);
        assert_eq!(lexer.next_token().unwrap(), Token::LParen);
        assert_eq!(lexer.next_token().unwrap(), Token::LParen);
        assert_eq!(lexer.next_token().unwrap(), Token::RParen);
        assert_eq!(lexer.next_token().unwrap(), Token::RParen);
        assert_eq!(lexer.next_token().unwrap(), Token::RParen);
        assert_eq!(lexer.next_token().unwrap(), Token::Eof);
    }

    #[test]
    fn test_consecutive_tokens_no_spaces() {
        let mut lexer = Lexer::new("a&&b||!c");
        assert_eq!(lexer.next_token().unwrap(), Token::Ident("a".into()));
        assert_eq!(lexer.next_token().unwrap(), Token::And);
        assert_eq!(lexer.next_token().unwrap(), Token::Ident("b".into()));
        assert_eq!(lexer.next_token().unwrap(), Token::Or);
        assert_eq!(lexer.next_token().unwrap(), Token::Not);
        assert_eq!(lexer.next_token().unwrap(), Token::Ident("c".into()));
    }

    #[test]
    fn test_error_position_tracking() {
        let mut lexer = Lexer::new("abc @");
        assert_eq!(lexer.next_token().unwrap(), Token::Ident("abc".into()));
        let err = lexer.next_token().unwrap_err();
        assert!(matches!(err, LexError::UnexpectedChar('@', 4)));
    }

    #[test]
    fn test_unterminated_string_position() {
        let mut lexer = Lexer::new("abc \"unterminated");
        assert_eq!(lexer.next_token().unwrap(), Token::Ident("abc".into()));
        let err = lexer.next_token().unwrap_err();
        assert!(matches!(err, LexError::UnterminatedString(4)));
    }

    #[test]
    fn test_all_token_types_combined() {
        let mut lexer = Lexer::new("func(true, false, 42, \"str\", x) && !y || z");
        assert_eq!(lexer.next_token().unwrap(), Token::Ident("func".into()));
        assert_eq!(lexer.next_token().unwrap(), Token::LParen);
        assert_eq!(lexer.next_token().unwrap(), Token::True);
        assert_eq!(lexer.next_token().unwrap(), Token::Comma);
        assert_eq!(lexer.next_token().unwrap(), Token::False);
        assert_eq!(lexer.next_token().unwrap(), Token::Comma);
        assert_eq!(lexer.next_token().unwrap(), Token::Number(42));
        assert_eq!(lexer.next_token().unwrap(), Token::Comma);
        assert_eq!(lexer.next_token().unwrap(), Token::String("str".into()));
        assert_eq!(lexer.next_token().unwrap(), Token::Comma);
        assert_eq!(lexer.next_token().unwrap(), Token::Ident("x".into()));
        assert_eq!(lexer.next_token().unwrap(), Token::RParen);
        assert_eq!(lexer.next_token().unwrap(), Token::And);
        assert_eq!(lexer.next_token().unwrap(), Token::Not);
        assert_eq!(lexer.next_token().unwrap(), Token::Ident("y".into()));
        assert_eq!(lexer.next_token().unwrap(), Token::Or);
        assert_eq!(lexer.next_token().unwrap(), Token::Ident("z".into()));
        assert_eq!(lexer.next_token().unwrap(), Token::Eof);
    }

    #[test]
    fn test_function_call_multiple_args() {
        let mut lexer = Lexer::new("between(DAY1_AM_6_00, DAY1_PM_6_00)");
        assert_eq!(lexer.next_token().unwrap(), Token::Ident("between".into()));
        assert_eq!(lexer.next_token().unwrap(), Token::LParen);
        assert_eq!(
            lexer.next_token().unwrap(),
            Token::Ident("DAY1_AM_6_00".into())
        );
        assert_eq!(lexer.next_token().unwrap(), Token::Comma);
        assert_eq!(
            lexer.next_token().unwrap(),
            Token::Ident("DAY1_PM_6_00".into())
        );
        assert_eq!(lexer.next_token().unwrap(), Token::RParen);
    }

    #[test]
    fn test_chained_and_operators() {
        let mut lexer = Lexer::new("a && b && c && d");
        assert_eq!(lexer.next_token().unwrap(), Token::Ident("a".into()));
        assert_eq!(lexer.next_token().unwrap(), Token::And);
        assert_eq!(lexer.next_token().unwrap(), Token::Ident("b".into()));
        assert_eq!(lexer.next_token().unwrap(), Token::And);
        assert_eq!(lexer.next_token().unwrap(), Token::Ident("c".into()));
        assert_eq!(lexer.next_token().unwrap(), Token::And);
        assert_eq!(lexer.next_token().unwrap(), Token::Ident("d".into()));
    }

    #[test]
    fn test_chained_or_operators() {
        let mut lexer = Lexer::new("x || y || z");
        assert_eq!(lexer.next_token().unwrap(), Token::Ident("x".into()));
        assert_eq!(lexer.next_token().unwrap(), Token::Or);
        assert_eq!(lexer.next_token().unwrap(), Token::Ident("y".into()));
        assert_eq!(lexer.next_token().unwrap(), Token::Or);
        assert_eq!(lexer.next_token().unwrap(), Token::Ident("z".into()));
    }

    #[test]
    fn test_not_with_parentheses() {
        let mut lexer = Lexer::new("!(a && b)");
        assert_eq!(lexer.next_token().unwrap(), Token::Not);
        assert_eq!(lexer.next_token().unwrap(), Token::LParen);
        assert_eq!(lexer.next_token().unwrap(), Token::Ident("a".into()));
        assert_eq!(lexer.next_token().unwrap(), Token::And);
        assert_eq!(lexer.next_token().unwrap(), Token::Ident("b".into()));
        assert_eq!(lexer.next_token().unwrap(), Token::RParen);
    }

    #[test]
    fn test_string_with_numbers() {
        let mut lexer = Lexer::new("\"123 abc 456\"");
        assert_eq!(
            lexer.next_token().unwrap(),
            Token::String("123 abc 456".into())
        );
    }

    #[test]
    fn test_identifier_prefix_matches_keyword() {
        let mut lexer = Lexer::new("truthy falsey");
        assert_eq!(lexer.next_token().unwrap(), Token::Ident("truthy".into()));
        assert_eq!(lexer.next_token().unwrap(), Token::Ident("falsey".into()));
    }

    #[test]
    fn test_only_whitespace() {
        let mut lexer = Lexer::new("   \t\n   ");
        assert_eq!(lexer.next_token().unwrap(), Token::Eof);
    }

    #[test]
    fn test_complex_nested_expression() {
        let mut lexer =
            Lexer::new("(has(SWORD) && (is_adult || has(SLINGSHOT))) || event(BOSS_DEFEATED)");
        assert_eq!(lexer.next_token().unwrap(), Token::LParen);
        assert_eq!(lexer.next_token().unwrap(), Token::Ident("has".into()));
        assert_eq!(lexer.next_token().unwrap(), Token::LParen);
        assert_eq!(lexer.next_token().unwrap(), Token::Ident("SWORD".into()));
        assert_eq!(lexer.next_token().unwrap(), Token::RParen);
        assert_eq!(lexer.next_token().unwrap(), Token::And);
        assert_eq!(lexer.next_token().unwrap(), Token::LParen);
        assert_eq!(lexer.next_token().unwrap(), Token::Ident("is_adult".into()));
        assert_eq!(lexer.next_token().unwrap(), Token::Or);
        assert_eq!(lexer.next_token().unwrap(), Token::Ident("has".into()));
        assert_eq!(lexer.next_token().unwrap(), Token::LParen);
        assert_eq!(
            lexer.next_token().unwrap(),
            Token::Ident("SLINGSHOT".into())
        );
        assert_eq!(lexer.next_token().unwrap(), Token::RParen);
        assert_eq!(lexer.next_token().unwrap(), Token::RParen);
        assert_eq!(lexer.next_token().unwrap(), Token::RParen);
        assert_eq!(lexer.next_token().unwrap(), Token::Or);
        assert_eq!(lexer.next_token().unwrap(), Token::Ident("event".into()));
        assert_eq!(lexer.next_token().unwrap(), Token::LParen);
        assert_eq!(
            lexer.next_token().unwrap(),
            Token::Ident("BOSS_DEFEATED".into())
        );
        assert_eq!(lexer.next_token().unwrap(), Token::RParen);
        assert_eq!(lexer.next_token().unwrap(), Token::Eof);
    }
}
