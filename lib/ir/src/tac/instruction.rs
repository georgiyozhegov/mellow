use std::fmt::{self, Display, Formatter};

#[derive(Debug)]
pub enum Instruction {
    Label(u64),
    Integer { to: u64, value: i128 },
    Get { to: u64, identifier: String },
    Set { identifier: String, from: u64 },
    String { to: u64, value: String },
    Add { to: u64, left: u64, right: u64 },
    Subtract { to: u64, left: u64, right: u64 },
    Multiply { to: u64, left: u64, right: u64 },
    Divide { to: u64, left: u64, right: u64 },
    Greater { to: u64, left: u64, right: u64 },
    Less { to: u64, left: u64, right: u64 },
    Equal { to: u64, left: u64, right: u64 },
    Jump(u64),
    JumpIf { condition: u64, to: u64 },
    Call { label: String, value: u64 },
}

impl Display for Instruction {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            Self::Label(id) => {
                write!(f, "@{id}")
            }
            Self::Integer { to, value } => {
                write!(f, "#{to} int {value}")
            }
            Self::Get { to, identifier } => {
                write!(f, "#{to} get ${identifier}")
            }
            Self::Set { identifier, from } => {
                write!(f, "${identifier} set #{from}")
            }
            Self::String { to, value } => {
                write!(f, "${to} str \"{value}\"")
            }
            Self::Add { to, left, right } => {
                write!(f, "#{to} add #{left} #{right}")
            }
            Self::Subtract { to, left, right } => {
                write!(f, "#{to} sub #{left} #{right}")
            }
            Self::Multiply { to, left, right } => {
                write!(f, "#{to} mul #{left} #{right}")
            }
            Self::Divide { to, left, right } => {
                write!(f, "#{to} div #{left} #{right}")
            }
            Self::Greater { to, left, right } => {
                write!(f, "#{to} gt #{left} #{right}")
            }
            Self::Less { to, left, right } => {
                write!(f, "#{to} lt #{left} #{right}")
            }
            Self::Equal { to, left, right } => {
                write!(f, "#{to} eq #{left} #{right}")
            }
            Self::Jump(label) => {
                write!(f, "jump @{label}")
            }
            Self::JumpIf { condition, to } => {
                write!(f, "jump @{to} if #{condition}")
            }
            Self::Call { label, value } => {
                write!(f, "call {label} #{value}")
            }
        }
    }
}
