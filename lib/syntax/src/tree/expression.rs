use crate::{token::Token, BinaryKind, UnaryKind};

#[derive(Debug, Clone)]
pub enum Expression {
    Integer(i128),
    Identifier(String),
    Boolean(bool),
    String(String),
    Binary(BinaryKind, Box<Expression>, Box<Expression>),
    Unary(UnaryKind, Box<Expression>),
    If {
        condition: Box<Expression>,
        if_: Box<Expression>,
        or: Vec<(Expression, Expression)>,
        else_: Option<Box<Expression>>,
    },
}

impl From<Token> for Expression {
    fn from(value: Token) -> Self {
        match value {
            Token::Integer(value) => Self::Integer(value),
            Token::Identifier(value) => Self::Identifier(value),
            Token::True => Self::Boolean(true),
            Token::False => Self::Boolean(false),
            Token::String(value) => Self::String(value),
            _ => panic!(),
        }
    }
}
