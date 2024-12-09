use std::{fs, process::exit};

// use ir::{constant_fold, ir::ir};
use syntax;

fn main() {
    let source = fs::read_to_string("source.mellow").unwrap();
    let ast = match syntax::construct(source.chars().peekable()) {
        Ok(ast) => ast,
        Err(error) => {
            eprintln!("{error}");
            exit(1);
        }
    };

    for statement in ast {
        println!("{statement:?}");
    }
}
