use crate::*;
use mellow_error::{Error, Result};

pub struct Lex {
    source: SourceBuffer,
}

impl Lex {
    pub fn new(source: SourceBuffer) -> Self {
        Self { source }
    }
}

impl Iterator for Lex {
    type Item = Result<Token>;

    fn next(&mut self) -> Option<Self::Item> {
        self.token()
    }
}

impl Lex {
    fn token(&mut self) -> Option<Result<Token>> {
        self.source.take_while(is_skip);
        let kind = match self.source.current()? {
            c if is_numeric(c) => self.numeric(),
            c if is_alphabetic(c) => self.alphabetic(),
            '"' => self.string(),
            '=' => self.one(TokenKind::Equal),
            '+' => self.one(TokenKind::Plus),
            '-' => self.one(TokenKind::Minus),
            '*' => self.one(TokenKind::Star),
            '/' => self.one(TokenKind::Slash),
            '>' => self.one(TokenKind::Greater),
            '<' => self.one(TokenKind::Less),
            '?' => self.one(TokenKind::Question),
            '!' => self.one(TokenKind::Not),
            '(' => self.one(TokenKind::LeftParenthesis),
            ')' => self.one(TokenKind::RightParenthesis),
            c => {
                return Some(Err(Error::InvalidCharacter(c)));
            }
        };
        
        Some(Ok(Token::new(kind)))
    }

    fn numeric(&mut self) -> TokenKind {
        let buffer = self.source.take_while(is_numeric);
        TokenKind::from_numeric(buffer)
    }

    fn alphabetic(&mut self) -> TokenKind {
        let buffer = self.source.take_while(is_alphanumeric);
        TokenKind::from_alphabetic(buffer)
    }

    fn string(&mut self) -> TokenKind {
        self.source.eat();
        let buffer = self.source.take_while(|c| c != '"');
        self.source.eat();
        TokenKind::String(buffer)
    }

    fn one(&mut self, kind: TokenKind) -> TokenKind {
        self.source.eat();
        kind
    }
}
