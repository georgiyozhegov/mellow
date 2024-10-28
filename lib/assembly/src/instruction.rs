#[derive(Debug)]
pub enum Size {
    Byte = 8,
    Word = 16,
    DoubleWord = 32,
    QuadWord = 64,
}

#[derive(Debug)]
pub enum Storage {
    Integer(i128),
    Register { index: u64, size: Size },
    Stack { offset: u64, size: Size },
}

#[derive(Debug)]
pub enum Instruction {
    Move { to: Storage, from: Storage },
}

pub type Block = Vec<Instruction>;
