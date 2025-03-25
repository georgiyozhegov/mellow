use mellow_lex::{Error, Result, Token};

use super::Statement;
use crate::{tree::Body, Expression, Parser};

#[derive(Debug, Clone)]
pub struct IfBranch {
    pub condition: Expression,
    pub body: Body,
}

impl IfBranch {
    pub fn parse(parser: &mut Parser) -> Result<Self> {
        let condition = Expression::parse(parser)?;
        parser.expect(Token::Then)?;
        let body = Body::parse(parser)?;
        Ok(Self { condition, body })
    }
}

#[derive(Debug, Clone)]
pub struct If {
    pub if_: IfBranch,
    pub or: Vec<IfBranch>,
    pub else_: Body,
}

impl If {
    pub fn parse(parser: &mut Parser) -> Result<Statement> {
        parser.expect(Token::If)?;
        let if_ = IfBranch::parse(parser)?;
        let or = Self::or(parser)?;
        let else_ = Self::else_(parser)?;
        parser.expect(Token::End)?;
        Ok(Statement::If(If { if_, or, else_ }))
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

    fn else_(parser: &mut Parser) -> Result<Body> {
        match parser.peek()? {
            Some(Token::Else) => {
                parser.next()?;
                Body::parse(parser)
            }
            Some(Token::End) => Ok(Body::empty()),
            token => Err(Error::grammar("'else', 'or' or 'end'", token)),
        }
    }
}
