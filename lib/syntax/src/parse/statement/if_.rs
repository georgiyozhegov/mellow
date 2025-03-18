use crate::{lex::Token, parse::{Expression, Parser}, Error, Result};

use super::Statement;

#[derive(Debug, Clone)]
pub struct If {
    pub condition: Expression,
    pub if_: Vec<Statement>,
    pub or: Vec<(Expression, Vec<Statement>)>,
    pub else_: Vec<Statement>,
}

impl If {
    pub fn parse(parser: &mut Parser) -> Result<Statement> {
        parser.expect(Token::If)?;
        let condition = Expression::parse(parser)?;
        parser.expect(Token::Then)?;
        let if_ = parser.body()?;
        let or = Self::or(parser)?;
        let else_ = Self::else_(parser)?;
        parser.expect(Token::End)?;
        Ok(Statement::If(If {
            condition,
            if_,
            or,
            else_,
        }))
    }

    fn or(parser: &mut Parser) -> Result<Vec<(Expression, Vec<Statement>)>> {
        let mut or = Vec::new();
        while parser.peek()?.is_some_and(|token| token == Token::Or) {
            parser.next()?;
            let condition = Expression::parse(parser)?;
            parser.expect(Token::Then)?;
            let body = parser.body()?;
            or.push((condition, body));
        }
        match parser.peek()? {
            Some(Token::Else) | Some(Token::End) => Ok(or),
            token => Err(Error::grammar("'else' or 'end' after 'or' body", token)),
        }
    }

    fn else_(parser: &mut Parser) -> Result<Vec<Statement>> {
        match parser.peek()? {
            Some(Token::Else) => {
                parser.next()?;
                parser.body()
            }
            Some(Token::End) => Ok(vec![]),
            token => Err(Error::grammar("'else', 'or' or 'end'", token)),
        }
    }
}
