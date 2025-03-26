use mellow_lex::{Lexer, Source, Result};
use mellow_parse::{Parser, Statement};

pub fn construct(source: Source) -> Result<Vec<Statement>> {
    let lexer = Lexer::new(source);
    let parser = Parser::new(lexer.peekable());
    parser.collect()
}
