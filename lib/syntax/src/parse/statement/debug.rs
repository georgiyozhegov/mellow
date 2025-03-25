use mellow_lex::{Result, Token};

use super::Statement;
use crate::parse::{Expression, Parser};

#[derive(Debug, Clone)]
pub struct Debug {
    pub value: Expression,
}

impl Debug {
    pub fn parse(parser: &mut Parser) -> Result<Statement> {
        parser.expect(Token::Debug)?;
        let value = Expression::parse(parser)?;
        Ok(Statement::Debug(Debug { value }))
    }
}
