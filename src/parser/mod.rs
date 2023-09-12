use lib_lexin::{Token, Lexer, Section};

macro_rules! safe_index {
    ($index:expr, $len:expr) => {
        if $index >= $len {
            break;
        }
    };
}

// shitty alternative to hashmap because hashmap uses immutable borrow in get() :(
struct Names(Vec<(String, String)>);

impl Names {
    pub fn new() -> Names {
        return Names(Vec::new());
    }

    pub fn get(&self, name: &str) -> Option<String> {
        for value in &self.0 {
            if &value.0 == name {
                return Some(value.1.clone());
            }
        }
        return None;
    }

    pub fn push(&mut self, name: &str, value: String) {
        self.0.push((name.to_string(), value));
    }
}

pub struct Parser<'a> {
    tokens: &'a Vec<Token>,
}

impl<'a> Parser<'a> {
    pub fn parse(&self) -> Result<Lexer, Box<dyn std::error::Error>> {
        let mut lexer = Lexer::new(&[], &[], &[]);
        let mut names: Names = Names::new();
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
                            let name = names.get(value).unwrap_or(String::new());
                            lexer.symbols.push((value.as_bytes()[0] as char, name));
                            index += 1;
                            safe_index!(index, self.tokens.len());
                        }
                        continue;
                    } else if keyword == "_name" {
                        index += 1;
                        while let Token::Section(_, value) = &self.tokens[index] {
                            index += 1;
                            names.push(value, self.tokens[index].is_section("string")?);
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

                            let end = self.tokens[index + 1].is_section("string")?;
                            let name = self.tokens[index + 2].is_section("string")?;
                            lexer.sections.push(Section::new(
                                &name,
                                start,
                                &end,
                            ));
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


