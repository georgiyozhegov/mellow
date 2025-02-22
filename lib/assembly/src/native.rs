use std::{collections::HashSet, fmt::{self, Display, Formatter}};

use crate::{Instruction, Tac};

#[derive(Debug)]
pub enum Assembly {
    Label(u64),
    Mov(String, String),
    Cmp(&'static str, &'static str),
    Add(&'static str, &'static str),
    Sete(&'static str), // =
    Setg(&'static str), // >
    Setl(&'static str), // <
    Jmp(u64),
    Je(u64),
}

impl Display for Assembly {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            Self::Label(id) => {
                write!(f, "_{id}:")
            }
            Self::Mov(to, from) => {
                write!(f, "mov {to}, {from}")
            }
            Self::Cmp(first, second) => {
                write!(f, "cmp {first}, {second}")
            }
            Self::Add(to, value) => {
                write!(f, "add {to}, {value}")
            }
            Self::Sete(register) => {
                write!(f, "sete {register}")
            }
            Self::Setg(register) => {
                write!(f, "setg {register}")
            }
            Self::Setl(register) => {
                write!(f, "setl {register}")
            }
            Self::Jmp(label) => {
                write!(f, "jmp _{label}")
            }
            Self::Je(label)   => {
                write!(f, "je _{label}")
            }
        }
    }
}

pub const REGISTERS: [&str; 6] = ["rax", "rbx", "rcx", "rdx", "rsi", "rdi"];

fn generate(block: Vec<Instruction>, output: &mut Vec<Assembly>, variables: &mut HashSet<String>) {
    for instruction in block {
        match instruction {
            Instruction::Integer { to, value } => {
                let to = REGISTERS[to as usize];
                output.push(Assembly::Mov(to.to_string(), value.to_string()));
            }
            Instruction::Add { to, left, right } => {
                let to = REGISTERS[to as usize];
                let left = REGISTERS[left as usize];
                let right = REGISTERS[right as usize];
                output.push(Assembly::Add(left, right));
                output.push(Assembly::Mov(to.into(), left.into()));
            }
            Instruction::Set { identifier, from } => {
                variables.insert(identifier.clone());
                let to = format!("[{identifier}]");
                let from = REGISTERS[from as usize];
                output.push(Assembly::Mov(to, from.into()));
            }
            _ => todo!(),
        }
    }
}

pub fn convert(tac: Tac) -> Vec<Assembly> {
    let mut output = Vec::new();
    let mut variables = HashSet::new();
    for (id, block) in tac.blocks.into_iter().enumerate() {
        output.push(Assembly::Label(id as u64));
        generate(block, &mut output, &mut variables);
    }
    output
}
