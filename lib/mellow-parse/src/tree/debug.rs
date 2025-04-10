use mellow_lex::Token;
use mellow_error::Result;

use crate::{Expression, Parse, Parser};

#[derive(Debug, Clone)]
pub struct Debug {
    pub value: Expression,
}

impl Parse for Debug {
    fn parse(parser: &mut Parser) -> Result<Self> {
        parser.expect(Token::Debug)?;
        let value = Expression::parse(parser)?;
        Ok(Debug { value })
    }
}
