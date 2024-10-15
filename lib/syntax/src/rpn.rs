use crate::{
    token::{BinaryOperator, Token, UnaryOperator},
    tree::Expression,
    SyntaxError,
};

#[macro_export]
macro_rules! end_of_expression {
    () => {
        Token::Let
            | Token::While
            | Token::Do
            | Token::Then
            | Token::Else
            | Token::In
            | Token::From
            | Token::To
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
            Self::Add => 1,
            Self::Subtract => 1,
            Self::Multiply => 2,
            Self::Divide => 2,
        }
    }
}

impl UnaryOperator {
    pub fn precedence(&self) -> u8 {
        match self {
            Self::Negate => 3,
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
    pub fn check(&mut self, token: &Token) -> Result<(), SyntaxError> {
        match (&self, token) {
            (
                Self::Value,
                Token::Integer(_) | Token::Identifier(_) | Token::True | Token::False | Token::If,
            ) => {
                *self = Self::Item;
                Ok(())
            }
            (Self::Value, Token::LeftParenthesis) => Ok(()),
            (Self::Item, Token::BinaryOperator(_)) => {
                *self = Self::Value;
                Ok(())
            }
            (Self::Item, Token::RightParenthesis | Token::UnaryOperator(_)) => Ok(()),
            (Self::Item, end_of_expression!()) => Ok(()),
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
