use crate::token::{BinaryOperator, Token, UnaryOperator};
use crate::SyntaxError;
use std::iter::Peekable;
use std::str::Chars;

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
        '+' | '-' | '*' | '/' | '(' | ')' | '='
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
    type Item = Result<Token, SyntaxError>;

    fn next(&mut self) -> Option<Self::Item> {
        self.token()
    }
}

impl<'l> Lex<'l> {
    pub fn token(&mut self) -> Option<Result<Token, SyntaxError>> {
        match self.source.peek()? {
            numeric!() => Some(Ok(self.numeric())),
            alphabetic!() => Some(Ok(self.alphabetic())),
            invisible!() => self.invisible(),
            single!() => Some(Ok(self.single())),
            c => Some(Err(SyntaxError::InvalidCharacter(*c))),
        }
    }

    fn numeric(&mut self) -> Token {
        let buffer = take_until(&mut self.source, |c| matches!(c, numeric!()));
        Token::Integer(buffer.parse().unwrap())
    }

    fn alphabetic(&mut self) -> Token {
        let buffer = take_until(&mut self.source, |c| {
            matches!(c, alphabetic!() | numeric!())
        });
        if let Some(token) = Lex::keyword(&buffer) {
            token
        } else {
            Token::Identifier(buffer)
        }
    }

    fn keyword(buffer: &str) -> Option<Token> {
        match buffer {
            "let" => Some(Token::Let),
            "if" => Some(Token::If),
            "match" => Some(Token::Match),
            "for" => Some(Token::For),
            "while" => Some(Token::While),
            "loop" => Some(Token::Loop),
            "do" => Some(Token::Do),
            "then" => Some(Token::Then),
            "else" => Some(Token::Else),
            "case" => Some(Token::Case),
            "from" => Some(Token::From),
            "to" => Some(Token::To),
            "in" => Some(Token::In),
            "end" => Some(Token::End),
            "true" => Some(Token::True),
            "false" => Some(Token::False),
            _ => None,
        }
    }

    fn invisible(&mut self) -> Option<Result<Token, SyntaxError>> {
        take_until(&mut self.source, |c| matches!(c, invisible!()));
        self.token()
    }

    fn single(&mut self) -> Token {
        match self.source.next().unwrap() {
            '+' => Token::BinaryOperator(BinaryOperator::Add),
            '-' => {
                if self
                    .source
                    .peek()
                    .is_some_and(|c| matches!(c, invisible!()))
                {
                    Token::BinaryOperator(BinaryOperator::Subtract)
                } else {
                    Token::UnaryOperator(UnaryOperator::Negate)
                }
            }
            '*' => Token::BinaryOperator(BinaryOperator::Multiply),
            '/' => Token::BinaryOperator(BinaryOperator::Divide),
            '(' => Token::LeftParenthesis,
            ')' => Token::RightParenthesis,
            '=' => Token::Equal,
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
