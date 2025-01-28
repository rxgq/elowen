use crate::token::Token;

#[derive(Debug, Clone)]
pub enum Expression {
    Literal(Literal),
    VariableDeclaration {
        verb: Token,
        identifier: Token,
        preposition: Token,
        value: Box<Expression>
    }
}

#[derive(Debug, Clone)]
pub enum Literal {
    Float(f32),
    Integer(i32),
    Identifier(String),
}