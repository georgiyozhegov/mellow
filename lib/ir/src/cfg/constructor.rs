use syntax::parse::{statement::*, VisitStatement};

use super::{block::BlockRange, Block};

pub struct Constructor {
    output: Vec<Block>,
}

impl Constructor {
    pub fn new() -> Self {
        Self { output: Vec::new() }
    }
}

impl Constructor {
    fn push(&mut self, block: Block) -> usize {
        self.output.push(block);
        self.last_id()
    }

    fn last_id(&self) -> usize {
        self.next_id() - 1
    }

    fn next_id(&self) -> usize {
        self.output.len()
    }
}

impl VisitStatement for Constructor {
    type Output = ();
    type Context = Vec<Statement>;

    fn let_(&mut self, node: Let, context: &mut Self::Context) -> Self::Output {
        context.push(Statement::Let(node));
    }

    fn assign(&mut self, node: Assign, context: &mut Self::Context) -> Self::Output {
        context.push(Statement::Assign(node));
    }

    fn debug(&mut self, node: Debug, context: &mut Self::Context) -> Self::Output {
        context.push(Statement::Debug(node));
    }

    fn if_(&mut self, mut node: If, context: &mut Self::Context) -> Self::Output {
        let mut previous = self.push(Block::new(context.clone()));
        context.clear();
        node.or.insert(0, node.if_);

        let mut branches = Vec::new();
        for branch in node.or {
            let condition = self.push(Block::empty());
            self.output[previous].direct(condition);

            let body = self.block(branch.body);
            let next = self.next_id();
            self.output[condition].branch(branch.condition, body.start, next);
            branches.push(body.end);

            previous = body.end;
        }

        let else_ = self.block(node.else_);
        self.output[previous].direct(else_.start);

        let end = self.push(Block::empty());
        for branch in branches {
            self.output[branch].direct(end);
        }
        self.output[else_.end].direct(end);
    }

    fn while_(&mut self, node: While, context: &mut Self::Context) -> Self::Output {
        let previous = self.push(Block::new(context.clone()));
        context.clear();
        let start = self.push(Block::empty());
        self.output[previous].direct(start);
        let body = self.block(node.body);
        let end = self.next_id();
        self.output[start].branch(node.condition, body.start, end);
        self.output[body.end].direct(start);
    }
}

impl Constructor {
    fn block(&mut self, source: Vec<Statement>) -> BlockRange {
        let start = self.next_id();
        let mut current = Vec::new();
        for statement in source {
            statement.visit(self, &mut current);
        }
        if !current.is_empty() {
            self.push(Block::new(current));
        }
        let end = self.last_id();
        BlockRange::new(start, end)
    }

    pub fn construct(mut self, source: Vec<Statement>) -> Vec<Block> {
        self.block(source);
        self.output
    }
}
