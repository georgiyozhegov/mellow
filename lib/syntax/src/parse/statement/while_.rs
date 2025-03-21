use crate::{
    lex::Token,
    parse::{Expression, Parser},
    Result,
};

use super::Statement;

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
