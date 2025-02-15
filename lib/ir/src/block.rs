use std::fmt::{self, Display, Formatter};
use syntax::tree::Statement;

#[derive(Debug)]
pub enum Block<T> {
    Basic(Vec<T>),
    Empty,
}

pub struct BlockRange {
    pub start: u64,
    pub end: u64,
}

impl BlockRange {
    pub fn new(start: u64, end: u64) -> Self {
        Self { start, end }
    }
}

impl<Instruction: Display> Display for Block<Instruction> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            Self::Empty => writeln!(f, ""),
            Self::Basic(block) => {
                for instruction in block.iter() {
                    writeln!(f, "{instruction}")?;
                }
                Ok(())
            }
        }
    }
}
