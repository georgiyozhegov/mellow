mod binary;
mod parse;
mod precedence;
mod rpn;
mod tree;
mod unary;
use std::iter::Peekable;

pub use binary::BinaryKind;
pub use parse::Parse;
pub use precedence::Precedence;
pub use tree::{Expression, Statement};
pub use unary::UnaryKind;

use crate::lex::Lex;

pub type Source<'s> = Peekable<Lex<'s>>;
