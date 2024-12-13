use crate::{
    token::{BinaryOperator, UnaryOperator},
    tree::{Expression, Statement},
};

pub trait VisitStatement {
    fn visit_let(&mut self, identifier: &String, mutable: bool, value: &Expression);
    fn visit_change(&mut self, identifier: &String, value: &Expression);
    fn visit_if(&mut self, condition: &Expression, true_: &Vec<Statement>, false_: &Vec<Statement>);
    fn visit_while(&mut self, condition: &Expression, body: &[Statement]);
    fn visit_for_in(&mut self, item: &String, sequence: &Expression, body: &Vec<Statement>);
    fn visit_for_from_to(
        &mut self,
        item: &String,
        start: &Expression,
        end: &Expression,
        body: &Vec<Statement>,
    );
}

pub trait VisitExpression {
    fn visit_integer(&mut self, value: i128);
    fn visit_identifier(&mut self, identifier: &String);
    fn visit_boolean(&mut self, value: bool);
    fn visit_string(&mut self, value: &String);
    fn visit_binary(&mut self, operator: &BinaryOperator, left: &Expression, right: &Expression);
    fn visit_unary(&mut self, operator: &UnaryOperator, operand: &Expression);
    fn visit_if(&mut self, condition: &Expression, true_: &Expression, false_: Option<&Expression>);
}

impl Statement {
    pub fn accept<V: VisitStatement>(&self, visit: &mut V) {
        match self {
            Statement::Let {
                identifier,
                mutable,
                value,
            } => {
                visit.visit_let(identifier, *mutable, value);
            }
            Statement::Change { identifier, value } => {
                visit.visit_change(identifier, value);
            }
            Statement::If {
                condition,
                true_,
                false_,
            } => {
                visit.visit_if(condition, true_, false_);
            }
            Statement::While { condition, body } => {
                visit.visit_while(condition, body);
            }
            Statement::ForIn {
                item,
                sequence,
                body,
            } => {
                visit.visit_for_in(item, sequence, body);
            }
            Statement::ForFromTo {
                item,
                start,
                end,
                body,
            } => {
                visit.visit_for_from_to(item, start, end, body);
            }
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
            Expression::If {
                condition,
                true_,
                false_,
            } => {
                visit.visit_if(condition, true_, false_.as_deref());
            }
        }
    }
}
