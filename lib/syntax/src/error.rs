use crate::Token;

#[derive(Debug)]
pub enum SyntaxError {
    InvalidCharacter(char),
    Grammar {
        expected: &'static str,
        found: Option<Token>,
    },
}
