use crate::token::Token;

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum UnaryKind {
    Not,
    Negate,
}

impl From<Token> for Option<UnaryKind> {
    fn from(token: Token) -> Self {
        match token {
            Token::Not => Some(UnaryKind::Not),
            Token::Negate => Some(UnaryKind::Negate),
            _ => None,
        }
    }
}

impl Token {
    pub fn is_unary(&self) -> bool {
        let unary: Option<UnaryKind> = self.clone().into();
        unary.is_some()
    }
}
