use std::{iter::Peekable, str::Chars};

use crate::{Token, TokenKind};
use mellow_error::{Error, Result};

macro_rules! numeric {
    () => {
        '0'..='9'
    };
}

macro_rules! alphabetic {
    () => {
        'a'..='z' | 'A'..='Z'
    };
}

macro_rules! skip {
    () => {
        ' ' | '\t' | '\n'
    };
}

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
        self.take_while(|c| matches!(c, skip!()));
        let kind = match self.source.peek()? {
            numeric!() => self.numeric(),
            alphabetic!() => self.alphabetic(),
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
        let buffer = self.take_while(|c| matches!(c, numeric!() | '_'));
        let buffer = buffer.replace('_', "");
        let value = buffer.parse().unwrap();
        TokenKind::Integer(value)
    }

    fn alphabetic(&mut self) -> TokenKind {
        let buffer = self.take_while(|c| matches!(c, alphabetic!() | numeric!() | '_'));
        match buffer.as_str() {
            "true" => TokenKind::True,
            "false" => TokenKind::False,
            "let" => TokenKind::Let,
            "mutable" => TokenKind::Mutable,
            "if" => TokenKind::If,
            "or" => TokenKind::Or,
            "else" => TokenKind::Else,
            "then" => TokenKind::Then,
            "while" => TokenKind::While,
            "do" => TokenKind::Do,
            "end" => TokenKind::End,
            "debug" => TokenKind::Debug,
            _ => TokenKind::Identifier(buffer),
        }
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
