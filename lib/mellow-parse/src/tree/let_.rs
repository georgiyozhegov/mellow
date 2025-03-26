use mellow_lex::{Result, Token};

use crate::{Parse, Expression, Parser};

#[derive(Debug, Clone)]
pub struct Let {
    pub identifier: String,
    pub mutable: bool,
    pub value: Expression,
}

impl Parse for Let {
    fn parse(parser: &mut Parser) -> Result<Self> {
        parser.expect(Token::Let)?;
        let mutable = parser.mutable()?;
        let identifier = parser.identifier()?;
        parser.expect(Token::Equal)?;
        let value = Expression::parse(parser)?;
        Ok(Self {
            identifier,
            mutable,
            value,
        })
    }
}
