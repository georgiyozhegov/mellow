#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    Integer(i128),
    Identifier(String),
    BinaryOperator(BinaryOperator),
    UnaryOperator(UnaryOperator),
    LeftParenthesis,
    RightParenthesis,
    Equal,
    Let,
    Mutable,
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
    True,
    False,
}

#[derive(Debug, Clone, PartialEq)]
pub enum BinaryOperator {
    Add,
    Subtract,
    Multiply,
    Divide,
    Greater,
    Less,
    Equal,
}

#[derive(Debug, Clone, PartialEq)]
pub enum UnaryOperator {
    Negate,
}
