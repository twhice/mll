/*
对PC的计量只能由高级可编译类型进行
避免重复计量PC导致错误
*/
use crate::{error::CTErr, DEBUG};

use super::{abi::*, code::*};
static mut OP_ID: usize = 0;
static mut PC: usize = 0;
static mut SHARD: Vec<Vec<char>> = Vec::new();
static mut DEFING: Option<Vec<char>> = None;
// 函数名 参数长 PC 有无返回值 映射
static mut FNS: Vec<(Vec<char>, usize, usize, bool)> = Vec::new();

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
    // 我是傻逼
    fn is_ctrl_return(&self) -> bool {
        false
    }
}

/*
if 条件     -> jump 条件取反 跳到if块最后一行的下一行
elif 条件   -> 同上

如果if/elif/else的总数>1,那除了最后一块都有
jump always 最后块后一行

方案1
set con = xxx
if con == 1 -> tag a
jump tag b
tag a: BODY
tag b: AFTER

+else:

set con = xxx
if con == 1 -> tag a
jump tag b0
tag a: BODY
jump tag z
tag b0: (if) con2 == 1 ->tag b
jump tag b1
...
tag z: AFTER

方案2
set con == xxx
if con != 1 ->tag b
BODY
tag b: AFTER

+else

set con == xxx
if con != 1 ->tag b
BODY
jump tag z
tag b: (if) con2 != 1->tag c
BODY
jump tag z
...
tag z: AFTER

方案混合

TODO

*/
impl Complite for CtrlIf {
    fn compliet(&self) -> Codes {
        if DEBUG {
            println!("TODO: core_::complier: CtrlIf编译优化 目前均用方案1");
            println!("TODO: core_::complier: CtrlIf编译优化 方案混合");
        }
        // 方案1
        let mut index_of_jump_to_next: Vec<usize> = Vec::new();
        let mut index_of_jump_to_end: Vec<usize> = Vec::new();
        let mut temp = Codes::new();
        let mut complier_one = |condition: &super::code::Condition,
                                statements: &Vec<Box<dyn Complite>>,
                                hava_next: bool| {
            unsafe {
                let old_temp_len = temp.len();
                // set con == ... && jump ...
                complier_condition(&mut temp, condition);
                // jump always ...
                if hava_next {
                    temp.push(LogicCode::Jump(
                        0,
                        super::abi::Condition::Always,
                        i_vul("114514"),
                        i_vul("191810"),
                    ));
                }
                // PC+=2/3
                PC += temp.len() - old_temp_len;
                // 重定向跳转位
                temp[PC - (1 + if hava_next { 1 } else { 0 })].change_jump_target(PC);
                // 放入重定向位
                if hava_next {
                    index_of_jump_to_next.push(PC - 1);
                }
                // 编译子语句
                for sta in statements {
                    temp.append(&mut sta.compliet());
                }
                if hava_next {
                    index_of_jump_to_end.push(PC);
                    temp.push(LogicCode::Jump(
                        0,
                        super::abi::Condition::Always,
                        i_vul("114514"),
                        i_vul("191810"),
                    ));
                    PC += 1;
                }
            }
        };
        // 主体
        complier_one(&self.condition, &self.if_statement, false);
        for cs in &self.elifs {
            complier_one(&cs.0, &cs.1, false)
        }
        if self.else_statement.len() != 0 {
            complier_one(
                &super::code::Condition::new(
                    Expr::Data(vec![' ']),
                    vec![' '],
                    Expr::Data(vec![' ']),
                ),
                &self.else_statement,
                true,
            )
        }
        return temp;
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
        println!("Called a functin that filled with bug.");
        unsafe {
            // 注册定义函数状态
            if let Some(fnindef) = &DEFING {
                CTErr::DefinDef(fnindef.to_owned(), self.fn_name.clone()).solve();
            } else {
                DEFING = Some(self.fn_name.clone());
            }
        }
        let mut temp = Codes::new();
        let fn_name = self.fn_name.clone();
        let mut have_ret = false;
        for sta in &self.statement {
            //  我爱泛型
            if sta.is_ctrl_return() {
                have_ret = true;
                break;
            }
        }
        unsafe {
            // 储存函数定义
            FNS.push((fn_name, self.args.len(), PC, have_ret));
            // 设置映射
            SHARD = self.args.clone();

            for cu in &self.statement {
                // 链接
                let ccu = cu.compliet();
                PC += ccu.len();
                temp.append(&mut cu.compliet());
            }

            DEFING = None;
        }

        return temp;
    }
}
/*
计算后接的表达式,拷贝到glb_return
 */
