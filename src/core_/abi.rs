use super::code::Expr;
use super::complier::VulType;
use crate::error::{CTErr, Err};
use crate::ToString;
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
    Sensor(Name, Name, Senseabled),

    UnitBind(Name),
    UnitControl(Name, Name, Name, Name, Name),
    UnitLocate(Target, Name, Name, Name, Name, Name),

    QuickilyAdded(Name),
}
impl Display for LogicCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            LogicCode::Set(l, r) => write!(f, "set {} {}", l.to_string(), r.to_string()),
            LogicCode::Jump(t, c, l, r) => write!(
                f,
                "jump {t} {} {} {}",
                c.to_string(),
                l.to_string(),
                r.to_string()
            ),
            LogicCode::Op(o, t, l, r) => write!(
                f,
                "op {} {} {} {}",
                o.to_string(),
                t.to_string(),
                l.to_string(),
                r.to_string()
            ),
            LogicCode::GetLink(r, i) => write!(f, "getlink {} {}", r.to_string(), i.to_string()),
            LogicCode::Sensor(r, n, s) => write!(
                f,
                "sensor {} {} {}",
                r.to_string(),
                n.to_string(),
                s.to_string()
            ),
            LogicCode::UnitBind(t) => write!(f, "ubind {}", t.to_string()),
            LogicCode::UnitControl(a, b, c, d, e) => write!(
                f,
                "ucontrol {} {} {} {} {}",
                a.to_string(),
                b.to_string(),
                c.to_string(),
                d.to_string(),
                e.to_string(),
            ),
            LogicCode::UnitLocate(q, e, x, y, find, b) => write!(
                f,
                "ulocate {} {} @copper {} {} {} {}",
                q.to_string(),
                e.to_string(),
                x.to_string(),
                y.to_string(),
                find.to_string(),
                b.to_string(),
            ),
            LogicCode::QuickilyAdded(cmd) => write!(f, "{}", cmd.to_string()),
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
        } else if match_text("!") {
            Self::Not
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
pub enum Senseabled {
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
impl Display for Senseabled {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match &self {
                Senseabled::TotalItem => "@totalItem".to_owned(),
                Senseabled::FirstItem => "@firstItem".to_owned(),
                Senseabled::TotalLiquids => "@totalLiquids".to_owned(),
                Senseabled::TotalPower => "@totalPower".to_owned(),
                Senseabled::ItemCapacity => "@itemCapacity".to_owned(),
                Senseabled::LiquidCapacity => "@liquidCapacity".to_owned(),
                Senseabled::PowerCapacity => "@powerCapacity".to_owned(),
                Senseabled::PowerNetStored => "@powerNetStored".to_owned(),
                Senseabled::PowerNetCapacity => "@powerNetCapacity".to_owned(),
                Senseabled::PowerNetIn => "@powerNetIn".to_owned(),
                Senseabled::PowerNetOut => "@powerNetOut".to_owned(),
                Senseabled::Ammo => "@ammo".to_owned(),
                Senseabled::AmmoCapacity => "@ammoCapacity".to_owned(),
                Senseabled::Health => "@health".to_owned(),
                Senseabled::MacHealth => "@macHealth".to_owned(),
                Senseabled::Heat => "@heat".to_owned(),
                Senseabled::Efficiency => "@efficiency".to_owned(),
                Senseabled::Prograss => "@prograss".to_owned(),
                Senseabled::Timescale => "@timescale".to_owned(),
                Senseabled::Rotation => "@rotation".to_owned(),
                Senseabled::X => "@x".to_owned(),
                Senseabled::Y => "@y".to_owned(),
                Senseabled::ShootX => "@shootX".to_owned(),
                Senseabled::ShootY => "@shootY".to_owned(),
                Senseabled::Size => "@size".to_owned(),
                Senseabled::Dead => "@dead".to_owned(),
                Senseabled::Range => "@range".to_owned(),
                Senseabled::Shooting => "@shooting".to_owned(),
                Senseabled::Boosting => "@boosting".to_owned(),
                Senseabled::MineX => "@mineX".to_owned(),
                Senseabled::MineY => "@mineY".to_owned(),
                Senseabled::Mining => "@mining".to_owned(),
                Senseabled::Speed => "@speed".to_owned(),
                Senseabled::Team => "@team".to_owned(),
                Senseabled::Typeflag => "@typeflag".to_owned(),
                Senseabled::Controlled => "@controlled".to_owned(),
                Senseabled::Controller => "@controller".to_owned(),
                Senseabled::Name => "@name".to_owned(),
                Senseabled::PayloadCount => "@payloadCount".to_owned(),
                Senseabled::PayloadType => "@payloadType".to_owned(),
                Senseabled::Enabled => "@enabled".to_owned(),
                Senseabled::Config => "@config".to_owned(),
                Senseabled::Color => "@color".to_owned(),
                Senseabled::Other(vec) => {
                    unsafe {
                        if crate::DEBUG {
                            CTErr::UnknowConst(vec.clone()).solve();
                        }
                    }
                    vec.to_string()
                }
            }
        )
    }
}
impl From<Vec<char>> for Senseabled {
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
impl From<&Expr> for Senseabled {
    fn from(value: &Expr) -> Self {
        match value {
            Expr::Op(vec) => Self::from(vec.clone()),
            Expr::Data(vec) => Self::from(vec.clone()),
            _ => todo!("你发现了Bug,速速上报!"),
        }
    }
}
impl Type for Senseabled {
    fn get_type(&self) -> VulType {
        match &self {
            Senseabled::TotalItem
            | Senseabled::FirstItem
            | Senseabled::TotalLiquids
            | Senseabled::TotalPower
            | Senseabled::ItemCapacity
            | Senseabled::LiquidCapacity
            | Senseabled::PowerCapacity
            | Senseabled::PowerNetStored
            | Senseabled::PowerNetCapacity
            | Senseabled::PowerNetIn
            | Senseabled::PowerNetOut
            | Senseabled::Ammo
            | Senseabled::AmmoCapacity
            | Senseabled::Health
            | Senseabled::Heat
            | Senseabled::MacHealth
            | Senseabled::X
            | Senseabled::Y
            | Senseabled::Timescale
            | Senseabled::Rotation
            | Senseabled::ShootX
            | Senseabled::ShootY
            | Senseabled::Size
            | Senseabled::Dead
            | Senseabled::Range
            | Senseabled::Shooting
            | Senseabled::Boosting
            | Senseabled::MineX
            | Senseabled::MineY
            | Senseabled::Mining
            | Senseabled::Speed
            | Senseabled::PayloadCount
            | Senseabled::Controlled => VulType::Basic,
            Senseabled::Enabled => todo!(),
            Senseabled::Efficiency => todo!(),  // ?
            Senseabled::Prograss => todo!(),    // ?
            Senseabled::Team => todo!(),        // ?
            Senseabled::Typeflag => todo!(),    // ?
            Senseabled::Name => todo!(),        // ?
            Senseabled::PayloadType => todo!(), // const
            Senseabled::Config => VulType::Senseabled,
            Senseabled::Controller => VulType::Unit,
            Senseabled::Color => VulType::Color,     // Color
            Senseabled::Other(_) => VulType::Unknow, // ?
        }
    }
}
#[derive(Clone)]
pub enum Target {
    Core,
    Other(Name),
}
impl Display for Target {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Target::Core => write!(f, "building core"),
            Target::Other(n) => write!(f, "{}", n.to_string()),
        }
    }
}
impl From<Vec<char>> for Target {
    fn from(value: Vec<char>) -> Self {
        let match_text = |text: &str| -> bool {
            (value.len() == text.len()) && (value == text.chars().collect::<Vec<char>>())
        };
        return if match_text("core") {
            Self::Core
        } else {
            Self::Other(value)
        };
    }
}
impl From<&Expr> for Target {
    fn from(value: &Expr) -> Self {
        match value {
            Expr::Op(vec) => Self::from(vec.clone()),
            Expr::Data(vec) => Self::from(vec.clone()),
            _ => todo!("你发现了Bug,速速上报!"),
        }
    }
}

