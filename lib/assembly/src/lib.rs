mod register;
use std::{
    collections::HashMap,
    fmt::{self, Display, Formatter},
};

use ir::{tac::Tac, Instruction};
pub use register::{Register, RegisterKind, Size};

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
    Sub(Data, Data),
    Imul(Data, Data),
    Idiv(Data),
    Cqo,
    Sete(Data), // =
    Setg(Data), // >
    Setl(Data), // <
    Jmp(u64),
    Je(u64),
    Call(String),
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
            Self::Sub(to, value) => {
                write!(f, "sub {to}, {value}")
            }
            Self::Imul(to, value) => {
                write!(f, "imul {to}, {value}")
            }
            Self::Idiv(data) => {
                write!(f, "idiv {data}")
            }
            Self::Cqo => {
                write!(f, "cqo")
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
            Self::Call(label) => {
                write!(f, "call {label}")
            }
        }
    }
}

fn register(id: u64, size: Size, allocated: &HashMap<u64, RegisterKind>) -> Data {
    let kind = allocated.get(&id).unwrap().clone();
    let register = Register::new(kind, size);
    Data::Register(register)
}

fn generate(
    block: Vec<Instruction>,
    output: &mut Vec<Assembly>,
    allocated: &HashMap<u64, RegisterKind>,
) {
    for instruction in block {
        match instruction {
            Instruction::Integer { to, value } => {
                let to = register(to, Size::Qword, allocated);
                let value = Data::Integer(value);
                output.push(Assembly::Mov(to, value));
            }
            Instruction::Add { to, left, right } => {
                let to = register(to, Size::Qword, allocated);
                let left = register(left, Size::Qword, allocated);
                let right = register(right, Size::Qword, allocated);
                output.push(Assembly::Add(left.clone(), right));
                output.push(Assembly::Mov(to, left));
            }
            Instruction::Subtract { to, left, right } => {
                let to = register(to, Size::Qword, allocated);
                let left = register(left, Size::Qword, allocated);
                let right = register(right, Size::Qword, allocated);
                output.push(Assembly::Sub(left.clone(), right));
                output.push(Assembly::Mov(to, left));
            }
            Instruction::Multiply { to, left, right } => {
                let to = register(to, Size::Qword, allocated);
                let left = register(left, Size::Qword, allocated);
                let right = register(right, Size::Qword, allocated);
                output.push(Assembly::Imul(left.clone(), right));
                output.push(Assembly::Mov(to, left));
            }
            Instruction::Divide { to, left, right } => {
                let to = register(to, Size::Qword, allocated);
                let left = register(left, Size::Qword, allocated);
                let right = register(right, Size::Qword, allocated);
                let rax = Data::Register(Register::new(RegisterKind::A, Size::Qword));
                output.push(Assembly::Mov(rax.clone(), left));
                output.push(Assembly::Cqo);
                output.push(Assembly::Idiv(right));
                output.push(Assembly::Mov(to, rax));
            }
            Instruction::Equal { to, left, right } => {
                let byte_to = register(to, Size::Byte, allocated);
                let qword_to = register(to, Size::Qword, allocated);
                let left = register(left, Size::Qword, allocated);
                let right = register(right, Size::Qword, allocated);
                output.push(Assembly::Cmp(left, right));
                output.push(Assembly::Mov(qword_to, Data::Integer(0)));
                output.push(Assembly::Sete(byte_to));
            }
            Instruction::Greater { to, left, right } => {
                let byte_to = register(to, Size::Byte, allocated);
                let qword_to = register(to, Size::Qword, allocated);
                let left = register(left, Size::Qword, allocated);
                let right = register(right, Size::Qword, allocated);
                output.push(Assembly::Cmp(left, right));
                output.push(Assembly::Mov(qword_to, Data::Integer(0)));
                output.push(Assembly::Setg(byte_to));
            }
            Instruction::Less { to, left, right } => {
                let byte_to = register(to, Size::Byte, allocated);
                let qword_to = register(to, Size::Qword, allocated);
                let left = register(left, Size::Qword, allocated);
                let right = register(right, Size::Qword, allocated);
                output.push(Assembly::Cmp(left, right));
                output.push(Assembly::Mov(qword_to, Data::Integer(0)));
                output.push(Assembly::Setl(byte_to));
            }
            Instruction::Set { identifier, from } => {
                let to = Data::Identifier(identifier);
                let from = register(from, Size::Qword, allocated);
                output.push(Assembly::Mov(to, from));
            }
            Instruction::Get { to, identifier } => {
                let to = register(to, Size::Qword, allocated);
                let from = Data::Identifier(identifier);
                output.push(Assembly::Mov(to, from));
            }
            Instruction::Jump { to } => {
                output.push(Assembly::Jmp(to));
            }
            Instruction::JumpIf { condition, to } => {
                let condition = register(condition, Size::Qword, allocated);
                output.push(Assembly::Cmp(condition, Data::Integer(1)));
                output.push(Assembly::Je(to));
            }
            Instruction::Call { label, value } => {
                let value = register(value, Size::Qword, allocated);
                let rdi = Data::Register(Register::new(RegisterKind::Di, Size::Qword));
                output.push(Assembly::Mov(rdi, value));
                output.push(Assembly::Call(label));
            }
            _ => todo!(),
        }
    }
}

fn map(allocated: HashMap<u64, u64>) -> HashMap<u64, RegisterKind> {
    let mut mapped = HashMap::new();
    let registers = RegisterKind::allocable();
    for (id, register) in allocated.iter() {
        mapped.insert(*id, registers.get(*register as usize).unwrap().clone());
    }
    mapped
}

pub fn convert(tac: Tac) -> Vec<Assembly> {
    let mut output = Vec::new();
    let registers = RegisterKind::allocable().len() as u64;
    let allocated = map(ir::allocate(&tac, registers));
    for (id, block) in tac.blocks.into_iter().enumerate() {
        output.push(Assembly::Label(id as u64));
        generate(block, &mut output, &allocated);
    }
    output
}
