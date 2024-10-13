use crate::{BinaryOperator, Token, UnaryOperator};

#[derive(Debug)]
pub enum Statement {
    Let {
        identifier: String,
        value: Expression,
    },
    If {
        condition: Expression,
        true_: Box<Statement>,
        false_: Option<Box<Statement>>,
    }
}

#[derive(Debug)]
pub enum Expression {
    Integer(i128),
    Identifier(String),
    Binary(BinaryOperator, Box<Expression>, Box<Expression>),
    Unary(UnaryOperator, Box<Expression>),
    If {
        condition: Box<Expression>,
        true_: Box<Expression>,
        false_: Option<Box<Expression>>,
    },
}

impl From<&Token> for Expression {
    fn from(value: &Token) -> Self {
        match value {
            Token::Integer(value) => Self::Integer(*value),
            Token::Identifier(value) => Self::Identifier(value.to_owned()),
            _ => panic!(),
        }
    }
}
