use std::fs;

#[derive(Debug)]
pub enum Token {
    Keyword(String),
    Section(String, String),
    Symbol(char, String),
    Ident(String),
}

#[derive(PartialEq, Eq)]
enum Mode {
    Section,
    Normal,
}

#[derive(Debug)]
pub struct Section {
    pub name: String,
    pub start: String,
    pub end: String,
}

impl Section {
    pub fn new() -> Section {
        return Section {
            name: String::new(),
            start: String::new(),
            end: String::new(),
        };
    }

    pub fn from_end(end: String) -> Section {
        return Section {
            name: String::new(),
            start: String::new(),
            end,
        };
    }
}

#[derive(Debug)]
pub struct Lexer<'a> {
    pub keywords: Vec<&'a str>,
    pub sections: Vec<Section>,
    pub symbols: Vec<(char, String)>,
}

#[derive(Debug)]
enum Value {
    Start(String),
    End(String, String),
}

enum StartOrSection<'a> {
    Start(Vec<String>),
    Section(&'a Section),
}

impl<'a> Lexer<'a> {
    fn symbols_contain(&self, value: &char) -> Option<String> {
        for symbol in &self.symbols {
            if symbol.0 == *value {
                return Some(symbol.1.clone());
            }
        }
        return None;
    }

    fn section_exists(&self, start: &str, end: &str) -> Result<String, ()> {
        for section in &self.sections {
            if section.start == start && section.end == end {
                return Ok(section.name.to_string());
            }
        }
        return Err(());
    }

    fn is_section(&self, value: Value) -> Result<StartOrSection, ()> {
        let mut matches: Vec<String> = Vec::new();
        for section in &self.sections {
            if let Value::Start(start) = &value {
                if &section.start == start {
                    matches.push(section.end.clone());
                }
            } else if let Value::End(start, end) = &value {
                if &section.end == end && &section.start == start {
                    return Ok(StartOrSection::Section(section)); // matches is not really needed here
                }
            }
        }

        if matches.len() != 0 {
            return Ok(StartOrSection::Start(matches));
        }
        return Err(());
    }

    fn lex_token(&self, token: &String) -> Option<Token> {
        if token != "\n" && token != "" {
            if self.keywords.contains(&token.as_str()) {
                return Some(Token::Keyword(token.clone()));
            } else if token.len() == 1 {
                let character = token.chars().collect::<Vec<char>>()[0];
                if let Some(symbol_name) = self.symbols_contain(&character) {
                    return Some(Token::Symbol(character, symbol_name));
                } else {
                    return Some(Token::Ident(token.clone()));
                }
            } else if let Ok(name) = self.section_exists(&token[0..1], &token[token.len()-1..token.len()]) {
                return Some(Token::Section(name, token[1..token.len() - 1].to_string()));
            } else {
                return Some(Token::Ident(token.clone()));
            }
        }
        return None;
    }

    pub fn tokenize(&mut self, filename: &str) -> Result<Vec<Token>, Box<dyn std::error::Error>> {
        if self.symbols_contain(&' ').is_none() {
            self.symbols.push((' ', "Space".to_string()));
        }
        let bytes = fs::read(filename)?;

        let mut mode = Mode::Normal;
        let mut token = String::new();
        let mut tokens: Vec<Token> = Vec::new();
        let mut section: Vec<Section> = Vec::new();

        let mut index = 0;
        while index < bytes.len() {
            let byte = &bytes[index];
            let character = String::from_utf8(vec![byte.clone()])?;
            if (index + 1) < bytes.len() {
                if mode == Mode::Normal {
                    if let Ok(StartOrSection::Start(ends)) = self.is_section(Value::Start(character.clone())) {
                        token = token + &character;
                        for end in ends {
                            section.push(Section::from_end(end.clone()));
                            let idx = section.len() - 1;
                            section[idx].start = character.clone();
                        }
                        mode = Mode::Section;
                    } else if character.as_str() == "\n" {
                        self.lex_token(&token).map(|t| tokens.push(t));
                        token = String::new();
                    } else if character.as_str() != " " {
                        token = token + &character;
                    }
                    if (self.symbols_contain(&char::from(byte.clone())).is_some() || self.symbols_contain(&char::from(bytes[index + 1])).is_some()) &&
                       section.len() == 0 { // making sure we aint lexing symbols when in a section
                        self.lex_token(&token).map(|t| tokens.push(t));
                        token = String::new();
                    }
                } else if mode == Mode::Section {
                    if let Ok(_) = self.is_section(Value::End(section[0].start.to_string(), character.clone())) { // index doesnt matter here because its all the same start
                        token = token + &character;
                        self.lex_token(&token).map(|t| tokens.push(t));
                        section = Vec::new();
                        token = String::new();
                        mode = Mode::Normal;
                    } else {
                        token = token + &character;
                    }
                }
            }
            index += 1;
        }
        return Ok(tokens);
    }
}


