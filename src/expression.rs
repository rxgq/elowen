use crate::token::Token;

pub enum Expression {
    Literal(Literal),
    VariableDeclaration {
        verb: Token,
        value: Box<Expression>
    }
}

pub enum Literal {
    Float(f32),
    Integer(i32)
}