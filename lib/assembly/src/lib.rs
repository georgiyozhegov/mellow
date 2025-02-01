mod instruction;
pub use instruction::Instruction;
use ir::cfg::Cfg;
use ir::Block;
use std::collections::HashMap;
use syntax::tree::Statement;

#[derive(Debug)]
pub struct Assembly {
    cfg: Cfg<Instruction>,
    variables: HashMap<String, u64>,
}

pub fn construct(mut cfg: Cfg<Statement>) -> Assembly {
    let mut to = 0;
    let mut variables = HashMap::new();
    let mut output = Vec::new();
    for block in cfg.blocks {
        let block = match block {
            Block::Basic(body) => {
                let mut instructions = Vec::new();
                for statement in body {
                    Instruction::statement(
                        statement, &mut to, &mut variables, &mut instructions);
                }
                Block::Basic(instructions)
            }
            Block::Empty => Block::Empty,
        };
        output.push(block);
    }
    let cfg = Cfg {
        blocks: output,
        links: cfg.links,
    };
    Assembly { cfg, variables }
}
