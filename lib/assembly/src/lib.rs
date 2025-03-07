mod assembly;
mod data;
mod register;
mod optimize;

use std::collections::HashMap;

use assembly::Assembly;
use data::Data;
use ir::Instruction;
use register::{Register, RegisterKind, Size};
pub use optimize::optimize;

macro_rules! arithmetic {
    ($operation:ident, $to:expr, $left:expr, $right:expr, $allocated:expr, $output:expr) => {
        let to = register($to, Size::Qword, $allocated);
        let left = register($left, Size::Qword, $allocated);
        let right = register($right, Size::Qword, $allocated);
        $output.extend(vec![
            Assembly::$operation(left.clone(), right),
            Assembly::Mov(to, left),
        ]);
    };
}

macro_rules! comparision {
    ($operation:ident, $to:expr, $left:expr, $right:expr, $allocated:expr, $output:expr) => {
        let byte_to = register($to, Size::Byte, $allocated);
        let qword_to = register($to, Size::Qword, $allocated);
        let left = register($left, Size::Qword, $allocated);
        let right = register($right, Size::Qword, $allocated);
        $output.extend(vec![
            Assembly::Cmp(left, right),
            Assembly::Mov(qword_to, Data::Integer(0)),
            Assembly::$operation(byte_to),
        ]);
    };
}

fn register(id: u64, size: Size, allocated: &HashMap<u64, RegisterKind>) -> Data {
    let kind = allocated.get(&id).unwrap().clone();
    let register = Register::new(kind, size);
    Data::Register(register)
}

fn generate(
    instruction: Instruction,
    output: &mut Vec<Assembly>,
    allocated: &HashMap<u64, RegisterKind>,
) {
    match instruction {
        Instruction::Label(id) => {
            output.push(Assembly::Label(id));
        }
        Instruction::Integer { to, value } => {
            let to = register(to, Size::Qword, allocated);
            let value = Data::Integer(value);
            output.push(Assembly::Mov(to, value));
        }
        Instruction::Add { to, left, right } => {
            arithmetic!(Add, to, left, right, allocated, output);
        }
        Instruction::Subtract { to, left, right } => {
            arithmetic!(Sub, to, left, right, allocated, output);
        }
        Instruction::Multiply { to, left, right } => {
            arithmetic!(Imul, to, left, right, allocated, output);
        }
        Instruction::Divide { to, left, right } => {
            let to = register(to, Size::Qword, allocated);
            let left = register(left, Size::Qword, allocated);
            let right = register(right, Size::Qword, allocated);
            let rax = Data::Register(Register::new(RegisterKind::A, Size::Qword));
            output.extend(vec![
                Assembly::Mov(rax.clone(), left),
                Assembly::Cqo,
                Assembly::Idiv(right),
                Assembly::Mov(to, rax),
            ]);
        }
        Instruction::Equal { to, left, right } => {
            comparision!(Sete, to, left, right, allocated, output);
        }
        Instruction::Greater { to, left, right } => {
            comparision!(Setg, to, left, right, allocated, output);
        }
        Instruction::Less { to, left, right } => {
            comparision!(Setl, to, left, right, allocated, output);
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
        Instruction::Jump(to) => {
            output.push(Assembly::Jmp(to));
        }
        Instruction::JumpIf { condition, to } => {
            let condition = register(condition, Size::Qword, allocated);
            output.extend(vec![
                Assembly::Cmp(condition, Data::Integer(1)),
                Assembly::Je(to),
            ]);
        }
        Instruction::Call { label, value } => {
            let value = register(value, Size::Qword, allocated);
            let rdi = Data::Register(Register::new(RegisterKind::Di, Size::Qword));
            output.extend(vec![Assembly::Mov(rdi, value), Assembly::Call(label)]);
        }
        _ => todo!(),
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

pub fn convert(tac: Vec<Instruction>) -> Vec<Assembly> {
    let mut output = Vec::new();
    let n = RegisterKind::allocable().len();
    let allocated = map(ir::allocate(&tac, n as u64));
    for instruction in tac.into_iter() {
        generate(instruction, &mut output, &allocated);
    }
    output
}
