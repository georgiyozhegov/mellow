use std::iter::Peekable;

use mellow_error::{Error, Result};
use mellow_lex::{Lex, Token, TokenKind};

use super::Statement;

pub type Source<'s> = Peekable<Lex<'s>>;

pub struct Parse<'p> {
    source: Source<'p>,
}

impl<'p> Parse<'p> {
    pub fn new(source: Source<'p>) -> Self {
        Self { source }
    }
}

impl Iterator for Parse<'_> {
    type Item = Result<Statement>;

    fn next(&mut self) -> Option<Self::Item> {
        self.source.peek()?;
        Some(Statement::parse(self))
    }
}

impl Parse<'_> {
    pub fn next(&mut self) -> Result<Token> {
        self.source
            .next()
            .unwrap_or(Err(Error::expected_but_got("statement", "EOF")))
    }

    pub fn peek(&mut self) -> Result<Option<Token>> {
        self.source.peek().cloned().transpose()
    }

    pub fn expect(&mut self, token: TokenKind) -> Result<()> {
        let next = self.next()?.take_kind();
        if next == token {
            Ok(())
        } else {
            Err(Error::expected_but_got("todo", "todo"))
        }
    }

    pub fn mutable(&mut self) -> Result<bool> {
        match self.peek()?.and_then(|token| Some(token.take_kind())) {
            Some(TokenKind::Mutable) => {
                self.next()?;
                Ok(true)
            }
            _ => Ok(false),
        }
    }
}

pub trait Parsable {
    fn parse(source: &mut Parse) -> Result<Self>
    where
        Self: Sized;
}
