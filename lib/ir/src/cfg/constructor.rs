use syntax::parse::{Expression, Statement, VisitStatement, *};

use super::{block::BlockRange, Block, Cfg, Link};

pub struct Constructor {
    output: Cfg<Block, Link>,
}

impl Constructor {
    pub fn new() -> Self {
        Self { output: Cfg::new() }
    }
}

impl VisitStatement for Constructor {
    type Output = ();
    type Context = Vec<Statement>;

    fn let_(
        &mut self,
        value: Let,
        context: &mut Self::Context,
    ) -> Self::Output {
        context.push(Statement::Let(value));
    }

    fn assign(
        &mut self,
        value: Assign,
        context: &mut Self::Context,
    ) -> Self::Output {
        context.push(Statement::Assign(value));
    }

    fn debug(&mut self, value: Debug, context: &mut Self::Context) -> Self::Output {
        context.push(Statement::Debug(value));
    }

    fn if_(
        &mut self,
        mut value: If,
        context: &mut Self::Context,
    ) -> Self::Output {
        let mut previous = self.output.insert(Block::Basic(context.clone()));
        context.clear();
        value.or.insert(0, (value.condition, value.if_.clone()));
        let mut last_condition = None;
        for (condition, body) in value.or {
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
        let else_ = self.block(value.else_.clone());
        if let Some(condition) = last_condition {
            self.output.branch(
                previous,
                condition.clone(),
                self.output.next_id(),
                else_.start,
            );
        }
    }

    fn while_(
        &mut self,
        value: While,
        context: &mut Self::Context,
    ) -> Self::Output {
        let previous = self.output.insert(Block::Basic(context.clone()));
        context.clear();
        let start = self.output.insert(Block::Empty);
        self.output.direct(previous, start);
        let body = self.block(value.body);
        let end = self.output.next_id();
        self.output
            .branch(start, value.condition, body.start, end);
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

    pub fn construct(mut self, source: Vec<Statement>) -> Cfg<Block, Link> {
        self.block(source);
        self.output.insert(Block::Empty);
        self.output
    }
}
