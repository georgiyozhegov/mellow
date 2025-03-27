use mellow_lex::Token;
use mellow_error::Result;

use super::{Expression, Identifier};
use crate::{Parse, Parser};

#[derive(Debug, Clone)]
pub struct Assign {
    pub identifier: Identifier,
    pub value: Expression,
}

impl Parse for Assign {
    fn parse(parser: &mut Parser) -> Result<Self> {
        let identifier = Identifier::parse(parser)?;
        parser.expect(Token::Equal)?;
        let value = Expression::parse(parser)?;
        Ok(Assign { identifier, value })
    }
}
