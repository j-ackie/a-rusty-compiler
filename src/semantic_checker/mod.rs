use crate::ast::{self, DataType, Expr, Identifier, Literal, Visitor};
use crate::lexer::token::KEYWORDS;
use std::collections::HashMap;

pub enum SemanticError {
    UndefinedVariable(String),
    ReservedKeyword(String),
    TypeError(String),
}

struct SemanticChecker {
    symbol_table: HashMap<String, Identifier>,
}

impl SemanticChecker {
    fn new() -> Self {
        SemanticChecker {
            symbol_table: HashMap::new(),
        }
    }

    fn declare_variable(&mut self, identifier: &Identifier) -> Result<(), SemanticError> {
        if KEYWORDS.contains_key(&identifier.name) {
            return Err(SemanticError::ReservedKeyword(format!(
                "Cannot use reserved keyword '{}' as variable name",
                identifier.name
            )));
        }

        if self.symbol_table.contains_key(&identifier.name) {
            return Err(SemanticError::TypeError(format!(
                "Variable '{}' already declared",
                identifier.name
            )));
        }

        self.symbol_table
            .insert(identifier.name.clone(), identifier.clone());

        Ok(())
    }

    fn get_variable(&self, name: &str) -> Option<&Identifier> {
        self.symbol_table.get(name)
    }
}

impl ast::Visitor<Result<(), SemanticError>> for SemanticChecker {
    fn visit_type(&mut self, _data_type: &ast::DataType) -> Result<(), SemanticError> {
        Ok(())
    }

    fn visit_literal(&mut self, _literal: &ast::Literal) -> Result<(), SemanticError> {
        Ok(())
    }

    fn visit_identifier(&mut self, identifier: &ast::Identifier) -> Result<(), SemanticError> {
        if self.get_variable(&identifier.name).is_none() {
            return Err(SemanticError::UndefinedVariable(format!(
                "Variable '{}' is not defined",
                identifier.name
            )));
        }
        Ok(())
    }

    fn visit_expr(&mut self, expr: &ast::Expr) -> Result<(), SemanticError> {
        match expr {
            ast::Expr::Literal(literal) => self.visit_literal(literal),
            ast::Expr::Identifier(identifier) => self.visit_identifier(identifier),
        }
    }

    fn visit_assignment(&mut self, assignment: &ast::Assignment) -> Result<(), SemanticError> {
        self.visit_expr(&assignment.value)?;

        self.declare_variable(&assignment.identifier)?;

        let _ = match assignment.data_type {
            DataType::Int => match &assignment.value {
                Expr::Literal(Literal::Integer(_)) => Ok(()),
                _ => {
                    return Err(SemanticError::TypeError(format!(
                        "Type mismatch: expected 'int' for variable '{}'",
                        assignment.identifier.name
                    )));
                }
            },
            DataType::Bool => match &assignment.value {
                Expr::Literal(Literal::Boolean(_)) => Ok(()),
                _ => {
                    return Err(SemanticError::TypeError(format!(
                        "Type mismatch: expected 'bool' for variable '{}'",
                        assignment.identifier.name
                    )));
                }
            },
            DataType::Float => match &assignment.value {
                Expr::Literal(Literal::Float(_)) => Ok(()),
                _ => {
                    return Err(SemanticError::TypeError(format!(
                        "Type mismatch: expected 'float' for variable '{}'",
                        assignment.identifier.name
                    )));
                }
            },
            DataType::Double => match &assignment.value {
                Expr::Literal(Literal::Float(_)) => Ok(()),
                _ => {
                    return Err(SemanticError::TypeError(format!(
                        "Type mismatch: expected 'double' for variable '{}'",
                        assignment.identifier.name
                    )));
                }
            },
            DataType::Char => match &assignment.value {
                Expr::Literal(Literal::Char(_)) => Ok(()),
                _ => {
                    return Err(SemanticError::TypeError(format!(
                        "Type mismatch: expected 'char' for variable '{}'",
                        assignment.identifier.name
                    )));
                }
            },
            DataType::Void => Err(SemanticError::TypeError(format!(
                "Cannot assign void type to variable '{}'",
                assignment.identifier.name
            ))),
        };

        Ok(())
    }

    fn visit_return(&mut self, return_stmt: &ast::Return) -> Result<(), SemanticError> {
        self.visit_expr(&return_stmt.value)
    }

    fn visit_instruction(&mut self, instruction: &ast::Instruction) -> Result<(), SemanticError> {
        match instruction {
            ast::Instruction::Assignment(assignment) => self.visit_assignment(assignment),
            ast::Instruction::Return(return_stmt) => self.visit_return(return_stmt),
        }
    }

    fn visit_function(&mut self, function: &ast::Function) -> Result<(), SemanticError> {
        for instruction in &function.instructions {
            self.visit_instruction(instruction)?;
        }
        Ok(())
    }

    fn visit_program(&mut self, program: &ast::Program) -> Result<(), SemanticError> {
        for function in &program.functions {
            self.visit_function(function)?;
        }
        Ok(())
    }
}

pub fn check(program: &ast::Program) -> Result<(), SemanticError> {
    let mut checker = SemanticChecker::new();

    checker.visit_program(program)
}
