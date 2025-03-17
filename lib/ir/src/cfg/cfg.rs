use std::{collections::HashMap, vec::IntoIter};

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

pub struct CfgIter {
    id: u64,
    blocks: IntoIter<Block>,
    links: HashMap<u64, Link>,
}

impl Iterator for CfgIter {
    type Item = (Block, Option<Link>);

    fn next(&mut self) -> Option<Self::Item> {
        let block = self.blocks.next()?;
        let link = self.links.remove(&self.id);
        self.id += 1;
        Some((block, link))
    }
}

impl IntoIterator for Cfg<Block, Link> {
    type Item = (Block, Option<Link>);
    type IntoIter = CfgIter;

    fn into_iter(self) -> Self::IntoIter {
        CfgIter {
            id: 0,
            blocks: self.blocks.into_iter(),
            links: self.links,
        }
    }
}
