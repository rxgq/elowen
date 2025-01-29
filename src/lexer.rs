use std::collections::HashMap;

use crate::token::Token;

pub struct Lexer {
    source: String,
    current: usize,
    line: usize,
    tokens: Vec<Token>,
    keywords: HashMap<&'static str, Token>
}

impl Lexer {
    fn declare_verb(keywords: &mut HashMap<&'static str, Token>, verb: &'static str) {
        keywords.insert(verb, Token::Verb(verb.to_string()));
    }

    fn declare_preposition(keywords: &mut HashMap<&'static str, Token>, preposition: &'static str) {
        keywords.insert(preposition, Token::Preposition(preposition.to_string()));
    }

    fn declare_noun(keywords: &mut HashMap<&'static str, Token>, noun: &'static str) {
        keywords.insert(noun, Token::Noun(noun.to_string()));
    }

    fn declare_determiner(keywords: &mut HashMap<&'static str, Token>, determiner: &'static str) {
        keywords.insert(determiner, Token::Determiner(determiner.to_string()));
    }
}

impl Lexer {
    pub fn new(source: String) -> Self {
        let mut keywords = HashMap::new();

        Self::declare_verb(&mut keywords, "declare");
        Self::declare_verb(&mut keywords, "set");
        Self::declare_verb(&mut keywords, "assign");
        Self::declare_verb(&mut keywords, "let");

        Self::declare_preposition(&mut keywords, "as");
        Self::declare_preposition(&mut keywords, "to");

        Self::declare_noun(&mut keywords, "variable");
        Self::declare_noun(&mut keywords, "thing");

        Self::declare_determiner(&mut keywords, "the");
        Self::declare_determiner(&mut keywords, "an");
        Self::declare_determiner(&mut keywords, "a");

        Self {
            source,
            current: 0,
            line: 1,
            tokens: Vec::new(),
            keywords
        }
    }

    pub fn tokenize(&mut self) -> &Vec<Token> {
        while self.current < self.source.len() {
            if let Some(c) = self.current_char() {
                if c == '\n' {
                    self.line += 1
                }

                if c.is_whitespace() {
                    self.advance();
                    continue
                }
            }

            let token = self.next_char();
            self.tokens.push(token);

            self.advance()
        }

        &self.tokens
    }

    fn next_char(&mut self) -> Token {
        match self.current_char() {
            Some(c) if c.is_alphabetic() => self.parse_identifier(),
            Some(c) if c.is_numeric() => self.parse_numeric(),
            Some(c) if c == '\'' => self.parse_char(),
            Some(c) if c == '\"' => self.parse_string(),
            Some(_) => {
                let current_char = self.current_char(); 
                self.syntax_error(&format!("invalid symbol '{}'", current_char.unwrap()));
                
                Token::Illegal(current_char.unwrap())
            },
            None => Token::Eof
        }
    }

    fn parse_string(&mut self) -> Token {
        self.advance();
        
        let start = self.current;
        while let Some(c) = self.current_char() {
            if c == '\"' {
                break
            }

            self.advance();
        }

        let str = &self.source[start..self.current];

        Token::String(String::from(str))
    }

    fn parse_char(&mut self) -> Token {
        self.advance();

        let c = match self.current_char() {
            Some(c) => c,
            None => {
                self.syntax_error("unterminated char literal");
                return Token::Illegal('\'');
            }
        };
        self.advance();

        match self.current_char() {
            Some('\'') => {
                self.advance();
                return Token::Char(c)
            }
            _ => {
                self.syntax_error("invalid char literal");
                Token::Illegal('\'')
            }
        }
    }

    fn parse_numeric(&mut self) -> Token {
        let start = self.current;
        
        let mut is_decimal = false;
        while let Some(c) = self.current_char() {
            match c {                
                c if c.is_numeric() || c == '.' => {
                    if c == '.' && is_decimal {
                        self.syntax_error("invalid numeric literal");
                    } else if c == '.' {
                        is_decimal = true
                    }

                    self.advance();
                }
                _ => break
            }
        }

        let numeric = &self.source[start..self.current];
        self.current -= 1;

        if is_decimal {
            return Token::Float(numeric.parse().unwrap());
        }
        
        Token::Integer(numeric.parse().unwrap())
    }

    fn parse_identifier(&mut self) -> Token {
        let start = self.current;

        while let Some(c) = self.current_char() {
            if c.is_alphabetic() {
                self.advance()
            } else {
                break
            }
        }

        let identifier = &self.source[start..self.current];
        self.current -= 1;

        if let Some(token) = self.keywords.get(identifier) {
            return token.clone();
        }
        
        Token::Identifier(identifier.to_string())
    }

    fn syntax_error(&mut self, error: &str) {
        println!("syntax error: {} on line {}", error, self.line)
    }

    fn current_char(&mut self) -> Option<char> {
        self.source.chars().nth(self.current)
    }

    fn advance(&mut self) {
        self.current += 1;
    }
}