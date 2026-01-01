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
}
