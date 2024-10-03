#[derive(Debug)]
pub enum Token {
    Integer(i128),
    Identifier(String),
    BinaryOperator(BinaryOperator),
    UnaryOperator(UnaryOperator),
    LeftParenthesis,
    RightParenthesis,
    Equal,
    Let,
    If,
    Match,
    For,
    While,
    Loop,
    Do,
    Then,
    Else,
    Case,
    From,
    To,
    In,
    End,
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
