use std::collections::HashMap;
use std::iter::Peekable;
use std::str::Chars;

use lazy_static::lazy_static;

use crate::lexer::position::Position;
use crate::lexer::token::Token;
use crate::lexer::token_type::TokenType;

pub mod token;
pub mod token_type;
pub mod position;

lazy_static! {
    static ref TOKEN_MAP: HashMap<&'static str, TokenType> = {
        let mut m = HashMap::new();
        m.insert("entry", TokenType::HeadNode);
        m.insert("body", TokenType::BodyNode);
        m.insert("end", TokenType::EndNode);
        m.insert("true", TokenType::True);
        m.insert("false", TokenType::False);
        m
    };

    static ref SINGLE_MAP: HashMap<char, TokenType> = {
        let mut m = HashMap::new();
        m.insert('(', TokenType::Open);
        m.insert(')', TokenType::Close);
        m.insert(',', TokenType::Comma);
        m
    };
}

pub struct Lexer<'a> {
    source: &'a str,
    chars: Peekable<Chars<'a>>,
    position: Position<'a>,
}

impl<'a> Lexer<'a> {
    pub fn new(source: &'a str) -> Self {
        let chars = source.chars().peekable();

        Self { source, chars, position: Position::new(source) }
    }

    fn str_case(c: char) -> bool {
        c != '"' && c != '\n'
    }

    fn num_case(c: char) -> bool {
        c.is_ascii_digit()
    }

    fn ident_case(c: char) -> bool {
        c.is_ascii_alphabetic()
    }

    fn generic(&mut self, case: fn(char) -> bool) -> String {
        let mut buffer = String::new();

        while let Some(current) = self.advance(false) {
            if !case(current) {
                break;
            }

            buffer.push(current);
        }

        buffer
    }

    fn advance(&mut self, newline: bool) -> Option<char> {
        self.position.advance(newline);
        self.chars.next()
    }
}

impl<'a> Iterator for Lexer<'a> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        return match self.chars.peek() {
            Some(&curr) => {
                if curr == '"' {
                    self.advance(false);
                    Some(Token::new(Some(self.generic(Self::str_case)), TokenType::String))
                } else if curr.is_ascii_digit() {
                    Some(Token::new(Some(self.generic(Self::num_case)), TokenType::Number))
                } else if curr.is_ascii_whitespace() {
                    self.advance(curr == '\n');
                    self.next()
                } else if let Some(single) = SINGLE_MAP.get(&curr) {
                    self.advance(false);
                    Some(Token::new(None, *single))
                } else {
                    let consumed = self.generic(Self::ident_case);

                    return match TOKEN_MAP.get(consumed.as_str()) {
                        Some(ident) => Some(Token::new(None, *ident)),
                        None => Some(Token::new(Some(self.generic(Self::ident_case)), TokenType::Ident))
                    };
                }
            }
            None => None
        };
    }
}