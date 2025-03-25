use mellow_lex::{Result, Token};

use super::Statement;
use crate::parse::{Expression, Parser};

#[derive(Debug, Clone)]
pub struct Let {
    pub identifier: String,
    pub mutable: bool,
    pub value: Expression,
}

impl Let {
    pub fn parse(parser: &mut Parser) -> Result<Statement> {
        parser.expect(Token::Let)?;
        let mutable = parser.mutable()?;
        let identifier = parser.identifier()?;
        parser.expect(Token::Equal)?;
        let value = Expression::parse(parser)?;
        Ok(Statement::Let(Let {
            identifier,
            mutable,
            value,
        }))
    }
}
