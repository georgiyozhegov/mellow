use std::{
    collections::HashMap,
    fmt::{self, Display, Formatter},
};

use crate::{
    lifetime::{self, scan, Register, RegisterKind, Size},
    Instruction, Tac,
};

#[derive(Debug, Clone)]
pub enum Data {
    Register(Register),
    Stack(u8),
    Integer(i128),
    Identifier(String), // NOTE: Temporary, will be removed
}

impl Display for Data {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            Self::Register(register) => write!(f, "{register}"),
            Self::Stack(offset) => write!(f, "[rsp - {offset}]"),
            Self::Integer(value) => write!(f, "{value}"),
            Self::Identifier(identifier) => write!(f, "[{identifier}]"),
        }
    }
}

#[derive(Debug)]
pub enum Assembly {
    Label(u64),
    Mov(Data, Data),
    Cmp(Data, Data),
    Add(Data, Data),
    Sete(Data), // =
    Setg(Data), // >
    Setl(Data), // <
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
            Self::Je(label) => {
                write!(f, "je _{label}")
            }
        }
    }
}

fn qword(kind: RegisterKind) -> Data {
    Data::Register(Register::new(kind, Size::Qword))
}

fn generate(
    block: Vec<Instruction>,
    output: &mut Vec<Assembly>,
    allocated: &HashMap<u64, RegisterKind>,
) {
    for instruction in block {
        match instruction {
            Instruction::Integer { to, value } => {
                let to = qword(allocated.get(&to).unwrap().clone());
                let value = Data::Integer(value);
                output.push(Assembly::Mov(to, value));
            }
            Instruction::Add { to, left, right } => {
                let to = qword(allocated.get(&to).unwrap().clone());
                let left = qword(allocated.get(&left).unwrap().clone());
                let right = qword(allocated.get(&right).unwrap().clone());
                output.push(Assembly::Add(left.clone(), right));
                output.push(Assembly::Mov(to, left));
            }
            Instruction::Set { identifier, from } => {
                let to = Data::Identifier(identifier);
                let from = qword(allocated.get(&from).unwrap().clone());
                output.push(Assembly::Mov(to, from));
            }
            _ => todo!(),
        }
    }
}

pub fn convert(tac: Tac) -> Vec<Assembly> {
    let mut output = Vec::new();
    let lifetimes = lifetime::scan(&tac);
    let allocated = lifetime::allocate(lifetimes);
    for (id, block) in tac.blocks.into_iter().enumerate() {
        output.push(Assembly::Label(id as u64));
        generate(block, &mut output, &allocated);
    }
    output
}
