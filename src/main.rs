mod lexer;
mod parser;

use lexer::Token;
use parser::Parser;
use std::env;
use std::process;

fn py_format(name: &str, value: String) -> String {
    return format!("\n    ('{}', '{}'),", name, value);
}

fn format(tokens: Vec<lexer::Token>, format: &str) -> Result<(), Box<dyn std::error::Error>> {
    let mut buffer = String::new();

    for token in tokens {
        match token {
            Token::Keyword(keyword) => {
                if format == "python" {
                    buffer = buffer + &py_format("keyword", keyword);
                }
            },
            Token::Symbol(symbol, name) => {
                if format == "python" {
                    let value = if name.is_empty() { symbol.to_string() } else { name };
                    buffer = buffer + &py_format("symbol", value);
                }
            },
            Token::Ident(ident) => {
                if format == "python" {
                    buffer = buffer + &py_format("ident", ident);
                }
            },
            Token::Section(name, value) => {
                if format == "python" {
                    buffer = buffer + &py_format(&name, value);
                }
            },
        }
    }

    if format == "python" {
        buffer.insert(0, '[');
        buffer = buffer + "\n    ('EOF', '')\n]";
        println!("{}", buffer);
    } else {
        println!("Unknown format: {}", format);
    }

    return Ok(());
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = env::args().collect::<Vec<String>>();
    if args.len() < 3 {
        println!("
Usage: lexin [config(*.lex)] [target] [options]
  Options:
    -format: [formats] defaults to python
      formats:
        json: output tokens in json format
        python: output tokens in python format
                 ");
        process::exit(1);
    }

    let mut token = lexer::Lexer {
        keywords: vec![
            "_sections",
            "_keywords",
            "_symbols",
            "_name",
        ],
        sections: vec![
            lexer::Section {
                name: "string".to_string(),
                start: "\"".to_string(),
                end: "\"".to_string(),
            },
        ],
        symbols: vec![
            ('-', "Dash".to_string()),
        ],
    };

    let tokens = token.tokenize(&args[1])?;

    let parser = Parser::new(&tokens);
    let lexer_config = parser.parse();

    if let Err(error) = lexer_config {
        println!("Parser: {:?}", error);
        process::exit(1);
    }

    let tokens = lexer_config.unwrap().tokenize(&args[2])?;

    if args.len() < 5 {
        format(tokens, "python")?;
    } else if &args[3] == "-format" && args.len() > 4 {
        format(tokens, &args[4])?;
    } else {
        format(tokens, "python")?;
    }

    return Ok(());
}


