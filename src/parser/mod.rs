use std::iter::Peekable;

use crate::lexer::token::{Token, TokenType};

#[derive(Debug)]
pub enum DataType {
    Int,
    Char,
    Float,
    Double,
    Bool,
    Void,
}

#[derive(Debug)]
pub struct Literal {
    pub value: String,
}

#[derive(Debug)]
pub struct Identifier {
    pub name: String,
}

#[derive(Debug)]
pub enum Expr {
    Literal(Literal),
    Identifier(Identifier),
}

#[derive(Debug)]
pub struct Assignment {
    pub data_type: DataType,
    pub identifier: Identifier,
    pub value: Expr,
}

#[derive(Debug)]
pub struct Return {
    pub value: Expr,
}

#[derive(Debug)]
pub enum Instruction {
    Assignment(Assignment),
    Return(Return),
}

#[derive(Debug)]
pub struct Function {
    pub return_type: DataType,
    pub name: String,
    pub instructions: Vec<Instruction>,
}

#[derive(Debug)]
pub struct Program {
    pub functions: Vec<Function>,
}

trait Visitor<T> {
    fn visit_literal(&mut self, literal: &Literal) -> T;
    fn visit_identifier(&mut self, identifier: &Identifier) -> T;
    fn visit_expr(&mut self, expr: &Expr) -> T;
}

pub struct AstPrinter;
impl AstPrinter {
    pub fn print(&mut self, expr: &Expr) {
        println!("{}", self.visit_expr(expr));
    }
}

impl Visitor<String> for AstPrinter {
    fn visit_literal(&mut self, literal: &Literal) -> String {
        return literal.value.clone();
    }

    fn visit_identifier(&mut self, identifier: &Identifier) -> String {
        return identifier.name.clone();
    }

    fn visit_expr(&mut self, expr: &Expr) -> String {
        match expr {
            Expr::Literal(literal) => self.visit_literal(literal),
            Expr::Identifier(identifier) => self.visit_identifier(identifier),
        }
    }
}

pub enum ParseError {
    UnexpectedToken(usize),
}

fn match_tokens(
    tokens: &mut impl Iterator<Item = Token>,
    expected_token_types: Vec<TokenType>,
) -> Option<Vec<Token>> {
    let mut new_tokens: Vec<Token> = vec![];
    let mut expected_token_types_iter = expected_token_types.iter();

    while let Some(expected_token_type) = expected_token_types_iter.next() {
        let token = match tokens.next() {
            Some(token) => token,
            None => return None,
        };

        if token.token_type != *expected_token_type {
            return None;
        }

        new_tokens.push(token);
    }

    Some(new_tokens)
}

fn match_token(
    tokens: &mut Peekable<impl Iterator<Item = Token>>,
    expected_token_type: TokenType,
) -> Option<Token> {
    let token = match tokens.peek() {
        Some(token) => token,
        None => return None,
    };

    println!("HELLO: {:?}, {:?}", token.token_type, expected_token_type);

    if token.token_type != expected_token_type {
        return None;
    }

    tokens.next()
}

fn parse_literal(tokens: &mut Peekable<impl Iterator<Item = Token>>) -> Option<Literal> {
    match tokens.peek() {
        Some(token) => match token.token_type {
            TokenType::Number | TokenType::String | TokenType::True | TokenType::False => {
                let token = tokens.next().unwrap();
                return Some(Literal {
                    value: token.lexeme,
                });
            }
            _ => return None,
        },
        None => None,
    }
}

fn parse_identifier(tokens: &mut Peekable<impl Iterator<Item = Token>>) -> Option<Identifier> {
    match tokens.peek() {
        Some(token) => match token.token_type {
            TokenType::Identifier => {
                let token = tokens.next().unwrap();
                return Some(Identifier { name: token.lexeme });
            }
            _ => return None,
        },
        None => None,
    }
}

fn parse_expression(tokens: &mut Peekable<impl Iterator<Item = Token>>) -> Option<Expr> {
    if let Some(literal) = parse_literal(tokens) {
        return Some(Expr::Literal(literal));
    }

    if let Some(identifier) = parse_identifier(tokens) {
        return Some(Expr::Identifier(identifier));
    }

    None
}

fn parse_type(tokens: &mut Peekable<impl Iterator<Item = Token>>) -> Option<DataType> {
    let data_type = match tokens.peek() {
        Some(token) => match token.token_type {
            TokenType::Int => DataType::Int,
            TokenType::Void => DataType::Void,
            _ => return None,
        },
        None => return None,
    };

    tokens.next();
    Some(data_type)
}

fn parse_assignment(tokens: &mut Peekable<impl Iterator<Item = Token>>) -> Option<Assignment> {
    let data_type = parse_type(tokens)?;
    let identifier = parse_identifier(tokens)?;

    match_token(tokens, TokenType::Equal)?;

    let expr = parse_expression(tokens)?;

    match_token(tokens, TokenType::Semicolon)?;

    Some(Assignment {
        data_type: data_type,
        identifier: identifier,
        value: expr,
    })
}

fn parse_return(tokens: &mut Peekable<impl Iterator<Item = Token>>) -> Option<Return> {
    match_token(tokens, TokenType::Return)?;

    let expression = parse_expression(tokens)?;

    match_token(tokens, TokenType::Semicolon)?;

    Some(Return { value: expression })
}

fn parse_instructions(
    tokens: &mut Peekable<impl Iterator<Item = Token>>,
) -> Result<Vec<Instruction>, ParseError> {
    let mut instructions: Vec<Instruction> = vec![];

    loop {
        if let Some(assignment) = parse_assignment(tokens) {
            instructions.push(Instruction::Assignment(assignment));
            continue;
        }

        if let Some(ret) = parse_return(tokens) {
            instructions.push(Instruction::Return(ret));
            continue;
        }

        if let Some(next_token) = tokens.peek() {
            if next_token.token_type == TokenType::RightBrace {
                break;
            }
        } else {
            return Err(ParseError::UnexpectedToken(1));
        }
    }

    Ok(instructions)
}

fn parse_function(
    tokens: &mut Peekable<impl Iterator<Item = Token>>,
) -> Result<Function, ParseError> {
    if match_tokens(
        tokens,
        vec![
            TokenType::Int,
            TokenType::Identifier,
            TokenType::LeftParenthesis,
            TokenType::Void,
            TokenType::RightParenthesis,
            TokenType::LeftBrace,
        ],
    )
    .is_none()
    {
        return Err(ParseError::UnexpectedToken(10));
    };

    let instructions = parse_instructions(tokens)?;

    if match_token(tokens, TokenType::RightBrace).is_none() {
        return Err(ParseError::UnexpectedToken(20));
    }

    Ok(Function {
        return_type: DataType::Int,
        name: "main".to_string(),
        instructions: instructions,
    })
}

pub fn parse(tokens: Vec<Token>) -> Result<Program, ParseError> {
    let mut tokens_iterator = tokens.into_iter().peekable();

    let functions = parse_function(&mut tokens_iterator)?;

    Ok(Program {
        functions: vec![functions],
    })
}

#[cfg(tests)]
mod tests {
    use super::*;

    #[test]
    fn test_ast_printer() {
        let mut printer = AstPrinter;
        let expr = Expr::Literal(Literal {
            value: "42".to_string(),
        });
        printer.print(&expr);
        // Expected output: 42
    }
}
