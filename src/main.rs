use std::{collections::HashSet, env, fs, process::exit};

use assembly::Instruction;
use ir::cfg;
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

    // println!("{cfg:#?}");

    let tac = assembly::construct(cfg);
    // println!("{tac}");

    println!("section .bss");
    let mut identifiers = HashSet::new();
    let mut labels = HashSet::new();
    for instruction in tac.blocks.iter().flatten() {
        match instruction {
            Instruction::Set { identifier, .. } => {
                identifiers.insert(identifier);
            }
            Instruction::Call { label, .. } => {
                labels.insert(label);
            }
            _ => {}
        }
    }
    for identifier in identifiers.iter() {
        println!("{identifier}: resq 1");
    }

    println!("section .text");
    println!("global _start:");
    println!("_start:");

    for label in labels.iter() {
        println!("extern {label}");
    }

    let assembly = assembly::convert(tac);
    for instruction in assembly {
        println!("{instruction}")
    }

    println!("mov rax, 60");
    println!("mov rdi, 0");
    println!("syscall");
}
