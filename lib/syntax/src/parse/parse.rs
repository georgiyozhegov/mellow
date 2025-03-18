use std::iter::Peekable;

use super::{
    expression,
    rpn::{ExpressionState, Rpn, RpnItem},
    statement, BinaryKind, Expression, Statement, UnaryKind,
};
use crate::{
    lex::{Lex, Token},
    literal, Error, Result,
};

pub type Source<'s> = Peekable<Lex<'s>>;

pub struct Parser<'p> {
    source: Source<'p>,
}

impl<'p> Parser<'p> {
    pub fn new(source: Source<'p>) -> Self {
        Self { source }
    }
}

impl Iterator for Parser<'_> {
    type Item = Result<Statement>;

    fn next(&mut self) -> Option<Self::Item> {
        self.source.peek()?;
        Some(Statement::parse(self))
    }
}

impl Parser<'_> {
    pub fn next(&mut self) -> Result<Token> {
        self.source
            .next()
            .unwrap_or(Err(Error::grammar("statement", None)))
    }

    pub fn peek(&mut self) -> Result<Option<Token>> {
        self.source.peek().cloned().transpose()
    }

    pub fn expect(&mut self, token: Token) -> Result<()> {
        let next = self.next()?;
        if next == token {
            Ok(())
        } else {
            Err(Error::grammar(token.to_string(), Some(next)))
        }
    }

    pub fn mutable(&mut self) -> Result<bool> {
        match self.peek()? {
            Some(Token::Mutable) => {
                self.next()?;
                Ok(true)
            }
            _ => Ok(false),
        }
    }

    pub fn identifier(&mut self) -> Result<String> {
        match self.next()? {
            Token::Identifier(identifier) => Ok(identifier),
            token => Err(Error::grammar("identifier", Some(token))),
        }
    }

    pub fn body(&mut self) -> Result<Vec<Statement>> {
        let mut body = Vec::new();
        while let Some(token) = self.peek()? {
            match token {
                Token::Or | Token::Else | Token::End => break,
                _ => body.push(Statement::parse(self)?),
            }
        }
        Ok(body)
    }
}

impl Statement {
    fn parse(parser: &mut Parser) -> Result<Statement> {
        match parser.peek()? {
            Some(Token::Let) => statement::Let::parse(parser),
            Some(Token::Identifier(..)) => statement::Assign::parse(parser),
            Some(Token::If) => statement::If::parse(parser),
            Some(Token::While) => statement::While::parse(parser),
            Some(Token::For) => statement::For::parse(parser),
            Some(Token::Debug) => statement::Debug::parse(parser),
            token => Err(Error::grammar("statement", token)),
        }
    }
}

impl statement::Let {
    fn parse(parser: &mut Parser) -> Result<Statement> {
        parser.expect(Token::Let)?;
        let mutable = parser.mutable()?;
        let identifier = parser.identifier()?;
        parser.expect(Token::Equal)?;
        let value = Expression::parse(parser)?;
        Ok(Statement::Let(statement::Let {
            identifier,
            mutable,
            value,
        }))
    }
}

impl statement::Assign {
    fn parse(parser: &mut Parser) -> Result<Statement> {
        let identifier = parser.identifier()?;
        parser.expect(Token::Equal)?;
        let value = Expression::parse(parser)?;
        Ok(Statement::Assign(statement::Assign { identifier, value }))
    }
}

impl statement::If {
    fn parse(parser: &mut Parser) -> Result<Statement> {
        parser.expect(Token::If)?;
        let condition = Expression::parse(parser)?;
        parser.expect(Token::Then)?;
        let if_ = parser.body()?;
        let or = Self::or(parser)?;
        let else_ = Self::else_(parser)?;
        parser.expect(Token::End)?;
        Ok(Statement::If(statement::If {
            condition,
            if_,
            or,
            else_,
        }))
    }

    fn or(parser: &mut Parser) -> Result<Vec<(Expression, Vec<Statement>)>> {
        let mut or = Vec::new();
        while parser.peek()?.is_some_and(|token| token == Token::Or) {
            parser.next()?;
            let condition = Expression::parse(parser)?;
            parser.expect(Token::Then)?;
            let body = parser.body()?;
            or.push((condition, body));
        }
        match parser.peek()? {
            Some(Token::Else) | Some(Token::End) => Ok(or),
            token => Err(Error::grammar("'else' or 'end' after 'or' body", token)),
        }
    }

