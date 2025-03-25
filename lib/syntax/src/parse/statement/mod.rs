mod assign;
mod debug;
mod for_;
mod if_;
mod let_;
mod while_;
pub use assign::*;
pub use debug::*;
pub use for_::*;
pub use if_::*;
pub use let_::*;
use mellow_lex::{Error, Result, Token};
pub use while_::*;

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
