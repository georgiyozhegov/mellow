mod error;
pub mod lex;
pub mod parse;
pub use error::{Error, Result};
use lex::Lex;
use parse::{Parse, Statement};

pub fn construct(source: lex::Source) -> Result<Vec<Statement>> {
    let lex = Lex::new(source);
    let parse = Parse::new(lex.peekable());
    parse.collect()
}
