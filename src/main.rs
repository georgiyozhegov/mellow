use syntax::{Lex, Parse};

use std::fs;

fn main() {
    let source = fs::read_to_string("source.mellow").unwrap();
    let lex = Lex::new(source.chars().peekable());
    let parse = Parse::new(lex.peekable());
    for statement in parse {
        let statement = statement.unwrap();
        println!("{statement:?}");
    }
}
