use core::panic;

use crate::{expression::{Expression, Literal}, token::Token};

enum ParseError {
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

    pub fn parse_ast(&mut self) -> &Vec<Expression> {
        while let Some(_) = self.current_token() {
            let expr = self.parse_statement();
            self.expressions.push(expr);

            self.current += 1
        }

        &self.expressions
    }

    fn parse_statement(&mut self) -> Expression {
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

    fn parse_definition_context(&mut self) -> Expression {
        self.advance();

        if !self.is_identifier() {
            if self.is_determiner() {
                self.advance();

                if !self.is_noun() {
                    panic!("expected noun after determiner")
                }
            }
            else if self.is_literal()  {
                self.advance();

                if !self.is_preposition() {
                    panic!("expected preposition after literal expression")
                }

                self.advance();

                if self.is_determiner() {
                    self.advance();

                    if !self.is_noun() {
                        panic!("expected noun after determiner")
                    }
                }

                self.advance();
                
                if !self.is_identifier() {
                    panic!("expected identifier after noun")
                }
            }
            else {
                panic!("expected determiner");
            }
        }
        
        panic!("got to the end");
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

    fn expect(&mut self, token: Token) -> bool {
        if self.check(token.clone()) {
            self.current += 1;
            return true
        }

        false
    }
    

    fn current_token(&mut self) -> Option<Token> {
        return self.tokens.iter().nth(self.current).cloned();
    }

    fn advance(&mut self) {
        self.current += 1
    }
}