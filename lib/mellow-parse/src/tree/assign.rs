use mellow_lex::{Result, Token};

use super::Expression;
use crate::{Parse, Parser};

#[derive(Debug, Clone)]
pub struct Assign {
    pub identifier: String,
    pub value: Expression,
}

impl Parse for Assign {
    fn parse(parser: &mut Parser) -> Result<Self> {
        let identifier = parser.identifier()?;
        parser.expect(Token::Equal)?;
        let value = Expression::parse(parser)?;
        Ok(Assign { identifier, value })
    }
}
