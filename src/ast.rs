#[derive(Debug, PartialEq)]
pub enum DataType {
    Int,
    Char,
    Float,
    Double,
    Bool,
    Void,
}

#[derive(Debug)]
pub struct BooleanLiteral {
    pub value: bool,
}

#[derive(Debug)]
pub struct IntegerLiteral {
    pub value: i64,
}

#[derive(Debug)]
pub struct FloatLiteral {
    pub value: f64,
}

#[derive(Debug)]
pub struct CharLiteral {
    pub value: char,
}

#[derive(Debug)]
pub struct StringLiteral {
    pub value: String,
}

#[derive(Debug)]
pub enum Literal {
    Boolean(BooleanLiteral),
    Integer(IntegerLiteral),
    Float(FloatLiteral),
    Char(CharLiteral),
    String(StringLiteral),
}

#[derive(Debug, Clone)]
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

pub trait Visitor<T> {
    fn visit_type(&mut self, data_type: &DataType) -> T;
    fn visit_literal(&mut self, literal: &Literal) -> T;
    fn visit_identifier(&mut self, identifier: &Identifier) -> T;
    fn visit_expr(&mut self, expr: &Expr) -> T;
    fn visit_assignment(&mut self, assignment: &Assignment) -> T;
    fn visit_return(&mut self, return_stmt: &Return) -> T;
    fn visit_instruction(&mut self, instruction: &Instruction) -> T;
    fn visit_function(&mut self, function: &Function) -> T;
    fn visit_program(&mut self, program: &Program) -> T;
}
