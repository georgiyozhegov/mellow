use mellow_lex::{Error, Result, Token};

use crate::{Parse, Expression, Parser};

#[derive(Debug, Clone)]
pub struct Branch<B>
where
    B: Parse,
{
    pub condition: Box<Expression>,
    pub body: Box<B>,
}

impl<B> Branch<B> where B: Parse {
    pub fn parse(parser: &mut Parser) -> Result<Self> {
        let condition = Expression::parse(parser)?;
        parser.expect(Token::Then)?;
        let body = B::parse(parser)?;
        Ok(Self {
            condition: Box::new(condition),
            body: Box::new(body),
        })
    }
}

#[derive(Debug, Clone)]
pub struct If<B>
where
    B: Parse,
{
    pub if_: Branch<B>,
    pub or: Vec<Branch<B>>,
    pub else_: Option<Box<B>>,
}

impl<B> If<B> where B: Parse {
    pub fn parse(parser: &mut Parser) -> Result<Self> {
        parser.expect(Token::If)?;
        let if_ = Branch::parse(parser)?;
        let or = Self::or(parser)?;
        let else_ = Self::else_(parser)?;
        parser.expect(Token::End)?;
        Ok(Self { if_, or, else_ })
    }

    fn or(parser: &mut Parser) -> Result<Vec<Branch<B>>> {
        let mut or = Vec::new();
        while parser.peek()?.is_some_and(|token| token == Token::Or) {
            parser.next()?;
            let branch = Branch::<B>::parse(parser)?;
            or.push(branch);
        }
        match parser.peek()? {
            Some(Token::Else) | Some(Token::End) => Ok(or),
            token => Err(Error::grammar("'else' or 'end' after 'or' body", token)),
        }
    }

    fn else_(parser: &mut Parser) -> Result<Option<Box<B>>> {
        match parser.peek()? {
            Some(Token::Else) => {
                parser.next()?;
                Ok(Some(Box::new(B::parse(parser)?)))
            }
            Some(Token::End) => Ok(None),
            token => Err(Error::grammar("'else', 'or' or 'end'", token)),
        }
    }
}
