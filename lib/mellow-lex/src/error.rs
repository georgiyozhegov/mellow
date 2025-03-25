use crate::Token;

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
    pub fn invalid_character(c: &char) -> Self {
        Self::InvalidCharacter(*c)
    }

    pub fn grammar<T: ToString>(expected: T, found: Option<Token>) -> Self {
        Self::Grammar {
            expected: expected.to_string(),
            found,
        }
    }
}
