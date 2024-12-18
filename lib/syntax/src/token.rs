#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    Integer(i128),
    Identifier(String),
    BinaryOperator(BinaryOperator),
    String(String),
    UnaryOperator(UnaryOperator),
    LeftParenthesis,
    RightParenthesis,
    Let,
    Mutable,
    Equal,
    If,
    Or,
    Else,
    Then,
    While,
    For,
    In,
    Loop,
    Do,
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
    Not,
}
