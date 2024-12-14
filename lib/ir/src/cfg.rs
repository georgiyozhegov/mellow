use std::collections::HashMap;

use syntax::tree::{Expression, Statement};

use crate::block::Block;

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
pub struct Cfg {
    blocks: HashMap<u64, Block>,
    links: HashMap<u64, Link>,
    id: u64,
}

impl Cfg {
    pub fn new() -> Self {
        Self {
            blocks: HashMap::new(),
            links: HashMap::new(),
            id: 0,
        }
    }
}

impl Cfg {
    pub fn insert(&mut self, block: Block) -> u64 {
        let id = self.id;
        self.id += 1;
        self.blocks.insert(id, block);
        id
    }

    pub fn link(&mut self, from: u64, to: Link) {
        self.links.insert(from, to);
    }
}

fn construct_(source: Vec<Statement>, cfg: &mut Cfg) -> (u64, u64) {
    let start = cfg.id;
    let mut current = Vec::new();
    for statement in source.iter() {
        match statement {
            Statement::Let { .. } | Statement::Change { .. } => {
                current.push(statement.clone());
            }
            Statement::If {
                condition,
                true_,
                false_,
            } => {
                let previous = cfg.insert(Block::Basic(current.clone()));
                current.clear();
                let (true_start, true_end) = construct_(true_.clone(), cfg);
                let (false_start, false_end) = construct_(false_.clone(), cfg);
                cfg.link(
                    previous,
                    Link::Branch {
                        condition: condition.clone(),
                        true_: true_start,
                        false_: false_start,
                    },
                );
                let end = cfg.insert(Block::Empty);
                cfg.link(true_end, Link::Direct(end));
                cfg.link(false_end, Link::Direct(end));
            }
            Statement::While { condition, body } => {
                let previous = cfg.insert(Block::Basic(current.clone()));
                current.clear();
                let (body_start, body_end) = construct_(body.clone(), cfg);
                cfg.link(previous, Link::Direct(body_start));
                let end = cfg.insert(Block::Empty);
                cfg.link(
                    body_end,
                    Link::Branch {
                        condition: condition.clone(),
                        true_: body_start,
                        false_: end,
                    },
                );
            }
            _ => todo!(),
        }
    }
    if !current.is_empty() {
        cfg.insert(Block::Basic(current));
    }
    let end = cfg.id - 1;
    (start, end)
}

pub fn construct(source: Vec<Statement>) -> Cfg {
    let mut cfg = Cfg::new();
    construct_(source, &mut cfg);
    cfg
}
