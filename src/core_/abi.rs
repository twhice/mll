use std::fmt::{Debug, Display};

use crate::error::Err;

use super::code::Expr;

type Name = Vec<char>;
#[derive(Clone)]
pub enum LogicCode {
    Set(Name, Name),
    /*
    jump -1 equal x false
    jump -1 notEqual x false
    jump -1 lessThan x false
    jump -1 lessThanEq x false
    jump -1 greaterThan x false
    jump -1 greaterThanEq x false
    jump -1 strictEqual x false
    jump -1 always x false
     */
    Jump(usize, Condition, Name, Name),
    /*
    op add result a b
    op sub result a b
    op mul result a b
    op div result a b
    op idiv result a b
    op mod result a b
    op pow result a b
    op equal result a b
    op notEqual result a b
    op land result a b
    op lessThan result a b
    op lessThanEq result a b
    op greaterThan result a b
    op greaterThanEq result a b
    op strictEqual result a b
    op shl result a b
    op shr result a b
    op or result a b
    op and result a b
    op xor result a b
    op not result a b
    op max result a b
    op min result a b
    op angle result a b
    op len result a b
    op noise result a b
    op abs result a b
    op log result a b
    op log10 result a b
    op floor result a b
    op ceil result a b
    op sqrt result a b
    op rand result a b
    op sin result a b
    op cos result a b
    op tan result a b
    op asin result a b
    op acos result a b
    op atan result a b
     */
    Op(Op, Name, Name, Name),
}

impl LogicCode {
    pub fn reset_target(&mut self, target: usize) {
        if let LogicCode::Jump(_, o, l, r) = self {
            *self = LogicCode::Jump(target, *o, l.clone(), r.clone())
        }
    }
    pub fn get_target(&self) -> usize {
        if let LogicCode::Jump(target, _, _, _) = self {
            return *target;
        }
        0
    }
}
#[derive(Clone, Copy)]
pub enum Condition {
    Eq,
    Seq,
    Greater,
    Less,
    NotEq,
    NotGreater,
    NotLess,
    Always,
}
impl Display for Condition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Eq => write!(f, "equal"),
            Self::Seq => write!(f, "strictEqual"),
            Self::Greater => write!(f, "greaterThan"),
            Self::Less => write!(f, "lessThan"),
            Self::NotEq => write!(f, "notEquale"),
            Self::NotGreater => write!(f, "lessThanEq"),
            Self::NotLess => write!(f, "greaterThanEq"),
            Self::Always => write!(f, "always"),
        }
    }
}
impl Debug for Condition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_string())
    }
}
#[derive(Clone, Copy)]
pub enum Op {
    Add,
    Sub,
    Mul,
    Div,
    Idiv,
    Mod,
    Pow,
    Equal,
    NotEqual,
    Land,
    LessThan,
    LessThanEq,
    GreaterThan,
    GreaterThanEq,
    StrictEqual,
    Shl,
    Shr,
    Or,
    And,
    Xor,
    Not,
    Max,
    Min,
    Angle,
    Len,
    Noise,
    Abs,
    Log,
    Log10,
    Floor,
    Ceil,
    Sqrt,
    Rand,
    Sin,
    Cos,
    Tan,
    Asin,
    Acos,
    Atan,
}
impl Display for Op {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Op::Add => "add",
                Op::Sub => "sub",
                Op::Mul => "mul",
                Op::Div => "div",
                Op::Idiv => "idiv",
                Op::Mod => "mod",
                Op::Pow => "pow",
                Op::Equal => "equal",
                Op::NotEqual => "notEqual",
                Op::Land => "land",
                Op::LessThan => "lessThan",
                Op::LessThanEq => "lessThanEq",
                Op::GreaterThan => "greaterThan",
                Op::GreaterThanEq => "greaterThanEq",
                Op::StrictEqual => "strictEqual",
                Op::Shl => "shl",
                Op::Shr => "shr",
                Op::Or => "or",
                Op::And => "and",
                Op::Xor => "xor",
                Op::Not => "not",
                Op::Max => "max",
                Op::Min => "min",
                Op::Angle => "angle",
                Op::Len => "len",
                Op::Noise => "noise",
                Op::Abs => "abs",
                Op::Log => "log",
                Op::Log10 => "log10",
                Op::Floor => "floor",
                Op::Ceil => "ceil",
                Op::Sqrt => "sqrt",
                Op::Rand => "rand",
                Op::Sin => "sin",
                Op::Cos => "cos",
                Op::Tan => "tan",
                Op::Asin => "asin",
                Op::Acos => "acos",
                Op::Atan => "atan",
            }
        )
    }
}
impl Debug for Op {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_string())
    }
}
impl TryFrom<Op> for Condition {
    type Error = Err;

    fn try_from(value: Op) -> Result<Self, Self::Error> {
        Ok(match value {
            Op::Equal => Self::Eq,
            Op::NotEqual => Self::NotEq,
            Op::LessThan => Self::Less,
            Op::LessThanEq => Self::NotGreater,
            Op::GreaterThan => Self::Greater,
            Op::GreaterThanEq => Self::NotLess,
            Op::StrictEqual => Self::Seq,
            _ => return Err(Err::None),
        })
    }
}
impl From<Vec<char>> for Op {
    fn from(vec: Vec<char>) -> Self {
        let match_text = |text: &str| -> bool { text.chars().collect::<Vec<char>>() == vec };
        return if match_text("+") {
            Self::Add
        } else if match_text("-") {
            Self::Sub
        } else if match_text("*") {
            Self::Mul
        } else if match_text("**") {
            Self::Pow
        } else if match_text("/") {
            Self::Div
        } else if match_text("//") {
            Self::Idiv
        } else if match_text("^") {
            //
            Self::Sub
        } else if match_text("%") {
            Self::Mod
        } else if match_text("<") {
            Self::LessThan
        } else if match_text(">") {
            Self::GreaterThan
        } else if match_text("<=") {
            Self::LessThanEq
        } else if match_text(">=") {
            Self::GreaterThanEq
        } else if match_text(">>") {
            Self::Shr
        } else if match_text("<<") {
            Self::Shl
        } else if match_text("|") {
            //
            Self::Sub
        } else if match_text("||") {
            Self::Or
        } else if match_text("&") {
            //
            Self::Abs
        } else if match_text("&&") {
            Self::And
        } else if match_text("===") {
            Self::StrictEqual
        } else if match_text("==") {
            Self::Equal
        } else {
            todo!()
        };
    }
}
impl From<&Expr> for Op {
    fn from(value: &Expr) -> Self {
        match value {
            Expr::Op(vec) => Self::from(vec.clone()),
            _ => todo!(),
        }
    }
}
