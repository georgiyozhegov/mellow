use crate::lex::Token;

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

impl From<Token> for Option<BinaryKind> {
    fn from(token: Token) -> Self {
        match token {
            Token::Plus => Some(BinaryKind::Add),
            Token::Minus => Some(BinaryKind::Subtract),
            Token::Star => Some(BinaryKind::Multiply),
            Token::Slash => Some(BinaryKind::Divide),
            Token::Greater => Some(BinaryKind::Greater),
            Token::Less => Some(BinaryKind::Less),
            Token::Equal => Some(BinaryKind::Equal),
            _ => None,
        }
    }
}

impl Token {
    pub fn is_binary(&self) -> bool {
        let binary: Option<BinaryKind> = self.clone().into();
        binary.is_some()
    }
}
