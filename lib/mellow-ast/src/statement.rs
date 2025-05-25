use std::vec::IntoIter;

use mellow_error::{Error, Result};
use mellow_lex::TokenKind;

use crate::Expression;

#[derive(Debug, Clone)]
pub enum Statement {
    Let(Let),
    Assign(Assign),
    If(If<Body>),
    While(While),
    Debug(Debug),
}

#[derive(Debug, Clone)]
pub struct Let {
    pub identifier: Identifier,
    pub mutable: bool,
    pub value: Expression,
}

impl Let {
    pub fn new(identifier: Identifier, mutable: bool, value: Expression) -> Self {
        Self {
            identifier,
            mutable,
            value,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Identifier {
    pub name: String,
}

impl Identifier {
    pub fn new(name: String) -> Self {
        Self { name }
    }
}

#[derive(Debug, Clone)]
pub struct Assign {
    pub identifier: Identifier,
    pub value: Expression,
}

impl Assign {
    pub fn new(identifier: Identifier, value: Expression) -> Self {
        Self { identifier, value }
    }
}

#[derive(Debug, Clone)]
pub struct If<T> {
    pub if_: Branch<T>,
    pub or: Vec<Branch<T>>,
    pub else_: Option<Box<T>>,
}


#[derive(Debug, Clone)]
pub struct Branch<B> {
    pub condition: Box<Expression>,
    pub body: Box<B>,
}


#[derive(Debug, Clone)]
pub struct While {
    pub condition: Expression,
    pub body: Body,
}

impl While {
    pub fn new(condition: Expression, body: Body) -> Self {
        Self { condition, body }
    }
}

#[derive(Debug, Clone)]
pub struct Body {
    inner: Vec<Statement>,
}

impl Body {
    pub fn new(inner: Vec<Statement>) -> Self {
        Self { inner }
    }
}

impl Body {
    pub fn empty() -> Self {
        Self { inner: Vec::new() }
    }
}

impl Default for Body {
    fn default() -> Self {
        Self::empty()
    }
}

impl IntoIterator for Body {
    type Item = Statement;
    type IntoIter = IntoIter<Statement>;

    fn into_iter(self) -> Self::IntoIter {
        self.inner.into_iter()
    }
}

#[derive(Debug, Clone)]
pub struct Debug {
    pub value: Expression,
}

impl Debug {
    pub fn new(value: Expression) -> Self {
        Self { value }
    }
}
