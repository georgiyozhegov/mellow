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
    for statement in source {
        match statement {
            Statement::Let { .. } | Statement::Assign { .. } => {
                current.push(statement.clone());
            }
            Statement::If {
                condition,
                if_,
                or,
                else_,
            } => {
                self::if_(condition, if_, or, else_, &mut current, cfg);
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

fn if_(condition: Expression, if_: Vec<Statement>, mut or: Vec<(Expression, Vec<Statement>)>, else_: Vec<Statement>, current: &mut Vec<Statement>, cfg: &mut Cfg) {
    let mut previous = cfg.insert(Block::Basic(current.clone()));

    or.insert(0, (condition, if_));
    for (condition, body) in or {
        let (start, end_) = construct_(body.clone(), cfg);
        cfg.link(previous, Link::Branch { condition, true_: start, false_: cfg.id });
        previous = end_;
    }

    construct_(else_, cfg);
}

pub fn construct(source: Vec<Statement>) -> Cfg {
    let mut cfg = Cfg::new();
    construct_(source, &mut cfg);
    cfg
}
