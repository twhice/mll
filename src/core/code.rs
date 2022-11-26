pub enum ComUnit {
    Set(Vec<Set>),
    Ctrl(Ctrl),
}
pub struct Set {
    lv: Vec<char>,
    rv: Expr,
}

impl Set {
    pub fn new(lv: Vec<char>, rv: Expr) -> Self {
        Self { lv, rv }
    }
}
pub enum Ctrl {
    Ctrl_if(CtrlIf),
    Ctrl_pg(CtrlPg),
    Ctrl_for(CtrlFor),
    Ctrl_switch(CtrlSwitch),
    Ctrl_return(CtrlReturn),
}

pub struct CtrlIf {
    condition: Expr,
    if_statement: Vec<ComUnit>,
    else_statement: Vec<ComUnit>,
}

impl CtrlIf {
    pub fn new(condition: Expr, if_statement: Vec<ComUnit>, else_statement: Vec<ComUnit>) -> Self {
        Self {
            condition,
            if_statement,
            else_statement,
        }
    }
}
pub struct CtrlPg {
    fn_name: Option<Vec<char>>,
    statement: Vec<ComUnit>,
}

impl CtrlPg {
    pub fn new(fn_name: Option<Vec<char>>, statement: Vec<ComUnit>) -> Self {
        Self { fn_name, statement }
    }
}
pub struct CtrlFor {
    index_name: Vec<char>,
    begin: isize,
    end: isize,
    step_size: isize,
    statement: Vec<ComUnit>,
}

impl CtrlFor {
    pub fn new(
        index_name: Vec<char>,
        begin: isize,
        end: isize,
        step_size: isize,
        statement: Vec<ComUnit>,
    ) -> Self {
        Self {
            index_name,
            begin,
            end,
            step_size,
            statement,
        }
    }
}
pub struct CtrlSwitch {
    cases: Vec<Vec<ComUnit>>,
}

impl CtrlSwitch {
    pub fn new(cases: Vec<Vec<ComUnit>>) -> Self {
        Self { cases }
    }
}
pub struct CtrlReturn {
    return_vul: Expr,
}

impl CtrlReturn {
    pub fn new(return_vul: Expr) -> Self {
        Self { return_vul }
    }
}

pub enum Expr {}
