use super::Expression;

#[derive(Debug, Clone)]
pub struct If {
    pub condition: Box<Expression>,
    pub if_: Box<Expression>,
    pub or: Vec<(Expression, Expression)>,
    pub else_: Option<Box<Expression>>,
}
