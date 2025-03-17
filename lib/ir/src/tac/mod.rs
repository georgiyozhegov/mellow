mod constructor;
mod instruction;
use constructor::Constructor;
pub use instruction::Instruction;

use crate::cfg::{Block, Cfg, Link};

pub fn construct(source: Cfg<Block, Link>) -> Vec<Instruction> {
    let constructor = Constructor::new();
    constructor.construct(source)
}
