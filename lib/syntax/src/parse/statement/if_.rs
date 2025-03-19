use crate::{lex::Token, parse::{Expression, Parser}, Error, Result};

use super::Statement;

#[derive(Debug, Clone)]
pub struct IfBranch {
    pub condition: Expression,
    pub body: Vec<Statement>,
}

impl IfBranch {
    pub fn new(condition: Expression, body: Vec<Statement>) -> Self {
        Self { condition, body }
    }
}

impl IfBranch {
    pub fn parse(parser: &mut Parser) -> Result<Self> {
        let condition = Expression::parse(parser)?;
        parser.expect(Token::Then)?;
        let body = parser.body()?;
        Ok(Self::new(condition, body))
    }
}

#[derive(Debug, Clone)]
pub struct If {
    pub if_: IfBranch,
    pub or: Vec<IfBranch>,
    pub else_: Vec<Statement>,
}

impl If {
    pub fn parse(parser: &mut Parser) -> Result<Statement> {
        parser.expect(Token::If)?;
        let if_ = IfBranch::parse(parser)?;
        let or = Self::or(parser)?;
        let else_ = Self::else_(parser)?;
        parser.expect(Token::End)?;
        Ok(Statement::If(If {
            if_,
            or,
            else_,
        }))
    }

    fn or(parser: &mut Parser) -> Result<Vec<IfBranch>> {
        let mut or = Vec::new();
        while parser.peek()?.is_some_and(|token| token == Token::Or) {
            parser.next()?;
            let branch = IfBranch::parse(parser)?;
            or.push(branch);
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