    fn else_(parser: &mut Parser) -> Result<Vec<Statement>> {
        match parser.peek()? {
            Some(Token::Else) => {
                parser.next()?;
                parser.body()
            }
            Some(Token::End) => Ok(vec![]),
            token => Err(Error::grammar("'else', 'or' or 'end'", token)),
        }
    }
}

impl statement::While {
    fn parse(parser: &mut Parser) -> Result<Statement> {
        parser.expect(Token::While)?;
        let condition = Expression::parse(parser)?;
        parser.expect(Token::Do)?;
        let body = parser.body()?;
        parser.expect(Token::End)?;
        Ok(Statement::While(statement::While { condition, body }))
    }
}

impl statement::For {
    fn parse(parser: &mut Parser) -> Result<Statement> {
        parser.expect(Token::For)?;
        let item = parser.identifier()?;
        parser.expect(Token::In)?;
        let sequence = Expression::parse(parser)?;
        parser.expect(Token::Do)?;
        let body = parser.body()?;
        parser.expect(Token::End)?;
        Ok(Statement::For(statement::For {
            item,
            sequence,
            body,
        }))
    }
}

impl statement::Debug {
    fn parse(parser: &mut Parser) -> Result<Statement> {
        parser.expect(Token::Debug)?;
        let value = Expression::parse(parser)?;
        Ok(Statement::Debug(statement::Debug { value }))
    }
}

pub trait ParseExpression {
    fn parse(parser: &mut Parser) -> Result<Expression>;
}

impl ParseExpression for Expression {
    fn parse(parser: &mut Parser) -> Result<Expression> {
        let mut rpn = Rpn::new();
        let mut status = ExpressionState::default();
        while let Some(token) = parser.peek()? {
            if status.stop(&token)? {
                break;
            }
            match token {
                literal!() => {
                    rpn.value(Expression::from(token));
                    parser.next()?;
                }
                token if token.is_binary() => {
                    let binary: Option<BinaryKind> = token.into();
                    rpn.binary(binary.unwrap());
                    parser.next()?;
                }
                token if token.is_unary() => {
                    let unary: Option<UnaryKind> = token.into();
                    rpn.unary(unary.unwrap());
                    parser.next()?;
                }
                Token::LeftParenthesis => {
                    rpn.item(RpnItem::Parenthesis);
                    parser.next()?;
                }
                Token::RightParenthesis => {
                    parser.next()?;
                    rpn.parenthesis();
                }
                Token::If => {
                    parser.next()?;
                    rpn.value(expression::If::parse(parser)?);
                }
                _ => {
                    return Err(Error::grammar("expression", Some(token)));
                }
            }
        }
        Ok(rpn.collapse())
    }
}

impl expression::If {
    fn parse(parser: &mut Parser) -> Result<Expression> {
        let condition = Expression::parse(parser)?;
        parser.expect(Token::Then)?;
        let true_ = Expression::parse(parser)?;
        let or = Self::or(parser)?;
        let else_ = Self::else_(parser)?;
        parser.expect(Token::End)?;
        Ok(Expression::If(expression::If {
            condition: Box::new(condition),
            if_: Box::new(true_),
            or,
            else_,
        }))
    }

    fn or(parser: &mut Parser) -> Result<Vec<(Expression, Expression)>> {
        let mut or = Vec::new();
        while parser.peek()?.is_some_and(|token| token == Token::Or) {
            parser.next()?;
            let condition = Expression::parse(parser)?;
            parser.expect(Token::Then)?;
            let body = Expression::parse(parser)?;
            or.push((condition, body));
        }
        match parser.peek()? {
            Some(Token::Else) | Some(Token::End) => Ok(or),
            token => Err(Error::grammar("'else' or 'end' after 'or' body", token)),
        }
    }

    fn else_(parser: &mut Parser) -> Result<Option<Box<Expression>>> {
        match parser.peek()? {
            Some(Token::Else) => {
                parser.next()?;
                Ok(Some(Box::new(Expression::parse(parser)?)))
            }
            Some(Token::End) => Ok(None),
            token => Err(Error::grammar(
                "'else', 'or' or 'end' after 'if' body",
                token,
            )),
        }
    }
}
