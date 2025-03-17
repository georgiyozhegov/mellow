use super::{tree::*, BinaryKind, Expression, Statement, UnaryKind};

#[allow(unused)]
pub trait VisitStatement {
    type Output;
    type Context;

    fn let_(&mut self, node: Let, context: &mut Self::Context) -> Self::Output {
        todo!()
    }
    fn assign(&mut self, node: Assign, context: &mut Self::Context) -> Self::Output {
        todo!()
    }
    fn if_(&mut self, node: If, context: &mut Self::Context) -> Self::Output {
        todo!()
    }
    fn while_(&mut self, node: While, context: &mut Self::Context) -> Self::Output {
        todo!()
    }
    fn for_(&mut self, node: For, context: &mut Self::Context) -> Self::Output {
        todo!()
    }
    fn debug(&mut self, node: Debug, context: &mut Self::Context) -> Self::Output {
        todo!()
    }
}

#[allow(unused)]
pub trait VisitExpression {
    type Output;

    fn integer(&mut self, value: &i128) -> Self::Output {
        todo!()
    }
    fn identifier(&mut self, name: &String) -> Self::Output {
        todo!()
    }
    fn boolean(&mut self, value: &bool) -> Self::Output {
        todo!()
    }
    fn string(&mut self, value: &String) -> Self::Output {
        todo!()
    }
    fn binary(
        &mut self,
        kind: &BinaryKind,
        left: &Box<Expression>,
        right: &Box<Expression>,
    ) -> Self::Output {
        todo!()
    }
    fn unary(&mut self, kind: &UnaryKind, value: &Box<Expression>) -> Self::Output {
        todo!()
    }
    fn if_(
        &mut self,
        condition: &Expression,
        if_: &Box<Expression>,
        or: &Vec<(Expression, Expression)>,
        else_: &Option<Box<Expression>>,
    ) -> Self::Output {
        todo!()
    }
}

impl Statement {
    pub fn visit<T: VisitStatement>(self, visit: &mut T, context: &mut T::Context) -> T::Output {
        match self {
            Self::Let(node) => visit.let_(node, context),
            Self::Assign(node) => visit.assign(node, context),
            Self::If(node) => visit.if_(node, context),
            Self::While(node) => visit.while_(node, context),
            Self::For(node) => visit.for_(node, context),
            Self::Debug(node) => visit.debug(node, context),
        }
    }
}

impl Expression {
    pub fn visit<T: VisitExpression>(&self, visit: &mut T) -> T::Output {
        match self {
            Self::Integer(value) => visit.integer(value),
            Self::Identifier(name) => visit.identifier(name),
            Self::Boolean(value) => visit.boolean(value),
            Self::String(value) => visit.string(value),
            Self::Binary(kind, left, right) => visit.binary(kind, left, right),
            Self::Unary(kind, value) => visit.unary(kind, value),
            Self::If {
                condition,
                if_,
                or,
                else_,
            } => visit.if_(condition, if_, or, else_),
        }
    }
}
