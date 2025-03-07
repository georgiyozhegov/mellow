use crate::{
    cfg::{Cfg, Link},
    instruction::Instruction,
    Block,
};

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

pub fn construct(cfg: Cfg<Block, Link>) -> Vec<Instruction> {
    let mut allocator = Allocator::new();
    cfg.blocks
        .into_iter()
        .enumerate()
        .map(|(id, block)| {
            let mut output = vec![Instruction::Label(id as u64)];
            self::block(block, &mut allocator, &mut output);
            if let Some(link) = cfg.links.get(&(id as u64)) {
                self::link(link, &mut allocator, &mut output);
            }
            output
        })
        .flatten()
        .collect()
}
