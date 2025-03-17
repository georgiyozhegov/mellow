use syntax::parse::{expression, statement, BinaryKind, VisitExpression, VisitStatement};

use super::Instruction;
use crate::cfg::{Block, Cfg, Link};

pub struct Constructor {
    output: Vec<Instruction>,
    temporary: u64,
}

impl Constructor {
    pub fn new() -> Self {
        Self {
            output: Vec::new(),
            temporary: 0,
        }
    }
}

impl Constructor {
    fn push(&mut self, instruction: Instruction) {
        self.output.push(instruction);
    }

    fn allocate(&mut self) -> u64 {
        let id = self.temporary;
        self.temporary += 1;
        id
    }

    fn block(&mut self, value: Block) {
        match value {
            Block::Basic(body) => {
                for statement in body {
                    statement.visit(self, &mut ());
                }
            }
            Block::Empty => {}
        };
    }

    fn link(&mut self, value: &Link) {
        match value {
            Link::Direct(to) => {
                self.push(Instruction::Jump(*to));
            }
            Link::Branch {
                condition,
                true_,
                false_,
            } => {
                let condition = condition.clone().visit(self);
                self.push(Instruction::JumpIf {
                    condition,
                    to: *true_,
                });
                self.push(Instruction::Jump(*false_));
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

impl VisitStatement for Constructor {
    type Output = ();
    type Context = ();

    fn let_(&mut self, node: statement::Let, _context: &mut Self::Context) -> Self::Output {
        let from = node.value.visit(self);
        self.push(Instruction::Set {
            identifier: node.identifier,
            from,
        });
    }

    fn assign(&mut self, node: statement::Assign, _context: &mut ()) -> Self::Output {
        let from = node.value.visit(self);
        self.output.push(Instruction::Set {
            identifier: node.identifier,
            from,
        });
    }

    fn debug(&mut self, node: statement::Debug, _context: &mut ()) -> Self::Output {
        let value = node.value.visit(self);
        let instruction = Instruction::Call {
            label: "debug_i64".into(),
            value,
        };
        self.push(instruction);
    }
}

impl VisitExpression for Constructor {
    type Output = u64;

    fn integer(&mut self, node: expression::Integer) -> Self::Output {
        let id = self.allocate();
        self.output.push(Instruction::Integer {
            to: id,
            value: node.value,
        });
        id
    }

    fn identifier(&mut self, node: expression::Identifier) -> Self::Output {
        let id = self.allocate();
        self.push(Instruction::Get {
            to: id,
            identifier: node.name,
        });
        id
    }

    fn boolean(&mut self, node: expression::Boolean) -> Self::Output {
        let id = self.allocate();
        self.output.push(Instruction::Integer {
            to: id,
            value: node.value as i128,
        });
        id
    }

    fn string(&mut self, node: expression::Str) -> Self::Output {
        let id = self.allocate();
        self.push(Instruction::String {
            to: id,
            value: node.value,
        });
        id
    }

    fn binary(&mut self, node: expression::Binary) -> Self::Output {
        let left = node.left.visit(self);
        let right = node.right.visit(self);
        let id = self.allocate();
        let instruction = match node.kind {
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
        self.push(instruction);
        id
    }
}
