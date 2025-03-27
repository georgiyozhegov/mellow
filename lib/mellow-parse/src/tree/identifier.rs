use mellow_lex::Token;
use mellow_error::{Result, Error};

use crate::parser::Parse;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Identifier {
    pub name: String,
}

impl Parse for Identifier {
    fn parse(parser: &mut crate::Parser) -> Result<Self>
    where
        Self: Sized,
    {
        match parser.next()? {
            Token::Identifier(name) => Ok(Identifier { name }),
            token => Err(Error::expected_but_got("identifier", token)),
        }
    }
}
