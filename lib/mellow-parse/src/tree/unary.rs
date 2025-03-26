use mellow_lex::Token;

use super::Expression;

#[derive(Debug, Clone)]
pub struct Unary {
    pub kind: UnaryKind,
    pub inner: Box<Expression>,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum UnaryKind {
    Not,
    Negate,
}

impl TryFrom<&Token> for UnaryKind {
    type Error = ();

    fn try_from(value: &Token) -> Result<Self, Self::Error> {
        match value {
            Token::Not => Ok(UnaryKind::Not),
            Token::Negate => Ok(UnaryKind::Negate),
            _ => Err(()),
        }
    }
}
