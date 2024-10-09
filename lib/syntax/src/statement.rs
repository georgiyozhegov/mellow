use crate::{BinaryOperator, Token, UnaryOperator};

#[derive(Debug)]
pub enum Statement {
    Let {
        identifier: String,
        value: Expression,
    },
}

#[derive(Debug)]
pub enum Expression {
    Integer(i128),
    Binary(BinaryOperator, Box<Expression>, Box<Expression>),
    Unary(UnaryOperator, Box<Expression>),
}

impl From<&Token> for Expression {
    fn from(value: &Token) -> Self {
        match value {
            Token::Integer(value) => Self::Integer(*value),
            _ => panic!(),
        }
    }
}
