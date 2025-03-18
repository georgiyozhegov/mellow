use syntax::parse::{statement::*, VisitStatement};

use super::{block::BlockRange, Block, Cfg};

pub struct Constructor {
    output: Cfg,
}

impl Constructor {
    pub fn new() -> Self {
        Self { output: Cfg::new() }
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
        let mut previous = self.output.insert(Block::Basic(context.clone()));
        context.clear();
        node.or.insert(0, (node.condition, node.if_.clone()));
        let mut last_condition = None;
        for (condition, body) in node.or {
            let body = self.block(body.clone());
            self.output.branch(
                previous,
                condition.clone(),
                body.start,
                self.output.next_id(),
            );
            previous = body.end;
            last_condition = Some(condition);
        }
        let else_ = self.block(node.else_.clone());
        if let Some(condition) = last_condition {
            self.output.branch(
                previous,
                condition.clone(),
                self.output.next_id(),
                else_.start,
            );
        }
    }

    fn while_(&mut self, node: While, context: &mut Self::Context) -> Self::Output {
        let previous = self.output.insert(Block::Basic(context.clone()));
        context.clear();
        let start = self.output.insert(Block::Empty);
        self.output.direct(previous, start);
        let body = self.block(node.body);
        let end = self.output.next_id();
        self.output.branch(start, node.condition, body.start, end);
        self.output.direct(body.end, start);
    }
}

impl Constructor {
    fn block(&mut self, source: Vec<Statement>) -> BlockRange {
        let start = self.output.next_id();
        let mut current = Vec::new();
        for statement in source {
            statement.visit(self, &mut current);
        }
        if !current.is_empty() {
            self.output.insert(Block::Basic(current));
        }
        let end = self.output.last_id();
        BlockRange::new(start, end)
    }

    pub fn construct(mut self, source: Vec<Statement>) -> Cfg {
        self.block(source);
        self.output.insert(Block::Empty);
        self.output
    }
}
