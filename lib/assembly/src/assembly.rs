use std::fmt::{self, Display, Formatter};

use crate::data::Data;

#[derive(Debug)]
pub enum Assembly {
    Label(u64),
    Mov(Data, Data),
    Cmp(Data, Data),
    Add(Data, Data),
    Sub(Data, Data),
    Imul(Data, Data),
    Idiv(Data),
    Cqo,
    Sete(Data),
    Setg(Data),
    Setl(Data),
    Jmp(u64),
    Je(u64),
    Call(String),
}

impl Display for Assembly {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            Self::Label(id) => {
                write!(f, "_{id}:")
            }
            Self::Mov(to, from) => {
                write!(f, "mov {to}, {from}")
            }
            Self::Cmp(first, second) => {
                write!(f, "cmp {first}, {second}")
            }
            Self::Add(to, value) => {
                write!(f, "add {to}, {value}")
            }
            Self::Sub(to, value) => {
                write!(f, "sub {to}, {value}")
            }
            Self::Imul(to, value) => {
                write!(f, "imul {to}, {value}")
            }
            Self::Idiv(data) => {
                write!(f, "idiv {data}")
            }
            Self::Cqo => {
                write!(f, "cqo")
            }
            Self::Sete(register) => {
                write!(f, "sete {register}")
            }
            Self::Setg(register) => {
                write!(f, "setg {register}")
            }
            Self::Setl(register) => {
                write!(f, "setl {register}")
            }
            Self::Jmp(label) => {
                write!(f, "jmp _{label}")
            }
            Self::Je(label) => {
                write!(f, "je _{label}")
            }
            Self::Call(label) => {
                write!(f, "call {label}")
            }
        }
    }
}
