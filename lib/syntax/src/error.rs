use std::fmt::{self, Display, Formatter};

use crate::token::{BinaryOperator, Token, UnaryOperator};

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Clone)]
pub enum Error {
    InvalidCharacter(char),
    Grammar {
        expected: String,
        found: Option<Token>,
    },
}

impl Error {
    pub fn grammar<T: ToString>(expected: T, found: Option<Token>) -> Self {
        Self::Grammar {
            expected: expected.to_string(),
            found,
        }
    }
}

impl Display for Error {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidCharacter(c) => write!(formatter, "error: invalid character: '{c}'"),
            Self::Grammar { expected, found } => {
                write!(
                    formatter,
                    "error: expected {}, but found {}",
                    expected,
                    eof_or_token(found.clone())
                )
            }
        }
    }
}

fn eof_or_token(token: Option<Token>) -> String {
    if let Some(token) = token {
        token.to_string()
    } else {
        "EOF".to_string()
    }
}

impl Display for Token {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "{}",
            match self {
                Self::Integer(value) => format!("integer literal '{value}'"),
                Self::Identifier(value) => format!("identifier '{value}'"),
                Self::String(value) => format!("string literal \"{value}\""),
                Self::BinaryOperator(operator) => operator.to_string(),
                Self::UnaryOperator(operator) => operator.to_string(),
                Self::LeftParenthesis => "(".to_string(),
                Self::RightParenthesis => ")".to_string(),
                Self::Let => "'let'".to_string(),
                Self::Mutable => "'mutable'".to_string(),
                Self::Equal => "'='".to_string(),
                Self::If => "'if'".to_string(),
                Self::Or => "'or'".to_string(),
                Self::Else => "'else'".to_string(),
                Self::Then => "'then'".to_string(),
                Self::While => "'while'".to_string(),
                Self::For => "'for'".to_string(),
                Self::In => "'in'".to_string(),
                Self::Loop => "'loop'".to_string(),
                Self::Do => "'do'".to_string(),
                Self::End => "'end'".to_string(),
                Self::True => "'true'".to_string(),
                Self::False => "'false'".to_string(),
                Self::Debug => "'debug'".to_string(),
            }
        )
    }
}

impl Display for BinaryOperator {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "{}",
            match self {
                Self::Add => "'+'",
                Self::Subtract => "'-'",
                Self::Multiply => "'*'",
                Self::Divide => "'/'",
                Self::Greater => "'>'",
                Self::Less => "'<'",
                Self::Equal => "'?'",
            }
        )
    }
}

impl Display for UnaryOperator {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "{}",
            match self {
                Self::Negate => "'-'",
                Self::Not => "'!'",
            }
        )
    }
}
