mod lifetime;
mod instruction;
mod constructor;
pub use lifetime::allocate;
pub use instruction::Instruction;
use constructor::Constructor;

use crate::cfg::{Block, Cfg, Link};

pub fn construct(source: Cfg<Block, Link>) -> Vec<Instruction> {
    let constructor = Constructor::new();
    constructor.construct(source)
}
