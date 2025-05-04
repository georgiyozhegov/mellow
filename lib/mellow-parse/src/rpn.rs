use mellow_error::Error;
use mellow_lex::TokenKind;

use crate::{Binary, BinaryKind, Expression, Precedence, Unary, UnaryKind};

#[macro_export]
macro_rules! literal {
    () => {
        TokenKind::Integer(_)
            | TokenKind::Identifier(_)
            | TokenKind::True
            | TokenKind::False
            | TokenKind::String(_)
    };
}

#[macro_export]
macro_rules! end_of_expression {
    () => {
        TokenKind::Let
            | TokenKind::Identifier(..)
            | TokenKind::If
            | TokenKind::Or
            | TokenKind::Then
            | TokenKind::Else
            | TokenKind::While
            | TokenKind::Do
            | TokenKind::End
            | TokenKind::Debug
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
                let expression = Expression::Binary(Binary::new(kind, left, right));
                self.values.push(expression);
            }
            RpnItem::Unary(kind) => {
                let inner = self.values.pop().unwrap();
                let expression = Expression::Unary(Unary::new(kind, inner));
                self.values.push(expression);
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
    fn value(&mut self, kind: &TokenKind) -> Result<bool, Error> {
        match kind {
            literal!() | TokenKind::If => {
                *self = Self::Item;
                Ok(false)
            }
            kind if UnaryKind::try_from(kind).is_ok() => Ok(false),
            kind if BinaryKind::try_from(kind).is_ok() => {
                *self = Self::Item;
                Ok(false)
            }
            TokenKind::RightParenthesis => Ok(false),
            _ => Err(Error::expected_but_got("literal, identifier or '('", "todo")),
        }
    }

    fn item(&mut self, kind: &TokenKind) -> Result<bool, Error> {
        match kind {
            kind if BinaryKind::try_from(kind).is_ok() => {
                *self = Self::Value;
                Ok(false)
            }
            TokenKind::RightParenthesis => Ok(false),
            end_of_expression!() => Ok(true),
            _ => Err(Error::expected_but_got("operator, statement or ')'", "todo")),
        }
    }

    pub fn stop(&mut self, kind: &TokenKind) -> Result<bool, Error> {
        match self {
            Self::Value => self.value(kind),
            Self::Item => self.item(kind),
        }
    }
}