pub enum UnitType {
    // sla
    Dagger,
    Mace,
    Fortress,
    Scepter,
    Reign,
    // slb
    Nova,
    Pulsar,
    Quasar,
    Vela,
    Corvus,
    // slc
    Crawler,
    Atrax,
    Spiroct,
    Arkyid,
    Toxopid,
    // saa
    Flare,
    Horzion,
    Zenith,
    Autumbra,
    Eclipse,
    // sab
    Momo,
    Poly,
    Mega,
    Quad,
    Oct,
    // ssa
    Risso,
    Minke,
    Bryde,
    Sei,
    Omura,
    // ssb
    Retusa,
    Oxynoe,
    Cyerce,
    Aegires,
    Navanax,
    // sac
    Alpha,
    Beta,
    Gamma,
    // et
    Stell,
    Locus,
    Precept,
    Vanquish,
    Conquer,
    // es
    Merui,
    Cleroi,
    Anthicus,
    Tecta,
    Collaris,
    // ea
    Elude,
    Avert,
    Obviate,
    Quell,
    Disrupt,
    // ec
    Evoke,
    Incite,
    Emanate,
}
impl From<Vec<char>> for UnitType {
    fn from(value: Vec<char>) -> Self {
        todo!()
    }
}
impl From<&Expr> for UnitType {
    fn from(value: &Expr) -> Self {
        match value {
            Expr::Op(vec) => Self::from(vec.clone()),
            Expr::Data(vec) => Self::from(vec.clone()),
            _ => todo!("你发现了Bug,速速上报!"),
        }
    }
}
impl Type for UnitType {
    fn get_type(&self) -> VulType {
        VulType::UnitType
    }
}
// 类型系统
pub trait Type {
    fn get_type(&self) -> VulType;
}
