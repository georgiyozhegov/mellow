use super::super::{BinaryKind, UnaryKind};
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

#[derive(Debug, Clone)]
pub struct Integer {
    pub value: i128,
}

#[derive(Debug, Clone)]
pub struct Identifier {
    pub name: String,
}

#[derive(Debug, Clone)]
pub struct Boolean {
    pub value: bool,
}

#[derive(Debug, Clone)]
pub struct Str {
    pub value: String,
}

#[derive(Debug, Clone)]
pub struct Binary {
    pub kind: BinaryKind,
    pub left: Box<Expression>,
    pub right: Box<Expression>,
}

#[derive(Debug, Clone)]
pub struct Unary {
    pub kind: UnaryKind,
    pub inner: Box<Expression>,
}

#[derive(Debug, Clone)]
pub struct If {
    pub condition: Box<Expression>,
    pub if_: Box<Expression>,
    pub or: Vec<(Expression, Expression)>,
    pub else_: Option<Box<Expression>>,
}
