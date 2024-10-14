use crate::{
    end_of_expression,
    rpn::{Grammar, Rpn, RpnItem},
    token, Expression, Lex, Statement, SyntaxError, Token,
};

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
        let token = self.source.next()?;
        match token {
            Token::Let => Some(self.let_()),
            Token::Do => Some(self.do_()),
            Token::While => Some(self.while_()),
            _ => Some(Err(SyntaxError::Grammar {
                expected: "'let', 'do' or 'while'",
                found: Some(token),
            })),
        }
    }

    fn let_(&mut self) -> Result<Statement, SyntaxError> {
        let identifier = self.identifier()?;
        self.equal()?;
        let value = self.expression()?;
        Ok(Statement::Let { identifier, value })
    }

    fn identifier(&mut self) -> Result<String, SyntaxError> {
        match self.source.next() {
            Some(Token::Identifier(identifier)) => Ok(identifier),
            token => Err(SyntaxError::Grammar {
                expected: "identifier",
                found: token,
            }),
        }
    }

    fn equal(&mut self) -> Result<(), SyntaxError> {
        match self.source.next() {
            Some(Token::Equal) => Ok(()),
            token => Err(SyntaxError::Grammar {
                expected: "'='",
                found: token,
            }),
        }
    }

    fn do_(&mut self) -> Result<Statement, SyntaxError> {
        match self.source.next() {
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
        let true_ = self.statement().unwrap()?;
        let false_ = self.do_else_()?;
        self.end()?;
        Ok(Statement::If {
            condition,
            true_: Box::new(true_),
            false_: false_.map(|statement| Box::new(statement)),
        })
    }

    fn do_else_(&mut self) -> Result<Option<Statement>, SyntaxError> {
        match self.source.peek() {
            Some(Token::Else) => {
                self.source.next();
                Ok(Some(self.statement().unwrap()?))
            }
            Some(Token::End) => Ok(None),
            token => Err(SyntaxError::Grammar {
                expected: "'else' or 'end'",
                found: token.cloned(),
            }),
        }
    }

    fn while_(&mut self) -> Result<Statement, SyntaxError> {
        let condition = self.expression()?;
        self.then()?;
        let body = self.statement().unwrap()?;
        self.end()?;
        Ok(Statement::While {
            condition,
            body: Box::new(body),
        })
    }
}

impl<'p> Parse<'p> {
    pub fn expression(&mut self) -> Result<Expression, SyntaxError> {
        let mut rpn = Rpn::default();
        let mut grammar = Grammar::default();
        while let Some(token) = self.source.peek() {
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
                _ => todo!(),
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
        match self.source.next() {
            Some(Token::Then) => Ok(()),
            token => Err(SyntaxError::Grammar {
                expected: "'then'",
                found: token,
            }),
        }
    }

    fn else_(&mut self) -> Result<Option<Box<Expression>>, SyntaxError> {
        match self.source.peek() {
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
        match self.source.next() {
            Some(Token::End) => Ok(()),
            token => Err(SyntaxError::Grammar {
                expected: "'end'",
                found: token,
            }),
        }
    }
}
