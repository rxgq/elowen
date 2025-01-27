use core::panic;

use crate::{expression::{Expression, Literal}, token::Token};

pub enum ParseError {
    ExpectedExpressionAfter(String, String)
}

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
    expressions: Vec<Expression>
}

type ParseResult<T> = Result<T, ParseError>;

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self {
            tokens,
            current: 0,
            expressions: vec![]
        }
    }

    pub fn parse_ast(&mut self) -> Result<&Vec<Expression>, ParseError> {
        while let Some(_) = self.current_token() {
            match self.parse_statement() {
                Ok(expr) => {
                    self.expressions.push(expr);
                    self.current += 1
                } 
                Err(err) => {
                    return Err(err)
                }
            }
        }

        Ok(&self.expressions)
    }

    fn parse_statement(&mut self) -> ParseResult<Expression> {
        let token = self.current_token();

        match token {
            Some(Token::Verb(_)) => {
                self.parse_definition_context()
            },
            _ => {
                todo!()
            }
        }
    }

    fn parse_definition_context(&mut self) -> ParseResult<Expression> {
        self.advance();
        
        let identifier = self.current_token();
        if !self.is_identifier() {
            return Err(ParseError::ExpectedExpressionAfter(
                "identifier".to_string(), "declaration verb".to_string())
            );
        }
        self.advance();
        
        if !self.is_preposition() {
            return Err(ParseError::ExpectedExpressionAfter(
                "initializing preposition".to_string(), "declaration verb".to_string())
            );
        }
        self.advance();

        let value = self.parse_expression();
        
        Ok(Expression::VariableDeclaration { 
            identifier: identifier.unwrap(),
            value: Box::new(value)
        })
    }

    fn parse_expression(&mut self) -> Expression {
        if let Some(c) = self.current_token() {
            match c {
                Token::Identifier(identifier) => {
                    Expression::Literal(Literal::Identifier(identifier))
                },
                Token::Integer(integer) => {
                    Expression::Literal(Literal::Integer(integer))
                }
                Token::Float(float) => {
                    Expression::Literal(Literal::Float(float))
                },
                _ => {
                    panic!("expected expression")
                }
            }
        } else {
            panic!("none token found")
        }
    }

    fn check(&mut self, token: Token) -> bool {
        if let Some(current) = self.current_token() {
            match (token, current) {
                (Token::Verb(_), Token::Verb(_)) |
                (Token::Identifier(_), Token::Identifier(_)) |
                (Token::Preposition(_), Token::Preposition(_)) |
                (Token::Determiner(_), Token::Determiner(_)) |
                (Token::Noun(_), Token::Noun(_)) |
                (Token::Float(_), Token::Float(_)) |
                (Token::Integer(_), Token::Integer(_)) 
                 => {
                    return true
                }
                _ => {}
            }
        }

        false
    }

    fn is_identifier(&mut self) -> bool {
        return self.check(Token::Identifier(String::new()));
    }

    fn is_determiner(&mut self) -> bool {
        return self.check(Token::Determiner(String::new()));
    }

    fn is_noun(&mut self) -> bool {
        return self.check(Token::Noun(String::new()));
    }

    fn is_literal(&mut self) -> bool {
        return self.check(Token::Integer(0)) || self.check(Token::Float(0.0));
    }

    fn is_preposition(&mut self) -> bool {
        return self.check(Token::Preposition(String::new()));
    }

    fn current_token(&mut self) -> Option<Token> {
        return self.tokens.iter().nth(self.current).cloned();
    }

    fn advance(&mut self) {
        self.current += 1
    }
}