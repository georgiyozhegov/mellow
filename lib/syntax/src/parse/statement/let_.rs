use crate::parse::Expression;

#[derive(Debug, Clone)]
pub struct Let {
    pub identifier: String,
    pub mutable: bool,
    pub value: Expression,
}
