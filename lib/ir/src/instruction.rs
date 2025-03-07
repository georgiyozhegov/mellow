use std::fmt::{self, Display, Formatter};

use syntax::{
    token::BinaryOperator,
    tree::{Expression, Statement},
};

use crate::tac::Allocator;

#[derive(Debug)]
pub enum Instruction {
    Label(u64),
    Integer { to: u64, value: i128 },
    Get { to: u64, identifier: String },
    Set { identifier: String, from: u64 },
    String { to: u64, value: String },
    Add { to: u64, left: u64, right: u64 },
    Subtract { to: u64, left: u64, right: u64 },
    Multiply { to: u64, left: u64, right: u64 },
    Divide { to: u64, left: u64, right: u64 },
    Greater { to: u64, left: u64, right: u64 },
    Less { to: u64, left: u64, right: u64 },
    Equal { to: u64, left: u64, right: u64 },
    Jump { to: u64 },
    JumpIf { condition: u64, to: u64 },
    Call { label: String, value: u64 },
}

impl Instruction {
    pub fn expression(
        expression: Expression,
        allocator: &mut Allocator,
        output: &mut Vec<Self>,
    ) -> u64 {
        match expression {
            Expression::Integer(value) => {
                let id = allocator.allocate();
                let instruction = Self::Integer { to: id, value };
                output.push(instruction);
                id
            }
            Expression::Identifier(identifier) => {
                let id = allocator.allocate();
                let instruction = Self::Get { to: id, identifier };
                output.push(instruction);
                id
            }
            Expression::Boolean(value) => {
                let id = allocator.allocate();
                let instruction = Self::Integer {
                    to: id,
                    value: value as i128,
                };
                output.push(instruction);
                id
            }
            Expression::String(value) => {
                let id = allocator.allocate();
                let instruction = Self::String { to: id, value };
                output.push(instruction);
                id
            }
            Expression::Binary(operator, left, right) => {
                let left = Self::expression(*left, allocator, output);
                let right = Self::expression(*right, allocator, output);
                let id = allocator.allocate();
                let instruction = match operator {
                    BinaryOperator::Add => Self::Add {
                        to: id,
                        left,
                        right,
                    },
                    BinaryOperator::Subtract => Self::Subtract {
                        to: id,
                        left,
                        right,
                    },
                    BinaryOperator::Multiply => Self::Multiply {
                        to: id,
                        left,
                        right,
                    },
                    BinaryOperator::Divide => Self::Divide {
                        to: id,
                        left,
                        right,
                    },
                    BinaryOperator::Greater => Self::Greater {
                        to: id,
                        left,
                        right,
                    },
                    BinaryOperator::Less => Self::Less {
                        to: id,
                        left,
                        right,
                    },
                    BinaryOperator::Equal => Self::Equal {
                        to: id,
                        left,
                        right,
                    },
                };
                output.push(instruction);
                id
            }
            expression => todo!("{expression:?}"),
        }
    }
}

impl Instruction {
    pub fn statement(statement: Statement, allocator: &mut Allocator, output: &mut Vec<Self>) {
        match statement {
            Statement::Let {
                identifier, value, ..
            }
            | Statement::Assign { identifier, value } => {
                let from = Self::expression(value, allocator, output);
                let instruction = Self::Set { identifier, from };
                output.push(instruction);
            }
            Statement::Debug(value) => {
                let value = Self::expression(value, allocator, output);
                let instruction = Self::Call {
                    label: "debug_i64".into(),
                    value,
                }; // TODO: implement type system
                output.push(instruction);
            }
            _ => unreachable!("conditional statements are not present in control flow graph"),
        }
    }
}

impl Display for Instruction {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            Self::Label(id) => {
                write!(f, "@{id}")
            }
            Self::Integer { to, value } => {
                write!(f, "#{to} int {value}")
            }
            Self::Get { to, identifier } => {
                write!(f, "#{to} get ${identifier}")
            }
            Self::Set { identifier, from } => {
                write!(f, "${identifier} set #{from}")
            }
            Self::String { to, value } => {
                write!(f, "${to} str \"{value}\"")
            }
            Self::Add { to, left, right } => {
                write!(f, "#{to} add #{left} #{right}")
            }
            Self::Subtract { to, left, right } => {
                write!(f, "#{to} sub #{left} #{right}")
            }
            Self::Multiply { to, left, right } => {
                write!(f, "#{to} mul #{left} #{right}")
            }
            Self::Divide { to, left, right } => {
                write!(f, "#{to} div #{left} #{right}")
            }
            Self::Greater { to, left, right } => {
                write!(f, "#{to} gt #{left} #{right}")
            }
            Self::Less { to, left, right } => {
                write!(f, "#{to} lt #{left} #{right}")
            }
            Self::Equal { to, left, right } => {
                write!(f, "#{to} eq #{left} #{right}")
            }
            Self::Jump { to } => {
                write!(f, "jump @{to}")
            }
            Self::JumpIf { condition, to } => {
                write!(f, "jump @{to} if #{condition}")
            }
            Self::Call { label, value } => {
                write!(f, "call {label} #{value}")
            }
        }
    }
}
