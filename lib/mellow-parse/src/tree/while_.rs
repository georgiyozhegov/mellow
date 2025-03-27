use mellow_lex::Token;
use mellow_error::Result;

use crate::{Expression, Parse, Parser, tree::Body};

#[derive(Debug, Clone)]
pub struct While {
    pub condition: Expression,
    pub body: Body,
}

impl Parse for While {
    fn parse(parser: &mut Parser) -> Result<Self> {
        parser.expect(Token::While)?;
        let condition = Expression::parse(parser)?;
        parser.expect(Token::Do)?;
        let body = Body::parse(parser)?;
        parser.expect(Token::End)?;
        Ok(Self { condition, body })
    }
}
