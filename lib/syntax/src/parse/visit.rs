use super::{Expression, Statement, expression, statement};

#[allow(unused)]
pub trait VisitStatement {
    type Output;
    type Context;

    fn let_(&mut self, node: statement::Let, context: &mut Self::Context) -> Self::Output {
        todo!()
    }
    fn assign(&mut self, node: statement::Assign, context: &mut Self::Context) -> Self::Output {
        todo!()
    }
    fn if_(&mut self, node: statement::If, context: &mut Self::Context) -> Self::Output {
        todo!()
    }
    fn while_(&mut self, node: statement::While, context: &mut Self::Context) -> Self::Output {
        todo!()
    }
    fn for_(&mut self, node: statement::For, context: &mut Self::Context) -> Self::Output {
        todo!()
    }
    fn debug(&mut self, node: statement::Debug, context: &mut Self::Context) -> Self::Output {
        todo!()
    }
}

#[allow(unused)]
pub trait VisitExpression {
    type Output;

    fn integer(&mut self, node: expression::Integer) -> Self::Output {
        todo!()
    }
    fn identifier(&mut self, node: expression::Identifier) -> Self::Output {
        todo!()
    }
    fn boolean(&mut self, node: expression::Boolean) -> Self::Output {
        todo!()
    }
    fn string(&mut self, string: expression::Str) -> Self::Output {
        todo!()
    }
    fn binary(&mut self, node: expression::Binary) -> Self::Output {
        todo!()
    }
    fn unary(&mut self, node: expression::Unary) -> Self::Output {
        todo!()
    }
    fn if_(&mut self, node: expression::If) -> Self::Output {
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
    pub fn visit<T: VisitExpression>(self, visit: &mut T) -> T::Output {
        match self {
            Self::Integer(node) => visit.integer(node),
            Self::Identifier(node) => visit.identifier(node),
            Self::Boolean(node) => visit.boolean(node),
            Self::String(node) => visit.string(node),
            Self::Binary(node) => visit.binary(node),
            Self::Unary(node) => visit.unary(node),
            Self::If(node) => visit.if_(node),
        }
    }
}
