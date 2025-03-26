use mellow_lex::{Error, Result, Token};

use super::*;

use crate::{
    literal, Parse, rpn::{ExpressionState, Rpn, RpnItem}, Parser
};

#[derive(Debug, Clone)]
pub enum Expression {
    Integer(Integer),
    Identifier(Identifier),
    Boolean(Boolean),
    String(Str),
    Binary(Binary),
    Unary(Unary),
    If(If<Expression>),
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

impl Parse for Expression {
    fn parse(parser: &mut Parser) -> Result<Self> {
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
                ref token if BinaryKind::try_from(token).is_ok() => {
                    // TODO: if let guard
                    let binary = BinaryKind::try_from(token).unwrap();
                    rpn.binary(binary);
                    parser.next()?;
                }
                ref token if UnaryKind::try_from(token).is_ok() => {
                    // TODO: if let guard
                    let unary = UnaryKind::try_from(token).unwrap();
                    rpn.unary(unary);
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
                    let expression = If::<Expression>::parse(parser)?;
                    rpn.value(Expression::If(expression));
                }
                _ => {
                    return Err(Error::grammar("expression", Some(token)));
                }
            }
        }
        Ok(rpn.collapse())
    }
}
