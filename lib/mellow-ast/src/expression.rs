use mellow_lex::TokenKind;

use super::*;

#[derive(Debug, Clone)]
pub enum Expression {
    Integer(Integer),
    Identifier(Identifier),
    Boolean(Boolean),
    String(Str),
    Binary(Binary),
    Unary(Unary),
    If(If<Expression>),
}

impl From<TokenKind> for Expression {
    fn from(kind: TokenKind) -> Self {
        match kind {
            TokenKind::Integer(value) => Self::Integer(Integer { value }),
            TokenKind::Identifier(name) => Self::Identifier(Identifier { name }),
            TokenKind::True => Self::Boolean(Boolean { value: true }),
            TokenKind::False => Self::Boolean(Boolean { value: false }),
            TokenKind::String(value) => Self::String(Str { value }),
            _ => panic!(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Integer {
    pub value: i128,
}

#[derive(Debug, Clone)]
pub struct Boolean {
    pub value: bool,
}

#[derive(Debug, Clone)]
pub struct Str {
    pub value: String,
}

#[derive(Debug, Clone)]
pub struct Binary {
    pub kind: BinaryKind,
    pub left: Box<Expression>,
    pub right: Box<Expression>,
}

impl Binary {
    pub fn new(kind: BinaryKind, left: Expression, right: Expression) -> Self {
        Self {
            kind,
            left: Box::new(left),
            right: Box::new(right),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum BinaryKind {
    Add,
    Subtract,
    Multiply,
    Divide,
    Greater,
    Less,
    Equal,
}

impl TryFrom<&TokenKind> for BinaryKind {
    type Error = ();

    fn try_from(kind: &TokenKind) -> std::result::Result<Self, Self::Error> {
        match kind {
            TokenKind::Plus => Ok(BinaryKind::Add),
            TokenKind::Minus => Ok(BinaryKind::Subtract),
            TokenKind::Star => Ok(BinaryKind::Multiply),
            TokenKind::Slash => Ok(BinaryKind::Divide),
            TokenKind::Greater => Ok(BinaryKind::Greater),
            TokenKind::Less => Ok(BinaryKind::Less),
            TokenKind::Question => Ok(BinaryKind::Equal),
            _ => todo!("remove this try_from implementation"),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Unary {
    pub kind: UnaryKind,
    pub inner: Box<Expression>,
}

impl Unary {
    pub fn new(kind: UnaryKind, inner: Expression) -> Self {
        Self {
            kind,
            inner: Box::new(inner),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum UnaryKind {
    Not,
    Negate,
}

impl TryFrom<&TokenKind> for UnaryKind {
    type Error = ();

    fn try_from(kind: &TokenKind) -> std::result::Result<Self, Self::Error> {
        match kind {
            TokenKind::Not => Ok(UnaryKind::Not),
            _ => todo!("remove this try_from implementation"),
        }
    }
}
