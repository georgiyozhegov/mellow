use std::vec::IntoIter;

use syntax::parse::{statement::Statement, Expression};

#[derive(Debug, Clone)]
pub enum Link {
    Direct(usize),
    Branch {
        condition: Expression,
        true_: usize,
        false_: usize,
    },
}

#[derive(Debug, Clone)]
pub struct Block {
    body: Vec<Statement>,
    next: Option<Link>,
}

impl Block {
    pub fn new(body: Vec<Statement>) -> Self {
        Self { body, next: None }
    }

    pub fn empty() -> Self {
        Self::new(Vec::new())
    }
}

impl Block {
    pub fn to(&mut self, link: Link) {
        self.next = Some(link);
    }

    pub fn direct(&mut self, to: usize) {
        self.next = Some(Link::Direct(to));
    }

    pub fn branch(&mut self, condition: Expression, true_: usize, false_: usize) {
        self.next = Some(Link::Branch {
            condition,
            true_,
            false_,
        })
    }

    pub fn next(&self) -> Option<&Link> {
        self.next.as_ref()
    }
}

impl IntoIterator for Block {
    type Item = Statement;
    type IntoIter = IntoIter<Statement>;

    fn into_iter(self) -> Self::IntoIter {
        self.body.into_iter()
    }
}

pub struct BlockRange {
    pub start: usize,
    pub end: usize,
}

impl BlockRange {
    pub fn new(start: usize, end: usize) -> Self {
        Self { start, end }
    }
}
