use std::fmt::Debug;

use super::code::*;
pub trait Complite
where
    Self: Debug,
{
    fn compliet(&self);
}
impl Complite for CtrlIf {
    fn compliet(&self) {
        todo!()
    }
}
impl Complite for CtrlWhile {
    fn compliet(&self) {
        todo!()
    }
}
impl Complite for CtrlDef {
    fn compliet(&self) {
        todo!()
    }
}
impl Complite for CtrlReturn {
    fn compliet(&self) {
        todo!()
    }
}
impl Complite for CtrlSwitch {
    fn compliet(&self) {
        todo!()
    }
}
impl Complite for Expr {
    fn compliet(&self) {
        todo!()
    }
}
impl Complite for Set {
    fn compliet(&self) {
        todo!()
    }
}
