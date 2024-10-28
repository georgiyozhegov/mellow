use syntax::{
    tree::{Expression, Statement},
    Parse,
};

use crate::instruction::{Block, Instruction, Size, Storage};

pub struct Assembly<'a> {
    source: Parse<'a>,
}

impl<'a> Assembly<'a> {
    pub fn new(source: Parse<'a>) -> Self {
        Self { source }
    }
}

impl<'a> Iterator for Assembly<'a> {
    type Item = Block;

    fn next(&mut self) -> Option<Self::Item> {
        self.statement()
    }
}

impl<'a> Assembly<'a> {
    pub fn statement(&mut self) -> Option<Block> {
        match self.source.next()?.unwrap() {
            Statement::Let {
                identifier, value, ..
            } => Some(self.r#let(identifier, value)),
            _ => todo!(),
        }
    }

    fn r#let(&self, _identifier: String, value: Expression) -> Block {
        let mut block = Vec::new();
        block.extend(self.expression(value));
        block.push(Instruction::Move {
            to: Storage::Stack {
                offset: 0,
                size: Size::Byte,
            },
            from: Storage::Register {
                index: 0,
                size: Size::Byte,
            },
        });
        block
    }
}

impl<'a> Assembly<'a> {
    pub fn expression(&self, expression: Expression) -> Block {
        match expression {
            Expression::Integer(value) => {
                let mut block = Vec::new();
                block.push(Instruction::Move {
                    to: Storage::Register {
                        index: 0,
                        size: Size::Byte,
                    },
                    from: Storage::Integer(value),
                });
                block
            }
            _ => todo!(),
        }
    }
}
