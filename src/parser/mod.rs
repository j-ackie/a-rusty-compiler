use std::iter::Peekable;

use crate::ast::*;
use crate::lexer::token::{Token, TokenType};

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

    if token.token_type != expected_token_type {
        return None;
    }

    tokens.next()
}

fn parse_literal(tokens: &mut Peekable<impl Iterator<Item = Token>>) -> Option<Literal> {
    let literal: Literal = match tokens.peek() {
        Some(token) => match token.token_type {
            TokenType::Number => Literal::Integer(IntegerLiteral {
                value: token.lexeme.parse().ok()?,
            }),
            TokenType::True => Literal::Boolean(BooleanLiteral { value: true }),
            TokenType::False => Literal::Boolean(BooleanLiteral { value: false }),
            TokenType::String => Literal::String(StringLiteral {
                value: token.lexeme.clone(),
            }),
            _ => return None,
        },
        None => return None,
    };

    tokens.next();
    Some(literal)
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
            TokenType::Bool => DataType::Bool,
            TokenType::Char => DataType::Char,
            TokenType::Float => DataType::Float,
            TokenType::Double => DataType::Double,
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
