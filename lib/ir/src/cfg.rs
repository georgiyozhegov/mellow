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
    blocks: Vec<Block>,
    links: HashMap<u64, Link>,
}

impl Cfg {
    pub fn new() -> Self {
        Self {
            blocks: Vec::new(),
            links: HashMap::new(),
        }
    }
}

impl Cfg {
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

fn construct_(source: Vec<Statement>, cfg: &mut Cfg) -> (u64, u64) {
    let start = cfg.next_id();
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
            } => self::if_(condition, if_, or, else_, &mut current, cfg),
            Statement::While { condition, body } => while_(condition, body, &mut current, cfg),
            _ => todo!(),
        }
    }
    if !current.is_empty() {
        cfg.insert(Block::Basic(current));
    }
    let end = cfg.last_id();
    (start, end)
}

fn if_(
    condition: Expression,
    if_: Vec<Statement>,
    mut or: Vec<(Expression, Vec<Statement>)>,
    else_: Vec<Statement>,
    current: &mut Vec<Statement>,
    cfg: &mut Cfg,
) {
    let mut previous = cfg.insert(Block::Basic(current.clone()));
    current.clear();
    or.insert(0, (condition, if_));
    for (condition, body) in or {
        let (start, end) = construct_(body.clone(), cfg);
        let next = cfg.next_id();
        cfg.branch(previous, condition, start, next);
        previous = end;
    }
    construct_(else_, cfg);
}

fn while_(
    condition: Expression,
    body: Vec<Statement>,
    current: &mut Vec<Statement>,
    cfg: &mut Cfg,
) {
    let previous = cfg.insert(Block::Basic(current.clone()));
    current.clear();
    let start = cfg.insert(Block::Empty);
    cfg.direct(previous, start);
    let (body_start, body_end) = construct_(body.clone(), cfg);
    let end = cfg.next_id();
    cfg.branch(start, condition, body_start, end);
    cfg.direct(body_end, start);
}

pub fn construct(source: Vec<Statement>) -> Cfg {
    let mut cfg = Cfg::new();
    construct_(source, &mut cfg);
    cfg
}
