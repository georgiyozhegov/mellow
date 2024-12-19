use std::{env::Args, iter::Peekable, str::SplitWhitespace};

use crate::{
    literal,
    rpn::{ExpressionState, Rpn, RpnItem},
    token::Token,
    tree::Expression, Statement},
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

macro_rules! except {
    ($source: expr, $token: pat, $expected: expr) => {
        match next!($source) {
            Some($token) => Ok(()),
            token => Err(SyntaxError::Grammar {
                expected: $expected,
                found: token,
            }),
        }
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
            Some(Token::Identifier(identifier)) => self.assign(identifier),
            Some(Token::If) => self.if_s(),
            Some(Token::While) => self.while_(),
            Some(Token::For) => self.for_(),
            token => Err(SyntaxError::Grammar {
                expected: "statement",
                found: token,
            }),
        }
    }

    // LET
    fn let_(&mut self) -> Result<Statement, SyntaxError> {
        let mutable = self.mutable()?;
        let identifier = self.identifier()?;
        except!(self.source, Token::Equal, "'='")?;
        let value = self.expression()?;
        Ok(Statement::Let {
            identifier,
            mutable,
            value,
        })
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
    
    fn assign(&mut self, identifier: String) -> Result<Statement, SyntaxError> {
        except!(self.source, Token::Equal, "'='")?;
        let value = self.expression()?;
        Ok(Statement::Assign { identifier, value })
    }

    fn if_s(&mut self) -> Result<Statement, SyntaxError> {
        let condition = self.expression()?;
        except!(self.source, Token::Then, "'then'")?;
        let if_ = self.body()?;
        let or = self.or_s()?;
        let else_ = self.else_s()?;
        except!(self.source, Token::End, "'end'")?;
        Ok(Statement::If {
            condition,
            if_,
            or,
            else_,
        })
    }

    fn or_s(&mut self) -> Result<Vec<(Expression, Vec<Statement>)>, SyntaxError> {
        let mut or = Vec::new();
        while peek!(self.source).is_some_and(|token| *token == Token::Or) {
            self.source.next();
            let condition = self.expression()?;
            except!(self.source, Token::Then, "'then'")?;
            let body = self.body()?;
            or.push((condition, body));
        }
        match peek!(self.source) {
            Some(Token::Else) | Some(Token::End) => Ok(or),
            token => Err(SyntaxError::Grammar {
                expected: "'else' or 'end' after 'or' body",
                found: token.cloned(),
            }),
        }
    }

    fn else_s(&mut self) -> Result<Vec<Statement>, SyntaxError> {
        match peek!(self.source) {
            Some(Token::Else) => {
                self.source.next();
                self.body()
            }
            Some(Token::End) => Ok(vec![]),
            token => Err(SyntaxError::Grammar {
                expected: "'else', 'or' or 'end'",
                found: token.cloned(),
            }),
        }
    }

    fn while_(&mut self) -> Result<Statement, SyntaxError> {
        let condition = self.expression()?;
        except!(self.source, Token::Do, "'do'")?;
        let body = self.body()?;
        except!(self.source, Token::End, "'end'")?;
        Ok(Statement::While { condition, body })
    }

    fn for_(&mut self) -> Result<Statement, SyntaxError> {
        let item = self.identifier()?;
        let sequence = self.expression()?;
        except!(self.source, Token::Then, "'then'")?;
        let body = self.body()?;
        except!(self.source, Token::End, "'end'")?;
        Ok(Statement::For {
            item,
            sequence,
            body,
        })
    }
}

impl<'p> Parse<'p> {
    pub fn expression(&mut self) -> Result<Expression, SyntaxError> {
        let mut rpn = Rpn::default();
        let mut status = ExpressionState::default();
        while let Some(token) = peek!(self.source) {
            if status.stop(token)? {
                break;
            }
            match token {
                literal!() => {
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
                    rpn.value(self.if_e()?);
                }
                _ => {
                    return Err(SyntaxError::Grammar {
                        expected: "expression",
                        found: Some(token.clone()),
                    })
                }
            }
        }
        Ok(rpn.collapse())
    }

    fn if_e(&mut self) -> Result<Expression, SyntaxError> {
        let condition = self.expression()?;
        except!(self.source, Token::Then, "'then'")?;
        let true_ = self.expression()?;
        let or = self.or_e()?;
        let else_ = self.else_e()?;
        except!(self.source, Token::End, "'end'")?;
        Ok(Expression::If {
            condition: Box::new(condition),
            if_: Box::new(true_),
            or,
            else_,
        })
    }

    fn or_e(&mut self) -> Result<Vec<(Expression, Expression)>, SyntaxError> {
        let mut or = Vec::new();
        while peek!(self.source).is_some_and(|token| *token == Token::Or) {
            self.source.next();
            let condition = self.expression()?;
            except!(self.source, Token::Then, "'then'")?;
            let body = self.expression()?;
            or.push((condition, body));
        }
        match peek!(self.source) {
            Some(Token::Else) | Some(Token::End) => Ok(or),
            token => Err(SyntaxError::Grammar {
                expected: "'else' or 'end' after 'or' body",
                found: token.cloned(),
            }),
        }
    }

    fn else_e(&mut self) -> Result<Option<Box<Expression>>, SyntaxError> {
        match peek!(self.source) {
            Some(Token::Else) => {
                self.source.next();
                Ok(Some(Box::new(self.expression()?)))
            }
            Some(Token::End) => Ok(None),
            token => Err(SyntaxError::Grammar {
                expected: "'else', 'or' or 'end' after 'if' body",
                found: token.cloned(),
            }),
        }
    }
}

impl<'p> Parse<'p> {
    fn body(&mut self) -> Result<Vec<Statement>, SyntaxError> {
        let mut body = Vec::new();
        while let Some(token) = peek!(self.source) {
            match token {
                Token::Or | Token::Else | Token::End => break,
                _ => body.push(self.statement()?),
            }
        }
        Ok(body)
    }
}
