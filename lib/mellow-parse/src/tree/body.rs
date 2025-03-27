use std::vec::IntoIter;

use mellow_lex::Token;
use mellow_error::Result;

use crate::{Parse, Parser, Statement};

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

impl Parse for Body {
    fn parse(parser: &mut Parser) -> Result<Self> {
        let mut inner = Vec::new();
        while let Some(token) = parser.peek()? {
            match token {
                Token::Or | Token::Else | Token::End => break,
                _ => inner.push(Statement::parse(parser)?),
            }
        }
        Ok(Self { inner })
    }
}

impl IntoIterator for Body {
    type Item = Statement;
    type IntoIter = IntoIter<Statement>;

    fn into_iter(self) -> Self::IntoIter {
        self.inner.into_iter()
    }
}
