use crate::token::Token;

pub struct Lexer {
    source: String,
    current: usize,
    line: usize,
    tokens: Vec<Token>
}

impl Lexer {
    pub fn new(source: String) -> Self {
        Self {
            source,
            current: 0,
            line: 1,
            tokens: Vec::new()
        }
    }

    pub fn tokenize(&mut self) -> &Vec<Token> {
        let chars: Vec<char> = self.source.chars().collect();

        while self.current < self.source.len() {
            if chars[self.current] == '\n' {
                self.line += 1
            }

            let token = self.next_char();
            self.tokens.push(token);

            self.current += 1
        }

        &self.tokens
    }

    fn next_char(&mut self) -> Token {
        match self.current_char() {
            Some(c) if c.is_alphabetic() => {
                self.parse_identifier()
            }

            _ => {
                Token::Illegal
            }
        }
    }

    fn parse_identifier(&mut self) -> Token {
        let start = self.current;

        while let Some(c) = self.current_char() {
            if c.is_alphabetic() {
                self.current += 1
            } else {
                break
            }
        }

        let identifier = &self.source[start..self.current];
        Token::Identifier(identifier.to_string())
    }

    fn current_char(&mut self) -> Option<char> {
        self.source.chars().nth(self.current)
    }
}