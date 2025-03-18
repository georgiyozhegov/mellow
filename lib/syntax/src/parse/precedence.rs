use super::{rpn::RpnItem, expression::{BinaryKind, UnaryKind}};

pub trait Precedence {
    fn precedence(&self) -> u8;
}

impl Precedence for BinaryKind {
    fn precedence(&self) -> u8 {
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

impl Precedence for UnaryKind {
    fn precedence(&self) -> u8 {
        match self {
            Self::Negate => 4,
            Self::Not => 4,
        }
    }
}

impl Precedence for RpnItem {
    fn precedence(&self) -> u8 {
        match self {
            Self::Binary(kind) => kind.precedence(),
            Self::Unary(kind) => kind.precedence(),
            Self::Parenthesis => 0,
        }
    }
}
