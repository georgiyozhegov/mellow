use mellow_lex::Token;
use mellow_error::Result;

use crate::{Expression, Parse, Parser};

use super::Identifier;

#[derive(Debug, Clone)]
pub struct Let {
    pub identifier: Identifier,
    pub mutable: bool,
    pub value: Expression,
}

impl Parse for Let {
    fn parse(parser: &mut Parser) -> Result<Self> {
        parser.expect(Token::Let)?;
        let mutable = parser.mutable()?;
        let identifier = Identifier::parse(parser)?;
        parser.expect(Token::Equal)?;
        let value = Expression::parse(parser)?;
        Ok(Self {
            identifier,
            mutable,
            value,
        })
    }
}
