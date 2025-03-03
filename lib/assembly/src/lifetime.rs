use std::{
    collections::{HashMap, HashSet},
    fmt::{self, Display},
};

use crate::{Instruction, Tac};

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum RegisterKind {
    A,
    B,
    C,
    D,
    Sp,
    Bp,
    Si,
    Di,
    R8,
    R9,
    R10,
    R11,
    R12,
    R13,
    R14,
    R15,
}

impl RegisterKind {
    pub fn all() -> Vec<Self> {
        use RegisterKind::*;
        vec![
            A, B, C, D, Sp, Bp, Si, Di, R8, R9, R10, R11, R12, R13, R14, R15,
        ]
    }

    pub fn allocable() -> Vec<Self> {
        use RegisterKind::*;
        vec![B, C, Si, Di, R8, R9, R10, R11, R12, R13, R14, R15]
    }
}

#[derive(Debug, Clone)]
pub enum Size {
    Byte = 8,
    Word = 16,
    Dword = 32,
    Qword = 64,
}

#[derive(Debug, Clone)]
pub struct Register {
    kind: RegisterKind,
    size: Size,
}

impl Register {
    pub fn new(kind: RegisterKind, size: Size) -> Self {
        Self { kind, size }
    }
}

impl Display for Register {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let kind = match self.kind {
            RegisterKind::A => "a",
            RegisterKind::B => "b",
            RegisterKind::C => "c",
            RegisterKind::D => "d",
            RegisterKind::Sp => "sp",
            RegisterKind::Bp => "bp",
            RegisterKind::Si => "si",
            RegisterKind::Di => "di",
            RegisterKind::R8 => "r8",
            RegisterKind::R9 => "r9",
            RegisterKind::R10 => "r10",
            RegisterKind::R11 => "r11",
            RegisterKind::R12 => "r12",
            RegisterKind::R13 => "r13",
            RegisterKind::R14 => "r14",
            RegisterKind::R15 => "r15",
        };
        let (size_prefix, size_suffix) = match self.kind {
            RegisterKind::A | RegisterKind::B | RegisterKind::C | RegisterKind::D => {
                match self.size {
                    Size::Byte => ("", "l"),
                    Size::Word => ("", "x"),
                    Size::Dword => ("e", "x"),
                    Size::Qword => ("r", "x"),
                }
            }
            RegisterKind::Sp | RegisterKind::Bp | RegisterKind::Si | RegisterKind::Di => {
                match self.size {
                    Size::Byte => ("", "l"),
                    Size::Word => ("", ""),
                    Size::Dword => ("e", ""),
                    Size::Qword => ("r", ""),
                }
            }

            RegisterKind::R8
            | RegisterKind::R9
            | RegisterKind::R10
            | RegisterKind::R11
            | RegisterKind::R12
            | RegisterKind::R13
            | RegisterKind::R14
            | RegisterKind::R15 => match self.size {
                Size::Byte => ("", "b"),
                Size::Word => ("", "w"),
                Size::Dword => ("", "d"),
                Size::Qword => ("", ""),
            },
        };
        write!(f, "{size_prefix}{kind}{size_suffix}")
    }
}

#[derive(Debug)]
pub enum Data {
    Register(Register),
    Stack(u64),
    Memory(String),
    Integer(i128),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Lifetime {
    pub start: usize,
    pub end: usize,
}

impl Lifetime {
    pub fn is_overlap(&self, other: &Self) -> bool {
        self.start <= other.start && other.end <= self.end
    }

    pub fn join(mut self, other: Self) -> Self {
        self.start = self.start.min(other.end).min(other.start);
        self.end = self.end.max(other.end).max(other.start);
        self
    }
}

pub fn scan(tac: &Tac) -> HashMap<u64, Lifetime> {
    let mut start = HashMap::new();
    let mut lifetimes = HashMap::new();
    for (index, instruction) in tac.blocks.iter().flatten().enumerate() {
        match instruction {
            Instruction::Add { to, left, right }
            | Instruction::Subtract { to, left, right }
            | Instruction::Multiply { to, left, right }
            | Instruction::Divide { to, left, right }
            | Instruction::Greater { to, left, right }
            | Instruction::Less { to, left, right }
            | Instruction::Equal { to, left, right } => {
                start.insert(to, index);
                // left
                let lifetime = Lifetime {
                    start: *start.get(left).unwrap(),
                    end: index,
                };
                lifetimes.insert(*left, lifetime);
                // right
                let lifetime = Lifetime {
                    start: *start.get(right).unwrap(),
                    end: index,
                };
                lifetimes.insert(*right, lifetime);
            }
            Instruction::JumpIf { condition, .. } => {
                let lifetime = Lifetime {
                    start: *start.get(condition).unwrap(),
                    end: index,
                };
                lifetimes.insert(*condition, lifetime);
            }
            Instruction::Integer { to, .. }
            | Instruction::Get { to, .. }
            | Instruction::String { to, .. } => {
                start.insert(to, index);
            }
            Instruction::Set { from, .. } => {
                let lifetime = Lifetime {
                    start: *start.get(from).unwrap(),
                    end: index,
                };
                lifetimes.insert(*from, lifetime);
            }
            _ => {}
        }
    }
    lifetimes
}

fn interference_graph(lifetimes: HashMap<u64, Lifetime>) -> HashMap<u64, HashSet<u64>> {
    let mut graph = HashMap::new();

    for id in lifetimes.keys() {
        graph.insert(*id, HashSet::new());
    }

    for a in lifetimes.iter() {
        for b in lifetimes.iter() {
            if a.1.is_overlap(b.1) {
                graph.get_mut(a.0).unwrap().insert(*b.0);
                graph.get_mut(b.0).unwrap().insert(*a.0);
            }
        }
    }

    graph
}

pub fn allocate(lifetimes: HashMap<u64, Lifetime>) -> HashMap<u64, RegisterKind> {
    let graph = interference_graph(lifetimes);
    let registers = RegisterKind::allocable();
    let mut allocated: HashMap<u64, RegisterKind> = HashMap::new();

    let mut ids = graph.keys().cloned().collect::<Vec<u64>>();
    ids.sort_by_key(|id| -(graph.get(id).unwrap().len() as isize));
    for id in ids.iter() {
        let mut used = HashSet::new();
        for overlap in graph.get(&id).unwrap().iter() {
            if let Some(register) = allocated.get(overlap) {
                used.insert(register.clone());
            }
        }

        if let Some(register) = registers.iter().find(|register| !used.contains(register)) {
            allocated.insert(*id, register.clone());
        } else {
            panic!("error: all registers have been allocated")
        }
    }

    allocated
}
