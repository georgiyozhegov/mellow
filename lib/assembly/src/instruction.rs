use std::collections::HashMap;
use std::fmt::{self, Display, Formatter};

use syntax::{
    token::BinaryOperator,
    tree::{Expression, Statement},
};

use crate::Allocator;

#[derive(Debug)]
pub enum Instruction {
    Integer { to: u64, value: i128 },
    Get { to: u64, identifier: String },
    Set { identifier: String, from: u64 },
    Add { to: u64, left: u64, right: u64 },
    Subtract { to: u64, left: u64, right: u64 },
    Multiply { to: u64, left: u64, right: u64 },
    Divide { to: u64, left: u64, right: u64 },
}

impl Instruction {
    fn expression(
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
                    _ => todo!(),
                };
                output.push(instruction);
                id
            }
            _ => todo!(),
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
            _ => unreachable!("conditional statements are not present in control flow graph"),
        }
    }
}

impl Display for Instruction {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            Self::Integer { to, value } => {
                write!(f, "#{to} = {value}")
            }
            Self::Get { to, identifier } => {
                write!(f, "#{to} = ${identifier}")
            }
            Self::Set { identifier, from } => {
                write!(f, "${identifier} = #{from}")
            }
            Self::Add { to, left, right } => {
                write!(f, "#{to} = #{left} + #{right}")
            }
            Self::Subtract { to, left, right } => {
                write!(f, "#{to} = #{left} - #{right}")
            }
            Self::Multiply { to, left, right } => {
                write!(f, "#{to} = #{left} * #{right}")
            }
            Self::Divide { to, left, right } => {
                write!(f, "#{to} = #{left} / #{right}")
            }
            _ => todo!(),
        }
    }
}
