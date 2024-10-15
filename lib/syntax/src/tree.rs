use crate::token::{BinaryOperator, Token, UnaryOperator};

#[derive(Debug)]
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
        r#true: Vec<Statement>,
        r#false: Vec<Statement>,
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

#[derive(Debug)]
pub enum Expression {
    Integer(i128),
    Identifier(String),
    Boolean(bool),
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
            _ => panic!(),
        }
    }
}
