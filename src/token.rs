#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Identifier(String),
    Integer(i32),
    Float(f32),
    Verb(String),
    Noun(String),
    Preposition(String),
    Determiner(String),
    String(String),
    Char(char),
    Illegal(char),
    Eof,
}