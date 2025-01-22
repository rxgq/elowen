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
        
        let mut identifier: Token;

        if !self.is_identifier() {
            if self.is_determiner() {
                self.advance();

                if !self.is_noun() {
                    return Err(ParseError::ExpectedExpressionAfter(
                        "noun".to_string(),
                        "determiner".to_string()
                    ))
                }
            }
            else if self.is_literal()  {
                self.advance();

                if !self.is_preposition() {
                    return Err(ParseError::ExpectedExpressionAfter(
                        "preposition".to_string(),
                        "literal expression".to_string()
                    ))
                }

                self.advance();

                if self.is_determiner() {
                    self.advance();

                    if !self.is_noun() {
                        return Err(ParseError::ExpectedExpressionAfter(
                            "noun".to_string(),
                            "determiner".to_string()
                        ))
                    } else {
                        self.advance();
                    }
                }
                
                if !self.is_identifier() {
                    identifier = self.current_token().unwrap();

                    return Err(ParseError::ExpectedExpressionAfter(
                        "identifier".to_string(),
                        "noun".to_string()
                    ))
                }
            }
            else {
                identifier = self.current_token().unwrap();
            }
        }


        Ok(Expression::VariableDeclaration { 
            identifier: identifier,
            value: Box::new(Expression::Literal(Literal::Integer(1)))
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
        return self.check(Token::Integer(0)) ||
            self.check(Token::Float(0.0));
    }

    fn is_preposition(&mut self) -> bool {
        return self.check(Token::Preposition(String::new()));
    }

    fn prefer_determiner() {
        // todo!
    }

    fn current_token(&mut self) -> Option<Token> {
        return self.tokens.iter().nth(self.current).cloned();
    }

    fn advance(&mut self) {
        self.current += 1
    }
}