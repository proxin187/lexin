mod parser;
mod format;

use argin::Argin;
use lib_lexin::{Lexer, Section};
use parser::Parser;
use std::process;

fn args() -> Argin {
    let mut arg = Argin::new();
    arg.add_positional_arg();
    arg.add_positional_arg();
    arg.add_value("-format");
    return arg.parse();
}

fn help() {
        println!("
Usage: lexin [config] [target] [options]
  Options:
    -format: [formats] defaults to python
      formats:
        json: output tokens in json format
        python: output tokens in python format
                 ");
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let arg = args();
    let config = arg.pos_arg.get(0);
    let source = arg.pos_arg.get(1);

    if config.is_none() || source.is_none() {
        help();
        process::exit(1);
    }

    let mut lexer = Lexer::new(
        &[
            "_sections",
            "_keywords",
            "_symbols",
            "_name",
        ],
        &[
            Section::new("comment", "/*", "*/"),
        ],
        &[
            ('-', "Dash"),
        ],
    );

    lexer.load_file(config.unwrap())?;

    let tokens = lexer.tokenize()?;

    let parser = Parser::new(&tokens);
    let lexer_config = parser.parse();

    if let Err(error) = lexer_config {
        println!("Parser: {:?}", error);
        process::exit(1);
    }

    let mut lexer_config = lexer_config.unwrap();

    lexer_config.load_file(source.unwrap())?;
    let tokens = lexer_config.tokenize()?;

    if let Some(format) = arg.values.get("-format") {
        format::format(tokens, format)?;
    } else { // default: python
        format::format(tokens, "python")?;
    }

    return Ok(());
}


