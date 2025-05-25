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

impl TokenKind {
    pub fn from_numeric(buffer: String) -> Self {
        let buffer = buffer.replace('_', "");
        let value = buffer.parse().unwrap();
        Self::Integer(value)
    }

    pub fn from_alphabetic(buffer: String) -> Self {
        match buffer.as_str() {
            "true" => Self::True,
            "false" => Self::False,
            "let" => Self::Let,
            "mutable" => Self::Mutable,
            "if" => Self::If,
            "or" => Self::Or,
            "else" => Self::Else,
            "then" => Self::Then,
            "while" => Self::While,
            "do" => Self::Do,
            "end" => Self::End,
            "debug" => Self::Debug,
            _ => Self::Identifier(buffer),
        }
    }
}
