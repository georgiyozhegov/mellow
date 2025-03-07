mod block;
pub use block::Block;
pub mod cfg;
mod instruction;
mod lifetime;
pub mod tac;
pub use instruction::Instruction;
pub use lifetime::allocate;
