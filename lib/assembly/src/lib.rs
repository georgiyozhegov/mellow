mod instruction;
use std::collections::HashMap;

pub use instruction::Instruction;
use ir::{cfg::Cfg, Block};
use syntax::tree::Statement;

pub struct Allocator {
    pub id: u64,
}

impl Allocator {
    pub fn allocate(&mut self) -> u64 {
        let id = self.id;
        self.id += 1;
        id
    }
}

pub fn construct(cfg: Cfg<Statement>) -> Cfg<Instruction> {
    let mut allocator = Allocator { id: 0 };
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
    Cfg {
        blocks,
        links: cfg.links,
    }
}
