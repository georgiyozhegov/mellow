mod let_;
mod assign;
mod if_;
mod while_;
mod for_;
mod debug;
pub use assign::*;
pub use let_::*;
pub use if_::*;
pub use while_::*;
pub use for_::*;
pub use debug::*;

#[derive(Debug, Clone)]
pub enum Statement {
    Let(Let),
    Assign(Assign),
    If(If),
    While(While),
    For(For),
    Debug(Debug),
}
