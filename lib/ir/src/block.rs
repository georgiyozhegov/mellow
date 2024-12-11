use syntax::tree::Statement;

#[derive(Debug)]
pub enum Block {
    Basic(Vec<Statement>),
    Empty,
}
