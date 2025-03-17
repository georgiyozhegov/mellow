use std::collections::HashMap;

use syntax::parse::expression::Expression;

use super::Block;

#[derive(Debug)]
pub enum Link {
    Direct(u64),
    Branch {
        condition: Expression,
        true_: u64,
        false_: u64,
    },
}

#[derive(Debug)]
pub struct Cfg<B, L> {
    pub blocks: Vec<B>,
    pub links: HashMap<u64, L>,
}

impl<B, L> Cfg<B, L> {
    pub fn new() -> Self {
        Self {
            blocks: Vec::new(),
            links: HashMap::new(),
        }
    }
}

impl<B, L> Default for Cfg<B, L> {
    fn default() -> Self {
        Self::new()
    }
}

impl Cfg<Block, Link> {
    pub fn insert(&mut self, block: Block) -> u64 {
        let id = self.blocks.len() as u64;
        self.blocks.push(block);
        id
    }

    pub fn link(&mut self, from: u64, to: Link) {
        self.links.insert(from, to);
    }

    pub fn direct(&mut self, from: u64, to: u64) {
        self.links.insert(from, Link::Direct(to));
    }

    pub fn branch(&mut self, from: u64, condition: Expression, true_: u64, false_: u64) {
        self.links.insert(
            from,
            Link::Branch {
                condition,
                true_,
                false_,
            },
        );
    }

    pub fn last_id(&self) -> u64 {
        self.blocks.len() as u64 - 1
    }

    pub fn next_id(&self) -> u64 {
        self.blocks.len() as u64
    }
}
