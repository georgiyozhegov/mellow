mod constructor;
mod instruction;
use constructor::Constructor;
pub use instruction::Instruction;

use crate::cfg::Cfg;

pub fn construct(source: Cfg) -> Vec<Instruction> {
    let constructor = Constructor::new();
    constructor.construct(source)
}
