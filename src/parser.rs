use core::panic;

use crate::{expression::{Expression, Literal}, token::Token};

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
    expressions: Vec<Expression>
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self {
            tokens,
            current: 0,
            expressions: vec![]
        }
    }

    pub fn parse_ast(&mut self) -> &Vec<Expression> {
        while let Some(token) = self.current_token() {
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
                self.parse_variable_declaration()
            },
            _ => {
                todo!()
            }
        }
    }

    fn parse_variable_declaration(&mut self) -> Expression {
        if let Some(_) = self.current_token() {
            self.expect(Token::Verb(String::new()));
        };

        let identifier = if let Some(token) = self.current_token() {
            self.check(Token::Identifier(String::new()));
            self.current += 1;
            token
        } else {
            panic!("Expected an Identifier token after Verb in variable declaration");
        };


        self.expect(Token::Preposition(String::new()));

        let value = self.parse_expression();

        Expression::VariableDeclaration { 
            identifier, 
            value: Box::new(value)
        }
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
                (Token::Determiner(_), Token::Determiner(_))
                 => {
                    return true
                }
                _ => {
                    println!("unknown check type. pattern may not be accounted for.")
                }
            }
        }

        false
    }

    fn expect(&mut self, token: Token) -> bool {
        if self.check(token.clone()) {
            self.current += 1;
            return true
        }

        panic!("expected {:?} here", token);

        false
    }
    

    fn current_token(&mut self) -> Option<Token> {
        return self.tokens.iter().nth(self.current).cloned();
    }
}