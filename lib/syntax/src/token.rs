#[derive(Debug)]
pub enum Token {
    Integer(i128),
    Identifier(String),
    BinaryOperator(BinaryOperator),
    UnaryOperator(UnaryOperator),
    LeftParenthesis,
    RightParenthesis,
}

#[derive(Debug)]
pub enum BinaryOperator {
    Add,
    Subtract,
    Multiply,
    Divide,
}

#[derive(Debug)]
pub enum UnaryOperator {
    Negate,
}
