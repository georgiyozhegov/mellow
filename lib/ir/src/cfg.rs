use std::collections::HashMap;
use std::fmt::{self, Display, Formatter};

use syntax::tree::{Expression, Statement};

use crate::block::{Block, BlockRange};

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
pub struct Cfg<T> {
    pub blocks: Vec<Block<T>>,
    pub links: HashMap<u64, Link>,
}

impl<T> Cfg<T> {
    pub fn new() -> Self {
        Self {
            blocks: Vec::new(),
            links: HashMap::new(),
        }
    }
}

impl<T> Cfg<T> {
    pub fn insert(&mut self, block: Block<T>) -> u64 {
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

fn construct_<T>(source: Vec<Statement>, cfg: &mut Cfg<Statement>) -> BlockRange {
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
    BlockRange::new(start, end)
}

fn if_(
    condition: Expression,
    if_: Vec<Statement>,
    mut or: Vec<(Expression, Vec<Statement>)>,
    else_: Vec<Statement>,
    current: &mut Vec<Statement>,
    cfg: &mut Cfg<Statement>,
) {
    let mut previous = cfg.insert(Block::Basic(current.clone()));
    current.clear();
    or.insert(0, (condition, if_));
    for (condition, body) in or {
        let body = construct_::<Statement>(body.clone(), cfg);
        let next = cfg.next_id();
        cfg.branch(previous, condition, body.start, next);
        previous = body.end;
    }
    construct_::<Statement>(else_, cfg);
}

fn while_(
    condition: Expression,
    body: Vec<Statement>,
    current: &mut Vec<Statement>,
    cfg: &mut Cfg<Statement>,
) {
    let previous = cfg.insert(Block::Basic(current.clone()));
    current.clear();
    let start = cfg.insert(Block::Empty);
    cfg.direct(previous, start);
    let body = construct_::<Statement>(body.clone(), cfg);
    let end = cfg.next_id();
    cfg.branch(start, condition, body.start, end);
    cfg.direct(body.end, start);
}

pub fn construct(source: Vec<Statement>) -> Cfg<Statement> {
    let mut cfg = Cfg::new();
    construct_::<Statement>(source, &mut cfg);
    cfg
}

impl<Instruction: Display> Display for Cfg<Instruction> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        for (id, block) in self.blocks.iter().enumerate() {
            writeln!(f, "@{id}")?;
            writeln!(f, "{block}")?;
            if let Some(link) = self.links.get(&(id as u64)) {
                writeln!(f, "{link}")?;
            }
        }
        Ok(())
    }
}

impl Display for Link {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            Self::Direct(id) => {
                write!(f, "jump @{id}")
            }
            Self::Branch {
                condition,
                true_,
                false_,
            } => {
                write!(f, "    ")?;
                writeln!(f, "on {condition:?}")?;
                write!(f, "    ")?;
                writeln!(f, "true @{true_}")?;
                write!(f, "    ")?;
                writeln!(f, "false @{false_}")?;
                Ok(())
            }
        }
    }
}