use crate::parse::Expression;

use super::Statement;

#[derive(Debug, Clone)]
pub struct For {
    pub item: String,
    pub sequence: Expression,
    pub body: Vec<Statement>,
}
