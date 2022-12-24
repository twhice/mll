use super::super::lang::vec_to_str;
use super::code::Expr;
use crate::error::{CTErr, Err};
use std::fmt::{Debug, Display};

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
    // getlink result 0
    GetLink(Name, Name),
    // sensor result block1 @copper
    Sensor(Name, Name, Sensor),
}
impl Display for LogicCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LogicCode::Set(l, r) => write!(f, "set {} {}", vec_to_str(l), vec_to_str(r)),
            LogicCode::Jump(t, c, l, r) => write!(
                f,
                "jump {t} {} {} {}",
                c.to_string(),
                vec_to_str(l),
                vec_to_str(r)
            ),
            LogicCode::Op(o, t, l, r) => write!(
                f,
                "op {} {} {} {}",
                o.to_string(),
                vec_to_str(t),
                vec_to_str(l),
                vec_to_str(r)
            ),
            LogicCode::GetLink(r, i) => write!(f, "getlink {} {}", vec_to_str(r), vec_to_str(i)),
            LogicCode::Sensor(r, n, s) => write!(
                f,
                "sensor {} {} {}",
                vec_to_str(r),
                vec_to_str(n),
                s.to_string()
            ),
        }
    }
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
    pub fn rename_result(&mut self, new_name: Name) {
        match self {
            LogicCode::Set(_, v) => *self = LogicCode::Set(new_name, v.clone()),
            LogicCode::Op(o, _, l, r) => *self = LogicCode::Op(*o, new_name, l.clone(), r.clone()),
            LogicCode::GetLink(_, i) => *self = LogicCode::GetLink(new_name, i.clone()),
            LogicCode::Sensor(_, n, s) => *self = LogicCode::Sensor(new_name, n.clone(), s.clone()),
            _ => {}
        }
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
            Self::NotEq => write!(f, "notEqual"),
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
    Sensor,
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
                Op::Sensor => ".",
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
        } else if match_text("!=") {
            Self::NotEqual
        } else if match_text(".") {
            Self::Sensor
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
#[derive(Clone)]
pub enum Sensor {
    TotalItem,
    FirstItem,
    TotalLiquids,
    TotalPower,
    ItemCapacity,
    LiquidCapacity,
    PowerCapacity,
    PowerNetStored,
    PowerNetCapacity,
    PowerNetIn,
    PowerNetOut,
    Ammo,
    AmmoCapacity,
    Health,
    MacHealth,
    Heat,
    Efficiency,
    Prograss,
    Timescale,
    Rotation,
    X,
    Y,
    ShootX,
    ShootY,
    Size,
    Dead,
    Range,
    Shooting,
    Boosting,
    MineX,
    MineY,
    Mining,
    Speed,
    Team,
    Typeflag,
    Controlled,
    Controller,
    Name,
    PayloadCount,
    PayloadType,
    Enabled,
    Config,
    Color,
    Other(Name),
}
impl Display for Sensor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match &self {
                Sensor::TotalItem => "@totalItem".to_owned(),
                Sensor::FirstItem => "@firstItem".to_owned(),
                Sensor::TotalLiquids => "@totalLiquids".to_owned(),
                Sensor::TotalPower => "@totalPower".to_owned(),
                Sensor::ItemCapacity => "@itemCapacity".to_owned(),
                Sensor::LiquidCapacity => "@liquidCapacity".to_owned(),
                Sensor::PowerCapacity => "@powerCapacity".to_owned(),
                Sensor::PowerNetStored => "@powerNetStored".to_owned(),
                Sensor::PowerNetCapacity => "@powerNetCapacity".to_owned(),
                Sensor::PowerNetIn => "@powerNetIn".to_owned(),
                Sensor::PowerNetOut => "@powerNetOut".to_owned(),
                Sensor::Ammo => "@ammo".to_owned(),
                Sensor::AmmoCapacity => "@ammoCapacity".to_owned(),
                Sensor::Health => "@health".to_owned(),
                Sensor::MacHealth => "@macHealth".to_owned(),
                Sensor::Heat => "@heat".to_owned(),
                Sensor::Efficiency => "@efficiency".to_owned(),
                Sensor::Prograss => "@prograss".to_owned(),
                Sensor::Timescale => "@timescale".to_owned(),
                Sensor::Rotation => "@rotation".to_owned(),
                Sensor::X => "@x".to_owned(),
                Sensor::Y => "@y".to_owned(),
                Sensor::ShootX => "@shootX".to_owned(),
                Sensor::ShootY => "@shootY".to_owned(),
                Sensor::Size => "@size".to_owned(),
                Sensor::Dead => "@dead".to_owned(),
                Sensor::Range => "@range".to_owned(),
                Sensor::Shooting => "@shooting".to_owned(),
                Sensor::Boosting => "@boosting".to_owned(),
                Sensor::MineX => "@mineX".to_owned(),
                Sensor::MineY => "@mineY".to_owned(),
                Sensor::Mining => "@mining".to_owned(),
                Sensor::Speed => "@speed".to_owned(),
                Sensor::Team => "@team".to_owned(),
                Sensor::Typeflag => "@typeflag".to_owned(),
                Sensor::Controlled => "@controlled".to_owned(),
                Sensor::Controller => "@controller".to_owned(),
                Sensor::Name => "@name".to_owned(),
                Sensor::PayloadCount => "@payloadCount".to_owned(),
                Sensor::PayloadType => "@payloadType".to_owned(),
                Sensor::Enabled => "@enabled".to_owned(),
                Sensor::Config => "@config".to_owned(),
                Sensor::Color => "@color".to_owned(),
                Sensor::Other(vec) => {
                    unsafe {
                        if crate::DEBUG {
                            CTErr::UnknowConst(vec.clone()).solve();
                        }
                    }
                    vec_to_str(vec)
                }
            }
        )
    }
}
impl From<Vec<char>> for Sensor {
    fn from(value: Vec<char>) -> Self {
        let match_text = |text: &str| -> bool {
            (value.len() == text.len()) && (value == text.chars().collect::<Vec<char>>())
        };
        return if match_text("totalItem") {
            Self::TotalItem
        } else if match_text("firstItem") {
            Self::FirstItem
        } else if match_text("totalLiquids") {
            Self::TotalLiquids
        } else if match_text("totalPower") {
            Self::TotalPower
        } else if match_text("itemCapacity") {
            Self::ItemCapacity
        } else if match_text("liquidCapacity") {
            Self::LiquidCapacity
        } else if match_text("powerCapacity") {
            Self::PowerCapacity
        } else if match_text("powerNetStored") {
            Self::PowerNetStored
        } else if match_text("powerNetCapacity") {
            Self::PowerNetCapacity
        } else if match_text("powerNetIn") {
            Self::PowerNetIn
        } else if match_text("powerNetOut") {
            Self::PowerNetOut
        } else if match_text("ammo") {
            Self::Ammo
        } else if match_text("ammoCapacity") {
            Self::AmmoCapacity
        } else if match_text("health") {
            Self::Health
        } else if match_text("macHealth") {
            Self::MacHealth
        } else if match_text("heat") {
            Self::Heat
        } else if match_text("efficiency") {
            Self::Efficiency
        } else if match_text("prograss") {
            Self::Prograss
        } else if match_text("timescale") {
            Self::Timescale
        } else if match_text("rotation") {
            Self::Rotation
        } else if match_text("x") {
            Self::X
        } else if match_text("y") {
            Self::Y
        } else if match_text("shootX") {
            Self::ShootX
        } else if match_text("shootY") {
            Self::ShootY
        } else if match_text("size") {
            Self::Size
        } else if match_text("dead") {
            Self::Dead
        } else if match_text("range") {
            Self::Range
        } else if match_text("shooting") {
            Self::Shooting
        } else if match_text("boosting") {
            Self::Boosting
        } else if match_text("mineX") {
            Self::MineX
        } else if match_text("mineY") {
            Self::MineY
        } else if match_text("mining") {
            Self::Mining
        } else if match_text("speed") {
            Self::Speed
        } else if match_text("team") {
            Self::Team
        } else if match_text("typeflag") {
            Self::Typeflag
        } else if match_text("controlled") {
            Self::Controlled
        } else if match_text("controller") {
            Self::Controller
        } else if match_text("name") {
            Self::Name
        } else if match_text("payloadCount") {
            Self::PayloadCount
        } else if match_text("payloadType") {
            Self::PayloadType
        } else if match_text("enabled") {
            Self::Enabled
        } else if match_text("config") {
            Self::Config
        } else if match_text("color") {
            Self::Color
        } else {
            Self::Other(value)
        };
    }
}
impl From<&Expr> for Sensor {
    fn from(value: &Expr) -> Self {
        match value {
            Expr::Op(vec) => Self::from(vec.clone()),
            Expr::Data(vec) => Self::from(vec.clone()),
            _ => todo!("你发现了Bug,速速上报!"),
        }
    }
}
