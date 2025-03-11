use syntax::parse::{BinaryKind, Expression, Statement, VisitExpression, VisitStatement};

use crate::{
    cfg::{Cfg, Link},
    Block, Instruction,
};

pub struct Convert {
    output: Vec<Instruction>,
    temporary: u64,
}

impl Convert {
    pub fn new() -> Self {
        Self {
            output: Vec::new(),
            temporary: 0,
        }
    }
}

impl Convert {
    fn allocate(&mut self) -> u64 {
        let id = self.temporary;
        self.temporary += 1;
        id
    }

    fn block(&mut self, value: Block) {
        match value {
            Block::Basic(body) => {
                for statement in body {
                    statement.visit(self);
                }
            }
            Block::Empty => {}
        };
    }

    fn link(&mut self, link: &Link) {
        match link {
            Link::Direct(to) => {
                self.output.push(Instruction::Jump(*to));
            }
            Link::Branch {
                condition,
                true_,
                false_,
            } => {
                let condition = condition.visit(self);
                self.output.extend(vec![
                    Instruction::JumpIf {
                        condition,
                        to: *true_,
                    },
                    Instruction::Jump(*false_),
                ]);
            }
        }
    }

    pub fn construct(mut self, source: Cfg<Block, Link>) -> Vec<Instruction> {
        for (id, block) in source.blocks.into_iter().enumerate() {
            self.block(block);
            if let Some(link) = source.links.get(&(id as u64)) {
                self.link(link);
            }
        }
        self.output
    }
}

impl VisitStatement for Convert {
    type Output = ();

    fn let_(&mut self, identifier: &String, mutable: &bool, value: &Expression) {
        let from = value.visit(self);
        self.output.push(Instruction::Set {
            identifier: identifier.clone(),
            from,
        });
        // TODO: mutable check
    }

    fn assign(&mut self, identifier: &String, value: &Expression) -> Self::Output {
        let from = value.visit(self);
        self.output.push(Instruction::Set {
            identifier: identifier.clone(),
            from,
        });
    }

    fn debug(&mut self, value: &Expression) -> Self::Output {
        let value = value.visit(self);
        let instruction = Instruction::Call {
            label: "debug_i64".into(),
            value,
        };
        self.output.push(instruction);
    }
}

impl VisitExpression for Convert {
    type Output = u64;

    fn integer(&mut self, value: &i128) -> Self::Output {
        let id = self.allocate();
        self.output.push(Instruction::Integer {
            to: id,
            value: *value,
        });
        id
    }

    fn identifier(&mut self, name: &String) -> Self::Output {
        let id = self.allocate();
        self.output.push(Instruction::Get {
            to: id,
            identifier: name.clone(),
        });
        id
    }

    fn boolean(&mut self, value: &bool) -> Self::Output {
        let id = self.allocate();
        self.output.push(Instruction::Integer {
            to: id,
            value: *value as i128,
        });
        id
    }

    fn string(&mut self, value: &String) -> Self::Output {
        let id = self.allocate();
        self.output.push(Instruction::String {
            to: id,
            value: value.clone(),
        });
        id
    }

    fn binary(
        &mut self,
        kind: &BinaryKind,
        left: &Box<Expression>,
        right: &Box<Expression>,
    ) -> Self::Output {
        let left = left.visit(self);
        let right = right.visit(self);
        let id = self.allocate();
        let instruction = match kind {
            BinaryKind::Add => Instruction::Add {
                to: id,
                left,
                right,
            },
            BinaryKind::Subtract => Instruction::Subtract {
                to: id,
                left,
                right,
            },
            BinaryKind::Multiply => Instruction::Multiply {
                to: id,
                left,
                right,
            },
            BinaryKind::Divide => Instruction::Divide {
                to: id,
                left,
                right,
            },
            BinaryKind::Greater => Instruction::Greater {
                to: id,
                left,
                right,
            },
            BinaryKind::Less => Instruction::Less {
                to: id,
                left,
                right,
            },
            BinaryKind::Equal => Instruction::Equal {
                to: id,
                left,
                right,
            },
        };
        self.output.push(instruction);
        id
    }
}
