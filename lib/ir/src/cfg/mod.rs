mod block;
mod constructor;
pub use block::{Block, Link};
pub use constructor::Constructor;
use syntax::parse::statement::Statement;

pub fn construct(source: Vec<Statement>) -> Vec<Block> {
    let constructor = Constructor::new();
    constructor.construct(source)
}
