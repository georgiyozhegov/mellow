mod constructor;
mod instruction;
use constructor::Constructor;
pub use instruction::Instruction;

use crate::cfg::Block;

pub fn construct(source: Vec<Block>) -> Vec<Instruction> {
    let constructor = Constructor::new();
    constructor.construct(source)
}
