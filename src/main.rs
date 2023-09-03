mod lexer;
mod parser;

use parser::Parser;
use std::env;
use std::process;

// NOTE: FINISH LATER
fn format(tokens: Vec<lexer::Token>, format: &str) {
    
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = env::args().collect::<Vec<String>>();
    if args.len() < 2 {
        println!("Usage: lexin [config] [target]");
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
    println!("{:?}", tokens);

    return Ok(());
}


