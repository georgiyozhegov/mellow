use crate::token::{BinaryOperator, Token, UnaryOperator};

#[derive(Debug, Clone)]
pub enum Statement<I: Sized> {
    Let {
        identifier: I,
        mutable: bool,
        value: Expression,
    },
    Assign {
        identifier: I,
        value: Expression,
    },
    If {
        condition: Expression,
        if_: Vec<Statement<I>>,
        or: Vec<(Expression, Vec<Statement<I>>)>,
        else_: Vec<Statement<I>>,
    },
    While {
        condition: Expression,
        body: Vec<Statement<I>>,
    },
    For {
        item: String,
        sequence: Expression,
        body: Vec<Statement<I>>,
    },
}

#[derive(Debug, Clone)]
pub enum Expression {
    Integer(i128),
    Identifier(String),
    Boolean(bool),
    String(String),
    Binary(BinaryOperator, Box<Expression>, Box<Expression>),
    Unary(UnaryOperator, Box<Expression>),
    If {
        condition: Box<Expression>,
        if_: Box<Expression>,
        or: Vec<(Expression, Expression)>,
        else_: Option<Box<Expression>>,
    },
}

impl From<&Token> for Expression {
    fn from(value: &Token) -> Self {
        match value {
            Token::Integer(value) => Self::Integer(*value),
            Token::Identifier(value) => Self::Identifier(value.to_owned()),
            Token::True => Self::Boolean(true),
            Token::False => Self::Boolean(false),
            Token::String(value) => Self::String(value.to_owned()),
            _ => panic!(),
        }
    }
}
