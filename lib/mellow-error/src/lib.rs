pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Clone)]
pub enum Error {
    InvalidCharacter(char),
    ExpectedButGot {
        expected: String,
        got: String,
    },
}

impl Error {
    pub fn expected_but_got<E: ToString, G: ToString>(expected: E, got: G) -> Self {
        Self::ExpectedButGot {
            expected: expected.to_string(),
            got: got.to_string(),
        }
    }
}
