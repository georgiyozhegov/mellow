#[derive(Debug, PartialEq)]
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
    True,
    False,
}

#[derive(Debug, Clone, PartialEq)]
pub enum BinaryOperator {
    Add,
    Subtract,
    Multiply,
    Divide,
}

#[derive(Debug, Clone, PartialEq)]
pub enum UnaryOperator {
    Negate,
}
