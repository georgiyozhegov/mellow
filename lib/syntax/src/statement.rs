#[derive(Debug)]
pub enum Statement {
    Let { identifier: String, value: Expression, },
}

#[derive(Debug)]
pub enum Expression {
    Integer(i128),
}
