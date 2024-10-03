#[derive(Debug)]
pub enum SyntaxError {
    InvalidCharacter(char),
    Grammar(String),
}
