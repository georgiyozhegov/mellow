use super::Expression;

#[derive(Debug, Clone)]
pub enum Statement {
    Let {
        identifier: String,
        mutable: bool,
        value: Expression,
    },
    Assign {
        identifier: String,
        value: Expression,
    },
    If {
        condition: Expression,
        if_: Vec<Statement>,
        or: Vec<(Expression, Vec<Statement>)>,
        else_: Vec<Statement>,
    },
    While {
        condition: Expression,
        body: Vec<Statement>,
    },
    For {
        item: String,
        sequence: Expression,
        body: Vec<Statement>,
    },
    Debug(Expression),
}
