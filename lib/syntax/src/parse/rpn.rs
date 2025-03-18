use super::{expression::{self, BinaryKind, UnaryKind}, Expression, Precedence};
use crate::{lex::Token, Error};

#[macro_export]
macro_rules! literal {
    () => {
        Token::Integer(_) | Token::Identifier(_) | Token::True | Token::False | Token::String(_)
    };
}

impl Token {
    pub fn is_literal(&self) -> bool {
        matches!(self, Token::Integer(_))
    }
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
    pub fn new() -> Self {
        Self {
            values: Vec::new(),
            items: Vec::new(),
        }
    }
}

impl Rpn {
    pub fn value(&mut self, value: Expression) {
        self.values.push(value);
    }

    pub fn item(&mut self, item: RpnItem) {
        self.items.push(item);
    }

    pub fn binary(&mut self, kind: BinaryKind) {
        let item = RpnItem::Binary(kind);
        while self
            .items
            .last()
            .is_some_and(|previous| previous.precedence() >= item.precedence())
        {
            let previous = self.items.pop().unwrap();
            self.fold(previous);
        }
        self.item(item);
    }

    pub fn unary(&mut self, kind: UnaryKind) {
        self.item(RpnItem::Unary(kind));
    }

    pub fn parenthesis(&mut self) {
        while let Some(item) = self
            .items
            .pop()
            .and_then(|item| (item != RpnItem::Parenthesis).then(|| item))
        {
            self.fold(item);
        }
    }

    pub fn collapse(&mut self) -> Expression {
        while let Some(item) = self.items.pop() {
            self.fold(item);
        }
        self.values.pop().unwrap()
    }

    fn fold(&mut self, item: RpnItem) {
        match item {
            RpnItem::Binary(kind) => {
                let right = self.values.pop().unwrap();
                let left = self.values.pop().unwrap();
                self.value(Expression::Binary(expression::Binary {
                    kind,
                    left: Box::new(left),
                    right: Box::new(right),
                }));
            }
            RpnItem::Unary(kind) => {
                let value = self.values.pop().unwrap();
                self.value(Expression::Unary(expression::Unary {
                    kind,
                    inner: Box::new(value),
                }));
            }
            _ => unreachable!(),
        }
    }
}

#[derive(PartialEq, Debug)]
pub enum RpnItem {
    Binary(BinaryKind),
    Unary(UnaryKind),
    Parenthesis,
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
    fn value(&mut self, token: &Token) -> Result<bool, Error> {
        match token {
            literal!() | Token::If => {
                *self = Self::Item;
                Ok(false)
            }
            token if token.is_unary() => Ok(false),
            token if token.is_binary() => {
                *self = Self::Item;
                Ok(false)
            }
            Token::RightParenthesis => Ok(false),
            _ => Err(Error::grammar(
                "literal, identifier or '('",
                Some(token.clone()),
            )),
        }
    }

    fn item(&mut self, token: &Token) -> Result<bool, Error> {
        match token {
            token if token.is_binary() => {
                *self = Self::Value;
                Ok(false)
            }
            Token::RightParenthesis => Ok(false),
            end_of_expression!() => Ok(true),
            _ => Err(Error::grammar(
                "operator, statement or ')'",
                Some(token.clone()),
            )),
        }
    }

    pub fn stop(&mut self, token: &Token) -> Result<bool, Error> {
        match self {
            Self::Value => self.value(token),
            Self::Item => self.item(token),
        }
    }
}
