use super::Expression;

#[derive(Debug, Clone)]
pub enum Statement {
    Let(Let),
    Assign(Assign),
    If(If),
    While(While),
    For(For),
    Debug(Debug),
}

#[derive(Debug, Clone)]
pub struct Let {
    pub identifier: String,
    pub mutable: bool,
    pub value: Expression,
}

#[derive(Debug, Clone)]
pub struct Assign {
    pub identifier: String,
    pub value: Expression,
}

#[derive(Debug, Clone)]
pub struct If {
    pub condition: Expression,
    pub if_: Vec<Statement>,
    pub or: Vec<(Expression, Vec<Statement>)>,
    pub else_: Vec<Statement>,
}

#[derive(Debug, Clone)]
pub struct While {
    pub condition: Expression,
    pub body: Vec<Statement>,
}

#[derive(Debug, Clone)]
pub struct For {
    pub item: String,
    pub sequence: Expression,
    pub body: Vec<Statement>,
}

#[derive(Debug, Clone)]
pub struct Debug {
    pub value: Expression,
}
