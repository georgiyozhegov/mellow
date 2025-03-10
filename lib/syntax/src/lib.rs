mod error;
mod lex;
mod parse;
pub mod pattern;
mod rpn;
pub mod token;
pub mod tree;
pub use error::Error;
pub use lex::Lex;
pub use parse::Parse;
use tree::Statement;

pub fn construct(source: lex::Source) -> Result<Vec<Statement>, Error> {
    let lex = Lex::new(source);
    let parse = Parse::new(lex.peekable());
    parse.collect()
}
