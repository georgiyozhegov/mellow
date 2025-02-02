mod instruction;
use std::collections::HashMap;

pub use instruction::Instruction;
use ir::{cfg::Cfg, Block};
use syntax::tree::Statement;

#[derive(Debug)]
pub struct Assembly {
    cfg: Cfg<Instruction>,
    variables: HashMap<String, u64>,
}

pub struct Allocator {
    pub id: u64,
    pub variables: HashMap<String, u64>,
}

pub fn construct(cfg: Cfg<Statement>) -> Assembly {
    let mut allocator = Allocator {
        id: 0,
        variables: HashMap::new(),
    };
    let blocks = cfg
        .blocks
        .into_iter()
        .map(|block| match block {
            Block::Basic(body) => {
                let mut instructions = Vec::new();
                for statement in body {
                    Instruction::statement(statement, &mut allocator, &mut instructions);
                }
                Block::Basic(instructions)
            }
            Block::Empty => Block::Empty,
        })
        .collect();
    let cfg = Cfg {
        blocks,
        links: cfg.links,
    };
    Assembly {
        cfg,
        variables: allocator.variables,
    }
}
