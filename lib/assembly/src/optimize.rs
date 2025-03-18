use std::{iter::Peekable, slice::Iter};

use crate::assembly::Assembly;

impl Assembly {
    pub fn optimize(self) -> Self {
        match &self {
            Self::Mov(to, from) => {
                if to == from {
                    return Self::Empty;
                }
            }
            _ => {}
        }
        self
    }

    pub fn optimize_with(self, source: &mut Peekable<Iter<Self>>) -> Self {
        match &self {
            Self::Mov(to, from) => {
                if let Some(Self::Mov(next_to, next_from)) = source.peek() {
                    if to == next_from {
                        source.next();
                        return Self::Mov(next_to.clone(), from.clone());
                    }
                }
            }
            Self::Jmp(label) => {
                if let Some(Self::Label(id)) = source.peek() {
                    if label == id {
                        return Self::Empty;
                    }
                }
            }
            _ => {}
        }
        self
    }
}

pub fn optimize(assembly: Vec<Assembly>) -> Vec<Assembly> {
    let mut output = Vec::new();
    let mut assembly = assembly.iter().peekable();
    while let Some(instruction) = assembly.next() {
        let instruction = instruction.clone().optimize_with(&mut assembly).optimize();
        if instruction != Assembly::Empty {
            output.push(instruction);
        }
    }
    output
}
