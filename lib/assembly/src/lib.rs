mod instruction;
mod native;
pub use native::convert;

use std::fmt::{self, Display, Formatter};

pub use instruction::Instruction;
use ir::{
    cfg::{Cfg, Link},
    Block,
};

#[derive(Debug)]
pub struct Tac {
    pub blocks: Vec<Vec<Instruction>>,
}

impl Display for Tac {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        for (id, block) in self.blocks.iter().enumerate() {
            writeln!(f, "@{id}")?;
            for instruction in block.iter() {
                writeln!(f, " {instruction}")?;
            }
        }
        Ok(())
    }
}

pub struct Allocator {
    pub id: u64,
}

impl Allocator {
    pub fn new() -> Self {
        Self { id: 0 }
    }
}

impl Default for Allocator {
    fn default() -> Self {
        Self::new()
    }
}

impl Allocator {
    pub fn allocate(&mut self) -> u64 {
        let id = self.id;
        self.id += 1;
        id
    }
}

fn block(block: Block, allocator: &mut Allocator, output: &mut Vec<Instruction>) {
    match block {
        Block::Basic(body) => {
            for statement in body {
                Instruction::statement(statement, allocator, output);
            }
        }
        Block::Empty => {}
    };
}

fn link(link: &Link, allocator: &mut Allocator, output: &mut Vec<Instruction>) {
    match link {
        Link::Direct(to) => {
            output.push(Instruction::Jump { to: *to });
        }
        Link::Branch {
            condition,
            true_,
            false_,
        } => {
            let condition = Instruction::expression(condition.clone(), allocator, output);
            output.push(Instruction::JumpIf {
                condition,
                to: *true_,
            });
            output.push(Instruction::Jump { to: *false_ });
        }
    }
}

pub fn construct(cfg: Cfg<Block, Link>) -> Tac {
    let mut allocator = Allocator::new();
    let blocks = cfg
        .blocks
        .into_iter()
        .enumerate()
        .map(|(id, block)| {
            let mut output = Vec::new();
            self::block(block, &mut allocator, &mut output);
            if let Some(link) = cfg.links.get(&(id as u64)) {
                self::link(link, &mut allocator, &mut output);
            }
            output
        })
        .collect();
    Tac { blocks }
}
