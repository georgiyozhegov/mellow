use std::{env, fs, process::exit};

use ir::cfg;
use syntax;

fn main() {
    let path = env::args().skip(1).next().unwrap_or("source.mellow".into());
    let source = fs::read_to_string(path).unwrap();
    let ast = match syntax::construct(source.chars().peekable()) {
        Ok(ast) => ast,
        Err(error) => {
            eprintln!("{error}");
            exit(1);
        }
    };

    let cfg = cfg::construct(ast);
    let tac = assembly::construct(cfg);
    println!("{tac}");
}
