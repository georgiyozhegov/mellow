use crate::{
    lex::Token,
    parse::{Expression, Parser},
    Result,
};

use super::Statement;

#[derive(Debug, Clone)]
pub struct For {
    pub item: String,
    pub sequence: Expression,
    pub body: Vec<Statement>,
}

impl For {
    pub fn parse(parser: &mut Parser) -> Result<Statement> {
        parser.expect(Token::For)?;
        let item = parser.identifier()?;
        parser.expect(Token::In)?;
        let sequence = Expression::parse(parser)?;
        parser.expect(Token::Do)?;
        let body = parser.body()?;
        parser.expect(Token::End)?;
        Ok(Statement::For(For {
            item,
            sequence,
            body,
        }))
    }
}
