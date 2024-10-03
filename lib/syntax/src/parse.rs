use crate::{Expression, Lex, Statement, SyntaxError, Token};

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
    type Item = Result<Statement, SyntaxError>;

    fn next(&mut self) -> Option<Self::Item> {
        self.statement()
    }
}

impl<'p> Parse<'p> {
    pub fn statement(&mut self) -> Option<Result<Statement, SyntaxError>> {
        let token = self.source.peek()?;
        match token {
            Token::Let => Some(self.let_()),
            _ => Some(Err(SyntaxError::Grammar("let".to_string()))),
        }
    }

    fn let_(&mut self) -> Result<Statement, SyntaxError> {
        self.source.next();
        let identifier = self.identifier()?;
        self.equal()?;
        let value = self.expression()?;
        Ok(Statement::Let { identifier, value })
    }

    fn identifier(&mut self) -> Result<String, SyntaxError> {
        let token = self.source.next();
        match token {
            Some(Token::Identifier(identifier)) => Ok(identifier),
            _ => Err(SyntaxError::Grammar("identifier".to_string())),
        }
    }

    fn equal(&mut self) -> Result<(), SyntaxError> {
        match self.source.next() {
            Some(Token::Equal) => Ok(()),
            _ => Err(SyntaxError::Grammar("'='".to_string())),
        }
    }
}

impl<'p> Parse<'p> {
    pub fn expression(&mut self) -> Result<Expression, SyntaxError> {
        match self.source.next() {
            Some(Token::Integer(integer)) => Ok(Expression::Integer(integer)),
            _ => Err(SyntaxError::Grammar("expression".to_string())),
        }
    }
}
