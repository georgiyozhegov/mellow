use std::{fs, process::exit};

use ir::{constant_fold, ir::ir};
use syntax::{Lex, Parse, SyntaxError};

fn main() {
    let source = fs::read_to_string("source.mellow").unwrap();
    let lex = Lex::new(source.chars().peekable());
    let parse = Parse::new(lex.peekable());

    let ir = ir(parse.collect::<Result<Vec<_>, SyntaxError>>().unwrap());

    for block in ir {
        println!("{block:?}");
    }

    /*
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
    */
}
