#[derive(Debug)]
pub enum Token {
    Integer(i128),
    Identifier(String),
    BinaryOperator(BinaryOperator),
}

#[derive(Debug)]
pub enum BinaryOperator {
    Add,
    Subtract,
    Multiply,
    Divide,
}
