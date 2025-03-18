mod lexer;
mod pattern;
mod token;
use std::{iter::Peekable, str::Chars};

pub use lexer::Lexer;
pub use token::Token;

pub type Source<'s> = Peekable<Chars<'s>>;
