use crate::{
    end_of_expression,
    rpn::{Grammar, Rpn, RpnItem},
    Expression, Lex, Statement, SyntaxError, Token,
};

use std::iter::Peekable;

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

impl<'p> Parse<'p> {
    pub fn statement(&mut self) -> Result<Statement, SyntaxError> {
        match next!(self.source) {
            Some(Token::Let) => self.let_(),
            Some(Token::Do) => self.do_(),
            Some(Token::While) => self.while_(),
            token => Err(SyntaxError::Grammar {
                expected: "'let', 'do' or 'while'",
                found: token,
            }),
        }
    }

    fn let_(&mut self) -> Result<Statement, SyntaxError> {
        let identifier = self.identifier()?;
        self.equal()?;
        let value = self.expression()?;
        Ok(Statement::Let { identifier, value })
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

    fn do_(&mut self) -> Result<Statement, SyntaxError> {
        match next!(self.source) {
            Some(Token::If) => self.do_if(),
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
        let false_ = self.do_else_()?;
        self.end()?;
        Ok(Statement::If {
            condition,
            true_,
            false_,
        })
    }

    fn do_else_(&mut self) -> Result<Vec<Statement>, SyntaxError> {
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

    fn while_(&mut self) -> Result<Statement, SyntaxError> {
        let condition = self.expression()?;
        self.then()?;
        let body = self.body()?;
        self.end()?;
        Ok(Statement::While { condition, body })
    }
}

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
                    rpn.value(self.if_()?);
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

    fn if_(&mut self) -> Result<Expression, SyntaxError> {
        let condition = self.expression()?;
        self.then()?;
        let true_ = self.expression()?;
        let false_ = self.else_()?;
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

    fn else_(&mut self) -> Result<Option<Box<Expression>>, SyntaxError> {
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
