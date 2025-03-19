pub mod expression;
mod parser;
mod precedence;
mod rpn;
pub mod statement;
mod visit;
use std::iter::Peekable;

pub use expression::Expression;
pub use parser::Parser;
pub use precedence::Precedence;
pub use statement::Statement;
pub use visit::*;

use crate::lex::Lexer;

pub type Source<'s> = Peekable<Lexer<'s>>;
