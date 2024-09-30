#[derive(Debug)]
pub enum Token {
    Integer(i128),
    Identifier(String),
}
