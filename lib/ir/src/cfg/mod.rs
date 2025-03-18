mod block;
mod cfg;
mod constructor;
pub use block::Block;
pub use cfg::{Cfg, Link};
pub use constructor::Constructor;
use syntax::parse::statement::Statement;

pub fn construct(source: Vec<Statement>) -> Cfg {
    let constructor = Constructor::new();
    constructor.construct(source)
}
