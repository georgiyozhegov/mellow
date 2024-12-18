use crate::token::{BinaryOperator, Token, UnaryOperator};

#[derive(Debug, Clone)]
pub enum Statement {
    Let {
        identifier: String,
        mutable: bool,
        value: Expression,
    },
    Change {
        identifier: String,
        value: Expression,
    },
    If {
        condition: Expression,
        true_: Vec<Statement>,
        false_: Vec<Statement>,
    },
    While {
        condition: Expression,
        body: Vec<Statement>,
    },
    ForIn {
        item: String,
        sequence: Expression,
        body: Vec<Statement>,
    },
    ForFromTo {
        item: String,
        start: Expression,
        end: Expression,
        body: Vec<Statement>,
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
        true_: Box<Expression>,
        false_: Option<Box<Expression>>,
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
