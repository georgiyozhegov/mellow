use mellow_lex::{Result, Token};

use crate::{Parse, tree::Body, Expression, Parser};

#[derive(Debug, Clone)]
pub struct For {
    pub item: String,
    pub sequence: Expression,
    pub body: Body,
}

impl Parse for For {
    fn parse(parser: &mut Parser) -> Result<Self> {
        parser.expect(Token::For)?;
        let item = parser.identifier()?;
        parser.expect(Token::In)?;
        let sequence = Expression::parse(parser)?;
        parser.expect(Token::Do)?;
        let body = Body::parse(parser)?;
        parser.expect(Token::End)?;
        Ok(For {
            item,
            sequence,
            body,
        })
    }
}
