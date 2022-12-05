use std::fmt::{Debug, Display};

use crate::{core_::Pos, lang::get_errmeg};

pub struct ErrMeg {
    pub pos: Pos,
    pub err: Err,
}
impl ErrMeg {
    pub fn new(pos: Pos, err: Err) -> Self {
        Self { pos, err }
    }
}
impl Display for ErrMeg {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}", self.pos, self.err)
    }
}
impl Debug for ErrMeg {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_string())
    }
}
pub enum Err {
    Empty,

    UnknownEscapeCharacter,
    UnknowSymbol,
    UnknowKeyword,
    UnknowSyntax,
    Unmatched,

    NotVul,
    NotName,

    MissBra,
    MissVul,
    MissName,

    UseSet,
    UseDef,
    UseBlock,
    UseCallFn,

    IoNoArg,
    IoMissArg,
    IoUnknowArg,
    IoTooMuchArg,

    None,
}
impl Display for Err {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", get_errmeg(self))
    }
}
