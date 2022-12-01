use std::fmt::{Debug, Display};

use crate::core_::Pos;

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
    UnknowSyntax,
    Unmatched,
    Empty,
    NotVul,
    MissBra,
    MissVul,
}
impl Display for Err {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let meg = match crate::LANGUAGE {
            crate::lang::Language::Chinese => match self {
                Err::UnknownEscapeCharacter => "未知的转义字符",
                Err::UnknowSymbol => "未知运算符",
                Err::UnknowSyntax => "未知语法",
                Err::UnknowKeyword => "未知关键字",
                Err::Empty => "缺少...",
                Err::Unmatched => "不合语法",
                Err::NotVul => "它不可以作为值",
                Err::MissBra => "缺少括号",
                Err::MissVul => "缺少值",
            },
            crate::lang::Language::English => todo!(),
        };
        write!(f, "{}", meg)
    }
}
