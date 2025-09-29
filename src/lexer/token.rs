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
        m.insert("int".to_string(), TokenType::Int);
        m.insert("bool".to_string(), TokenType::Bool);
        m.insert("char".to_string(), TokenType::Char);
        m.insert("float".to_string(), TokenType::Float);
        m.insert("double".to_string(), TokenType::Double);
        m.insert("void".to_string(), TokenType::Void);
        m.insert("return".to_string(), TokenType::Return);

        m
    };
}

#[derive(Debug, PartialEq, Clone)]
pub enum TokenType {
    // Single-character
    LeftParenthesis,
    RightParenthesis,
    LeftBrace,
    RightBrace,
    Semicolon,

    // One or two character tokens
    Bang,
    BangEqual,
    Equal,
    EqualEqual,

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
    Int,
    Bool,
    Char,
    Float,
    Double,
    Void,
    Return,

    EOF,
}

#[derive(Debug, PartialEq)]
pub struct Token {
    pub token_type: TokenType,
    pub lexeme: String,
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
