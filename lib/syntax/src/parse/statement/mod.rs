mod let_;
mod assign;
mod if_;
mod while_;
mod for_;
mod debug;
pub use assign::Assign;
pub use let_::Let;
pub use if_::If;
pub use while_::While;
pub use for_::For;
pub use debug::Debug;

#[derive(Debug, Clone)]
pub enum Statement {
    Let(Let),
    Assign(Assign),
    If(If),
    While(While),
    For(For),
    Debug(Debug),
}
