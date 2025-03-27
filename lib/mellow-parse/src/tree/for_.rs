use mellow_lex::Token;
use mellow_error::Result;

use crate::{Expression, Parse, Parser, tree::Body};

use super::Identifier;

#[derive(Debug, Clone)]
pub struct For {
    pub item: Identifier,
    pub sequence: Expression,
    pub body: Body,
}

impl Parse for For {
    fn parse(parser: &mut Parser) -> Result<Self> {
        parser.expect(Token::For)?;
        let item = Identifier::parse(parser)?;
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
