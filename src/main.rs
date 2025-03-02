use assembly::{lifetime, Instruction};
use ir::cfg;
use std::{collections::HashSet, env, fs, process::exit};
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

    let cfg = cfg::construct(ast);
    let tac = assembly::construct(cfg);
    // println!("{tac}");


    println!("section .bss");
    let mut identifiers = HashSet::new();
    for instruction in tac.blocks.iter().flatten() {
        match instruction {
            Instruction::Set { identifier, .. } => { identifiers.insert(identifier); }
            _ => {}
        }
    }
    for identifier in identifiers.iter() {
        println!("{identifier}: resq 1");
    }

    println!("section .text");
    println!("global _start");
    println!("_start:");

    let assembly = assembly::convert(tac);
    for instruction in assembly {
        println!("{instruction}")
    }

    println!("mov rax, 60");
    println!("mov rdi, [a]");
    println!("syscall");
}
