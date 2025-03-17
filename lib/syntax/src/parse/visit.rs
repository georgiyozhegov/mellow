use super::{BinaryKind, Expression, Statement, UnaryKind};


#[allow(unused)]
pub trait VisitStatement {
    type Output;

    fn let_(&mut self, identifier: &String, mutable: &bool, value: &Expression) -> Self::Output {
        todo!()
    }
    fn assign(&mut self, identifier: &String, value: &Expression) -> Self::Output {
        todo!()
    }
    fn if_(
        &mut self,
        condition: &Expression,
        if_: &Vec<Statement>,
        or: &Vec<(Expression, Vec<Statement>)>,
        else_: &Vec<Statement>,
    ) -> Self::Output {
        todo!()
    }
    fn while_(&mut self, condition: &Expression, body: &Vec<Statement>) -> Self::Output {
        todo!()
    }
    fn for_(
        &mut self,
        item: &String,
        sequence: &Expression,
        body: &Vec<Statement>,
    ) -> Self::Output {
        todo!()
    }
    fn debug(&mut self, value: &Expression) -> Self::Output {
        todo!()
    }
}

impl Statement {
    pub fn visit<T: VisitStatement>(&self, visit: &mut T) -> T::Output {
        match self {
            Self::Let {
                identifier,
                mutable,
                value,
            } => visit.let_(identifier, mutable, value),
            Self::Assign { identifier, value } => visit.assign(identifier, value),
            Self::If {
                condition,
                if_,
                or,
                else_,
            } => visit.if_(condition, if_, or, else_),
            Self::While { condition, body } => visit.while_(condition, body),
            Self::For {
                item,
                sequence,
                body,
            } => visit.for_(item, sequence, body),
            Self::Debug(value) => visit.debug(value),
        }
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
