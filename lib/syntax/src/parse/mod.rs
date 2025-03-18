mod parse;
mod precedence;
mod rpn;
pub mod expression;
pub mod statement;
mod visit;
use std::iter::Peekable;

pub use parse::Parser;
pub use precedence::Precedence;
pub use expression::Expression;
pub use statement::Statement;
pub use visit::*;

use crate::lex::Lex;

pub type Source<'s> = Peekable<Lex<'s>>;
