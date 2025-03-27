use mellow_lex::{Lexer, Source};
use mellow_parse::{Parser, Statement};
use mellow_error::Result;

pub fn construct(source: Source) -> Result<Vec<Statement>> {
    let lexer = Lexer::new(source);
    let parser = Parser::new(lexer.peekable());
    parser.collect()
}
