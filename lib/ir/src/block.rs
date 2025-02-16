use std::fmt::{self, Display, Formatter};

use syntax::tree::Statement;

#[derive(Debug)]
pub enum Block {
    Basic(Vec<Statement>),
    Empty,
}

pub struct BlockRange {
    pub start: u64,
    pub end: u64,
}

impl BlockRange {
    pub fn new(start: u64, end: u64) -> Self {
        Self { start, end }
    }
}
