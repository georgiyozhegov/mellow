mod binary;
mod boolean;
mod identifier;
mod if_;
mod integer;
mod string;
mod unary;

pub use binary::*;
pub use boolean::*;
pub use identifier::*;
pub use if_::*;
pub use integer::*;
pub use string::*;
pub use unary::*;

use crate::{lex::Token, literal, Error, Result};

use super::{
    rpn::{ExpressionState, Rpn, RpnItem},
    Parser,
};

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

impl Expression {
    pub fn parse(parser: &mut Parser) -> Result<Expression> {
        let mut rpn = Rpn::new();
        let mut status = ExpressionState::default();
        while let Some(token) = parser.peek()? {
            if status.stop(&token)? {
                break;
            }
            match token {
                literal!() => {
                    rpn.value(Expression::from(token));
                    parser.next()?;
                }
                token if token.is_binary() => {
                    let binary: Option<BinaryKind> = token.into();
                    rpn.binary(binary.unwrap());
                    parser.next()?;
                }
                token if token.is_unary() => {
                    let unary: Option<UnaryKind> = token.into();
                    rpn.unary(unary.unwrap());
                    parser.next()?;
                }
                Token::LeftParenthesis => {
                    rpn.item(RpnItem::Parenthesis);
                    parser.next()?;
                }
                Token::RightParenthesis => {
                    parser.next()?;
                    rpn.parenthesis();
                }
                Token::If => {
                    parser.next()?;
                    rpn.value(If::parse(parser)?);
                }
                _ => {
                    return Err(Error::grammar("expression", Some(token)));
                }
            }
        }
        Ok(rpn.collapse())
    }
}
