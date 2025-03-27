use std::{iter::Peekable, str::Chars};

use crate::Token;
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

macro_rules! quote {
    () => {
        '"'
    };
}

macro_rules! underscore {
    () => {
        '_'
    };
}

pub type Source<'s> = Peekable<Chars<'s>>;

pub struct Lexer<'l> {
    source: Source<'l>,
}

impl<'l> Lexer<'l> {
    pub fn new(source: Source<'l>) -> Self {
        Self { source }
    }
}

impl Iterator for Lexer<'_> {
    type Item = Result<Token>;

    fn next(&mut self) -> Option<Self::Item> {
        let c = *self.source.peek()?;
        match c {
            numeric!() => Some(Ok(self.numeric())),
            alphabetic!() => Some(Ok(self.alphabetic())),
            quote!() => Some(Ok(self.string())),
            skip!() => self.skip(),
            _ => {
                if let Some(token) = self.single() {
                    Some(Ok(token))
                } else {
                    Some(Err(Error::InvalidCharacter(c)))
                }
            }
        }
    }
}

impl Lexer<'_> {
    fn numeric(&mut self) -> Token {
        let buffer = self.take_while(|c| matches!(c, numeric!() | underscore!()));
        Token::Integer(buffer.parse().unwrap())
    }

    fn alphabetic(&mut self) -> Token {
        let buffer = self.take_while(|c| matches!(c, alphabetic!() | numeric!() | underscore!()));
        Self::keyword(&buffer).unwrap_or(Token::Identifier(buffer))
    }

    fn keyword(buffer: &str) -> Option<Token> {
        match buffer {
            "let" => Some(Token::Let),
            "mutable" => Some(Token::Mutable),
            "if" => Some(Token::If),
            "or" => Some(Token::Or),
            "else" => Some(Token::Else),
            "then" => Some(Token::Then),
            "while" => Some(Token::While),
            "for" => Some(Token::For),
            "in" => Some(Token::In),
            "loop" => Some(Token::Loop),
            "do" => Some(Token::Do),
            "end" => Some(Token::End),
            "true" => Some(Token::True),
            "false" => Some(Token::False),
            "debug" => Some(Token::Debug),
            _ => None,
        }
    }

    fn string(&mut self) -> Token {
        self.source.next();
        let buffer = self.take_while(|c| *c != quote!());
        self.source.next();
        Token::String(buffer)
    }

    fn skip(&mut self) -> Option<Result<Token>> {
        self.take_while(|c| matches!(c, skip!()));
        self.next()
    }

    fn single(&mut self) -> Option<Token> {
        match self.source.next().unwrap() {
            '+' => Some(Token::Plus),
            '-' => Some(Token::Minus),
            '*' => Some(Token::Star),
            '/' => Some(Token::Slash),
            '>' => Some(Token::Greater),
            '<' => Some(Token::Less),
            '?' => Some(Token::Question),
            '(' => Some(Token::LeftParenthesis),
            ')' => Some(Token::RightParenthesis),
            '=' => Some(Token::Equal),
            '!' => Some(Token::Not),
            _ => None,
        }
    }

    fn take_while(&mut self, predicate: fn(&char) -> bool) -> String {
        let mut output = String::new();
        while let Some(c) = self.source.peek().and_then(|c| predicate(c).then_some(c)) {
            output.push(*c);
            self.source.next();
        }
        output
    }
}
