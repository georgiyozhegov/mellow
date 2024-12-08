use std::{collections::HashMap, fmt::Display};

use syntax::{token::BinaryOperator, tree::{Expression, Statement}};

#[derive(Debug)]
pub enum Block {
    Instruction(Instruction),
    Scope(Vec<Block>),
}

#[derive(Debug)]
pub enum Instruction {
    Load(u64, Value),
    Copy(u64, u64),
    Binary { operator: BinaryOperator, left: u64, right: u64 },
    Label(&'static str),
    JumpIf { if_: u64, to: &'static str },
    Jump(&'static str),
}

#[derive(Debug)]
pub enum Value {
    Integer(i128),
}

pub struct Allocator {
    id: u64,
    map: HashMap<String, u64>,
}

impl Allocator {
    pub fn new() -> Self {
        Self { id: 0, map: HashMap::new() }
    }
}

impl Allocator {
    pub fn allocate(&mut self) -> u64 {
        let id = self.id;
        self.id += 1;
        id
    }

    pub fn last(&self) -> u64 {
        self.id
    }

    pub fn before_last(&self) -> u64 {
        self.id - 1
    }

    pub fn assign(&mut self, id: u64, identifier: String) {
        self.map.insert(identifier, id);
    }

    pub fn variable(&mut self, identifier: &String) -> u64 {
        *self.map.get(identifier).unwrap()
    }
}

fn expression(expression: Expression, output: &mut Vec<Block>, allocator: &mut Allocator) -> u64 {
    match expression {
        Expression::Integer(value) => {
            let id = allocator.allocate();
            let value = Value::Integer(value);
            let instruction = Instruction::Load(id, value);
            let block = Block::Instruction(instruction);
            output.push(block);
            id
        }
        Expression::Boolean(value) => {
            let id = allocator.allocate();
            let value = Value::Integer(value as i128);
            let instruction = Instruction::Load(id, value);
            let block = Block::Instruction(instruction);
            output.push(block);
            id
        }
        Expression::Identifier(identifier) => {
            allocator.variable(&identifier)
        }
        Expression::Binary(operator, left, right) => {
            let left = self::expression(*left, output, allocator);
            let right = self::expression(*right, output, allocator);
            let instruction = Instruction::Binary { operator, left, right };
            let block = Block::Instruction(instruction);
            output.push(block);
            left
        }
        _ => todo!(),
    }
}

fn statement(statement: Statement, output: &mut Vec<Block>, allocator: &mut Allocator) {
    match statement {
        Statement::Let { identifier, mutable: _, value } => {
            let id = expression(value, output, allocator);
            allocator.assign(id, identifier);
        }
        Statement::Change { identifier, value } => {
            let id = allocator.variable(&identifier);
            let value = self::expression(value, output, allocator);
            let instruction = Instruction::Copy(id, value);
            let block = Block::Instruction(instruction);
            output.push(block);
        }
        Statement::If { condition, true_, false_ } => {
            let id = expression(condition, output, allocator);
            output.push(Block::Instruction(Instruction::JumpIf { if_: id, to: "then" }));
            let mut false_block = Vec::new();
            for statement in false_ {
                self::statement(statement, &mut false_block, allocator);
            }
            output.push(Block::Scope(false_block));
            output.push(Block::Instruction(Instruction::Jump("end")));
            output.push(Block::Instruction(Instruction::Label("then")));
            let mut true_block = Vec::new();
            for statement in true_ {
                self::statement(statement, &mut true_block, allocator);
            }
            output.push(Block::Scope(true_block));
            output.push(Block::Instruction(Instruction::Label("end")));
        }
        _ => todo!(),
    }
}

pub fn ir(tree: Vec<Statement>) -> Vec<Block> {
    let mut output = Vec::new();
    let mut allocator = Allocator::new();
    for statement in tree {
        self::statement(statement, &mut output, &mut allocator);
    }
    output
}
