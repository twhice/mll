use std::fmt::Debug;

use super::Token;
pub enum ComUnit {
    Set(Vec<Set>),
    Ctrl(Ctrl),
}
impl Debug for ComUnit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ComUnit::Set(set) => write!(f, "{:?}", set),
            ComUnit::Ctrl(ctrl) => write!(f, "{:?}", ctrl),
        }
    }
}

pub struct Set {
    lv: Vec<char>,
    rv: Expr,
}
impl Debug for Set {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut lvts = String::new();
        for c in &self.lv {
            lvts += String::from(*c).as_str();
        }
        write!(f, "[SET {lvts} = {rv:?} ]", rv = self.rv)
    }
}

impl Set {
    pub fn new(lv: Vec<char>, rv: Expr) -> Self {
        Self { lv, rv }
    }
}
#[derive(Debug)]
pub enum Ctrl {
    CtrlIf(CtrlIf),
    CtrlDef(CtrlDef),
    CtrlWhile(CtrlWhile),
    CtrlSwitch(CtrlSwitch),
    CtrlReturn(CtrlReturn),
}

pub struct CtrlIf {
    condition: Condition,
    if_statement: Vec<ComUnit>,
    elifs: Vec<(Condition, Vec<ComUnit>)>,
    else_statement: Vec<ComUnit>,
}
impl Debug for CtrlIf {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut all = String::new();
        all += &format!("{:?}", self.condition);
        all += &cus_to_str(&self.if_statement);

        for elif in &self.elifs {
            all += &format!("{:?}", elif.0);
            all += &cus_to_str(&elif.1);
        }
        if self.else_statement.len() != 0 {
            all += &cus_to_str(&self.else_statement);
        }
        write!(f, "{all}")
    }
}
impl CtrlIf {
    pub fn new(
        condition: Condition,
        if_statement: Vec<ComUnit>,
        elifs: Vec<(Condition, Vec<ComUnit>)>,
        else_statement: Vec<ComUnit>,
    ) -> Self {
        Self {
            condition,
            if_statement,
            elifs,
            else_statement,
        }
    }
}

#[derive(Debug)]
pub struct CtrlDef {
    fn_name: Vec<char>,
    args: Vec<Vec<char>>,
    statement: Vec<ComUnit>,
}

impl CtrlDef {
    pub fn new(fn_name: Vec<char>, args: Vec<Vec<char>>, statement: Vec<ComUnit>) -> Self {
        Self {
            fn_name,
            args,
            statement,
        }
    }
}
#[derive(Debug)]
pub struct CtrlWhile {
    condition: Condition,
    statements: Vec<ComUnit>,
}

impl CtrlWhile {
    pub fn new(condition: Condition, statements: Vec<ComUnit>) -> Self {
        Self {
            condition,
            statements,
        }
    }
}

#[derive(Debug)]
pub struct CtrlSwitch {
    condition: Expr,
    cases: Vec<Vec<ComUnit>>,
}

impl CtrlSwitch {
    pub fn new(condition: Expr, cases: Vec<Vec<ComUnit>>) -> Self {
        Self { condition, cases }
    }
}

#[derive(Debug)]
pub struct CtrlReturn {
    return_vul: Expr,
}

impl CtrlReturn {
    pub fn new(return_vul: Expr) -> Self {
        Self { return_vul }
    }
}
#[derive(Clone)]
pub enum Expr {
    Eoe(Box<Expr>, Box<Expr>, Box<Expr>),
    Eo(Box<Expr>, Vec<char>),
    Oe(Box<Expr>, Box<Expr>),
    Data(Vec<char>),
    Op(Vec<char>),
    CallFn(Vec<char>, Vec<Vec<char>>),
}
impl Expr {
    pub fn is_right_part(&self) -> bool {
        if let Expr::Op(op) = self {
            *op == ")".chars().collect::<Vec<char>>()
        } else {
            false
        }
    }
    pub fn is_left_part(&self) -> bool {
        if let Expr::Op(op) = self {
            *op == "(".chars().collect::<Vec<char>>()
        } else {
            false
        }
    }
    pub fn is_not(&self) -> bool {
        if let Expr::Op(op) = self {
            *op == "!".chars().collect::<Vec<char>>()
        } else {
            false
        }
    }
}
impl Debug for Expr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Eoe(arg0, arg1, arg2) => write!(f, "({:?}{:?}{:?})", arg0, arg1, arg2),
            Self::Eo(arg0, arg1) => write!(f, "({:?}{:?})", arg0, arg1),
            Self::Oe(arg0, arg1) => write!(f, "({:?}{:?})", arg0, arg1),
            Self::Data(arg0) => {
                let mut lvts = String::new();
                for c in arg0 {
                    lvts += String::from(*c).as_str();
                }
                write!(f, "{}", lvts)
            }
            Self::Op(arg0) => {
                let mut lvts = String::new();
                for c in arg0 {
                    lvts += String::from(*c).as_str();
                }
                write!(f, "{}", lvts)
            }
            Self::CallFn(arg0, arg1) => {
                let mut lvts = String::new();
                for c in arg0 {
                    lvts += String::from(*c).as_str();
                }
                let mut args = String::new();
                for arg in arg1 {
                    let mut s = String::new();
                    for c in arg {
                        lvts += String::from(*c).as_str();
                    }
                    s += " ";
                    args += s.as_str();
                }
                write!(f, "{}({})", lvts, args)
            }
        }
    }
}
impl From<&Token> for Expr {
    fn from(token: &Token) -> Self {
        match token.get_type() {
            super::TokenType::Name | super::TokenType::Num => Self::Data(token.get_text().clone()),
            super::TokenType::Symbol => Self::Op(token.get_text().clone()),
            _ => todo!(),
        }
    }
}
#[derive(Debug, Clone)]
pub struct Condition {
    lexpr: Expr,
    op: Vec<char>,
    rexpr: Expr,
}

impl Condition {
    pub fn new(lexpr: Expr, op: Vec<char>, rexpr: Expr) -> Self {
        Self { lexpr, op, rexpr }
    }
}
fn cus_to_str(cus: &Vec<ComUnit>) -> String {
    let mut ret = String::from("{");
    for cu in cus {
        ret += &format!("{:?}", cu);
    }
    ret + "}"
}
