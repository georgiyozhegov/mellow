use crate::parse::Expression;

use super::Statement;

#[derive(Debug, Clone)]
pub struct While {
    pub condition: Expression,
    pub body: Vec<Statement>,
}
