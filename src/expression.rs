use crate::token::Token;

#[derive(Debug)]
pub enum Expression {
    Literal(Literal),
    VariableDeclaration {
        identifier: Token,
        value: Box<Expression>
    }
}

#[derive(Debug)]
pub enum Literal {
    Float(f32),
    Integer(i32),
    Identifier(String),
}