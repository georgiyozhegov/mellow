use crate::parse::Expression;

#[derive(Debug, Clone)]
pub struct Assign {
    pub identifier: String,
    pub value: Expression,
}
