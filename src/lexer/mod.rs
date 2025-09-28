use std::{iter::Peekable, str::Chars};

pub mod token;

use {token::KEYWORDS, token::Token, token::TokenType};

#[derive(Debug)]
pub enum LexerError {
    UnexpectedCharacter(char, usize),
    UnterminatedString(usize),
}

pub fn tokenize(source: String, test: bool) -> Result<Vec<Token>, Vec<LexerError>> {
    let mut tokens: Vec<Token> = vec![];
    let mut errors: Vec<LexerError> = vec![];
    let mut source_chars = source.chars().peekable();
    let mut line = 1;
    loop {
        let result = scan_token(&mut source_chars, &mut line);
        let token = match result {
            Ok(t) => t,
            Err(e) => {
                errors.push(e);
                continue;
            }
        };

        if token.token_type == TokenType::EOF {
            break;
        }

        tokens.push(token);
    }

    if !errors.is_empty() && !test {
        return Err(errors);
    }

    Ok(tokens)
}

fn scan_token(source_chars: &mut Peekable<Chars>, line: &mut usize) -> Result<Token, LexerError> {
    let character = match source_chars.next() {
        Some(c) => c,
        None => return Ok(Token::new(TokenType::EOF, "".to_string(), *line)),
    };

    let mut final_text = character.to_string();

    let token_type = match character {
        // Single-character tokens
        '(' => TokenType::LeftParenthesis,
        ')' => TokenType::RightParenthesis,
        '{' => TokenType::LeftBrace,
        '}' => TokenType::RightBrace,
        '!' => {
            if match_two_char_token('=', source_chars) {
                final_text.push('=');
                TokenType::BangEqual
            } else {
                TokenType::Bang
            }
        }
        '=' => {
            if match_two_char_token('=', source_chars) {
                final_text.push('=');
                TokenType::EqualEqual
            } else {
                TokenType::Equal
            }
        }
        ';' => TokenType::Semicolon,

        // Literals
        '"' => {
            let string = match_string(source_chars, *line)?;
            final_text = string;
            TokenType::String
        }
        '0'..='9' => {
            let number = match_number(source_chars);
            final_text.push_str(&number);
            TokenType::Number
        }

        character if character.is_alphabetic() => {
            let identifier = match_identifier(source_chars);
            final_text.push_str(&identifier);

            if let Some(keyword_type) = KEYWORDS.get(&final_text) {
                return Ok(Token::new(keyword_type.clone(), final_text, *line));
            }

            TokenType::Identifier
        }

        // Whitespace
        ' ' | '\r' | '\t' => return scan_token(source_chars, line),

        '\n' => {
            *line += 1;
            return scan_token(source_chars, line);
        }

        _ => return Err(LexerError::UnexpectedCharacter(character, *line)),
    };

    Ok(Token::new(token_type, final_text, *line))
}

fn match_two_char_token(expected: char, source_chars: &mut Peekable<Chars>) -> bool {
    let character = match source_chars.peek() {
        Some(c) => c,
        None => return false,
    };

    if *character == expected {
        source_chars.next();
        return true;
    }

    return false;
}

fn match_string(source_chars: &mut Peekable<Chars>, line: usize) -> Result<String, LexerError> {
    let mut string = String::new();

    loop {
        let character = match source_chars.next() {
            Some(c) => c,
            None => return Err(LexerError::UnterminatedString(line)),
        };

        match character {
            '"' => return Ok(string),
            _ => string.push(character),
        }
    }
}

fn match_number(source_chars: &mut Peekable<Chars>) -> String {
    let mut number = String::new();

    while let Some(c) = source_chars.peek() {
        if c.is_ascii_digit() {
            number.push(*c);
            source_chars.next();
        } else {
            break;
        }
    }

    number
}

fn match_identifier(source_chars: &mut Peekable<Chars>) -> String {
    let mut identifier = String::new();

    while let Some(c) = source_chars.peek() {
        if c.is_alphanumeric() || *c == '_' {
            identifier.push(*c);
            source_chars.next();
        } else {
            break;
        }
    }

    identifier
}

pub fn pretty_print_tokens(tokens: &Vec<Token>) {
    let mut current_line = 0;

    for token in tokens {
        let line = token.line;
        if line != current_line {
            current_line = line;
            println!("\n{}: {}", line, token.to_string());
        } else {
            print!("{} ", token.to_string());
        }
    }
    println!();
}
