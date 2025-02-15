mod instruction;
use std::collections::HashMap;

pub use instruction::Instruction;
use ir::{cfg::{Cfg, Link}, Block};
use syntax::tree::{Statement, Expression};

#[derive(Debug)]
pub struct Tac {
    pub blocks: Vec<Vec<Instruction>>,
}

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

pub fn construct(cfg: Cfg<Block, Link>) -> Tac {
    let mut allocator = Allocator { id: 0 };
    let blocks = cfg
        .blocks
        .into_iter()
        .enumerate()
        .map(|(id, block)| {
            let mut block = match block {
                Block::Basic(body) => {
                    let mut instructions = Vec::new();
                    for statement in body {
                        Instruction::statement(statement, &mut allocator, &mut instructions);
                    }
                    instructions
                }
                Block::Empty => vec![],
            };
            match cfg.links.get(&(id as u64)) {
                Some(Link::Direct(to)) =>  {
                    block.push(Instruction::Jump { to: *to });
                }
                Some(Link::Branch { condition, true_, false_ }) => {
                    let condition = Instruction::expression(condition.clone(), &mut allocator, &mut block);
                    block.push(Instruction::JumpIf { condition, to: *true_ });
                    block.push(Instruction::Jump { to: *false_ });
                }
                _ => {}
            }
            block
        })
        .collect::<Vec<Vec<Instruction>>>();
    Tac { blocks }
}