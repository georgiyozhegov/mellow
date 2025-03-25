use std::fmt::{self, Display, Formatter};

#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    Integer(i128),
    Identifier(String),
    Plus,
    Minus,
    Star,
    Slash,
    Greater,
    Less,
    Question,
    String(String),
    Negate,
    Not,
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
    Debug,
}

macro_rules! quotify {
    ($value:expr) => {
        String::from("'") + $value + "'"
    };
}

impl Display for Token {
    fn fmt(&self, formatter: &mut Formatter) -> fmt::Result {
        let string = match self {
            Self::Integer(value) => format!("integer literal '{value}'"),
            Self::Identifier(value) => format!("identifier '{value}'"),
            Self::String(value) => format!("string literal \"{value}\""),
            Self::Plus => quotify!("+"),
            Self::Minus => quotify!("-"),
            Self::Star => quotify!("*"),
            Self::Slash => quotify!("/"),
            Self::Greater => "'>'".into(),
            Self::Less => "'<'".into(),
            Self::Question => "'?'".into(),
            Self::Negate => "'-'".into(),
            Self::Not => "'!'".into(),
            Self::LeftParenthesis => "(".into(),
            Self::RightParenthesis => ")".into(),
            Self::Let => "'let'".into(),
            Self::Mutable => "'mutable'".into(),
            Self::Equal => "'='".into(),
            Self::If => "'if'".into(),
            Self::Or => "'or'".into(),
            Self::Else => "'else'".into(),
            Self::Then => "'then'".into(),
            Self::While => "'while'".into(),
            Self::For => "'for'".into(),
            Self::In => "'in'".into(),
            Self::Loop => "'loop'".into(),
            Self::Do => "'do'".into(),
            Self::End => "'end'".into(),
            Self::True => "'true'".into(),
            Self::False => "'false'".into(),
            Self::Debug => "'debug'".into(),
        };
        write!(formatter, "{string}")
    }
}
