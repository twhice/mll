use super::Token;
#[derive(Debug)]
pub enum ComUnit {
    Set(Vec<Set>),
    Ctrl(Ctrl),
}
#[derive(Debug)]
pub struct Set {
    lv: Vec<char>,
    rv: Expr,
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
#[derive(Debug)]
pub struct CtrlIf {
    condition: Expr,
    if_statement: Vec<ComUnit>,
    elifs: Vec<(Expr, Vec<ComUnit>)>,
    else_statement: Vec<ComUnit>,
}

impl CtrlIf {
    pub fn new(
        condition: Expr,
        if_statement: Vec<ComUnit>,
        elifs: Vec<(Expr, Vec<ComUnit>)>,
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
    condition: Expr,
    statements: Vec<ComUnit>,
}

impl CtrlWhile {
    pub fn new(condition: Expr, statements: Vec<ComUnit>) -> Self {
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

#[derive(Clone, Debug)]
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
impl From<&Token> for Expr {
    fn from(token: &Token) -> Self {
        match token.get_type() {
            super::TokenType::Name | super::TokenType::Num => Self::Data(token.get_text().clone()),
            super::TokenType::Symbol => Self::Op(token.get_text().clone()),
            super::TokenType::Str | super::TokenType::Space => todo!(),
        }
    }
}
