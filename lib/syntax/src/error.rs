use std::fmt::{self, Display, Formatter};
use crate::token::{BinaryOperator, Token, UnaryOperator};

#[derive(Debug, Clone)]
pub enum SyntaxError {
    InvalidCharacter(char),
    Grammar {
        expected: &'static str,
        found: Option<Token>,
    },
}

impl Display for SyntaxError {
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
                Self::Integer(value) => value.to_string(),
                Self::Identifier(value) => format!("identifier '{value}'"),
                Self::BinaryOperator(operator) => operator.to_string(),
                Self::UnaryOperator(operator) => operator.to_string(),
                Self::LeftParenthesis => "(".to_string(),
                Self::RightParenthesis => ")".to_string(),
                Self::Equal => "'='".to_string(),
                Self::Let => "'let'".to_string(),
                Self::If => "'if'".to_string(),
                Self::Match => "'match'".to_string(),
                Self::For => "'for'".to_string(),
                Self::While => "'while'".to_string(),
                Self::Loop => "'loop'".to_string(),
                Self::Do => "'do'".to_string(),
                Self::Then => "'then'".to_string(),
                Self::Else => "'else'".to_string(),
                Self::Case => "'case'".to_string(),
                Self::From => "'from'".to_string(),
                Self::To => "'to'".to_string(),
                Self::In => "'in'".to_string(),
                Self::End => "'end'".to_string(),
                Self::True => "'true'".to_string(),
                Self::False => "'false'".to_string(),
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
            }
        )
    }
}
