use mellow_lex::Token;
use mellow_error::{Result, Error};

use crate::{Expression, Parse, Parser};

#[derive(Debug, Clone)]
pub struct Branch<B>
where
    B: Parse,
{
    pub condition: Box<Expression>,
    pub body: Box<B>,
}

impl<B> Branch<B>
where
    B: Parse,
{
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

impl<B> If<B>
where
    B: Parse,
{
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
            Some(token) => Err(Error::expected_but_got("'else' or 'end' after 'or' body", token)),
            _ => Err(Error::expected_but_got("'else' or 'end' after 'or' body", "EOF")),
        }
    }

    fn else_(parser: &mut Parser) -> Result<Option<Box<B>>> {
        match parser.peek()? {
            Some(Token::Else) => {
                parser.next()?;
                Ok(Some(Box::new(B::parse(parser)?)))
            }
            Some(Token::End) => Ok(None),
            Some(token) => Err(Error::expected_but_got("'else', 'or' or 'end'", token)),
            _ => Err(Error::expected_but_got("'else', 'or' or 'end'", "EOF")),
        }
    }
}
