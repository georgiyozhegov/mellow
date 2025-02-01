use syntax::tree::{Expression, Statement};
use std::collections::HashMap;

#[derive(Debug)]
pub enum Instruction {
    Integer { to: u64, value: i128 },
    Copy { to: u64, from: u64 },
    Add { to: u64, value: u64 },
    Subtract { to: u64, value: u64 },
    Multiply { to: u64, value: u64 },
    Divide { to: u64, value: u64 },
}

impl Instruction {
    fn expression(expression: Expression, to: &mut u64, output: &mut Vec<Self>) {
        match expression {
            Expression::Integer(value) => {
                let instruction = Self::Integer { to: *to, value };
                *to += 1;
                output.push(instruction);
            }
            _ => todo!(),
        };
    }
}

impl Instruction {
    pub fn statement(
        statement: Statement,
        to: &mut u64,
        variables: &mut HashMap<String, u64>,
        output: &mut Vec<Self>,
    ) {
        match statement {
            Statement::Let {
                identifier, value, ..
            }
            | Statement::Assign {
                identifier, value, ..
            } => {
                Self::expression(value, to, output);
                variables.insert(identifier, *to);
            }
            _ => todo!(),
        }
    }
}