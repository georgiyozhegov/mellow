use crate::{
    token::{BinaryOperator, UnaryOperator},
    tree::{Expression, Statement},
};

pub trait VisitStatement<I: Sized>: Sized {
    fn visit_tree(&mut self, tree: &Vec<Statement<I>>) {
        for statement in tree {
            statement.accept(self);
        }
    }
    fn visit_let(&mut self, identifier: &I, mutable: bool, value: &Expression);
    fn visit_change(&mut self, identifier: &I, value: &Expression);
    fn visit_while(&mut self, condition: &Expression, body: &Vec<Statement<I>>);
    fn visit_for(&mut self, item: &I, sequence: &Expression, body: &Vec<Statement<I>>);
}

pub trait VisitExpression {
    fn visit_integer(&mut self, value: i128);
    fn visit_identifier(&mut self, identifier: &String);
    fn visit_boolean(&mut self, value: bool);
    fn visit_string(&mut self, value: &String);
    fn visit_binary(&mut self, operator: &BinaryOperator, left: &Expression, right: &Expression);
    fn visit_unary(&mut self, operator: &UnaryOperator, operand: &Expression);
}

impl<I: Sized> Statement<I> {
    pub fn accept<V: VisitStatement<I>>(&self, visit: &mut V) {
        match self {
            Statement::Let {
                identifier,
                mutable,
                value,
            } => {
                visit.visit_let(identifier, *mutable, value);
            }
            Statement::Assign { identifier, value } => {
                visit.visit_change(identifier, value);
            }
            Statement::While { condition, body } => {
                visit.visit_while(condition, body);
            }
            Statement::For {
                item,
                sequence,
                body,
            } => {
                visit.visit_for(item, sequence, body);
            }
            _ => todo!()
        }
    }
}

impl Expression {
    pub fn accept<V: VisitExpression>(&self, visit: &mut V) {
        match self {
            Expression::Integer(value) => {
                visit.visit_integer(*value);
            }
            Expression::Identifier(identifier) => {
                visit.visit_identifier(identifier);
            }
            Expression::Boolean(value) => {
                visit.visit_boolean(*value);
            }
            Expression::String(value) => {
                visit.visit_string(value);
            }
            Expression::Binary(operator, left, right) => {
                visit.visit_binary(operator, left, right);
            }
            Expression::Unary(operator, operand) => {
                visit.visit_unary(operator, operand);
            }
            _ => todo!(),
        }
    }
}
