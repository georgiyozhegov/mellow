use crate::{
    cfg::{Cfg, Link},
    convert::Convert,
    Block, Instruction,
};

pub fn construct(cfg: Cfg<Block, Link>) -> Vec<Instruction> {
    let convert = Convert::new();
    convert.construct(cfg)
}
