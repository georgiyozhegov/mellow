use std::{iter::Peekable, str::Chars};

use crate::{Token, TokenKind};
use mellow_error::{Error, Result};

pub type Source<'s> = Peekable<Chars<'s>>;

pub struct Lex<'l> {
    source: Source<'l>,
}

impl<'l> Lex<'l> {
    pub fn new(source: Source<'l>) -> Self {
        Self { source }
    }
}

impl Iterator for Lex<'_> {
    type Item = Result<Token>;

    fn next(&mut self) -> Option<Self::Item> {
        self.take_while(is_skip);
        let kind = match self.source.peek()? {
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
                return Some(Err(Error::InvalidCharacter(*c)));
            }
        };
        
        Some(Ok(Token::new(kind)))
    }
}

impl Lex<'_> {
    fn take_while(&mut self, predicate: fn(&char) -> bool) -> String {
        let mut output = String::new();
        while self.source.peek().is_some_and(|c| predicate(c)) {
            output.push(self.source.next().unwrap());
        }
        output
    }

    fn numeric(&mut self) -> TokenKind {
        let buffer = self.take_while(is_numeric);
        TokenKind::from_numeric(buffer)
    }

    fn alphabetic(&mut self) -> TokenKind {
        let buffer = self.take_while(is_alphanumeric);
        TokenKind::from_alphabetic(buffer)
    }

    fn string(&mut self) -> TokenKind {
        self.source.next();
        let buffer = self.take_while(|c| *c != '"');
        self.source.next();
        TokenKind::String(buffer)
    }

    fn one(&mut self, kind: TokenKind) -> TokenKind {
        self.source.next();
        kind
    }
}

fn is_alphanumeric(c: &char) -> bool {
    is_alphabetic(c) | is_numeric(c)
}

fn is_numeric(c: &char) -> bool {
    matches!(c, '0'..='9')
}

fn is_alphabetic(c: &char) -> bool {
    matches!(c, 'a'..='z' | 'A'..='Z' | '_')
}

fn is_skip(c: &char) -> bool {
    matches!(c, ' ' | '\t' | '\n')
}
