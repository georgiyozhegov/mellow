use crate::{
    token::{BinaryOperator, Token, UnaryOperator},
    tree::Expression,
    SyntaxError,
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
    };
}

pub struct Rpn {
    value_stack: Vec<Expression>,
    item_stack: Vec<RpnItem>,
}

impl Rpn {
    pub fn new(value_stack: Vec<Expression>, item_stack: Vec<RpnItem>) -> Self {
        Self {
            value_stack,
            item_stack,
        }
    }
}

impl Default for Rpn {
    fn default() -> Self {
        Self::new(Vec::new(), Vec::new())
    }
}

impl Rpn {
    pub fn value(&mut self, value: Expression) {
        self.value_stack.push(value);
    }

    pub fn item(&mut self, item: RpnItem) {
        self.item_stack.push(item);
    }

    pub fn binary(&mut self, item: RpnItem) {
        while self
            .item_stack
            .last()
            .is_some_and(|previous_item| previous_item.precedence() >= item.precedence())
        {
            let previous_item = self.item_stack.pop().unwrap();
            self.fold(previous_item);
        }
        self.item_stack.push(item);
    }

    pub fn unary(&mut self, item: RpnItem) {
        self.item_stack.push(item);
    }

    pub fn parenthesis(&mut self) {
        while let Some(item) = self.item_stack.pop() {
            if item == RpnItem::Parenthesis {
                break;
            }
            self.fold(item);
        }
    }

    pub fn collapse(&mut self) -> Expression {
        while let Some(item) = self.item_stack.pop() {
            self.fold(item);
        }
        self.value_stack.pop().unwrap()
    }
}

impl Rpn {
    fn fold(&mut self, item: RpnItem) {
        match item {
            RpnItem::Binary(operator) => {
                let left = self.value_stack.pop().unwrap();
                let right = self.value_stack.pop().unwrap();
                self.value_stack.push(Expression::Binary(
                    operator,
                    Box::new(right),
                    Box::new(left),
                ));
            }
            RpnItem::Unary(operator) => {
                let value = self.value_stack.pop().unwrap();
                self.value_stack
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

impl From<&Token> for RpnItem {
    fn from(value: &Token) -> Self {
        match value {
            Token::BinaryOperator(operator) => Self::Binary(operator.clone()),
            Token::UnaryOperator(operator) => Self::Unary(operator.clone()),
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
pub enum Grammar {
    Value,
    Item,
}

impl Default for Grammar {
    fn default() -> Self {
        Self::Value
    }
}

impl Grammar {
    pub fn stop(&mut self, token: &Token) -> Result<bool, SyntaxError> {
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
            (Self::Value, _) => Err(SyntaxError::Grammar {
                expected: "literal, identifier or '('",
                found: Some(token.clone()),
            }),
            (Self::Item, _) => Err(SyntaxError::Grammar {
                expected: "operator, statement or ')'",
                found: Some(token.clone()),
            }),
        }
    }
}
