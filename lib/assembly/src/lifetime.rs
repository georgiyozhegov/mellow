use std::collections::{HashMap, HashSet};

use ir::tac::Instruction;

use crate::register::RegisterKind;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Lifetime {
    pub start: usize,
    pub end: usize,
}

impl Lifetime {
    pub fn overlaps(&self, other: &Self) -> bool {
        self.start <= other.start && other.end <= self.end
    }
}

macro_rules! begin {
    ($start:expr, $id:expr, $index:expr) => {
        $start.insert($id, $index);
    };
}

macro_rules! end {
    ($lifetimes:expr, $start:expr, $id:expr, $index:expr) => {
        let lifetime = Lifetime {
            start: *$start.get(&$id).unwrap(),
            end: $index,
        };
        $lifetimes.insert(*$id, lifetime);
    };
}

fn scan(tac: &Vec<Instruction>) -> HashMap<u64, Lifetime> {
    let mut start = HashMap::new();
    let mut lifetimes = HashMap::new();
    for (index, instruction) in tac.iter().enumerate() {
        match instruction {
            Instruction::Add { to, left, right }
            | Instruction::Subtract { to, left, right }
            | Instruction::Multiply { to, left, right }
            | Instruction::Divide { to, left, right }
            | Instruction::Greater { to, left, right }
            | Instruction::Less { to, left, right }
            | Instruction::Equal { to, left, right } => {
                begin!(start, to, index);
                end!(lifetimes, start, left, index);
                end!(lifetimes, start, right, index);
            }
            Instruction::JumpIf { condition, .. } => {
                end!(lifetimes, start, condition, index);
            }
            Instruction::Integer { to, .. }
            | Instruction::Get { to, .. }
            | Instruction::String { to, .. } => {
                begin!(start, to, index);
            }
            Instruction::Set { from, .. } => {
                end!(lifetimes, start, from, index);
            }
            Instruction::Call { value, .. } => {
                end!(lifetimes, start, value, index);
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
            if a.1.overlaps(b.1) {
                graph.get_mut(a.0).unwrap().insert(*b.0);
                graph.get_mut(b.0).unwrap().insert(*a.0);
            }
        }
    }

    graph
}

pub fn allocate(tac: &Vec<Instruction>) -> HashMap<u64, RegisterKind> {
    let lifetimes = scan(tac);
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
            panic!("error: all registers have been allocated") // TODO: memory spilling
        }
    }

    allocated
}
