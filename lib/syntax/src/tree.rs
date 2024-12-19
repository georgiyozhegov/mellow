use crate::token::{BinaryOperator, Token, UnaryOperator};

#[derive(Debug, Clone)]
pub enum Statement {
    Let {
        identifier: String,
        mutable: bool,
        value: Expression,
    },
    Assign {
        identifier: String,
        value: Expression,
    },
    If {
        condition: Expression,
        if_: Vec<Statement>,
        or: Vec<(Expression, Vec<Statement>)>,
        else_: Vec<Statement>,
    },
    While {
        condition: Expression,
        body: Vec<Statement>,
    },
    For {
        item: String,
        sequence: Expression,
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

pub enum EitherKind {
    Statement,
    Expression,
}

pub enum Either {
    Statement(Statement),
    Expression(Expression),
}

impl Either {
    pub fn statement(self) -> Statement {
        match self {
            Self::Statement(statement) => statement,
            _ => panic!(),
        }
    }
}

impl Either {
    pub fn expression(self) -> Expression {
        match self {
            Self::Expression(expression) => expression,
            _ => panic!(),
        }
    }
}
