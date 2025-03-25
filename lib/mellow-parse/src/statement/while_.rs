use mellow_lex::{Result, Token};

use super::Statement;
use crate::{tree::Body, Expression, Parser};

#[derive(Debug, Clone)]
pub struct While {
    pub condition: Expression,
    pub body: Body,
}

impl While {
    pub fn parse(parser: &mut Parser) -> Result<Statement> {
        parser.expect(Token::While)?;
        let condition = Expression::parse(parser)?;
        parser.expect(Token::Do)?;
        let body = Body::parse(parser)?;
        parser.expect(Token::End)?;
        Ok(Statement::While(While { condition, body }))
    }
}
