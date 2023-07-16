use crate::logic::{ast::parse, interpeter::interpret};

mod logic;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        println!("Error: No file path provided");
        return;
    }

    let path = &args[1];

    let script = match std::fs::read_to_string(path) {
        Ok(script) => script,
        Err(e) => {
            println!("Error: {}", e);
            return;
        }
    };

    let tokens = logic::tokenizer::tokenize(&script);

    let parsed = match parse(tokens) {
        Ok(parsed) => parsed,
        Err(e) => {
            println!("Error: {}", e);
            return;
        }
    };

    interpret(parsed);
}
