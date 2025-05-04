#[derive(Clone, Debug)]
pub struct Token {
    kind: TokenKind,
    // todo: span tracking
}

impl Token {
    pub fn new(kind: TokenKind) -> Self {
        Self { kind }
    }

    pub fn kind(&self) -> &TokenKind {
        &self.kind
    }

    pub fn take_kind(self) -> TokenKind {
        self.kind
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum TokenKind {
    // literals
    Integer(i128),
    Identifier(String),
    String(String),
    // keywords
    True,
    False,
    Let,
    Mutable,
    If,
    Or,
    Else,
    Then,
    While,
    Do,
    End,
    Debug,
    // operators and punctuation
    Equal,
    Plus,
    Minus,
    Star,
    Slash,
    Greater,
    Less,
    Question,
    Not,
    LeftParenthesis,
    RightParenthesis,
}
