use std::fmt::{self, Display};

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Clone)]
pub enum Error {
    InvalidCharacter(char),
    ExpectedButGot {
        expected: String,
        got: String,
    },
    NotImplemented(String),
}

impl Error {
    pub fn expected_but_got<E: ToString, G: ToString>(expected: E, got: G) -> Self {
        Self::ExpectedButGot {
            expected: expected.to_string(),
            got: got.to_string(),
        }
    }
}

impl Display for Error {
    fn fmt(&self, formatter: &mut std::fmt::Formatter) -> fmt::Result {
        match self {
            Self::InvalidCharacter(c) => {
                write!(formatter, "invalid character: '{c}'")
            }
            Self::ExpectedButGot { expected, got } => {
                write!(formatter, "expected {expected}, but got {got}")
            }
            Self::NotImplemented(message) => {
                write!(formatter, "not implemented yet: {message}")
            }
        }
    }
}
