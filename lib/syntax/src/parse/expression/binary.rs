use mellow_lex::Token;

use super::Expression;

#[derive(Debug, Clone)]
pub struct Binary {
    pub kind: BinaryKind,
    pub left: Box<Expression>,
    pub right: Box<Expression>,
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

impl TryFrom<&Token> for BinaryKind {
    type Error = ();

    fn try_from(value: &Token) -> Result<Self, Self::Error> {
        match value {
            Token::Plus => Ok(BinaryKind::Add),
            Token::Minus => Ok(BinaryKind::Subtract),
            Token::Star => Ok(BinaryKind::Multiply),
            Token::Slash => Ok(BinaryKind::Divide),
            Token::Greater => Ok(BinaryKind::Greater),
            Token::Less => Ok(BinaryKind::Less),
            Token::Question => Ok(BinaryKind::Equal),
            _ => Err(()),
        }
    }
}
