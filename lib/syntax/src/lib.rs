mod binary;
mod error;
mod lex;
mod parse;
pub mod pattern;
mod precedence;
mod rpn;
pub mod token;
pub mod tree;
mod unary;
pub use binary::BinaryKind;
pub use error::Error;
pub use lex::Lex;
pub use parse::Parse;
pub use precedence::Precedence;
use tree::Statement;
pub use unary::UnaryKind;

pub fn construct(source: lex::Source) -> Result<Vec<Statement>, Error> {
    let lex = Lex::new(source);
    let parse = Parse::new(lex.peekable());
    parse.collect()
}
