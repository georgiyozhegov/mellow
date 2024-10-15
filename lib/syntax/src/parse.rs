use std::iter::Peekable;

use crate::{
    end_of_expression,
    rpn::{Grammar, Rpn, RpnItem},
    token::Token,
    tree::{Expression, Statement},
    Lex, SyntaxError,
};

type Source<'s> = Peekable<Lex<'s>>;

macro_rules! next {
    ($source: expr) => {
        match $source.next() {
            Some(Ok(token)) => Some(token),
            None => None,
            Some(Err(error)) => return Err(error),
        }
    };
}

macro_rules! peek {
    ($source: expr) => {
        match $source.peek() {
            Some(Ok(token)) => Some(token),
            None => None,
            Some(Err(error)) => return Err(error.clone()),
        }
    };
}

macro_rules! end_of_body {
    () => {
        Token::Else | Token::End
    };
}

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
        self.source.peek()?;
        Some(self.statement())
    }
}

// STATEMENT
impl<'p> Parse<'p> {
    pub fn statement(&mut self) -> Result<Statement, SyntaxError> {
        match next!(self.source) {
            Some(Token::Let) => self.r#let(),
            Some(Token::Do) => self.r#do(),
            Some(Token::While) => self.r#while(),
            Some(Token::For) => self.r#for(),
            token => Err(SyntaxError::Grammar {
                expected: "'let', 'do' or 'while'",
                found: token,
            }),
        }
    }

    fn r#let(&mut self) -> Result<Statement, SyntaxError> {
        let mutable = self.mutable()?;
        let identifier = self.identifier()?;
        self.equal()?;
        let value = self.expression()?;
        Ok(Statement::Let { identifier, mutable, value })
    }

    fn mutable(&mut self) -> Result<bool, SyntaxError> {
        match peek!(self.source) {
            Some(Token::Mutable) => {
                self.source.next();
                Ok(true)
            }
            _ => Ok(false),
        }
    }

    fn identifier(&mut self) -> Result<String, SyntaxError> {
        match next!(self.source) {
            Some(Token::Identifier(identifier)) => Ok(identifier),
            token => Err(SyntaxError::Grammar {
                expected: "identifier",
                found: token,
            }),
        }
    }

    fn equal(&mut self) -> Result<(), SyntaxError> {
        match next!(self.source) {
            Some(Token::Equal) => Ok(()),
            token => Err(SyntaxError::Grammar {
                expected: "'='",
                found: token,
            }),
        }
    }

    fn r#do(&mut self) -> Result<Statement, SyntaxError> {
        match next!(self.source) {
            Some(Token::If) => self.do_if(),
            Some(Token::Identifier(value)) => self.change(value),
            token => Err(SyntaxError::Grammar {
                expected: "'if'",
                found: token,
            }),
        }
    }

    fn do_if(&mut self) -> Result<Statement, SyntaxError> {
        let condition = self.expression()?;
        self.then()?;
        let true_ = self.body()?;
        let false_ = self.do_else()?;
        self.end()?;
        Ok(Statement::If {
            condition,
            true_,
            false_,
        })
    }

    fn do_else(&mut self) -> Result<Vec<Statement>, SyntaxError> {
        match peek!(self.source) {
            Some(Token::Else) => {
                self.source.next();
                self.body()
            }
            Some(Token::End) => Ok(Vec::new()),
            token => Err(SyntaxError::Grammar {
                expected: "'else' or 'end'",
                found: token.cloned(),
            }),
        }
    }

    fn change(&mut self, identifier: String) -> Result<Statement, SyntaxError> {
        self.equal()?;
        let value = self.expression()?;
        Ok(Statement::Change { identifier, value })
    }

    fn r#while(&mut self) -> Result<Statement, SyntaxError> {
        let condition = self.expression()?;
        self.then()?;
        let body = self.body()?;
        self.end()?;
        Ok(Statement::While { condition, body })
    }

    fn r#for(&mut self) -> Result<Statement, SyntaxError> {
        let item = self.identifier()?;
        match next!(self.source) {
            Some(Token::In) => self.for_in(item),
            Some(Token::From) => self.for_from_to(item),
            token => Err(SyntaxError::Grammar {
                expected: "'in' or 'from'",
                found: token,
            }),
        }
    }

    fn for_in(&mut self, item: String) -> Result<Statement, SyntaxError> {
        let sequence = self.expression()?;
        self.then()?;
        let body = self.body()?;
        self.end()?;
        Ok(Statement::ForIn {
            item,
            sequence,
            body,
        })
    }

    fn for_from_to(&mut self, item: String) -> Result<Statement, SyntaxError> {
        let start = self.expression()?;
        self.to()?;
        let end = self.expression()?;
        self.then()?;
        let body = self.body()?;
        self.end()?;
        Ok(Statement::ForFromTo {
            item,
            start,
            end,
            body,
        })
    }

    fn to(&mut self) -> Result<(), SyntaxError> {
        match next!(self.source) {
            Some(Token::To) => Ok(()),
            token => Err(SyntaxError::Grammar {
                expected: "'to'",
                found: token,
            }),
        }
    }
}

