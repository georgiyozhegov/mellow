use syntax::Lex;

use std::fs;

fn main() {
    let source = fs::read_to_string("source.mellow").unwrap();
    let lex = Lex::new(source.chars().peekable());
    for token in lex {
        println!("{token:?}");
    }
}
