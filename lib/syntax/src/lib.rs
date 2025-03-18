mod error;
pub mod lex;
pub mod parse;
pub use error::{Error, Result};
use lex::Lexer;
use parse::{Parser, Statement};

pub fn construct(source: lex::Source) -> Result<Vec<Statement>> {
    let lexer = Lexer::new(source);
    let parser = Parser::new(lexer.peekable());
    parser.collect()
}
