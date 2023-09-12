use lib_lexin::Token;

fn tuple_format(name: &str, value: String) -> String {
    return format!("\n    ('{}', '{}'),", name, value);
}

pub fn format(tokens: Vec<Token>, format: &str) -> Result<(), Box<dyn std::error::Error>> {
    let mut buffer = String::new();

    for token in tokens {
        match token {
            Token::Keyword(keyword) => {
                if format == "python" || format == "json" {
                    buffer = buffer + &tuple_format("keyword", keyword);
                }
            },
            Token::Symbol(symbol, name) => {
                if format == "python" || format == "json" {
                    let value = if name.is_empty() { symbol.to_string() } else { name };
                    buffer = buffer + &tuple_format("symbol", value);
                }
            },
            Token::Ident(ident) => {
                if format == "python" || format == "json" {
                    buffer = buffer + &tuple_format("ident", ident);
                }
            },
            Token::Section(name, value) => {
                if format == "python" || format == "json" {
                    buffer = buffer + &tuple_format(&name, value);
                }
            },
            Token::Integer(integer) => {
                if format == "python" || format == "json" {
                    buffer = buffer + &tuple_format("integer", integer.to_string());
                }
            },
            Token::Float(float) => {
                if format == "python" || format == "json" {
                    buffer = buffer + &tuple_format("integer", float.to_string());
                }
            },
        }
    }

    if format == "python" {
        buffer.insert(0, '[');
        buffer = buffer + "\n    ('EOF', '')\n]";
        println!("{}", buffer);
    } else if format == "json" {
        buffer = "tokens: [".to_string() + &buffer + "\n]";
        println!("{}", buffer);
    } else {
        println!("Unknown format: {}", format);
    }

    return Ok(());
}

