use crate::{
    rpn::{Grammar, Rpn, RpnItem},
    Expression, Lex, Statement, SyntaxError, Token,
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
            _ => Some(Err(SyntaxError::Grammar("'let'".to_string()))),
        }
    }

    fn let_(&mut self) -> Result<Statement, SyntaxError> {
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

    fn do_(&mut self) -> Result<Statement, SyntaxError> {
        match self.source.next() {
            Some(Token::If) => self.do_if(),
            _ => Err(SyntaxError::Grammar("'if'".to_string())),
        }
    }

    fn do_if(&mut self) -> Result<Statement, SyntaxError> {
        let condition = self.expression()?;
        self.then()?;
        let true_ = self.statement().unwrap()?;
        let false_ = self.do_else_()?;
        self.end()?;
        Ok(Statement::If { condition, true_: Box::new(true_), false_ })
    }

    fn do_else_(&mut self) -> Result<Option<Box<Statement>>, SyntaxError> {
        let token = self.source.peek();
        match token {
            Some(Token::Else) => {
                self.source.next();
                Ok(Some(Box::new(self.statement().unwrap()?)))
            }
            Some(Token::End) => Ok(None),
            _ => Err(SyntaxError::Grammar("'else' or 'end'".to_string())),
        }
    }
}

impl<'p> Parse<'p> {
    pub fn expression(&mut self) -> Result<Expression, SyntaxError> {
        let mut rpn = Rpn::default();
        let mut grammar = Grammar::default();
        while let Some(token) = self.source.peek() {
            grammar.check(token)?;
            match token {
                Token::Integer(_) | Token::Identifier(_) => {
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
                Token::Let | Token::Do | Token::Then | Token::Else | Token::End => {
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
            _ => Err(SyntaxError::Grammar("'then'".to_string())),
        }
    }

    fn else_(&mut self) -> Result<Option<Box<Expression>>, SyntaxError> {
        match self.source.peek() {
            Some(Token::Else) => {
                self.source.next();
                Ok(Some(Box::new(self.expression()?)))
            }
            Some(Token::End) => Ok(None),
            _ => Err(SyntaxError::Grammar("'else' or 'end'".to_string())),
        }
    }

    fn end(&mut self) -> Result<(), SyntaxError> {
        match self.source.next() {
            Some(Token::End) => Ok(()),
            _ => Err(SyntaxError::Grammar("'end'".to_string())),
        }
    }
}
