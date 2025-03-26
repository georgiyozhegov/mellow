use std::iter::Peekable;

use mellow_lex::{Error, Lexer, Result, Token};

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
            .unwrap_or(Err(Error::grammar("statement", None)))
    }

    pub fn peek(&mut self) -> Result<Option<Token>> {
        self.source.peek().cloned().transpose()
    }

    pub fn expect(&mut self, token: Token) -> Result<()> {
        let next = self.next()?;
        if next == token {
            Ok(())
        } else {
            Err(Error::grammar(token.to_string(), Some(next)))
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

    pub fn identifier(&mut self) -> Result<String> {
        match self.next()? {
            Token::Identifier(identifier) => Ok(identifier),
            token => Err(Error::grammar("identifier", Some(token))),
        }
    }
}

pub trait Parse {
    fn parse(parser: &mut Parser) -> Result<Self>
    where
        Self: Sized;
}
