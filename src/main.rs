use std::{env, fs, process::exit};

use mellow_lex::{Lex, SourceBuffer};
use mellow_parse::Parse;

fn main() {
    let path = env::args().nth(1).unwrap_or("source.mellow".into());
    let source = fs::read_to_string(path).unwrap();
    let args: Vec<_> = env::args().skip(1).collect();

    let source_buffer = SourceBuffer::from(source);
    let lex = Lex::new(source_buffer);
    let parse = Parse::new(lex.peekable());

    let ast = match parse.collect() {
        Ok(ast) => ast,
        Err(error) => {
            eprintln!("error: {error}");
            exit(1);
        }
    };

    if args.iter().any(|flag| flag == "--ast") {
        println!("{ast:#?}");
    }

    let symbol_table = match ir::symbol_table::construct(&ast) {
        Ok(table) => table,
        Err(error) => {
            eprintln!("{error:?}");
            exit(1);
        }
    };

    if args.iter().any(|flag| flag == "--st") {
        println!("{symbol_table:#?}");
    }

    let cfg = ir::cfg::construct(ast);
    if args.iter().any(|flag| flag == "--cfg") {
        println!("{cfg:#?}");
    }

    let tac = ir::tac::construct(cfg);
    if args.iter().any(|flag| flag == "--tac") {
        println!("{tac:#?}");
    }

    println!("section .bss");
    for (identifier, _) in symbol_table.variables() {
        println!("{}: resq 1", identifier.name);
    }

    println!("section .text");
    println!("global _start:");
    println!("_start:");

    for (identifier, meta) in symbol_table.functions() {
        if meta.external {
            println!("extern {}", identifier.name);
        }
    }

    let assembly = assembly::convert(tac);
    let assembly = assembly::optimize(assembly);
    for instruction in assembly {
        println!("{instruction}")
    }

    println!("mov rax, 60");
    println!("mov rdi, 0");
    println!("syscall");
}
