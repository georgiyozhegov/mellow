use crate::{
    lex::Token,
    parse::{Expression, Parser},
    Result,
};

use super::Statement;

#[derive(Debug, Clone)]
pub struct Assign {
    pub identifier: String,
    pub value: Expression,
}

impl Assign {
    pub fn parse(parser: &mut Parser) -> Result<Statement> {
        let identifier = parser.identifier()?;
        parser.expect(Token::Equal)?;
        let value = Expression::parse(parser)?;
        Ok(Statement::Assign(Assign { identifier, value }))
    }
}
