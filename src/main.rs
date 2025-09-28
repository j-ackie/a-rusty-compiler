pub mod lexer;
pub mod parser;

use std::env;
use std::fs;

use lexer::LexerError;

fn main() {
    let args: Vec<String> = env::args().collect();

    let mut test = false;

    for arg in args.iter() {
        match arg.as_str() {
            "test" => {
                test = true;
            }
            _ => {}
        }
    }

    let contents = fs::read_to_string("src/program.c").expect("Failed to read file");

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
}
