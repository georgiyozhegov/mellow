use mellow_lex::{Result, Token};

use super::Statement;
use crate::parse::{Expression, Parser};

#[derive(Debug, Clone)]
pub struct While {
    pub condition: Expression,
    pub body: Vec<Statement>,
}

impl While {
    pub fn parse(parser: &mut Parser) -> Result<Statement> {
        parser.expect(Token::While)?;
        let condition = Expression::parse(parser)?;
        parser.expect(Token::Do)?;
        let body = parser.body()?;
        parser.expect(Token::End)?;
        Ok(Statement::While(While { condition, body }))
    }
}
