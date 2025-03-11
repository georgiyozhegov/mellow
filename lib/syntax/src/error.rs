use std::fmt::{self, Display, Formatter};

use crate::token::Token;

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
                Self::Plus => "'+'".into(),
                Self::Minus => "'-'".into(),
                Self::Star => "'*'".into(),
                Self::Slash => "'/'".into(),
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
            }
        )
    }
}
