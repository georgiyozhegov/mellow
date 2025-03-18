mod binary;
mod parse;
mod precedence;
mod rpn;
pub mod expression;
pub mod statement;
mod unary;
mod visit;
use std::iter::Peekable;

pub use binary::BinaryKind;
pub use parse::Parser;
pub use precedence::Precedence;
pub use expression::Expression;
pub use statement::Statement;
pub use unary::UnaryKind;
pub use visit::*;

use crate::lex::Lex;

pub type Source<'s> = Peekable<Lex<'s>>;
