use crate::error::CTErr;

use super::abi::*;
use super::code::*;
static mut OP_ID: usize = 0;
static mut PC: usize = 0;
static mut shard: Vec<Vec<char>> = Vec::new();
static mut defing: Option<Vec<char>> = None;
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
        println!("Called a functin that filled with bug.");
        unsafe {
            // 注册定义函数状态
            if let Some(fnindef) = &defing {
                CTErr::DefinDef(fnindef.to_owned(), self.fn_name.clone()).solve();
            } else {
                defing = Some(self.fn_name.clone());
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
            shard = self.args.clone();
        }
        for cu in &self.statement {
            // 编译并链接
            temp.append(&mut link(cu.compliet()));
        }
        unsafe {
            PC += temp.len();
            defing = None;
        }

        return temp;
    }
}
/*
计算后接的表达式,拷贝到glb_return
 */
impl Complite for CtrlReturn {
    fn compliet(&self) -> Codes {
        todo!()
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
                for shar_id in 0..shard.len() {
                    let shar = &shard[shar_id];
                    // 是映射
                    if v == shar {
                        // 断言获取函数名
                        if let Some(fn_name) = &defing {
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
                            if let Some(fnn) = &defing {
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
// 链接 局部跳转转化为全局跳转
fn link(mut local_codes: Codes) -> Codes {
    for lc_id in 0..local_codes.len() {
        if let LogicCode::Jump(target, ..) = local_codes[lc_id] {
            unsafe {
                local_codes[lc_id].change_jump_target(target + PC);
            }
        }
    }
    unsafe {
        PC += local_codes.len();
    }
    local_codes
}
