use super::*;
use crate::{Parse, Parser};
use mellow_lex::Token;
use mellow_error::{Result, Error};

#[derive(Debug, Clone)]
pub enum Statement {
    Let(Let),
    Assign(Assign),
    If(If<Body>),
    While(While),
    For(For),
    Debug(Debug),
}

impl Statement {
    pub fn parse(parser: &mut Parser) -> Result<Statement> {
        match parser.peek()? {
            Some(Token::Let) => Ok(Self::Let(Let::parse(parser)?)),
            Some(Token::Identifier(..)) => Ok(Self::Assign(Assign::parse(parser)?)),
            Some(Token::If) => Ok(Self::If(If::<Body>::parse(parser)?)),
            Some(Token::While) => Ok(Self::While(While::parse(parser)?)),
            Some(Token::For) => Ok(Self::For(For::parse(parser)?)),
            Some(Token::Debug) => Ok(Self::Debug(Debug::parse(parser)?)),
            Some(token) => Err(Error::expected_but_got("statement", token)),
            _ => Err(Error::expected_but_got("statement", "EOF")),
        }
    }
}
