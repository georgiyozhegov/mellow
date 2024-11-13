use std::{fs, process::exit};

use syntax::{Lex, Parse};
use ir::constant_fold;

fn main() {
    let source = fs::read_to_string("source.mellow").unwrap();
    let lex = Lex::new(source.chars().peekable());
    let parse = Parse::new(lex.peekable());

    for statement in parse {
        let statement = match statement {
            Ok(statement) => statement,
            Err(error) => {
                eprintln!("{error}");
                exit(1);
            }
        };
        let statement = constant_fold::statement(statement);
        println!("{statement:?}");
    }
}
