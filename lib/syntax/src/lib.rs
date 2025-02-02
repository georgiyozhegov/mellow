mod error;
mod lex;
mod parse;
mod rpn;
pub mod token;
pub mod tree;
pub use error::SyntaxError;
pub use lex::Lex;
pub use parse::Parse;
use tree::Statement;

pub fn construct(source: lex::Source) -> Result<Vec<Statement>, SyntaxError> {
    let lex = Lex::new(source);
    let parse = Parse::new(lex.peekable());
    parse.collect()
}
