use crate::parse::Expression;

use super::Statement;

#[derive(Debug, Clone)]
pub struct If {
    pub condition: Expression,
    pub if_: Vec<Statement>,
    pub or: Vec<(Expression, Vec<Statement>)>,
    pub else_: Vec<Statement>,
}
