use std::fmt::{self, Display};

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum RegisterKind {
    A,
    B,
    C,
    D,
    Sp,
    Bp,
    Si,
    Di,
    R8,
    R9,
    R10,
    R11,
    R12,
    R13,
    R14,
    R15,
}

impl RegisterKind {
    pub fn all() -> Vec<Self> {
        use RegisterKind::*;
        vec![
            A, B, C, D, Sp, Bp, Si, Di, R8, R9, R10, R11, R12, R13, R14, R15,
        ]
    }

    pub fn allocable() -> Vec<Self> {
        use RegisterKind::*;
        vec![B, C, Si, Di, R8, R9, R10, R11, R12, R13, R14, R15]
    }
}

#[derive(Debug, Clone)]
pub enum Size {
    Byte = 8,
    Word = 16,
    Dword = 32,
    Qword = 64,
}

#[derive(Debug, Clone)]
pub struct Register {
    kind: RegisterKind,
    size: Size,
}

impl Register {
    pub fn new(kind: RegisterKind, size: Size) -> Self {
        Self { kind, size }
    }
}

impl Display for Register {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let kind = match self.kind {
            RegisterKind::A => "a",
            RegisterKind::B => "b",
            RegisterKind::C => "c",
            RegisterKind::D => "d",
            RegisterKind::Sp => "sp",
            RegisterKind::Bp => "bp",
            RegisterKind::Si => "si",
            RegisterKind::Di => "di",
            RegisterKind::R8 => "r8",
            RegisterKind::R9 => "r9",
            RegisterKind::R10 => "r10",
            RegisterKind::R11 => "r11",
            RegisterKind::R12 => "r12",
            RegisterKind::R13 => "r13",
            RegisterKind::R14 => "r14",
            RegisterKind::R15 => "r15",
        };
        let (size_prefix, size_suffix) = match self.kind {
            RegisterKind::A | RegisterKind::B | RegisterKind::C | RegisterKind::D => {
                match self.size {
                    Size::Byte => ("", "l"),
                    Size::Word => ("", "x"),
                    Size::Dword => ("e", "x"),
                    Size::Qword => ("r", "x"),
                }
            }
            RegisterKind::Sp | RegisterKind::Bp | RegisterKind::Si | RegisterKind::Di => {
                match self.size {
                    Size::Byte => ("", "l"),
                    Size::Word => ("", ""),
                    Size::Dword => ("e", ""),
                    Size::Qword => ("r", ""),
                }
            }

            RegisterKind::R8
            | RegisterKind::R9
            | RegisterKind::R10
            | RegisterKind::R11
            | RegisterKind::R12
            | RegisterKind::R13
            | RegisterKind::R14
            | RegisterKind::R15 => match self.size {
                Size::Byte => ("", "b"),
                Size::Word => ("", "w"),
                Size::Dword => ("", "d"),
                Size::Qword => ("", ""),
            },
        };
        write!(f, "{size_prefix}{kind}{size_suffix}")
    }
}