impl Complite for CtrlReturn {
    fn compliet(&self) -> Codes {
        let mut retv = Codes::new();
        complie_vul(&mut retv, &self.return_vul);
        // 忽略空值
        if retv.len() == 1 {
            if let LogicCode::Set(_, r) = &retv[0] {
                if r == &vec![' '] {
                    // 忽略
                    return Vec::new();
                }
            }
        }
        unsafe {
            // 增加计数器
            PC += retv.len();
        }
        return retv;
    }
    fn is_ctrl_return(&self) -> bool {
        true
    }
}
impl Complite for CtrlSwitch {
    fn compliet(&self) -> Codes {
        todo!()
    }
}
impl Complite for Expr {
    fn compliet(&self) -> Codes {
        match self {
            Expr::Eoe(lv, op, rv) => {
                let mut ret = Codes::new();
                // 获取 l
                let l = complie_vul(&mut ret, lv);
                // 获取 r
                let r = complie_vul(&mut ret, rv);
                // 获取符号
                let o: Op = complie_op(op);
                let t = alloc_new_id();
                ret.push(LogicCode::Op(o, t, l, r));
                return ret;
            }
            Expr::Eo(_, _) =>
            /*目前还没有吧*/
            {
                todo!()
            }
            Expr::Oe(op, vul) => {
                let mut temp = Codes::new();
                let v = complie_vul(&mut temp, vul);
                let o = complie_op(op);
                let t = alloc_new_id();
                let e = empty_vul();
                temp.push(LogicCode::Op(o, t, v, e));
                temp
            }
            Expr::Data(v) => unsafe {
                for shar_id in 0..SHARD.len() {
                    let shar = &SHARD[shar_id];
                    // 是映射
                    if v == shar {
                        // 断言获取函数名
                        if let Some(fn_name) = &DEFING {
                            let mut v = fn_name.clone();
                            v.append(&mut format!("_arg_{}", shar_id).chars().collect());
                            return vec![LogicCode::Set(alloc_new_id(), v)];
                        }
                    }
                }
                // 默认情况
                vec![LogicCode::Set(alloc_new_id(), v.clone())]
            },
            // 不直接调用
            Expr::Op(_) => todo!(),
            Expr::CallFn(fn_name, args) => {
                let mut temp = Codes::new();
                // 简化
                let match_marco = |text: &str, args_len: usize| -> bool {
                    *fn_name == text.chars().collect::<Vec<char>>() && args.len() == args_len
                };
                // 尝试宏展开
                {
                    // 三角函数
                    {
                        if match_marco("sin", 1) {
                            let t = alloc_new_id();
                            let l = complie_vul(&mut temp, &args[0]);
                            let r = empty_vul();
                            temp.push(LogicCode::Op(Op::Sin, t, l, r));
                            return temp;
                        }
                        if match_marco("cos", 1) {
                            let t = alloc_new_id();
                            let l = complie_vul(&mut temp, &args[0]);
                            let r = empty_vul();
                            temp.push(LogicCode::Op(Op::Cos, t, l, r));
                            return temp;
                        }
                        if match_marco("tan", 1) {
                            let t = alloc_new_id();
                            let l = complie_vul(&mut temp, &args[0]);
                            let r = empty_vul();
                            temp.push(LogicCode::Op(Op::Tan, t, l, r));
                            return temp;
                        }
                        if match_marco("asin", 1) {
                            let t = alloc_new_id();
                            let l = complie_vul(&mut temp, &args[0]);
                            let r = empty_vul();
                            temp.push(LogicCode::Op(Op::Asin, t, l, r));
                            return temp;
                        }
                        if match_marco("acos", 1) {
                            let t = alloc_new_id();
                            let l = complie_vul(&mut temp, &args[0]);
                            let r = empty_vul();
                            temp.push(LogicCode::Op(Op::Acos, t, l, r));
                            return temp;
                        }
                        if match_marco("atan", 1) {
                            let t = alloc_new_id();
                            let l = complie_vul(&mut temp, &args[0]);
                            let r = empty_vul();
                            temp.push(LogicCode::Op(Op::Atan, t, l, r));
                            return temp;
                        }
                    }
                    // 数学方法
                    {
                        if match_marco("abs", 1) {
                            let t = alloc_new_id();
                            let l = complie_vul(&mut temp, &args[0]);
                            let r = empty_vul();
                            temp.push(LogicCode::Op(Op::Abs, t, l, r));
                            return temp;
                        }
                        if match_marco("ceil", 1) {
                            let t = alloc_new_id();
                            let l = complie_vul(&mut temp, &args[0]);
                            let r = empty_vul();
                            temp.push(LogicCode::Op(Op::Ceil, t, l, r));
                            return temp;
                        }
                        if match_marco("floor", 1) {
                            let t = alloc_new_id();
                            let l = complie_vul(&mut temp, &args[0]);
                            let r = empty_vul();
                            temp.push(LogicCode::Op(Op::Floor, t, l, r));
                            return temp;
                        }
                        if match_marco("sqrt", 1) {
                            let t = alloc_new_id();
                            let l = complie_vul(&mut temp, &args[0]);
                            let r = empty_vul();
                            temp.push(LogicCode::Op(Op::Sqrt, t, l, r));
                            return temp;
                        }
                    }
                    // 向量
                    {}
                }
                // 在函数表中搜索
                unsafe {
                    for func in &FNS {
                        // 名称 与 参数长均相等->确定函数
                        if *fn_name == func.0 && args.len() == func.1 {
                            // 避免递归
                            if let Some(fnn) = &DEFING {
                                if fnn == fn_name {
                                    CTErr::CallFninDef(fn_name.clone()).solve()
                                }
                            }
                            // 传递参数
                            for index in 0..args.len() {
                                let mut arg_name = fn_name.clone();
                                arg_name.append(&mut format!("_arg_{}", index).chars().collect());
                                let arg_vul = complie_vul(&mut temp, &args[index]);
                                temp.push(LogicCode::Set(arg_name, arg_vul))
                            }
                            // 保存计数器，使得函数能够跳转会
                            temp.push(LogicCode::Op(
                                Op::Add,
                                i_vul("mll_global_callfnpos"),
                                i_vul("@counter"),
                                i_vul("1"),
                            ));

                            // 跳转到函数
                            temp.push(LogicCode::Set(
                                i_vul("@counter"),
                                i_vul(func.2.to_string().as_str()),
                            ));
                            // 若有返回，则复制返回值
                            if func.3 {
                                temp.push(LogicCode::Set(alloc_new_id(), i_vul("mll_global_fnret")))
                            }
                            return temp;
                        }
                    }
                }
                // 未遂
                {
                    CTErr::UnknowFn(fn_name.clone()).solve();
                    todo!()
                }
            }
        }
    }
}
impl Complite for Set {
    fn compliet(&self) -> Codes {
        todo!()
    }
}
fn complie_vul(temp: &mut Codes, v: &Expr) -> Vec<char> {
    let ret;
    let mut v_code = v.compliet();
    // 优化: 立即值内联
    // 断言 v_code.len() == 1
    if let LogicCode::Set(_, vul) = &v_code[v_code.len() - 1] {
        ret = vul.clone();
    } else if let LogicCode::Op(_, vul, _, _) = &v_code[v_code.len() - 1] {
        ret = vul.clone();
        temp.append(&mut v_code);
    } else {
        todo!()
    }
    ret
}
fn complie_op(v: &Expr) -> Op {
    match (*v).clone().try_into() {
        Ok(ok) => ok,
        // 断言不会失败
        Err(_) => todo!(),
    }
}
fn alloc_new_id() -> Vec<char> {
    unsafe {
        let mut ret = String::from("mll_op_");
        ret += OP_ID.to_string().as_str();
        OP_ID += 1;
        ret.chars().collect::<Vec<char>>()
    }
}
fn empty_vul() -> Vec<char> {
    "0".chars().collect()
}
fn i_vul(vul: &str) -> Vec<char> {
    vul.chars().collect()
}
fn complier_condition(mut temp: &mut Codes, condition: &super::code::Condition) {
    // 编译左
    let nolv = complie_vul(&mut temp, &condition.lexpr);
    // 编译右
    let norv = complie_vul(&mut temp, &condition.rexpr);
    let op = Op::from(condition.op.clone());
    let ano_op = Op::from(condition.op.clone());
    match super::abi::Condition::try_from(op) {
        Ok(condition) => temp.push(LogicCode::Jump(0, condition, nolv, norv)),
        Err(_) => {
            let new_id = alloc_new_id();
            temp.push(LogicCode::Op(ano_op, new_id.clone(), nolv, norv));
            temp.push(LogicCode::Jump(
                0,
                super::abi::Condition::NotEq,
                new_id,
                empty_vul(),
            ))
        }
    }
}

// 链接部分,应当由complier集成
// 链接 局部跳转转化为全局跳转
//
// 作废
