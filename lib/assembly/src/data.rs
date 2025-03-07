use std::fmt::{self, Display, Formatter};

use crate::Register;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Data {
    Register(Register),
    Stack(u8),
    Integer(i128),
    Identifier(String), // NOTE: Temporary, will be removed
}

impl Display for Data {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            Self::Register(register) => write!(f, "{register}"),
            Self::Stack(offset) => write!(f, "[rsp - {offset}]"),
            Self::Integer(value) => write!(f, "{value}"),
            Self::Identifier(identifier) => write!(f, "[{identifier}]"),
        }
    }
}
