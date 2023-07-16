use crate::logic::{ast::parse, interpeter::interpret};

mod logic;

fn main() {
    let script = std::fs::read_to_string("./examples/spec_test.nl").unwrap();

    let tokens = logic::tokenizer::tokenize(&script);

    let parsed = parse(tokens).unwrap();

    println!("{:#?}", parsed);

    interpret(parsed);
}
