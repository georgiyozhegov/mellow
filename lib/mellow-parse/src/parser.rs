use std::iter::Peekable;

use mellow_lex::{Lexer, Token};
use mellow_error::{Result, Error};

use super::Statement;

pub type Source<'s> = Peekable<Lexer<'s>>;

pub struct Parser<'p> {
    source: Source<'p>,
}

impl<'p> Parser<'p> {
    pub fn new(source: Source<'p>) -> Self {
        Self { source }
    }
}

impl Iterator for Parser<'_> {
    type Item = Result<Statement>;

    fn next(&mut self) -> Option<Self::Item> {
        self.source.peek()?;
        Some(Statement::parse(self))
    }
}

impl Parser<'_> {
    pub fn next(&mut self) -> Result<Token> {
        self.source
            .next()
            .unwrap_or(Err(Error::expected_but_got("statement", "EOF")))
    }

    pub fn peek(&mut self) -> Result<Option<Token>> {
        self.source.peek().cloned().transpose()
    }

    pub fn expect(&mut self, token: Token) -> Result<()> {
        let next = self.next()?;
        if next == token {
            Ok(())
        } else {
            Err(Error::expected_but_got(token, next))
        }
    }

    pub fn mutable(&mut self) -> Result<bool> {
        match self.peek()? {
            Some(Token::Mutable) => {
                self.next()?;
                Ok(true)
            }
            _ => Ok(false),
        }
    }
}

pub trait Parse {
    fn parse(parser: &mut Parser) -> Result<Self>
    where
        Self: Sized;
}
