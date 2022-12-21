/*
对PC的计量只能由高级可编译类型进行
避免重复计量PC导致错误
*/
use crate::{error::CTErr, DEBUG};

use super::{abi::*, code::*};
static mut FNS: Vec<(Vec<char>, usize, usize, bool)> = Vec::new();
pub trait Complite
where
    Self: Debug,
{
    fn compliet(&self) -> Codes;
    // 我是傻逼
    fn is_ctrl_return(&self) -> bool {
        false
    }
}

use std::fmt::Debug;
/*
    代码生成部分
*/
type Codes = Vec<LogicCode>;
impl Complite for CtrlDef {
    fn compliet(&self) -> Codes {
        todo!()
    }
}
impl Complite for CtrlIf {
    fn compliet(&self) -> Codes {
        todo!()
    }
}
impl Complite for CtrlReturn {
    fn compliet(&self) -> Codes {
        todo!()
    }
}
impl Complite for CtrlSwitch {
    fn compliet(&self) -> Codes {
        todo!()
    }
}
impl Complite for CtrlWhile {
    fn compliet(&self) -> Codes {
        todo!()
    }
}
impl Complite for Set {
    fn compliet(&self) -> Codes {
        todo!()
    }
}
impl Complite for Expr {
    fn compliet(&self) -> Codes {
        todo!()
    }
}
