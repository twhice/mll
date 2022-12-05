use std::fmt::{Debug, Display};

use crate::error::Err;

type Name = Vec<char>;
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
                Add => "add",
                Sub => "sub",
                Mul => "mul",
                Div => "div",
                Idiv => "idiv",
                Mod => "mod",
                Pow => "pow",
                Equal => "equal",
                NotEqual => "notEqual",
                Land => "land",
                LessThan => "lessThan",
                LessThanEq => "lessThanEq",
                GreaterThan => "greaterThan",
                GreaterThanEq => "greaterThanEq",
                StrictEqual => "strictEqual",
                Shl => "shl",
                Shr => "shr",
                Or => "or",
                And => "and",
                Xor => "xor",
                Not => "not",
                Max => "max",
                Min => "min",
                Angle => "angle",
                Len => "len",
                Noise => "noise",
                Abs => "abs",
                Log => "log",
                Log10 => "log10",
                Floor => "floor",
                Ceil => "ceil",
                Sqrt => "sqrt",
                Rand => "rand",
                Sin => "sin",
                Cos => "cos",
                Tan => "tan",
                Asin => "asin",
                Acos => "acos",
                Atan => "atan",
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
        } else {
            todo!()
        };
    }
}
