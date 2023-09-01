use crate::lexer::{Token, Lexer, Section};
use std::collections::HashMap;

macro_rules! safe_index {
    ($index:expr, $len:expr) => {
        if $index >= $len {
            break;
        }
    };
}

pub struct Parser<'a> {
    tokens: &'a Vec<Token>,
}

impl<'a> Parser<'a> {
    fn get_str(&self, token: &Token) -> Result<String, Box<dyn std::error::Error>> {
        if let Token::Section(_, string) = token {
            return Ok(string.clone());
        } else {
            return Err(format!("expected String but got {:?}", token).into());
        }

    }

    pub fn parse(&self) -> Result<Lexer, Box<dyn std::error::Error>> {
        let mut lexer = Lexer {
            keywords: Vec::new(),
            sections: Vec::new(),
            symbols: Vec::new(),
        };
        let mut names: HashMap<String, String> = HashMap::new();
        let mut index = 0;

        while index < self.tokens.len() {
            match &self.tokens[index] {
                Token::Keyword(keyword) => {
                    if keyword == "_keywords" {
                        index += 1;
                        while let Token::Section(_, value) = &self.tokens[index] {
                            lexer.keywords.push(value);
                            index += 1;
                            safe_index!(index, self.tokens.len());
                        }
                        continue;
                    } else if keyword == "_symbols" {
                        index += 1;
                        while let Token::Section(_, value) = &self.tokens[index] {
                            lexer.symbols.push((value.as_bytes()[0] as char, names.get(value).unwrap_or(&"".to_string()).to_string()));
                            index += 1;
                            safe_index!(index, self.tokens.len());
                        }
                        continue;
                    } else if keyword == "_name" {
                        index += 1;
                        while let Token::Section(_, value) = &self.tokens[index] {
                            index += 1;
                            names.insert(value.clone(), self.get_str(&self.tokens[index])?);
                            index += 1;
                            safe_index!(index, self.tokens.len());
                        }
                        continue;
                    } else if keyword == "_sections" {
                        index += 1;

                        while let Token::Section(_, start) = &self.tokens[index] {
                            index += 1;
                            if if let Token::Symbol(symbol, _) = &self.tokens[index] { *symbol != '-' } else { true } {
                                return Err(format!("expected Symbol but got {:?}", self.tokens[index]).into());
                            }

                            let end = self.get_str(&self.tokens[index + 1])?;
                            let name = self.get_str(&self.tokens[index + 2])?;
                            lexer.sections.push(Section {
                                name: name.clone(),
                                start: start.clone(),
                                end,
                            });
                            index += 3;
                            safe_index!(index, self.tokens.len());
                        }
                        continue;
                    }
                },
                _ => {},
            }
            index += 1;
        }
        return Ok(lexer);
    }

    pub fn new(tokens: &Vec<Token>) -> Parser {
        return Parser {
            tokens,
        };
    }
}


