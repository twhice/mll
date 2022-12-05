use super::abi::*;
use super::code::*;
use std::fmt::Debug;
/*
    代码生成部分
*/
type Codes = Vec<LogicCode>;
pub trait Complite
where
    Self: Debug,
{
    fn compliet(&self) -> Codes;
}
/*
if 条件     -> jump 条件取反 跳到if块最后一行的下一行
elif 条件   -> 同上

如果if/elif/else的总数>1,那除了最后一块都有
jump always 最后块后一行

*/
impl Complite for CtrlIf {
    fn compliet(&self) -> Codes {
        todo!()
    }
}
/*
while 条件  ->jump 条件取反
块最后一行加上jump always 判定行
*/
impl Complite for CtrlWhile {
    fn compliet(&self) -> Codes {
        todo!()
    }
}
/*
所有def编译到程序头
程序第零行跳转到所有def块后

逐行翻译语句
*/
impl Complite for CtrlDef {
    fn compliet(&self) -> Codes {
        todo!()
    }
}
/*
计算后接的表达式,拷贝到glb_return
 */
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
impl Complite for Expr {
    fn compliet(&self) -> Codes {
        todo!()
    }
}
impl Complite for Set {
    fn compliet(&self) -> Codes {
        todo!()
    }
}
