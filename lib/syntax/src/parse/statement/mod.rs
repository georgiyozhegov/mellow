mod let_;
mod assign;
mod if_;
mod while_;
mod for_;
mod debug;
pub use assign::*;
pub use let_::*;
pub use if_::*;
pub use while_::*;
pub use for_::*;
pub use debug::*;

use crate::{lex::Token, Error, Result};

use super::Parser;

#[derive(Debug, Clone)]
pub enum Statement {
    Let(Let),
    Assign(Assign),
    If(If),
    While(While),
    For(For),
    Debug(Debug),
}

impl Statement {
    pub fn parse(parser: &mut Parser) -> Result<Statement> {
        match parser.peek()? {
            Some(Token::Let) => Let::parse(parser),
            Some(Token::Identifier(..)) => Assign::parse(parser),
            Some(Token::If) => If::parse(parser),
            Some(Token::While) => While::parse(parser),
            Some(Token::For) => For::parse(parser),
            Some(Token::Debug) => Debug::parse(parser),
            token => Err(Error::grammar("statement", token)),
        }
    }
}
