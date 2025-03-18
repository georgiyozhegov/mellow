mod integer;
mod identifier;
mod boolean;
mod string;
mod binary;
mod unary;
mod if_;

pub use boolean::*;
pub use identifier::*;
pub use if_::*;
pub use integer::*;
pub use string::*;
pub use binary::*;
pub use unary::*;

use crate::lex::Token;

#[derive(Debug, Clone)]
pub enum Expression {
    Integer(Integer),
    Identifier(Identifier),
    Boolean(Boolean),
    String(Str),
    Binary(Binary),
    Unary(Unary),
    If(If),
}

impl From<Token> for Expression {
    fn from(value: Token) -> Self {
        match value {
            Token::Integer(value) => Self::Integer(Integer { value }),
            Token::Identifier(name) => Self::Identifier(Identifier { name }),
            Token::True => Self::Boolean(Boolean { value: true }),
            Token::False => Self::Boolean(Boolean { value: false }),
            Token::String(value) => Self::String(Str { value }),
            _ => panic!(),
        }
    }
}
