use crate::{
    lex::Token,
    parse::{Expression, Parser},
    Result,
};

use super::Statement;

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
