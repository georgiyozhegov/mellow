use crate::{
    cfg::{Cfg, Link},
    instruction::Instruction,
    Block,
};

pub struct TemporaryAllocator {
    pub id: u64,
}

impl TemporaryAllocator {
    pub fn new() -> Self {
        Self { id: 0 }
    }
}

impl TemporaryAllocator {
    pub fn allocate(&mut self) -> u64 {
        let id = self.id;
        self.id += 1;
        id
    }
}

fn block(block: Block, allocator: &mut TemporaryAllocator, output: &mut Vec<Instruction>) {
    match block {
        Block::Basic(body) => {
            for statement in body {
                Instruction::statement(statement, allocator, output);
            }
        }
        Block::Empty => {}
    };
}

fn link(link: &Link, allocator: &mut TemporaryAllocator, output: &mut Vec<Instruction>) {
    match link {
        Link::Direct(to) => {
            output.push(Instruction::Jump(*to));
        }
        Link::Branch {
            condition,
            true_,
            false_,
        } => {
            let condition = Instruction::expression(condition.clone(), allocator, output);
            output.extend(vec![
                Instruction::JumpIf {
                    condition,
                    to: *true_,
                },
                Instruction::Jump(*false_),
            ]);
        }
    }
}

pub fn construct(cfg: Cfg<Block, Link>) -> Vec<Instruction> {
    let mut allocator = TemporaryAllocator::new();
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
