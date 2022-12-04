use std::fmt::{Debug, Display};

use crate::{core_::Pos, lang::get_errmeg};

pub struct ErrMeg {
    pub pos: Pos,
    err: Err,
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
    UnknownEscapeCharacter,
    UnknowSymbol,
    UnknowKeyword,
    Unmatched,
    Empty,
    NotVul,
    MissBra,
    MissVul,

    IoMissArg,
    IoUnknowArg,

    IoNoArg,
    IoTooMuchArg,
}
impl Display for Err {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", get_errmeg(self))
    }
}
