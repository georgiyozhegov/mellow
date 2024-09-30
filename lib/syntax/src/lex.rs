use crate::{BinaryOperator, Token, UnaryOperator};

use std::str::Chars;
use std::iter::Peekable;

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

macro_rules! invisible {
    () => {
        ' ' | '\t' | '\n'
    };
}

macro_rules! single {
    () => {
        '+' | '-' | '*' | '/'
    };
}

type Source<'s> = Peekable<Chars<'s>>;

pub struct Lex<'l> {
    source: Source<'l>,
}

impl<'l> Lex<'l> {
    pub fn new(source: Source<'l>) -> Self {
        Self { source }
    }
}

impl<'l> Iterator for Lex<'l> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        self.token()
    }
}

impl<'l> Lex<'l> {
    pub fn token(&mut self) -> Option<Token> {
        match self.source.peek()? {
            numeric!() => Some(self.numeric()),
            alphabetic!() => Some(self.alphabetic()),
            invisible!() => self.invisible(),
            single!() => Some(self.single()),
            _ => todo!(),
        }
    }

    fn numeric(&mut self) -> Token {
        let buffer = take_until(&mut self.source, |c| matches!(c, numeric!()));
        Token::Integer(buffer.parse().unwrap())
    }

    fn alphabetic(&mut self) -> Token {
        let buffer = take_until(&mut self.source, |c| matches!(c, alphabetic!() | numeric!()));
        Token::Identifier(buffer)
    }

    fn invisible(&mut self) -> Option<Token> {
        take_until(&mut self.source, |c| matches!(c, invisible!()));
        self.token()
    }

    fn single(&mut self) -> Token {
        match self.source.next().unwrap() {
            '+' => Token::BinaryOperator(BinaryOperator::Add),
            '-' => if self.source.peek().is_some_and(|c| matches!(c, invisible!())) {
                Token::BinaryOperator(BinaryOperator::Subtract)
            } else {
                Token::UnaryOperator(UnaryOperator::Negate)
            }
            '*' => Token::BinaryOperator(BinaryOperator::Multiply),
            '/' => Token::BinaryOperator(BinaryOperator::Divide),
            _ => unreachable!(),
        }
    }
}

fn take_until(source: &mut Source, until: fn(char) -> bool) -> String {
    let mut buffer = String::new();
    while let Some(c) = source.peek() {
        if !until(*c) {
            break;
        }
        buffer.push(*c);
        source.next().unwrap();
    }
    buffer
}
