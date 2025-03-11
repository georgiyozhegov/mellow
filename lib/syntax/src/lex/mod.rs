mod lex;
mod pattern;
mod token;
use std::{iter::Peekable, str::Chars};

pub use lex::Lex;
pub use token::Token;

pub type Source<'s> = Peekable<Chars<'s>>;
