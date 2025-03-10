use std::{iter::Peekable, str::Chars};

use crate::{
    alphabetic,
    error::Result,
    numeric, quote, single, skip,
    token::{BinaryOperator, Token, UnaryOperator},
    Error,
};

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
        match self.source.peek()? {
            numeric!() => Some(Ok(self.numeric())),
            alphabetic!() => Some(Ok(self.alphabetic())),
            quote!() => Some(Ok(self.string())),
            skip!() => self.skip(),
            single!() => Some(Ok(self.single())),
            c => Some(Err(Error::InvalidCharacter(*c))),
        }
    }
}

impl Lex<'_> {
    fn numeric(&mut self) -> Token {
        let buffer = self.take_while(|c| matches!(c, numeric!()));
        Token::Integer(buffer.parse().unwrap())
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

    fn alphabetic(&mut self) -> Token {
        let buffer = self.take_while(|c| matches!(c, alphabetic!() | numeric!()));
        Self::keyword(&buffer).unwrap_or(Token::Identifier(buffer))
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

    fn single(&mut self) -> Token {
        match self.source.next().unwrap() {
            '+' => Token::BinaryOperator(BinaryOperator::Add),
            '-' => {
                if self.source.peek().is_some_and(|c| matches!(c, skip!())) {
                    Token::BinaryOperator(BinaryOperator::Subtract)
                } else {
                    Token::UnaryOperator(UnaryOperator::Negate)
                }
            }
            '*' => Token::BinaryOperator(BinaryOperator::Multiply),
            '/' => Token::BinaryOperator(BinaryOperator::Divide),
            '>' => Token::BinaryOperator(BinaryOperator::Greater),
            '<' => Token::BinaryOperator(BinaryOperator::Less),
            '?' => Token::BinaryOperator(BinaryOperator::Equal),
            '(' => Token::LeftParenthesis,
            ')' => Token::RightParenthesis,
            '=' => Token::Equal,
            '!' => Token::UnaryOperator(UnaryOperator::Not),
            _ => unreachable!(),
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
