pub mod ast;
pub mod lexer;
pub mod parser;
pub mod semantic_checker;

use std::env;
use std::fs;

use lexer::LexerError;

fn main() {
    let args: Vec<String> = env::args().collect();

    let mut test = false;
    let mut file = String::new();

    let mut iter = args.iter().skip(1);
    while let Some(arg) = iter.next() {
        match arg.as_str() {
            "test" => {
                test = true;
            }
            "-c" => {
                if let Some(f) = iter.next() {
                    file = f.to_string();
                } else {
                    eprintln!("Expected file name after -c");
                    std::process::exit(1);
                }
            }
            _ => {}
        }
    }

    let contents = fs::read_to_string(file).expect("Failed to read file");

    let tokens = match lexer::tokenize(contents, test) {
        Ok(tokens) => tokens,
        Err(errors) => {
            eprintln!("Errors occurred during parsing: ");
            for error in errors {
                match error {
                    LexerError::UnexpectedCharacter(char, line) => {
                        eprintln!(
                            "\x1b[31mUnexpected character: '{}' at line {}\x1b[0m",
                            char, line
                        )
                    }
                    LexerError::UnterminatedString(line) => {
                        eprintln!("\x1b[31mUnterminated string at line {}\x1b[0m", line)
                    }
                }
            }
            std::process::exit(1);
        }
    };

    lexer::pretty_print_tokens(&tokens);

    let program = match parser::parse(tokens) {
        Ok(program) => program,
        Err(error) => {
            eprintln!("Errors occurred during parsing: ");
            match error {
                parser::ParseError::UnexpectedToken(line) => {
                    eprintln!("\x1b[31mUnexpected token at line {}\x1b[0m", line)
                }
            }
            std::process::exit(1);
        }
    };

    println!("Parsed program: {:#?}", program);

    match semantic_checker::check(&program) {
        Ok(()) => println!("\x1b[32mSemantic check passed!\x1b[0m"),
        Err(error) => {
            eprintln!("Errors occurred during semantic checking: ");
            match error {
                semantic_checker::SemanticError::UndefinedVariable(name) => {
                    eprintln!("\x1b[31mUndefined variable: '{}'\x1b[0m", name)
                }
                semantic_checker::SemanticError::ReservedKeyword(name) => {
                    eprintln!(
                        "\x1b[31mReserved keyword used as identifier: '{}'\x1b[0m",
                        name
                    )
                }
                semantic_checker::SemanticError::TypeError(message) => {
                    eprintln!("\x1b[31mType error: {}\x1b[0m", message)
                }
            }
            std::process::exit(1);
        }
    };
}
