use std::iter::Peekable;

use super::{
    expression::{self, Expression},
    rpn::{ExpressionState, Rpn, RpnItem},
    statement::{self, Statement},
    BinaryKind, UnaryKind,
};
use crate::{
    lex::{Lex, Token},
    literal, Error, Result,
};

pub type Source<'s> = Peekable<Lex<'s>>;

pub struct Parse<'p> {
    source: Source<'p>,
}

impl<'p> Parse<'p> {
    pub fn new(source: Source<'p>) -> Self {
        Self { source }
    }
}

impl Iterator for Parse<'_> {
    type Item = Result<Statement>;

    fn next(&mut self) -> Option<Self::Item> {
        self.source.peek()?;
        Some(self.statement())
    }
}

impl Parse<'_> {
    fn next(&mut self) -> Result<Token> {
        self.source
            .next()
            .unwrap_or(Err(Error::grammar("statement", None)))
    }

    fn peek(&mut self) -> Result<Option<Token>> {
        self.source.peek().cloned().transpose()
    }

    fn expect(&mut self, token: Token) -> Result<()> {
        let next = self.next()?;
        if next == token {
            Ok(())
        } else {
            Err(Error::grammar(token.to_string(), Some(next)))
        }
    }

    pub fn statement(&mut self) -> Result<Statement> {
        match self.next()? {
            Token::Let => self.let_(),
            Token::Identifier(identifier) => self.assign(identifier),
            Token::If => self.if_s(),
            Token::While => self.while_(),
            Token::For => self.for_(),
            Token::Debug => self.debug(),
            token => Err(Error::grammar("statement", Some(token))),
        }
    }

    fn let_(&mut self) -> Result<Statement> {
        let mutable = self.mutable()?;
        let identifier = self.identifier()?;
        self.expect(Token::Equal)?;
        let value = self.expression()?;
        Ok(Statement::Let(statement::Let {
            identifier,
            mutable,
            value,
        }))
    }

    fn mutable(&mut self) -> Result<bool> {
        match self.peek()? {
            Some(Token::Mutable) => {
                self.next()?;
                Ok(true)
            }
            _ => Ok(false),
        }
    }

    fn identifier(&mut self) -> Result<String> {
        match self.next()? {
            Token::Identifier(identifier) => Ok(identifier),
            token => Err(Error::grammar("identifier", Some(token))),
        }
    }

    fn assign(&mut self, identifier: String) -> Result<Statement> {
        self.expect(Token::Equal)?;
        let value = self.expression()?;
        Ok(Statement::Assign(statement::Assign { identifier, value }))
    }

    fn if_s(&mut self) -> Result<Statement> {
        let condition = self.expression()?;
        self.expect(Token::Then)?;
        let if_ = self.body()?;
        let or = self.or_s()?;
        let else_ = self.else_s()?;
        self.expect(Token::End)?;
        Ok(Statement::If(statement::If {
            condition,
            if_,
            or,
            else_,
        }))
    }

    fn or_s(&mut self) -> Result<Vec<(Expression, Vec<Statement>)>> {
        let mut or = Vec::new();
        while self.peek()?.is_some_and(|token| token == Token::Or) {
            self.next()?;
            let condition = self.expression()?;
            self.expect(Token::Then)?;
            let body = self.body()?;
            or.push((condition, body));
        }
        match self.peek()? {
            Some(Token::Else) | Some(Token::End) => Ok(or),
            token => Err(Error::grammar("'else' or 'end' after 'or' body", token)),
        }
    }

    fn else_s(&mut self) -> Result<Vec<Statement>> {
        match self.peek()? {
            Some(Token::Else) => {
                self.next()?;
                self.body()
            }
            Some(Token::End) => Ok(vec![]),
            token => Err(Error::grammar("'else', 'or' or 'end'", token)),
        }
    }

    fn while_(&mut self) -> Result<Statement> {
        let condition = self.expression()?;
        self.expect(Token::Do)?;
        let body = self.body()?;
        self.expect(Token::End)?;
        Ok(Statement::While(statement::While { condition, body }))
    }

    fn for_(&mut self) -> Result<Statement> {
        let item = self.identifier()?;
        self.expect(Token::In)?;
        let sequence = self.expression()?;
        self.expect(Token::Do)?;
        let body = self.body()?;
        self.expect(Token::End)?;
        Ok(Statement::For(statement::For {
            item,
            sequence,
            body,
        }))
    }

    fn debug(&mut self) -> Result<Statement> {
        let value = self.expression()?;
        Ok(Statement::Debug(statement::Debug { value }))
    }
}

impl Parse<'_> {
    pub fn expression(&mut self) -> Result<Expression> {
        let mut rpn = Rpn::new();
        let mut status = ExpressionState::default();
        while let Some(token) = self.peek()? {
            if status.stop(&token)? {
                break;
            }
            match token {
                literal!() => {
                    rpn.value(Expression::from(token));
                    self.next()?;
                }
                token if token.is_binary() => {
                    let binary: Option<BinaryKind> = token.into();
                    rpn.binary(binary.unwrap());
                    self.next()?;
                }
                token if token.is_unary() => {
                    let unary: Option<UnaryKind> = token.into();
                    rpn.unary(unary.unwrap());
                    self.next()?;
                }
                Token::LeftParenthesis => {
                    rpn.item(RpnItem::Parenthesis);
                    self.next()?;
                }
                Token::RightParenthesis => {
                    self.next()?;
                    rpn.parenthesis();
                }
                Token::If => {
                    self.next()?;
                    rpn.value(self.if_e()?);
                }
                _ => {
                    return Err(Error::grammar("expression", Some(token)));
                }
            }
        }
        Ok(rpn.collapse())
    }

    fn if_e(&mut self) -> Result<Expression> {
        let condition = self.expression()?;
        self.expect(Token::Then)?;
        let true_ = self.expression()?;
        let or = self.or_e()?;
        let else_ = self.else_e()?;
        self.expect(Token::End)?;
        Ok(Expression::If(expression::If {
            condition: Box::new(condition),
            if_: Box::new(true_),
            or,
            else_,
        }))
    }

    fn or_e(&mut self) -> Result<Vec<(Expression, Expression)>> {
        let mut or = Vec::new();
        while self.peek()?.is_some_and(|token| token == Token::Or) {
            self.next()?;
            let condition = self.expression()?;
            self.expect(Token::Then)?;
            let body = self.expression()?;
            or.push((condition, body));
        }
        match self.peek()? {
            Some(Token::Else) | Some(Token::End) => Ok(or),
            token => Err(Error::grammar("'else' or 'end' after 'or' body", token)),
        }
    }

    fn else_e(&mut self) -> Result<Option<Box<Expression>>> {
        match self.peek()? {
            Some(Token::Else) => {
                self.next()?;
                Ok(Some(Box::new(self.expression()?)))
            }
            Some(Token::End) => Ok(None),
            token => Err(Error::grammar(
                "'else', 'or' or 'end' after 'if' body",
                token,
            )),
        }
    }
}

impl Parse<'_> {
    fn body(&mut self) -> Result<Vec<Statement>> {
        let mut body = Vec::new();
        while let Some(token) = self.peek()? {
            match token {
                Token::Or | Token::Else | Token::End => break,
                _ => body.push(self.statement()?),
            }
        }
        Ok(body)
    }
}
