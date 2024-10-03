use crate::{Expression, Lex, Statement, Token};

use std::iter::Peekable;

type Source<'s> = Peekable<Lex<'s>>;

pub struct Parse<'p> {
    source: Source<'p>,
}

impl<'p> Parse<'p> {
    pub fn new(source: Source<'p>) -> Self {
        Self { source }
    }
}

impl<'p> Iterator for Parse<'p> {
    type Item = Statement;

    fn next(&mut self) -> Option<Self::Item> {
        self.statement()
    }
}

impl<'p> Parse<'p> {
    pub fn statement(&mut self) -> Option<Statement> {
        let token = self.source.peek()?;
        match token {
            Token::Let => Some(self.let_()),
            _ => todo!(),
        }
    }

    fn let_(&mut self) -> Statement {
        self.source.next();
        let identifier = self.identifier();
        self.equal();
        let value = self.expression();
        Statement::Let { identifier, value }
    }

    fn identifier(&mut self) -> String {
        match self.source.next() {
            Some(Token::Identifier(identifier)) => identifier,
            _ => todo!("Error handling"),
        }
    }

    fn equal(&mut self) {
        match self.source.next() {
            Some(Token::Equal) => {},
            _ => todo!("Error handling"),
        }
    }
}

impl<'p> Parse<'p> {
    pub fn expression(&mut self) -> Expression {
        match self.source.next() {
            Some(Token::Integer(integer)) => Expression::Integer(integer),
            _ => todo!("Error handling"),
        }
    }
}
