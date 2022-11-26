use std::fmt::{Debug, Display};

use crate::core::Pos;

pub struct ErrMeg {
    pos: Pos,
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
    CommentforError,
    UnknownEscapeCharacter,
    UnknowSymbol,
    UnknowKeyword,
    Unmatched,
    Empty,
    NotVul,
}
impl Display for Err {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let meg = match crate::LANGUAGE {
            crate::lang::Language::Chinese => match self {
                Err::CommentforError => "你可能在尝试编写一个注释 正确的注释为:\n// 注释",
                Err::UnknownEscapeCharacter => "未知的转义字符",
                Err::UnknowSymbol => "未知运算符",
                Err::UnknowKeyword => "未知关键字",
                Err::Empty => "空",
                Err::Unmatched => "不匹配",
                Err::NotVul => "它不可以作为值",
            },
            crate::lang::Language::English => todo!(),
        };
        write!(f, "{}", meg)
    }
}
