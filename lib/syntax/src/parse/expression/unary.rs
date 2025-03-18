use crate::parse::UnaryKind;

use super::Expression;

#[derive(Debug, Clone)]
pub struct Unary {
    pub kind: UnaryKind,
    pub inner: Box<Expression>,
}
