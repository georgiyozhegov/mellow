use mellow_lex::{Error, Result, Token};

use crate::{Expression, Parser};

#[derive(Debug, Clone)]
pub struct If {
    pub condition: Box<Expression>,
    pub if_: Box<Expression>,
    pub or: Vec<(Expression, Expression)>,
    pub else_: Option<Box<Expression>>,
}

impl If {
    pub fn parse(parser: &mut Parser) -> Result<Expression> {
        let condition = Expression::parse(parser)?;
        parser.expect(Token::Then)?;
        let true_ = Expression::parse(parser)?;
        let or = Self::or(parser)?;
        let else_ = Self::else_(parser)?;
        parser.expect(Token::End)?;
        Ok(Expression::If(If {
            condition: Box::new(condition),
            if_: Box::new(true_),
            or,
            else_,
        }))
    }

    fn or(parser: &mut Parser) -> Result<Vec<(Expression, Expression)>> {
        let mut or = Vec::new();
        while parser.peek()?.is_some_and(|token| token == Token::Or) {
            parser.next()?;
            let condition = Expression::parse(parser)?;
            parser.expect(Token::Then)?;
            let body = Expression::parse(parser)?;
            or.push((condition, body));
        }
        match parser.peek()? {
            Some(Token::Else) | Some(Token::End) => Ok(or),
            token => Err(Error::grammar("'else' or 'end' after 'or' body", token)),
        }
    }

    fn else_(parser: &mut Parser) -> Result<Option<Box<Expression>>> {
        match parser.peek()? {
            Some(Token::Else) => {
                parser.next()?;
                Ok(Some(Box::new(Expression::parse(parser)?)))
            }
            Some(Token::End) => Ok(None),
            token => Err(Error::grammar(
                "'else', 'or' or 'end' after 'if' body",
                token,
            )),
        }
    }
}
