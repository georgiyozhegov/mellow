mod block;
mod cfg;
mod constructor;
pub use block::Block;
pub use cfg::{Cfg, Link};
pub use constructor::Constructor;
use syntax::parse::Statement;

pub fn construct(source: Vec<Statement>) -> Cfg<Block, Link> {
    let constructor = Constructor::new();
    constructor.construct(source)
}
