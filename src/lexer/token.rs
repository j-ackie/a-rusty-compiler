/*
  https://craftinginterpreters.com/scanning.html#the-scanner-class
*/

use lazy_static::lazy_static;
use std::collections::HashMap;

lazy_static! {
    pub static ref KEYWORDS: HashMap<String, TokenType> = {
        let mut m = HashMap::new();

        m.insert("and".to_string(), TokenType::And);
        m.insert("class".to_string(), TokenType::Class);
        m.insert("if".to_string(), TokenType::If);
        m.insert("else".to_string(), TokenType::Else);
        m.insert("true".to_string(), TokenType::True);
        m.insert("false".to_string(), TokenType::False);

        m
    };
}

#[derive(Debug, PartialEq, Clone)]
pub enum TokenType {
    // Single-character
    LeftParenthesis,
    RightParenthesis,

    // One or two character tokens
    Bang,
    BangEqual,

    // Literals
    Identifier,
    String,
    Number,

    // Keywords
    And,
    Class,
    If,
    Else,
    True,
    False,

    EOF,
}

#[derive(Debug)]
pub struct Token {
    pub token_type: TokenType,
    lexeme: String,
    pub line: usize,
}

impl Token {
    pub fn new(token_type: TokenType, lexeme: String, line: usize) -> Self {
        Token {
            token_type,
            lexeme,
            line,
        }
    }

    pub fn to_string(&self) -> String {
        format!("{:?} {}", self.token_type, self.lexeme)
    }
}
