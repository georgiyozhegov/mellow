use std::collections::HashMap;

use syntax::tree::{Expression, Statement};

use crate::Allocator;

#[derive(Debug)]
pub enum Instruction {
    Integer { to: u64, value: i128 },
    Copy { to: u64, from: u64 },
    Add { to: u64, value: u64 },
    Subtract { to: u64, value: u64 },
    Multiply { to: u64, value: u64 },
    Divide { to: u64, value: u64 },
}

fn variable_not_found(identifier: &str) -> ! {
    panic!("error: variable {identifier} is not found") // TODO: proper error-handling
}

impl Instruction {
    fn expression(
        expression: Expression,
        allocator: &mut Allocator,
        output: &mut Vec<Self>,
    ) -> u64 {
        match expression {
            Expression::Integer(value) => {
                allocator.id += 1;
                let to = allocator.id;
                let instruction = Self::Integer { to, value };
                output.push(instruction);
                to
            }
            Expression::Identifier(identifier) => {
                if let Some(from) = allocator.variables.get(&identifier) {
                    allocator.id += 1;
                    let to = allocator.id;
                    let instruction = Self::Copy { to, from: *from };
                    output.push(instruction);
                    to
                } else {
                    variable_not_found(&identifier);
                }
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
            } => {
                let id = Self::expression(value, allocator, output);
                allocator.variables.insert(identifier, id);
            }
            Statement::Assign { identifier, value } => {
                let from = Self::expression(value, allocator, output);
                if let Some(to) = allocator.variables.get(&identifier) {
                    let instruction = Self::Copy { to: *to, from };
                    output.push(instruction);
                } else {
                    variable_not_found(&identifier);
                }
            }
            _ => unreachable!("conditional statements are not present in control flow graph"),
        }
    }
}
