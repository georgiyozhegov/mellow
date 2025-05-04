use std::vec::IntoIter;

use mellow_error::{Error, Result};
use mellow_lex::TokenKind;

use crate::{Expression, Parsable, Parse};

#[derive(Debug, Clone)]
pub enum Statement {
    Let(Let),
    Assign(Assign),
    If(If<Body>),
    While(While),
    Debug(Debug),
}

impl Parsable for Statement {
    fn parse(source: &mut crate::Parse) -> Result<Self>
    where
        Self: Sized,
    {
        match source.peek()?.and_then(|token| Some(token.take_kind())) {
            Some(TokenKind::Let) => Ok(Self::Let(Let::parse(source)?)),
            Some(TokenKind::Identifier(..)) => Ok(Self::Assign(Assign::parse(source)?)),
            Some(TokenKind::If) => Ok(Self::If(If::<Body>::parse(source)?)),
            Some(TokenKind::While) => Ok(Self::While(While::parse(source)?)),
            Some(TokenKind::Debug) => Ok(Self::Debug(Debug::parse(source)?)),
            Some(_) => Err(Error::expected_but_got("statement", "todo")),
            _ => Err(Error::expected_but_got("statement", "EOF")),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Let {
    pub identifier: Identifier,
    pub mutable: bool,
    pub value: Expression,
}

impl Let {
    pub fn new(identifier: Identifier, mutable: bool, value: Expression) -> Self {
        Self {
            identifier,
            mutable,
            value,
        }
    }
}

impl Parsable for Let {
    fn parse(source: &mut crate::Parse) -> Result<Self>
    where
        Self: Sized,
    {
        source.expect(TokenKind::Let)?;
        let mutable = source.mutable()?;
        let identifier = Identifier::parse(source)?;
        source.expect(TokenKind::Equal)?;
        let value = Expression::parse(source)?;
        Ok(Self::new(identifier, mutable, value))
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Identifier {
    pub name: String,
}

impl Identifier {
    pub fn new(name: String) -> Self {
        Self { name }
    }
}

impl Parsable for Identifier {
    fn parse(source: &mut crate::Parse) -> Result<Self>
    where
        Self: Sized,
    {
        match source.next()?.take_kind() {
            TokenKind::Identifier(name) => Ok(Identifier::new(name)),
            _ => Err(Error::expected_but_got("identifier", "todo")),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Assign {
    pub identifier: Identifier,
    pub value: Expression,
}

impl Assign {
    pub fn new(identifier: Identifier, value: Expression) -> Self {
        Self { identifier, value }
    }
}

impl Parsable for Assign {
    fn parse(source: &mut Parse) -> Result<Self>
    where
        Self: Sized,
    {
        let identifier = Identifier::parse(source)?;
        source.expect(TokenKind::Equal)?;
        let value = Expression::parse(source)?;
        Ok(Assign::new(identifier, value))
    }
}

#[derive(Debug, Clone)]
pub struct If<B>
where
    B: Parsable,
{
    pub if_: Branch<B>,
    pub or: Vec<Branch<B>>,
    pub else_: Option<Box<B>>,
}

impl<B> If<B>
where
    B: Parsable,
{
    pub fn parse(source: &mut Parse) -> Result<Self> {
        source.expect(TokenKind::If)?;
        let if_ = Branch::parse(source)?;
        let or = Self::or(source)?;
        let else_ = Self::else_(source)?;
        source.expect(TokenKind::End)?;
        Ok(Self { if_, or, else_ })
    }

    fn or(source: &mut Parse) -> Result<Vec<Branch<B>>> {
        let mut or = Vec::new();
        while source
            .peek()?
            .is_some_and(|token| token.kind() == &TokenKind::Or)
        {
            source.next()?;
            let branch = Branch::<B>::parse(source)?;
            or.push(branch);
        }
        match source.peek()?.and_then(|token| Some(token.take_kind())) {
            Some(TokenKind::Else) | Some(TokenKind::End) => Ok(or),
            Some(_) => Err(Error::expected_but_got(
                "'else' or 'end' after 'or' body",
                "todo",
            )),
            _ => Err(Error::expected_but_got(
                "'else' or 'end' after 'or' body",
                "EOF",
            )),
        }
    }

    fn else_(source: &mut Parse) -> Result<Option<Box<B>>> {
        match source.peek()?.and_then(|token| Some(token.take_kind())) {
            Some(TokenKind::Else) => {
                source.next()?;
                Ok(Some(Box::new(B::parse(source)?)))
            }
            Some(TokenKind::End) => Ok(None),
            Some(_) => Err(Error::expected_but_got("'else', 'or' or 'end'", "todo")),
            _ => Err(Error::expected_but_got("'else', 'or' or 'end'", "EOF")),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Branch<B>
where
    B: Parsable,
{
    pub condition: Box<Expression>,
    pub body: Box<B>,
}

impl<B> Branch<B>
where
    B: Parsable,
{
    pub fn new(condition: Expression, body: B) -> Self {
        Self {
            condition: Box::new(condition),
            body: Box::new(body),
        }
    }
}

impl<B> Parsable for Branch<B>
where
    B: Parsable,
{
    fn parse(source: &mut Parse) -> Result<Self>
    where
        Self: Sized,
    {
        let condition = Expression::parse(source)?;
        source.expect(TokenKind::Then)?;
        let body = B::parse(source)?;
        Ok(Self::new(condition, body))
    }
}

#[derive(Debug, Clone)]
pub struct While {
    pub condition: Expression,
    pub body: Body,
}

impl While {
    pub fn new(condition: Expression, body: Body) -> Self {
        Self { condition, body }
    }
}

impl Parsable for While {
    fn parse(source: &mut Parse) -> Result<Self>
    where
        Self: Sized,
    {
        source.expect(TokenKind::While)?;
        let condition = Expression::parse(source)?;
        source.expect(TokenKind::Do)?;
        let body = Body::parse(source)?;
        source.expect(TokenKind::End)?;
        Ok(Self::new(condition, body))
    }
}

#[derive(Debug, Clone)]
pub struct Body {
    inner: Vec<Statement>,
}

impl Body {
    pub fn new(inner: Vec<Statement>) -> Self {
        Self { inner }
    }
}

impl Body {
    pub fn empty() -> Self {
        Self { inner: Vec::new() }
    }
}

impl Default for Body {
    fn default() -> Self {
        Self::empty()
    }
}

impl Parsable for Body {
    fn parse(source: &mut Parse) -> Result<Self>
    where
        Self: Sized,
    {
        let mut inner = Vec::new();
        while let Some(token) = source.peek()? {
            match token.take_kind() {
                TokenKind::Or | TokenKind::Else | TokenKind::End => break,
                _ => {
                    let statement = Statement::parse(source)?;
                    inner.push(statement);
                }
            }
        }
        Ok(Self::new(inner))
    }
}

impl IntoIterator for Body {
    type Item = Statement;
    type IntoIter = IntoIter<Statement>;

    fn into_iter(self) -> Self::IntoIter {
        self.inner.into_iter()
    }
}

#[derive(Debug, Clone)]
pub struct Debug {
    pub value: Expression,
}

impl Debug {
    pub fn new(value: Expression) -> Self {
        Self { value }
    }
}

impl Parsable for Debug {
    fn parse(source: &mut Parse) -> Result<Self>
    where
        Self: Sized,
    {
        source.expect(TokenKind::Debug)?;
        let value = Expression::parse(source)?;
        Ok(Self::new(value))
    }
}
