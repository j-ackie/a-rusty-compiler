use crate::lexer::token::Token;

trait Expr {}

struct Literal {
    value: String,
}

pub fn parse(tokens: Vec<Token>) {
    println!("Parsing {} tokens", tokens.len());
}
