use crate::{
    token::{BinaryOperator, Token, UnaryOperator},
    tree::Expression,
    Error,
};

#[macro_export]
macro_rules! literal {
    () => {
        Token::Integer(_) | Token::Identifier(_) | Token::True | Token::False | Token::String(_)
    };
}

#[macro_export]
macro_rules! end_of_expression {
    () => {
        Token::Let
            | Token::Identifier(..)
            | Token::If
            | Token::Or
            | Token::Then
            | Token::Else
            | Token::While
            | Token::In
            | Token::Do
            | Token::End
            | Token::Debug
    };
}

pub struct Rpn {
    values: Vec<Expression>,
    items: Vec<RpnItem>,
}

impl Rpn {
    pub fn new(values: Vec<Expression>, items: Vec<RpnItem>) -> Self {
        Self { values, items }
    }
}

impl Default for Rpn {
    fn default() -> Self {
        Self::new(Vec::new(), Vec::new())
    }
}

impl Rpn {
    pub fn value(&mut self, value: Expression) {
        self.values.push(value);
    }

    pub fn item(&mut self, item: RpnItem) {
        self.items.push(item);
    }

    pub fn binary(&mut self, item: RpnItem) {
        while self
            .items
            .last()
            .is_some_and(|previous| previous.precedence() >= item.precedence())
        {
            let previous = self.items.pop().unwrap();
            self.fold(previous);
        }
        self.items.push(item);
    }

    pub fn unary(&mut self, item: RpnItem) {
        self.items.push(item);
    }

    pub fn parenthesis(&mut self) {
        while let Some(item) = self.items.pop() {
            if item == RpnItem::Parenthesis {
                break;
            }
            self.fold(item);
        }
    }

    pub fn collapse(&mut self) -> Expression {
        while let Some(item) = self.items.pop() {
            self.fold(item);
        }
        self.values.pop().unwrap()
    }
}

impl Rpn {
    fn fold(&mut self, item: RpnItem) {
        match item {
            RpnItem::Binary(operator) => {
                let right = self.values.pop().unwrap();
                let left = self.values.pop().unwrap();
                self.values.push(Expression::Binary(
                    operator,
                    Box::new(left),
                    Box::new(right),
                ));
            }
            RpnItem::Unary(operator) => {
                let value = self.values.pop().unwrap();
                self.values
                    .push(Expression::Unary(operator, Box::new(value)));
            }
            _ => panic!(),
        }
    }
}

#[derive(PartialEq, Debug)]
pub enum RpnItem {
    Binary(BinaryOperator),
    Unary(UnaryOperator),
    Parenthesis,
}

impl From<Token> for RpnItem {
    fn from(value: Token) -> Self {
        match value {
            Token::BinaryOperator(operator) => Self::Binary(operator),
            Token::UnaryOperator(operator) => Self::Unary(operator),
            Token::LeftParenthesis => Self::Parenthesis,
            _ => panic!(),
        }
    }
}

impl RpnItem {
    pub fn precedence(&self) -> u8 {
        match self {
            Self::Binary(operator) => operator.precedence(),
            Self::Unary(operator) => operator.precedence(),
            Self::Parenthesis => 0,
        }
    }
}

impl BinaryOperator {
    pub fn precedence(&self) -> u8 {
        match self {
            Self::Add => 2,
            Self::Subtract => 2,
            Self::Multiply => 3,
            Self::Divide => 3,
            Self::Greater => 1,
            Self::Less => 1,
            Self::Equal => 1,
        }
    }
}

impl UnaryOperator {
    pub fn precedence(&self) -> u8 {
        match self {
            Self::Negate => 4,
            Self::Not => 4,
        }
    }
}

#[derive(PartialEq)]
pub enum ExpressionState {
    Value,
    Item,
}

impl Default for ExpressionState {
    fn default() -> Self {
        Self::Value
    }
}

impl ExpressionState {
    pub fn stop(&mut self, token: &Token) -> Result<bool, Error> {
        match (&self, token) {
            (Self::Value, literal!() | Token::If) => {
                *self = Self::Item;
                Ok(false)
            }
            (Self::Value, Token::LeftParenthesis | Token::UnaryOperator(_)) => Ok(false),
            (Self::Item, Token::BinaryOperator(_)) => {
                *self = Self::Value;
                Ok(false)
            }
            (Self::Item, Token::RightParenthesis) => Ok(false),
            (Self::Item, end_of_expression!()) => Ok(true),
            (Self::Value, _) => Err(Error::grammar(
                "literal, identifier or '('",
                Some(token.clone()),
            )),
            (Self::Item, _) => Err(Error::grammar(
                "operator, statement or ')'",
                Some(token.clone()),
            )),
        }
    }
}