// EXPRESSION
impl<'p> Parse<'p> {
    pub fn expression(&mut self) -> Result<Expression, SyntaxError> {
        let mut rpn = Rpn::default();
        let mut grammar = Grammar::default();
        while let Some(token) = peek!(self.source) {
            grammar.check(token)?;
            match token {
                Token::Integer(_) | Token::Identifier(_) | Token::True | Token::False => {
                    rpn.value(Expression::from(token));
                    self.source.next();
                }
                Token::BinaryOperator(_) => {
                    rpn.binary(RpnItem::from(token));
                    self.source.next();
                }
                Token::UnaryOperator(_) => {
                    rpn.unary(RpnItem::from(token));
                    self.source.next();
                }
                Token::LeftParenthesis => {
                    rpn.item(RpnItem::from(token));
                    self.source.next();
                }
                Token::RightParenthesis => {
                    self.source.next();
                    rpn.parenthesis();
                }
                Token::If => {
                    self.source.next();
                    rpn.value(self.r#if()?);
                }
                end_of_expression!() => {
                    break;
                }
                _ => {
                    return Err(SyntaxError::Grammar {
                        expected: "expression ",
                        found: Some(token.clone()),
                    })
                }
            }
        }
        Ok(rpn.collapse())
    }

    fn r#if(&mut self) -> Result<Expression, SyntaxError> {
        let condition = self.expression()?;
        self.then()?;
        let true_ = self.expression()?;
        let false_ = self.r#else()?;
        self.end()?;
        Ok(Expression::If {
            condition: Box::new(condition),
            true_: Box::new(true_),
            false_,
        })
    }

    fn then(&mut self) -> Result<(), SyntaxError> {
        match next!(self.source) {
            Some(Token::Then) => Ok(()),
            token => Err(SyntaxError::Grammar {
                expected: "'then'",
                found: token,
            }),
        }
    }

    fn r#else(&mut self) -> Result<Option<Box<Expression>>, SyntaxError> {
        match peek!(self.source) {
            Some(Token::Else) => {
                self.source.next();
                Ok(Some(Box::new(self.expression()?)))
            }
            Some(Token::End) => Ok(None),
            token => Err(SyntaxError::Grammar {
                expected: "'else' or 'end'",
                found: token.cloned(),
            }),
        }
    }

    fn end(&mut self) -> Result<(), SyntaxError> {
        match next!(self.source) {
            Some(Token::End) => Ok(()),
            token => Err(SyntaxError::Grammar {
                expected: "'end'",
                found: token,
            }),
        }
    }
}

impl<'p> Parse<'p> {
    fn body(&mut self) -> Result<Vec<Statement>, SyntaxError> {
        let mut body = Vec::new();
        while let Some(token) = peek!(self.source) {
            match token {
                end_of_body!() => break,
                _ => body.push(self.statement()?),
            }
        }
        Ok(body)
    }
}
