use std::{env, fs, process::exit};

fn main() {
    let path = env::args().nth(1).unwrap_or("source.mellow".into());
    let source = fs::read_to_string(path).unwrap();
    let ast = match syntax::construct(source.chars().peekable()) {
        Ok(ast) => ast,
        Err(error) => {
            eprintln!("{error}");
            exit(1);
        }
    };

    // println!("{ast:#?}");

    // let symbol_table = ir::symbol_table::construct(&ast);
    let cfg = ir::cfg::construct(ast);
    let tac = ir::tac::construct(cfg);

    println!("{tac:#?}");

    /*
    println!("section .bss");
    for (identifier, _) in symbol_table.variables() {
        println!("{identifier}: resq 1");
    }
    */

    println!("section .text");
    println!("global _start:");
    println!("_start:");

    /*
    for (identifier, meta) in symbol_table.functions() {
        if meta.external {
            println!("extern {identifier}");
        }
    }
    */

    let assembly = assembly::convert(tac);
    let assembly = assembly::optimize(assembly);
    for instruction in assembly {
        println!("{instruction}")
    }

    println!("mov rax, 60");
    println!("mov rdi, 0");
    println!("syscall");
}
