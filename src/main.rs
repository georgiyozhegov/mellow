use std::{fs, process::exit};

use ir::cfg;
use syntax;

fn main() {
    let source = fs::read_to_string("source.mellow").unwrap();
    let ast = syntax::construct(source.chars().peekable());
    let ast = match syntax::construct(source.chars().peekable()) {
        Ok(ast) => ast,
        Err(error) => {
            eprintln!("{error}");
            exit(1);
        }
    };

    let cfg = cfg::construct(ast);
    println!("{:#?}", cfg);
}
